# Zed No AI

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
- Adds **repl: Set Up Python** and an **Install ipykernel** prompt in the kernel menu.
- Enables the upstream experimental `.ipynb` editor without an account or remote feature flag. Python `# %%` script cells also remain available.
- Opens a plain editor on first launch and rejects AI agent/skill deep links.
- Uses separate app data (`ZedNoAI`) and configuration (`~/.config/zednoai`) so the fork can coexist with upstream Zed.
- Signed releases automatically update from this fork's GitHub releases, verify the Apple team, bundle identity, notarization assessment, and version before replacing the app, and allow automatic updates to be disabled. Ad-hoc builds keep updates disabled.

Folder browsing, tabs, file outline, search, language-server completion, syntax highlighting, terminal, Git, debugging, and Vim mode are inherited from Zed. Language-server completion is ordinary code completion and does not use an AI model.

**Scope:** this is a fork with AI functionality disabled and its application integration removed. Shared AI settings/model types remain in upstream editor, Git, and title-bar components; the full upstream workspace also retains unused AI source. The production dependency audit rejects agent, Copilot, edit prediction engine, and model provider crates. Compared with the initial fork, the Apple Silicon production/build dependency tree contains 100 fewer package/version pairs (1,145 → 1,045). It is not yet a source tree or binary proven to contain zero AI-related code. No startup/performance benchmark has been run. The notebook UI is experimental upstream functionality.

## Build on a Mac

Install full Xcode, select it with `sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer`, accept its license, and install [Rust via rustup](https://rustup.rs/). The Rust version is pinned in `rust-toolchain.toml`.

```sh
brew install cmake pkg-config
./script/build-no-ai-mac
```

For signed, notarized releases and automatic updates, follow [RELEASES.md](RELEASES.md). Signing runs only when **Signed Zed No AI release** is manually dispatched; branch builds produce development packages without Apple credentials, and tag pushes do not start signing. Apple Developer enrollment and repository signing/notarization secrets are required; the release pipeline is implemented but a signed release has not yet been produced.

The script creates `target/release/bundle/osx/Zed No AI.app`, plus ZIP and DMG packages in `target/no-ai-arm64/` or `target/no-ai-x86_64/`. It builds for the current Mac architecture. It uses ad-hoc signing; downloaded builds are not Apple-notarized. Use Finder's Open action or macOS Privacy & Security to approve a build you trust.

The **Build Zed No AI for Mac** GitHub Actions workflow builds Apple Silicon and Intel packages and runs the dependency audit, telemetry regression tests, production application check, enforced AI-settings test, notebook preservation tests, and notebook open/save/backup-failure tests. Download successful build artifacts from the repository's Actions tab.

The earlier [validated Apple Silicon build](https://github.com/ar4ft/zed-no-ai/actions/runs/36673142318) passed the AI/notebook checks and produced ZIP and DMG packages, but predates telemetry removal. Use a successful run containing the telemetry-removal commit for the updated app. Intel passed all checks and notebook tests in the same run, but its release build exceeded the original three-hour job limit. The workflow now allows six hours and saves Rust caches even when a step fails. A completed Intel package remains pending. The complete application and notebook/settings test targets also compile on Linux, and five standalone notebook preservation tests pass locally. No startup or memory benchmark has been run.

## Python and Jupyter

Open your project folder, run **repl: Set Up Python** from the command palette (or **Set Up Python** in the notebook kernel menu), and confirm setup. This creates or reuses the folder’s `.venv`, installs `ipykernel`, and makes **Python (.venv)** available in the kernel menu. Selecting an existing Python environment without `ipykernel` offers to install it in that interpreter. Setup requires Python 3 and network access to the package index. Remote environments must be prepared on their remote host.

You can also create a Python kernel manually:

```sh
python3 -m venv ~/.venvs/zednoai
~/.venvs/zednoai/bin/python -m pip install ipykernel
~/.venvs/zednoai/bin/python -m ipykernel install --user --name zednoai --display-name 'Python (Zed No AI)'
```

Open `examples/python.ipynb`, select **Python (.venv)** (or your manually installed kernel) in the kernel picker, and run a cell with **Shift+Enter**. **Cmd+Enter** runs a cell; **Cmd+Shift+Enter** runs all cells. The notebook editor supports code and Markdown cells, outputs, kernel interrupt/restart, and saving. These capabilities are inherited from upstream and require Mac runtime validation; preserve a copy of valuable notebooks while evaluating the experimental editor.

Each save that overwrites an existing notebook first updates `<filename>.ipynb.bak` with its previous contents. A backup failure aborts the save and leaves edits dirty. Restore by copying that backup over the notebook while it is closed. This is a single previous version, not a version history.

For a script-based notebook, open `examples/python_cells.py` and use the REPL actions on `# %%` cells.

## Upstream maintenance

A weekly stable-update workflow opens draft PRs, reports conflicts and sensitive changes, and runs the AI/telemetry safeguards and Mac checks without automatically merging or signing. See [UPSTREAM_MAINTENANCE.md](UPSTREAM_MAINTENANCE.md) for the schedule, source-guard review and conflict recovery.

## Repository

The fork is published at [ar4ft/zed-no-ai](https://github.com/ar4ft/zed-no-ai). The AI-disabled source is on the `no-ai` branch. The original source snapshot is upstream `decbf641b18f1982b3475c037e7c5c554471574f`; `UPSTREAM_REVISION` records the most recently reviewed upstream stable release.

```sh
git clone --branch no-ai https://github.com/ar4ft/zed-no-ai.git
cd zed-no-ai
./script/build-no-ai-mac
```

The existing upstream history and license notices are preserved.

## License and attribution

The application retains Zed's GPL-3.0-or-later license; applicable components retain their Apache-2.0 or other upstream licenses. See `LICENSE-GPL`, `LICENSE-APACHE`, and the notices generated by `script/generate-licenses`. Distribute matching source with binaries as required by their licenses. This independent fork is not an official Zed release. The original upstream README is in `README.upstream.md`.
