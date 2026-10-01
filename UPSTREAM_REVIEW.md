# Reviewed upstream release: v1.22.0

Incoming release: [`v1.22.0`](https://github.com/zed-industries/zed/releases/tag/v1.22.0), commit `76659a55a8c10ed355a070f8764a0b1733e3c115`.
Original development snapshot: `decbf641b18f1982b3475c037e7c5c554471574f`.

## Result

All five conflicts are resolved. The merge records the upstream parent for future maintenance, and `UPSTREAM_REVISION` now identifies the latest reviewed stable release. The fork retains its newer development snapshot and its existing application source; this is not a downgrade to upstream v1.22.0.

The remaining incoming changes relative to the fork were AI-specific assets/model-provider updates, an AI announcement in the updater UI, the announcement's `project` dependency, and the upstream stable channel marker. None is an applicable ordinary-editor fix. They were excluded rather than added to the fork.

## Decisions

- Retained the fork's updater UI and Cargo manifest/lockfile, omitting the Delta AI announcement and its dependency.
- Retained the existing unused provider source instead of importing additional OpenAI/OpenCode functionality. Those provider crates remain rejected by the production audit.
- Omitted Delta promotional assets and illustration code; retained the fork's existing shared UI implementation.
- Kept the development channel, separate app identity and fork-only automatic-update route. Apple signing remains available only through a manually dispatched release action.
- Preserved LSP completion, Python/notebook features, all AI/telemetry restrictions and the existing protected-source hashes without modification.

## Validation

- The complete editor/build scripts, dependencies and regression-test implementations are identical to the PR's previously validated source. Apple Silicon and Intel development builds passed for that source in [run 36767803640](https://github.com/ar4ft/zed-no-ai/actions/runs/36767803640).
- Protected-source and production dependency audits pass after conflict resolution.
- All six local maintenance tests pass; all fork workflows validate.
- No Apple-signed release is created by this merge.
