use crate::config::models::DwindleSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_dwindle_section(section: &mut DwindleSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "pseudotile" => section.pseudotile = parse_bool(value),
                "preserve_split" => section.preserve_split = parse_bool(value),
                "smart_split" => section.smart_split = parse_bool(value),
                "force_split" => section.force_split = value.parse().unwrap_or(0),
                "permanent_direction_override" => section.permanent_direction_override = parse_bool(value),
                "special_scale_factor" => section.special_scale_factor = value.parse().unwrap_or(0.8),
                "split_width_multiplier" => section.split_width_multiplier = value.parse().unwrap_or(1.0),
                "use_active_for_splits" => section.use_active_for_splits = parse_bool(value),
                "default_split_ratio" => section.default_split_ratio = value.parse().unwrap_or(1.0),
                "split_bias" => section.split_bias = value.parse().unwrap_or(0),
                "smart_resizing" => section.smart_resizing = parse_bool(value),
                _ => debug!("Unknown dwindle setting: {}", key),
            }
        }
    }
    
    Ok(())
}