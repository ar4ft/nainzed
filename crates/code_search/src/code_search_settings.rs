use code_search_provider::SearchMode;
use gpui::{Pixels, px};
use settings::{CodeSearchMode, DockSide, RegisterSetting, Settings, SettingsContent};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, RegisterSetting)]
pub struct CodeSearchSettings {
    pub enabled: bool,
    pub button: bool,
    pub dock: DockSide,
    pub default_width: Pixels,
    pub agx_path: Option<PathBuf>,
    pub default_mode: SearchMode,
}
impl Settings for CodeSearchSettings {
    fn from_settings(content: &SettingsContent) -> Self {
        let panel = content.code_search.as_ref();
        Self {
            enabled: panel.and_then(|p| p.enabled).unwrap_or(true),
            button: panel.and_then(|p| p.button).unwrap_or(true),
            dock: panel.and_then(|p| p.dock).unwrap_or(DockSide::Left),
            default_width: px(panel
                .and_then(|p| p.default_width)
                .map_or(360., |v| v.0)
                .max(280.)),
            agx_path: panel.and_then(|p| p.agx_path.as_ref()).map(PathBuf::from),
            default_mode: match panel.and_then(|p| p.default_mode).unwrap_or_default() {
                CodeSearchMode::Text => SearchMode::Text,
                CodeSearchMode::Symbol => SearchMode::Symbol,
                CodeSearchMode::Ranked => SearchMode::Ranked,
            },
        }
    }
}
