use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::{CommentStyle, parse_bool, write_boolean_option, write_section_header};
use crate::config::models::core::ConfigSection;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemSection {
    pub no_update_news: bool,
    pub no_donation_nag: bool,
    pub enforce_permissions: bool,
}

pub fn parse_ecosystem_section(section: &mut EcosystemSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        
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

/// Write the ecosystem section to the provided writer
pub fn write_ecosystem_section<W: Write>(
    writer: &mut W,
    ecosystem: &EcosystemSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Write section header
    write_section_header(writer, "ecosystem {", comment_style)?;
    
    // Write all the configuration options
    write_boolean_option(writer, " no_update_news", ecosystem.no_update_news, None, comment_style)?;
    write_boolean_option(writer, " no_donation_nag", ecosystem.no_donation_nag, None, comment_style)?;
    write_boolean_option(writer, " enforce_permissions", ecosystem.enforce_permissions, None, comment_style)?;
    
    // Close the section
    writeln!(writer, "}}")?;
    
    Ok(())
}

/// Implement ConfigSection for EcosystemSection
impl ConfigSection for EcosystemSection {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        write_ecosystem_section(writer, self, comment_style)
    }
}