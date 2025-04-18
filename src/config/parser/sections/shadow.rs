use crate::config::models::ShadowSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_shadow_section(section: &mut ShadowSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "enabled" => section.enabled = parse_bool(value),
                "range" => section.range = value.parse().unwrap_or(4),
                "render_power" => section.render_power = value.parse().unwrap_or(3),
                "sharp" => section.sharp = parse_bool(value),
                "ignore_window" => section.ignore_window = parse_bool(value),
                "color" => section.color = value.to_string(),
                "color_inactive" => section.color_inactive = value.to_string(),
                "scale" => section.scale = value.parse().unwrap_or(1.0),
                "offset" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.offset = (x, y);
                    }
                },
                _ => debug!("Unknown shadow setting: {}", key),
            }
        }
    }
    
    Ok(())
}