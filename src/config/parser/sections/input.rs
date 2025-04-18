use crate::config::models::InputSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_input_section(section: &mut InputSection, content: &str) -> Result<()> {
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
                "kb_model" => section.kb_model = value.to_string(),
                "kb_layout" => section.kb_layout = value.to_string(),
                "kb_variant" => section.kb_variant = value.to_string(),
                "kb_options" => section.kb_options = value.to_string(),
                "kb_rules" => section.kb_rules = value.to_string(),
                "kb_file" => section.kb_file = value.to_string(),
                "numlock_by_default" => section.numlock_by_default = parse_bool(value),
                "resolve_binds_by_sym" => section.resolve_binds_by_sym = parse_bool(value),
                "repeat_rate" => section.repeat_rate = value.parse().unwrap_or(25),
                "repeat_delay" => section.repeat_delay = value.parse().unwrap_or(600),
                "sensitivity" => section.sensitivity = value.parse().unwrap_or(0.0),
                "accel_profile" => section.accel_profile = value.to_string(),
                "force_no_accel" => section.force_no_accel = parse_bool(value),
                "left_handed" => section.left_handed = parse_bool(value),
                "scroll_points" => section.scroll_points = value.to_string(),
                "scroll_method" => section.scroll_method = value.to_string(),
                "scroll_button" => section.scroll_button = value.parse().unwrap_or(0),
                "scroll_button_lock" => section.scroll_button_lock = parse_bool(value),
                "scroll_factor" => section.scroll_factor = value.parse().unwrap_or(1.0),
                "natural_scroll" => section.natural_scroll = parse_bool(value),
                "follow_mouse" => section.follow_mouse = value.parse().unwrap_or(1),
                "follow_mouse_threshold" => section.follow_mouse_threshold = value.parse().unwrap_or(0.0),
                "focus_on_close" => section.focus_on_close = value.parse().unwrap_or(0),
                "mouse_refocus" => section.mouse_refocus = parse_bool(value),
                "float_switch_override_focus" => section.float_switch_override_focus = value.parse().unwrap_or(1),
                "special_fallthrough" => section.special_fallthrough = parse_bool(value),
                "off_window_axis_events" => section.off_window_axis_events = value.parse().unwrap_or(1),
                "emulate_discrete_scroll" => section.emulate_discrete_scroll = value.parse().unwrap_or(1),
                "drag_threshold" => section.drag_threshold = value.parse().unwrap_or(0),
                _ => debug!("Unknown input setting: {}", key),
            }
        }
    }
    
    Ok(())
}