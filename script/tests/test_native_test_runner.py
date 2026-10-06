import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import MagicMock, patch

spec = importlib.util.spec_from_file_location(
    'native_tests', Path(__file__).resolve().parents[1] / 'test-no-ai-mac.py')
native = importlib.util.module_from_spec(spec)
spec.loader.exec_module(native)


def artifact(package, executable, kind='lib', test=True):
    return {'reason': 'compiler-artifact', 'target': {'name': package, 'kind': [kind]},
            'profile': {'test': test}, 'executable': str(executable)}


def compiler(messages, returncode=0):
    process = MagicMock()
    process.stdout = io.StringIO(''.join(json.dumps(item) + '\n' for item in messages))
    process.wait.return_value = returncode
    process.poll.return_value = returncode
    return process


class NativeTestRunner(unittest.TestCase):
    def test_shared_compile_selects_only_test_library_executables(self):
        messages = [artifact(name, f'/tmp/{name}-test') for name in native.PACKAGES]
        messages += [artifact('repl', '/tmp/repl-example', kind='example'),
                     artifact('repl', '/tmp/repl-production', test=False),
                     {'reason': 'build-script-executed', 'linked_paths': ['native=/tmp/native-libs']}]
        with patch.object(native.subprocess, 'Popen', return_value=compiler(messages)) as popen:
            binaries, paths = native.compile_tests(Path('/tmp/workspace'))
        popen.assert_called_once()
        command = popen.call_args.args[0]
        self.assertEqual(command[:6], ['cargo', 'test', '--locked', '--no-run', '--lib', '--message-format=json'])
        self.assertEqual(command.count('-p'), len(native.PACKAGES))
        self.assertEqual(binaries['repl'], Path('/tmp/repl-test'))
        self.assertEqual(paths, {'/tmp/native-libs'})

    def test_invalid_compilation_cannot_be_treated_as_passing_tests(self):
        messages = [artifact(name, f'/tmp/{name}-test') for name in native.PACKAGES]
        for output, code, error in [(messages, 1, 'compilation failed'),
                                    (messages[:-1], 0, 'did not produce'),
                                    (messages + [artifact('repl', '/tmp/another')], 0, 'Ambiguous')]:
            with self.subTest(error=error), patch.object(
                native.subprocess, 'Popen', return_value=compiler(output, code)):
                with self.assertRaisesRegex((ValueError, RuntimeError), error):
                    native.compile_tests(Path('/tmp/workspace'))

    def test_library_runner_matches_cargo_context_and_fails_closed(self):
        # A subprocess fixture verifies cwd and loader environment instead of mocking them.
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            crate = root / 'crates' / 'repl'
            crate.mkdir(parents=True)
            binary = root / 'test-binary'
            binary.write_text('''#!/usr/bin/env python3
import os, pathlib, sys
assert pathlib.Path.cwd() == pathlib.Path(os.environ['CARGO_MANIFEST_DIR'])
assert pathlib.Path(os.environ['CARGO_MANIFEST_PATH']).parent == pathlib.Path.cwd()
key = 'DYLD_LIBRARY_PATH' if sys.platform == 'darwin' else 'LD_LIBRARY_PATH'
assert str(pathlib.Path(__file__).parent) in os.environ[key].split(os.pathsep)
assert '/tmp/native-libs' in os.environ[key].split(os.pathsep)
selector = sys.argv[1]
if '--list' in sys.argv:
    if selector != 'renamed': print('example::test: test')
elif selector == 'failed':
    print('test result: FAILED. 0 passed; 1 failed;')
    sys.exit(1)
elif selector == 'ignored':
    print('test result: ok. 0 passed; 0 failed; 1 ignored;')
else:
    print('test result: ok. 2 passed; 0 failed; 0 ignored;')
''')
            binary.chmod(0o755)
            with patch('sys.stdout', new_callable=io.StringIO):
                result = native.run_suite(root, binary, 'repl', 'selected', {'/tmp/native-libs'})
                self.assertEqual(result['passed'], 2)
                for selector, error in [('renamed', 'No tests matched'), ('failed', 'Tests failed'),
                                        ('ignored', 'No tests executed')]:
                    with self.subTest(selector=selector):
                        with self.assertRaisesRegex((ValueError, RuntimeError), error):
                            native.run_suite(root, binary, 'repl', selector, {'/tmp/native-libs'})
                binary.unlink()
                with self.assertRaisesRegex(ValueError, 'missing'):
                    native.run_suite(root, binary, 'repl', 'selected', set())

    def test_failed_suite_is_written_to_report_and_stops_later_suites(self):
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / 'report.json'
            with patch('sys.argv', ['test-no-ai-mac.py', '--output', str(output)]), \
                 patch('sys.stdout', new_callable=io.StringIO), \
                 patch.object(native, 'compile_tests', return_value=(
                     {name: Path('/tmp/binary') for name in native.PACKAGES}, set())), \
                 patch.object(native, 'run_suite', side_effect=RuntimeError('fixture failure')) as run:
                with self.assertRaisesRegex(SystemExit, 'fixture failure'):
                    native.main()
            run.assert_called_once()
            report = json.loads(output.read_text())
            self.assertFalse(report['success'])
            self.assertEqual(report['error'], 'fixture failure')
            self.assertIn('compile_seconds', report)


if __name__ == '__main__':
    unittest.main()
