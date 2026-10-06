#!/usr/bin/env python3
"""Publish a checked development build without signing or enabling updates."""
import argparse
import hashlib
import json
import re
import subprocess
from pathlib import Path


ASSETS = tuple(f'nain-{arch}.{extension}'
               for arch in ('arm64', 'x86_64') for extension in ('dmg', 'zip'))

LEGACY_ASSETS = tuple(name.replace('nain-', 'Zed-No-AI-') for name in ASSETS)


def gh(*args, check=True):
    result = subprocess.run(['gh', *args], text=True, capture_output=True)
    if check and result.returncode:
        raise RuntimeError(result.stderr.strip() or 'GitHub request failed')
    return result


def completed_build(repository, run_id):
    if not re.fullmatch(r'[0-9]+', run_id):
        raise ValueError('Expected a numeric completed build run ID')
    run = json.loads(gh('api', f'repos/{repository}/actions/runs/{run_id}').stdout)
    if (run['status'] != 'completed' or run['conclusion'] != 'success'
            or run['head_branch'] != 'main'
            or run['event'] not in ('push', 'workflow_dispatch')
            or run['path'] != '.github/workflows/no-ai-mac.yml'):
        raise ValueError('Only a successful completed main development build can be published')
    jobs = json.loads(gh('api', f'repos/{repository}/actions/runs/{run_id}/jobs?per_page=100').stdout)['jobs']
    passed = {job['name'] for job in jobs
              if job['status'] == 'completed' and job['conclusion'] == 'success'}
    if not {'package (macos-15)', 'package (macos-15-intel)'}.issubset(passed):
        raise ValueError('Both Mac packaging and runtime-check jobs must have passed')
    return run


def tag_commit(repository, tag):
    result = gh('api', f'repos/{repository}/git/ref/tags/{tag}', check=False)
    if result.returncode:
        if 'HTTP 404' in result.stderr:
            return None
        raise RuntimeError(result.stderr.strip() or 'Cannot inspect release tag')
    obj = json.loads(result.stdout)['object']
    for _ in range(16):
        if obj['type'] == 'commit':
            return obj['sha']
        if obj['type'] != 'tag':
            break
        obj = json.loads(gh('api', f'repos/{repository}/git/tags/{obj["sha"]}').stdout)['object']
    raise ValueError('Release tag does not resolve to a commit')


def publish(directory, repository, commit, run_number, run_url):
    if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('Expected owner/repository')
    if not re.fullmatch(r'[0-9a-f]{40}', commit) or run_number < 1:
        raise ValueError('Expected a full commit SHA and positive build number')
    if not re.fullmatch(r'https://github\.com/' + re.escape(repository)
                        + r'/actions/runs/[0-9]+', run_url):
        raise ValueError('Expected this repository\'s Actions run URL')

    directory = Path(directory)
    assets = ASSETS
    if not any(directory.rglob('nain-*.dmg')) and any(directory.rglob('Zed-No-AI-*.dmg')):
        assets = LEGACY_ASSETS
    installers = {}
    for name in assets:
        # upload-artifact preserves architecture directories.
        paths = list(directory.rglob(name))
        if len(paths) > 1:
            raise ValueError(f'Ambiguous installer: {name}')
        if not paths or not paths[0].is_file() or not paths[0].stat().st_size:
            raise ValueError(f'Missing or empty installer: {name}')
        installers[name] = paths[0]

    tag = f'dev-{run_number}-{commit[:12]}'
    url = f'https://github.com/{repository}/releases/tag/{tag}'
    tagged_commit = tag_commit(repository, tag)
    if tagged_commit is not None and tagged_commit != commit:
        raise ValueError('Release tag points to a different source commit')
    result = gh('api', f'repos/{repository}/releases/tags/{tag}', check=False)
    existing = None
    if result.returncode:
        if 'HTTP 404' not in result.stderr:
            raise RuntimeError(result.stderr.strip() or 'Cannot inspect existing release')
    else:
        existing = json.loads(result.stdout)
        if not existing['prerelease']:
            raise ValueError('Refusing to change an unrelated or stable release')
        # target_commitish can be "main" even when an existing tag pins the
        # correct source. The resolved tag is authoritative for published builds.
        if ((tagged_commit is None and not existing['draft'])
                or (tagged_commit is None and existing['target_commitish'] != commit)):
            raise ValueError('Existing release is not pinned to the requested source commit')
        if not existing['draft']:
            names = {asset['name'] for asset in existing['assets']}
            if not set((*assets, 'SHA256SUMS.txt')).issubset(names):
                raise ValueError('Published prerelease is incomplete; refusing to overwrite it')
            return url

    checksums = []
    for name in assets:
        with installers[name].open('rb') as file:
            digest = hashlib.file_digest(file, 'sha256').hexdigest()
        checksums.append(f'{digest}  {name}\n')
    (directory / 'SHA256SUMS.txt').write_text(''.join(checksums))
    notes = directory / 'release-notes.txt'
    notes.write_text(
        'Development packages for Apple Silicon and Intel Macs. AI and telemetry are disabled.\n\n'
        f'Source: [`{commit}`](https://github.com/{repository}/commit/{commit})\n\n'
        f'[Validated build and runtime reports]({run_url})\n\n'
        'These packages use ad-hoc signatures and are not Apple-notarized. '
        'Automatic updates are disabled; install development versions manually. '
        'Apple signing and notarization remain a separate manual release workflow.\n'
    )
    if existing is None:
        # Explicit older target SHAs can require workflow-write permission, even
        # when the tag exists. Use the verified tag without a target override.
        target = ('--verify-tag',) if tagged_commit is not None else ('--target', commit)
        try:
            gh('release', 'create', tag, '--repo', repository, *target,
               '--draft', '--prerelease', '--latest=false', '--title', f'nain development build {run_number}',
               '--notes-file', str(notes))
        except RuntimeError as error:
            if tagged_commit is None and '403' in str(error):
                raise RuntimeError(f'{error}\nFor historical builds, create tag {tag} at {commit} '
                                   'with maintainer permissions, then retry.') from error
            raise
    gh('release', 'upload', tag, '--repo', repository, '--clobber',
       *(str(installers[name]) for name in assets), str(directory / 'SHA256SUMS.txt'))
    gh('release', 'edit', tag, '--repo', repository, '--draft=false', '--prerelease',
       '--latest=false', '--notes-file', str(notes))
    return url


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--packages', type=Path, required=True)
    parser.add_argument('--repository', required=True)
    parser.add_argument('--commit')
    parser.add_argument('--run-number', type=int)
    parser.add_argument('--run-url')
    parser.add_argument('--completed-run', help='Publish artifacts from a previously successful main build')
    args = parser.parse_args()
    try:
        if args.completed_run:
            run = completed_build(args.repository, args.completed_run)
            args.commit, args.run_number, args.run_url = run['head_sha'], run['run_number'], run['html_url']
        elif not all((args.commit, args.run_number, args.run_url)):
            parser.error('Provide --completed-run or --commit, --run-number and --run-url')
        print(publish(args.packages, args.repository, args.commit, args.run_number, args.run_url))
    except (OSError, ValueError, RuntimeError) as error:
        raise SystemExit(str(error))


if __name__ == '__main__':
    main()
