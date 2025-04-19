use std::io::{self, Write};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::config::utils::CommentStyle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    pub name: String,
    pub resolution: String,
    pub position: String,
    pub scale: f32,
    pub transform: Option<i32>,
    pub mirror: Option<String>,
    pub bitdepth: Option<i32>,
    pub color_management: Option<String>,
    pub sdr_brightness: Option<f32>,
    pub sdr_saturation: Option<f32>,
    pub vrr: Option<i32>,
    pub disable: bool,
    pub reserved_area: Option<(i32, i32, i32, i32)>,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            name: String::default(),
            resolution: String::default(),
            position: String::default(),
            scale: 1.0,
            transform: None,
            mirror: None,
            bitdepth: None,
            color_management: None,
            sdr_brightness: None,
            sdr_saturation: None,
            vrr: None,
            disable: false,
            reserved_area: None,
        }
    }
}

/// Parse a monitor configuration string
pub fn parse_monitor_config(content: &str) -> Result<MonitorConfig> {
    let mut monitor = MonitorConfig::default();
    let parts: Vec<&str> = content.split(',').collect();
    if parts.len() < 3 {
        return Ok(monitor);
    }
    monitor.name = parts[0].trim().to_string();
    monitor.resolution = parts[1].trim().to_string();
    monitor.position = parts[2].trim().to_string();
    for part in &parts[3..] {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("transform:") {
            monitor.transform = Some(value.trim().parse().unwrap_or(0));
        } else if let Some(value) = part.strip_prefix("scale:") {
            monitor.scale = value.trim().parse().unwrap_or(1.0);
        } else if let Some(value) = part.strip_prefix("mirror:") {
            monitor.mirror = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("bitdepth:") {
            monitor.bitdepth = Some(value.trim().parse().unwrap_or(8));
        } else if let Some(value) = part.strip_prefix("color_management:") {
            monitor.color_management = Some(value.trim().to_string());
        } else if let Some(value) = part.strip_prefix("sdr_brightness:") {
            monitor.sdr_brightness = Some(value.trim().parse().unwrap_or(1.0));
        } else if let Some(value) = part.strip_prefix("sdr_saturation:") {
            monitor.sdr_saturation = Some(value.trim().parse().unwrap_or(1.0));
        } else if let Some(value) = part.strip_prefix("vrr:") {
            monitor.vrr = Some(value.trim().parse().unwrap_or(0));
        } else if part == "disable" {
            monitor.disable = true;
        } else if let Some(value) = part.strip_prefix("reserved_area:") {
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
    Ok(monitor)
}

/// Write a monitor configuration to the writer
pub fn write_monitor<W: Write>(
    writer: &mut W,
    monitor: &MonitorConfig,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Start building the monitor configuration string
    let mut monitor_config = format!(
        "monitor = {},{},{}",
        monitor.name,
        monitor.resolution,
        monitor.position
    );

    // Add scale
    monitor_config.push_str(&format!(",{}", monitor.scale));

    // Add optional parameters
    if let Some(transform) = monitor.transform {
        monitor_config.push_str(&format!(",transform:{}", transform));
    }

    if let Some(mirror) = &monitor.mirror {
        monitor_config.push_str(&format!(",mirror:{}", mirror));
    }

    if let Some(bitdepth) = monitor.bitdepth {
        monitor_config.push_str(&format!(",bitdepth:{}", bitdepth));
    }

    if let Some(color_management) = &monitor.color_management {
        monitor_config.push_str(&format!(",color_management:{}", color_management));
    }

    if let Some(sdr_brightness) = monitor.sdr_brightness {
        monitor_config.push_str(&format!(",sdr_brightness:{}", sdr_brightness));
    }

    if let Some(sdr_saturation) = monitor.sdr_saturation {
        monitor_config.push_str(&format!(",sdr_saturation:{}", sdr_saturation));
    }

    if let Some(vrr) = monitor.vrr {
        monitor_config.push_str(&format!(",vrr:{}", vrr));
    }

    if monitor.disable {
        monitor_config.push_str(",disable");
    }

    if let Some((top, bottom, left, right)) = monitor.reserved_area {
        monitor_config.push_str(&format!(",reserved_area:{} {} {} {}", top, bottom, left, right));
    }

    // Simply write the line without any comment handling for now
    writeln!(writer, "{}", monitor_config)?;

    Ok(())
}

/// Implements the ConfigSection trait for a collection of monitors
impl crate::config::models::core::ConfigSection for Vec<MonitorConfig> {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for monitor in self {
            write_monitor(writer, monitor, comment_style)?;
        }
        Ok(())
    }
}