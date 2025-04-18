use crate::config::models::OpenGLSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_opengl_section(section: &mut OpenGLSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "nvidia_anti_flicker" => section.nvidia_anti_flicker = parse_bool(value),
                _ => debug!("Unknown opengl setting: {}", key),
            }
        }
    }
    
    Ok(())
}