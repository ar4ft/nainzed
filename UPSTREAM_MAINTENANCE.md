# Upstream maintenance

**Review upstream Zed stable updates** runs every Monday at 09:17 UTC on the repository's default branch, or through **Actions → Review upstream Zed stable updates → Run workflow**. It checks Zed's latest published stable release, skips releases already incorporated into the fork, and prepares at most one open upstream draft PR against `main`. It never automatically approves an update, validates candidate code, merges a PR, signs a binary, or publishes a release. **Every upstream update requires explicit human approval after review, even when the merge is clean and all automated checks would pass.**

Updates use merge commits to retain upstream ancestry and the fork's Git changes. That alone does not preserve application behavior: conflict-free additions can introduce workflows, defaults, startup hooks or new features. If the initial import lacks upstream ancestry, the update branch first records the known `UPSTREAM_REVISION` as a parent using an `ours` merge; its file tree is verified to remain identical. This bootstrap does not change the main branch automatically. A clean source merge records the incoming upstream SHA in `UPSTREAM_REVISION` and writes `UPSTREAM_REVIEW.md`.

The report lists every incoming added, modified or deleted path, assigns a conservative potential-impact category, and separately lists the actual changes to Nain against the fork baseline. It covers defaults and keymaps, startup and menus, editing/LSP/search, notebooks, dependencies, packaging, networking, updates and automation. Unknown paths also require review. AI/telemetry/network keyword matches are additional review aids; absence of a match never means behavior is unchanged. The report identifies potential effects, not a complete semantic analysis. Reviewers must inspect each relevant diff and document whether a behavior change is accepted, removed or adapted.

Incoming `.github/` changes are held out of clean candidates: added automation is removed, and modified or deleted workflows, local actions and repository configuration are restored from the fork baseline. This prevents a clean merge from silently activating upstream CI or publication workflows. The report records the withheld changes. Port wanted automation deliberately after review; the candidate retains upstream ancestry even though its GitHub automation remains the fork's version.

If a merge conflicts, the workflow aborts it and opens a draft PR containing only a conflict report. The upstream reference and application source stay unchanged. No Mac update validation is requested until the source merge is resolved. An existing draft PR is left alone so subsequent weekly runs do not overwrite a maintainer's fixes. Existing update branches without open PRs are preserved as well; create the draft PR manually after correcting any repository permission problem.

## Checks and review

The preparation workflow records `upstream/manual-review` as pending. Bot-created PRs using `GITHUB_TOKEN` do not generate PR workflow runs; preparation no longer calls candidate validation automatically.

After reading the report, reviewing the diff and documenting behavior decisions, open [Approve upstream candidate](https://github.com/ar4ft/nainzed/actions/workflows/no-ai-upstream-review.yml). Run it on **main**, supply the upstream PR number and its **full current head SHA**, and explicitly check the review confirmation. The trusted main script verifies the open PR, its source repository, resolved upstream revision, impact report and current SHA. It accepts only a manual workflow run by a human maintainer with write, maintain or admin access; bot and scheduled approvals are rejected. It records the reviewer, exact SHA and workflow run in `upstream/manual-review`, then starts Mac validation. A newer commit receives no approval from an older SHA; rerun the review after inspecting the changes. API failures, missing approval, pending status or an invalid approval record block the gate.

`Build Nain for Mac` has an `upstream-review` job before validation and packaging. For upstream-update PRs and reusable upstream calls, it checks the exact candidate's approval using trusted main/base scripts, without executing candidate code. Normal feature PRs pass this job without requiring an upstream approval. After approval, source/privacy guards run on Linux, followed by Apple Silicon and Intel validation. Normal PRs do not build installers. Approved upstream calls and main builds compile development packages concurrently with validation, followed by startup proxy checks and benchmark artifacts. The manual review run reports the result on the candidate as `upstream/mac-validation`; failed validation still prevents readiness for merging. Validation and package/runtime checks must all pass before a main build publishes. Upstream review builds do not publish. Kache restores prior compiler snapshots; upstream-review and PR jobs never save them. Trusted main jobs trim and save snapshots. Cache restore/save operations are best effort and bounded to eight minutes each; see [PERFORMANCE.md](PERFORMANCE.md). No Apple signing credentials are passed to review builds.

The checks include:

- Protected source fingerprints for the inert telemetry APIs, empty model registry, enforced AI settings, startup hooks, app menus and settings pages.
- Production dependency rejection for known AI providers/engines and agent families, crash capture, hang telemetry, and removed collaboration/audio implementations, in both the editor and remote helper.
- Tests that AI and telemetry cannot be re-enabled and telemetry property expressions are never evaluated.
- Application and remote-helper compilation, notebook preservation/open/save/recovery regressions, Python environment selection, kernel completion/output protocol, and updater selection/rollback tests.
- Actual-application startup proxy checks and Mac startup/memory/large-file/notebook reports for package builds; see [NETWORK_PRIVACY.md](NETWORK_PRIVACY.md) and [PERFORMANCE.md](PERFORMANCE.md).

The protected-source audit intentionally rejects **any** change to a protected implementation, including legitimate upstream edits. Read the relevant diff, confirm the fork's restrictions remain enforced, update the implementation and regression tests if needed, and only then refresh the corresponding SHA-256 values in `assets/no-ai-source-guards.json`. Do not blindly regenerate the manifest to make a failed check pass. New code outside the protected files still needs human review; neither fingerprints nor the dependency denylist proves that every possible future AI or telemetry implementation is absent.

Before merging, complete the checklist in the draft PR, inspect the actual fork diff (including build scripts and all withheld automation), explicitly approve the current candidate, and review both Mac jobs. Keep the signed release workflow manual. Mark the reviewed draft ready to trigger the normal PR checks against GitHub's proposed merge with the current `main`; this also creates the checks required by the ruleset for bot-created PRs. Merge manually after those checks pass. Use **Create a merge commit** for upstream update PRs: squashing or rebasing discards the upstream parent history and causes unnecessary conflicts in later updates. `main` is the maintained branch. After merging, delete the merged `upstream-update/*` review branch. Signed releases remain a separate manual action.

## Resolve a conflict PR

Fetch the draft branch and merge the incoming SHA listed in its report:

```sh
git fetch origin
git checkout upstream-update/vX.Y.Z
git merge --no-ff --no-commit INCOMING_UPSTREAM_SHA
# Restore the fork's GitHub automation, including removing incoming additions.
git restore --source=origin/main --staged --worktree -- .github
# Resolve conflicts while preserving the fork's AI/telemetry restrictions.
git add PATHS_YOU_RESOLVED
printf '%s\n' INCOMING_UPSTREAM_SHA > UPSTREAM_REVISION
# Update UPSTREAM_REVIEW.md to describe the resolved merge and required review.
git add UPSTREAM_REVISION UPSTREAM_REVIEW.md
git commit
git push origin HEAD
```

The PR then gets the manual approval gate. Keep it in draft, regenerate the impact report and hold out incoming automation while resolving conflicts, review the current full SHA, and run **Approve upstream candidate**. Do not change `UPSTREAM_REVISION` while the merge is unresolved.

## Repository setup

The schedule becomes active when this workflow exists on `main`. Under **Settings → Actions → General → Workflow permissions**, enable **Allow GitHub Actions to create and approve pull requests** if bot PR creation is disabled. Preparation does not approve PRs; GitHub groups PR creation and approval under the same setting. Preparation requests contents/PR/status write permission. The separate approval job has status write permission and runs only trusted main scripts; validation has read permissions. No personal token or Apple credentials are required.

Repository rules are needed to prevent bypassing the workflow by directly pushing or merging an unchecked branch. **The workflow alone cannot protect an unprotected `main` branch.** Under [Settings → Rules → Rulesets](https://github.com/ar4ft/nainzed/settings/rules), import [main-review-ruleset.json](.github/main-review-ruleset.json) and activate it. It requires PRs and up-to-date passing `upstream-review`, source-guard and both Mac validation checks; blocks force pushes/deletion; and grants no bypass actors. The upstream gate supplies the explicit human approval, so a second formal GitHub review is not required by this ruleset. Keep the required-check names unchanged. Use a merge commit for upstream PRs. GitHub administration access is needed to activate rules; committing the JSON does not activate them.

For a read-only update plan on a clean checkout, run `python3 script/upstream-maintenance.py` with an authenticated GitHub CLI. It fetches the stable tag but does not modify source files, create branches, push, or open a PR. `--apply` creates the branch and draft PR. The local maintenance tests use temporary Git repositories and require neither GitHub credentials nor network access:

```sh
python3 -m unittest discover -s script/tests -p 'test_upstream*.py'
script/audit-no-ai-source
```
