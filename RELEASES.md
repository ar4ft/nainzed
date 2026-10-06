# Signed releases and automatic updates

The manually dispatched `Signed nain release` workflow builds Apple Silicon and Intel apps, signs them with your Developer ID Application certificate and the hardened runtime, submits the app and DMG to Apple, staples notarization tickets, and publishes both architectures together in a GitHub Release. After initial tag and credential checks, packaging runs concurrently with AI, telemetry, notebook, and update-selection validation. All validation and packaging jobs must pass before publication; a failed architecture or rejected notarization prevents publication. Signed releases cannot fall back to ad-hoc signing.

`nain.app` and `nain-<architecture>.dmg`/`.zip` are the product names. The original fork bundle identity and user-data directories stay unchanged so existing settings, recovery data, and signatures remain compatible.

Automatic updates are enabled only in signed release builds. The app checks `https://api.github.com/repos/ar4ft/nainzed/releases/latest` at startup and hourly, downloads its matching DMG from this repository, verifies the Apple signature against the team ID embedded at build time and the retained bundle identity `io.github.ar4ft.ZedNoAI`, checks Apple's notarization assessment and the installer version, and stages the app beside the installed copy before replacing it. A replacement failure restores the old app; if restoration also fails, the error reports the retained recovery copy. Restart the editor to use an installed update. Users can disable automatic downloads with `"auto_update": false` and still check manually. Requests contain no telemetry IDs; update checks and downloads require network access to GitHub and Apple assessment may contact Apple.

Branch pushes and local builds produce development packages with ad-hoc signatures, without Apple credentials or notarization, and keep automatic updates disabled. Successful `main` pushes and manual development builds on `main` publish a prerelease named `dev-BUILD_NUMBER-COMMIT`, containing both architectures' DMG and ZIP installers and `SHA256SUMS.txt`. Publication waits for all regression, privacy, and performance checks; failed builds, PRs, and upstream review branches do not publish. Prereleases are published after all assets upload and do not replace the latest stable release used by automatic updates. Re-running a published build preserves its existing release; partially uploaded drafts can be completed by retrying the publishing job.

Tag pushes do not sign or publish anything. Existing unsigned builds must be replaced manually with the first signed release. Installation in `/Applications` (or another writable, permanent app folder) is recommended; updating an app running from a mounted DMG will fail. Remote helper downloads retain the existing upstream protocol and are separate from the Mac application release feed.

To publish an older successful development run without rebuilding, manually run **Publish completed development build** on `main` and enter its Actions run ID. It verifies that the run was a successful `main` development build and that source guards, both Mac validation jobs, and both packaging/runtime-check jobs passed, then publishes that run's original installers and source commit. This recovery workflow also uses no Apple credentials.

GitHub can restrict the Actions token from creating tags for older workflow revisions. If the recovery workflow reports this permission error, create its `dev-BUILD_NUMBER-COMMIT` tag at the run's full source commit using your maintainer account, then retry. Existing tags are checked against the original source commit before publication. Normal builds create their own development tags.

## 1. Enroll with Apple

Enroll at [Apple Developer Program](https://developer.apple.com/programs/enroll/). Enrollment normally costs US$99 per year, with regional pricing and eligibility exceptions. Use the same team for signing and every subsequent release; changing the team requires a manually installed release to establish the new trust.

## 2. Create a Developer ID Application certificate on your Mac

1. Open Keychain Access and choose **Certificate Assistant → Request a Certificate From a Certificate Authority**. Save the certificate signing request to disk.
2. Open [Certificates, Identifiers & Profiles](https://developer.apple.com/account/resources/certificates/list), add a **Developer ID Application** certificate, and upload the request. This is the certificate for apps distributed outside the Mac App Store.
3. Download the certificate and open it to add it to the same Mac's keychain. Under **My Certificates**, check that its private key appears beneath it.
4. Export the certificate **and its private key** as a password-protected `.p12`. Keep the password and export securely.
5. Run `security find-identity -v -p codesigning` and copy the full `Developer ID Application: Your Name (TEAMID)` identity. Your ten-character team ID is also listed in Apple's membership details.

## 3. Create a notarization API key

In [App Store Connect](https://appstoreconnect.apple.com/access/integrations/api), open **Users and Access → Integrations → App Store Connect API**, request API access if necessary, and create a **team API key** with the Developer role (or App Manager). Record its key ID and issuer ID. Download the `.p8` private key; Apple only offers the download once. The workflow uses this team key with `notarytool`; individual keys have a different authentication flow and are not supported by this configuration.

## 4. Add GitHub Actions secrets

Open [this repository's Actions secrets](https://github.com/ar4ft/nainzed/settings/secrets/actions). Add these repository secrets directly in GitHub; do not put credentials in commits, issues, or chat.

| Secret | Value |
| --- | --- |
| `MACOS_CERTIFICATE_BASE64` | Base64-encoded `.p12` including its private key |
| `MACOS_CERTIFICATE_PASSWORD` | Password used when exporting the `.p12` |
| `MACOS_SIGNING_IDENTITY` | Full `Developer ID Application: … (TEAMID)` identity |
| `APPLE_TEAM_ID` | Ten-character Apple team ID |
| `APPLE_NOTARIZATION_KEY` | Complete `.p8` file contents, including header/footer |
| `APPLE_NOTARIZATION_KEY_ID` | App Store Connect team API key ID |
| `APPLE_NOTARIZATION_ISSUER_ID` | App Store Connect issuer UUID |

On your Mac, `base64 < certificate.p12 | tr -d '\n' | pbcopy` copies the certificate's encoded value for GitHub's secret field. Clear the clipboard after adding it. The workflow creates a temporary keychain and private key file on each hosted Mac runner and deletes them after use. No personal GitHub token is needed; the publish job uses the built-in Actions token with repository contents write permission.

## 5. Publish the first release

After the secrets are configured, tag a commit containing this pipeline on `main`. Pushing the tag does not start signing:

```sh
git checkout main
git pull --ff-only
# The fork's release version is independent of upstream Zed's version.
git tag -a v1.0.0 -m 'nain 1.0.0'
git push origin v1.0.0
```

Open [Signed nain release](https://github.com/ar4ft/nainzed/actions/workflows/no-ai-release.yml). Click **Run workflow**, choose the `main` branch, enter the existing tag (for example `v1.0.0`), and run it. This explicit manual action starts signing and notarization. Both Mac jobs must pass before the draft becomes public and the release becomes the latest update. A partial failure can leave a draft, which is invisible to the update feed. Re-run failed jobs, or manually dispatch the workflow with that existing tag. Published releases are not overwritten by retries. Fixes after a published release require a new, increasing version such as `v1.0.1`; never move an existing release tag.

Download the architecture's signed DMG or ZIP from [Releases](https://github.com/ar4ft/nainzed/releases) and replace the existing unsigned app. Subsequent signed versions use automatic updates. Release assets include `SHA256SUMS.txt` for manual verification. Keep the Developer ID certificate valid and renew it before expiry; the signing identity and Apple team must match the updater's embedded team.

## Native verification before relying on updates

A successful signed workflow validates signatures, notarization acceptance and stapling. Also install `v1.0.0` on a Mac, publish `v1.0.1`, run **auto update: Check**, restart, and verify the new version. Repeat for Intel and Apple Silicon. Test with automatic updates disabled and confirm manual checking still works. These runtime checks cannot be performed in the Linux development environment.
