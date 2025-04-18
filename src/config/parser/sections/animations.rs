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
                    if parts.len() >= 3 {
                        let name = parts[0].trim();
                        // Parse animation parameters
                        let mut enabled = true;
                        let mut speed = 10; // Default speed
                        let mut curve = "default".to_string();
                        let mut style = None;
                        
                        for i in 1..parts.len() {
                            let param = parts[i].trim();
                            if param == "enabled" || param == "on" {
                                enabled = true;
                            } else if param == "disabled" || param == "off" {
                                enabled = false;
                            } else if param.starts_with("speed:") {
                                if let Some(speed_str) = param.strip_prefix("speed:") {
                                    speed = speed_str.trim().parse().unwrap_or(10);
                                }
                            } else if param.starts_with("curve:") {
                                if let Some(curve_str) = param.strip_prefix("curve:") {
                                    curve = curve_str.trim().to_string();
                                }
                            } else if param.starts_with("style:") {
                                if let Some(style_str) = param.strip_prefix("style:") {
                                    style = Some(style_str.trim().to_string());
                                }
                            }
                        }
                        
                        section.animations.push(Animation {
                            name: name.to_string(),
                            enabled,
                            speed,
                            curve,
                            style,
                        });
                    }
                },
                _ => debug!("Unknown animations setting: {}", key),
            }
        }
    }
    
    Ok(())
}