use crate::config::models::GroupSection;
use crate::config::models::GroupbarSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_group_section(section: &mut GroupSection, content: &str) -> Result<()> {
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
                "auto_group" => section.auto_group = parse_bool(value),
                "insert_after_current" => section.insert_after_current = parse_bool(value),
                "focus_removed_window" => section.focus_removed_window = parse_bool(value),
                "drag_into_group" => section.drag_into_group = value.parse().unwrap_or(1),
                "merge_groups_on_drag" => section.merge_groups_on_drag = parse_bool(value),
                "merge_groups_on_groupbar" => section.merge_groups_on_groupbar = parse_bool(value),
                "merge_floated_into_tiled_on_groupbar" => section.merge_floated_into_tiled_on_groupbar = parse_bool(value),
                "group_on_movetoworkspace" => section.group_on_movetoworkspace = parse_bool(value),
                "col.border_active" => section.col_border_active = value.to_string(),
                "col.border_inactive" => section.col_border_inactive = value.to_string(),
                "col.border_locked_active" => section.col_border_locked_active = value.to_string(),
                "col.border_locked_inactive" => section.col_border_locked_inactive = value.to_string(),
                _ => debug!("Unknown group setting: {}", key),
            }
        }
    }
    
    Ok(())
}


pub fn parse_groupbar_section(section: &mut GroupbarSection, content: &str) -> Result<()> {
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
                "font_family" => section.font_family = value.to_string(),
                "font_size" => section.font_size = value.parse().unwrap_or(8),
                "gradients" => section.gradients = parse_bool(value),
                "height" => section.height = value.parse().unwrap_or(14),
                "indicator_height" => section.indicator_height = value.parse().unwrap_or(3),
                "stacked" => section.stacked = parse_bool(value),
                "priority" => section.priority = value.parse().unwrap_or(3),
                "render_titles" => section.render_titles = parse_bool(value),
                "text_offset" => section.text_offset = value.parse().unwrap_or(0),
                "scrolling" => section.scrolling = parse_bool(value),
                "rounding" => section.rounding = value.parse().unwrap_or(1),
                "gradient_rounding" => section.gradient_rounding = value.parse().unwrap_or(2),
                "round_only_edges" => section.round_only_edges = parse_bool(value),
                "gradient_round_only_edges" => section.gradient_round_only_edges = parse_bool(value),
                "text_color" => section.text_color = value.to_string(),
                "col.active" => section.col_active = value.to_string(),
                "col.inactive" => section.col_inactive = value.to_string(),
                "col.locked_active" => section.col_locked_active = value.to_string(),
                "col.locked_inactive" => section.col_locked_inactive = value.to_string(),
                "gaps_in" => section.gaps_in = value.parse().unwrap_or(2),
                "gaps_out" => section.gaps_out = value.parse().unwrap_or(2),
                "keep_upper_gap" => section.keep_upper_gap = parse_bool(value),
                _ => debug!("Unknown groupbar setting: {}", key),
            }
        }
    }
    
    Ok(())
}
