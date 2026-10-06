//! Dockable optional search through agx's restricted local editor worker.
mod code_search_settings;

use ::settings::{DockSide, Settings as _, SettingsStore};
use anyhow::{Context as _, Result, ensure};
use code_search_provider::{
    AgxProvider, Cancellation, Document, Evidence, SearchMode, SearchRequest, WorkspaceRoot,
    confined_path,
};
use code_search_settings::CodeSearchSettings;
use editor::{Editor, EditorEvent, SelectionEffects, scroll::Autoscroll};
use futures::FutureExt as _;
use gpui::{
    Action, App, AppContext as _, AsyncWindowContext, Context, Entity, EventEmitter, FocusHandle,
    Focusable, Pixels, ScrollStrategy, Subscription, Task, UniformListScrollHandle, WeakEntity,
    Window, actions, uniform_list,
};
use language::{Buffer, BufferEvent, BufferId, Point};
use project::{Project, ProjectPath, buffer_store::BufferStoreEvent};
use std::{collections::HashMap, sync::Arc, time::Duration};
use ui::{ListItem, Tooltip, prelude::*};
use util::rel_path::RelPath;
use workspace::{
    HideStatusItem, ItemHandle, Panel, StatusItemView, Workspace,
    dock::{DockPosition, PanelEvent},
};

actions!(
    code_search,
    [
        /// Focus the optional local Code Search panel.
        ToggleFocus,
        /// Run the current Code Search query.
        Search,
        /// Rebuild the local Code Search index and run the current query.
        Refresh,
        /// Open the selected Code Search result.
        OpenSelected,
        /// Stop the current Code Search query.
        Cancel,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            if CodeSearchSettings::get_global(cx).enabled {
                workspace.toggle_panel_focus::<CodeSearchPanel>(window, cx);
            }
        });
    })
    .detach();
}

#[derive(Clone)]
enum Row {
    Root(String),
    Hit {
        root: WorkspaceRoot,
        evidence: Evidence,
    },
}

pub struct CodeSearchPanel {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    fs: Arc<dyn fs::Fs>,
    focus: FocusHandle,
    query: Entity<Editor>,
    globs: Entity<Editor>,
    languages: Entity<Editor>,
    preview: Entity<Editor>,
    mode: SearchMode,
    settings: CodeSearchSettings,
    provider: AgxProvider,
    cancel: Option<Cancellation>,
    task: Option<Task<()>>,
    generation: u64,
    active: bool,
    busy: bool,
    status: String,
    rows: Vec<Row>,
    selected: Option<usize>,
    scroll: UniformListScrollHandle,
    _subscriptions: Vec<Subscription>,
    buffers: HashMap<BufferId, Subscription>,
}

impl CodeSearchPanel {
    pub async fn load(
        workspace: WeakEntity<Workspace>,
        mut cx: AsyncWindowContext,
    ) -> Result<Entity<Self>> {
        workspace.update_in(&mut cx, |workspace, window, cx| {
            let project = workspace.project().clone();
            let fs = workspace.app_state().fs.clone();
            let workspace = cx.entity().downgrade();
            cx.new(|cx| Self::new(workspace, project, fs, window, cx))
        })
    }

    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        fs: Arc<dyn fs::Fs>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let query = Self::input("Search code…", window, cx);
        let globs = Self::input("Files: *.rs, !tests/**", window, cx);
        let languages = Self::input("Languages: rust, python", window, cx);
        let preview = cx.new(|cx| {
            let mut editor = Editor::multi_line(window, cx);
            editor.set_read_only(true);
            editor
        });
        let mut subscriptions = Vec::new();
        for editor in [&query, &globs, &languages] {
            subscriptions.push(
                cx.subscribe_in(editor, window, |panel, _, event, window, cx| {
                    if matches!(event, EditorEvent::BufferEdited) {
                        panel.schedule(false, window, cx);
                    }
                }),
            );
        }
        subscriptions.push(
            cx.subscribe_in(&project, window, |panel, _, event, window, cx| {
                if matches!(
                    event,
                    project::Event::WorktreeAdded(_)
                        | project::Event::WorktreeRemoved(_)
                        | project::Event::WorktreePathsChanged { .. }
                        | project::Event::WorktreeUpdatedEntries(..)
                        | project::Event::WorktreeOrderChanged
                        | project::Event::DisconnectedFromHost
                        | project::Event::DisconnectedFromRemote { .. }
                ) {
                    panel.schedule(false, window, cx);
                }
            }),
        );
        let store = project.read(cx).buffer_store().clone();
        subscriptions.push(
            cx.subscribe_in(&store, window, |panel, _, event, window, cx| {
                match event {
                    BufferStoreEvent::BufferAdded(buffer) => {
                        panel.watch_buffer(buffer.clone(), window, cx);
                        if !buffer.read(cx).is_dirty() {
                            return;
                        }
                    }
                    BufferStoreEvent::BufferDropped(id) => {
                        panel.buffers.remove(id);
                    }
                    _ => {}
                }
                panel.schedule(false, window, cx);
            }),
        );
        subscriptions.push(
            cx.observe_global_in::<SettingsStore>(window, |panel, window, cx| {
                let settings = CodeSearchSettings::get_global(cx).clone();
                let restart = settings.enabled != panel.settings.enabled
                    || settings.agx_path != panel.settings.agx_path;
                let mode_changed = settings.default_mode != panel.settings.default_mode;
                if mode_changed {
                    panel.mode = settings.default_mode;
                }
                panel.settings = settings;
                if restart {
                    panel.provider = AgxProvider::new();
                }
                if restart || mode_changed {
                    panel.schedule(false, window, cx);
                }
                cx.notify();
            }),
        );
        let mode = CodeSearchSettings::get_global(cx).default_mode;
        let mut panel = Self {
            workspace,
            project,
            fs,
            focus: cx.focus_handle(),
            query,
            globs,
            languages,
            preview,
            mode,
            settings: CodeSearchSettings::get_global(cx).clone(),
            provider: AgxProvider::new(),
            cancel: None,
            task: None,
            generation: 0,
            active: false,
            busy: false,
            status: "Local text, symbols, and BM25 search. Install agx 0.3+ to begin.".into(),
            rows: Vec::new(),
            selected: None,
            scroll: UniformListScrollHandle::new(),
            _subscriptions: subscriptions,
            buffers: HashMap::new(),
        };
        let buffers = store.read(cx).buffers().collect::<Vec<_>>();
        for buffer in buffers {
            panel.watch_buffer(buffer, window, cx);
        }
        panel
    }
    fn input(
        placeholder: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Editor> {
        cx.new(|cx| {
            let mut editor = Editor::single_line(window, cx);
            editor.set_placeholder_text(placeholder, window, cx);
            editor
        })
    }
    fn watch_buffer(
        &mut self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let id = buffer.read(cx).remote_id();
        self.buffers.entry(id).or_insert_with(|| {
            cx.subscribe_in(&buffer, window, |panel, _, event, window, cx| {
                if matches!(
                    event,
                    BufferEvent::Edited { .. }
                        | BufferEvent::Saved
                        | BufferEvent::DirtyChanged
                        | BufferEvent::Reloaded
                        | BufferEvent::FileHandleChanged
                ) {
                    panel.schedule(false, window, cx);
                }
            })
        });
    }
    fn invalidate(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.generation += 1;
        if let Some(cancel) = self.cancel.take() {
            cancel.cancel();
        }
        self.task = None;
        self.busy = false;
        self.rows.clear();
        self.selected = None;
        self.update_preview(window, cx);
    }
    fn schedule(&mut self, rescan: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.invalidate(window, cx);
        if !CodeSearchSettings::get_global(cx).enabled {
            self.provider.shutdown();
            self.status = "Code Search is disabled in settings.".into();
        } else if !self.project.read(cx).is_local() {
            self.status = "Code Search supports local project folders only.".into();
        } else if self.query.read(cx).text(cx).trim().is_empty() {
            self.status =
                "Enter a query. Search runs locally with agx; no models or network.".into();
        } else if self.active {
            self.busy = true;
            self.status = "Indexing and searching…".into();
            let generation = self.generation;
            self.task = Some(cx.spawn_in(window, async move |panel, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(350))
                    .await;
                let Some(receiver) = panel
                    .update_in(cx, |panel, _, cx| {
                        if panel.generation != generation {
                            return None;
                        }
                        match panel
                            .request(rescan, cx)
                            .and_then(|request| panel.provider.search(request))
                        {
                            Ok((cancel, receiver)) => {
                                panel.cancel = Some(cancel);
                                Some(receiver)
                            }
                            Err(error) => {
                                panel.busy = false;
                                panel.status = format!("{error:#}");
                                cx.notify();
                                None
                            }
                        }
                    })
                    .ok()
                    .flatten()
                else {
                    return;
                };
                let mut receiver = receiver.fuse();
                let output = loop {
                    let timer = cx.background_executor().timer(Duration::from_millis(100)).fuse();
                    futures::pin_mut!(timer);
                    futures::select! {
                        result = receiver => break result.unwrap_or_else(|_| Err(anyhow::anyhow!("Code Search worker stopped; retry the query"))),
                        _ = timer => {
                            let _ = panel.update(cx, |panel, cx| {
                                if panel.generation != generation { return; }
                                if let Some(progress) = panel.cancel.as_ref().and_then(Cancellation::progress) {
                                    panel.status = format!("Indexing · {} seen · {} read · {} reused", progress.files_seen, progress.files_read, progress.files_reused);
                                    cx.notify();
                                }
                            });
                        }
                    }
                };
                let _ = panel.update_in(cx, |panel, window, cx| {
                    if panel.generation != generation {
                        return;
                    }
                    panel.busy = false;
                    panel.cancel = None;
                    match output {
                        Ok(output) => {
                            let roots = panel.roots(cx);
                            let mut matched: usize = 0;
                            let mut shown: usize = 0;
                            let mut notes = Vec::new();
                            for group in output.roots {
                                let Some(root) = roots.iter().find(|r| r.id == group.root_id)
                                else {
                                    continue;
                                };
                                matched = matched.saturating_add(group.matched_units);
                                shown = shown.saturating_add(group.returned_units);
                                let name =
                                    root.path.file_name().unwrap_or_default().to_string_lossy();
                                panel.rows.push(Row::Root(format!(
                                    "{name} · {} results",
                                    group.returned_units
                                )));
                                panel.rows.extend(group.results.into_iter().map(|evidence| {
                                    Row::Hit {
                                        root: root.clone(),
                                        evidence,
                                    }
                                }));
                                if group.incomplete {
                                    notes.push(format!(
                                        "{name}: index incomplete (file/memory limits)"
                                    ));
                                }
                                if group.truncated {
                                    notes.push(format!("{name}: results limited"));
                                }
                                if group.skipped_files["total"].as_u64().unwrap_or(0) > 0 {
                                    notes.push(format!("{name}: some files skipped"));
                                }
                                notes.extend(group.warnings);
                            }
                            panel.status = format!(
                                "{shown} shown / {matched} matches{}",
                                if notes.is_empty() {
                                    String::new()
                                } else {
                                    format!(" · {}", notes.join("; "))
                                }
                            );
                            panel.selected =
                                panel.rows.iter().position(|r| matches!(r, Row::Hit { .. }));
                            panel.update_preview(window, cx);
                        }
                        Err(error) => panel.status = format!("{error:#}"),
                    }
                    cx.notify();
                });
            }));
        }
        cx.notify();
    }
    fn roots(&self, cx: &App) -> Vec<WorkspaceRoot> {
        self.project
            .read(cx)
            .visible_worktrees(cx)
            .filter_map(|worktree| {
                let tree = worktree.read(cx);
                (!tree.is_single_file()).then(|| WorkspaceRoot {
                    id: tree.id().to_proto().to_string(),
                    path: tree.abs_path().to_path_buf(),
                })
            })
            .collect()
    }
    fn request(&self, rescan: bool, cx: &App) -> Result<SearchRequest> {
        let roots = self.roots(cx);
        ensure!(
            !roots.is_empty(),
            "Open a local project folder to use Code Search"
        );
        let project = self.project.read(cx);
        let mut documents = Vec::new();
        let mut document_bytes = 0;
        for buffer in project.buffer_store().read(cx).buffers() {
            let buffer = buffer.read(cx);
            if !buffer.is_dirty() {
                continue;
            }
            let Some(file) = buffer.file() else {
                continue;
            };
            let root_id = file.worktree_id(cx).to_proto().to_string();
            let path = file.path().as_unix_str().to_string();
            if !roots.iter().any(|r| r.id == root_id) || path.to_lowercase().ends_with(".ipynb") {
                continue;
            }
            ensure!(
                buffer.len() <= 2 * 1024 * 1024,
                "Unsaved file {path} exceeds Code Search’s 2 MiB limit"
            );
            document_bytes += buffer.len();
            ensure!(
                document_bytes <= 16 * 1024 * 1024,
                "Unsaved buffers exceed Code Search’s 16 MiB limit; save files and retry"
            );
            documents.push(Document {
                root_id,
                path,
                content: buffer.text(),
            });
        }
        fn filters(text: String) -> Vec<String> {
            text.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned)
                .collect()
        }
        Ok(SearchRequest {
            executable: CodeSearchSettings::get_global(cx).agx_path.clone(),
            roots,
            documents,
            query: self.query.read(cx).text(cx),
            mode: self.mode,
            globs: filters(self.globs.read(cx).text(cx)),
            languages: filters(self.languages.read(cx).text(cx)),
            rescan,
        })
    }
    fn update_preview(&self, window: &mut Window, cx: &mut Context<Self>) {
        let content = match self.selected.and_then(|i| self.rows.get(i)) {
            Some(Row::Hit { evidence, .. }) => evidence.content.clone(),
            _ => String::new(),
        };
        self.preview.update(cx, |preview, cx| {
            preview.set_read_only(false);
            preview.set_text(content, window, cx);
            preview.set_read_only(true);
        });
    }
    fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if !matches!(self.rows.get(index), Some(Row::Hit { .. })) {
            return;
        }
        self.selected = Some(index);
        self.scroll.scroll_to_item(index, ScrollStrategy::Top);
        self.update_preview(window, cx);
        cx.notify();
    }
    fn move_selection(&mut self, forward: bool, window: &mut Window, cx: &mut Context<Self>) {
        let selectable = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(i, r)| matches!(r, Row::Hit { .. }).then_some(i))
            .collect::<Vec<_>>();
        if selectable.is_empty() {
            return;
        }
        let current = selectable
            .iter()
            .position(|i| Some(*i) == self.selected)
            .unwrap_or(0);
        let next = if forward {
            (current + 1) % selectable.len()
        } else {
            (current + selectable.len() - 1) % selectable.len()
        };
        self.select(selectable[next], window, cx);
    }
    fn open_selected(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(Row::Hit { root, evidence }) =
            self.selected.and_then(|i| self.rows.get(i)).cloned()
        else {
            return;
        };
        let workspace = self.workspace.clone();
        let generation = self.generation;
        cx.spawn_in(window, async move |panel, cx| {
            let result: Result<()> = async {
                // Validate filesystem confinement off the UI thread immediately before opening.
                cx.background_spawn({
                    let root = root.clone();
                    let path = evidence.path.clone();
                    async move { confined_path(&root.path, &path) }
                })
                .await?;
                let task = panel.update_in(cx, |panel, window, cx| -> Result<_> {
                    ensure!(
                        panel.generation == generation
                            && CodeSearchSettings::get_global(cx).enabled
                            && panel.project.read(cx).is_local(),
                        "Code Search results changed; search again"
                    );
                    let root_id = root.id.parse::<u64>()?;
                    let path = ProjectPath {
                        worktree_id: ::settings::WorktreeId::from_proto(root_id),
                        path: RelPath::new(
                            std::path::Path::new(&evidence.path),
                            util::paths::PathStyle::Unix,
                        )?
                        .into(),
                    };
                    workspace.update(cx, |workspace, cx| {
                        workspace.open_path(path, None, true, window, cx)
                    })
                })??;
                let item = task.await?;
                let editor = item
                    .downcast::<Editor>()
                    .context("This result cannot be opened as text")?;
                let (snapshot, dirty) = editor.read_with(cx, |editor, cx| -> Result<_> {
                    let buffer = editor
                        .buffer()
                        .read(cx)
                        .as_singleton()
                        .context("Expected a single source file")?;
                    Ok((buffer.read(cx).snapshot(), buffer.read(cx).is_dirty()))
                })?;
                let expected = evidence.content_hash.clone();
                let disk_source = evidence.source == "disk";
                let version = snapshot.version.clone();
                let start = Point::new(
                    evidence.source_range.start.line - 1,
                    evidence.source_range.start.byte_column,
                );
                let end = Point::new(
                    evidence.source_range.end.line - 1,
                    evidence.source_range.end.byte_column,
                );
                let valid = cx
                    .background_spawn(async move {
                        if snapshot.clip_point(start, language::Bias::Left) != start
                            || snapshot.clip_point(end, language::Bias::Left) != end
                            || start > end
                        {
                            return false;
                        }
                        let mut text = snapshot.text();
                        // Disk identity is computed over original line endings; overlays use editor text.
                        if disk_source && snapshot.line_ending() == language::LineEnding::Windows {
                            text = text.replace('\n', "\r\n");
                        }
                        blake3::hash(text.as_bytes()).to_hex().as_str() == expected
                    })
                    .await;
                ensure!(
                    valid && !(disk_source && dirty),
                    "This file changed since the search; refresh Code Search"
                );
                panel.update_in(cx, |panel, window, cx| -> Result<()> {
                    ensure!(
                        panel.generation == generation,
                        "Code Search results changed; search again"
                    );
                    editor.update(cx, |editor, cx| -> Result<()> {
                        let buffer = editor
                            .buffer()
                            .read(cx)
                            .as_singleton()
                            .context("Expected a source file")?;
                        ensure!(
                            buffer.read(cx).version() == version,
                            "This file changed; refresh Code Search"
                        );
                        editor.change_selections(
                            SelectionEffects::scroll(Autoscroll::center()),
                            window,
                            cx,
                            |selections| selections.select_ranges(Some(start..end)),
                        );
                        window.focus(&editor.focus_handle(cx), cx);
                        Ok(())
                    })
                })??;
                Ok(())
            }
            .await;
            if let Err(error) = result {
                let _ = panel.update(cx, |panel, cx| {
                    panel.status = format!("{error:#}");
                    cx.notify();
                });
            }
        })
        .detach();
    }
}

impl Drop for CodeSearchPanel {
    fn drop(&mut self) {
        if let Some(cancel) = &self.cancel {
            cancel.cancel();
        }
        self.provider.shutdown();
    }
}
impl Focusable for CodeSearchPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}
impl EventEmitter<PanelEvent> for CodeSearchPanel {}
impl Panel for CodeSearchPanel {
    fn persistent_name() -> &'static str {
        "Code Search"
    }
    fn panel_key() -> &'static str {
        "code_search"
    }
    fn activation_focus_handle(&self, cx: &App) -> FocusHandle {
        self.query.focus_handle(cx)
    }
    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        match CodeSearchSettings::get_global(cx).dock {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        }
    }
    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }
    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        ::settings::update_settings_file(self.fs.clone(), cx, move |settings, _| {
            settings.code_search.get_or_insert_default().dock =
                Some(if position == DockPosition::Right {
                    DockSide::Right
                } else {
                    DockSide::Left
                });
        });
    }
    fn default_size(&self, _: &Window, cx: &App) -> Pixels {
        CodeSearchSettings::get_global(cx).default_width
    }
    fn min_size(&self, _: &Window, _: &App) -> Option<Pixels> {
        Some(px(280.))
    }
    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        let settings = CodeSearchSettings::get_global(cx);
        (settings.enabled && settings.button).then_some(IconName::Code)
    }
    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Code Search")
    }
    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFocus)
    }
    fn activation_priority(&self) -> u32 {
        7
    }
    fn enabled(&self, cx: &App) -> bool {
        CodeSearchSettings::get_global(cx).enabled
    }
    fn set_active(&mut self, active: bool, window: &mut Window, cx: &mut Context<Self>) {
        if self.active == active {
            return;
        }
        self.active = active;
        if active {
            self.schedule(false, window, cx);
        } else {
            self.invalidate(window, cx);
            cx.notify();
        }
    }
    fn hide_button_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|s| {
            s.code_search.get_or_insert_default().button = Some(false)
        }))
    }
}
impl Render for CodeSearchPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut modes = h_flex().gap_1();
        for mode in [SearchMode::Text, SearchMode::Symbol, SearchMode::Ranked] {
            modes = modes.child(
                Button::new(
                    mode.label(),
                    if mode == SearchMode::Ranked {
                        "Ranked (BM25)"
                    } else {
                        mode.label()
                    },
                )
                .toggle_state(self.mode == mode)
                .on_click(cx.listener(move |panel, _, window, cx| {
                    panel.mode = mode;
                    panel.schedule(false, window, cx);
                })),
            );
        }
        let count = self.rows.len();
        let preview_label =
            self.selected
                .and_then(|i| self.rows.get(i))
                .and_then(|row| match row {
                    Row::Hit { evidence, .. } => Some(format!(
                        "{}:{}–{}{}{}",
                        evidence.path,
                        evidence.start_line,
                        evidence.end_line,
                        if evidence.source == "overlay" {
                            " · unsaved"
                        } else {
                            ""
                        },
                        if evidence.excerpt_truncated {
                            " · excerpt limited"
                        } else {
                            ""
                        }
                    )),
                    _ => None,
                });
        v_flex()
            .id("code-search-panel")
            .key_context("CodeSearch")
            .track_focus(&self.focus)
            .size_full()
            .overflow_hidden()
            .text_ui(cx)
            .on_action(
                cx.listener(|panel, _: &Search, window, cx| panel.schedule(false, window, cx)),
            )
            .on_action(
                cx.listener(|panel, _: &Refresh, window, cx| panel.schedule(true, window, cx)),
            )
            .on_action(
                cx.listener(|panel, _: &OpenSelected, window, cx| panel.open_selected(window, cx)),
            )
            .on_action(cx.listener(|panel, _: &Cancel, window, cx| {
                panel.invalidate(window, cx);
                panel.status = "Search cancelled.".into();
                cx.notify();
            }))
            .on_action(cx.listener(|panel, _: &menu::SelectNext, window, cx| {
                panel.move_selection(true, window, cx)
            }))
            .on_action(cx.listener(|panel, _: &menu::SelectPrevious, window, cx| {
                panel.move_selection(false, window, cx)
            }))
            .child(
                v_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        h_flex()
                            .justify_between()
                            .child(Label::new("Code Search"))
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        IconButton::new("search", IconName::MagnifyingGlass)
                                            .tooltip(Tooltip::text("Run search"))
                                            .on_click(cx.listener(|panel, _, window, cx| {
                                                panel.schedule(false, window, cx)
                                            })),
                                    )
                                    .child(
                                        IconButton::new("refresh", IconName::ArrowCircle)
                                            .tooltip(Tooltip::text("Rescan project folders"))
                                            .on_click(cx.listener(|panel, _, window, cx| {
                                                panel.schedule(true, window, cx)
                                            })),
                                    )
                                    .when(self.busy, |row| {
                                        row.child(
                                            IconButton::new("cancel", IconName::Close)
                                                .tooltip(Tooltip::text("Cancel search"))
                                                .on_click(cx.listener(|panel, _, window, cx| {
                                                    panel.invalidate(window, cx);
                                                    panel.status = "Search cancelled.".into();
                                                    cx.notify();
                                                })),
                                        )
                                    })
                                    .child(
                                        IconButton::new("settings", IconName::Settings)
                                            .tooltip(Tooltip::text(
                                                "Configure Code Search in settings JSON",
                                            ))
                                            .on_click(|_, window, cx| {
                                                window.dispatch_action(
                                                    Box::new(zed_actions::OpenSettingsFile),
                                                    cx,
                                                )
                                            }),
                                    ),
                            ),
                    )
                    .child(modes)
                    .child(div().p_1().child(self.query.clone()))
                    .child(div().p_1().child(self.globs.clone()))
                    .child(div().p_1().child(self.languages.clone()))
                    .child(
                        div()
                            .id("code-search-status")
                            .child(Label::new(self.status.clone()).size(LabelSize::Small))
                            .tooltip(Tooltip::text(self.status.clone())),
                    ),
            )
            .child(
                uniform_list(
                    "code-search-results",
                    count,
                    cx.processor(|panel, range: std::ops::Range<usize>, _, cx| {
                        range
                            .map(|i| match &panel.rows[i] {
                                Row::Root(name) => div()
                                    .h_10()
                                    .px_2()
                                    .flex()
                                    .items_center()
                                    .child(Label::new(name.clone()).size(LabelSize::Small))
                                    .into_any_element(),
                                Row::Hit { evidence, .. } => ListItem::new(i)
                                    .height(px(40.))
                                    .toggle_state(panel.selected == Some(i))
                                    .child(
                                        v_flex()
                                            .h_10()
                                            .overflow_hidden()
                                            .child(Label::new(
                                                evidence
                                                    .symbol
                                                    .clone()
                                                    .unwrap_or_else(|| evidence.path.clone()),
                                            ))
                                            .child(
                                                Label::new(format!(
                                                    "{}:{} · {}",
                                                    evidence.path,
                                                    evidence.start_line,
                                                    evidence.kind
                                                ))
                                                .size(LabelSize::Small)
                                                .color(Color::Muted),
                                            ),
                                    )
                                    .on_click(cx.listener(
                                        move |panel, event: &gpui::ClickEvent, window, cx| {
                                            panel.select(i, window, cx);
                                            if event.click_count() > 1 {
                                                panel.open_selected(window, cx);
                                            }
                                        },
                                    ))
                                    .into_any_element(),
                            })
                            .collect::<Vec<_>>()
                    }),
                )
                .track_scroll(&self.scroll)
                .flex_1()
                .min_h(px(60.)),
            )
            .when_some(preview_label, |panel, label| {
                panel.child(
                    v_flex()
                        .p_2()
                        .gap_1()
                        .h(px(200.))
                        .overflow_hidden()
                        .child(Label::new(label).size(LabelSize::Small))
                        .child(div().flex_1().overflow_hidden().child(self.preview.clone()))
                        .child(Button::new("open-result", "Open result").on_click(
                            cx.listener(|panel, _, window, cx| panel.open_selected(window, cx)),
                        )),
                )
            })
    }
}

pub struct CodeSearchButton;
impl Render for CodeSearchButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !CodeSearchSettings::get_global(cx).enabled || !CodeSearchSettings::get_global(cx).button
        {
            return div().hidden();
        }
        div().child(
            IconButton::new("code-search-indicator", IconName::Code)
                .icon_size(IconSize::Small)
                .aria_label("Code Search")
                .tooltip(|_, cx| Tooltip::for_action("Code Search", &ToggleFocus, cx))
                .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleFocus), cx)),
        )
    }
}
impl StatusItemView for CodeSearchButton {
    fn set_active_pane_item(
        &mut self,
        _: Option<&dyn ItemHandle>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
    }
    fn hide_setting(&self, _: &App) -> Option<HideStatusItem> {
        Some(HideStatusItem::new(|s| {
            s.code_search.get_or_insert_default().button = Some(false)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, UpdateGlobal as _, VisualTestContext};
    use serde_json::json;

    async fn panel(
        cx: &mut TestAppContext,
    ) -> (
        Entity<Workspace>,
        Entity<CodeSearchPanel>,
        VisualTestContext,
    ) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            cx.set_global(db::AppDatabase::test_new());
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
            init(cx);
        });
        let fs = fs::FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({"main.py": "def disk(): pass\n", "notebook.ipynb": "{}"}),
        )
        .await;
        let project = Project::test(fs.clone(), [std::path::Path::new("/project")], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let panel = workspace.update_in(cx, |_, window, cx| {
            let workspace = cx.entity().downgrade();
            cx.new(|cx| CodeSearchPanel::new(workspace, project, fs, window, cx))
        });
        workspace.update_in(cx, |workspace, window, cx| {
            workspace.add_panel(panel.clone(), window, cx)
        });
        (workspace, panel, cx.clone())
    }
    fn hit(root: WorkspaceRoot, source: &str) -> Row {
        Row::Hit { root: root.clone(), evidence: serde_json::from_value(json!({
            "workspace_id":"test", "root_id":root.id, "index_version":1, "path":"main.py",
            "start_line":1, "end_line":1, "symbol":"needle", "kind":"function", "score":1,
            "content":source, "excerpt_truncated":false, "source":"disk", "document_version":null,
            "content_hash":"1".repeat(64), "source_range":{"start":{"line":1,"byte_column":0},"end":{"line":1,"byte_column":1},"end_exclusive":true}
        })).unwrap() }
    }
    #[gpui::test]
    async fn no_ai_fork_code_search_navigation_and_cancellation(cx: &mut TestAppContext) {
        let (_, panel, mut cx) = panel(cx).await;
        panel.update_in(&mut cx, |panel, window, cx| {
            let root = panel.roots(cx).remove(0);
            panel.rows = vec![
                Row::Root("project".into()),
                hit(root.clone(), "[untrusted](https://example.invalid)"),
                Row::Root("other".into()),
                hit(root, "literal source"),
            ];
            panel.select(0, window, cx);
            assert_eq!(panel.selected, None);
            panel.select(1, window, cx);
            assert_eq!(
                panel.preview.read(cx).text(cx),
                "[untrusted](https://example.invalid)"
            );
            panel.move_selection(true, window, cx);
            assert_eq!(panel.selected, Some(3));
            panel.move_selection(true, window, cx);
            assert_eq!(panel.selected, Some(1));
            panel.move_selection(false, window, cx);
            assert_eq!(panel.selected, Some(3));
            let cancellation = Cancellation::default();
            panel.cancel = Some(cancellation.clone());
            let generation = panel.generation;
            panel.invalidate(window, cx);
            assert!(cancellation.is_cancelled());
            assert!(panel.rows.is_empty());
            assert_eq!(panel.selected, None);
            assert!(panel.generation > generation);
            assert!(panel.preview.read(cx).text(cx).is_empty());
        });
    }
    #[gpui::test]
    async fn no_ai_fork_code_search_unsaved_buffers_exclude_notebooks(cx: &mut TestAppContext) {
        let (_, panel, mut cx) = panel(cx).await;
        let project = panel.read_with(&cx, |panel, _| panel.project.clone());
        // BufferStore holds weak references; retain handles as open editors would.
        let mut open_buffers = Vec::new();
        for path in ["/project/main.py", "/project/notebook.ipynb"] {
            let buffer = project
                .update(&mut cx, |project, cx| {
                    let path = project.find_project_path(path, cx).unwrap();
                    project.open_buffer(path, cx)
                })
                .await
                .unwrap();
            buffer.update(&mut cx, |buffer, cx| buffer.set_text("unsaved source", cx));
            open_buffers.push(buffer);
        }
        panel.update_in(&mut cx, |panel, window, cx| {
            panel
                .query
                .update(cx, |query, cx| query.set_text("unsaved", window, cx));
            let request = panel.request(false, cx).unwrap();
            assert_eq!(request.documents.len(), 1);
            assert_eq!(request.documents[0].path, "main.py");
            assert_eq!(request.documents[0].content, "unsaved source");
            assert_eq!(request.mode, SearchMode::Symbol);
        });
        drop(open_buffers);
        cx.run_until_parked();
        panel.read_with(&cx, |panel, cx| {
            assert!(panel.request(false, cx).unwrap().documents.is_empty());
        });
    }
    #[gpui::test]
    async fn no_ai_fork_code_search_settings_disable_panel(cx: &mut TestAppContext) {
        let (_, panel, mut cx) = panel(cx).await;
        cx.update(|_, cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings.code_search.get_or_insert_default().enabled = Some(false)
                });
            })
        });
        cx.run_until_parked();
        panel.update_in(&mut cx, |panel, window, cx| {
            assert!(!panel.enabled(cx));
            assert!(panel.icon(window, cx).is_none());
            assert!(!panel.is_agent_panel());
            assert!(panel.cancel.is_none());
            assert!(panel.rows.is_empty());
        });
    }
}
