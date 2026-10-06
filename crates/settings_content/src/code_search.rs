use crate::{DockSide, PixelSetting};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug, PartialEq)]
pub struct CodeSearchSettingsContent {
    /// Enable the optional local Code Search panel. No worker starts until a query is submitted.
    /// Default: true
    pub enabled: Option<bool>,
    /// Show the Code Search buttons. Default: true
    pub button: Option<bool>,
    /// Dock at the left or right. Default: left
    pub dock: Option<DockSide>,
    /// Default panel width in pixels. Default: 360
    pub default_width: Option<PixelSetting>,
    /// Absolute path to an existing agx executable. Unset searches PATH and common install locations.
    pub agx_path: Option<String>,
    /// Default search mode. Ranked search uses lexical BM25 without models.
    pub default_mode: Option<CodeSearchMode>,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize, JsonSchema, MergeFrom)]
#[serde(rename_all = "lowercase")]
pub enum CodeSearchMode {
    Text,
    #[default]
    Symbol,
    Ranked,
}
