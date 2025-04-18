// src/config/parser/sections/animations.rs
use crate::config::models::Animation;
use crate::config::models::AnimationsSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;


/// Parse the animations section
pub fn parse_animations_section(section: &mut AnimationsSection, content: &str) -> Result<()> {
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
                "first_launch_animation" => section.first_launch_animation = parse_bool(value),
                "bezier" => {
                    let parts: Vec<&str> = value.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        let curve = parts[1..].join(",").trim().to_string();
                        section.beziers.insert(name.to_string(), curve);
                    }
                },
                "animation" => {
                    let parts: Vec<&str> = value.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        let params = parts[1..].join(",").trim().to_string();
                        section.animations.push(Animation {
                            name: name.to_string(),
                            params,
                        });
                    }
                },
                _ => debug!("Unknown animations setting: {}", key),
            }
        }
    }
    
    Ok(())
}