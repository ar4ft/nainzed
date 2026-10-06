# Code Search with agentgrep

Code Search is an optional, dockable panel built into nain. Install the normal **agx** command from [agentgrep](https://github.com/ar4ft/agentgrep); no separate Mac application, extension adapter, account, or model is needed. Use a nain build containing this integration.

## Install agx

Agentgrep must support restricted editor protocol v1 and result schema 2 (available since 0.3.0). On Apple Silicon and Intel Macs, the [official installer](https://github.com/ar4ft/agentgrep/blob/main/docs/installation.md) downloads the prebuilt executable without Rust or Homebrew:

```sh
curl -fsSL https://raw.githubusercontent.com/ar4ft/agentgrep/main/scripts/install.sh | sh
. "$HOME/.agx/env"
agx --version
```

The default selects the newest published release, including development prereleases. Current agentgrep releases are unsigned development builds; the installer does not enable automatic updates. See agentgrep's installation guide for version pins, `--stable`, script inspection, and signing details. Rerun the installer explicitly to upgrade, then restart nain so existing workers use the new binary.

The installer uses `~/.agx/bin/agx` and can update your shell profile. Nain also checks that location directly, so Finder launches do not depend on sourcing the shell profile. Discovery order is an explicitly configured executable, absolute directories in nain's PATH, `~/.agx/bin/agx`, `~/.cargo/bin/agx`, `~/.local/bin/agx`, `/opt/homebrew/bin/agx`, and `/usr/local/bin/agx`. Existing installations on PATH keep priority; check `command -v agx` if you have multiple copies.

Source installation remains supported:

```sh
git clone https://github.com/ar4ft/agentgrep.git
cd agentgrep
cargo install --path . --locked
```

Cargo installs `agx` into `~/.cargo/bin`. A custom installer `--prefix` puts it under `PREFIX/bin/agx`; configure that absolute path in nain.

If discovery fails, open **Menu → Settings** or **Open Settings File** from the command palette and add the following using your actual Mac username:

```json
{
  "code_search": {
    "agx_path": "/Users/YOUR_USERNAME/.agx/bin/agx"
  }
}
```

Use an absolute executable path; `~`, environment-variable expansion, shell commands, and relative paths are not accepted. An invalid explicit path fails visibly instead of selecting a different executable. Installing and updating agx remain user actions; nain never downloads it or runs its updater.

## Use the panel

Open a local project folder. Click the **Code Search** code icon in the panel dock or run **code search: Toggle Focus** from the command palette. The macOS shortcut is **Cmd+Ctrl+Shift+F**. The panel docks alongside Files and Outline and can be moved left/right and resized through nain's dock controls. Its dock icon indicates when the panel is active. Dock position and resized width are saved by the workspace.

- **Text** searches literal source text.
- **Symbol** searches parsed symbol names.
- **Ranked (BM25)** ranks source fragments lexically; it does not use embeddings or models.

Queries debounce for 350 ms. File and language filters accept comma-separated values, for example `*.rs, !tests/**` and `rust, python`. Agentgrep's supported language names and ignore rules apply. Arrow keys navigate hits, a single click shows a read-only source preview, and **Enter**, a double click, or **Open result** opens the file and selects the source range. **Cmd+Enter** reruns the query. **Escape** or Cancel stops an active query. Refresh rescans file hashes before searching, including changes missed by watchers.

Results are grouped by project folder; BM25 scores from different roots are not combined. The status displays indexing counts, matches, truncation, skipped files, and incomplete indexes. Hover it for the full message. Errors explain missing/incompatible executables or why a query failed; correct the setting or retry. Existing project search and **Cmd+Shift+F** keep their behavior.

## Local data and limits

The subprocess receives only structured argv `agx serve --stdio --restricted` and newline-delimited JSON. The handshake requires network, telemetry, models, and hybrid search to be disabled. The adapter exposes only text, symbol, and ranked modes and imports no agent/model-provider or telemetry implementations. It never invokes a shell, `doctor`, MCP, embeddings, or an agent skill.

Indexing starts on the first nonempty query, not at application launch. One persistent worker per workspace reuses parsed files, with metadata refreshes on later searches. Closed workspaces stop and reap the worker off the UI thread. Superseded queries are cancelled; unresponsive workers are terminated, and a later query starts a fresh worker. A crashed worker also restarts on the next query.

Dirty, named text buffers are sent as in-memory document overrides before searching. Their acknowledged versions and content hashes are checked; nain does not write unsaved source to temporary files for this integration. Saving or closing a buffer retires its override on the next search. Query generations discard superseded results; opening a hit additionally checks current buffer contents before selecting a range.

The first version supports 1–8 local project folders, up to 25,000 files per root, 2 MiB per file, a negotiated 128 MiB worker index budget, 16 MiB of unsaved-buffer text per request, and 100 results per folder. These bounds can omit results; the panel reports limits rather than claiming the full project was searched. Hidden, ignored, binary, and unsupported files follow agentgrep's traversal rules. A path that escapes the project or crosses a symlink is rejected. Remote projects, untitled buffers without a path, and notebook JSON are outside this panel. Use nain's ordinary search or cell-aware notebook search for those cases.

Queries and source stay in the local worker; the adapter does not retain worker stderr or log query/source payloads. This is an optional external executable that you install and trust. Nain's startup proxy test does not observe subprocess traffic; protocol validation and source review cover the restricted worker contract. Agentgrep's standalone CLI may offer other modes, but nain cannot request them.

## Verify the integration

Normal CI runs deterministic subprocess tests covering the restricted argv/handshake, persistent workers, multi-root grouping, cancellation/restart, frame bounds, stale responses, path confinement, and Finder-style discovery without a terminal PATH, plus native panel tests for navigation, literal previews, unsaved-buffer collection, notebook exclusion, and disabled settings. The production AI/telemetry dependency and source audits still run.

To test against your installed real worker:

```sh
NAIN_TEST_AGX="$HOME/.agx/bin/agx" cargo test --locked \
  -p code_search_provider --lib real_worker -- --ignored
```

The real-worker integration test is explicit because CI does not download/install an optional runtime tool into the app. This implementation was tested against agentgrep 0.3.0 and 0.3.3, including all three search modes and unsaved-buffer version transitions. Native macOS UI and packaged-app validation run through the existing Apple Silicon and Intel pipeline before prerelease publication.
