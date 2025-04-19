use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


/// Input section with touchpad, touchdevice, and tablet subcategories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InputSection {
    pub kb_model: String,
    pub kb_layout: String,
    pub kb_variant: String,
    pub kb_options: String,
    pub kb_rules: String,
    pub kb_file: String,
    pub numlock_by_default: bool,
    pub resolve_binds_by_sym: bool,
    pub repeat_rate: i32,
    pub repeat_delay: i32,
    pub sensitivity: f32,
    pub accel_profile: String,
    pub force_no_accel: bool,
    pub left_handed: bool,
    pub scroll_points: String,
    pub scroll_method: String,
    pub scroll_button: i32,
    pub scroll_button_lock: bool,
    pub scroll_factor: f32,
    pub natural_scroll: bool,
    pub follow_mouse: i32,
    pub follow_mouse_threshold: f32,
    pub focus_on_close: i32,
    pub mouse_refocus: bool,
    pub float_switch_override_focus: i32,
    pub special_fallthrough: bool,
    pub off_window_axis_events: i32,
    pub emulate_discrete_scroll: i32,
    pub drag_threshold: i32,
    pub touchpad: TouchpadSection,
    pub touchdevice: TouchdeviceSection,
    pub tablet: TabletSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchpadSection {
    pub disable_while_typing: bool,
    pub natural_scroll: bool,
    pub scroll_factor: f32,
    pub middle_button_emulation: bool,
    pub tap_button_map: String,
    pub clickfinger_behavior: bool,
    pub tap_to_click: bool,
    pub drag_lock: bool,
    pub tap_and_drag: bool,
    pub flip_x: bool,
    pub flip_y: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchdeviceSection {
    pub transform: i32,
    pub output: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TabletSection {
    pub transform: i32,
    pub output: String,
    pub region_position: (f32, f32),
    pub absolute_region_position: bool,
    pub region_size: (f32, f32),
    pub relative_input: bool,
    pub left_handed: bool,
    pub active_area_size: (f32, f32),
    pub active_area_position: (f32, f32),
}

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

pub fn parse_touchpad_section(section: &mut TouchpadSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "disable_while_typing" => section.disable_while_typing = parse_bool(value),
                "natural_scroll" => section.natural_scroll = parse_bool(value),
                "scroll_factor" => section.scroll_factor = value.parse().unwrap_or(1.0),
                "middle_button_emulation" => section.middle_button_emulation = parse_bool(value),
                "tap_button_map" => section.tap_button_map = value.to_string(),
                "clickfinger_behavior" => section.clickfinger_behavior = parse_bool(value),
                "tap-to-click" | "tap_to_click" => section.tap_to_click = parse_bool(value),
                "drag_lock" => section.drag_lock = parse_bool(value),
                "tap-and-drag" | "tap_and_drag" => section.tap_and_drag = parse_bool(value),
                "flip_x" => section.flip_x = parse_bool(value),
                "flip_y" => section.flip_y = parse_bool(value),
                _ => debug!("Unknown touchpad setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn parse_touchdevice_section(section: &mut TouchdeviceSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "transform" => section.transform = value.parse().unwrap_or(-1),
                "output" => section.output = value.to_string(),
                "enabled" => section.enabled = parse_bool(value),
                _ => debug!("Unknown touchdevice setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn parse_tablet_section(section: &mut TabletSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "transform" => section.transform = value.parse().unwrap_or(-1),
                "output" => section.output = value.to_string(),
                "relative_input" => section.relative_input = parse_bool(value),
                "left_handed" => section.left_handed = parse_bool(value),
                "absolute_region_position" => section.absolute_region_position = parse_bool(value),
                "region_position" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.region_position = (x, y);
                    }
                },
                "region_size" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.region_size = (x, y);
                    }
                },
                "active_area_size" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.active_area_size = (x, y);
                    }
                },
                "active_area_position" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.active_area_position = (x, y);
                    }
                },
                _ => debug!("Unknown tablet setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    input: &InputSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "input {", comment_style)?;
    
    write_option(writer, " kb_model", &input.kb_model, None, comment_style)?;
    write_option(writer, " kb_layout", &input.kb_layout, None, comment_style)?;
    write_option(writer, " kb_variant", &input.kb_variant, None, comment_style)?;
    write_option(writer, " kb_options", &input.kb_options, None, comment_style)?;
    write_option(writer, " kb_rules", &input.kb_rules, None, comment_style)?;
    write_option(writer, " kb_file", &input.kb_file, None, comment_style)?;
    write_boolean_option(writer, " numlock_by_default", input.numlock_by_default, None, comment_style)?;
    write_boolean_option(writer, " resolve_binds_by_sym", input.resolve_binds_by_sym, None, comment_style)?;
    write_option(writer, " repeat_rate", &input.repeat_rate.to_string(), None, comment_style)?;
    write_option(writer, " repeat_delay", &input.repeat_delay.to_string(), None, comment_style)?;
    write_option(writer, " sensitivity", &input.sensitivity.to_string(), None, comment_style)?;
    write_option(writer, " accel_profile", &input.accel_profile, None, comment_style)?;
    write_boolean_option(writer, " force_no_accel", input.force_no_accel, None, comment_style)?;
    write_boolean_option(writer, " left_handed", input.left_handed, None, comment_style)?;
    write_option(writer, " scroll_points", &input.scroll_points, None, comment_style)?;
    write_option(writer, " scroll_method", &input.scroll_method, None, comment_style)?;
    write_option(writer, " scroll_button", &input.scroll_button.to_string(), None, comment_style)?;
    write_boolean_option(writer, " scroll_button_lock", input.scroll_button_lock, None, comment_style)?;
    write_option(writer, " scroll_factor", &input.scroll_factor.to_string(), None, comment_style)?;
    write_boolean_option(writer, " natural_scroll", input.natural_scroll, None, comment_style)?;
    write_option(writer, " follow_mouse", &input.follow_mouse.to_string(), None, comment_style)?;
    write_option(writer, " follow_mouse_threshold", &input.follow_mouse_threshold.to_string(), None, comment_style)?;
    write_option(writer, " focus_on_close", &input.focus_on_close.to_string(), None, comment_style)?;
    write_boolean_option(writer, " mouse_refocus", input.mouse_refocus, None, comment_style)?;
    write_option(writer, " float_switch_override_focus", &input.float_switch_override_focus.to_string(), None, comment_style)?;
    write_boolean_option(writer, " special_fallthrough", input.special_fallthrough, None, comment_style)?;
    write_option(writer, " off_window_axis_events", &input.off_window_axis_events.to_string(), None, comment_style)?;
    write_option(writer, " emulate_discrete_scroll", &input.emulate_discrete_scroll.to_string(), None, comment_style)?;
    write_option(writer, " drag_threshold", &input.drag_threshold.to_string(), None, comment_style)?;
    
    // Write touchpad subsection
    writeln!(writer, " touchpad {{")?;
    write_boolean_option(writer, "  disable_while_typing", input.touchpad.disable_while_typing, None, comment_style)?;
    write_boolean_option(writer, "  natural_scroll", input.touchpad.natural_scroll, None, comment_style)?;
    write_option(writer, "  scroll_factor", &input.touchpad.scroll_factor.to_string(), None, comment_style)?;
    write_boolean_option(writer, "  middle_button_emulation", input.touchpad.middle_button_emulation, None, comment_style)?;
    write_option(writer, "  tap_button_map", &input.touchpad.tap_button_map, None, comment_style)?;
    write_boolean_option(writer, "  clickfinger_behavior", input.touchpad.clickfinger_behavior, None, comment_style)?;
    write_boolean_option(writer, "  tap_to_click", input.touchpad.tap_to_click, None, comment_style)?;
    write_boolean_option(writer, "  drag_lock", input.touchpad.drag_lock, None, comment_style)?;
    write_boolean_option(writer, "  tap_and_drag", input.touchpad.tap_and_drag, None, comment_style)?;
    write_boolean_option(writer, "  flip_x", input.touchpad.flip_x, None, comment_style)?;
    write_boolean_option(writer, "  flip_y", input.touchpad.flip_y, None, comment_style)?;
    writeln!(writer, " }}")?;
    
    // Write touchdevice subsection
    writeln!(writer, " touchdevice {{")?;
    write_option(writer, "  transform", &input.touchdevice.transform.to_string(), None, comment_style)?;
    write_option(writer, "  output", &input.touchdevice.output, None, comment_style)?;
    write_boolean_option(writer, "  enabled", input.touchdevice.enabled, None, comment_style)?;
    writeln!(writer, " }}")?;
    
    // Write tablet subsection
    writeln!(writer, " tablet {{")?;
    write_option(writer, "  transform", &input.tablet.transform.to_string(), None, comment_style)?;
    write_option(writer, "  output", &input.tablet.output, None, comment_style)?;
    write_option(writer, "  region_position", &format!("{} {}", input.tablet.region_position.0, input.tablet.region_position.1), None, comment_style)?;
    write_boolean_option(writer, "  absolute_region_position", input.tablet.absolute_region_position, None, comment_style)?;
    write_option(writer, "  region_size", &format!("{} {}", input.tablet.region_size.0, input.tablet.region_size.1), None, comment_style)?;
    write_boolean_option(writer, "  relative_input", input.tablet.relative_input, None, comment_style)?;
    write_boolean_option(writer, "  left_handed", input.tablet.left_handed, None, comment_style)?;
    write_option(writer, "  active_area_size", &format!("{} {}", input.tablet.active_area_size.0, input.tablet.active_area_size.1), None, comment_style)?;
    write_option(writer, "  active_area_position", &format!("{} {}", input.tablet.active_area_position.0, input.tablet.active_area_position.1), None, comment_style)?;
    writeln!(writer, " }}")?;
    
    writeln!(writer, "}}")?;
    Ok(())
}