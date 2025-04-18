use crate::config::models::CursorSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_cursor_section(section: &mut CursorSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "sync_gsettings_theme" => section.sync_gsettings_theme = parse_bool(value),
                "no_hardware_cursors" => section.no_hardware_cursors = value.parse().unwrap_or(2),
                "no_break_fs_vrr" => section.no_break_fs_vrr = value.parse().unwrap_or(2),
                "min_refresh_rate" => section.min_refresh_rate = value.parse().unwrap_or(24),
                "hotspot_padding" => section.hotspot_padding = value.parse().unwrap_or(1),
                "inactive_timeout" => section.inactive_timeout = value.parse().unwrap_or(0.0),
                "no_warps" => section.no_warps = parse_bool(value),
                "persistent_warps" => section.persistent_warps = parse_bool(value),
                "warp_on_change_workspace" => section.warp_on_change_workspace = value.parse().unwrap_or(0),
                "warp_on_toggle_special" => section.warp_on_toggle_special = value.parse().unwrap_or(0),
                "default_monitor" => section.default_monitor = value.to_string(),
                "zoom_factor" => section.zoom_factor = value.parse().unwrap_or(1.0),
                "zoom_rigid" => section.zoom_rigid = parse_bool(value),
                "enable_hyprcursor" => section.enable_hyprcursor = parse_bool(value),
                "hide_on_key_press" => section.hide_on_key_press = parse_bool(value),
                "hide_on_touch" => section.hide_on_touch = parse_bool(value),
                "use_cpu_buffer" => section.use_cpu_buffer = value.parse().unwrap_or(2),
                "warp_back_after_non_mouse_input" => section.warp_back_after_non_mouse_input = parse_bool(value),
                _ => debug!("Unknown cursor setting: {}", key),
            }
        }
    }
    
    Ok(())
}