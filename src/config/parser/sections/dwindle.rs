use crate::config::models::DwindleSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_dwindle_section(section: &mut DwindleSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "pseudotile" => section.pseudotile = parse_bool(value),
                "preserve_split" => section.preserve_split = parse_bool(value),
                _ => debug!("Unknown dwindle setting: {}", key),
            }
        }
    }
    
    Ok(())
}
