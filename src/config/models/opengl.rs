use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_section_header};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenGLSection {
    pub nvidia_anti_flicker: bool,
}

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

pub fn write_section<W: Write>(
    writer: &mut W,
    opengl: &OpenGLSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "opengl {", comment_style)?;
    write_boolean_option(writer, " nvidia_anti_flicker", opengl.nvidia_anti_flicker, None, comment_style)?;
    writeln!(writer, "}}")?;
    Ok(())
}