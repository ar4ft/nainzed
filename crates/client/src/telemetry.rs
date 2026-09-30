//! Compatibility API for upstream callers. This fork has no telemetry collector.
use anyhow::Result;
use clock::SystemClock;
use fs::Fs;
use futures::channel::mpsc;
use gpui::{App, Task};
use http_client::HttpClientWithUrl;
use release_channel::ReleaseChannel;
use std::{
    path::PathBuf,
    sync::{Arc, LazyLock},
};
use telemetry_events::{AssistantEventData, EventWrapper};
use worktree::{UpdatedEntriesSet, WorktreeId};

pub struct Telemetry;
pub struct TelemetrySubscription {
    pub historical_events: Result<HistoricalEvents>,
    pub queued_events: Vec<EventWrapper>,
    pub live_events: mpsc::UnboundedReceiver<EventWrapper>,
}
pub struct HistoricalEvents {
    pub events: Vec<EventWrapper>,
    pub parse_error_count: usize,
}
// Build-time and runtime endpoint overrides cannot enable crash uploads.
pub static MINIDUMP_ENDPOINT: LazyLock<Option<String>> = LazyLock::new(|| None);
pub fn should_install_crash_handler(_: ReleaseChannel) -> bool {
    false
}

pub fn os_name() -> String {
    #[cfg(target_os = "macos")]
    {
        "macOS".to_string()
    }
    #[cfg(target_os = "linux")]
    {
        format!("Linux {}", gpui::guess_compositor())
    }
    #[cfg(target_os = "freebsd")]
    {
        format!("FreeBSD {}", gpui::guess_compositor())
    }

    #[cfg(target_os = "windows")]
    {
        "Windows".to_string()
    }
}

/// Note: This might do blocking IO! Only call from background threads
pub fn os_version() -> String {
    cfg_select! {
       feature = "test-support" => {
           // MacOS branch in particular is quite slow, hence we ought to "avoid" it in tests.
           "test binary".to_owned()
       }
       target_os = "macos" => {
           use regex::Regex;
           static MACOS_VERSION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
               Regex::new(r"(\s*\(Build [^)]*[0-9]\))").unwrap()
           });
           use objc2_foundation::NSProcessInfo;
           let process_info = NSProcessInfo::processInfo();
           let version_nsstring = process_info.operatingSystemVersionString();
           // "Version 15.6.1 (Build 24G90)" -> "15.6.1 (Build 24G90)"
           let version_string = version_nsstring.to_string().replace("Version ", "");
           // "15.6.1 (Build 24G90)" -> "15.6.1"
           // "26.0.0 (Build 25A5349a)" -> unchanged (Beta or Rapid Security Response; ends with letter)
           MACOS_VERSION_REGEX
               .replace_all(&version_string, "")
               .to_string()
       }
       any(target_os = "linux", target_os = "freebsd") => {
           use std::path::Path;

           let content = if let Ok(file) = std::fs::read_to_string(&Path::new("/etc/os-release")) {
               file
           } else if let Ok(file) = std::fs::read_to_string(&Path::new("/usr/lib/os-release")) {
               file
           } else if let Ok(file) = std::fs::read_to_string(&Path::new("/var/run/os-release")) {
               file
           } else {
               log::error!(
                   "Failed to load /etc/os-release, /usr/lib/os-release, or /var/run/os-release"
               );
               "".to_string()
           };
           util::parse_os_release(&content).unwrap_or_else(|| "unknown".to_string())
       }
       target_os = "windows" => {
           let mut info = unsafe { std::mem::zeroed() };
           let status = unsafe { windows::Wdk::System::SystemServices::RtlGetVersion(&mut info) };
           if status.is_ok() {
               semver::Version::new(
                   info.dwMajorVersion as _,
                   info.dwMinorVersion as _,
                   info.dwBuildNumber as _,
               )
               .to_string()
           } else {
               "unknown".to_string()
           }
       }
    }
}

impl Telemetry {
    pub fn new(_: Arc<dyn SystemClock>, _: Arc<HttpClientWithUrl>, _: &mut App) -> Arc<Self> {
        Arc::new(Self)
    }
    pub fn log_file_path() -> PathBuf {
        paths::logs_dir().join("telemetry.log")
    }
    pub async fn subscribe_with_history(self: &Arc<Self>, _: Arc<dyn Fs>) -> TelemetrySubscription {
        let (_, rx) = mpsc::unbounded();
        TelemetrySubscription {
            historical_events: Ok(HistoricalEvents {
                events: vec![],
                parse_error_count: 0,
            }),
            queued_events: vec![],
            live_events: rx,
        }
    }
    pub fn has_checksum_seed(&self) -> bool {
        false
    }
    pub fn start(self: &Arc<Self>, _: Option<String>, _: Option<String>, _: String, _: &App) {}
    pub fn metrics_enabled(self: &Arc<Self>) -> bool {
        false
    }
    pub fn diagnostics_enabled(self: &Arc<Self>) -> bool {
        false
    }
    pub fn set_authenticated_user_info(self: &Arc<Self>, _: Option<String>, _: bool) {}
    pub fn report_assistant_event(self: &Arc<Self>, _: AssistantEventData) {}
    pub fn log_edit_event(self: &Arc<Self>, _: &'static str, _: bool) {}
    pub fn report_discovered_project_type_events(
        self: &Arc<Self>,
        _: WorktreeId,
        _: &UpdatedEntriesSet,
    ) {
    }
    pub fn report_remote_event(
        self: &Arc<Self>,
        _: &str,
        _: &str,
        _: String,
        _: Option<String>,
        _: String,
    ) -> Result<()> {
        Ok(())
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn queued_events(self: &Arc<Self>) -> Vec<telemetry_events::FlexibleEvent> {
        vec![]
    }
    pub fn metrics_id(self: &Arc<Self>) -> Option<Arc<str>> {
        None
    }
    pub fn system_id(self: &Arc<Self>) -> Option<Arc<str>> {
        None
    }
    pub fn installation_id(self: &Arc<Self>) -> Option<Arc<str>> {
        None
    }
    pub fn is_staff(self: &Arc<Self>) -> Option<bool> {
        None
    }
    pub async fn flush_events_inner(self: &Arc<Self>) -> Result<()> {
        Ok(())
    }
    pub fn flush_events(self: &Arc<Self>) -> Task<()> {
        Task::ready(())
    }
}
pub fn calculate_json_checksum(_: &impl AsRef<[u8]>) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TelemetrySettings;
    use settings::Settings;
    #[test]
    fn no_telemetry_fork_settings_cannot_enable_reporting() {
        let content = serde_json::from_str(
            r#"{"telemetry":{"metrics":true,"diagnostics":true,"anthropic_retention":true}}"#,
        )
        .unwrap();
        let settings = TelemetrySettings::from_settings(&content);
        assert!(!settings.metrics && !settings.diagnostics && !settings.anthropic_retention);
        assert!(!should_install_crash_handler(ReleaseChannel::Stable));
        assert!(MINIDUMP_ENDPOINT.is_none());
    }
    #[test]
    fn no_telemetry_fork_discards_remote_events_and_identifiers() {
        let telemetry = Arc::new(Telemetry);
        telemetry.set_authenticated_user_info(Some("private-id".into()), true);
        assert!(
            telemetry
                .report_remote_event(
                    "unparsed private data",
                    "ssh",
                    "ignored".into(),
                    None,
                    "ignored".into()
                )
                .is_ok()
        );
        assert!(telemetry.queued_events().is_empty());
        assert!(
            telemetry.metrics_id().is_none()
                && telemetry.system_id().is_none()
                && telemetry.installation_id().is_none()
        );
        assert!(!telemetry.metrics_enabled() && !telemetry.diagnostics_enabled());
    }
}
