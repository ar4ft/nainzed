use std::{rc::Rc, time::Duration};

use anyhow::Result;
use editor::{CompletionProvider, Editor};
use futures::FutureExt;
use gpui::{Context, Entity, Task, WeakEntity, Window};
use jupyter_protocol::ReplyStatus;
use language::{Anchor, Buffer, CodeLabel};
use project::{Completion, CompletionResponse, CompletionSource};
use text::ToOffset;

use super::NotebookEditor;

pub(super) struct KernelCompletionProvider(pub WeakEntity<NotebookEditor>);

impl KernelCompletionProvider {
    pub fn install(
        notebook: WeakEntity<NotebookEditor>,
        editor: &Entity<Editor>,
        cx: &mut gpui::App,
    ) {
        editor.update(cx, |editor, _| {
            editor.set_completion_provider(Some(Rc::new(Self(notebook))));
        });
    }
}

/// Jupyter offsets count Unicode characters; editor offsets count UTF-8 bytes.
fn byte_offset(code: &str, characters: usize) -> Option<usize> {
    code.char_indices()
        .map(|(offset, _)| offset)
        .chain([code.len()])
        .nth(characters)
}

impl CompletionProvider for KernelCompletionProvider {
    fn completions(
        &self,
        buffer: &Entity<Buffer>,
        position: Anchor,
        _: lsp::CompletionContext,
        _: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let snapshot = buffer.read(cx).snapshot();
        let code = buffer.read(cx).text();
        let offset = position.to_offset(&snapshot);
        let cursor_pos = code[..offset].chars().count();
        let notebook = self.0.clone();
        let request = notebook.update(cx, |notebook, cx| {
            notebook.request_completion(code.clone(), cursor_pos, cx)
        });
        let Ok(Some((id, response))) = request else {
            return Task::ready(Ok(Vec::new()));
        };
        let buffer = buffer.clone();
        cx.spawn(async move |_, cx| {
            let timer = cx.background_executor().timer(Duration::from_secs(3));
            let reply = futures::select! {
                reply = response.fuse() => reply.ok(),
                _ = timer.fuse() => None,
            };
            notebook
                .update(cx, |notebook, _| {
                    notebook.completion_requests.remove(&id);
                })
                .ok();
            let Some(reply) = reply else {
                return Ok(Vec::new());
            };
            if reply.status != ReplyStatus::Ok {
                return Ok(Vec::new());
            }
            let Some(start) = byte_offset(&code, reply.cursor_start) else {
                return Ok(Vec::new());
            };
            let Some(end) = byte_offset(&code, reply.cursor_end) else {
                return Ok(Vec::new());
            };
            if start > end || end > offset {
                return Ok(Vec::new());
            }
            // Never apply a response computed for an older cell version.
            if buffer.read_with(cx, |buffer, _| buffer.text()) != code {
                return Ok(Vec::new());
            }
            let range = snapshot.anchor_before(start)..snapshot.anchor_after(end);
            let completions = reply
                .matches
                .into_iter()
                .take(500)
                .map(|new_text| Completion {
                    label: CodeLabel {
                        text: new_text.clone(),
                        runs: Vec::new(),
                        filter_range: 0..new_text.len(),
                    },
                    new_text,
                    replace_range: range.clone(),
                    source: CompletionSource::Custom,
                    documentation: None,
                    icon_path: None,
                    icon_color: None,
                    match_start: Some(range.start),
                    snippet_deduplication_key: None,
                    insert_text_mode: None,
                    confirm: None,
                    group: None,
                })
                .collect();
            Ok(vec![CompletionResponse {
                completions,
                display_options: Default::default(),
                is_incomplete: true,
            }])
        })
    }

    fn is_completion_trigger(
        &self,
        _: &Entity<Buffer>,
        _: Anchor,
        text: &str,
        trigger_in_words: bool,
        _: &mut Context<Editor>,
    ) -> bool {
        text.ends_with('.')
            || (trigger_in_words
                && text
                    .chars()
                    .last()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_ai_fork_notebook_completion_unicode_offsets() {
        assert_eq!(byte_offset("π.x", 1), Some(2));
        assert_eq!(byte_offset("π.x", 3), Some(4));
        assert_eq!(byte_offset("π.x", 4), None);
        assert_eq!(byte_offset("", 0), Some(0));
    }
}
