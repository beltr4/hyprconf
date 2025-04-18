use crate::config::models::TouchpadSection;
use crate::config::models::TouchdeviceSection;
use crate::config::models::TabletSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_touchpad_section(section: &mut TouchpadSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "disable_while_typing" => section.disable_while_typing = parse_bool(value),
                "natural_scroll" => section.natural_scroll = parse_bool(value),
                "scroll_factor" => section.scroll_factor = value.parse().unwrap_or(1.0),
                "middle_button_emulation" => section.middle_button_emulation = parse_bool(value),
                "tap_button_map" => section.tap_button_map = value.to_string(),
                "clickfinger_behavior" => section.clickfinger_behavior = parse_bool(value),
                "tap-to-click" => section.tap_to_click = parse_bool(value),
                "drag_lock" => section.drag_lock = parse_bool(value),
                "tap-and-drag" => section.tap_and_drag = parse_bool(value),
                "flip_x" => section.flip_x = parse_bool(value),
                "flip_y" => section.flip_y = parse_bool(value),
                _ => debug!("Unknown touchpad setting: {}", key),
            }
        }
    }
    
    Ok(())
}

/// Parse the touchdevice section
pub fn parse_touchdevice_section(section: &mut TouchdeviceSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "transform" => section.transform = value.parse().unwrap_or(-1),
                "output" => section.output = value.to_string(),
                "enabled" => section.enabled = parse_bool(value),
                _ => debug!("Unknown touchdevice setting: {}", key),
            }
        }
    }
    
    Ok(())
}

/// Parse the tablet section
pub fn parse_tablet_section(section: &mut TabletSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "transform" => section.transform = value.parse().unwrap_or(-1),
                "output" => section.output = value.to_string(),
                "relative_input" => section.relative_input = parse_bool(value),
                "left_handed" => section.left_handed = parse_bool(value),
                "absolute_region_position" => section.absolute_region_position = parse_bool(value),
                "region_position" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.region_position = (x, y);
                    }
                },
                "region_size" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.region_size = (x, y);
                    }
                },
                "active_area_size" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.active_area_size = (x, y);
                    }
                },
                "active_area_position" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.active_area_position = (x, y);
                    }
                },
                _ => debug!("Unknown tablet setting: {}", key),
            }
        }
    }
    
    Ok(())
}
