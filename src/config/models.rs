// src/config/models.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a complete Hyprland configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprlandConfig {
    pub general: GeneralSection,
    pub decoration: DecorationSection,
    pub animations: AnimationsSection,
    pub input: InputSection,
    pub gestures: GesturesSection,
    pub misc: MiscSection,
    pub binds: Vec<KeyBinding>,
    pub monitors: Vec<MonitorConfig>,
    pub devices: Vec<DeviceConfig>,
    pub window_rules: Vec<WindowRule>,
    pub workspace_rules: Vec<WorkspaceRule>,
    pub variables: HashMap<String, String>,
    pub dwindle: DwindleSection,
    pub master: MasterSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralSection {
    pub border_size: i32,
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub col_active_border: String,
    pub col_inactive_border: String,
    pub cursor_inactive_timeout: i32,
    pub resize_on_border: bool,
    pub allow_tearing: bool,
    pub layout: String,
}

impl Default for GeneralSection {
    fn default() -> Self {
        Self {
            border_size: 2,
            gaps_in: 5,
            gaps_out: 20,
            col_active_border: "rgba(33ccffee)".to_string(),
            col_inactive_border: "rgba(595959aa)".to_string(),
            cursor_inactive_timeout: 0,
            resize_on_border: false,
            allow_tearing: false,
            layout: "dwindle".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecorationSection {
    pub rounding: i32,
    pub blur: bool,
    pub blur_size: i32,
    pub blur_passes: i32,
    pub active_opacity: f32,
    pub inactive_opacity: f32,
    pub shadow_enabled: bool,
    pub shadow_range: i32,
    pub shadow_render_power: i32,
    pub shadow_color: String,
    pub blur_vibrancy: f32,
}

impl Default for DecorationSection {
    fn default() -> Self {
        Self {
            rounding: 10,
            blur: true,
            blur_size: 3,
            blur_passes: 1,
            active_opacity: 1.0,
            inactive_opacity: 1.0,
            shadow_enabled: true,
            shadow_range: 4,
            shadow_render_power: 3,
            shadow_color: "rgba(1a1a1aee)".to_string(),
            blur_vibrancy: 0.1696,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimationsSection {
    pub enabled: bool,
    pub beziers: HashMap<String, String>,
    pub animations: Vec<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub params: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputSection {
    pub kb_layout: String,
    pub kb_variant: String,
    pub kb_model: String,
    pub kb_options: String,
    pub kb_rules: String,
    pub follow_mouse: i32,
    pub sensitivity: f32,
    pub touchpad: TouchpadConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchpadConfig {
    pub natural_scroll: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GesturesSection {
    pub workspace_swipe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiscSection {
    pub force_default_wallpaper: i32,
    pub disable_hyprland_logo: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub modifiers: String,
    pub key: String,
    pub action: String,
    pub params: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitorConfig {
    pub name: String,
    pub resolution: String,
    pub position: String,
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceConfig {
    pub name: String,
    pub sensitivity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowRule {
    pub rule: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceRule {
    pub name: String,
    pub params: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DwindleSection {
    pub pseudotile: bool,
    pub preserve_split: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasterSection {
    pub new_status: String,
}