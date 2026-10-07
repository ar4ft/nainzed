import hashlib
import importlib.machinery
import importlib.util
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1]


def load(name, file):
    loader = importlib.machinery.SourceFileLoader(name, str(file))
    spec = importlib.util.spec_from_loader(name, loader)
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


maintenance = load('maintenance', SCRIPTS/'upstream-maintenance.py')
guards = load('guards', SCRIPTS/'audit-no-ai-source')


def git(root, *args):
    return subprocess.check_output(['git', *args], cwd=root, text=True, stderr=subprocess.DEVNULL).strip()


class MaintenanceTests(unittest.TestCase):
    def fixture(self, conflict=False, files=None):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        base = Path(temp.name)
        upstream, fork = base/'upstream', base/'fork'
        upstream.mkdir()
        git(upstream, 'init', '-b', 'main')
        git(upstream, 'config', 'user.name', 'Test')
        git(upstream, 'config', 'user.email', 'test@example.com')
        (upstream/'source.txt').write_text('original\n')
        for name, content in (files or {}).items():
            path = upstream/name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(content)
        git(upstream, 'add', '.')
        git(upstream, 'commit', '-m', 'Baseline')
        previous = git(upstream, 'rev-parse', 'HEAD')
        git(base, 'clone', str(upstream), str(fork))
        git(fork, 'config', 'user.name', 'Test')
        git(fork, 'config', 'user.email', 'test@example.com')
        (fork/'UPSTREAM_REVISION').write_text(previous+'\n')
        (fork/'fork.txt').write_text('AI and telemetry disabled\n')
        if conflict:
            (fork/'source.txt').write_text('fork disables telemetry\n')
        git(fork, 'add', '.')
        git(fork, 'commit', '-m', 'Fork safeguards')
        if conflict:
            (upstream/'source.txt').write_text('upstream collector changed\n')
        else:
            (upstream/'fix.txt').write_text('upstream editor fix\n')
        git(upstream, 'add', '.')
        git(upstream, 'commit', '-m', 'Stable fix')
        git(upstream, 'tag', 'v1.0.1')
        return fork, upstream, previous

    def test_clean_merge_preserves_fork_and_records_upstream_parent(self):
        fork, upstream, previous = self.fixture()
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        self.assertEqual(result['conflicts'], [])
        self.assertEqual((fork/'UPSTREAM_REVISION').read_text().strip(), result['target'])
        self.assertEqual((fork/'fork.txt').read_text(), 'AI and telemetry disabled\n')
        self.assertTrue((fork/'fix.txt').exists())
        self.assertIn(result['target'], git(fork, 'show', '-s', '--format=%P', 'HEAD').split())
        self.assertIn('Required human review', (fork/'UPSTREAM_REVIEW.md').read_text())
        self.assertEqual(git(fork, 'status', '--porcelain'), '')

    def test_conflict_aborts_merge_and_preserves_reference(self):
        fork, upstream, previous = self.fixture(True)
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        self.assertEqual(result['conflicts'], ['source.txt'])
        self.assertEqual((fork/'UPSTREAM_REVISION').read_text().strip(), previous)
        self.assertEqual((fork/'source.txt').read_text(), 'fork disables telemetry\n')
        self.assertEqual(len(git(fork, 'show', '-s', '--format=%P', 'HEAD').split()), 1)
        self.assertEqual(git(fork, 'diff', '--name-only', '--diff-filter=U'), '')

    def test_incorporated_release_is_skipped(self):
        fork, upstream, _ = self.fixture()
        maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        head = git(fork, 'rev-parse', 'HEAD')
        self.assertIsNone(maintenance.prepare(fork, 'v1.0.1', str(upstream), True))
        self.assertEqual(git(fork, 'rev-parse', 'HEAD'), head)

    def test_plan_does_not_modify_branch_or_files(self):
        fork, upstream, previous = self.fixture()
        head = git(fork, 'rev-parse', 'HEAD')
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream))
        self.assertIn('target', result)
        self.assertEqual(git(fork, 'rev-parse', 'HEAD'), head)
        self.assertEqual((fork/'UPSTREAM_REVISION').read_text().strip(), previous)

    def test_import_without_upstream_history_preserves_fork_changes(self):
        fork, upstream, previous = self.fixture()
        # Recreate an import with only files and no upstream commit ancestry.
        git(fork, 'checkout', '--orphan', 'import')
        git(fork, 'commit', '-m', 'Imported fork tree')
        imported_tree = git(fork, 'rev-parse', 'HEAD^{tree}')
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        self.assertEqual(result['conflicts'], [])
        ancestry = git(fork, 'rev-parse', 'HEAD^1')
        self.assertEqual(git(fork, 'rev-parse', ancestry+'^{tree}'), imported_tree)
        self.assertIn(previous, git(fork, 'show', '-s', '--format=%P', ancestry).split())
        self.assertEqual((fork/'fork.txt').read_text(), 'AI and telemetry disabled\n')
        self.assertTrue((fork/'fix.txt').exists())

    def advance(self, upstream, files):
        for name, content in files.items():
            path = upstream/name
            if content is None:
                path.unlink()
            else:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content)
        git(upstream, 'add', '-A')
        git(upstream, 'commit', '-m', 'Incoming behavior changes')
        git(upstream, 'tag', '-f', 'v1.0.1')

    def test_clean_merge_withholds_new_workflows_and_actions_and_reports_defaults(self):
        fork, upstream, _ = self.fixture()
        self.advance(upstream, {
            '.github/workflows/publish.yml': 'on: push\njobs: {}\n',
            '.github/actions/build/action.yml': 'runs:\n  using: composite\n',
            'assets/settings/default.json': '{"new_mode": true}\n',
        })
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        self.assertEqual(result['conflicts'], [])
        self.assertFalse((fork/'.github/workflows/publish.yml').exists())
        self.assertFalse((fork/'.github/actions/build/action.yml').exists())
        self.assertTrue((fork/'assets/settings/default.json').exists())
        report = result['report']
        self.assertIn('Manual approval required for this exact candidate, including a clean merge', report)
        self.assertIn('Incoming automation held out', report)
        self.assertIn('`.github/workflows/publish.yml`', report)
        self.assertIn('Defaults, settings, shortcuts or feature availability', report)
        self.assertIn('No keyword matches. Behavior impact remains unverified', report)
        self.assertIn('Actual changes to Nain requiring approval', report)
        self.assertEqual(git(fork, 'status', '--porcelain'), '')

    def test_clean_automation_modification_and_deletion_preserve_fork_workflows(self):
        initial = {'.github/workflows/ci.yml': 'original CI\n',
                   '.github/workflows/release.yml': 'manual releases\n'}
        fork, upstream, _ = self.fixture(files=initial)
        self.advance(upstream, {'.github/workflows/ci.yml': 'different CI\n',
                                '.github/workflows/release.yml': None})
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        self.assertEqual(result['conflicts'], [])
        for name, content in initial.items():
            self.assertEqual((fork/name).read_text(), content)
        self.assertIn('| Modified | `.github/workflows/ci.yml`', result['report'])
        self.assertIn('| Deleted | `.github/workflows/release.yml`', result['report'])

    def test_keyword_free_edit_and_removed_source_still_require_behavior_review(self):
        fork, upstream, _ = self.fixture(files={'crates/editor/src/editor.rs': 'old option\n'})
        self.advance(upstream, {'crates/editor/src/editor.rs': None,
                                'crates/workspace/src/menu.rs': 'new command\n'})
        result = maintenance.prepare(fork, 'v1.0.1', str(upstream), True)
        self.assertIn('| Deleted | `crates/editor/src/editor.rs`', result['report'])
        self.assertIn('Startup, UI, menus, window behavior or action registration', result['report'])
        self.assertIn('Editing, files, LSP, completion or search behavior', result['report'])
        self.assertIn('Approval expires on any new commit', result['report'])


class GuardTests(unittest.TestCase):
    def test_changed_missing_and_unprotected_implementations_fail(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            manifest = {}
            for name in guards.PROTECTED:
                path = root/name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('[dependencies]\n' if name.endswith('Cargo.toml') else 'disabled implementation\n')
                manifest[name] = hashlib.sha256(path.read_bytes()).hexdigest()
            (root/'assets').mkdir(exist_ok=True)
            file = root/'assets/no-ai-source-guards.json'
            file.write_text(json.dumps(manifest))
            guards.audit(root)
            protected = root/guards.PROTECTED[0]
            protected.write_text('collect private data\n')
            with self.assertRaises(ValueError):
                guards.audit(root)
            protected.unlink()
            with self.assertRaises(ValueError):
                guards.audit(root)
            file.write_text('{}')
            with self.assertRaises(ValueError):
                guards.audit(root)


if __name__ == '__main__':
    unittest.main()
