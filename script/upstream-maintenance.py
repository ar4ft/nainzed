#!/usr/bin/env python3
"""Prepare a reviewed upstream stable merge. No automatic merge or release."""
import argparse
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

FORK = 'ar4ft/zed-no-ai'
UPSTREAM = 'zed-industries/zed'


def run(*args, cwd=None, check=True):
    return subprocess.run(args, cwd=cwd, text=True, capture_output=True, check=check)


def git(root, *args, check=True):
    return run('git', *args, cwd=root, check=check)


def review_report(root, previous, target, tag, conflicts):
    changes = git(root, 'diff', '--name-only', previous, target).stdout.splitlines()
    relevant = [p for p in changes if p.startswith(('crates/', 'script/', '.github/', 'assets/settings/')) or p in ('Cargo.toml', 'Cargo.lock')]
    # Candidate additions need human review; matching a word does not prove telemetry or AI.
    diff = git(root, 'diff', '--unified=0', previous, target, '--', 'crates', 'script', '.github').stdout
    file = ''
    flagged = set()
    added_urls = set()
    for line in diff.splitlines():
        if line.startswith('+++ b/'):
            file = line[6:]
        elif line.startswith('+') and not line.startswith('+++'):
            if re.search(r'(?i)telemetry|diagnostic|crash|hang.report|copilot|anthropic|openai|language.model|edit.prediction|agent|https?://|http.client|spawn|background', line):
                flagged.add(file)
            if re.search(r'https?://|http.client', line):
                added_urls.add(file)
    lines = [f'# Upstream review: {tag}', '', f'Previous upstream reference: `{previous}`',
             f'Incoming stable release: [`{tag}`](https://github.com/{UPSTREAM}/releases/tag/{tag}) (`{target}`)',
             f'[Upstream comparison](https://github.com/{UPSTREAM}/compare/{previous}...{target})', '',
             'This report identifies changes for review; automated checks do not establish that every new AI or telemetry path is absent.', '',
             '## Merge state', '']
    if conflicts:
        lines += ['The merge was aborted. This draft PR contains only this report; no upstream source has been integrated.', '', 'Conflicting paths:', ''] + [f'- `{p}`' for p in conflicts]
    else:
        lines += ['The branch contains the upstream merge. Review the diff and all checks before marking this draft ready.']
    for heading, paths in [('AI, telemetry, startup and background-task candidates', sorted(flagged)),
                           ('Network-related additions', sorted(added_urls)),
                           ('Changed source, settings, build and workflow paths', relevant)]:
        lines += ['', '## '+heading, ''] + ([f'- `{p}`' for p in paths] or ['None detected.'])
    lines += ['', '## Required human review', '',
              '- [ ] Review new startup hooks, background tasks, network endpoints, menus and command registrations.',
              '- [ ] Confirm AI providers, agents and telemetry collectors remain disabled; review guard changes before refreshing hashes.',
              '- [ ] Review Apple Silicon and Intel dependency audits, regression tests and development packages.',
              '- [ ] Check Python kernel discovery and notebook open/save/data preservation.',
              '- [ ] Confirm signing remains manual and no upstream workflow can replace the fork with upstream binaries.',
              '- [ ] Merge this PR manually only after resolving failures. No signed release is triggered by merging.', '']
    return '\n'.join(lines)


def prepare(root, tag, upstream_url, apply=False):
    if not re.fullmatch(r'v[0-9]+\.[0-9]+\.[0-9]+', tag):
        raise ValueError('Expected a stable vMAJOR.MINOR.PATCH tag')
    if git(root, 'status', '--porcelain').stdout.strip():
        raise ValueError('Working tree must be clean')
    previous = (root / 'UPSTREAM_REVISION').read_text().strip()
    if not re.fullmatch(r'[0-9a-f]{40}', previous):
        raise ValueError('UPSTREAM_REVISION must contain a full commit SHA')
    git(root, 'fetch', '--no-tags', upstream_url, f'refs/tags/{tag}')
    target = git(root, 'rev-parse', 'FETCH_HEAD^{commit}').stdout.strip()
    if git(root, 'merge-base', '--is-ancestor', target, 'HEAD', check=False).returncode == 0:
        print(f'{tag} is already incorporated; no update PR is needed.')
        return None
    git(root, 'cat-file', '-e', previous+'^{commit}')
    if not apply:
        print(f'Update available: {tag} ({target}); recorded upstream {previous}.')
        return {'target': target, 'tag': tag}
    branch = 'upstream-update/'+tag
    # Refuse to overwrite an existing branch, including a maintainer's conflict fixes.
    git(root, 'checkout', '-b', branch)
    merge = git(root, 'merge', '--no-commit', '--no-ff', target, check=False)
    conflicts = git(root, 'diff', '--name-only', '--diff-filter=U').stdout.splitlines()
    if merge.returncode:
        git(root, 'merge', '--abort', check=False)
        if not conflicts:
            raise RuntimeError('Upstream merge failed without conflicts: '+merge.stderr)
    else:
        (root / 'UPSTREAM_REVISION').write_text(target+'\n')
    report = review_report(root, previous, target, tag, conflicts)
    (root / 'UPSTREAM_REVIEW.md').write_text(report)
    git(root, 'add', 'UPSTREAM_REVIEW.md')
    if not conflicts:
        git(root, 'add', 'UPSTREAM_REVISION')
    git(root, 'commit', '-m', f'{"Report conflicts for" if conflicts else "Merge"} upstream Zed {tag}')
    return {'branch': branch, 'target': target, 'tag': tag, 'conflicts': conflicts,
            'head': git(root, 'rev-parse', 'HEAD').stdout.strip(), 'report': report}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply', action='store_true', help='Push a branch and open a draft PR')
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    if args.apply:
        prs = json.loads(run('gh', 'pr', 'list', '--repo', FORK, '--base', 'main', '--state', 'open', '--json', 'headRefName,url').stdout)
        pending = [p for p in prs if p['headRefName'].startswith('upstream-update/')]
        if pending:
            print('Finish the existing upstream PR first: '+pending[0]['url'])
            return
        git(root, 'config', 'user.name', 'github-actions[bot]')
        git(root, 'config', 'user.email', '41898282+github-actions[bot]@users.noreply.github.com')
    release = json.loads(run('gh', 'api', f'repos/{UPSTREAM}/releases/latest').stdout)
    if release['draft'] or release['prerelease']:
        raise ValueError('Expected the latest published stable release')
    branch = 'upstream-update/'+release['tag_name']
    if args.apply and git(root, 'ls-remote', '--heads', 'origin', 'refs/heads/'+branch).stdout.strip():
        print('The existing update branch will not be overwritten: '+branch+'. Create a draft PR or resolve its previous review before retrying.')
        return
    result = prepare(root, release['tag_name'], f'https://github.com/{UPSTREAM}.git', args.apply)
    if not args.apply or not result or 'branch' not in result:
        return
    git(root, 'push', 'origin', result['branch'])
    with tempfile.TemporaryDirectory(prefix='upstream-pr-') as temp:
        body = Path(temp)/'body.md'
        validation = ''
        if os.environ.get('GITHUB_RUN_ID'):
            validation = '\n\n[Maintenance and Mac validation run](https://github.com/'+FORK+'/actions/runs/'+os.environ['GITHUB_RUN_ID']+').'
        body.write_text(result['report']+validation)
        pr = run('gh', 'pr', 'create', '--repo', FORK, '--base', 'main', '--head', result['branch'],
                 '--draft', '--title', f'Update from upstream Zed {result["tag"]}', '--body-file', str(body), check=False)
    if pr.returncode:
        raise RuntimeError(pr.stderr+'\nThe update branch was preserved. If GitHub blocks bot PR creation, enable Settings → Actions → General → Allow GitHub Actions to create and approve pull requests, then create a draft PR from '+result['branch']+'.')
    print(pr.stdout.strip())
    if not result['conflicts'] and os.environ.get('GITHUB_OUTPUT'):
        with open(os.environ['GITHUB_OUTPUT'], 'a') as output:
            output.write('check_ref='+result['head']+'\n')


if __name__ == '__main__':
    main()
