use crate::config::models::GeneralSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_general_section(section: &mut GeneralSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Skip nested sections, they'll be handled separately
        if line.contains('{') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "border_size" => section.border_size = value.parse().unwrap_or(1),
                "no_border_on_floating" => section.no_border_on_floating = parse_bool(value),
                "gaps_in" => section.gaps_in = value.to_string(),
                "gaps_out" => section.gaps_out = value.to_string(),
                "gaps_workspaces" => section.gaps_workspaces = value.parse().unwrap_or(0),
                "col.inactive_border" => section.col_inactive_border = value.to_string(),
                "col.active_border" => section.col_active_border = value.to_string(),
                "col.nogroup_border" => section.col_nogroup_border = value.to_string(),
                "col.nogroup_border_active" => section.col_nogroup_border_active = value.to_string(),
                "layout" => section.layout = value.to_string(),
                "no_focus_fallback" => section.no_focus_fallback = parse_bool(value),
                "resize_on_border" => section.resize_on_border = parse_bool(value),
                "extend_border_grab_area" => section.extend_border_grab_area = value.parse().unwrap_or(15),
                "hover_icon_on_border" => section.hover_icon_on_border = parse_bool(value),
                "allow_tearing" => section.allow_tearing = parse_bool(value),
                "resize_corner" => section.resize_corner = value.parse().unwrap_or(0),
                _ => debug!("Unknown general setting: {}", key),
            }
        }
    }
    
    Ok(())
}