use crate::config::models::SnapSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_snap_section(section: &mut SnapSection, content: &str) -> Result<()> {
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
                "window_gap" => section.window_gap = value.parse().unwrap_or(10),
                "monitor_gap" => section.monitor_gap = value.parse().unwrap_or(10),
                "border_overlap" => section.border_overlap = parse_bool(value),
                _ => debug!("Unknown snap setting: {}", key),
            }
        }
    }
    
    Ok(())
}