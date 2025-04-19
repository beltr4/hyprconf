use std::io::{self, Write};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::config::utils::{CommentStyle, parse_bool};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceConfig {
    pub name: String,
    pub sensitivity: Option<f32>,
    pub accel_profile: Option<String>,
    pub kb_layout: Option<String>,
    pub kb_model: Option<String>,
    pub kb_options: Option<String>,
    pub kb_rules: Option<String>,
    pub kb_variant: Option<String>,
    pub repeat_delay: Option<i32>,
    pub repeat_rate: Option<i32>,
    pub natural_scroll: Option<bool>,
    pub tap_and_drag: Option<bool>,
    pub tap_button_map: Option<String>,
    pub tap_to_click: Option<bool>,
    pub middle_button_emulation: Option<bool>,
    pub clickfinger_behavior: Option<bool>,
    pub drag_lock: Option<bool>,
    pub left_handed: Option<bool>,
    pub scroll_button: Option<i32>,
    pub scroll_method: Option<String>,
    pub transform: Option<i32>,
    pub output: Option<String>,
    pub enabled: Option<bool>,
    pub keybinds: Option<bool>,
}

pub fn parse_device_config(content: &str) -> Result<DeviceConfig> {
    let mut device = DeviceConfig::default();
    let parts: Vec<&str> = content.split(',').collect();
    if parts.is_empty() {
        return Ok(device);
    }
    device.name = parts[0].trim().to_string();
    for part in &parts[1..] {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("accel_profile:") {
            device.accel_profile = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("clickfinger_behavior:") {
            device.clickfinger_behavior = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("drag_lock:") {
            device.drag_lock = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("enabled:") {
            device.enabled = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("kb_layout:") {
            device.kb_layout = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("kb_model:") {
            device.kb_model = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("kb_options:") {
            device.kb_options = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("kb_rules:") {
            device.kb_rules = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("kb_variant:") {
            device.kb_variant = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("keybinds:") {
            device.keybinds = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("left_handed:") {
            device.left_handed = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("middle_button_emulation:") {
            device.middle_button_emulation = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("natural_scroll:") {
            device.natural_scroll = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("output:") {
            device.output = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("repeat_delay:") {
            device.repeat_delay = Some(value.trim().parse().unwrap_or(600));
        } else if let Some(value) = part.strip_prefix("repeat_rate:") {
            device.repeat_rate = Some(value.trim().parse().unwrap_or(25));
        } else if let Some(value) = part.strip_prefix("scroll_button:") {
            device.scroll_button = Some(value.trim().parse().unwrap_or(0));
        } else if let Some(value) = part.strip_prefix("scroll_method:") {
            device.scroll_method = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("sensitivity:") {
            device.sensitivity = Some(value.trim().parse().unwrap_or(0.0));
        } else if let Some(value) = part.strip_prefix("tap_and_drag:") {
            device.tap_and_drag = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("tap_button_map:") {
            device.tap_button_map = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("tap_to_click:") {
            device.tap_to_click = Some(parse_bool(value.trim()));
        } else if let Some(value) = part.strip_prefix("transform:") {
            device.transform = Some(value.trim().parse().unwrap_or(0));
        }
    }
    Ok(device)
}

/// Write a device configuration to the provided writer
pub fn write_device<W: Write>(
    writer: &mut W,
    device: &DeviceConfig,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Start with the device name
    let mut device_str = format!("device = {}", device.name);
    
    // Add optional parameters with appropriate prefixes
    if let Some(sensitivity) = device.sensitivity {
        device_str.push_str(&format!(",sensitivity:{}", sensitivity));
    }
    
    if let Some(accel_profile) = &device.accel_profile {
        device_str.push_str(&format!(",accel_profile:{}", accel_profile));
    }
    
    if let Some(kb_layout) = &device.kb_layout {
        device_str.push_str(&format!(",kb_layout:{}", kb_layout));
    }
    
    if let Some(kb_model) = &device.kb_model {
        device_str.push_str(&format!(",kb_model:{}", kb_model));
    }
    
    if let Some(kb_options) = &device.kb_options {
        device_str.push_str(&format!(",kb_options:{}", kb_options));
    }
    
    if let Some(kb_rules) = &device.kb_rules {
        device_str.push_str(&format!(",kb_rules:{}", kb_rules));
    }
    
    if let Some(kb_variant) = &device.kb_variant {
        device_str.push_str(&format!(",kb_variant:{}", kb_variant));
    }
    
    if let Some(repeat_delay) = device.repeat_delay {
        device_str.push_str(&format!(",repeat_delay:{}", repeat_delay));
    }
    
    if let Some(repeat_rate) = device.repeat_rate {
        device_str.push_str(&format!(",repeat_rate:{}", repeat_rate));
    }
    
    if let Some(natural_scroll) = device.natural_scroll {
        device_str.push_str(&format!(",natural_scroll:{}", natural_scroll));
    }
    
    if let Some(tap_and_drag) = device.tap_and_drag {
        device_str.push_str(&format!(",tap_and_drag:{}", tap_and_drag));
    }
    
    if let Some(tap_button_map) = &device.tap_button_map {
        device_str.push_str(&format!(",tap_button_map:{}", tap_button_map));
    }
    
    if let Some(tap_to_click) = device.tap_to_click {
        device_str.push_str(&format!(",tap_to_click:{}", tap_to_click));
    }
    
    if let Some(middle_button_emulation) = device.middle_button_emulation {
        device_str.push_str(&format!(",middle_button_emulation:{}", middle_button_emulation));
    }
    
    if let Some(clickfinger_behavior) = device.clickfinger_behavior {
        device_str.push_str(&format!(",clickfinger_behavior:{}", clickfinger_behavior));
    }
    
    if let Some(drag_lock) = device.drag_lock {
        device_str.push_str(&format!(",drag_lock:{}", drag_lock));
    }
    
    if let Some(left_handed) = device.left_handed {
        device_str.push_str(&format!(",left_handed:{}", left_handed));
    }
    
    if let Some(scroll_button) = device.scroll_button {
        device_str.push_str(&format!(",scroll_button:{}", scroll_button));
    }
    
    if let Some(scroll_method) = &device.scroll_method {
        device_str.push_str(&format!(",scroll_method:{}", scroll_method));
    }
    
    if let Some(transform) = device.transform {
        device_str.push_str(&format!(",transform:{}", transform));
    }
    
    if let Some(output) = &device.output {
        device_str.push_str(&format!(",output:{}", output));
    }
    
    if let Some(enabled) = device.enabled {
        device_str.push_str(&format!(",enabled:{}", enabled));
    }
    
    if let Some(keybinds) = device.keybinds {
        device_str.push_str(&format!(",keybinds:{}", keybinds));
    }
    
    // Write the completed device string
    writeln!(writer, "{}", device_str)?;
    
    Ok(())
}

/// Implement ConfigSection for Vec<DeviceConfig>
impl crate::config::models::core::ConfigSection for Vec<DeviceConfig> {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for device in self {
            write_device(writer, device, comment_style)?;
        }
        Ok(())
    }
}