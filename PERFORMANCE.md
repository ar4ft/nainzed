# Performance checks

The production Apple Silicon normal/build dependency tree has fallen from 1,045 to 980 unique package/version pairs in this change (65 removed, none added). The initial fork had 1,145. Call/audio, channel UI, LiveKit, WebRTC, CPAL and Rodio implementations are excluded from the production editor. SSH, language servers, file browsing, Git, debugging and local notebooks remain enabled. Development/test dependencies and unused upstream source are retained.

The measured [headless results](PERFORMANCE_RESULTS.json) on Linux x86_64, using the optimized release profile and seven samples after warmup:

| Cells | Input size | Median save merge | Median recovery round trip |
| --- | --- | --- | --- |
| 50 | 16,053 bytes | 0.58 ms | 0.61 ms |
| 500 | 161,253 bytes | 5.94 ms | 5.43 ms |
| 2,000 | 648,753 bytes | 28.86 ms | 22.94 ms |

These measure the notebook preservation/recovery library. They exclude UI construction, editor buffers, disk writes and kernel startup. They are a baseline for regression checks, not evidence of faster Mac startup. Notebook UI serialization now indexes code cells once rather than scanning all cells for each output; search highlighting groups matches once rather than rescanning every match per cell. Output-only messages no longer invalidate cell-text searches.

Run the same headless check with:

```sh
cargo run --locked --release -p notebook_safety --example benchmark
```

Mac package jobs measure three launches each for an empty editor, 10 MiB and 100 MiB plain text files, and a 2,000-cell notebook. They use fresh isolated data/config profiles and disable default extension installation and automatic updates. First-window startup uses CoreGraphics to detect an on-screen layer-zero window belonging to the launched process. RSS is sampled five seconds after that window appears. OS caches are warm; this does not measure the moment the entire file becomes ready. Python environments, kernel processes and child-process memory are not included in RSS. CI runner hardware and load affect results. CI permits emulated GPUs to avoid the upstream hardware-warning dialog; reports record that permission. Compare results on representative physical Macs before drawing user performance conclusions.

Download `runtime-reports-macos-15` and `runtime-reports-macos-15-intel` from Actions for native measurements. A missing GUI session or a crashed process fails the check and is recorded as an error; missing measurements are not treated as success. Native Mac results for this change are pending its first package run.

To compare two applications on the same Mac:

```sh
python3 script/benchmark-no-ai-mac.py \
  --app '/Applications/nain.app' \
  --baseline '/path/to/previous/nain.app' \
  --repeat 3 --output benchmark.json
```

Keep the machine idle and compare repeated runs. The tool terminates only the processes it starts and never uses your normal editor profile. The baseline must support `--user-data-dir`. No performance threshold is enforced until representative native baselines are available.
