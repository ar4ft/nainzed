import importlib.util
from pathlib import Path
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('upstream_review', Path(__file__).resolve().parents[1]/'upstream-review.py')
review = importlib.util.module_from_spec(spec)
spec.loader.exec_module(review)
SHA = 'a'*40


class ReviewTests(unittest.TestCase):
    def fixture(self):
        self.pr = {'state': 'open', 'base': {'ref': 'main', 'repo': {'full_name': review.FORK}},
                   'head': {'ref': 'upstream-update/v1.0.1', 'sha': SHA, 'repo': {'full_name': review.FORK}}}
        self.run = {'event': 'workflow_dispatch', 'head_branch': 'main', 'path': review.WORKFLOW,
                    'actor': {'login': 'maintainer', 'type': 'User'}}
        self.files = [{'filename': 'UPSTREAM_REVISION'}, {'filename': 'UPSTREAM_REVIEW.md'}]
        self.permission = 'write'
        self.statuses = [{'context': review.CONTEXT, 'state': 'success',
                          'creator': {'login': 'github-actions[bot]'},
                          'description': f'Human approval: maintainer; commit {SHA}',
                          'target_url': f'https://github.com/{review.FORK}/actions/runs/123'}]
        self.posts = []

    def api(self, path, *args):
        if args:
            self.posts.append((path, args))
            return {}
        if '/statuses?' in path:
            return self.statuses if f'/commits/{SHA}/' in path else []
        if '/files?' in path:
            return self.files
        if '/pulls/' in path:
            return self.pr
        if '/actions/runs/' in path:
            return self.run
        if '/permission' in path:
            return {'permission': self.permission}
        raise AssertionError(path)

    def setUp(self):
        self.fixture()
        patcher = patch.object(review, 'api', side_effect=self.api)
        patcher.start()
        self.addCleanup(patcher.stop)

    def test_human_attestation_records_the_exact_current_candidate(self):
        review.approve('7', SHA, '123', True)
        self.assertEqual(len(self.posts), 1)
        self.assertIn('/statuses/'+SHA, self.posts[0][0])
        self.assertIn(f'description=Human approval: maintainer; commit {SHA}', self.posts[0][1])
        review.check(SHA)

    def test_confirmation_missing_and_new_commit_do_not_grant_approval(self):
        with self.assertRaises(ValueError):
            review.approve('7', SHA, '123', False)
        self.pr['head']['sha'] = 'b'*40
        with self.assertRaises(ValueError):
            review.approve('7', SHA, '123', True)
        self.assertEqual(self.posts, [])
        with self.assertRaises(ValueError):
            review.check('b'*40)

    def test_bots_nonmaintainers_and_nonmanual_runs_cannot_approve(self):
        for key, value in [('actor', {'login': 'bot', 'type': 'Bot'}),
                           ('event', 'schedule'), ('head_branch', 'upstream-update/v1.0.1'),
                           ('path', '.github/workflows/different.yml')]:
            self.fixture()
            self.run[key] = value
            with self.assertRaises(ValueError):
                review.approve('7', SHA, '123', True)
            self.assertEqual(self.posts, [])
        self.fixture()
        self.permission = 'read'
        with self.assertRaises(ValueError):
            review.approve('7', SHA, '123', True)
        self.assertEqual(self.posts, [])

    def test_conflict_only_external_or_closed_pr_cannot_be_approved(self):
        self.files = [{'filename': 'UPSTREAM_REVIEW.md'}]
        with self.assertRaises(ValueError):
            review.approve('7', SHA, '123', True)
        self.fixture()
        self.pr['head']['repo']['full_name'] = 'someone/other'
        with self.assertRaises(ValueError):
            review.approve('7', SHA, '123', True)
        self.fixture()
        self.pr['state'] = 'closed'
        with self.assertRaises(ValueError):
            review.approve('7', SHA, '123', True)
        self.assertEqual(self.posts, [])

    def test_empty_pending_and_forged_statuses_fail_closed(self):
        for state in [None, 'pending', 'failure']:
            self.fixture()
            self.statuses = [] if state is None else [dict(self.statuses[0], state=state)]
            with self.assertRaises(ValueError):
                review.check(SHA)
        for field, value in [('creator', {'login': 'someone'}),
                             ('target_url', 'https://example.com/approval'),
                             ('description', 'Human approval: maintainer; commit '+'b'*40)]:
            self.fixture()
            self.statuses[0][field] = value
            with self.assertRaises(ValueError):
                review.check(SHA)

    def test_newer_pending_status_overrides_an_older_success(self):
        self.statuses.insert(0, dict(self.statuses[0], state='pending'))
        with self.assertRaises(ValueError):
            review.check(SHA)

    def test_push_during_review_is_rejected_before_status_write(self):
        original = self.api
        reads = 0
        def racing_api(path, *args):
            nonlocal reads
            if path.endswith('/pulls/7'):
                reads += 1
                if reads == 2:
                    self.pr['head']['sha'] = 'b'*40
            return original(path, *args)
        with patch.object(review, 'api', side_effect=racing_api), self.assertRaises(ValueError):
            review.approve('7', SHA, '123', True)
        self.assertEqual(self.posts, [])


if __name__ == '__main__':
    unittest.main()
