use crate::config::models::GesturesSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

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