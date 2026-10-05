import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch


SCRIPT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('prerelease', SCRIPT / 'publish-no-ai-prerelease.py')
prerelease = importlib.util.module_from_spec(spec)
spec.loader.exec_module(prerelease)

REPO = 'ar4ft/nainzed'
COMMIT = 'a' * 40
RUN_URL = f'https://github.com/{REPO}/actions/runs/123'


def response(body=None, returncode=0, stderr=''):
    return subprocess.CompletedProcess([], returncode, json.dumps(body) if body else '', stderr)


class PrereleasePublishing(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.packages = Path(self.temporary.name)
        for name in prerelease.ASSETS:
            (self.packages / name).write_bytes(name.encode())

    def publish(self):
        return prerelease.publish(self.packages, REPO, COMMIT, 31, RUN_URL)

    def existing(self, **changes):
        release = {'draft': False, 'prerelease': True, 'target_commitish': COMMIT,
                   'assets': [{'name': name} for name in (*prerelease.ASSETS, 'SHA256SUMS.txt')]}
        return dict(release, **changes)

    def test_missing_architecture_or_empty_installer_prevents_publication(self):
        installer = self.packages / 'Zed-No-AI-x86_64.dmg'
        installer.unlink()
        with patch.object(prerelease, 'gh') as gh:
            with self.assertRaisesRegex(ValueError, 'Missing or empty'):
                self.publish()
            gh.assert_not_called()
        installer.write_bytes(b'')
        with patch.object(prerelease, 'gh') as gh:
            with self.assertRaises(ValueError):
                self.publish()
            gh.assert_not_called()

    def test_both_architectures_upload_before_draft_becomes_public(self):
        with patch.object(prerelease, 'gh', side_effect=[
            response(returncode=1, stderr='HTTP 404'), response(), response(), response()
        ]) as gh:
            url = self.publish()
        calls = [call.args for call in gh.call_args_list]
        self.assertEqual([call[:2] for call in calls[1:]],
                         [('release', 'create'), ('release', 'upload'), ('release', 'edit')])
        self.assertIn('--draft', calls[1])
        self.assertIn('--prerelease', calls[1])
        self.assertEqual(calls[1][calls[1].index('--target') + 1], COMMIT)
        for name in (*prerelease.ASSETS, 'SHA256SUMS.txt'):
            self.assertIn(str(self.packages / name), calls[2])
        self.assertIn('--draft=false', calls[3])
        self.assertIn('--latest=false', calls[3])
        self.assertIn('--prerelease', calls[3])
        self.assertTrue(url.endswith('dev-31-' + COMMIT[:12]))
        sums = (self.packages / 'SHA256SUMS.txt').read_text().splitlines()
        self.assertEqual(len(sums), 4)
        for line in sums:
            checksum, name = line.split('  ')
            self.assertEqual(checksum, hashlib.sha256((self.packages / name).read_bytes()).hexdigest())

    def test_upload_failure_keeps_release_unpublished(self):
        with patch.object(prerelease, 'gh', side_effect=[
            response(returncode=1, stderr='HTTP 404'), response(), RuntimeError('Upload failed')
        ]) as gh:
            with self.assertRaises(RuntimeError):
                self.publish()
        self.assertFalse(any(call.args[:2] == ('release', 'edit') for call in gh.call_args_list))

    def test_downloaded_artifacts_can_keep_architecture_directories(self):
        expected = []
        for name in prerelease.ASSETS:
            architecture = 'arm64' if 'arm64' in name else 'x86_64'
            directory = self.packages / f'no-ai-{architecture}'
            directory.mkdir(exist_ok=True)
            path = directory / name
            (self.packages / name).rename(path)
            expected.append(str(path))
        with patch.object(prerelease, 'gh', side_effect=[
            response(returncode=1, stderr='HTTP 404'), response(), response(), response()
        ]) as gh:
            self.publish()
        upload = gh.call_args_list[2].args
        for path in expected:
            self.assertIn(path, upload)
        sums = (self.packages / 'SHA256SUMS.txt').read_text()
        for name in prerelease.ASSETS:
            self.assertIn('  ' + name + '\n', sums)

    def test_ambiguous_installer_names_prevent_publication(self):
        duplicate = self.packages / 'duplicate'
        duplicate.mkdir()
        (duplicate / prerelease.ASSETS[0]).write_bytes(b'wrong installer')
        with patch.object(prerelease, 'gh') as gh:
            with self.assertRaisesRegex(ValueError, 'Ambiguous installer'):
                self.publish()
            gh.assert_not_called()

    def test_published_build_is_preserved_on_retry(self):
        with patch.object(prerelease, 'gh', return_value=response(self.existing())) as gh:
            self.publish()
            gh.assert_called_once()
        self.assertFalse((self.packages / 'SHA256SUMS.txt').exists())

    def test_stable_unrelated_and_incomplete_published_releases_cannot_be_overwritten(self):
        for changes in [{'prerelease': False}, {'target_commitish': 'b' * 40}, {'assets': []}]:
            with self.subTest(changes=changes), patch.object(
                prerelease, 'gh', return_value=response(self.existing(**changes))
            ) as gh:
                with self.assertRaises(ValueError):
                    self.publish()
                gh.assert_called_once()

    def test_permission_error_is_not_treated_as_a_missing_release(self):
        with patch.object(prerelease, 'gh', return_value=response(returncode=1, stderr='HTTP 403')) as gh:
            with self.assertRaisesRegex(RuntimeError, '403'):
                self.publish()
            gh.assert_called_once()

    def test_retry_completes_an_existing_draft(self):
        with patch.object(prerelease, 'gh', side_effect=[
            response(self.existing(draft=True)), response(), response()
        ]) as gh:
            self.publish()
        self.assertEqual([call.args[:2] for call in gh.call_args_list[1:]],
                         [('release', 'upload'), ('release', 'edit')])

    def test_completed_build_requires_main_and_both_successful_mac_jobs(self):
        run = {'status': 'completed', 'conclusion': 'success', 'head_branch': 'main',
               'event': 'push', 'path': '.github/workflows/no-ai-mac.yml'}
        jobs = {'jobs': [{'name': f'package ({runner})', 'status': 'completed', 'conclusion': 'success'}
                         for runner in ['macos-15', 'macos-15-intel']]}
        with patch.object(prerelease, 'gh', side_effect=[response(run), response(jobs)]):
            self.assertEqual(prerelease.completed_build(REPO, '123'), run)
        for changes in [{'head_branch': 'upstream-update/v1.0.0'}, {'conclusion': 'failure'},
                        {'status': 'in_progress'}, {'event': 'pull_request'},
                        {'path': '.github/workflows/no-ai-upstream.yml'}]:
            with self.subTest(changes=changes), patch.object(
                prerelease, 'gh', return_value=response(dict(run, **changes))
            ) as gh:
                with self.assertRaises(ValueError):
                    prerelease.completed_build(REPO, '123')
                gh.assert_called_once()
        jobs['jobs'].pop()
        with patch.object(prerelease, 'gh', side_effect=[response(run), response(jobs)]):
            with self.assertRaisesRegex(ValueError, 'Both Mac'):
                prerelease.completed_build(REPO, '123')


if __name__ == '__main__':
    unittest.main()
