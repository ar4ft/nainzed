# Upstream maintenance

**Review upstream Zed stable updates** runs every Monday at 09:17 UTC on the repository's default branch, or through **Actions → Review upstream Zed stable updates → Run workflow**. It checks Zed's latest published stable release, skips releases already incorporated into the fork, and prepares at most one open upstream draft PR against `main`. It never automatically merges a PR, signs a binary, or publishes a release.

Updates use merge commits to retain upstream ancestry and the fork's changes. If the initial import lacks upstream ancestry, the update branch first records the known `UPSTREAM_REVISION` as a parent using an `ours` merge; its file tree is verified to remain identical. This bootstrap does not change the main branch automatically. A clean merge records the incoming upstream SHA in `UPSTREAM_REVISION` and writes `UPSTREAM_REVIEW.md`. That report links the upstream comparison, identifies changed source/settings/workflow paths, and flags additions involving AI, telemetry, network calls and background tasks for human review. Those keyword matches are review aids, not a complete network or security analysis.

If a merge conflicts, the workflow aborts it and opens a draft PR containing only a conflict report. The upstream reference and application source stay unchanged. No Mac update validation is requested until the source merge is resolved. An existing draft PR is left alone so subsequent weekly runs do not overwrite a maintainer's fixes. Existing update branches without open PRs are preserved as well; create the draft PR manually after correcting any repository permission problem.

## Checks and review

`Build Zed No AI for Mac` accepts normal PR events and reusable workflow calls. Bot-created PRs using `GITHUB_TOKEN` do not generate new PR workflow runs, so the weekly workflow calls the reusable checks itself with the exact clean-merge commit. The maintenance run is linked in the PR and reports its final result on the update commit as `upstream/mac-validation`, because reusable jobs alone attach to the caller's commit. Manual fixes pushed to an open PR trigger normal PR checks, including GitHub's proposed merge commit. Source/privacy guard tests run on Linux first, followed by Apple Silicon and Intel validation. Normal PRs do not build installers. Clean weekly update calls and branch builds continue to development packages with startup proxy checks and benchmark artifacts. Cache restore/save operations are best effort and bounded to eight minutes each. No Apple signing credentials are passed to those jobs.

The checks include:

- Protected source fingerprints for the inert telemetry APIs, empty model registry, enforced AI settings, startup hooks, app menus and settings pages.
- Production dependency rejection for known AI providers/engines and agent families, crash capture, hang telemetry, and removed collaboration/audio implementations, in both the editor and remote helper.
- Tests that AI and telemetry cannot be re-enabled and telemetry property expressions are never evaluated.
- Application and remote-helper compilation, notebook preservation/open/save/recovery regressions, Python environment selection, kernel completion/output protocol, and updater selection/rollback tests.
- Actual-application startup proxy checks and Mac startup/memory/large-file/notebook reports for package builds; see [NETWORK_PRIVACY.md](NETWORK_PRIVACY.md) and [PERFORMANCE.md](PERFORMANCE.md).

The protected-source audit intentionally rejects **any** change to a protected implementation, including legitimate upstream edits. Read the relevant diff, confirm the fork's restrictions remain enforced, update the implementation and regression tests if needed, and only then refresh the corresponding SHA-256 values in `assets/no-ai-source-guards.json`. Do not blindly regenerate the manifest to make a failed check pass. New code outside the protected files still needs human review; neither fingerprints nor the dependency denylist proves that every possible future AI or telemetry implementation is absent.

Before merging, complete the checklist in the draft PR, inspect the actual fork diff (including build scripts and all workflow changes), and review both Mac jobs. Keep the signed release workflow manual. Resolve the draft, mark it ready, and merge it manually. Use **Create a merge commit** for upstream update PRs: squashing or rebasing discards the upstream parent history and causes unnecessary conflicts in later updates. After merging to `main`, fast-forward `no-ai` if you want both branches synchronized. Signed releases remain a separate manual action.

## Resolve a conflict PR

Fetch the draft branch and merge the incoming SHA listed in its report:

```sh
git fetch origin
git checkout upstream-update/vX.Y.Z
git merge --no-ff --no-commit INCOMING_UPSTREAM_SHA
# Resolve conflicts while preserving the fork's AI/telemetry restrictions.
git add PATHS_YOU_RESOLVED
printf '%s\n' INCOMING_UPSTREAM_SHA > UPSTREAM_REVISION
# Update UPSTREAM_REVIEW.md to describe the resolved merge and required review.
git add UPSTREAM_REVISION UPSTREAM_REVIEW.md
git commit
git push origin HEAD
```

The PR then gets normal checks. Keep it in draft until all review items and failures are addressed. Do not change `UPSTREAM_REVISION` while the merge is unresolved.

## Repository setup

The schedule becomes active when this workflow exists on `main`. Under **Settings → Actions → General → Workflow permissions**, enable **Allow GitHub Actions to create and approve pull requests** if bot PR creation is disabled. This workflow does not approve PRs; GitHub groups PR creation and approval under the same setting. The prepare job requests contents/PR write permission, while build jobs have only contents read permission. No personal token or Apple credentials are required.

For a read-only update plan on a clean checkout, run `python3 script/upstream-maintenance.py` with an authenticated GitHub CLI. It fetches the stable tag but does not modify source files, create branches, push, or open a PR. `--apply` creates the branch and draft PR. The local maintenance tests use temporary Git repositories and require neither GitHub credentials nor network access:

```sh
python3 -m unittest discover -s script/tests -p test_upstream_maintenance.py
script/audit-no-ai-source
```
