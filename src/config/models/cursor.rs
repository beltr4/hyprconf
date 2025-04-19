use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


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

pub fn write_section<W: Write>(
    writer: &mut W,
    cursor: &CursorSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "cursor {", comment_style)?;
    
    // Write all cursor settings
    write_boolean_option(writer, " sync_gsettings_theme", cursor.sync_gsettings_theme, None, comment_style)?;
    write_option(writer, " no_hardware_cursors", &cursor.no_hardware_cursors.to_string(), None, comment_style)?;
    write_option(writer, " no_break_fs_vrr", &cursor.no_break_fs_vrr.to_string(), None, comment_style)?;
    write_option(writer, " min_refresh_rate", &cursor.min_refresh_rate.to_string(), None, comment_style)?;
    write_option(writer, " hotspot_padding", &cursor.hotspot_padding.to_string(), None, comment_style)?;
    write_option(writer, " inactive_timeout", &cursor.inactive_timeout.to_string(), None, comment_style)?;
    write_boolean_option(writer, " no_warps", cursor.no_warps, None, comment_style)?;
    write_boolean_option(writer, " persistent_warps", cursor.persistent_warps, None, comment_style)?;
    write_option(writer, " warp_on_change_workspace", &cursor.warp_on_change_workspace.to_string(), None, comment_style)?;
    write_option(writer, " warp_on_toggle_special", &cursor.warp_on_toggle_special.to_string(), None, comment_style)?;
    write_option(writer, " default_monitor", &cursor.default_monitor, None, comment_style)?;
    write_option(writer, " zoom_factor", &cursor.zoom_factor.to_string(), None, comment_style)?;
    write_boolean_option(writer, " zoom_rigid", cursor.zoom_rigid, None, comment_style)?;
    write_boolean_option(writer, " enable_hyprcursor", cursor.enable_hyprcursor, None, comment_style)?;
    write_boolean_option(writer, " hide_on_key_press", cursor.hide_on_key_press, None, comment_style)?;
    write_boolean_option(writer, " hide_on_touch", cursor.hide_on_touch, None, comment_style)?;
    write_option(writer, " use_cpu_buffer", &cursor.use_cpu_buffer.to_string(), None, comment_style)?;
    write_boolean_option(writer, " warp_back_after_non_mouse_input", cursor.warp_back_after_non_mouse_input, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}