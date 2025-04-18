use crate::config::models::DeviceConfig;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_device_config(content: &str) -> Result<DeviceConfig> {
    let mut device = DeviceConfig::default();
    
    let parts: Vec<&str> = content.split(',').collect();
    if parts.is_empty() {
        return Ok(device); // No valid device config
    }
    
    // First part is the device name
    device.name = parts[0].trim().to_string();
    
    // Parse optional properties
    for i in 1..parts.len() {
        let part = parts[i].trim();
        
        if part.starts_with("sensitivity:") {
            if let Some(value) = part.strip_prefix("sensitivity:") {
                device.sensitivity = Some(value.trim().parse().unwrap_or(0.0));
            }
        } else if part.starts_with("accel_profile:") {
            if let Some(value) = part.strip_prefix("accel_profile:") {
                device.accel_profile = Some(value.trim().to_string());
            }
        } else if part.starts_with("kb_layout:") {
            if let Some(value) = part.strip_prefix("kb_layout:") {
                device.kb_layout = Some(value.trim().to_string());
            }
        } else if part.starts_with("kb_variant:") {
            if let Some(value) = part.strip_prefix("kb_variant:") {
                device.kb_variant = Some(value.trim().to_string());
            }
        } else if part.starts_with("kb_options:") {
            if let Some(value) = part.strip_prefix("kb_options:") {
                device.kb_options = Some(value.trim().to_string());
            }
        } else if part.starts_with("kb_rules:") {
            if let Some(value) = part.strip_prefix("kb_rules:") {
                device.kb_rules = Some(value.trim().to_string());
            }
        } else if part.starts_with("kb_model:") {
            if let Some(value) = part.strip_prefix("kb_model:") {
                device.kb_model = Some(value.trim().to_string());
            }
        } else if part.starts_with("repeat_rate:") {
            if let Some(value) = part.strip_prefix("repeat_rate:") {
                device.repeat_rate = Some(value.trim().parse().unwrap_or(25));
            }
        } else if part.starts_with("repeat_delay:") {
            if let Some(value) = part.strip_prefix("repeat_delay:") {
                device.repeat_delay = Some(value.trim().parse().unwrap_or(600));
            }
        } else if part.starts_with("natural_scroll:") {
            if let Some(value) = part.strip_prefix("natural_scroll:") {
                device.natural_scroll = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("tap_button_map:") {
            if let Some(value) = part.strip_prefix("tap_button_map:") {
                device.tap_button_map = Some(value.trim().to_string());
            }
        } else if part.starts_with("tap_to_click:") {
            if let Some(value) = part.strip_prefix("tap_to_click:") {
                device.tap_to_click = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("middle_button_emulation:") {
            if let Some(value) = part.strip_prefix("middle_button_emulation:") {
                device.middle_button_emulation = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("clickfinger_behavior:") {
            if let Some(value) = part.strip_prefix("clickfinger_behavior:") {
                device.clickfinger_behavior = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("drag_lock:") {
            if let Some(value) = part.strip_prefix("drag_lock:") {
                device.drag_lock = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("tap_and_drag:") {
            if let Some(value) = part.strip_prefix("tap_and_drag:") {
                device.tap_and_drag = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("left_handed:") {
            if let Some(value) = part.strip_prefix("left_handed:") {
                device.left_handed = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("scroll_method:") {
            if let Some(value) = part.strip_prefix("scroll_method:") {
                device.scroll_method = Some(value.trim().to_string());
            }
        } else if part.starts_with("scroll_button:") {
            if let Some(value) = part.strip_prefix("scroll_button:") {
                device.scroll_button = Some(value.trim().parse().unwrap_or(0));
            }
        } else if part.starts_with("transform:") {
            if let Some(value) = part.strip_prefix("transform:") {
                device.transform = Some(value.trim().parse().unwrap_or(0));
            }
        } else if part.starts_with("output:") {
            if let Some(value) = part.strip_prefix("output:") {
                device.output = Some(value.trim().to_string());
            }
        } else if part.starts_with("enabled:") {
            if let Some(value) = part.strip_prefix("enabled:") {
                device.enabled = Some(parse_bool(value.trim()));
            }
        } else if part.starts_with("keybinds:") {
            if let Some(value) = part.strip_prefix("keybinds:") {
                device.keybinds = Some(parse_bool(value.trim()));
            }
        }
    }
    
    Ok(device)
}