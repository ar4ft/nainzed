# nain

A macOS-focused fork of [Zed](https://github.com/zed-industries/zed) for text editing, folder/file browsing, Python, and Jupyter notebooks. No AI services are started by the application. Application telemetry collection and sending are removed. AI cannot be re-enabled with a user or project setting.

## What changed

- Removes usage-event collection, telemetry logs and uploaders, editor activity tracking, telemetry identifiers, crash/minidump capture and uploads, hang reports, build-timing uploads, and remote event/diagnostic forwarding. Telemetry event expressions are ignored without evaluating them; shared upstream callers use inert compatibility APIs. Telemetry settings are permanently false and the telemetry controls are removed. Local editor logs and session restore remain available.
- Enforces `disable_ai: true` in the settings loader, even if settings request `false`.
- Removes startup for Copilot, chat/model providers, edit predictions, agents, agent registry downloads, AI web search, prompt loading, and the global agent-rules watcher.
- Removes the agent panel and threads sidebar, AI prediction status control, AI toolbars, AI onboarding, AI settings pages, and AI skill-install handlers.
- Drops AI keyboard bindings and hides AI commands, including editor prediction commands and Git commit-message generation.
- Keeps the shared model registry empty and rejects provider registration, including extension providers.
- Removes AI provider and agent crates from the production dependency tree, including the AI settings page implementations.
- Creates a `.bak` copy before overwriting a notebook, refuses conflicting edits, and preserves rich outputs, Markdown attachments, and unknown notebook fields.
- Keeps a draggable title bar above editor tabs and an account-free **Menu** with settings, keymaps, themes, and extensions.
- Adds **repl: Set Up Python** and an **Install ipykernel** prompt in the kernel menu.
- Adds notebook search across code and Markdown cells, per-cell output collapsing, and saved/unsaved status with tab updates.
- Adds kernel-backed notebook autocomplete, project Python environment selection, local unsaved-draft recovery, and restoration of the last 20 deleted cells.
- Handles Jupyter clear-output and shared display updates while preserving their saved data.
- Excludes collaboration/call/audio, LiveKit and WebRTC implementations from production builds.
- Runs validation and installer compilation in parallel, consolidates native test compilation, and uses bounded Kache compiler snapshots. Publication waits for all checks.
- Enables the upstream experimental `.ipynb` editor without an account or remote feature flag. Python `# %%` script cells also remain available.
- Offers theme, keymap, settings import, and Vim setup on first launch without agent or telemetry controls and rejects AI agent/skill deep links.
- Uses separate app data (`ZedNoAI`) and configuration (`~/.config/zednoai`) so the fork can coexist with upstream Zed. These storage paths are retained after the nain rename to preserve settings and recovered drafts.
- Signed releases automatically update from this fork's GitHub releases, verify the Apple team, bundle identity, notarization assessment, and version before replacing the app, and allow automatic updates to be disabled. Ad-hoc builds keep updates disabled.

Folder browsing, tabs, file outline, search, language-server completion, syntax highlighting, terminal, Git, debugging, and Vim mode are inherited from Zed. Language-server completion is ordinary code completion and does not use an AI model.

**Scope:** this is a fork with AI functionality disabled and its application integration removed. Shared AI settings/model types remain in upstream editor, Git, and title-bar components; the full upstream workspace also retains unused AI source. The production dependency audit rejects agent, Copilot, edit prediction engine, and model provider crates. Compared with the initial fork, the Apple Silicon production/build dependency tree contains 165 fewer package/version pairs (1,145 → 980), including 65 removed by the latest collaboration/audio trimming. It is not yet a source tree or binary proven to contain zero AI-related code. Headless notebook measurements and native benchmark instructions are in [PERFORMANCE.md](PERFORMANCE.md); native startup/memory results for this change are pending CI. The notebook UI is experimental upstream functionality. See [NETWORK_PRIVACY.md](NETWORK_PRIVACY.md) for network behavior and the limits of the proxy-based startup check.

## Build on a Mac

Install full Xcode, select it with `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`, accept its license, and install [Rust via rustup](https://rustup.rs/). The Rust version is pinned in `rust-toolchain.toml`.

```sh
brew install cmake pkg-config
./script/build-no-ai-mac
```

For signed, notarized releases and automatic updates, follow [RELEASES.md](RELEASES.md). Signing runs only when **Signed nain release** is manually dispatched; branch builds produce development packages without Apple credentials, and tag pushes do not start signing. Apple Developer enrollment and repository signing/notarization secrets are required; the release pipeline is implemented but a signed release has not yet been produced.

The script creates `target/release/bundle/osx/nain.app`, plus ZIP and DMG packages in `target/nain-arm64/` or `target/nain-x86_64/`. It builds for the current Mac architecture. It uses ad-hoc signing; downloaded builds are not Apple-notarized. Use Finder's Open action or macOS Privacy & Security to approve a build you trust.

The **Build nain for Mac** workflow runs Linux source/privacy guards and Apple Silicon/Intel regression checks through **Validate nain for Mac**. PRs stop after validation; branch pushes and manual development builds create packages concurrently with validation, then run actual-application startup privacy and performance checks. After validation and packaging pass on both architectures, `main` builds publish a [development prerelease](https://github.com/ar4ft/nainzed/releases) with DMG and ZIP installers for Apple Silicon and Intel, plus SHA-256 checksums. PRs and upstream review builds do not publish releases. Development prereleases are not Apple-notarized and do not enter the stable automatic-update feed. Cache restore/save are best effort and limited to eight minutes each. Kache compiler statistics and shared-test timings accompany runtime reports in the repository’s Actions tab; see [PERFORMANCE.md](PERFORMANCE.md) for cache limits and cold/warm build comparisons. No Apple credentials are needed for development builds.

The [validated Mac build](https://github.com/ar4ft/nainzed/actions/runs/36767803640) passed the AI/telemetry checks and notebook tests and produced packages for both Apple Silicon and Intel. It predates the notebook search, output collapsing, and status improvements; use a successful later run for those changes. Nine standalone notebook preservation/recovery tests pass locally. Native runtime checks for the latest changes must pass in the new package run before treating those packages as validated.

## Python and Jupyter

Open your project folder, run **repl: Set Up Python** from the command palette (or **Set Up Python** in the notebook kernel menu), and confirm setup. This creates or reuses the folder’s `.venv`, installs `ipykernel`, and makes **Python (.venv)** available in the kernel menu. Selecting an existing Python environment without `ipykernel` offers to install it in that interpreter. Setup requires Python 3 and network access to the package index. Remote environments must be prepared on their remote host.

You can also create a Python kernel manually:

```sh
python3 -m venv ~/.venvs/nain
~/.venvs/nain/bin/python -m pip install ipykernel
~/.venvs/nain/bin/python -m ipykernel install --user --name nain --display-name 'Python (nain)'
```

Open `examples/python.ipynb`, select **Python (.venv)** (or your manually installed kernel) in the kernel picker, and run a cell with **Shift+Enter**. **Cmd+Enter** runs a cell; **Cmd+Shift+Enter** runs all cells. The notebook editor supports code and Markdown cells, outputs, kernel interrupt/restart, and saving. These capabilities are inherited from upstream and require Mac runtime validation; preserve a copy of valuable notebooks while evaluating the experimental editor.

Use **Cmd+F** to search code and Markdown source across the notebook, with case-sensitive, whole-word, and regular-expression options. Search navigation scrolls to the matching cell and opens Markdown source when needed. **Hide outputs / Show outputs** collapses a code cell’s output for this session without changing the notebook file or discarding results. The footer shows **Saved**, **Unsaved changes**, or **Saving…**, and edits update the tab’s dirty indicator.

Notebook completion uses Jupyter `complete_request` / `complete_reply` from the running kernel, including variables and installed packages in its environment. Use the editor’s normal completion shortcut or type a word/dot. The notebook chooses the project’s active Python environment when it has `ipykernel`, while an explicit kernel selection takes priority. Selecting a detected Python environment also activates that interpreter for project Python tooling. Kernel completion needs a running kernel; it does not provide the full language-server diagnostics available in `.py` files.

Local notebooks save unsaved drafts to the fork’s data directory after 750 ms of quiet time. Reopening offers **Recover** or **Discard**; recovery marks the notebook dirty and leaves its disk file untouched until Save. Independently changed or malformed recovery snapshots are retained under a separate conflict filename and their location is shown for manual reconciliation. **Restore deleted cell** restores the last deleted cell’s source and outputs, up to 20 cells in the current session. Remote notebooks do not use local recovery snapshots.

Each save that overwrites an existing notebook first updates `<filename>.ipynb.bak` with its previous contents. A backup failure aborts the save and leaves edits dirty. Restore by copying that backup over the notebook while it is closed. This is a single previous version, not a version history.

For a script-based notebook, open `examples/python_cells.py` and use the REPL actions on `# %%` cells.

## Upstream maintenance

A weekly stable-update workflow opens draft PRs, reports conflicts and sensitive changes, and runs the AI/telemetry safeguards and Mac checks without automatically merging or signing. See [UPSTREAM_MAINTENANCE.md](UPSTREAM_MAINTENANCE.md) for the schedule, source-guard review and conflict recovery.

## Repository

The fork is published at [ar4ft/nainzed](https://github.com/ar4ft/nainzed). `main` is the default and maintained branch. The original source snapshot is upstream `decbf641b18f1982b3475c037e7c5c554471574f`; `UPSTREAM_REVISION` records the most recently reviewed upstream stable release.

```sh
git clone https://github.com/ar4ft/nainzed.git
cd nainzed
./script/build-no-ai-mac
```

The existing upstream history and license notices are preserved.

## License and attribution

The application retains Zed's GPL-3.0-or-later license; applicable components retain their Apache-2.0 or other upstream licenses. See `LICENSE-GPL`, `LICENSE-APACHE`, and the notices generated by `script/generate-licenses`. Distribute matching source with binaries as required by their licenses. This independent fork is not an official Zed release. The original upstream README is in `README.upstream.md`.
