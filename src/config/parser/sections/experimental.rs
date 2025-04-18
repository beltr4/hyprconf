use crate::config::models::ExperimentalSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_experimental_section(section: &mut ExperimentalSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "xx_color_management_v4" => section.xx_color_management_v4 = parse_bool(value),
                _ => debug!("Unknown experimental setting: {}", key),
            }
        }
    }
    
    Ok(())
}