use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::{CommentStyle, parse_bool, write_boolean_option, write_section_header};
use crate::config::models::core::ConfigSection;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExperimentalSection {
    pub xx_color_management_v4: bool,
}

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

/// Write the experimental section to the provided writer
pub fn write_experimental_section<W: Write>(
    writer: &mut W,
    experimental: &ExperimentalSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Write section header
    write_section_header(writer, "experimental {", comment_style)?;
    
    // Write all the configuration options
    write_boolean_option(writer, " xx_color_management_v4", experimental.xx_color_management_v4, None, comment_style)?;
    
    // Close the section
    writeln!(writer, "}}")?;
    
    Ok(())
}

/// Implement ConfigSection for ExperimentalSection
impl ConfigSection for ExperimentalSection {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        write_experimental_section(writer, self, comment_style)
    }
}