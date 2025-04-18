use crate::config::models::EcosystemSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_ecosystem_section(section: &mut EcosystemSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "no_update_news" => section.no_update_news = parse_bool(value),
                "no_donation_nag" => section.no_donation_nag = parse_bool(value),
                "enforce_permissions" => section.enforce_permissions = parse_bool(value),
                _ => debug!("Unknown ecosystem setting: {}", key),
            }
        }
    }
    
    Ok(())
}