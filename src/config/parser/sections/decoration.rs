use crate::config::models::DecorationSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_decoration_section(section: &mut DecorationSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line.contains('{') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "rounding" => section.rounding = value.parse().unwrap_or(0),
                "rounding_power" => section.rounding_power = value.parse().unwrap_or(2.0),
                "active_opacity" => section.active_opacity = value.parse().unwrap_or(1.0),
                "inactive_opacity" => section.inactive_opacity = value.parse().unwrap_or(1.0),
                "fullscreen_opacity" => section.fullscreen_opacity = value.parse().unwrap_or(1.0),
                "dim_inactive" => section.dim_inactive = parse_bool(value),
                "dim_strength" => section.dim_strength = value.parse().unwrap_or(0.5),
                "dim_special" => section.dim_special = value.parse().unwrap_or(0.2),
                "dim_around" => section.dim_around = value.parse().unwrap_or(0.4),
                "screen_shader" => section.screen_shader = value.to_string(),
                "border_part_of_window" => section.border_part_of_window = parse_bool(value),
                _ => debug!("Unknown decoration setting: {}", key),
            }
        }
    }
    
    Ok(())
}