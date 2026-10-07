#!/usr/bin/env python3
"""Approve one upstream candidate manually, or verify its commit-bound approval."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess

FORK = 'ar4ft/nainzed'
CONTEXT = 'upstream/manual-review'
WORKFLOW = '.github/workflows/no-ai-upstream-review.yml'


def api(path, *args):
    result = subprocess.run(['gh', 'api', path, *args], text=True, capture_output=True, check=True)
    return json.loads(result.stdout)


def valid_sha(sha):
    if not re.fullmatch(r'[0-9a-f]{40}', sha):
        raise ValueError('Use the full current candidate commit SHA')


def review_run(run_id):
    run = api(f'repos/{FORK}/actions/runs/{run_id}')
    actor = run.get('actor', {})
    if (run.get('event') != 'workflow_dispatch' or run.get('head_branch') != 'main'
            or run.get('path') != WORKFLOW or actor.get('type') != 'User'):
        raise ValueError('Approval must come from a human running the review workflow on main')
    permission = api(f'repos/{FORK}/collaborators/{actor["login"]}/permission')['permission']
    if permission not in ('admin', 'maintain', 'write'):
        raise ValueError('A repository maintainer must approve the candidate')
    return actor['login']


def candidate(pr_number, sha):
    valid_sha(sha)
    if not re.fullmatch(r'[1-9][0-9]*', str(pr_number)):
        raise ValueError('Expected an upstream PR number')
    pr = api(f'repos/{FORK}/pulls/{pr_number}')
    if (pr.get('state') != 'open' or pr['base']['ref'] != 'main'
            or pr['base']['repo']['full_name'] != FORK
            or not pr['head'].get('repo') or pr['head']['repo']['full_name'] != FORK
            or not pr['head']['ref'].startswith('upstream-update/')
            or pr['head']['sha'] != sha):
        raise ValueError('The PR must be an open upstream candidate against main at this exact SHA')
    paths = set()
    page = 1
    while True:
        files = api(f'repos/{FORK}/pulls/{pr_number}/files?per_page=100&page={page}')
        paths.update(file['filename'] for file in files)
        if len(files) < 100:
            break
        page += 1
    if 'UPSTREAM_REVISION' not in paths or 'UPSTREAM_REVIEW.md' not in paths:
        raise ValueError('Resolve the source merge and update its impact report before approving')
    # Approval is an explicit human attestation, not an automated semantic proof.
    return pr


def approve(pr_number, sha, run_id, confirmed):
    if not confirmed:
        raise ValueError('Confirm that the impact report and behavior changes were reviewed')
    if not re.fullmatch(r'[1-9][0-9]*', str(run_id)):
        raise ValueError('Expected a GitHub review workflow run')
    candidate(pr_number, sha)
    actor = review_run(run_id)
    # Recheck after API reads: a newer pushed commit invalidates this request.
    candidate(pr_number, sha)
    return api(f'repos/{FORK}/statuses/{sha}', '--method', 'POST',
               '-f', 'state=success', '-f', 'context='+CONTEXT,
               '-f', f'description=Human approval: {actor}; commit {sha}',
               '-f', f'target_url=https://github.com/{FORK}/actions/runs/{run_id}')


def check(sha):
    valid_sha(sha)
    page = 1
    status = None
    while status is None:
        statuses = api(f'repos/{FORK}/commits/{sha}/statuses?per_page=100&page={page}')
        status = next((entry for entry in statuses if entry['context'] == CONTEXT), None)
        if len(statuses) < 100:
            break
        page += 1
    if status is None or status.get('state') != 'success':
        raise ValueError('Manual approval is required for this commit, even when the upstream merge is clean')
    match = re.fullmatch(r'https://github.com/'+re.escape(FORK)+r'/actions/runs/([1-9][0-9]*)', status.get('target_url', ''))
    if not match or status.get('creator', {}).get('login') != 'github-actions[bot]':
        raise ValueError('Approval must be recorded by the trusted manual review workflow')
    actor = review_run(match[1])
    if status.get('description') != f'Human approval: {actor}; commit {sha}':
        raise ValueError('Approval does not identify this exact commit and human reviewer')
    print(f'Manual approval verified for {sha} by {actor}.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--head-sha', required=True)
    parser.add_argument('--approve', action='store_true')
    parser.add_argument('--pr')
    args = parser.parse_args()
    if args.approve:
        if os.environ.get('GITHUB_REF') != 'refs/heads/main':
            raise ValueError('Run the manual approval workflow on main')
        approve(args.pr, args.head_sha, os.environ.get('GITHUB_RUN_ID', ''),
                os.environ.get('REVIEW_CONFIRMED') == 'true')
        with Path(os.environ['GITHUB_OUTPUT']).open('a') as output:
            output.write('check_ref='+args.head_sha+'\n')
    else:
        check(args.head_sha)


if __name__ == '__main__':
    main()
