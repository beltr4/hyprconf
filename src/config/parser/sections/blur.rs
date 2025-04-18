use crate::config::models::BlurSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_blur_section(section: &mut BlurSection, content: &str) -> Result<()> {
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
                "size" => section.size = value.parse().unwrap_or(8),
                "passes" => section.passes = value.parse().unwrap_or(1),
                "ignore_opacity" => section.ignore_opacity = parse_bool(value),
                "new_optimizations" => section.new_optimizations = parse_bool(value),
                "xray" => section.xray = parse_bool(value),
                "noise" => section.noise = value.parse().unwrap_or(0.0117),
                "contrast" => section.contrast = value.parse().unwrap_or(0.8916),
                "brightness" => section.brightness = value.parse().unwrap_or(0.8172),
                "vibrancy" => section.vibrancy = value.parse().unwrap_or(0.1696),
                "vibrancy_darkness" => section.vibrancy_darkness = value.parse().unwrap_or(0.0),
                "special" => section.special = parse_bool(value),
                "popups" => section.popups = parse_bool(value),
                "popups_ignorealpha" => section.popups_ignorealpha = value.parse().unwrap_or(0.2),
                "input_methods" => section.input_methods = parse_bool(value),
                "input_methods_ignorealpha" => section.input_methods_ignorealpha = value.parse().unwrap_or(0.2),
                _ => debug!("Unknown blur setting: {}", key),
            }
        }
    }
    
    Ok(())
}