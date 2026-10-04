#!/usr/bin/env python3
"""Measure first-window startup and RSS on macOS; optionally compare a prior app."""
import argparse
import json
import os
from pathlib import Path
import platform
import statistics
import subprocess
import tempfile
import time

from no_ai_runtime import MacWindows, executable, launch, notebook, profile, retain_logs, stop, wait_window


def measure(binary, repeats, logs):
    results = []
    windows = MacWindows()
    with tempfile.TemporaryDirectory(prefix='no-ai-benchmark-') as temporary:
        root = Path(temporary)
        small = root / '10MiB.txt'
        large = root / '100MiB.txt'
        for path, size in [(small, 10), (large, 100)]:
            with path.open('wb') as file:
                block = b'plain text benchmark fixture\n' * 4096
                remaining = size * 1024 * 1024
                while remaining:
                    data = block[:remaining]
                    file.write(data)
                    remaining -= len(data)
        cells = root / '2000-cells.ipynb'
        cells.write_text(json.dumps(notebook(2000)))
        for name, files in [('empty', []), ('10MiB', [small]), ('100MiB', [large]), ('2000-cells', [cells])]:
            samples = []
            for iteration in range(repeats):
                data = profile(root / f'{name}-{iteration}')
                with (root / f'{name}-{iteration}.log').open('wb') as log:
                    started = time.monotonic()
                    process = launch(binary, data, files, log)
                    try:
                        wait_window(process, windows)
                        first_window_ms = (time.monotonic() - started) * 1000
                        time.sleep(5)  # Fixed settling period, not a claim that file loading finished.
                        if process.poll() is not None:
                            raise RuntimeError('Editor exited during workload sampling')
                        rss = int(subprocess.check_output(['ps', '-o', 'rss=', '-p', str(process.pid)], text=True).strip())
                        samples.append({'first_window_ms': first_window_ms, 'rss_kib_after_5s': rss})
                    finally:
                        stop(process)
                        log.flush()
                        retain_logs(data, log.name, logs, f'{name}-{iteration}')
            results.append({'workload': name, 'samples': samples,
                            'median_first_window_ms': statistics.median(s['first_window_ms'] for s in samples),
                            'median_rss_kib_after_5s': statistics.median(s['rss_kib_after_5s'] for s in samples)})
    return results


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', required=True)
    parser.add_argument('--baseline')
    parser.add_argument('--repeat', type=int, default=3)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if platform.system() != 'Darwin' or args.repeat < 1:
        parser.error('Requires macOS and at least one repeat')
    report = {'platform': platform.platform(), 'architecture': platform.machine(),
              'gpu_emulation_allowed': os.environ.get('ZED_ALLOW_EMULATED_GPU') == '1',
              'method': 'Fresh isolated profiles; warm OS caches; first visible window; RSS 5s later. File-ready time is not measured.'}
    try:
        report['current'] = measure(executable(args.app), args.repeat, args.output.parent / 'logs/current')
        if args.baseline:
            report['baseline'] = measure(executable(args.baseline), args.repeat, args.output.parent / 'logs/baseline')
    except Exception as error:
        report['error'] = str(error)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    if report.get('error'):
        raise SystemExit(1)


if __name__ == '__main__':
    main()
