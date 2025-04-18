use crate::config::models::MonitorConfig;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_monitor_config(content: &str) -> Result<MonitorConfig> {
    let mut monitor = MonitorConfig::default();
    
    let parts: Vec<&str> = content.split(',').collect();
    if parts.len() < 3 {
        return Ok(monitor); // Not enough parts for a valid monitor config
    }
    
    // Parse basic monitor properties
    monitor.name = parts[0].trim().to_string();
    monitor.resolution = parts[1].trim().to_string();
    monitor.position = parts[2].trim().to_string();
    
    // Parse optional properties
    for i in 3..parts.len() {
        let part = parts[i].trim();
        
        if part.starts_with("transform:") {
            if let Some(value) = part.strip_prefix("transform:") {
                monitor.transform = Some(value.trim().parse().unwrap_or(0));
            }
        } else if part.starts_with("scale:") {
            if let Some(value) = part.strip_prefix("scale:") {
                monitor.scale = value.trim().parse().unwrap_or(1.0);
            }
        } else if part.starts_with("mirror:") {
            if let Some(value) = part.strip_prefix("mirror:") {
                monitor.mirror = Some(value.trim().to_string());
            }
        } else if part.starts_with("bitdepth:") {
            if let Some(value) = part.strip_prefix("bitdepth:") {
                monitor.bitdepth = Some(value.trim().parse().unwrap_or(8));
            }
        } else if part.starts_with("color_management:") {
            if let Some(value) = part.strip_prefix("color_management:") {
                monitor.color_management = Some(value.trim().to_string());
            }
        } else if part.starts_with("sdr_brightness:") {
            if let Some(value) = part.strip_prefix("sdr_brightness:") {
                monitor.sdr_brightness = Some(value.trim().parse().unwrap_or(1.0));
            }
        } else if part.starts_with("sdr_saturation:") {
            if let Some(value) = part.strip_prefix("sdr_saturation:") {
                monitor.sdr_saturation = Some(value.trim().parse().unwrap_or(1.0));
            }
        } else if part.starts_with("vrr:") {
            if let Some(value) = part.strip_prefix("vrr:") {
                monitor.vrr = Some(value.trim().parse().unwrap_or(0));
            }
        } else if part == "disable" {
            monitor.disable = true;
        } else if part.starts_with("reserved_area:") {
            if let Some(value) = part.strip_prefix("reserved_area:") {
                let area_parts: Vec<&str> = value.trim().split_whitespace().collect();
                if area_parts.len() == 4 {
                    let top = area_parts[0].parse().unwrap_or(0);
                    let bottom = area_parts[1].parse().unwrap_or(0);
                    let left = area_parts[2].parse().unwrap_or(0);
                    let right = area_parts[3].parse().unwrap_or(0);
                    monitor.reserved_area = Some((top, bottom, left, right));
                }
            }
        }
    }
    
    Ok(monitor)
}