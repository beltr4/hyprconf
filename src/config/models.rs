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
    pub group: GroupSection,
    pub misc: MiscSection,
    pub binds: BindsSection,
    pub xwayland: XWaylandSection,
    pub opengl: OpenGLSection,
    pub render: RenderSection,
    pub cursor: CursorSection,
    pub dwindle: DwindleSection,
    pub master: MasterSection,
    pub debug: DebugSection,
    pub monitors: Vec<MonitorConfig>,
    pub devices: Vec<DeviceConfig>,
    pub window_rules: Vec<WindowRule>,
    pub workspace_rules: Vec<WorkspaceRule>,
    pub layer_rules: Vec<LayerRule>,
    pub variables: HashMap<String, String>,
    pub environment_variables: HashMap<String, String>,
    pub autostart_programs: Vec<String>,
    pub bezier_curves: HashMap<String, String>,
    pub submap_definitions: HashMap<String, Vec<KeyBind>>,
    pub ecosystem: EcosystemSection,
    pub experimental: ExperimentalSection,
    pub permissions: Vec<Permission>,
}

/// General section with snap subcategory
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralSection {
    pub border_size: i32,
    pub no_border_on_floating: bool,
    pub gaps_in: String,  // Could be int or css-style gaps
    pub gaps_out: String, // Could be int or css-style gaps
    pub gaps_workspaces: i32,
    pub col_inactive_border: String,
    pub col_active_border: String,
    pub col_nogroup_border: String,
    pub col_nogroup_border_active: String,
    pub layout: String,
    pub no_focus_fallback: bool,
    pub resize_on_border: bool,
    pub extend_border_grab_area: i32,
    pub hover_icon_on_border: bool,
    pub allow_tearing: bool,
    pub resize_corner: i32,
    pub snap: SnapSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapSection {
    pub enabled: bool,
    pub window_gap: i32,
    pub monitor_gap: i32,
    pub border_overlap: bool,
}

/// Decoration section with blur and shadow subcategories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecorationSection {
    pub rounding: i32,
    pub rounding_power: f32,
    pub active_opacity: f32,
    pub inactive_opacity: f32,
    pub fullscreen_opacity: f32,
    pub dim_inactive: bool,
    pub dim_strength: f32,
    pub dim_special: f32,
    pub dim_around: f32,
    pub screen_shader: String,
    pub border_part_of_window: bool,
    pub blur: BlurSection,
    pub shadow: ShadowSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlurSection {
    pub enabled: bool,
    pub size: i32,
    pub passes: i32,
    pub ignore_opacity: bool,
    pub new_optimizations: bool,
    pub xray: bool,
    pub noise: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub vibrancy: f32,
    pub vibrancy_darkness: f32,
    pub special: bool,
    pub popups: bool,
    pub popups_ignorealpha: f32,
    pub input_methods: bool,
    pub input_methods_ignorealpha: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShadowSection {
    pub enabled: bool,
    pub range: i32,
    pub render_power: i32,
    pub sharp: bool,
    pub ignore_window: bool,
    pub color: String,
    pub color_inactive: String,
    pub offset: (f32, f32),
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimationsSection {
    pub enabled: bool,
    pub first_launch_animation: bool,
    pub beziers: HashMap<String, String>,
    pub animations: Vec<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub enabled: bool,
    pub speed: i32,
    pub curve: String,
    pub style: Option<String>,
}

/// Input section with touchpad, touchdevice, and tablet subcategories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputSection {
    pub kb_model: String,
    pub kb_layout: String,
    pub kb_variant: String,
    pub kb_options: String,
    pub kb_rules: String,
    pub kb_file: String,
    pub numlock_by_default: bool,
    pub resolve_binds_by_sym: bool,
    pub repeat_rate: i32,
    pub repeat_delay: i32,
    pub sensitivity: f32,
    pub accel_profile: String,
    pub force_no_accel: bool,
    pub left_handed: bool,
    pub scroll_points: String,
    pub scroll_method: String,
    pub scroll_button: i32,
    pub scroll_button_lock: bool,
    pub scroll_factor: f32,
    pub natural_scroll: bool,
    pub follow_mouse: i32,
    pub follow_mouse_threshold: f32,
    pub focus_on_close: i32,
    pub mouse_refocus: bool,
    pub float_switch_override_focus: i32,
    pub special_fallthrough: bool,
    pub off_window_axis_events: i32,
    pub emulate_discrete_scroll: i32,
    pub drag_threshold: i32,
    pub touchpad: TouchpadSection,
    pub touchdevice: TouchdeviceSection,
    pub tablet: TabletSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchpadSection {
    pub disable_while_typing: bool,
    pub natural_scroll: bool,
    pub scroll_factor: f32,
    pub middle_button_emulation: bool,
    pub tap_button_map: String,
    pub clickfinger_behavior: bool,
    pub tap_to_click: bool,
    pub drag_lock: bool,
    pub tap_and_drag: bool,
    pub flip_x: bool,
    pub flip_y: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchdeviceSection {
    pub transform: i32,
    pub output: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TabletSection {
    pub transform: i32,
    pub output: String,
    pub region_position: (f32, f32),
    pub absolute_region_position: bool,
    pub region_size: (f32, f32),
    pub relative_input: bool,
    pub left_handed: bool,
    pub active_area_size: (f32, f32),
    pub active_area_position: (f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GesturesSection {
    pub workspace_swipe: bool,
    pub workspace_swipe_fingers: i32,
    pub workspace_swipe_min_fingers: bool,
    pub workspace_swipe_distance: i32,
    pub workspace_swipe_touch: bool,
    pub workspace_swipe_invert: bool,
    pub workspace_swipe_touch_invert: bool,
    pub workspace_swipe_min_speed_to_force: i32,
    pub workspace_swipe_cancel_ratio: f32,
    pub workspace_swipe_create_new: bool,
    pub workspace_swipe_direction_lock: bool,
    pub workspace_swipe_direction_lock_threshold: i32,
    pub workspace_swipe_forever: bool,
    pub workspace_swipe_use_r: bool,
}

/// Group section with groupbar subcategory
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupSection {
    pub auto_group: bool,
    pub insert_after_current: bool,
    pub focus_removed_window: bool,
    pub drag_into_group: i32,
    pub merge_groups_on_drag: bool,
    pub merge_groups_on_groupbar: bool,
    pub merge_floated_into_tiled_on_groupbar: bool,
    pub group_on_movetoworkspace: bool,
    pub col_border_active: String,
    pub col_border_inactive: String,
    pub col_border_locked_active: String,
    pub col_border_locked_inactive: String,
    pub groupbar: GroupbarSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupbarSection {
    pub enabled: bool,
    pub font_family: String,
    pub font_size: i32,
    pub gradients: bool,
    pub height: i32,
    pub indicator_height: i32,
    pub stacked: bool,
    pub priority: i32,
    pub render_titles: bool,
    pub text_offset: i32,
    pub scrolling: bool,
    pub rounding: i32,
    pub gradient_rounding: i32,
    pub round_only_edges: bool,
    pub gradient_round_only_edges: bool,
    pub text_color: String,
    pub col_active: String,
    pub col_inactive: String,
    pub col_locked_active: String,
    pub col_locked_inactive: String,
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub keep_upper_gap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiscSection {
    pub disable_hyprland_logo: bool,
    pub disable_splash_rendering: bool,
    pub col_splash: String,
    pub font_family: String,
    pub splash_font_family: String,
    pub force_default_wallpaper: i32,
    pub vfr: bool,
    pub vrr: i32,
    pub mouse_move_enables_dpms: bool,
    pub key_press_enables_dpms: bool,
    pub always_follow_on_dnd: bool,
    pub layers_hog_keyboard_focus: bool,
    pub animate_manual_resizes: bool,
    pub animate_mouse_windowdragging: bool,
    pub disable_autoreload: bool,
    pub enable_swallow: bool,
    pub swallow_regex: String,
    pub swallow_exception_regex: String,
    pub focus_on_activate: bool,
    pub mouse_move_focuses_monitor: bool,
    pub render_ahead_of_time: bool,
    pub render_ahead_safezone: i32,
    pub allow_session_lock_restore: bool,
    pub background_color: String,
    pub close_special_on_empty: bool,
    pub new_window_takes_over_fullscreen: i32,
    pub exit_window_retains_fullscreen: bool,
    pub initial_workspace_tracking: i32,
    pub middle_click_paste: bool,
    pub render_unfocused_fps: i32,
    pub disable_xdg_env_checks: bool,
    pub disable_hyprland_qtutils_check: bool,
    pub lockdead_screen_delay: i32,
    pub enable_anr_dialog: bool,
    pub anr_missed_pings: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BindsSection {
    pub pass_mouse_when_bound: bool,
    pub scroll_event_delay: i32,
    pub workspace_back_and_forth: bool,
    pub hide_special_on_workspace_change: bool,
    pub allow_workspace_cycles: bool,
    pub workspace_center_on: i32,
    pub focus_preferred_method: i32,
    pub ignore_group_lock: bool,
    pub movefocus_cycles_fullscreen: bool,
    pub movefocus_cycles_groupfirst: bool,
    pub disable_keybind_grabbing: bool,
    pub window_direction_monitor_fallback: bool,
    pub allow_pin_fullscreen: bool,
    pub keybinds: Vec<KeyBind>,
    pub submaps: HashMap<String, Vec<KeyBind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBind {
    pub modifiers: String,
    pub key: String,
    pub dispatchers: Vec<String>,
    pub arg: String,
    pub flags: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct XWaylandSection {
    pub enabled: bool,
    pub use_nearest_neighbor: bool,
    pub force_zero_scaling: bool,
    pub create_abstract_socket: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenGLSection {
    pub nvidia_anti_flicker: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RenderSection {
    pub explicit_sync: i32,
    pub explicit_sync_kms: i32,
    pub direct_scanout: i32,
    pub expand_undersized_textures: bool,
    pub xp_mode: bool,
    pub ctm_animation: i32,
    pub cm_fs_passthrough: i32,
    pub cm_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorSection {
    pub sync_gsettings_theme: bool,
    pub no_hardware_cursors: i32,
    pub no_break_fs_vrr: i32,
    pub min_refresh_rate: i32,
    pub hotspot_padding: i32,
    pub inactive_timeout: f32,
    pub no_warps: bool,
    pub persistent_warps: bool,
    pub warp_on_change_workspace: i32,
    pub warp_on_toggle_special: i32,
    pub default_monitor: String,
    pub zoom_factor: f32,
    pub zoom_rigid: bool,
    pub enable_hyprcursor: bool,
    pub hide_on_key_press: bool,
    pub hide_on_touch: bool,
    pub use_cpu_buffer: i32,
    pub warp_back_after_non_mouse_input: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DwindleSection {
    pub pseudotile: bool,
    pub preserve_split: bool,
    pub smart_split: bool,
    pub force_split: i32,
    pub permanent_direction_override: bool,
    pub special_scale_factor: f32,
    pub split_width_multiplier: f32,
    pub use_active_for_splits: bool,
    pub default_split_ratio: f32,
    pub split_bias: i32,
    pub smart_resizing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasterSection {
    pub allow_small_split: bool,
    pub special_scale_factor: f32,
    pub mfact: f32,
    pub new_status: String,
    pub new_on_top: bool,
    pub new_on_active: String,
    pub orientation: String,
    pub inherit_fullscreen: bool,
    pub slave_count_for_center_master: i32,
    pub center_master_slaves_on_right: bool,
    pub smart_resizing: bool,
    pub drop_at_cursor: bool,
    pub always_keep_position: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugSection {
    pub overlay: bool,
    pub damage_blink: bool,
    pub disable_logs: bool,
    pub disable_time: bool,
    pub damage_tracking: i32,
    pub enable_stdout_logs: bool,
    pub manual_crash: i32,
    pub suppress_errors: bool,
    pub watchdog_timeout: i32,
    pub disable_scale_checks: bool,
    pub error_limit: i32,
    pub error_position: i32,
    pub colored_stdout_logs: bool,
    pub pass: bool,
    pub full_cm_proto: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemSection {
    pub no_update_news: bool,
    pub no_donation_nag: bool,
    pub enforce_permissions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperimentalSection {
    pub xx_color_management_v4: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub name: String,
    pub resolution: String,
    pub position: String,
    pub scale: f32,
    pub transform: Option<i32>,
    pub mirror: Option<String>,
    pub bitdepth: Option<i32>,
    pub color_management: Option<String>,
    pub sdr_brightness: Option<f32>,
    pub sdr_saturation: Option<f32>,
    pub vrr: Option<i32>,
    pub disable: bool,
    pub reserved_area: Option<(i32, i32, i32, i32)>,  // top, bottom, left, right
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            name: String::default(),
            resolution: String::default(),
            position: String::default(),
            scale: 1.0,
            transform: None,
            mirror: None,
            bitdepth: None,
            color_management: None,
            sdr_brightness: None,
            sdr_saturation: None,
            vrr: None,
            disable: false,
            reserved_area: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceConfig {
    pub name: String,
    pub sensitivity: Option<f32>,
    pub accel_profile: Option<String>,
    pub kb_layout: Option<String>,
    pub kb_variant: Option<String>,
    pub kb_options: Option<String>,
    pub kb_rules: Option<String>,
    pub kb_model: Option<String>,
    pub repeat_rate: Option<i32>,
    pub repeat_delay: Option<i32>,
    pub natural_scroll: Option<bool>,
    pub tap_button_map: Option<String>,
    pub tap_to_click: Option<bool>,
    pub middle_button_emulation: Option<bool>,
    pub clickfinger_behavior: Option<bool>,
    pub drag_lock: Option<bool>,
    pub tap_and_drag: Option<bool>,
    pub left_handed: Option<bool>,
    pub scroll_method: Option<String>,
    pub scroll_button: Option<i32>,
    pub transform: Option<i32>,
    pub output: Option<String>,
    pub enabled: Option<bool>,
    pub keybinds: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowRule {
    pub rule: String,
    pub value: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceRule {
    pub workspace: String,
    pub rules: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayerRule {
    pub rule: String,
    pub target: String,
    pub value: Option<String>,
}

/// Permission definition for Hyprland's permission system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub path_regex: String,
    pub permission_type: String,
    pub mode: PermissionMode,
}

/// Permission modes available in Hyprland
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionMode {
    Allow,
    Ask,
    Deny,
}

impl std::fmt::Display for PermissionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionMode::Allow => write!(f, "allow"),
            PermissionMode::Ask => write!(f, "ask"),
            PermissionMode::Deny => write!(f, "deny"),
        }
    }
}

impl std::str::FromStr for PermissionMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "allow" => Ok(PermissionMode::Allow),
            "ask" => Ok(PermissionMode::Ask),
            "deny" => Ok(PermissionMode::Deny),
            _ => Err(format!("Invalid permission mode: {}", s)),
        }
    }
}

/// Hyprland socket event type for reactive scripting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HyprlandEvent {
    WorkspaceChanged(String),
    FocusedMon(String, String), // Monitor name, workspace name
    ActiveWindow(String, String), // Window address, window title
    FullscreenState(String), // Window address
    MonitorRemoved(String), // Monitor name
    MonitorAdded(String), // Monitor name
    CreateWorkspace(String), // Workspace name
    DestroyWorkspace(String), // Workspace name
    MoveWorkspace(String, String), // Workspace name, monitor name
    ActiveLayout(String, String), // Keyboard name, layout name
    ActiveSpecial(String), // Special workspace name
    UrgentWindow(String), // Window address
    Minimize(String, bool), // Window address, minimized state
    SubMap(String), // Submap name
    WindowClose(String), // Window address
    Unknown(String), // Any other events
}

/// Hyprctl batch command structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyprctlBatch {
    pub commands: Vec<String>,
}

impl HyprctlBatch {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn to_string(&self) -> String {
        self.commands.join(" ; ")
    }
}