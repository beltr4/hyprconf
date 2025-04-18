use crate::config::models::XWaylandSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_xwayland_section(section: &mut XWaylandSection, content: &str) -> Result<()> {
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
                "use_nearest_neighbor" => section.use_nearest_neighbor = parse_bool(value),
                "force_zero_scaling" => section.force_zero_scaling = parse_bool(value),
                "create_abstract_socket" => section.create_abstract_socket = parse_bool(value),
                _ => debug!("Unknown xwayland setting: {}", key),
            }
        }
    }
    
    Ok(())
}