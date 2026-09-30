use anyhow::{Context as _, Result, ensure};
use semver::Version;
use serde::Deserialize;

pub const REPOSITORY: &str = "ar4ft/zed-no-ai";
pub const BUNDLE_ID: &str = "io.github.ar4ft.ZedNoAI";
pub const VERSION: Option<&str> = option_env!("ZED_NO_AI_RELEASE_VERSION");
pub const TEAM_ID: Option<&str> = option_env!("ZED_NO_AI_TEAM_ID");

pub fn enabled() -> bool {
    cfg!(target_os = "macos") && VERSION.is_some() && TEAM_ID.is_some()
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}
#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub fn parse(body: &[u8], arch: &str) -> Result<super::ReleaseAsset> {
    let release: Release = serde_json::from_slice(body)?;
    ensure!(
        !release.draft && !release.prerelease,
        "Not a stable release"
    );
    let version = release
        .tag_name
        .strip_prefix('v')
        .context("Expected a v-prefixed release tag")?;
    let parsed = Version::parse(version)?;
    ensure!(
        parsed.pre.is_empty() && parsed.build.is_empty(),
        "Expected a stable version"
    );
    let arch = match arch {
        "aarch64" => "arm64",
        "x86_64" => "x86_64",
        _ => anyhow::bail!("Unsupported Mac architecture"),
    };
    let name = format!("Zed-No-AI-{arch}.dmg");
    // Publish both architectures together so all Macs see the same version.
    for arch in ["arm64", "x86_64"] {
        ensure!(
            release
                .assets
                .iter()
                .any(|a| a.name == format!("Zed-No-AI-{arch}.dmg")),
            "Release is missing a Mac architecture"
        );
    }
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == name)
        .context("Missing installer")?;
    let expected = format!(
        "https://github.com/{REPOSITORY}/releases/download/{}/{name}",
        release.tag_name
    );
    ensure!(
        asset.browser_download_url == expected,
        "Unexpected update download URL"
    );
    Ok(super::ReleaseAsset {
        version: version.into(),
        url: expected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn release() -> serde_json::Value {
        serde_json::json!({"tag_name":"v1.0.1","draft":false,"prerelease":false,"assets":[
            {"name":"Zed-No-AI-arm64.dmg","browser_download_url":"https://github.com/ar4ft/zed-no-ai/releases/download/v1.0.1/Zed-No-AI-arm64.dmg"},
            {"name":"Zed-No-AI-x86_64.dmg","browser_download_url":"https://github.com/ar4ft/zed-no-ai/releases/download/v1.0.1/Zed-No-AI-x86_64.dmg"}
        ]})
    }
    #[test]
    fn no_ai_fork_selects_matching_signed_release_architecture() {
        for (arch, suffix) in [("aarch64", "arm64"), ("x86_64", "x86_64")] {
            let result = parse(&serde_json::to_vec(&release()).unwrap(), arch).unwrap();
            assert_eq!(result.version, "1.0.1");
            assert!(result.url.ends_with(&format!("-{suffix}.dmg")));
        }
    }
    #[test]
    fn no_ai_fork_rejects_untrusted_or_incomplete_releases() {
        let mut unsafe_url = release();
        unsafe_url["assets"][0]["browser_download_url"] = "https://example.com/app.dmg".into();
        let mut draft = release();
        draft["draft"] = true.into();
        let mut prerelease = release();
        prerelease["prerelease"] = true.into();
        let mut missing = release();
        missing["assets"].as_array_mut().unwrap().pop();
        let mut invalid = release();
        invalid["tag_name"] = "v1.0.1-beta".into();
        for value in [unsafe_url, draft, prerelease, missing, invalid] {
            assert!(parse(&serde_json::to_vec(&value).unwrap(), "aarch64").is_err());
        }
    }
}

/// Stage and backup live on the app's filesystem so rename does not copy code.
pub fn replace_app(
    staged: &std::path::Path,
    installed: &std::path::Path,
    backup: &std::path::Path,
) -> Result<()> {
    std::fs::rename(installed, backup).context("Cannot move installed app to backup")?;
    if let Err(error) = std::fs::rename(staged, installed) {
        if let Err(rollback) = std::fs::rename(backup, installed) {
            anyhow::bail!(
                "Update failed: {error}; rollback failed: {rollback}. Previous app: {}",
                backup.display()
            );
        }
        return Err(error).context("Replacement failed; the previous app was restored");
    }
    Ok(())
}

#[cfg(test)]
mod replacement_tests {
    use super::*;
    #[test]
    fn no_ai_fork_installs_staged_app_and_keeps_backup_until_cleanup() {
        let root = tempfile::tempdir().unwrap();
        let installed = root.path().join("installed.app");
        let staged = root.path().join("new.app");
        let backup = root.path().join("old.app");
        std::fs::create_dir(&installed).unwrap();
        std::fs::write(installed.join("version"), "old").unwrap();
        std::fs::create_dir(&staged).unwrap();
        std::fs::write(staged.join("version"), "new").unwrap();
        replace_app(&staged, &installed, &backup).unwrap();
        assert_eq!(
            std::fs::read_to_string(installed.join("version")).unwrap(),
            "new"
        );
        assert_eq!(
            std::fs::read_to_string(backup.join("version")).unwrap(),
            "old"
        );
    }
    #[test]
    fn no_ai_fork_restores_previous_app_if_replacement_fails() {
        let root = tempfile::tempdir().unwrap();
        let installed = root.path().join("installed.app");
        let staged = root.path().join("missing.app");
        let backup = root.path().join("old.app");
        std::fs::create_dir(&installed).unwrap();
        std::fs::write(installed.join("version"), "old").unwrap();
        assert!(replace_app(&staged, &installed, &backup).is_err());
        assert_eq!(
            std::fs::read_to_string(installed.join("version")).unwrap(),
            "old"
        );
        assert!(!backup.exists());
    }
}
