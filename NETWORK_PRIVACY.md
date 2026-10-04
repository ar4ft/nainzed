# Network behavior and privacy checks

Application event collection, telemetry identifiers, telemetry uploads, crash/minidump capture and uploading, and hang reporting remain removed. Settings cannot re-enable them or AI. The shared telemetry APIs are inert. Tests exercise the HTTP transport with a positive control, then verify telemetry startup, remote events, edit events and flushing issue zero requests. Event-property expressions are never evaluated.

`script/audit-no-ai-source` checks reviewed fingerprints for privacy/startup implementations, settings and dependency feature gates, update and signing controls, and privacy/build workflows. It also checks direct production manifests for newly introduced forbidden dependencies. `script/audit-no-ai` inspects resolved normal/build dependency trees for both the editor and remote helper, rejects AI/agent/provider families, telemetry collectors, and removed collaboration/audio implementations. Changing a protected file requires reviewing its behavior and regression tests before updating its fingerprint. New upstream socket, HTTP-client, process/background-task and collaboration/audio additions are flagged in maintenance reports for review. These checks support source review; they cannot classify arbitrary renamed future implementations.

Mac development packages and manually signed releases run `privacy-no-ai-mac.py`. This starts the actual application with a fresh isolated profile, opens plain text, and deliberately requests AI and telemetry enabled in settings. A local recording proxy returns 502 for every request and forwards no traffic. A positive control proves recording works. The check waits for the editor's visible window, observes another 15 seconds, and fails on **any** recorded connection attempt, including unfamiliar hosts and shared Zed service domains. It captures HTTP methods, destination hosts and proxy targets without recording request headers or bodies. Reports are uploaded even when the check fails. Development package jobs also upload startup/memory benchmarks.

The fresh-profile check disables automatic updates and the default HTML extension installation. Empty extension update lists return locally without contacting Zed. Installed extension updates and other user-requested downloads are not exercised in this startup test.

This is an HTTP(S) proxy test, not whole-system packet capture. Direct sockets, traffic that bypasses the configured proxy, SSH, extensions, and child processes are outside its observation. Combine it with the source safeguards and dependency audits; zero recorded requests alone is not proof that all possible editor workflows are offline. Opening Python notebooks is covered by protocol/recovery tests, not by this plain-text startup traffic check.

Expected network use during normal editing includes:

- Language-server and extension installation/update downloads, including GitHub and the Zed extension registry. Upstream extension registry domains also host other services; they are not broadly allowlisted by the startup check.
- User-selected Git remotes, SSH hosts, remote language servers and Jupyter servers.
- Explicit Python setup and `ipykernel` installation through the selected interpreter's package index.
- Automatic updates from `github.com/ar4ft/zed-no-ai` in signed releases only, when enabled. Development builds do not update automatically.

Downloaded tools and extensions have their own network behavior. Local editor logs and notebook recovery snapshots stay on disk. Recovery snapshots contain notebook source and outputs and use atomic temporary files with owner-only permissions on macOS. Recovery is local-project only; remote notebook backups and normal saves still go through the project filesystem.

On a Mac, run:

```sh
python3 script/privacy-no-ai-mac.py \
  --app '/Applications/Zed No AI.app' --output privacy.json
```
