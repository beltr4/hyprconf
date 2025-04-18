use crate::config::models::MasterSection;
use anyhow::Result;
use log::debug;

pub fn parse_master_section(section: &mut MasterSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "new_status" => section.new_status = value.to_string(),
                _ => debug!("Unknown master setting: {}", key),
            }
        }
    }
    
    Ok(())
}

