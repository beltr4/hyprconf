// src/config/parser/sections/binds.rs
use crate::config::models::BindsSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

/// Parse the binds section
pub fn parse_binds_section(section: &mut BindsSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "pass_mouse_when_bound" => section.pass_mouse_when_bound = parse_bool(value),
                "scroll_event_delay" => section.scroll_event_delay = value.parse().unwrap_or(300),
                "workspace_back_and_forth" => section.workspace_back_and_forth = parse_bool(value),
                "hide_special_on_workspace_change" => section.hide_special_on_workspace_change = parse_bool(value),
                "allow_workspace_cycles" => section.allow_workspace_cycles = parse_bool(value),
                "workspace_center_on" => section.workspace_center_on = value.parse().unwrap_or(0),
                "focus_preferred_method" => section.focus_preferred_method = value.parse().unwrap_or(0),
                "ignore_group_lock" => section.ignore_group_lock = parse_bool(value),
                "movefocus_cycles_fullscreen" => section.movefocus_cycles_fullscreen = parse_bool(value),
                "movefocus_cycles_groupfirst" => section.movefocus_cycles_groupfirst = parse_bool(value),
                "disable_keybind_grabbing" => section.disable_keybind_grabbing = parse_bool(value),
                "window_direction_monitor_fallback" => section.window_direction_monitor_fallback = parse_bool(value),
                "allow_pin_fullscreen" => section.allow_pin_fullscreen = parse_bool(value),
                _ => debug!("Unknown binds setting: {}", key),
            }
        }
    }
    
    Ok(())
}