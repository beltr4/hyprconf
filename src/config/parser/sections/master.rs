use crate::config::models::MasterSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_master_section(section: &mut MasterSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "allow_small_split" => section.allow_small_split = parse_bool(value),
                "special_scale_factor" => section.special_scale_factor = value.parse().unwrap_or(0.8),
                "mfact" => section.mfact = value.parse().unwrap_or(0.55),
                "new_status" => section.new_status = value.to_string(),
                "new_on_top" => section.new_on_top = parse_bool(value),
                "new_on_active" => section.new_on_active = value.to_string(),
                "orientation" => section.orientation = value.to_string(),
                "inherit_fullscreen" => section.inherit_fullscreen = parse_bool(value),
                "slave_count_for_center_master" => section.slave_count_for_center_master = value.parse().unwrap_or(2),
                "center_master_slaves_on_right" => section.center_master_slaves_on_right = parse_bool(value),
                "smart_resizing" => section.smart_resizing = parse_bool(value),
                "drop_at_cursor" => section.drop_at_cursor = parse_bool(value),
                "always_keep_position" => section.always_keep_position = parse_bool(value),
                _ => debug!("Unknown master setting: {}", key),
            }
        }
    }
    
    Ok(())
}