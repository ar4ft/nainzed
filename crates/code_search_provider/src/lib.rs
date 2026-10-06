//! Optional local, lexical search provider. No model, HTTP, MCP or telemetry dependencies.
use anyhow::{Context as _, Result, bail, ensure};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashMap},
    io::{BufRead, BufReader, Read, Write},
    path::{Component, Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};
const FRAME_LIMIT: usize = 4 * 1024 * 1024;
const FILE_LIMIT: usize = 2 * 1024 * 1024;
const JOB_TIMEOUT: Duration = Duration::from_secs(180);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SearchMode {
    Text,
    #[default]
    Symbol,
    Ranked,
}
impl SearchMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Text => "Text",
            Self::Symbol => "Symbol",
            Self::Ranked => "Ranked",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRoot {
    pub id: String,
    pub path: PathBuf,
}
#[derive(Clone, Debug)]
pub struct Document {
    pub root_id: String,
    pub path: String,
    pub content: String,
}
#[derive(Clone, Debug)]
pub struct SearchRequest {
    pub executable: Option<PathBuf>,
    pub roots: Vec<WorkspaceRoot>,
    pub documents: Vec<Document>,
    pub query: String,
    pub mode: SearchMode,
    pub globs: Vec<String>,
    pub languages: Vec<String>,
    pub rescan: bool,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Evidence {
    pub workspace_id: String,
    pub root_id: String,
    pub index_version: u64,
    pub path: String,
    pub start_line: u32,
    pub end_line: u32,
    pub symbol: Option<String>,
    pub kind: String,
    pub score: f64,
    pub content: String,
    pub excerpt_truncated: bool,
    pub source: String,
    pub document_version: Option<u64>,
    pub content_hash: String,
    pub source_range: SourceRange,
}
#[derive(Clone, Debug, Deserialize)]
pub struct SourceRange {
    pub start: SourcePoint,
    pub end: SourcePoint,
    pub end_exclusive: bool,
}
#[derive(Clone, Debug, Deserialize)]
pub struct SourcePoint {
    pub line: u32,
    pub byte_column: u32,
}
#[derive(Debug, Deserialize)]
pub struct RootResults {
    pub schema_version: u32,
    pub workspace_id: String,
    pub session_id: String,
    pub root_id: String,
    pub index_version: u64,
    pub mode: SearchMode,
    pub query: String,
    pub results: Vec<Evidence>,
    pub matched_units: usize,
    pub returned_units: usize,
    pub truncated: bool,
    pub incomplete: bool,
    pub warnings: Vec<String>,
    pub skipped_files: Value,
}
#[derive(Debug)]
pub struct SearchOutput {
    pub roots: Vec<RootResults>,
    pub executable: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IndexProgress {
    pub root_id: String,
    pub phase: String,
    pub files_seen: u64,
    pub files_read: u64,
    pub files_reused: u64,
}
#[derive(Clone, Default)]
pub struct Cancellation {
    cancelled: Arc<AtomicBool>,
    progress: Arc<Mutex<Option<IndexProgress>>>,
}
impl Cancellation {
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
    pub fn progress(&self) -> Option<IndexProgress> {
        self.progress.lock().unwrap().clone()
    }
}
/// Independent of editor, agent, model, and telemetry registries.
pub trait CodeSearchProvider {
    fn search(
        &self,
        request: SearchRequest,
    ) -> Result<(Cancellation, oneshot::Receiver<Result<SearchOutput>>)>;
    fn shutdown(&self);
}
struct Job {
    request: SearchRequest,
    cancel: Cancellation,
    result: oneshot::Sender<Result<SearchOutput>>,
}
/// UI calls enqueue bounded work; all process and filesystem I/O runs on this actor.
/// Dropping the provider stops its worker without joining on the UI thread.
pub struct AgxProvider {
    jobs: mpsc::SyncSender<Job>,
    closed: Arc<AtomicBool>,
}
impl AgxProvider {
    pub fn new() -> Self {
        let (jobs, incoming) = mpsc::sync_channel::<Job>(4);
        let closed = Arc::new(AtomicBool::new(false));
        let stopped = closed.clone();
        std::thread::spawn(move || {
            let mut worker: Option<Worker> = None;
            while !stopped.load(Ordering::Acquire) {
                let job = match incoming.recv_timeout(Duration::from_millis(100)) {
                    Ok(job) => job,
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(_) => break,
                };
                if job.cancel.is_cancelled() {
                    let _ = job.result.send(Err(anyhow::anyhow!("Search cancelled")));
                    continue;
                }
                let result = (|| {
                    let executable = locate_executable(job.request.executable.as_deref())?;
                    let roots = canonical_roots(&job.request.roots)?;
                    if worker.as_ref().is_none_or(|w| {
                        w.executable != executable || w.roots != roots || !w.alive()
                    }) {
                        worker = Some(Worker::spawn(executable, roots, stopped.clone())?);
                    }
                    worker.as_mut().unwrap().search(job.request, &job.cancel)
                })();
                if result.is_err() {
                    worker = None;
                }
                let _ = job.result.send(result);
            }
        });
        Self { jobs, closed }
    }
    pub fn search(
        &self,
        request: SearchRequest,
    ) -> Result<(Cancellation, oneshot::Receiver<Result<SearchOutput>>)> {
        ensure!(
            !self.closed.load(Ordering::Acquire),
            "Code Search is stopped"
        );
        let cancel = Cancellation::default();
        let (result, receiver) = oneshot::channel();
        self.jobs
            .try_send(Job {
                request,
                cancel: cancel.clone(),
                result,
            })
            .map_err(|_| anyhow::anyhow!("Code Search is busy; retry the search"))?;
        Ok((cancel, receiver))
    }
    pub fn shutdown(&self) {
        self.closed.store(true, Ordering::Release);
    }
}
impl Default for AgxProvider {
    fn default() -> Self {
        Self::new()
    }
}
impl CodeSearchProvider for AgxProvider {
    fn search(
        &self,
        request: SearchRequest,
    ) -> Result<(Cancellation, oneshot::Receiver<Result<SearchOutput>>)> {
        AgxProvider::search(self, request)
    }
    fn shutdown(&self) {
        AgxProvider::shutdown(self);
    }
}
impl Drop for AgxProvider {
    fn drop(&mut self) {
        self.shutdown();
    }
}

pub fn locate_executable(explicit: Option<&Path>) -> Result<PathBuf> {
    fn executable(path: &Path) -> bool {
        let Ok(metadata) = path.metadata() else {
            return false;
        };
        if !metadata.is_file() {
            return false;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode() & 0o111 != 0
        }
        #[cfg(not(unix))]
        {
            true
        }
    }
    if let Some(path) = explicit {
        ensure!(
            path.is_absolute() && executable(path),
            "code_search.agx_path must be an absolute executable file"
        );
        return path
            .canonicalize()
            .context("Cannot open configured agx executable");
    }
    let mut candidates = std::env::var_os("PATH")
        .map(|p| {
            std::env::split_paths(&p)
                .filter(|p| p.is_absolute())
                .map(|p| p.join("agx"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if let Some(home) = std::env::var_os("HOME") {
        candidates.extend([
            PathBuf::from(&home).join(".agx/bin/agx"),
            PathBuf::from(&home).join(".cargo/bin/agx"),
            PathBuf::from(home).join(".local/bin/agx"),
        ]);
    }
    candidates.extend([
        PathBuf::from("/opt/homebrew/bin/agx"),
        PathBuf::from("/usr/local/bin/agx"),
    ]);
    candidates.into_iter().find(|p| executable(p)).context("agx was not found. Install agentgrep 0.3 or later, or set code_search.agx_path to its absolute executable path.")?.canonicalize().map_err(Into::into)
}
fn canonical_roots(roots: &[WorkspaceRoot]) -> Result<Vec<WorkspaceRoot>> {
    ensure!(
        !roots.is_empty() && roots.len() <= 8,
        "Code Search requires 1–8 local project folders"
    );
    let mut result = Vec::new();
    for root in roots {
        ensure!(!root.id.is_empty(), "Missing workspace root identity");
        let path = root
            .path
            .canonicalize()
            .context("Cannot access project folder")?;
        ensure!(path.is_dir(), "Code Search requires project folders");
        ensure!(
            path.to_str().is_some(),
            "Code Search requires UTF-8 project paths"
        );
        ensure!(
            !result
                .iter()
                .any(|r: &WorkspaceRoot| r.id == root.id || r.path == path),
            "Duplicate Code Search project root"
        );
        result.push(WorkspaceRoot {
            id: root.id.clone(),
            path,
        });
    }
    Ok(result)
}
/// Constrain untrusted response paths, including symlinked parents and unsaved files.
pub fn confined_path(root: &Path, relative: &str) -> Result<PathBuf> {
    let relative_path = Path::new(relative);
    ensure!(
        !relative.is_empty()
            && relative.len() <= 4096
            && !relative.contains('\\')
            && relative
                .split('/')
                .all(|p| !p.is_empty() && p != "." && p != "..")
            && relative_path
                .components()
                .all(|c| matches!(c, Component::Normal(_))),
        "Invalid Code Search result path"
    );
    ensure!(
        !relative_path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("ipynb")),
        "Notebook JSON is outside Code Search; use notebook search"
    );
    let root = root.canonicalize()?;
    let mut full = root.clone();
    for component in relative_path.components() {
        full.push(component);
        match std::fs::symlink_metadata(&full) {
            Ok(metadata) => ensure!(
                !metadata.file_type().is_symlink(),
                "Code Search refuses symlinked result paths"
            ),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    let mut existing = full.as_path();
    while !existing.exists() {
        existing = existing.parent().context("Missing result parent")?;
    }
    ensure!(
        existing.canonicalize()?.starts_with(&root),
        "Code Search result escaped project root"
    );
    Ok(full)
}
struct Active {
    cancel: Cancellation,
    started: Instant,
}
struct Worker {
    executable: PathBuf,
    roots: Vec<WorkspaceRoot>,
    child: Arc<Mutex<Child>>,
    input: ChildStdin,
    messages: mpsc::Receiver<Result<Value>>,
    next: u64,
    session: String,
    workspace: String,
    overlays: BTreeMap<(String, String), (String, u64)>,
    versions: HashMap<(String, String), u64>,
    active: Arc<Mutex<Option<Active>>>,
    stopped: Arc<AtomicBool>,
    initialized: bool,
}
impl Worker {
    fn spawn(
        executable: PathBuf,
        roots: Vec<WorkspaceRoot>,
        closed: Arc<AtomicBool>,
    ) -> Result<Self> {
        let mut process = Command::new(&executable)
            .args(["serve", "--stdio", "--restricted"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Cannot start agx")?;
        let input = process.stdin.take().context("agx stdin unavailable")?;
        let output = process.stdout.take().context("agx stdout unavailable")?;
        let stderr = process.stderr.take().context("agx stderr unavailable")?;
        // Drain diagnostics without retaining or logging source/query data.
        std::thread::spawn(move || {
            let _ = std::io::copy(&mut BufReader::new(stderr), &mut std::io::sink());
        });
        let (sender, messages) = mpsc::sync_channel(16);
        std::thread::spawn(move || {
            let mut reader = BufReader::new(output);
            loop {
                let mut bytes = Vec::new();
                let read = Read::by_ref(&mut reader)
                    .take(FRAME_LIMIT as u64 + 1)
                    .read_until(b'\n', &mut bytes);
                let message = match read {
                    Ok(0) => break,
                    Ok(_) if bytes.len() <= FRAME_LIMIT && bytes.last() == Some(&b'\n') => {
                        serde_json::from_slice(&bytes).context("Invalid agx response JSON")
                    }
                    Ok(_) => Err(anyhow::anyhow!("agx response exceeded the frame limit")),
                    Err(e) => Err(e.into()),
                };
                let failed = message.is_err();
                if sender.send(message).is_err() || failed {
                    break;
                }
            }
        });
        let child = Arc::new(Mutex::new(process));
        let active: Arc<Mutex<Option<Active>>> = Arc::new(Mutex::new(None));
        let stopped = Arc::new(AtomicBool::new(false));
        let (process, activity, finished) = (child.clone(), active.clone(), stopped.clone());
        std::thread::spawn(move || {
            let mut cancellation_started = None;
            while !finished.load(Ordering::Acquire) {
                let kill = closed.load(Ordering::Acquire) || {
                    let active = activity.lock().unwrap();
                    let cancelled = active.as_ref().is_some_and(|a| a.cancel.is_cancelled());
                    if cancelled {
                        cancellation_started.get_or_insert_with(Instant::now);
                    } else {
                        cancellation_started = None;
                    }
                    active
                        .as_ref()
                        .is_some_and(|a| a.started.elapsed() > JOB_TIMEOUT)
                        || cancellation_started
                            .is_some_and(|t| t.elapsed() > Duration::from_secs(2))
                };
                if kill {
                    let _ = process.lock().unwrap().kill();
                    break;
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        });
        Ok(Self {
            executable,
            roots,
            child,
            input,
            messages,
            next: 0,
            session: String::new(),
            workspace: "nain-code-search".into(),
            overlays: BTreeMap::new(),
            versions: HashMap::new(),
            active,
            stopped,
            initialized: false,
        })
    }
    fn alive(&self) -> bool {
        self.child
            .lock()
            .unwrap()
            .try_wait()
            .ok()
            .flatten()
            .is_none()
    }
    fn write(&mut self, value: Value) -> Result<()> {
        let mut bytes = serde_json::to_vec(&value)?;
        ensure!(
            bytes.len() < FRAME_LIMIT,
            "Open document exceeds agx's request frame limit"
        );
        bytes.push(b'\n');
        self.input.write_all(&bytes)?;
        self.input.flush()?;
        Ok(())
    }
    fn request(&mut self, method: &str, params: Value, cancel: &Cancellation) -> Result<Value> {
        ensure!(!cancel.is_cancelled(), "Search cancelled");
        self.next += 1;
        let id = format!("nain-{}", self.next);
        self.write(json!({"id":id,"method":method,"params":params}))?;
        let started = Instant::now();
        let mut cancellation_sent = false;
        loop {
            if cancel.is_cancelled() && !cancellation_sent {
                self.write(json!({"method":"cancel","params":{"request_id":id}}))?;
                cancellation_sent = true;
            }
            ensure!(
                started.elapsed() < JOB_TIMEOUT,
                "agx timed out; retry Code Search"
            );
            let message = match self.messages.recv_timeout(Duration::from_millis(25)) {
                Ok(message) => message?,
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(_) => bail!("agx stopped responding; retry Code Search"),
            };
            ensure!(
                message["protocol_version"] == 1,
                "Unsupported agx protocol; install agentgrep 0.3 or later"
            );
            if message.get("id").is_none() && message["method"] == "index/progress" {
                let params = &message["params"];
                ensure!(
                    params["request_id"] == id
                        && params["workspace_id"] == self.workspace
                        && params["session_id"] == self.session,
                    "Mismatched agx progress identity"
                );
                let progress: IndexProgress = serde_json::from_value(params.clone())?;
                ensure!(
                    self.roots.iter().any(|root| root.id == progress.root_id)
                        && matches!(progress.phase.as_str(), "started" | "indexing" | "complete"),
                    "Invalid agx indexing progress"
                );
                *cancel.progress.lock().unwrap() = Some(progress);
                continue;
            }
            ensure!(message["id"] == id, "Unexpected agx response identity");
            if cancellation_sent {
                bail!("Search cancelled");
            }
            if let Some(error) = message.get("error") {
                bail!(
                    "agx {}: {}",
                    error["code"].as_str().unwrap_or("error"),
                    error["message"].as_str().unwrap_or("Search failed")
                );
            }
            return message
                .get("result")
                .cloned()
                .context("agx response is missing a result");
        }
    }
    fn initialize(&mut self, cancel: &Cancellation) -> Result<()> {
        let value = self.request("initialize", json!({"protocol_version":1,"workspace_id":self.workspace,"restricted":true,"roots":self.roots,"limits":{"memory_bytes":134217728,"max_file_bytes":FILE_LIMIT,"max_files":25000,"response_bytes":FRAME_LIMIT}}), cancel)?;
        validate_capabilities(&value)?;
        self.session = value["session_id"]
            .as_str()
            .filter(|s| !s.is_empty())
            .context("agx session identity missing")?
            .into();
        ensure!(
            value["workspace_id"] == self.workspace,
            "Wrong agx workspace identity"
        );
        let returned: Vec<WorkspaceRoot> = serde_json::from_value(value["roots"].clone())?;
        ensure!(
            returned.len() == self.roots.len()
                && self.roots.iter().all(|root| returned.contains(root)),
            "agx root identity mismatch"
        );
        self.initialized = true;
        Ok(())
    }
    fn search(&mut self, request: SearchRequest, cancel: &Cancellation) -> Result<SearchOutput> {
        *self.active.lock().unwrap() = Some(Active {
            cancel: cancel.clone(),
            started: Instant::now(),
        });
        let result = self.search_inner(request, cancel);
        *self.active.lock().unwrap() = None;
        result
    }
    fn search_inner(
        &mut self,
        mut request: SearchRequest,
        cancel: &Cancellation,
    ) -> Result<SearchOutput> {
        ensure!(
            !request.query.trim().is_empty() && request.query.len() <= 4096,
            "Enter a query of at most 4096 bytes"
        );
        ensure!(
            request.globs.len() <= 63 && request.languages.len() <= 16,
            "Too many Code Search filters"
        );
        if !self.initialized {
            self.initialize(cancel)?;
        }
        let mut documents = BTreeMap::new();
        for document in std::mem::take(&mut request.documents) {
            let root = self
                .roots
                .iter()
                .find(|r| r.id == document.root_id)
                .context("Unknown document root")?;
            confined_path(&root.path, &document.path)?;
            ensure!(
                document.content.len() <= FILE_LIMIT,
                "Unsaved file {} exceeds Code Search's 2 MiB limit",
                document.path
            );
            documents.insert((document.root_id, document.path), document.content);
        }
        // Exact overlay versions retire saved/closed buffers; new edits always get newer versions.
        let retired = self
            .overlays
            .keys()
            .filter(|key| !documents.contains_key(*key))
            .cloned()
            .collect::<Vec<_>>();
        for key in retired {
            let version = self.overlays[&key].1;
            self.request(
                "document/close",
                json!({"root_id":key.0,"path":key.1,"version":version}),
                cancel,
            )?;
            self.overlays.remove(&key);
        }
        for (key, content) in documents {
            let hash = blake3::hash(content.as_bytes()).to_hex().to_string();
            if self.overlays.get(&key).is_some_and(|old| old.0 == hash) {
                continue;
            }
            let version = self.versions.get(&key).copied().unwrap_or(0) + 1;
            let value = self.request(
                "document/update",
                json!({"root_id":key.0,"path":key.1,"version":version,"content":content}),
                cancel,
            )?;
            ensure!(
                value["document_version"] == version,
                "agx did not acknowledge the current document"
            );
            self.versions.insert(key.clone(), version);
            self.overlays.insert(key, (hash, version));
        }
        let mut globs = request.globs.clone();
        globs.push("!*.ipynb".into());
        let mut results = Vec::new();
        for root in self.roots.clone() {
            let status = self.request(
                if request.rescan {
                    "index/rescan"
                } else {
                    "index/refresh"
                },
                json!({"root_id":root.id}),
                cancel,
            )?;
            let version = status["index_version"]
                .as_u64()
                .context("Missing index version")?;
            let value = self.request("search", json!({"root_id":root.id,"expected_index_version":version,"query":request.query,"mode":request.mode,"glob":globs,"languages":request.languages,"limit":100,"budget_bytes":64000}), cancel)?;
            let output: RootResults =
                serde_json::from_value(value).context("Invalid agx search result")?;
            self.validate_results(&output, &root, version, &request)?;
            results.push(output);
        }
        Ok(SearchOutput {
            roots: results,
            executable: self.executable.clone(),
        })
    }
    fn validate_results(
        &self,
        output: &RootResults,
        root: &WorkspaceRoot,
        version: u64,
        request: &SearchRequest,
    ) -> Result<()> {
        ensure!(
            output.schema_version == 2
                && output.session_id == self.session
                && output.workspace_id == self.workspace
                && output.root_id == root.id
                && output.index_version == version
                && output.query == request.query
                && output.mode == request.mode,
            "Stale or mismatched agx search result"
        );
        ensure!(
            output.returned_units == output.results.len()
                && output.results.len() <= 100
                && output.matched_units >= output.returned_units,
            "Invalid agx result counts"
        );
        for hit in &output.results {
            confined_path(&root.path, &hit.path)?;
            ensure!(
                hit.workspace_id == self.workspace
                    && hit.root_id == root.id
                    && hit.index_version == version
                    && hit.score.is_finite()
                    && hit.start_line > 0
                    && hit.end_line >= hit.start_line
                    && hit.source_range.start.line == hit.start_line
                    && hit.source_range.start.byte_column == 0
                    && hit.source_range.end.line == hit.end_line
                    && hit.source_range.end_exclusive,
                "Invalid agx source range/identity"
            );
            ensure!(
                hit.content_hash.len() == 64
                    && hit.content_hash.bytes().all(|c| c.is_ascii_hexdigit()),
                "Invalid agx content identity"
            );
            match hit.source.as_str() {
                "overlay" => ensure!(
                    self.overlays
                        .get(&(root.id.clone(), hit.path.clone()))
                        .is_some_and(|(hash, version)| *hash == hit.content_hash
                            && Some(*version) == hit.document_version),
                    "Stale unsaved-document result"
                ),
                "disk" => ensure!(
                    hit.document_version.is_none()
                        && !self
                            .overlays
                            .contains_key(&(root.id.clone(), hit.path.clone())),
                    "Disk result replaced an unsaved document"
                ),
                _ => bail!("Unknown agx source type"),
            }
        }
        Ok(())
    }
}
fn validate_capabilities(value: &Value) -> Result<()> {
    ensure!(
        value["protocol_version"] == 1
            && value["result_schema_version"] == 2
            && value["restricted"] == true,
        "agx must support restricted editor protocol v1/schema 2; install agentgrep 0.3 or later"
    );
    let modes = value["search_modes"]
        .as_array()
        .context("Missing agx search modes")?;
    ensure!(
        modes.len() == 3
            && ["text", "symbol", "ranked"]
                .iter()
                .all(|mode| modes.contains(&json!(mode))),
        "agx advertised unsupported search modes"
    );
    for key in ["network", "telemetry", "hybrid", "models"] {
        ensure!(
            value["capabilities"][key] == false,
            "agx must disable {key}"
        );
    }
    for key in [
        "cancellation",
        "document_overrides",
        "document_versions",
        "file_notifications",
    ] {
        ensure!(
            value["capabilities"][key] == true,
            "agx does not support {key}"
        );
    }
    ensure!(
        value["limits"]["request_bytes"]
            .as_u64()
            .is_some_and(|v| v >= FRAME_LIMIT as u64)
            && value["limits"]["max_file_bytes"] == FILE_LIMIT,
        "Incompatible agx document limits"
    );
    Ok(())
}
impl Drop for Worker {
    fn drop(&mut self) {
        self.stopped.store(true, Ordering::Release);
        if let Ok(mut child) = self.child.lock() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use tempfile::TempDir;

    fn fixture(case: &str) -> (TempDir, SearchRequest) {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join(".case"), case).unwrap();
        std::fs::write(directory.path().join("evidence.py"), "def needle(): pass\n").unwrap();
        let executable = directory.path().join("fake-agx");
        std::fs::write(&executable, include_str!("../tests/worker.py")).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let request = SearchRequest {
            executable: Some(executable),
            roots: vec![WorkspaceRoot {
                id: "root".into(),
                path: directory.path().to_owned(),
            }],
            documents: Vec::new(),
            query: "needle".into(),
            mode: SearchMode::Symbol,
            globs: vec!["*.py".into()],
            languages: vec!["python".into()],
            rescan: false,
        };
        (directory, request)
    }
    fn run(provider: &AgxProvider, request: SearchRequest) -> Result<SearchOutput> {
        let (_, result) = provider.search(request)?;
        block_on(result).context("Worker result dropped")?
    }
    #[test]
    fn worker_is_restricted_persistent_and_groups_roots() {
        let (directory, mut request) = fixture("normal");
        let second = tempfile::tempdir().unwrap();
        request.roots.push(WorkspaceRoot {
            id: "second".into(),
            path: second.path().to_owned(),
        });
        let provider = AgxProvider::new();
        assert_eq!(run(&provider, request.clone()).unwrap().roots.len(), 2);
        request.mode = SearchMode::Ranked;
        assert_eq!(
            run(&provider, request).unwrap().roots[0].mode,
            SearchMode::Ranked
        );
        let trace = std::fs::read_to_string(directory.path().join(".trace")).unwrap();
        let messages = trace
            .lines()
            .map(|s| serde_json::from_str::<Value>(s).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            messages
                .iter()
                .filter(|m| m["method"] == "initialize")
                .count(),
            1
        );
        assert!(messages.iter().all(|m| m["pid"] == messages[0]["pid"]));
        assert!(messages.iter().all(|m| {
            ["initialize", "index/refresh", "search"].contains(&m["method"].as_str().unwrap())
        }));
    }
    #[test]
    fn rejects_network_stale_oversized_and_escaped_results() {
        for case in [
            "network",
            "stale",
            "oversized",
            "escape",
            "notebook",
            "crash",
        ] {
            let (_directory, request) = fixture(case);
            assert!(
                run(&AgxProvider::new(), request).is_err(),
                "Accepted {case}"
            );
        }
    }
    #[test]
    fn installer_location_is_found_without_a_terminal_path() {
        const CHILD: &str = "NAIN_TEST_AGX_DISCOVERY_CHILD";
        if let Some(home) = std::env::var_os(CHILD) {
            let expected = PathBuf::from(home)
                .join(".agx/bin/agx")
                .canonicalize()
                .unwrap();
            assert_eq!(locate_executable(None).unwrap(), expected);
            return;
        }
        let home = tempfile::tempdir().unwrap();
        // Retain a Cargo installation too: the native install wins among HOME fallbacks.
        for relative in [".agx/bin/agx", ".cargo/bin/agx"] {
            let path = home.path().join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, "test executable").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
            }
        }
        // Run in a child so HOME/PATH changes cannot race with parallel Rust tests.
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "tests::installer_location_is_found_without_a_terminal_path",
            ])
            .env(CHILD, home.path())
            .env("HOME", home.path())
            .env("PATH", "")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    #[test]
    fn configured_invalid_executable_never_falls_back() {
        assert!(locate_executable(Some(Path::new("agx"))).is_err());
        assert!(locate_executable(Some(Path::new("/missing-nain-agx"))).is_err());
    }
    #[test]
    fn paths_are_confined_and_unsaved_new_files_are_allowed() {
        let root = tempfile::tempdir().unwrap();
        for path in [
            "",
            "/etc/passwd",
            "../outside",
            "src/../../outside",
            "src//bad",
            "src/./bad",
            "bad\\path",
            "sample.IPYNB",
        ] {
            assert!(confined_path(root.path(), path).is_err(), "Accepted {path}");
        }
        assert!(confined_path(root.path(), "new/file.py").is_ok());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/tmp", root.path().join("linked")).unwrap();
            assert!(confined_path(root.path(), "linked/file.py").is_err());
        }
    }
    #[test]
    fn cancellation_kills_an_unresponsive_worker_and_recovers() {
        let (directory, request) = fixture("hang");
        let provider = AgxProvider::new();
        let (cancel, result) = provider.search(request.clone()).unwrap();
        let start = Instant::now();
        loop {
            if std::fs::read_to_string(directory.path().join(".trace"))
                .unwrap_or_default()
                .contains("\"method\": \"search\"")
            {
                break;
            }
            assert!(start.elapsed() < Duration::from_secs(10));
            std::thread::sleep(Duration::from_millis(20));
        }
        cancel.cancel();
        assert!(block_on(result).unwrap().is_err());
        assert!(start.elapsed() < Duration::from_secs(8));
        std::fs::write(directory.path().join(".case"), "normal").unwrap();
        assert!(run(&provider, request).is_ok());
    }
    /// Run explicitly with NAIN_TEST_AGX pointing to a reviewed real worker.
    #[test]
    #[ignore = "requires separately installed agentgrep 0.3+; protocol fixtures run in normal CI"]
    fn real_worker_unsaved_buffers_filters_and_versions() {
        let executable = std::env::var_os("NAIN_TEST_AGX")
            .expect("Set NAIN_TEST_AGX to agx")
            .into();
        let root = tempfile::tempdir().unwrap();
        std::fs::write(
            root.path().join("main.py"),
            "def needle_disk():\n    return 'lexicaltoken'\n",
        )
        .unwrap();
        std::fs::write(
            root.path().join("ignored.py"),
            "def needle_ignored(): pass\n",
        )
        .unwrap();
        std::fs::write(root.path().join(".gitignore"), "ignored.py\n").unwrap();
        std::fs::write(root.path().join("notebook.ipynb"), "{\"needle\":true}").unwrap();
        let provider = AgxProvider::new();
        let mut request = SearchRequest {
            executable: Some(executable),
            roots: vec![WorkspaceRoot {
                id: "root".into(),
                path: root.path().to_owned(),
            }],
            documents: Vec::new(),
            query: "needle".into(),
            mode: SearchMode::Symbol,
            globs: Vec::new(),
            languages: vec!["python".into()],
            rescan: false,
        };
        let disk = run(&provider, request.clone()).unwrap();
        assert!(disk.roots[0].results.iter().any(|h| h.path == "main.py"));
        assert!(
            disk.roots[0]
                .results
                .iter()
                .all(|h| h.path != "ignored.py" && !h.path.ends_with(".ipynb"))
        );
        request.documents.push(Document {
            root_id: "root".into(),
            path: "main.py".into(),
            content: "def needle_unsaved():\n    return 'lexicaltoken'\n".into(),
        });
        let edited = run(&provider, request.clone()).unwrap();
        assert!(
            edited.roots[0]
                .results
                .iter()
                .all(|h| h.source == "overlay" && h.document_version == Some(1))
        );
        request.documents[0].content = "def needle_newest():\n    return 'lexicaltoken'\n".into();
        let edited = run(&provider, request.clone()).unwrap();
        assert!(
            edited.roots[0]
                .results
                .iter()
                .all(|h| h.document_version == Some(2))
        );
        request.mode = SearchMode::Ranked;
        request.query = "lexicaltoken".into();
        assert!(
            !run(&provider, request.clone()).unwrap().roots[0]
                .results
                .is_empty()
        );
        request.mode = SearchMode::Text;
        assert!(
            !run(&provider, request.clone()).unwrap().roots[0]
                .results
                .is_empty()
        );
        request.documents.clear();
        request.query = "needle".into();
        let closed = run(&provider, request.clone()).unwrap();
        assert!(closed.roots[0].results.iter().all(|h| h.source == "disk"));
        request.documents.push(Document {
            root_id: "root".into(),
            path: "main.py".into(),
            content: "def needle_reopen(): pass\n".into(),
        });
        assert!(
            run(&provider, request).unwrap().roots[0]
                .results
                .iter()
                .all(|h| h.document_version == Some(3))
        );
    }
}
