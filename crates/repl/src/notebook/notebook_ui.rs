#![allow(unused, dead_code)]
use std::future::Future;
use std::{path::PathBuf, sync::Arc};

use anyhow::{Context as _, Result};
use client::proto::ViewId;
use collections::HashMap;
use editor::{Anchor, DisplayPoint};
use futures::FutureExt;
use futures::future::Shared;
use gpui::{
    AnyElement, App, Entity, EventEmitter, FocusHandle, Focusable, KeyContext, ListScrollEvent,
    ListState, Point, Task, TaskExt, actions, list, prelude::*,
};
use jupyter_protocol::JupyterKernelspec;
use language::{Language, LanguageRegistry};
use log;
use project::search::SearchQuery;
use project::{Project, ProjectEntryId, ProjectPath};
use settings::Settings as _;
use std::ops::Range;
use ui::{CommonAnimationExt, KeyBinding, Tooltip, prelude::*};
use workspace::item::{ItemEvent, SaveOptions, TabContentParams};
use workspace::searchable::{
    Direction, SearchEvent, SearchOptions, SearchToken, SearchableItem, SearchableItemHandle,
};
use workspace::{Item, ItemHandle, Pane, ProjectItem, ToolbarItemLocation};

use super::{Cell, CellEvent, CellPosition, MarkdownCellEvent, RenderableCell};

use nbformat::v4::CellId;
use nbformat::v4::Metadata as NotebookMetadata;
use serde_json;
use uuid::Uuid;

use crate::components::{KernelPickerDelegate, KernelSelector};
use crate::kernels::{
    Kernel, KernelSession, KernelSpecification, KernelStatus, LocalKernelSpecification,
    NativeRunningKernel, RemoteRunningKernel, SshRunningKernel, WslRunningKernel,
};
use crate::notebook::MovementDirection;
use crate::repl_store::ReplStore;

use picker::Picker;
use runtimelib::{ExecuteRequest, JupyterMessage, JupyterMessageContent};
use ui::PopoverMenuHandle;
use zed_actions::editor::{MoveDown, MoveUp};
use zed_actions::notebook::{
    AddCodeBlock, AddMarkdownBlock, ClearOutputs, DeleteCell, EnterCommandMode, EnterEditMode,
    InterruptKernel, MoveCellDown, MoveCellUp, NotebookMoveDown, NotebookMoveUp, OpenNotebook,
    RestartKernel, Run, RunAll, RunAndAdvance,
};

/// Whether the notebook is in command mode (navigating cells) or edit mode (editing a cell).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NotebookMode {
    Command,
    Edit,
}

#[derive(PartialEq, Eq)]
enum SelectionMode {
    SelectOnly,
    SelectAndMove,
}

pub(crate) const MAX_TEXT_BLOCK_WIDTH: f32 = 9999.0;
pub(crate) const SMALL_SPACING_SIZE: f32 = 8.0;
pub(crate) const MEDIUM_SPACING_SIZE: f32 = 12.0;
pub(crate) const LARGE_SPACING_SIZE: f32 = 16.0;
pub(crate) const GUTTER_WIDTH: f32 = 19.0;
pub(crate) const CODE_BLOCK_INSET: f32 = MEDIUM_SPACING_SIZE;
pub(crate) const CONTROL_SIZE: f32 = 20.0;

const NOTEBOOK_EXTENSION: &str = "ipynb";
actions!(notebook, [RestoreDeletedCell]);

pub fn init(cx: &mut App) {
    workspace::register_project_item::<NotebookEditor>(cx);
}

#[derive(Clone)]
pub struct NotebookMatch {
    cell_id: CellId,
    range: Range<Anchor>,
}

pub struct NotebookEditor {
    languages: Arc<LanguageRegistry>,
    project: Entity<Project>,
    worktree_id: project::WorktreeId,
    focus_handle: FocusHandle,
    notebook_item: Entity<NotebookItem>,
    notebook_language: Shared<Task<Option<Arc<Language>>>>,
    remote_id: Option<ViewId>,
    cell_list: ListState,
    notebook_mode: NotebookMode,
    selected_cell_index: usize,
    cell_order: Vec<CellId>,
    original_cell_order: Vec<CellId>,
    cell_map: HashMap<CellId, Cell>,
    kernel: Kernel,
    kernel_specification: Option<KernelSpecification>,
    execution_requests: HashMap<String, CellId>,
    pub(super) completion_requests:
        HashMap<String, futures::channel::oneshot::Sender<jupyter_protocol::CompleteReply>>,
    kernel_picker_handle: PopoverMenuHandle<Picker<KernelPickerDelegate>>,
    saving: bool,
    search_matches: Vec<NotebookMatch>,
    saved_metadata: serde_json::Value,
    recovery_dirty: bool,
    recovery_pending: bool,
    recovery_task: Task<()>,
    recovery_io: Shared<Task<()>>,
    deleted_cells: Vec<(usize, CellId, Cell)>,
}

enum SaveDestination {
    CurrentPath,
    NewPath(ProjectPath),
}

impl NotebookEditor {
    pub fn new(
        project: Entity<Project>,
        notebook_item: Entity<NotebookItem>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        let languages = project.read(cx).languages().clone();
        let language_name = notebook_item.read(cx).language_name();
        let worktree_id = notebook_item.read(cx).project_path.worktree_id;

        let notebook_language = notebook_item.read(cx).notebook_language();
        let notebook_language = cx
            .spawn_in(window, async move |_, _| notebook_language.await)
            .shared();

        let mut cell_order = vec![]; // Vec<CellId>
        let mut cell_map = HashMap::default(); // HashMap<CellId, Cell>

        let cell_count = notebook_item.read(cx).notebook.cells.len();
        for index in 0..cell_count {
            let cell = notebook_item.read(cx).notebook.cells[index].clone();
            let cell_id = cell.id();
            cell_order.push(cell_id.clone());
            let cell_entity = Cell::load(&cell, &languages, notebook_language.clone(), window, cx);

            match &cell_entity {
                Cell::Code(code_cell) => {
                    let cell_id_for_focus = cell_id.clone();
                    cx.subscribe_in(code_cell, window, move |this, _cell, event, window, cx| {
                        match event {
                            CellEvent::Run(cell_id) => {
                                this.execute_cell(cell_id.clone(), window, cx)
                            }
                            CellEvent::FocusedIn(_) => {
                                this.select_cell_by_id(&cell_id_for_focus, cx)
                            }
                        }
                    })
                    .detach();

                    let cell_id_for_editor = cell_id.clone();
                    let editor = code_cell.read(cx).editor().clone();
                    cx.subscribe(&editor, move |this, _editor, event, cx| {
                        if let editor::EditorEvent::Focused = event {
                            this.select_cell_by_id(&cell_id_for_editor, cx);
                        }
                    })
                    .detach();
                }
                Cell::Markdown(markdown_cell) => {
                    cx.subscribe(
                        markdown_cell,
                        move |_this, cell, event: &MarkdownCellEvent, cx| {
                            match event {
                                MarkdownCellEvent::FinishedEditing => {
                                    cell.update(cx, |cell, cx| {
                                        cell.reparse_markdown(cx);
                                    });
                                }
                                MarkdownCellEvent::Run(_cell_id) => {
                                    // run is handled separately by move_to_next_cell
                                    // Just reparse here
                                    cell.update(cx, |cell, cx| {
                                        cell.reparse_markdown(cx);
                                    });
                                }
                            }
                        },
                    )
                    .detach();

                    let cell_id_for_editor = cell_id.clone();
                    let editor = markdown_cell.read(cx).editor().clone();
                    cx.subscribe(&editor, move |this, _editor, event, cx| {
                        if let editor::EditorEvent::Focused = event {
                            this.select_cell_by_id(&cell_id_for_editor, cx);
                        }
                    })
                    .detach();
                }
                Cell::Raw(_) => {}
            }

            cell_map.insert(cell_id.clone(), cell_entity);
        }

        let notebook_handle = cx.entity().downgrade();
        let cell_count = cell_order.len();

        let this = cx.entity();
        let cell_list = ListState::new(cell_count, gpui::ListAlignment::Top, px(1000.));

        let saved_metadata =
            serde_json::to_value(&notebook_item.read(cx).notebook.metadata).unwrap_or_default();
        let mut editor = Self {
            saving: false,
            search_matches: Vec::new(),
            saved_metadata,
            recovery_dirty: false,
            recovery_pending: true,
            recovery_task: Task::ready(()),
            recovery_io: Task::ready(()).shared(),
            deleted_cells: Vec::new(),
            project,
            languages: languages.clone(),
            worktree_id,
            focus_handle,
            notebook_item: notebook_item.clone(),
            notebook_language,
            remote_id: None,
            cell_list,
            notebook_mode: NotebookMode::Command,
            selected_cell_index: 0,
            cell_order: cell_order.clone(),
            original_cell_order: cell_order.clone(),
            cell_map: cell_map.clone(),
            kernel: Kernel::Shutdown,
            kernel_specification: None,
            execution_requests: HashMap::default(),
            completion_requests: HashMap::default(),
            kernel_picker_handle: PopoverMenuHandle::default(),
        };
        for cell in editor.cell_map.values() {
            editor.observe_cell_edits(cell, cx);
        }
        editor.launch_kernel(window, cx);
        editor.refresh_language(cx);
        editor.refresh_kernelspecs(cx);
        editor.offer_recovery(window, cx);

        cx.subscribe(&notebook_item, |this, _item, _event, cx| {
            this.refresh_language(cx);
            this.notebook_changed(cx);
        })
        .detach();

        editor
    }

    pub(super) fn request_completion(
        &mut self,
        code: String,
        cursor_pos: usize,
        _: &mut Context<Self>,
    ) -> Option<(
        String,
        futures::channel::oneshot::Receiver<jupyter_protocol::CompleteReply>,
    )> {
        let Kernel::RunningKernel(kernel) = &mut self.kernel else {
            return None;
        };
        let message: JupyterMessage = jupyter_protocol::CompleteRequest { code, cursor_pos }.into();
        let id = message.header.msg_id.clone();
        let (sender, receiver) = futures::channel::oneshot::channel();
        kernel.request_tx().try_send(message).ok()?;
        // At most one active editor request needs an answer. Dropping old senders
        // cancels superseded requests and bounds memory while typing rapidly.
        self.completion_requests.clear();
        self.completion_requests.insert(id.clone(), sender);
        Some((id, receiver))
    }

    fn notebook_changed(&mut self, cx: &mut Context<Self>) {
        self.notebook_state_changed(cx);
        cx.emit(SearchEvent::MatchesInvalidated);
    }

    fn notebook_state_changed(&mut self, cx: &mut Context<Self>) {
        cx.emit(ItemEvent::Edit);
        cx.emit(ItemEvent::UpdateTab);
        self.schedule_recovery(cx);
        cx.notify();
    }

    fn observe_cell_edits(&self, cell: &Cell, cx: &mut Context<Self>) {
        if let Some(editor) = cell.editor(cx).cloned() {
            if matches!(cell, Cell::Code(_)) {
                super::completion::KernelCompletionProvider::install(
                    cx.entity().downgrade(),
                    &editor,
                    cx,
                );
            }
            let cell_id = cell.id(cx);
            cx.subscribe(&editor, move |this, _, event, cx| {
                if matches!(event, editor::EditorEvent::Focused) {
                    this.select_cell_by_id(&cell_id, cx);
                    cx.emit(SearchEvent::ActiveMatchChanged);
                }
                if matches!(
                    event,
                    editor::EditorEvent::Edited { .. }
                        | editor::EditorEvent::BufferEdited
                        | editor::EditorEvent::DirtyChanged
                ) {
                    this.notebook_changed(cx);
                }
            })
            .detach();
            cx.subscribe(&editor, |_, _, event: &SearchEvent, cx| {
                if matches!(event, SearchEvent::ActiveMatchChanged) {
                    cx.emit(SearchEvent::ActiveMatchChanged);
                }
            })
            .detach();
        }
    }

    fn save_status(&self, cx: &App) -> &'static str {
        if self.saving {
            "Saving…"
        } else if self.is_dirty(cx) {
            "Unsaved changes"
        } else {
            "Saved"
        }
    }

    fn recovery_path(&self, cx: &App) -> Option<PathBuf> {
        use sha2::{Digest, Sha256};
        if !self.project.read(cx).is_local() {
            return None;
        }
        let path = self
            .project
            .read(cx)
            .absolute_path(&self.notebook_item.read(cx).project_path, cx)?;
        let key = format!("{:x}", Sha256::digest(path.to_string_lossy().as_bytes()));
        Some(
            paths::data_dir()
                .join("notebook-recovery")
                .join(format!("{key}.json")),
        )
    }

    fn schedule_recovery(&mut self, cx: &mut Context<Self>) {
        if self.recovery_pending {
            return;
        }
        if !self.is_dirty(cx) {
            self.clear_recovery(cx);
            return;
        }
        let Some(path) = self.recovery_path(cx) else {
            return;
        };
        // Cancelling the previous task coalesces typing and streamed outputs.
        self.recovery_task = cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(std::time::Duration::from_millis(750))
                .await;
            let snapshot = this
                .read_with(cx, |this, cx| {
                    if !this.is_dirty(cx) {
                        return None;
                    }
                    let draft = this.serialized_notebook(cx).ok()?;
                    let record = notebook_safety::recovery_record(
                        &this.notebook_item.read(cx).original_text,
                        &draft,
                    )
                    .ok()?;
                    Some((this.project.read(cx).fs().clone(), record))
                })
                .ok()
                .flatten();
            if let Some((fs, record)) = snapshot {
                this.update(cx, |this, cx| {
                    let previous = this.recovery_io.clone();
                    // Debouncing may cancel timers, but never an in-flight write.
                    // Saves, clean undo and subsequent snapshots queue behind it.
                    this.recovery_io = cx
                        .spawn(async move |_, _| {
                            previous.await;
                            let result = async {
                                fs.create_dir(path.parent().unwrap()).await?;
                                fs.atomic_write(path, record).await
                            }
                            .await;
                            if let Err(error) = result {
                                log::error!("Notebook recovery snapshot failed: {error}");
                            }
                        })
                        .shared();
                })
                .ok();
            }
        });
    }

    fn clear_recovery(&mut self, cx: &mut Context<Self>) {
        self.recovery_pending = false;
        self.recovery_task = Task::ready(());
        let Some(path) = self.recovery_path(cx) else {
            return;
        };
        let fs = self.project.read(cx).fs().clone();
        let previous = self.recovery_io.clone();
        self.recovery_io = cx
            .spawn(async move |_, _| {
                previous.await;
                if let Err(error) = fs
                    .remove_file(
                        &path,
                        fs::RemoveOptions {
                            ignore_if_not_exists: true,
                            ..Default::default()
                        },
                    )
                    .await
                {
                    log::error!("Could not remove saved notebook recovery snapshot: {error}");
                }
            })
            .shared();
    }

    fn finish_recovery_check(&mut self, cx: &mut Context<Self>) {
        self.recovery_pending = false;
        self.schedule_recovery(cx);
    }

    fn offer_recovery(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(path) = self.recovery_path(cx) else {
            self.recovery_pending = false;
            return;
        };
        let fs = self.project.read(cx).fs().clone();
        cx.spawn_in(window, async move |this, cx| {
            let record = match fs.load(&path).await {
                Ok(record) => record,
                Err(error) => {
                    if fs.metadata(&path).await.is_ok_and(|metadata| metadata.is_none()) {
                        this.update(cx, |this, cx| this.finish_recovery_check(cx)).ok();
                    } else {
                        log::error!("Could not read notebook recovery snapshot; retained: {error}");
                    }
                    return;
                }
            };
            let draft = this.read_with(cx, |this, cx| notebook_safety::restore_recovery(&record, &this.notebook_item.read(cx).original_text));
            let draft = match draft {
                Ok(Ok(Some(draft))) => draft,
                Ok(Ok(None)) => {
                    if fs.remove_file(&path, fs::RemoveOptions { ignore_if_not_exists: true, ..Default::default() }).await.is_ok() {
                        this.update(cx, |this, cx| this.finish_recovery_check(cx)).ok();
                    }
                    return;
                }
                Ok(Err(error)) => {
                    // Keep conflicting drafts separately so new edits can recover too.
                    let retained = path.with_extension(format!("conflict-{}.json", uuid::Uuid::new_v4()));
                    let moved = fs.rename(&path, &retained, fs::RenameOptions::default()).await.is_ok();
                    let retained = if moved { &retained } else { &path };
                    this.update_in(cx, |this, window, cx| {
                        if moved { this.finish_recovery_check(cx); }
                        crate::python_setup::show_message(window, cx, gpui::PromptLevel::Warning,
                            "Notebook recovery needs review", Some(&format!("{error}. Snapshot: {}", retained.display())));
                    }).ok();
                    return;
                }
                Err(_) => return,
            };
            // Kernel startup can change notebook metadata while the prompt is open.
            // Only user-visible cell changes prevent restoring this draft.
            let checkpoint = this.read_with(cx, |this, cx| serde_json::to_value(this.to_notebook(cx)).map(|value| value["cells"].clone())).ok().and_then(Result::ok);
            let decision = this.update_in(cx, |_, window, cx| window.prompt(gpui::PromptLevel::Info,
                "Recover unsaved notebook changes?", Some("A local recovery snapshot is available. The notebook on disk will stay unchanged until you save."), &["Recover", "Discard"], cx));
            let Ok(decision) = decision else { return; };
            match decision.await.ok() {
                Some(0) => {
                    this.update_in(cx, |this, window, cx| {
                        let current = serde_json::to_value(this.to_notebook(cx)).ok().map(|value| value["cells"].clone());
                        if checkpoint.is_none() || current != checkpoint {
                            crate::python_setup::show_message(window, cx, gpui::PromptLevel::Warning,
                                "Notebook changed during recovery", Some("The recovery snapshot was retained. Reopen the notebook to recover it."));
                            return;
                        }
                        if let Ok(nbformat::Notebook::V4(notebook)) = nbformat::parse_notebook(&draft) {
                            this.notebook_item.update(cx, |item, _| { item.notebook = notebook.clone(); });
                            this.replace_cells(&notebook, window, cx);
                            this.recovery_dirty = true;
                            this.recovery_pending = false;
                            this.notebook_changed(cx);
                        }
                    }).ok();
                }
                Some(1) => {
                    if fs.remove_file(&path, fs::RemoveOptions { ignore_if_not_exists: true, ..Default::default() }).await.is_ok() {
                        this.update(cx, |this, cx| this.finish_recovery_check(cx)).ok();
                    }
                }
                _ => {}
            }
        }).detach();
    }

    fn replace_cells(
        &mut self,
        notebook: &nbformat::v4::Notebook,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.cell_order.clear();
        self.cell_map.clear();
        self.deleted_cells.clear();
        self.search_matches.clear();
        self.selected_cell_index = 0;
        for source in &notebook.cells {
            let id = source.id();
            let cell = Cell::load(
                source,
                &self.languages,
                self.notebook_language.clone(),
                window,
                cx,
            );
            self.observe_cell_edits(&cell, cx);
            if let Cell::Code(code) = &cell {
                cx.subscribe_in(code, window, |this, _, event, window, cx| match event {
                    CellEvent::Run(id) => this.execute_cell(id.clone(), window, cx),
                    CellEvent::FocusedIn(id) => this.select_cell_by_id(id, cx),
                })
                .detach();
            }
            if let Cell::Markdown(markdown) = &cell {
                cx.subscribe(markdown, |_, cell, _: &MarkdownCellEvent, cx| {
                    cell.update(cx, |cell, cx| cell.reparse_markdown(cx));
                })
                .detach();
            }
            self.cell_order.push(id.clone());
            self.cell_map.insert(id.clone(), cell);
        }
        self.cell_list = ListState::new(self.cell_order.len(), gpui::ListAlignment::Top, px(1000.));
    }

    fn refresh_kernelspecs(&mut self, cx: &mut Context<Self>) {
        let store = ReplStore::global(cx);
        let project = self.project.clone();
        let worktree_id = self.worktree_id;

        let refresh_task = store.update(cx, |store, cx| {
            store.refresh_python_kernelspecs(worktree_id, &project, cx)
        });

        cx.background_spawn(refresh_task).detach_and_log_err(cx);
    }

    fn refresh_language(&mut self, cx: &mut Context<Self>) {
        let notebook_language = self.notebook_item.read(cx).notebook_language();
        let task = cx.spawn(async move |this, cx| {
            let language = notebook_language.await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| {
                    for cell in this.cell_map.values() {
                        if let Cell::Code(code_cell) = cell {
                            code_cell.update(cx, |cell, cx| {
                                cell.set_language(language.clone(), cx);
                            });
                        }
                    }
                });
            }
            language
        });
        self.notebook_language = task.shared();
    }

    fn has_structural_changes(&self) -> bool {
        self.recovery_dirty || self.cell_order != self.original_cell_order
    }

    fn has_content_changes(&self, cx: &App) -> bool {
        self.cell_map.values().any(|cell| cell.is_dirty(cx))
            || serde_json::to_value(&self.notebook_item.read(cx).notebook.metadata)
                .unwrap_or_default()
                != self.saved_metadata
    }

    pub fn to_notebook(&self, cx: &App) -> nbformat::v4::Notebook {
        let cells: Vec<nbformat::v4::Cell> = self
            .cell_order
            .iter()
            .filter_map(|cell_id| {
                self.cell_map
                    .get(cell_id)
                    .map(|cell| cell.to_nbformat_cell(cx))
            })
            .collect();

        let metadata = self.notebook_item.read(cx).notebook.metadata.clone();

        nbformat::v4::Notebook {
            metadata,
            nbformat: 4,
            nbformat_minor: 5,
            cells,
        }
    }

    pub fn mark_as_saved(&mut self, cx: &mut Context<Self>) {
        self.recovery_dirty = false;
        self.clear_recovery(cx);
        self.original_cell_order = self.cell_order.clone();
        self.saved_metadata = serde_json::to_value(&self.notebook_item.read(cx).notebook.metadata)
            .unwrap_or_default();

        for cell in self.cell_map.values() {
            match cell {
                Cell::Code(code_cell) => {
                    code_cell.update(cx, |code_cell, cx| {
                        code_cell.outputs_changed = false;
                        let editor = code_cell.editor();
                        editor.update(cx, |editor, cx| {
                            editor.buffer().update(cx, |buffer, cx| {
                                if let Some(buf) = buffer.as_singleton() {
                                    buf.update(cx, |b, cx| {
                                        let version = b.version();
                                        b.did_save(version, None, cx);
                                    });
                                }
                            });
                        });
                    });
                }
                Cell::Markdown(markdown_cell) => {
                    markdown_cell.update(cx, |markdown_cell, cx| {
                        let editor = markdown_cell.editor();
                        editor.update(cx, |editor, cx| {
                            editor.buffer().update(cx, |buffer, cx| {
                                if let Some(buf) = buffer.as_singleton() {
                                    buf.update(cx, |b, cx| {
                                        let version = b.version();
                                        b.did_save(version, None, cx);
                                    });
                                }
                            });
                        });
                    });
                }
                Cell::Raw(_) => {}
            }
        }
        cx.emit(ItemEvent::UpdateTab);
        cx.notify();
    }

    fn serialized_notebook(&self, cx: &App) -> Result<String> {
        let preserve_outputs = self
            .cell_map
            .iter()
            .filter_map(|(id, cell)| match cell {
                Cell::Code(cell) if !cell.read(cx).outputs_changed => Some(id.to_string()),
                _ => None,
            })
            .collect();
        let code_cells: HashMap<_, _> = self
            .cell_map
            .iter()
            .filter_map(|(id, cell)| match cell {
                Cell::Code(code) => Some((id.as_str(), code)),
                _ => None,
            })
            .collect();
        let mut edited = serde_json::to_value(self.to_notebook(cx))?;
        if let Some(cells) = edited["cells"].as_array_mut() {
            for cell in cells {
                if let Some(code) = cell["id"].as_str().and_then(|id| code_cells.get(id)) {
                    cell["outputs"] = serde_json::Value::Array(code.read(cx).raw_outputs.clone());
                }
            }
        }
        let json = notebook_safety::prepare_save(
            &self.notebook_item.read(cx).original_json,
            edited,
            &preserve_outputs,
        )?;
        nbformat::parse_notebook(&json)
            .context("Notebook could not be read back after serialization")?;
        Ok(json)
    }

    fn save_impl(
        &mut self,
        destination: SaveDestination,
        project: Entity<Project>,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        if self.saving {
            return Task::ready(Err(anyhow::anyhow!(
                "A notebook save is already in progress"
            )));
        }
        let json = match self.serialized_notebook(cx) {
            Ok(json) => json,
            Err(error) => return Task::ready(Err(error)),
        };
        let old_recovery_path = self.recovery_path(cx);
        let item = self.notebook_item.read(cx);
        let project_path = item.project_path.clone();
        let original_text = item.original_text.clone();
        let cell_order = self.cell_order.clone();
        let saved_metadata = serde_json::to_value(&item.notebook.metadata).unwrap_or_default();
        self.saving = true;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = async {
                let buffer = project
                    .update(cx, |project, cx| {
                        project.open_buffer(project_path.clone(), cx)
                    })
                    .await?;
                anyhow::ensure!(
                    buffer.read_with(cx, |buffer, _| buffer.text()) == original_text,
                    "Notebook changed in another editor. Reload it before saving."
                );
                let target = match &destination {
                    SaveDestination::CurrentPath => project_path.clone(),
                    SaveDestination::NewPath(path) => path.clone(),
                };
                // Back up the file that will be overwritten, before editing its buffer.
                let (is_local, fs, absolute_path, entry_exists) =
                    project.read_with(cx, |project, cx| {
                        (
                            project.is_local(),
                            project.fs().clone(),
                            project.absolute_path(&target, cx),
                            project.entry_for_path(&target, cx).is_some(),
                        )
                    });
                if is_local {
                    if let Some(path) = absolute_path
                        && fs.metadata(&path).await?.is_some()
                    {
                        if target == project_path {
                            anyhow::ensure!(
                                fs.load(&path).await?.replace("\r\n", "\n") == original_text,
                                "Notebook changed on disk. Reload it before saving."
                            );
                        }
                        let mut backup = path.as_os_str().to_os_string();
                        backup.push(".bak");
                        fs.copy_file(
                            &path,
                            &PathBuf::from(backup),
                            fs::CopyOptions {
                                overwrite: true,
                                ..Default::default()
                            },
                        )
                        .await
                        .context(
                            "Could not create the notebook backup; original was not overwritten",
                        )?;
                    }
                } else if entry_exists {
                    anyhow::ensure!(
                        !target.path.is_empty(),
                        "Open the remote notebook's parent folder to create its backup"
                    );
                    let old_target = project
                        .update(cx, |project, cx| project.open_buffer(target.clone(), cx))
                        .await?;
                    let text = old_target.read_with(cx, |buffer, _| buffer.text());
                    let backup = project
                        .update(cx, |project, cx| project.create_buffer(None, false, cx))
                        .await?;
                    backup.update(cx, |buffer, cx| buffer.set_text(text, cx));
                    let backup_path = ProjectPath {
                        worktree_id: target.worktree_id,
                        path: util::rel_path::RelPath::from_unix_str(&format!(
                            "{}.bak",
                            target.path.as_unix_str()
                        ))?
                        .into(),
                    };
                    project
                        .update(cx, |project, cx| {
                            project.save_buffer_as(backup, backup_path, cx)
                        })
                        .await
                        .context(
                            "Could not create the notebook backup; original was not overwritten",
                        )?;
                }
                anyhow::ensure!(
                    buffer.read_with(cx, |buffer, _| buffer.text()) == original_text,
                    "Notebook changed while its backup was being created. Reload it before saving."
                );
                buffer.update(cx, |buffer, cx| buffer.set_text(json.clone(), cx));
                let save_result = match destination {
                    SaveDestination::CurrentPath => {
                        project
                            .update(cx, |project, cx| project.save_buffer(buffer.clone(), cx))
                            .await
                    }
                    SaveDestination::NewPath(ref path) => {
                        project
                            .update(cx, |project, cx| {
                                project.save_buffer_as(buffer.clone(), path.clone(), cx)
                            })
                            .await
                    }
                };
                if let Err(error) = save_result {
                    // Restore our staged buffer if no other view has edited it in the meantime.
                    buffer.update(cx, |buffer, cx| {
                        if buffer.text() == json {
                            buffer.set_text(original_text, cx);
                        }
                    });
                    return Err(error);
                }
                let entry_id = project.read_with(cx, |project, cx| {
                    project.entry_for_path(&target, cx).map(|entry| entry.id)
                });
                this.update(cx, |this, cx| {
                    let unchanged = this
                        .serialized_notebook(cx)
                        .is_ok_and(|current| current == json);
                    this.notebook_item.update(cx, |item, _| {
                        item.project_path = target;
                        if let Some(id) = entry_id {
                            item.id = id;
                        }
                        item.original_json =
                            serde_json::from_str(&json).expect("validated notebook JSON");
                        item.original_text = json;
                    });
                    this.original_cell_order = cell_order;
                    this.saved_metadata = saved_metadata;
                    // Edits made while the write was running must remain dirty.
                    if unchanged {
                        this.mark_as_saved(cx);
                    } else {
                        this.schedule_recovery(cx);
                    }
                    cx.notify();
                });
                if let Some(old_path) = old_recovery_path {
                    let current_path = this
                        .read_with(cx, |this, cx| this.recovery_path(cx))
                        .ok()
                        .flatten();
                    if current_path.as_ref() != Some(&old_path) {
                        if let Ok(previous) = this.read_with(cx, |this, _| this.recovery_io.clone())
                        {
                            previous.await;
                        }
                        let fs = project.read_with(cx, |project, _| project.fs().clone());
                        let _ = fs
                            .remove_file(
                                &old_path,
                                fs::RemoveOptions {
                                    ignore_if_not_exists: true,
                                    ..Default::default()
                                },
                            )
                            .await;
                    }
                }
                Ok(())
            }
            .await;
            this.update(cx, |this, cx| {
                this.saving = false;
                cx.notify();
            })
            .ok();
            result
        })
    }

    fn launch_kernel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let spec = self.kernel_specification.clone().or_else(|| {
            let language = self
                .notebook_item
                .read(cx)
                .notebook
                .metadata
                .language_info
                .as_ref()
                .map(|info| info.name.as_str())
                .unwrap_or("python");
            ReplStore::global(cx)
                .read(cx)
                .notebook_kernelspec(self.worktree_id, language)
        });

        let spec = spec.unwrap_or_else(|| {
            KernelSpecification::Jupyter(LocalKernelSpecification {
                name: "python3".to_string(),
                path: PathBuf::from("python3"),
                kernelspec: JupyterKernelspec {
                    argv: vec![
                        "python3".to_string(),
                        "-m".to_string(),
                        "ipykernel_launcher".to_string(),
                        "-f".to_string(),
                        "{connection_file}".to_string(),
                    ],
                    display_name: "Python 3".to_string(),
                    language: "python".to_string(),
                    interrupt_mode: None,
                    metadata: None,
                    env: None,
                },
            })
        });

        self.launch_kernel_with_spec(spec, window, cx);
    }

    fn launch_kernel_with_spec(
        &mut self,
        spec: KernelSpecification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Replies from the previous interpreter must never reach a new completion menu.
        self.completion_requests.clear();
        let entity_id = cx.entity_id();
        let working_directory = self
            .project
            .read(cx)
            .worktree_for_id(self.worktree_id, cx)
            .map(|worktree| worktree.read(cx).abs_path().to_path_buf())
            .unwrap_or_else(std::env::temp_dir);
        let fs = self.project.read(cx).fs().clone();
        let view = cx.entity();

        self.kernel_specification = Some(spec.clone());
        ReplStore::global(cx).update(cx, |store, cx| {
            store.set_active_kernelspec(self.worktree_id, spec.clone(), cx)
        });
        if let KernelSpecification::PythonEnv(environment) = &spec {
            if self.project.read(cx).is_local() {
                let interpreter = environment.path.clone();
                let resolve = self.project.read(cx).resolve_toolchain(
                    interpreter.clone(),
                    "Python".into(),
                    cx,
                );
                cx.spawn(async move |this, cx| {
                    let Ok(toolchain) = resolve.await else { return; };
                    this.update(cx, |this, cx| {
                        if !matches!(&this.kernel_specification, Some(KernelSpecification::PythonEnv(env)) if env.path == interpreter) { return; }
                        let path = this.notebook_item.read(cx).project_path.clone();
                        this.project.update(cx, |project, cx| project.activate_toolchain(path, toolchain, cx)).detach();
                    }).ok();
                }).detach();
            }
        }

        self.notebook_item.update(cx, |item, cx| {
            let kernel_name = spec.name().to_string();
            let language = spec.language().to_string();

            let display_name = match &spec {
                KernelSpecification::Jupyter(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::PythonEnv(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::JupyterServer(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::SshRemote(s) => s.kernelspec.display_name.clone(),
                KernelSpecification::WslRemote(s) => s.kernelspec.display_name.clone(),
            };

            let kernelspec_json = serde_json::json!({
                "display_name": display_name,
                "name": kernel_name,
                "language": language
            });

            if let Ok(k) = serde_json::from_value(kernelspec_json) {
                item.notebook.metadata.kernelspec = Some(k);
                cx.emit(());
            }
        });

        let kernel_task = match spec {
            KernelSpecification::Jupyter(local_spec) => NativeRunningKernel::new(
                local_spec,
                entity_id,
                working_directory,
                fs,
                view,
                window,
                cx,
            ),
            KernelSpecification::PythonEnv(env_spec) => NativeRunningKernel::new(
                env_spec.as_local_spec(),
                entity_id,
                working_directory,
                fs,
                view,
                window,
                cx,
            ),
            KernelSpecification::JupyterServer(remote_spec) => {
                RemoteRunningKernel::new(remote_spec, working_directory, view, window, cx)
            }

            KernelSpecification::SshRemote(spec) => {
                let project = self.project.clone();
                SshRunningKernel::new(spec, working_directory, project, view, window, cx)
            }
            KernelSpecification::WslRemote(spec) => {
                WslRunningKernel::new(spec, entity_id, working_directory, fs, view, window, cx)
            }
        };

        let pending_kernel = cx
            .spawn(async move |this, cx| {
                let kernel = kernel_task.await;

                match kernel {
                    Ok(kernel) => {
                        this.update(cx, |editor, cx| {
                            editor.kernel = Kernel::RunningKernel(kernel);
                            cx.notify();
                        })
                        .ok();
                    }
                    Err(err) => {
                        log::error!("Kernel failed to start: {:?}", err);
                        this.update(cx, |editor, cx| {
                            editor.kernel = Kernel::ErroredLaunch(err.to_string());
                            cx.notify();
                        })
                        .ok();
                    }
                }
            })
            .shared();

        self.kernel = Kernel::StartingKernel(pending_kernel);
        cx.notify();
    }

    // Note: Python environments are only detected as kernels if ipykernel is installed.
    // Users need to run `pip install ipykernel` (or `uv pip install ipykernel`) in their
    // virtual environment for it to appear in the kernel selector.
    // This happens because we have an ipykernel check inside the function python_env_kernel_specification in mod.rs L:121

    fn change_kernel(
        &mut self,
        spec: KernelSpecification,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Kernel::RunningKernel(kernel) = &mut self.kernel {
            kernel.force_shutdown(window, cx).detach();
        }

        self.execution_requests.clear();

        self.launch_kernel_with_spec(spec, window, cx);
    }

    fn restart_kernel(&mut self, _: &RestartKernel, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(spec) = self.kernel_specification.clone() {
            if let Kernel::RunningKernel(kernel) = &mut self.kernel {
                kernel.force_shutdown(window, cx).detach();
            }

            self.kernel = Kernel::Restarting;
            cx.notify();

            self.launch_kernel_with_spec(spec, window, cx);
        }
    }

    fn interrupt_kernel(
        &mut self,
        _: &InterruptKernel,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Kernel::RunningKernel(kernel) = &self.kernel {
            let interrupt_request = runtimelib::InterruptRequest {};
            let message: JupyterMessage = interrupt_request.into();
            kernel.request_tx().try_send(message).ok();
            cx.notify();
        }
    }

    fn execute_cell(&mut self, cell_id: CellId, window: &mut Window, cx: &mut Context<Self>) {
        let code = if let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id) {
            let editor = cell.read(cx).editor().clone();
            let buffer = editor.read(cx).buffer().read(cx);
            buffer
                .as_singleton()
                .map(|b| b.read(cx).text())
                .unwrap_or_default()
        } else {
            return;
        };

        let request = ExecuteRequest {
            code,
            ..Default::default()
        };
        let message: JupyterMessage = request.into();
        let msg_id = message.header.msg_id.clone();

        let send_result = match &mut self.kernel {
            Kernel::RunningKernel(kernel) => kernel
                .request_tx()
                .try_send(message)
                .map_err(|err| format!("failed to send execute request to kernel (the kernel process may have died): {err}")),
            Kernel::StartingKernel(_) => Err("the kernel is still starting".to_string()),
            Kernel::ErroredLaunch(error) => Err(format!("the kernel failed to launch: {error}")),
            Kernel::ShuttingDown | Kernel::Shutdown => Err("the kernel is shut down".to_string()),
            Kernel::Restarting => Err("the kernel is restarting".to_string()),
        };

        if let Some(Cell::Code(cell)) = self.cell_map.get(&cell_id) {
            cell.update(cx, |cell, cx| {
                if cell.has_outputs() {
                    cell.clear_outputs();
                }
                if let Err(error) = &send_result {
                    cell.show_kernel_error(error, window, cx);
                } else {
                    cell.start_execution();
                }
                cx.notify();
            });
            self.notebook_changed(cx);
        }

        if let Err(error) = send_result {
            log::error!("notebook: cannot execute cell: {error}");
        } else {
            self.execution_requests.insert(msg_id, cell_id.clone());
        }
    }

    fn get_selected_cell(&self) -> Option<&Cell> {
        self.cell_order
            .get(self.selected_cell_index)
            .and_then(|cell_id| self.cell_map.get(cell_id))
    }

    fn has_outputs(&self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.cell_map.values().any(|cell| {
            if let Cell::Code(code_cell) = cell {
                code_cell.read(cx).has_outputs()
            } else {
                false
            }
        })
    }

    fn clear_outputs(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for cell in self.cell_map.values() {
            if let Cell::Code(code_cell) = cell {
                code_cell.update(cx, |cell, cx| {
                    cell.clear_outputs();
                    cx.notify();
                });
            }
        }
        self.notebook_changed(cx);
    }

    fn run_cells(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for cell_id in self.cell_order.clone() {
            self.execute_cell(cell_id, window, cx);
        }
    }

    fn run_current_cell(&mut self, _: &Run, window: &mut Window, cx: &mut Context<Self>) {
        let Some(cell_id) = self.cell_order.get(self.selected_cell_index).cloned() else {
            return;
        };
        let Some(cell) = self.cell_map.get(&cell_id) else {
            return;
        };
        match cell {
            Cell::Code(_) => {
                self.execute_cell(cell_id, window, cx);
            }
            Cell::Markdown(markdown_cell) => {
                // for markdown, finish editing and move to next cell
                let is_editing = markdown_cell.read(cx).is_editing();
                if is_editing {
                    markdown_cell.update(cx, |cell, cx| {
                        cell.run(cx);
                    });
                    self.enter_command_mode(window, cx);
                }
            }
            Cell::Raw(_) => {}
        }
    }

    fn run_and_advance(&mut self, _: &RunAndAdvance, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(cell_id) = self.cell_order.get(self.selected_cell_index).cloned() {
            if let Some(cell) = self.cell_map.get(&cell_id) {
                match cell {
                    Cell::Code(_) => {
                        self.execute_cell(cell_id, window, cx);
                    }
                    Cell::Markdown(markdown_cell) => {
                        if markdown_cell.read(cx).is_editing() {
                            markdown_cell.update(cx, |cell, cx| {
                                cell.run(cx);
                            });
                        }
                    }
                    Cell::Raw(_) => {}
                }
            }
        }

        let is_last_cell = self.selected_cell_index == self.cell_count().saturating_sub(1);
        if is_last_cell {
            self.add_code_block(window, cx);
            self.enter_command_mode(window, cx);
        } else {
            self.advance_in_command_mode(window, cx);
        }
    }

    fn enter_edit_mode(&mut self, _: &EnterEditMode, window: &mut Window, cx: &mut Context<Self>) {
        self.notebook_mode = NotebookMode::Edit;
        if let Some(cell_id) = self.cell_order.get(self.selected_cell_index) {
            if let Some(cell) = self.cell_map.get(cell_id) {
                match cell {
                    Cell::Code(code_cell) => {
                        let editor = code_cell.read(cx).editor().clone();
                        window.focus(&editor.focus_handle(cx), cx);
                    }
                    Cell::Markdown(markdown_cell) => {
                        markdown_cell.update(cx, |cell, cx| {
                            cell.set_editing(true);
                            cx.notify();
                        });
                        let editor = markdown_cell.read(cx).editor().clone();
                        window.focus(&editor.focus_handle(cx), cx);
                    }
                    Cell::Raw(_) => {}
                }
            }
        }
        cx.notify();
    }

    fn enter_command_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.notebook_mode = NotebookMode::Command;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    fn handle_enter_command_mode(
        &mut self,
        _: &EnterCommandMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.enter_command_mode(window, cx);
    }

    /// Advances to the next cell while staying in command mode (used by RunAndAdvance and shift-enter).
    fn advance_in_command_mode(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let count = self.cell_count();
        if count == 0 {
            return;
        }
        if self.selected_cell_index < count - 1 {
            self.selected_cell_index += 1;
            self.cell_list
                .scroll_to_reveal_item(self.selected_cell_index);
        }
        self.notebook_mode = NotebookMode::Command;
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    // Discussion can be done on this default implementation
    /// Moves focus to the next cell editor (used when already in edit mode).
    fn move_to_next_cell(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.cell_order.is_empty() && self.selected_cell_index < self.cell_order.len() - 1 {
            self.selected_cell_index += 1;
            // focus the new cell's editor
            if let Some(cell_id) = self.cell_order.get(self.selected_cell_index) {
                if let Some(cell) = self.cell_map.get(cell_id) {
                    match cell {
                        Cell::Code(code_cell) => {
                            let editor = code_cell.read(cx).editor();
                            window.focus(&editor.focus_handle(cx), cx);
                        }
                        Cell::Markdown(markdown_cell) => {
                            // Don't auto-enter edit mode for next markdown cell
                            // Just select it
                        }
                        Cell::Raw(_) => {}
                    }
                }
            }
            cx.notify();
        } else {
            // in the end, could optionally create a new cell
            // For now, just stay on the current cell
        }
    }

    fn open_notebook(&mut self, _: &OpenNotebook, _window: &mut Window, _cx: &mut Context<Self>) {
        println!("Open notebook triggered");
    }

    fn move_cell_up(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Move cell up triggered");
        if self.selected_cell_index > 0 {
            self.cell_order
                .swap(self.selected_cell_index, self.selected_cell_index - 1);
            self.selected_cell_index -= 1;
            self.notebook_changed(cx);
        }
    }

    fn move_cell_down(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        println!("Move cell down triggered");
        if !self.cell_order.is_empty() && self.selected_cell_index < self.cell_order.len() - 1 {
            self.cell_order
                .swap(self.selected_cell_index, self.selected_cell_index + 1);
            self.selected_cell_index += 1;
            self.notebook_changed(cx);
        }
    }

    fn delete_cell(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.cell_order.is_empty() {
            return;
        }
        let index = self.selected_cell_index.min(self.cell_order.len() - 1);
        let cell_id = self.cell_order.remove(index);
        if let Some(cell) = self.cell_map.remove(&cell_id) {
            self.deleted_cells.push((index, cell_id, cell));
            if self.deleted_cells.len() > 20 {
                self.deleted_cells.remove(0);
            }
        }
        self.cell_list.splice(index..index + 1, 0);

        if self.cell_order.is_empty() {
            self.selected_cell_index = 0;
        } else {
            self.selected_cell_index = index.min(self.cell_order.len() - 1);
            self.cell_list
                .scroll_to_reveal_item(self.selected_cell_index);
        }
        self.notebook_mode = NotebookMode::Command;
        window.focus(&self.focus_handle, cx);
        self.notebook_changed(cx);
    }

    fn restore_deleted_cell(
        &mut self,
        _: &RestoreDeletedCell,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((index, id, cell)) = self.deleted_cells.pop() else {
            return;
        };
        let index = index.min(self.cell_order.len());
        self.cell_order.insert(index, id.clone());
        self.cell_map.insert(id, cell);
        self.cell_list.splice(index..index, 1);
        self.selected_cell_index = index;
        self.cell_list.scroll_to_reveal_item(index);
        self.notebook_changed(cx);
    }

    fn insert_cell_at_current_position(&mut self, cell_id: CellId, cell: Cell) {
        let insert_index = if self.cell_order.is_empty() {
            0
        } else {
            self.selected_cell_index + 1
        };
        self.cell_order.insert(insert_index, cell_id.clone());
        self.cell_map.insert(cell_id, cell);
        self.selected_cell_index = insert_index;
        self.cell_list.splice(insert_index..insert_index, 1);
        self.cell_list.scroll_to_reveal_item(insert_index);
    }

    fn add_markdown_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_cell_id: CellId = Uuid::new_v4().into();
        let languages = self.languages.clone();
        let metadata: nbformat::v4::CellMetadata =
            serde_json::from_str("{}").expect("empty object should parse");

        let markdown_cell = cx.new(|cx| {
            super::MarkdownCell::new(
                new_cell_id.clone(),
                metadata,
                String::new(),
                languages,
                window,
                cx,
            )
        });

        cx.subscribe(
            &markdown_cell,
            move |_this, cell, event: &MarkdownCellEvent, cx| match event {
                MarkdownCellEvent::FinishedEditing | MarkdownCellEvent::Run(_) => {
                    cell.update(cx, |cell, cx| {
                        cell.reparse_markdown(cx);
                    });
                }
            },
        )
        .detach();

        let cell_id_for_editor = new_cell_id.clone();
        let editor = markdown_cell.read(cx).editor().clone();
        cx.subscribe(&editor, move |this, _editor, event, cx| {
            if let editor::EditorEvent::Focused = event {
                this.select_cell_by_id(&cell_id_for_editor, cx);
            }
        })
        .detach();

        self.observe_cell_edits(&Cell::Markdown(markdown_cell.clone()), cx);
        self.insert_cell_at_current_position(new_cell_id, Cell::Markdown(markdown_cell.clone()));
        markdown_cell.update(cx, |cell, cx| {
            cell.set_editing(true);
            cx.notify();
        });
        let editor = markdown_cell.read(cx).editor().clone();
        window.focus(&editor.focus_handle(cx), cx);
        self.notebook_mode = NotebookMode::Edit;
        self.notebook_changed(cx);
    }

    fn add_code_block(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let new_cell_id: CellId = Uuid::new_v4().into();
        let notebook_language = self.notebook_language.clone();
        let metadata: nbformat::v4::CellMetadata =
            serde_json::from_str("{}").expect("empty object should parse");

        let code_cell = cx.new(|cx| {
            super::CodeCell::new(
                super::CellSource::None,
                new_cell_id.clone(),
                metadata,
                String::new(),
                notebook_language,
                window,
                cx,
            )
        });

        let cell_id_for_run = new_cell_id.clone();
        cx.subscribe_in(
            &code_cell,
            window,
            move |this, _cell, event, window, cx| match event {
                CellEvent::Run(cell_id) => this.execute_cell(cell_id.clone(), window, cx),
                CellEvent::FocusedIn(_) => this.select_cell_by_id(&cell_id_for_run, cx),
            },
        )
        .detach();

        let cell_id_for_editor = new_cell_id.clone();
        let editor = code_cell.read(cx).editor().clone();
        cx.subscribe(&editor, move |this, _editor, event, cx| {
            if let editor::EditorEvent::Focused = event {
                this.select_cell_by_id(&cell_id_for_editor, cx);
            }
        })
        .detach();

        self.observe_cell_edits(&Cell::Code(code_cell.clone()), cx);
        self.insert_cell_at_current_position(new_cell_id, Cell::Code(code_cell.clone()));
        let editor = code_cell.read(cx).editor().clone();
        window.focus(&editor.focus_handle(cx), cx);
        self.notebook_mode = NotebookMode::Edit;
        self.notebook_changed(cx);
    }

    fn cell_count(&self) -> usize {
        self.cell_map.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_cell_index
    }

    fn select_cell_by_id(&mut self, cell_id: &CellId, cx: &mut Context<Self>) {
        if let Some(index) = self.cell_order.iter().position(|id| id == cell_id) {
            self.selected_cell_index = index;
            self.notebook_mode = NotebookMode::Edit;
            cx.notify();
        }
    }

    pub fn set_selected_index(
        &mut self,
        index: usize,
        jump_to_index: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // let previous_index = self.selected_cell_index;
        self.selected_cell_index = index;
        let current_index = self.selected_cell_index;

        // in the future we may have some `on_cell_change` event that we want to fire here

        if jump_to_index {
            self.jump_to_cell(current_index, window, cx);
        }
    }

    fn select_next(
        &mut self,
        _: &menu::SelectNext,
        selection_mode: SelectionMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == count - 1 {
                count - 1
            } else {
                index + 1
            };
            self.set_selected_index(ix, true, window, cx);

            if selection_mode == SelectionMode::SelectAndMove
                && let Some(cell) = self.get_selected_cell()
            {
                cell.move_to(MovementDirection::Start, window, cx);
            }

            cx.notify();
        }
    }

    fn select_previous(
        &mut self,
        _: &menu::SelectPrevious,
        selection_mode: SelectionMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            let index = self.selected_index();
            let ix = if index == 0 { 0 } else { index - 1 };
            self.set_selected_index(ix, true, window, cx);

            if selection_mode == SelectionMode::SelectAndMove
                && let Some(cell) = self.get_selected_cell()
            {
                cell.move_to(MovementDirection::End, window, cx);
            }

            cx.notify();
        }
    }

    pub fn select_first(
        &mut self,
        _: &menu::SelectFirst,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(0, true, window, cx);
            cx.notify();
        }
    }

    pub fn select_last(
        &mut self,
        _: &menu::SelectLast,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let count = self.cell_count();
        if count > 0 {
            self.set_selected_index(count - 1, true, window, cx);
            cx.notify();
        }
    }

    fn jump_to_cell(&mut self, index: usize, _window: &mut Window, _cx: &mut Context<Self>) {
        self.cell_list.scroll_to_reveal_item(index);
    }

    fn button_group(window: &mut Window, cx: &mut Context<Self>) -> Div {
        v_flex()
            .gap(DynamicSpacing::Base04.rems(cx))
            .items_center()
            .w(px(CONTROL_SIZE + 4.0))
            .overflow_hidden()
            .rounded(px(5.))
            .bg(cx.theme().colors().title_bar_background)
            .p_px()
            .border_1()
            .border_color(cx.theme().colors().border)
    }

    fn render_notebook_control(
        id: impl Into<SharedString>,
        icon: IconName,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> IconButton {
        let id: ElementId = ElementId::Name(id.into());
        IconButton::new(id, icon).width(px(CONTROL_SIZE))
    }

    fn render_notebook_controls(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_outputs = self.has_outputs(window, cx);

        v_flex()
            .max_w(px(CONTROL_SIZE + 4.0))
            .items_center()
            .gap(DynamicSpacing::Base16.rems(cx))
            .justify_between()
            .flex_none()
            .h_full()
            .py(DynamicSpacing::Base12.px(cx))
            .child(
                v_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "run-all-cells",
                                    IconName::PlayFilled,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Execute all cells", &RunAll, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(RunAll), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "clear-all-outputs",
                                    IconName::ListX,
                                    window,
                                    cx,
                                )
                                .disabled(!has_outputs)
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Clear all outputs", &ClearOutputs, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(ClearOutputs), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-up",
                                    IconName::ArrowUp,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Move cell up", &MoveCellUp, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(MoveCellUp), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "move-cell-down",
                                    IconName::ArrowDown,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Move cell down", &MoveCellDown, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(MoveCellDown), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx)
                            .child(
                                Self::render_notebook_control(
                                    "new-markdown-cell",
                                    IconName::Plus,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Add markdown block", &AddMarkdownBlock, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddMarkdownBlock), cx);
                                }),
                            )
                            .child(
                                Self::render_notebook_control(
                                    "new-code-cell",
                                    IconName::Code,
                                    window,
                                    cx,
                                )
                                .tooltip(move |window, cx| {
                                    Tooltip::for_action("Add code block", &AddCodeBlock, cx)
                                })
                                .on_click(|_, window, cx| {
                                    window.dispatch_action(Box::new(AddCodeBlock), cx);
                                }),
                            ),
                    )
                    .child(
                        Self::button_group(window, cx).child(
                            Self::render_notebook_control(
                                "delete-cell",
                                IconName::Trash,
                                window,
                                cx,
                            )
                            .disabled(self.cell_order.is_empty())
                            .tooltip(move |window, cx| {
                                Tooltip::for_action("Delete cell", &DeleteCell, cx)
                            })
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(DeleteCell), cx);
                            }),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .gap(DynamicSpacing::Base08.rems(cx))
                    .items_center()
                    .child(
                        Self::render_notebook_control("more-menu", IconName::Ellipsis, window, cx)
                            .tooltip(move |window, cx| (Tooltip::text("More options"))(window, cx)),
                    )
                    .child(Self::button_group(window, cx).child({
                        let kernel_status = self.kernel.status();
                        let (icon, icon_color) = match &kernel_status {
                            KernelStatus::Idle => (IconName::ReplNeutral, Color::Success),
                            KernelStatus::Busy => (IconName::ReplNeutral, Color::Warning),
                            KernelStatus::Starting => (IconName::ReplNeutral, Color::Muted),
                            KernelStatus::Error => (IconName::ReplNeutral, Color::Error),
                            KernelStatus::ShuttingDown => (IconName::ReplNeutral, Color::Muted),
                            KernelStatus::Shutdown => (IconName::ReplNeutral, Color::Disabled),
                            KernelStatus::Restarting => (IconName::ReplNeutral, Color::Warning),
                        };
                        let kernel_name = self
                            .kernel_specification
                            .as_ref()
                            .map(|spec| spec.name().to_string())
                            .unwrap_or_else(|| "Select Kernel".to_string());
                        IconButton::new("repl", icon)
                            .icon_color(icon_color)
                            .tooltip(move |window, cx| {
                                Tooltip::text(format!(
                                    "{} ({}). Click to change kernel.",
                                    kernel_name,
                                    kernel_status.to_string()
                                ))(window, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.kernel_picker_handle.toggle(window, cx);
                            }))
                    })),
            )
    }

    fn render_kernel_status_bar(
        &self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let kernel_status = self.kernel.status();
        let kernel_name = self
            .kernel_specification
            .as_ref()
            .map(|spec| spec.name().to_string())
            .unwrap_or_else(|| "Select Kernel".to_string());

        let (status_icon, status_color) = match &kernel_status {
            KernelStatus::Idle => (IconName::Circle, Color::Success),
            KernelStatus::Busy => (IconName::ArrowCircle, Color::Warning),
            KernelStatus::Starting => (IconName::ArrowCircle, Color::Muted),
            KernelStatus::Error => (IconName::XCircle, Color::Error),
            KernelStatus::ShuttingDown => (IconName::ArrowCircle, Color::Muted),
            KernelStatus::Shutdown => (IconName::Circle, Color::Muted),
            KernelStatus::Restarting => (IconName::ArrowCircle, Color::Warning),
        };

        let is_spinning = matches!(
            kernel_status,
            KernelStatus::Busy
                | KernelStatus::Starting
                | KernelStatus::ShuttingDown
                | KernelStatus::Restarting
        );

        let status_icon_element = if is_spinning {
            Icon::new(status_icon)
                .size(IconSize::Small)
                .color(status_color)
                .with_rotate_animation(2)
                .into_any_element()
        } else {
            Icon::new(status_icon)
                .size(IconSize::Small)
                .color(status_color)
                .into_any_element()
        };

        let worktree_id = self.worktree_id;
        let kernel_picker_handle = self.kernel_picker_handle.clone();
        let view = cx.entity().downgrade();

        h_flex()
            .w_full()
            .px_3()
            .py_1()
            .gap_2()
            .items_center()
            .justify_between()
            .bg(cx.theme().colors().status_bar_background)
            .child(
                KernelSelector::new(
                    Box::new(move |spec: KernelSpecification, window, cx| {
                        if let Some(view) = view.upgrade() {
                            view.update(cx, |this, cx| {
                                this.change_kernel(spec, window, cx);
                            });
                        }
                    }),
                    worktree_id,
                    Button::new("kernel-selector", kernel_name.clone())
                        .label_size(LabelSize::Small)
                        .start_icon(
                            Icon::new(status_icon)
                                .size(IconSize::Small)
                                .color(status_color),
                        ),
                    Tooltip::text(format!(
                        "Kernel: {} ({}). Click to change.",
                        kernel_name,
                        kernel_status.to_string()
                    )),
                )
                .with_handle(kernel_picker_handle),
            )
            .child(
                Label::new(self.save_status(cx))
                    .size(LabelSize::Small)
                    .color(if self.is_dirty(cx) {
                        Color::Warning
                    } else {
                        Color::Muted
                    }),
            )
            .child(
                Button::new("restore-deleted-cell", "Restore deleted cell")
                    .style(ButtonStyle::Subtle)
                    .label_size(LabelSize::Small)
                    .start_icon(Icon::new(IconName::Undo).size(IconSize::Small))
                    .disabled(self.deleted_cells.is_empty())
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.restore_deleted_cell(&RestoreDeletedCell, window, cx)
                    })),
            )
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        IconButton::new("restart-kernel", IconName::RotateCw)
                            .icon_size(IconSize::Small)
                            .tooltip(|window, cx| {
                                Tooltip::for_action("Restart Kernel", &RestartKernel, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.restart_kernel(&RestartKernel, window, cx);
                            })),
                    )
                    .child(
                        IconButton::new("interrupt-kernel", IconName::Stop)
                            .icon_size(IconSize::Small)
                            .disabled(!matches!(kernel_status, KernelStatus::Busy))
                            .tooltip(|window, cx| {
                                Tooltip::for_action("Interrupt Kernel", &InterruptKernel, cx)
                            })
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.interrupt_kernel(&InterruptKernel, window, cx);
                            })),
                    ),
            )
    }

    fn cell_list(&self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity();
        list(self.cell_list.clone(), move |index, window, cx| {
            view.update(cx, |this, cx| {
                let cell_id = &this.cell_order[index];
                let cell = this.cell_map.get(cell_id).unwrap();
                this.render_cell(index, cell, window, cx).into_any_element()
            })
        })
        .size_full()
    }

    fn render_empty_state(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .items_center()
            .justify_center()
            .gap_3()
            .child(Label::new("This notebook is empty.").color(Color::Muted))
            .child(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("empty-state-add-code", "Add code cell")
                            .start_icon(Icon::new(IconName::Code))
                            .key_binding(KeyBinding::for_action_in(
                                &AddCodeBlock,
                                &self.focus_handle,
                                cx,
                            ))
                            .on_click(
                                cx.listener(|this, _, window, cx| this.add_code_block(window, cx)),
                            ),
                    )
                    .child(
                        Button::new("empty-state-add-markdown", "Add markdown cell")
                            .style(ButtonStyle::Subtle)
                            .start_icon(Icon::new(IconName::FileMarkdown))
                            .key_binding(KeyBinding::for_action_in(
                                &AddMarkdownBlock,
                                &self.focus_handle,
                                cx,
                            ))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.add_markdown_block(window, cx)
                            })),
                    ),
            )
    }

    fn cell_position(&self, index: usize) -> CellPosition {
        match index {
            0 => CellPosition::First,
            index if index == self.cell_count() - 1 => CellPosition::Last,
            _ => CellPosition::Middle,
        }
    }

    fn render_cell(
        &self,
        index: usize,
        cell: &Cell,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let cell_position = self.cell_position(index);

        let is_selected = index == self.selected_cell_index;

        match cell {
            Cell::Code(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
            Cell::Markdown(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
            Cell::Raw(cell) => {
                cell.update(cx, |cell, _cx| {
                    cell.set_selected(is_selected)
                        .set_cell_position(cell_position);
                });
                cell.clone().into_any_element()
            }
        }
    }
}

impl Render for NotebookEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut key_context = KeyContext::new_with_defaults();
        key_context.add("NotebookEditor");
        key_context.set(
            "notebook_mode",
            match self.notebook_mode {
                NotebookMode::Command => "command",
                NotebookMode::Edit => "edit",
            },
        );

        v_flex()
            .size_full()
            .key_context(key_context)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &OpenNotebook, window, cx| {
                this.open_notebook(&OpenNotebook, window, cx)
            }))
            .on_action(
                cx.listener(|this, _: &ClearOutputs, window, cx| this.clear_outputs(window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &Run, window, cx| this.run_current_cell(&Run, window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.run_and_advance(action, window, cx)),
            )
            .on_action(cx.listener(|this, _: &RunAll, window, cx| this.run_cells(window, cx)))
            .on_action(
                cx.listener(|this, _: &MoveCellUp, window, cx| this.move_cell_up(window, cx)),
            )
            .on_action(
                cx.listener(|this, _: &MoveCellDown, window, cx| this.move_cell_down(window, cx)),
            )
            .on_action(cx.listener(|this, _: &AddMarkdownBlock, window, cx| {
                this.add_markdown_block(window, cx)
            }))
            .on_action(
                cx.listener(|this, _: &AddCodeBlock, window, cx| this.add_code_block(window, cx)),
            )
            .on_action(cx.listener(|this, _: &DeleteCell, window, cx| this.delete_cell(window, cx)))
            .on_action(cx.listener(Self::restore_deleted_cell))
            .on_action(
                cx.listener(|this, action, window, cx| this.enter_edit_mode(action, window, cx)),
            )
            .on_action(cx.listener(|this, action, window, cx| {
                this.handle_enter_command_mode(action, window, cx)
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                this.select_next(action, SelectionMode::SelectOnly, window, cx)
            }))
            .on_action(cx.listener(|this, action, window, cx| {
                this.select_previous(action, SelectionMode::SelectOnly, window, cx)
            }))
            .on_action(cx.listener(Self::select_first))
            .on_action(cx.listener(Self::select_last))
            .on_action(cx.listener(|this, _: &MoveDown, window, cx| {
                this.select_next(
                    &Default::default(),
                    SelectionMode::SelectAndMove,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &MoveUp, window, cx| {
                this.select_previous(
                    &Default::default(),
                    SelectionMode::SelectAndMove,
                    window,
                    cx,
                );
            }))
            .on_action(cx.listener(|this, _: &NotebookMoveDown, window, cx| {
                let Some(cell) = this.get_selected_cell() else {
                    return;
                };

                let Some(editor) = cell.editor(cx).cloned() else {
                    return;
                };

                let is_at_last_line = editor.update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    let selections = editor.selections.all_display(&display_snapshot);
                    if let Some(selection) = selections.last() {
                        let head = selection.head();
                        let cursor_row = head.row();
                        let max_row = display_snapshot.max_point().row();

                        cursor_row >= max_row
                    } else {
                        false
                    }
                });

                if is_at_last_line {
                    this.select_next(
                        &Default::default(),
                        SelectionMode::SelectAndMove,
                        window,
                        cx,
                    );
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.move_down(&Default::default(), window, cx);
                    });
                }
            }))
            .on_action(cx.listener(|this, _: &NotebookMoveUp, window, cx| {
                let Some(cell) = this.get_selected_cell() else {
                    return;
                };

                let Some(editor) = cell.editor(cx).cloned() else {
                    return;
                };

                let is_at_first_line = editor.update(cx, |editor, cx| {
                    let display_snapshot = editor.display_snapshot(cx);
                    let selections = editor.selections.all_display(&display_snapshot);
                    if let Some(selection) = selections.first() {
                        let head = selection.head();
                        let cursor_row = head.row();

                        cursor_row.0 == 0
                    } else {
                        false
                    }
                });

                if is_at_first_line {
                    this.select_previous(
                        &Default::default(),
                        SelectionMode::SelectAndMove,
                        window,
                        cx,
                    );
                } else {
                    editor.update(cx, |editor, cx| {
                        editor.move_up(&Default::default(), window, cx);
                    });
                }
            }))
            .on_action(
                cx.listener(|this, action, window, cx| this.restart_kernel(action, window, cx)),
            )
            .on_action(
                cx.listener(|this, action, window, cx| this.interrupt_kernel(action, window, cx)),
            )
            .child(
                h_flex()
                    .flex_1()
                    .w_full()
                    .h_full()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .child(if self.cell_order.is_empty() {
                                self.render_empty_state(cx).into_any_element()
                            } else {
                                self.cell_list(window, cx).into_any_element()
                            }),
                    )
                    .child(self.render_notebook_controls(window, cx)),
            )
            .child(self.render_kernel_status_bar(window, cx))
    }
}

impl Focusable for NotebookEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

fn document_with_ids(text: &str, notebook: &nbformat::v4::Notebook) -> Result<serde_json::Value> {
    let mut original = if text.trim().is_empty() {
        serde_json::to_value(notebook)?
    } else {
        serde_json::from_str::<serde_json::Value>(text)?
    };
    if original["nbformat"] != 4 {
        original = serde_json::to_value(notebook)?;
    }
    if let Some(cells) = original["cells"].as_array_mut() {
        for (cell, typed_cell) in cells.iter_mut().zip(&notebook.cells) {
            cell["id"] = serde_json::Value::String(typed_cell.id().to_string());
        }
    }
    Ok(original)
}

// Intended to be a NotebookBuffer
pub struct NotebookItem {
    project_path: ProjectPath,
    languages: Arc<LanguageRegistry>,
    // Raw notebook data
    notebook: nbformat::v4::Notebook,
    original_json: serde_json::Value,
    original_text: String,
    // Store our version of the notebook in memory (cell_order, cell_map)
    id: ProjectEntryId,
}

impl project::ProjectItem for NotebookItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        let path = path.clone();
        let project = project.clone();
        let languages = project.read(cx).languages().clone();

        // For single-file worktrees the relative path is empty, so fall back
        // to the absolute path to detect notebooks opened directly.
        let abs_path = project.read(cx).absolute_path(&path, cx);
        let is_notebook = path.path.extension().unwrap_or_default() == NOTEBOOK_EXTENSION
            || abs_path
                .as_ref()
                .and_then(|abs_path| abs_path.extension())
                .is_some_and(|extension| extension == NOTEBOOK_EXTENSION);

        if is_notebook {
            Some(cx.spawn(async move |cx| {
                // todo: watch for changes to the file
                let buffer = project
                    .update(cx, |project, cx| project.open_buffer(path.clone(), cx))
                    .await?;
                let file_content = buffer.read_with(cx, |buffer, _| buffer.text());

                let notebook = if file_content.trim().is_empty() {
                    nbformat::v4::Notebook {
                        nbformat: 4,
                        nbformat_minor: 5,
                        cells: vec![],
                        metadata: serde_json::from_str("{}").unwrap(),
                    }
                } else {
                    let notebook = match nbformat::parse_notebook(&file_content) {
                        Ok(nb) => nb,
                        Err(_) => {
                            // Pre-process to ensure IDs exist
                            let mut json: serde_json::Value = serde_json::from_str(&file_content)?;
                            if let Some(cells) =
                                json.get_mut("cells").and_then(|c| c.as_array_mut())
                            {
                                for cell in cells {
                                    if cell.get("id").is_none() {
                                        cell["id"] =
                                            serde_json::Value::String(Uuid::new_v4().to_string());
                                    }
                                }
                            }
                            let file_content = serde_json::to_string(&json)?;
                            nbformat::parse_notebook(&file_content)?
                        }
                    };

                    match notebook {
                        nbformat::Notebook::V4(notebook) => notebook,
                        // 4.1 - 4.4 are converted to 4.5
                        nbformat::Notebook::Legacy(legacy_notebook) => {
                            // TODO: Decide if we want to mutate the notebook by including Cell IDs
                            // and any other conversions

                            nbformat::upgrade_legacy_notebook(legacy_notebook)?
                        }
                        nbformat::Notebook::V3(v3_notebook) => {
                            nbformat::upgrade_v3_notebook(v3_notebook)?
                        }
                    }
                };

                let id = project
                    .update(cx, |project, cx| {
                        project.entry_for_path(&path, cx).map(|entry| entry.id)
                    })
                    .context("Entry not found")?;

                let original_json = document_with_ids(&file_content, &notebook)?;
                Ok(cx.new(|_| NotebookItem {
                    original_json,
                    original_text: file_content,
                    project_path: path,
                    languages,
                    notebook,
                    id,
                }))
            }))
        } else {
            None
        }
    }

    fn entry_id(&self, _: &App) -> Option<ProjectEntryId> {
        Some(self.id)
    }

    fn project_path(&self, _: &App) -> Option<ProjectPath> {
        Some(self.project_path.clone())
    }

    fn is_dirty(&self) -> bool {
        // TODO: Track if notebook metadata or structure has changed
        false
    }
}

impl NotebookItem {
    pub fn language_name(&self) -> Option<String> {
        self.notebook
            .metadata
            .language_info
            .as_ref()
            .map(|l| l.name.clone())
            .or(self
                .notebook
                .metadata
                .kernelspec
                .as_ref()
                .and_then(|spec| spec.language.clone()))
    }

    pub fn notebook_language(&self) -> impl Future<Output = Option<Arc<Language>>> + use<> {
        let language_name = self.language_name();
        let languages = self.languages.clone();

        async move {
            if let Some(language_name) = language_name {
                languages.language_for_name(&language_name).await.ok()
            } else {
                None
            }
        }
    }
}

impl EventEmitter<()> for NotebookItem {}

impl EventEmitter<ItemEvent> for NotebookEditor {}
impl EventEmitter<SearchEvent> for NotebookEditor {}

// pub struct NotebookControls {
//     pane_focused: bool,
//     active_item: Option<Box<dyn ItemHandle>>,
//     // subscription: Option<Subscription>,
// }

// impl NotebookControls {
//     pub fn new() -> Self {
//         Self {
//             pane_focused: false,
//             active_item: Default::default(),
//             // subscription: Default::default(),
//         }
//     }
// }

// impl EventEmitter<ToolbarItemEvent> for NotebookControls {}

// impl Render for NotebookControls {
//     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//         div().child("notebook controls")
//     }
// }

// impl ToolbarItemView for NotebookControls {
//     fn set_active_pane_item(
//         &mut self,
//         active_pane_item: Option<&dyn workspace::ItemHandle>,
//         window: &mut Window, cx: &mut Context<Self>,
//     ) -> workspace::ToolbarItemLocation {
//         cx.notify();
//         self.active_item = None;

//         let Some(item) = active_pane_item else {
//             return ToolbarItemLocation::Hidden;
//         };

//         ToolbarItemLocation::PrimaryLeft
//     }

//     fn pane_focus_update(&mut self, pane_focused: bool, _window: &mut Window, _cx: &mut Context<Self>) {
//         self.pane_focused = pane_focused;
//     }
// }

impl SearchableItem for NotebookEditor {
    type Match = NotebookMatch;

    fn supported_options(&self) -> SearchOptions {
        SearchOptions {
            case: true,
            word: true,
            regex: true,
            ..Default::default()
        }
    }

    fn get_matches(&self, _: &mut Window, _: &mut App) -> (Vec<NotebookMatch>, SearchToken) {
        (self.search_matches.clone(), SearchToken::default())
    }

    fn clear_matches(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.search_matches.clear();
        for cell in self.cell_map.values() {
            if let Some(editor) = cell.editor(cx).cloned() {
                editor.update(cx, |editor, cx| editor.clear_matches(window, cx));
            }
        }
        cx.notify();
    }

    fn update_matches(
        &mut self,
        matches: &[NotebookMatch],
        active: Option<usize>,
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.search_matches = matches.to_vec();
        let mut by_cell: HashMap<_, Vec<_>> = HashMap::default();
        for (index, found) in matches.iter().enumerate() {
            by_cell
                .entry(&found.cell_id)
                .or_default()
                .push((index, found));
        }
        for (id, cell) in &self.cell_map {
            if let Some(editor) = cell.editor(cx).cloned() {
                let local = by_cell.remove(id).unwrap_or_default();
                let active = local.iter().position(|(i, _)| Some(*i) == active);
                let ranges: Vec<_> = local.iter().map(|(_, m)| m.range.clone()).collect();
                editor.update(cx, |editor, cx| {
                    editor.update_matches(&ranges, active, token, window, cx)
                });
            }
        }
        cx.notify();
    }

    fn query_suggestion(
        &mut self,
        seed: Option<settings::SeedQuerySetting>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> String {
        let editor = self
            .cell_order
            .get(self.selected_cell_index)
            .and_then(|id| self.cell_map.get(id))
            .and_then(|cell| cell.editor(cx))
            .cloned();
        editor
            .map(|editor| editor.update(cx, |editor, cx| editor.query_suggestion(seed, window, cx)))
            .unwrap_or_default()
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[NotebookMatch],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(hit) = matches.get(index) else {
            return;
        };
        let Some(cell_index) = self.cell_order.iter().position(|id| id == &hit.cell_id) else {
            return;
        };
        self.selected_cell_index = cell_index;
        self.cell_list.scroll_to_reveal_item(cell_index);
        if let Some(Cell::Markdown(cell)) = self.cell_map.get(&hit.cell_id) {
            cell.update(cx, |cell, cx| {
                cell.set_editing(true);
                cx.notify();
            });
        }
        if let Some(editor) = self
            .cell_map
            .get(&hit.cell_id)
            .and_then(|cell| cell.editor(cx))
            .cloned()
        {
            editor.update(cx, |editor, cx| {
                editor.activate_match(0, &[hit.range.clone()], token, window, cx)
            });
        }
        cx.notify();
    }

    fn select_matches(
        &mut self,
        _: &[NotebookMatch],
        _: SearchToken,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
    }
    fn replace(
        &mut self,
        _: &NotebookMatch,
        _: &SearchQuery,
        _: SearchToken,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
    }

    fn find_matches(
        &mut self,
        query: Arc<SearchQuery>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Vec<NotebookMatch>> {
        let tasks: Vec<_> = self
            .cell_order
            .iter()
            .filter_map(|id| {
                let editor = self.cell_map.get(id)?.editor(cx)?.clone();
                Some((
                    id.clone(),
                    editor.update(cx, |editor, cx| {
                        editor.find_matches(query.clone(), window, cx)
                    }),
                ))
            })
            .collect();
        cx.spawn(async move |_, _| {
            let cells =
                futures::future::join_all(tasks.into_iter().map(|(cell_id, task)| async move {
                    task.await
                        .into_iter()
                        .map(|range| NotebookMatch {
                            cell_id: cell_id.clone(),
                            range,
                        })
                        .collect::<Vec<_>>()
                }))
                .await;
            cells.into_iter().flatten().collect()
        })
    }

    fn active_match_index(
        &mut self,
        direction: Direction,
        matches: &[NotebookMatch],
        token: SearchToken,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<usize> {
        if matches.is_empty() {
            return None;
        }
        if let Some(id) = self.cell_order.get(self.selected_cell_index) {
            let local: Vec<_> = matches
                .iter()
                .enumerate()
                .filter(|(_, m)| &m.cell_id == id)
                .collect();
            if let Some(editor) = self
                .cell_map
                .get(id)
                .and_then(|cell| cell.editor(cx))
                .cloned()
            {
                let ranges: Vec<_> = local.iter().map(|(_, m)| m.range.clone()).collect();
                if let Some(index) = editor.update(cx, |editor, cx| {
                    editor.active_match_index(direction, &ranges, token, window, cx)
                }) {
                    return Some(local[index].0);
                }
            }
        }
        let cell_index = |m: &NotebookMatch| self.cell_order.iter().position(|id| id == &m.cell_id);
        match direction {
            Direction::Next => Some(
                matches
                    .iter()
                    .position(|m| cell_index(m).is_some_and(|i| i >= self.selected_cell_index))
                    .unwrap_or(0),
            ),
            Direction::Prev => Some(
                matches
                    .iter()
                    .rposition(|m| cell_index(m).is_some_and(|i| i <= self.selected_cell_index))
                    .unwrap_or(matches.len() - 1),
            ),
        }
    }
}

impl Item for NotebookEditor {
    type Event = ItemEvent;

    fn to_item_events(event: &Self::Event, f: &mut dyn FnMut(ItemEvent)) {
        f(*event);
    }

    fn can_split(&self) -> bool {
        true
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<workspace::WorkspaceId>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Option<Entity<Self>>>
    where
        Self: Sized,
    {
        Task::ready(Some(cx.new(|cx| {
            Self::new(self.project.clone(), self.notebook_item.clone(), window, cx)
        })))
    }

    fn buffer_kind(&self, _: &App) -> workspace::item::ItemBufferKind {
        workspace::item::ItemBufferKind::Singleton
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        f(self.notebook_item.entity_id(), self.notebook_item.read(cx))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.notebook_item
            .read(cx)
            .project_path
            .path
            .file_name()
            .map(|s| s.to_string())
            .unwrap_or_default()
            .into()
    }

    fn tab_content(&self, params: TabContentParams, window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or(0), cx))
            .single_line()
            .color(params.text_color())
            .when(params.preview, |this| this.italic())
            .into_any_element()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(IconName::Book.into())
    }

    fn show_toolbar(&self) -> bool {
        true
    }

    // TODO
    fn pixel_position_of_cursor(&self, _: &App) -> Option<Point<Pixels>> {
        None
    }

    // TODO
    fn as_searchable(
        &self,
        handle: &Entity<Self>,
        _: &App,
    ) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn set_nav_history(
        &mut self,
        _: workspace::ItemNavHistory,
        _window: &mut Window,
        _: &mut Context<Self>,
    ) {
        // TODO
    }

    fn can_save(&self, _cx: &App) -> bool {
        true
    }

    fn save(
        &mut self,
        _options: SaveOptions,
        project: Entity<Project>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.save_impl(SaveDestination::CurrentPath, project, cx)
    }

    fn save_as(
        &mut self,
        project: Entity<Project>,
        path: ProjectPath,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.save_impl(SaveDestination::NewPath(path), project, cx)
    }

    fn reload(
        &mut self,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        let project_path = self.notebook_item.read(cx).project_path.clone();
        let languages = self.languages.clone();
        let notebook_language = self.notebook_language.clone();

        cx.spawn_in(window, async move |this, cx| {
            let buffer = this
                .update(cx, |this, cx| {
                    this.project
                        .update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            let file_content = buffer.read_with(cx, |buffer, _| buffer.text());

            let mut json: serde_json::Value = serde_json::from_str(&file_content)?;
            if let Some(cells) = json.get_mut("cells").and_then(|c| c.as_array_mut()) {
                for cell in cells {
                    if cell.get("id").is_none() {
                        cell["id"] = serde_json::Value::String(Uuid::new_v4().to_string());
                    }
                }
            }
            let file_content = serde_json::to_string(&json)?;

            let notebook = nbformat::parse_notebook(&file_content);
            let notebook = match notebook {
                Ok(nbformat::Notebook::V4(notebook)) => notebook,
                Ok(nbformat::Notebook::Legacy(legacy_notebook)) => {
                    nbformat::upgrade_legacy_notebook(legacy_notebook)?
                }
                Ok(nbformat::Notebook::V3(v3_notebook)) => {
                    nbformat::upgrade_v3_notebook(v3_notebook)?
                }
                Err(e) => {
                    anyhow::bail!("Failed to parse notebook: {:?}", e);
                }
            };

            let original_json = document_with_ids(&file_content, &notebook)?;
            this.update_in(cx, |this, window, cx| {
                this.saved_metadata = serde_json::to_value(&notebook.metadata).unwrap_or_default();
                this.notebook_item.update(cx, |item, cx| {
                    item.original_json = original_json;
                    item.original_text = buffer.read(cx).text();
                    item.notebook = notebook.clone();
                });
                this.replace_cells(&notebook, window, cx);
                this.original_cell_order = this.cell_order.clone();
                this.recovery_dirty = false;
                this.clear_recovery(cx);
                cx.emit(ItemEvent::UpdateTab);
                cx.emit(SearchEvent::MatchesInvalidated);
                cx.notify();
            })?;

            Ok(())
        })
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.has_structural_changes() || self.has_content_changes(cx)
    }
}

impl ProjectItem for NotebookEditor {
    type Item = NotebookItem;

    fn for_project_item(
        project: Entity<Project>,
        _pane: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(project, item, window, cx)
    }
}

impl KernelSession for NotebookEditor {
    fn route(&mut self, message: &JupyterMessage, window: &mut Window, cx: &mut Context<Self>) {
        if let JupyterMessageContent::CompleteReply(reply) = &message.content {
            if let Some(parent) = &message.parent_header {
                if let Some(sender) = self.completion_requests.remove(&parent.msg_id) {
                    let _ = sender.send(reply.clone());
                }
            }
            return;
        }
        // Display IDs can be shared across cells, and updates may be emitted
        // under a different execution request. Update every matching output.
        if let JupyterMessageContent::UpdateDisplayData(update) = &message.content {
            let mut changed = false;
            for cell in self.cell_map.values() {
                if let Cell::Code(cell) = cell {
                    changed |=
                        cell.update(cx, |cell, cx| cell.update_display_data(update, window, cx));
                }
            }
            if changed {
                self.notebook_state_changed(cx);
            }
            return;
        }
        // Handle kernel status updates (these are broadcast to all)
        if let JupyterMessageContent::Status(status) = &message.content {
            self.kernel.set_execution_state(&status.execution_state);
            cx.notify();
        }

        if let JupyterMessageContent::KernelInfoReply(reply) = &message.content {
            self.kernel.set_kernel_info(reply);

            if let Ok(language_info) = serde_json::from_value::<nbformat::v4::LanguageInfo>(
                serde_json::to_value(&reply.language_info).unwrap(),
            ) {
                self.notebook_item.update(cx, |item, cx| {
                    item.notebook.metadata.language_info = Some(language_info);
                    cx.emit(());
                });
            }
            cx.notify();
        }

        // Handle cell-specific messages
        if let Some(parent_header) = &message.parent_header {
            if let Some(cell_id) = self.execution_requests.get(&parent_header.msg_id) {
                if let Some(Cell::Code(cell)) = self.cell_map.get(cell_id) {
                    cell.update(cx, |cell, cx| {
                        cell.handle_message(message, window, cx);
                    });
                    self.notebook_state_changed(cx);
                }
            }
        }
    }

    fn kernel_errored(&mut self, error_message: String, cx: &mut Context<Self>) {
        self.kernel = Kernel::ErroredLaunch(error_message);
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project, ProjectItem as _};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use util::rel_path::rel_path;

    const NOTEBOOK_WITH_ONE_CODE_CELL: &str = r#"{
        "metadata": {
            "kernelspec": {
                "display_name": "Python 3",
                "language": "python",
                "name": "python3"
            },
            "language_info": {
                "name": "python"
            }
        },
        "nbformat": 4,
        "nbformat_minor": 5,
        "cells": [
            {
                "cell_type": "code",
                "id": "cell-one",
                "metadata": {},
                "execution_count": null,
                "outputs": [],
                "source": ["print('hello')"]
            }
        ]
    }"#;

    async fn notebook_feature_fixture(
        cx: &mut TestAppContext,
    ) -> (Entity<Project>, Entity<NotebookItem>) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
        let mut notebook: serde_json::Value =
            serde_json::from_str(NOTEBOOK_WITH_ONE_CODE_CELL).unwrap();
        notebook["cells"][0]["source"] = json!(["needle = 1\nprint(needle)"]);
        notebook["cells"][0]["outputs"] = json!([{"output_type":"stream","name":"stdout","text":["needle in preserved output\n"]}]);
        notebook["cells"].as_array_mut().unwrap().push(json!({"cell_type":"markdown","id":"markdown-note","metadata":{},"source":["A needle note"]}));
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "test.ipynb": serde_json::to_string(&notebook).unwrap() }),
        )
        .await;
        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs, cx));
        let path = project.read_with(cx, |project, cx| ProjectPath {
            worktree_id: project.worktrees(cx).next().unwrap().read(cx).id(),
            path: rel_path("test.ipynb").into(),
        });
        let item = cx
            .update(|cx| NotebookItem::try_open(&project, &path, cx).unwrap())
            .await
            .unwrap();
        (project, item)
    }

    fn feature_editor(
        project: Entity<Project>,
        item: Entity<NotebookItem>,
        window: &mut Window,
        cx: &mut Context<NotebookEditor>,
    ) -> NotebookEditor {
        let mut notebook = NotebookEditor::new(project, item, window, cx);
        // These tests exercise editing and presentation, so cancel kernel startup
        // before it probes real TCP ports or starts an interpreter.
        notebook.kernel = Kernel::Shutdown;
        notebook
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_search_navigates_code_and_markdown(cx: &mut TestAppContext) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project, item, window, cx)));
        let query = Arc::new(
            SearchQuery::text(
                "needle",
                false,
                false,
                false,
                Default::default(),
                Default::default(),
                false,
                None,
            )
            .unwrap(),
        );
        let matches = notebook
            .update_in(cx, |notebook, window, cx| {
                notebook.find_matches(query, window, cx)
            })
            .await;
        assert_eq!(
            matches.len(),
            3,
            "search code and markdown source, excluding outputs"
        );
        assert_eq!(matches[0].cell_id, matches[1].cell_id);
        assert_ne!(matches[1].cell_id, matches[2].cell_id);
        notebook.update_in(cx, |notebook, window, cx| {
            let token = SearchToken::default();
            notebook.update_matches(&matches, Some(2), token, window, cx);
            notebook.activate_match(2, &matches, token, window, cx);
            assert_eq!(notebook.selected_cell_index, 1);
            assert_eq!(
                notebook.active_match_index(Direction::Next, &matches, token, window, cx),
                Some(2)
            );
            let next = notebook.match_index_for_direction(
                &matches,
                2,
                Direction::Next,
                1,
                token,
                window,
                cx,
            );
            assert_eq!(next, 0, "navigation wraps to the first code cell");
            let previous = notebook.match_index_for_direction(
                &matches,
                0,
                Direction::Prev,
                1,
                token,
                window,
                cx,
            );
            assert_eq!(previous, 2);
            let Cell::Markdown(cell) = notebook.cell_map.get(&matches[2].cell_id).unwrap() else {
                panic!("expected Markdown")
            };
            assert!(cell.read(cx).is_editing());
            assert_eq!(notebook.get_matches(window, cx).0.len(), 3);
            notebook.clear_matches(window, cx);
            assert!(notebook.get_matches(window, cx).0.is_empty());
        });
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_collapsing_preserves_outputs_and_dirty_state(
        cx: &mut TestAppContext,
    ) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project, item, window, cx)));
        let before =
            notebook.read_with(cx, |notebook, cx| notebook.serialized_notebook(cx).unwrap());
        let dirty_before = notebook.read_with(cx, |notebook, cx| notebook.is_dirty(cx));
        let cell = notebook.read_with(cx, |notebook, _| {
            match notebook.cell_map.get(&notebook.cell_order[0]).unwrap() {
                Cell::Code(cell) => cell.clone(),
                _ => panic!("expected code"),
            }
        });
        cell.update(cx, |cell, cx| {
            assert!(cell.has_outputs());
            cell.toggle_outputs(cx);
            assert!(cell.outputs_collapsed);
        });
        notebook.read_with(cx, |notebook, cx| {
            assert_eq!(notebook.serialized_notebook(cx).unwrap(), before);
            assert_eq!(notebook.is_dirty(cx), dirty_before);
        });
        cell.update(cx, |cell, cx| {
            cell.toggle_outputs(cx);
            assert!(!cell.outputs_collapsed);
        });
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_dirty_indicator_tracks_edits_and_save(cx: &mut TestAppContext) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project.clone(), item, window, cx)));
        let events = Arc::new(std::sync::Mutex::new(Vec::new()));
        let captured_events = events.clone();
        let _subscription = cx.update(|_, cx| {
            cx.subscribe(&notebook, move |_, event: &ItemEvent, _| {
                captured_events.lock().unwrap().push(*event);
            })
        });
        let cell_editor = notebook.read_with(cx, |notebook, cx| {
            notebook
                .cell_map
                .get(&notebook.cell_order[0])
                .unwrap()
                .editor(cx)
                .unwrap()
                .clone()
        });
        cell_editor.update_in(cx, |editor, window, cx| {
            editor.set_text("changed = True", window, cx)
        });
        notebook.read_with(cx, |notebook, cx| {
            assert_eq!(notebook.save_status(cx), "Unsaved changes")
        });
        {
            let events = events.lock().unwrap();
            assert!(events.iter().any(|event| matches!(event, ItemEvent::Edit)));
            assert!(
                events
                    .iter()
                    .any(|event| matches!(event, ItemEvent::UpdateTab))
            );
        }
        notebook
            .update_in(cx, |notebook, window, cx| {
                notebook.save(SaveOptions::default(), project, window, cx)
            })
            .await
            .unwrap();
        notebook.read_with(cx, |notebook, cx| {
            assert_eq!(notebook.save_status(cx), "Saved")
        });
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_clear_and_display_update_protocol(cx: &mut TestAppContext) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project, item, window, cx)));
        let cell = notebook.read_with(cx, |notebook, _| {
            match notebook.cell_map.get(&notebook.cell_order[0]).unwrap() {
                Cell::Code(cell) => cell.clone(),
                _ => unreachable!(),
            }
        });
        cell.update_in(cx, |cell, window, cx| {
            cell.handle_message(
                &jupyter_protocol::ClearOutput { wait: true }.into(),
                window,
                cx,
            );
            assert_eq!(
                cell.raw_outputs.len(),
                1,
                "wait keeps the previous output until replacement arrives"
            );
            let display: jupyter_protocol::DisplayData = serde_json::from_value(json!({
                "data":{"text/plain":"before"}, "metadata":{}, "transient":{"display_id":"shared"}
            }))
            .unwrap();
            cell.handle_message(&display.into(), window, cx);
            assert_eq!(cell.raw_outputs.len(), 1);
        });
        notebook.update_in(cx, |notebook, window, cx| {
            let update: jupyter_protocol::UpdateDisplayData = serde_json::from_value(json!({
                "data":{"text/plain":"after"}, "metadata":{"custom":true}, "transient":{"display_id":"shared"}
            })).unwrap();
            notebook.route(&update.into(), window, cx);
            let saved: serde_json::Value = serde_json::from_str(&notebook.serialized_notebook(cx).unwrap()).unwrap();
            assert_eq!(saved["cells"][0]["outputs"][0]["data"]["text/plain"], "after");
            assert_eq!(saved["cells"][0]["outputs"][0]["metadata"]["custom"], true);
        });
        cell.update_in(cx, |cell, window, cx| {
            cell.handle_message(
                &jupyter_protocol::ClearOutput { wait: false }.into(),
                window,
                cx,
            );
            assert!(!cell.has_outputs());
            assert!(cell.raw_outputs.is_empty());
        });
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_restores_deleted_cell_and_writes_recovery(
        cx: &mut TestAppContext,
    ) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let fs = project.read_with(cx, |project, _| project.fs().clone());
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project, item, window, cx)));
        let (path, before) = notebook.read_with(cx, |notebook, cx| {
            (
                notebook.recovery_path(cx).unwrap(),
                notebook.serialized_notebook(cx).unwrap(),
            )
        });
        cx.run_until_parked();
        notebook.update_in(cx, |notebook, window, cx| {
            notebook.delete_cell(window, cx);
            assert_eq!(notebook.cell_order.len(), 1);
        });
        cx.run_until_parked();
        cx.executor()
            .advance_clock(std::time::Duration::from_secs(2));
        cx.run_until_parked();
        let record = fs
            .load(&path)
            .await
            .expect("unsaved changes should have a recovery snapshot");
        let draft: serde_json::Value = serde_json::from_str(&record).unwrap();
        let restored: serde_json::Value =
            serde_json::from_str(draft["draft_text"].as_str().unwrap()).unwrap();
        assert_eq!(restored["cells"].as_array().unwrap().len(), 1);
        notebook.update_in(cx, |notebook, window, cx| {
            notebook.restore_deleted_cell(&RestoreDeletedCell, window, cx);
            assert_eq!(
                notebook.serialized_notebook(cx).unwrap(),
                before,
                "restore keeps source and outputs"
            );
        });
        cx.run_until_parked();
        assert!(
            fs.metadata(&path).await.unwrap().is_none(),
            "clean undo removes obsolete recovery"
        );
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_completion_reply_does_not_change_content(cx: &mut TestAppContext) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project, item, window, cx)));
        let request: JupyterMessage = jupyter_protocol::CompleteRequest {
            code: "str.up".into(),
            cursor_pos: 6,
        }
        .into();
        let (sender, receiver) = futures::channel::oneshot::channel();
        notebook.update_in(cx, |notebook, window, cx| {
            let before = notebook.serialized_notebook(cx).unwrap();
            notebook
                .completion_requests
                .insert(request.header.msg_id.clone(), sender);
            let mut reply: JupyterMessage = jupyter_protocol::CompleteReply {
                matches: vec!["upper".into()],
                ..Default::default()
            }
            .into();
            reply.parent_header = Some(request.header.clone());
            notebook.route(&reply, window, cx);
            assert_eq!(notebook.serialized_notebook(cx).unwrap(), before);
            assert!(notebook.completion_requests.is_empty());
        });
        assert_eq!(receiver.await.unwrap().matches, vec!["upper"]);
    }

    #[gpui::test]
    async fn no_ai_fork_notebook_recovers_despite_kernel_metadata_changes(cx: &mut TestAppContext) {
        let (project, item) = notebook_feature_fixture(cx).await;
        let fs = project.read_with(cx, |project, _| project.fs().clone());
        let original = item.read_with(cx, |item, _| item.original_text.clone());
        let mut draft: serde_json::Value = serde_json::from_str(&original).unwrap();
        draft["cells"][0]["source"] = json!(["recovered_value = 42"]);
        let draft = serde_json::to_string(&draft).unwrap();
        let record = notebook_safety::recovery_record(&original, &draft).unwrap();
        let cx = cx.add_empty_window();
        let notebook =
            cx.update(|window, cx| cx.new(|cx| feature_editor(project, item.clone(), window, cx)));
        cx.run_until_parked();
        let path = notebook.read_with(cx, |notebook, cx| notebook.recovery_path(cx).unwrap());
        fs.create_dir(path.parent().unwrap()).await.unwrap();
        fs.atomic_write(path.clone(), record.clone()).await.unwrap();
        notebook.update_in(cx, |notebook, window, cx| {
            notebook.recovery_pending = true;
            notebook.offer_recovery(window, cx);
        });
        cx.run_until_parked();
        assert!(cx.has_pending_prompt());
        // Simulate the kernel_info_reply that used to make recovery refuse its draft.
        item.update(cx, |item, cx| {
            item.notebook.metadata.kernelspec = None;
            cx.emit(());
        });
        cx.run_until_parked();
        cx.executor()
            .advance_clock(std::time::Duration::from_secs(2));
        cx.run_until_parked();
        assert_eq!(
            fs.load(&path).await.unwrap(),
            record,
            "startup must not overwrite a pending draft"
        );
        cx.simulate_prompt_answer("Recover");
        cx.run_until_parked();
        notebook.read_with(cx, |notebook, cx| {
            let recovered: serde_json::Value =
                serde_json::from_str(&notebook.serialized_notebook(cx).unwrap()).unwrap();
            assert_eq!(
                recovered["cells"][0]["source"],
                json!(["recovered_value = 42"])
            );
            assert!(notebook.is_dirty(cx));
            assert!(!notebook.recovery_pending);
        });
        assert_eq!(
            fs.load(path!("/notebooks/test.ipynb").as_ref())
                .await
                .unwrap(),
            original,
            "recovery does not overwrite disk before Save"
        );
    }

    /// When the configured interpreter doesn't exist (e.g. Python isn't installed),
    /// running a cell must not leave it stuck in the executing state. It should
    /// instead surface the kernel launch error as an error output on the cell.
    #[gpui::test]
    async fn test_run_cell_with_missing_interpreter_shows_error(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "test.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });

        // Select a kernel whose interpreter doesn't exist, simulating a machine
        // where Python isn't installed properly. This is the same path the
        // kernel picker uses.
        let missing_interpreter = path!("/nonexistent/python3");
        let broken_spec = KernelSpecification::Jupyter(LocalKernelSpecification {
            name: "python3".to_string(),
            path: PathBuf::from(missing_interpreter),
            kernelspec: JupyterKernelspec {
                argv: vec![
                    missing_interpreter.to_string(),
                    "-m".to_string(),
                    "ipykernel_launcher".to_string(),
                    "-f".to_string(),
                    "{connection_file}".to_string(),
                ],
                display_name: "Python 3".to_string(),
                language: "python".to_string(),
                interrupt_mode: None,
                metadata: None,
                env: None,
            },
        });
        cx.update(|cx| {
            ReplStore::global(cx).update(cx, |store, cx| {
                store.set_active_kernelspec(worktree_id, broken_spec, cx);
            })
        });

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(
                    &project,
                    &ProjectPath {
                        worktree_id,
                        path: rel_path("test.ipynb").into(),
                    },
                    cx,
                )
                .expect("ipynb files should be openable as notebooks")
            })
            .await
            .expect("notebook should parse");

        // Don't render the notebook UI itself: its animated kernel status icon
        // schedules a new frame on every render, which makes `run_until_parked`
        // spin forever in tests. The editor entity is created inside an empty
        // window instead; we are testing execution behavior, not rendering.
        let cx = cx.add_empty_window();

        // Launching a kernel probes real TCP ports on localhost, which the
        // deterministic test scheduler cannot drive.
        cx.executor().allow_parking();

        let editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });

        // Creating the editor launches the kernel. Wait for the actual launch
        // task, which fails because the interpreter cannot be spawned.
        let pending_kernel = editor.read_with(cx, |editor, _| match &editor.kernel {
            Kernel::StartingKernel(task) => task.clone(),
            _ => panic!("kernel should be starting right after the editor is created"),
        });
        pending_kernel.await;

        editor.read_with(cx, |editor, _| {
            assert!(
                matches!(editor.kernel, Kernel::ErroredLaunch(_)),
                "kernel launch should fail, instead status is: {}",
                editor.kernel.status().to_string()
            );
        });

        // Run the (only) cell via the production action handler.
        editor.update_in(cx, |editor, window, cx| {
            editor.run_current_cell(&Run, window, cx);
        });

        editor.read_with(cx, |editor, cx| {
            let cell_id = editor.cell_order.first().expect("notebook has one cell");
            let Some(Cell::Code(cell)) = editor.cell_map.get(cell_id) else {
                panic!("expected a code cell");
            };
            let cell = cell.read(cx);

            assert!(
                !cell.is_executing(),
                "cell must not be stuck in the executing state when the kernel is not running"
            );

            let nbformat::v4::Cell::Code { outputs, .. } = cell.to_nbformat_cell(cx) else {
                panic!("expected a code cell");
            };
            match outputs.as_slice() {
                [nbformat::v4::Output::Error(error)] => {
                    assert_eq!(error.ename, "Kernel Error");
                    let traceback = error.traceback.join("\n");
                    assert!(
                        traceback.contains("the kernel failed to launch"),
                        "error output should explain why the cell could not run, got: {traceback}"
                    );
                }
                other => panic!("expected a single error output, got: {other:?}"),
            }
        });
    }

    /// Opening a notebook as a single file (its own worktree) leaves the
    /// worktree-relative path empty, so only the absolute path carries the
    /// `.ipynb` extension. `try_open` must still recognize it as a notebook.
    #[gpui::test]
    async fn test_open_single_file_notebook(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "single.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL }),
        )
        .await;

        let project =
            Project::test(fs.clone(), [path!("/notebooks/single.ipynb").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let project_path = project.read_with(cx, |project, cx| {
            let worktree = project.worktrees(cx).next().unwrap();
            let worktree = worktree.read(cx);
            assert!(
                worktree.is_single_file(),
                "opening a bare .ipynb should create a single-file worktree"
            );
            ProjectPath {
                worktree_id: worktree.id(),
                path: worktree.root_entry().unwrap().path.clone(),
            }
        });

        assert!(
            project_path.path.extension().is_none(),
            "single-file worktree relative path should have no extension"
        );

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(&project, &project_path, cx)
                    .expect("single-file .ipynb should open as a notebook")
            })
            .await
            .expect("notebook should parse");

        notebook_item.read_with(cx, |item, _| {
            assert_eq!(item.notebook.cells.len(), 1);
        });
    }

    /// Notebooks must be saved through the project rather than through the
    /// client's own filesystem, otherwise a remote notebook's path is resolved
    /// against the local machine and the save fails.
    #[gpui::test]
    async fn test_save_goes_through_the_project(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "test.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let project_path = project.read_with(cx, |project, cx| ProjectPath {
            worktree_id: project.worktrees(cx).next().unwrap().read(cx).id(),
            path: rel_path("test.ipynb").into(),
        });

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(&project, &project_path, cx)
                    .expect("ipynb files should be openable as notebooks")
            })
            .await
            .expect("notebook should parse");

        // Held across the save: a save that bypasses the project writes the file
        // behind this buffer's back, leaving it stale.
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })
            .await
            .expect("notebook buffer should open");

        // Rendering the notebook animates the kernel status icon, which makes
        // `run_until_parked` spin forever; only the editor entity is needed here.
        let cx = cx.add_empty_window();
        let notebook_editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });

        let cell_editor = notebook_editor.read_with(cx, |notebook_editor, cx| {
            let cell_id = notebook_editor
                .cell_order
                .first()
                .expect("notebook has one cell");
            let Some(Cell::Code(cell)) = notebook_editor.cell_map.get(cell_id) else {
                panic!("expected a code cell");
            };
            cell.read(cx).editor().clone()
        });
        cell_editor.update_in(cx, |cell_editor, window, cx| {
            cell_editor.set_text("print('goodbye')", window, cx);
        });

        notebook_editor
            .update_in(cx, |notebook_editor, window, cx| {
                notebook_editor.save(SaveOptions::default(), project.clone(), window, cx)
            })
            .await
            .expect("saving the notebook should succeed");

        let saved = String::from_utf8(
            fs.read_file_sync(path!("/notebooks/test.ipynb"))
                .expect("notebook should still exist"),
        )
        .expect("notebook should be valid UTF-8");
        assert!(
            saved.contains("print('goodbye')"),
            "the edited cell should be written to the notebook, got: {saved}"
        );

        assert_eq!(
            fs.read_file_sync(path!("/notebooks/test.ipynb.bak"))
                .unwrap(),
            NOTEBOOK_WITH_ONE_CODE_CELL.as_bytes(),
            "backup must contain the original bytes"
        );
        notebook_editor.read_with(cx, |editor, cx| {
            assert!(
                !editor.is_dirty(cx),
                "only a successful write may clear the dirty state"
            );
        });

        buffer.read_with(cx, |buffer, _| {
            assert_eq!(
                buffer.text(),
                saved,
                "the project's buffer should hold the saved notebook"
            );
            assert!(!buffer.is_dirty(), "saving should leave the buffer clean");
        });
    }
    #[gpui::test]
    async fn no_ai_fork_notebook_backup_failure_keeps_changes_dirty(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/notebooks"),
            json!({ "test.ipynb": NOTEBOOK_WITH_ONE_CODE_CELL, "test.ipynb.bak": {} }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/notebooks").as_ref()], cx).await;
        cx.update(|cx| ReplStore::init(fs.clone(), cx));

        let project_path = project.read_with(cx, |project, cx| ProjectPath {
            worktree_id: project.worktrees(cx).next().unwrap().read(cx).id(),
            path: rel_path("test.ipynb").into(),
        });

        let notebook_item = cx
            .update(|cx| {
                NotebookItem::try_open(&project, &project_path, cx)
                    .expect("ipynb files should be openable as notebooks")
            })
            .await
            .expect("notebook should parse");

        // Held across the save: a save that bypasses the project writes the file
        // behind this buffer's back, leaving it stale.
        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(project_path.clone(), cx)
            })
            .await
            .expect("notebook buffer should open");

        // Rendering the notebook animates the kernel status icon, which makes
        // `run_until_parked` spin forever; only the editor entity is needed here.
        let cx = cx.add_empty_window();
        let notebook_editor = cx.update(|window, cx| {
            cx.new(|cx| NotebookEditor::new(project.clone(), notebook_item, window, cx))
        });

        let cell_editor = notebook_editor.read_with(cx, |notebook_editor, cx| {
            let cell_id = notebook_editor
                .cell_order
                .first()
                .expect("notebook has one cell");
            let Some(Cell::Code(cell)) = notebook_editor.cell_map.get(cell_id) else {
                panic!("expected a code cell");
            };
            cell.read(cx).editor().clone()
        });
        cell_editor.update_in(cx, |cell_editor, window, cx| {
            cell_editor.set_text("print('goodbye')", window, cx);
        });

        notebook_editor
            .update_in(cx, |notebook_editor, window, cx| {
                notebook_editor.save(SaveOptions::default(), project.clone(), window, cx)
            })
            .await
            .expect_err("saving must fail if its backup cannot be created");

        assert_eq!(
            fs.read_file_sync(path!("/notebooks/test.ipynb")).unwrap(),
            NOTEBOOK_WITH_ONE_CODE_CELL.as_bytes(),
            "backup failure must leave the original untouched"
        );
        notebook_editor.read_with(cx, |editor, cx| {
            assert!(editor.is_dirty(cx));
            assert!(!editor.saving);
        });
        cell_editor.read_with(cx, |editor, cx| {
            assert_eq!(
                editor.buffer().read(cx).snapshot(cx).text(),
                "print('goodbye')"
            );
        });
    }
}
