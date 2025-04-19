use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GesturesSection {
    pub workspace_swipe: bool,
    pub workspace_swipe_fingers: i32,
    pub workspace_swipe_min_fingers: bool,
    pub workspace_swipe_distance: i32,
    pub workspace_swipe_touch: bool,
    pub workspace_swipe_invert: bool,
    pub workspace_swipe_touch_invert: bool,
    pub workspace_swipe_min_speed_to_force: i32,
    pub workspace_swipe_cancel_ratio: f32,
    pub workspace_swipe_create_new: bool,
    pub workspace_swipe_direction_lock: bool,
    pub workspace_swipe_direction_lock_threshold: i32,
    pub workspace_swipe_forever: bool,
    pub workspace_swipe_use_r: bool,
}

pub fn parse_gestures_section(section: &mut GesturesSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "workspace_swipe" => section.workspace_swipe = parse_bool(value),
                "workspace_swipe_fingers" => section.workspace_swipe_fingers = value.parse().unwrap_or(3),
                "workspace_swipe_min_fingers" => section.workspace_swipe_min_fingers = parse_bool(value),
                "workspace_swipe_distance" => section.workspace_swipe_distance = value.parse().unwrap_or(300),
                "workspace_swipe_touch" => section.workspace_swipe_touch = parse_bool(value),
                "workspace_swipe_invert" => section.workspace_swipe_invert = parse_bool(value),
                "workspace_swipe_touch_invert" => section.workspace_swipe_touch_invert = parse_bool(value),
                "workspace_swipe_min_speed_to_force" => section.workspace_swipe_min_speed_to_force = value.parse().unwrap_or(30),
                "workspace_swipe_cancel_ratio" => section.workspace_swipe_cancel_ratio = value.parse().unwrap_or(0.5),
                "workspace_swipe_create_new" => section.workspace_swipe_create_new = parse_bool(value),
                "workspace_swipe_direction_lock" => section.workspace_swipe_direction_lock = parse_bool(value),
                "workspace_swipe_direction_lock_threshold" => section.workspace_swipe_direction_lock_threshold = value.parse().unwrap_or(10),
                "workspace_swipe_forever" => section.workspace_swipe_forever = parse_bool(value),
                "workspace_swipe_use_r" => section.workspace_swipe_use_r = parse_bool(value),
                _ => debug!("Unknown gestures setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    gestures: &GesturesSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "gestures {", comment_style)?;
    
    write_boolean_option(writer, " workspace_swipe", gestures.workspace_swipe, None, comment_style)?;
    write_option(writer, " workspace_swipe_fingers", &gestures.workspace_swipe_fingers.to_string(), None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_min_fingers", gestures.workspace_swipe_min_fingers, None, comment_style)?;
    write_option(writer, " workspace_swipe_distance", &gestures.workspace_swipe_distance.to_string(), None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_touch", gestures.workspace_swipe_touch, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_invert", gestures.workspace_swipe_invert, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_touch_invert", gestures.workspace_swipe_touch_invert, None, comment_style)?;
    write_option(writer, " workspace_swipe_min_speed_to_force", &gestures.workspace_swipe_min_speed_to_force.to_string(), None, comment_style)?;
    write_option(writer, " workspace_swipe_cancel_ratio", &gestures.workspace_swipe_cancel_ratio.to_string(), None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_create_new", gestures.workspace_swipe_create_new, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_direction_lock", gestures.workspace_swipe_direction_lock, None, comment_style)?;
    write_option(writer, " workspace_swipe_direction_lock_threshold", &gestures.workspace_swipe_direction_lock_threshold.to_string(), None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_forever", gestures.workspace_swipe_forever, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_use_r", gestures.workspace_swipe_use_r, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}