#!/usr/bin/env python3
"""Prepare an unapproved upstream candidate. Every update needs manual review."""
import argparse
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

FORK = 'ar4ft/nainzed'
UPSTREAM = 'zed-industries/zed'


def run(*args, cwd=None, check=True):
    return subprocess.run(args, cwd=cwd, text=True, capture_output=True, check=check)


def git(root, *args, check=True):
    return run('git', *args, cwd=root, check=check)


def changed_paths(root, previous, target=None):
    args = ['diff', '--name-status', '--no-renames', '-z', previous]
    if target is not None:
        args.append(target)
    fields = git(root, *args).stdout.split('\0')
    return list(zip(fields[::2], fields[1::2]))


def impact(path):
    """Conservative review categories, never a claim of unchanged behavior."""
    if path == '.github' or path.startswith('.github/'):
        return 'GitHub automation, permissions, checks or publication'
    if path.startswith(('script/', '.cargo/')) or path in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml') or path.endswith(('Cargo.toml', 'build.rs')):
        return 'Dependencies, compilation, packaging or release behavior'
    if path.startswith(('assets/settings/', 'assets/keymaps/', 'crates/settings')):
        return 'Defaults, settings, shortcuts or feature availability'
    if path.startswith(('crates/zed/', 'crates/workspace/', 'crates/title_bar/', 'crates/onboarding/', 'crates/platform_title_bar/')):
        return 'Startup, UI, menus, window behavior or action registration'
    if path.startswith(('crates/telemetry/', 'crates/client/', 'crates/auto_update/', 'crates/http_client/', 'crates/remote')):
        return 'Privacy, networking, background work or updates'
    if re.search(r'agent|assistant|language_model|copilot|prediction|collab|livekit|webrtc', path):
        return 'AI or collaboration surface; verify it stays unavailable'
    if path.startswith(('crates/notebook', 'crates/repl/', 'crates/jupyter')):
        return 'Notebook data, execution, kernels or completion'
    if path.startswith(('crates/editor/', 'crates/project/', 'crates/language', 'crates/code_search')):
        return 'Editing, files, LSP, completion or search behavior'
    if path.startswith('assets/'):
        return 'Application assets, themes or branding'
    return 'Other upstream change; effect on Nain needs manual review'


def preserve_fork_automation(root, fork_base):
    """Do not activate upstream workflows/actions merely because Git merged them."""
    withheld = [(status, path) for status, path in changed_paths(root, fork_base)
                if path == '.github' or path.startswith('.github/')]
    for _, path in withheld:
        if git(root, 'cat-file', '-e', fork_base+':'+path, check=False).returncode == 0:
            git(root, 'restore', '--source='+fork_base, '--staged', '--worktree', '--', path)
        else:
            git(root, 'rm', '-f', '--', path)
    return withheld


def path_table(changes):
    names = {'A': 'Added', 'M': 'Modified', 'D': 'Deleted', 'T': 'Type changed'}
    return ['| Change | Path | Potential effect requiring review |',
            '| --- | --- | --- |'] + [
        f'| {names.get(status, status)} | `{path.replace("`", "&#96;").replace("|", "&#124;").replace(chr(10), "&#10;")}` | {impact(path)} |'
        for status, path in changes]


def review_report(root, previous, target, tag, conflicts, fork_base, withheld):
    incoming = changed_paths(root, previous, target)
    effective = [(status, path) for status, path in changed_paths(root, fork_base)
                 if path not in ('UPSTREAM_REVISION', 'UPSTREAM_REVIEW.md')]
    # Candidate additions need human review; matching a word does not prove telemetry or AI.
    diff = git(root, 'diff', '--unified=0', previous, target).stdout
    file = ''
    flagged = set()
    added_urls = set()
    for line in diff.splitlines():
        if line.startswith('+++ b/'):
            file = line[6:]
        elif line.startswith('+') and not line.startswith('+++'):
            if re.search(r'(?i)telemetry|diagnostic|crash|hang.report|copilot|anthropic|openai|language.model|edit.prediction|agent|livekit|webrtc|collab|rodio|cpal|https?://|http.client|reqwest|TcpStream|UdpSocket|connect\(|spawn|background', line):
                flagged.add(file)
            if re.search(r'https?://|http.client|reqwest|TcpStream|UdpSocket|connect\(', line):
                added_urls.add(file)
    lines = [f'# Upstream review: {tag}', '', f'Previous upstream reference: `{previous}`',
             f'Incoming stable release: [`{tag}`](https://github.com/{UPSTREAM}/releases/tag/{tag}) (`{target}`)',
             f'[Upstream comparison](https://github.com/{UPSTREAM}/compare/{previous}...{target})', '',
             '**Manual approval required for this exact candidate, including a clean merge.**', '',
             'A clean merge only means Git found no textual conflicts. It can introduce new workflows, defaults, startup hooks or other behavior. '
             'Every path below needs review; passing checks or an empty keyword scan never grants approval.', '',
             f'Fork baseline for the actual changes: `{fork_base}`.', '',
             f'**Behavior preservation: unverified.** {len(effective)} actual changed paths require review; '
             f'{len(withheld)} incoming automation paths were withheld. A maintainer must explicitly accept or remove the changes before approving.', '',
             '## Merge state', '']
    if conflicts:
        lines += ['The merge was aborted. This draft PR contains only this report; no upstream source has been integrated.', '', 'Conflicting paths:', ''] + [f'- `{p}`' for p in conflicts]
    else:
        lines += ['The branch contains an **unapproved** source merge. Incoming `.github/` changes are withheld so new or changed upstream automation cannot become active automatically. '
                  'The PR stays in draft; candidate validation waits for the manual review workflow.']
    lines += ['', '## Incoming automation held out of the candidate', '']
    lines += path_table(withheld) if withheld else ['No automation was imported into the candidate. Review the incoming automation paths below even if the merge conflicted.']
    lines += ['', '## Actual changes to Nain requiring approval', '']
    lines += path_table(effective) if effective else ['No application changes were integrated; approval is still required once a candidate is resolved.']
    lines += ['', '## All incoming upstream changes and potential behavior impact', '']
    lines += path_table(incoming) if incoming else ['No changed paths in the incoming comparison; manual approval is still required.']
    for heading, paths in [('AI, telemetry, startup and background-task candidates', sorted(flagged)),
                           ('Network-related additions', sorted(added_urls))]:
        lines += ['', '## '+heading, ''] + ([f'- `{p}`' for p in paths] or ['No keyword matches. Behavior impact remains unverified; manual review is mandatory.'])
    lines += ['', '## Required human review', '',
              '- [ ] Review new startup hooks, background tasks, network endpoints, menus and command registrations.',
              '- [ ] Confirm AI providers, agents and telemetry collectors remain disabled; review guard changes before refreshing hashes.',
              '- [ ] Review Apple Silicon and Intel dependency audits, regression tests and development packages.',
              '- [ ] Check Python kernel discovery and notebook open/save/data preservation.',
              '- [ ] Review proxy-recorded startup traffic and benchmark artifacts; investigate unknown hosts or missing graphical checks.',
              '- [ ] Confirm editor and remote-helper dependency trees still exclude call/audio, LiveKit and WebRTC implementations.',
              '- [ ] Confirm signing remains manual and no upstream workflow can replace the fork with upstream binaries.',
              '- [ ] Explicitly accept or remove every behavior change and document the decision in this PR; port any wanted upstream automation separately.',
              '- [ ] After reviewing, manually run **Approve upstream candidate** on `main` with this PR number and its current full head SHA. Confirm the review checkbox. Approval expires on any new commit.',
              '- [ ] Review the resulting Mac checks, then mark this draft ready and merge manually using a merge commit. No signed release is triggered by merging.', '']
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
    # Resolve the recorded baseline even in imported or shallow development clones.
    unshallow = ['--unshallow'] if git(root, 'rev-parse', '--is-shallow-repository').stdout.strip() == 'true' else []
    git(root, 'fetch', '--no-tags', '--filter=blob:none', *unshallow, upstream_url, previous)
    if git(root, 'merge-base', '--is-ancestor', target, 'HEAD', check=False).returncode == 0:
        print(f'{tag} is already incorporated; no update PR is needed.')
        return None
    git(root, 'cat-file', '-e', previous+'^{commit}')
    if not apply:
        print(f'Update available: {tag} ({target}); recorded upstream {previous}.')
        return {'target': target, 'tag': tag}
    branch = 'upstream-update/'+tag
    fork_base = git(root, 'rev-parse', 'HEAD').stdout.strip()
    # Refuse to overwrite an existing branch, including a maintainer's conflict fixes.
    git(root, 'checkout', '-b', branch)
    # The initial GitHub import may contain Zed's files without its parent history.
    # Connect the recorded baseline using an ours merge; its tree stays identical.
    if git(root, 'merge-base', 'HEAD', previous, check=False).returncode:
        before = git(root, 'rev-parse', 'HEAD^{tree}').stdout.strip()
        git(root, 'merge', '--allow-unrelated-histories', '--strategy=ours', '--no-ff',
            previous, '-m', 'Record upstream baseline ancestry without changing fork files')
        if git(root, 'rev-parse', 'HEAD^{tree}').stdout.strip() != before:
            raise RuntimeError('Recording baseline ancestry unexpectedly changed the fork tree')
    merge = git(root, 'merge', '--no-commit', '--no-ff', target, check=False)
    conflicts = git(root, 'diff', '--name-only', '--diff-filter=U').stdout.splitlines()
    withheld = []
    if merge.returncode:
        git(root, 'merge', '--abort', check=False)
        if not conflicts:
            raise RuntimeError('Upstream merge failed without conflicts: '+merge.stderr)
    else:
        withheld = preserve_fork_automation(root, fork_base)
        (root / 'UPSTREAM_REVISION').write_text(target+'\n')
    report = review_report(root, previous, target, tag, conflicts, fork_base, withheld)
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
        report = result['report']
        if len(report.encode()) > 55000:
            report = (report[:6000]+'\n\n[Full path-by-path impact report](https://github.com/'+FORK+'/blob/'+result['head']+'/UPSTREAM_REVIEW.md).\n\n'
                      +report[report.index('## Required human review'):])
        body.write_text(report+validation)
        pr = run('gh', 'pr', 'create', '--repo', FORK, '--base', 'main', '--head', result['branch'],
                 '--draft', '--title', f'Update from upstream Zed {result["tag"]}', '--body-file', str(body), check=False)
    if pr.returncode:
        raise RuntimeError(pr.stderr+'\nThe update branch was preserved. If GitHub blocks bot PR creation, enable Settings → Actions → General → Allow GitHub Actions to create and approve pull requests, then create a draft PR from '+result['branch']+'.')
    print(pr.stdout.strip())
    run('gh', 'api', '--method', 'POST', f'repos/{FORK}/statuses/{result["head"]}',
        '-f', 'state=pending', '-f', 'context=upstream/manual-review',
        '-f', 'description=Manual review and explicit approval required, even for clean merges',
        '-f', 'target_url='+pr.stdout.strip())
    print('Candidate remains unapproved. No validation, merge or release is authorized by a clean merge.')


if __name__ == '__main__':
    main()
