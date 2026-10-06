#!/usr/bin/env python3
"""Compile the native safeguard test libraries together, then run selected tests."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import time


# Keeping this selection in one Cargo invocation unifies shared dependency features.
# Production dependency audits and application checks run separately without test features.
SUITES = (
    ('code_search_provider', None),
    ('code_search', 'no_ai_fork_code_search'),
    ('telemetry', None),
    ('client', 'no_telemetry_fork'),
    ('settings', 'no_ai_fork_keymap'),
    ('extension_host', 'no_ai_fork_empty_extension_update_check_never_sends'),
    ('title_bar', 'no_ai_fork_workspace_keeps_title_bar'),
    ('auto_update', 'no_ai_fork'),
    ('notebook_safety', None),
    ('project', 'no_ai_fork_settings_cannot_enable_ai'),
    ('repl', 'notebook::notebook_ui::tests::test_open_single_file_notebook'),
    ('repl', 'notebook::notebook_ui::tests::test_save_goes_through_the_project'),
    ('repl', 'no_ai_fork_notebook'),
    ('repl', 'kernels::native_kernel::test::test_get_kernelspecs'),
)
PACKAGES = tuple(dict.fromkeys(package for package, _ in SUITES))


def compile_tests(root):
    command = ['cargo', 'test', '--locked', '--no-run', '--lib', '--message-format=json']
    for package in PACKAGES:
        command.extend(('-p', package))
    binaries, library_paths = {}, set()
    process = subprocess.Popen(command, cwd=root, stdout=subprocess.PIPE, text=True)
    try:
        for line in process.stdout:
            try:
                message = json.loads(line)
            except json.JSONDecodeError:
                print(line, end='', flush=True)
                continue
            if message.get('reason') == 'compiler-message':
                rendered = message['message'].get('rendered')
                if rendered:
                    print(rendered, end='', flush=True)
            elif message.get('reason') == 'build-script-executed':
                for path in message.get('linked_paths', []):
                    if path.startswith('native='):
                        library_paths.add(path.removeprefix('native='))
            elif (message.get('reason') == 'compiler-artifact'
                  and message.get('profile', {}).get('test')
                  and 'lib' in message.get('target', {}).get('kind', [])
                  and message.get('executable')):
                package = message['target']['name']
                if package in PACKAGES:
                    executable = Path(message['executable']).resolve()
                    if package in binaries and binaries[package] != executable:
                        raise ValueError(f'Ambiguous test binary for {package}')
                    binaries[package] = executable
        returncode = process.wait()
    finally:
        if process.poll() is None:
            process.terminate()
        process.wait()
        process.stdout.close()
    if returncode:
        raise RuntimeError('Native test compilation failed; no tests were run')
    missing = set(PACKAGES) - binaries.keys()
    if missing:
        raise ValueError('Cargo did not produce test libraries: ' + ', '.join(sorted(missing)))
    return binaries, library_paths


def run_suite(root, binary, package, selector, library_paths):
    if not binary.is_file():
        raise ValueError(f'Test binary is missing: {binary}')
    environment = os.environ.copy()
    crate = root / 'crates' / package
    # Match cargo test's crate working directory and dynamic-library search paths.
    environment.update(CARGO_MANIFEST_DIR=str(crate), CARGO_MANIFEST_PATH=str(crate / 'Cargo.toml'))
    dynamic_key = 'DYLD_LIBRARY_PATH' if os.uname().sysname == 'Darwin' else 'LD_LIBRARY_PATH'
    search = [str(binary.parent), *sorted(library_paths)]
    if environment.get(dynamic_key):
        search.append(environment[dynamic_key])
    environment[dynamic_key] = os.pathsep.join(search)
    command = [str(binary)] + ([selector] if selector else [])
    listing = subprocess.run([*command, '--list', '--format=terse'], cwd=crate,
                             env=environment, text=True, capture_output=True, check=True)
    if not any(line.endswith(': test') for line in listing.stdout.splitlines()):
        raise ValueError(f'No tests matched {package}: {selector or "all"}')
    started = time.monotonic()
    result = subprocess.run(command, cwd=crate, env=environment, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print(result.stdout, end='', flush=True)
    if result.returncode:
        raise RuntimeError(f'Tests failed for {package}: {selector or "all"}')
    summary = re.search(r'test result: ok\. (\d+) passed;', result.stdout)
    if not summary or int(summary[1]) == 0:
        raise ValueError(f'No tests executed for {package}: {selector or "all"}')
    return {'package': package, 'filter': selector, 'passed': int(summary[1]),
            'seconds': round(time.monotonic() - started, 3)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=Path('target/runtime-reports/native-tests.json'))
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    report = {'suites': [], 'success': False}
    try:
        started = time.monotonic()
        binaries, library_paths = compile_tests(root)
        report['compile_seconds'] = round(time.monotonic() - started, 3)
        for package, selector in SUITES:
            print(f'::group::{package}: {selector or "all tests"}', flush=True)
            try:
                report['suites'].append(run_suite(root, binaries[package], package, selector, library_paths))
            finally:
                print('::endgroup::', flush=True)
        report['success'] = True
    except (OSError, ValueError, RuntimeError, subprocess.SubprocessError) as error:
        report['error'] = str(error)
        raise SystemExit(str(error))
    finally:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
