use std::collections::HashMap;
use std::io::{self, Write};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};
use crate::config::utils::parse_bool;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimationsSection {
    pub enabled: bool,
    pub first_launch_animation: bool,
    pub beziers: HashMap<String, String>,
    pub animations: Vec<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub enabled: bool,
    pub speed: i32,
    pub curve: String,
    pub style: Option<String>,
}

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

pub fn write_animations_section<W: Write>(
    writer: &mut W,
    animations: &AnimationsSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "animations {", comment_style)?;

    // Write enabled state
    write_boolean_option(writer, " enabled", animations.enabled, None, comment_style)?;

    // Write first launch animation
    write_boolean_option(writer, " first_launch_animation", animations.first_launch_animation, None, comment_style)?;

    // Write bezier curves
    for (name, curve) in &animations.beziers {
        write_option(writer, &format!(" bezier = {}", name), curve, None, comment_style)?;
    }

    // Write animations
    for anim in &animations.animations {
        // Build the animation line: name,enabled,speed,curve[,style]
        let mut line = format!("{}, {}, {}, {}",
            anim.name,
            anim.enabled,
            anim.speed,
            anim.curve
        );
        if let Some(style) = &anim.style {
            line.push_str(", ");
            line.push_str(style);
        }
        write_option(writer, " animation", &line, None, comment_style)?;
    }

    writeln!(writer, "}}")?;
    Ok(())
}