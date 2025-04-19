use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_section_header};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct XWaylandSection {
    pub enabled: bool,
    pub use_nearest_neighbor: bool,
    pub force_zero_scaling: bool,
    pub create_abstract_socket: bool,
}

pub fn parse_xwayland_section(section: &mut XWaylandSection, content: &str) -> Result<()> {
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
                "use_nearest_neighbor" => section.use_nearest_neighbor = parse_bool(value),
                "force_zero_scaling" => section.force_zero_scaling = parse_bool(value),
                "create_abstract_socket" => section.create_abstract_socket = parse_bool(value),
                _ => debug!("Unknown xwayland setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    xwayland: &XWaylandSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "xwayland {", comment_style)?;
    
    write_boolean_option(writer, " enabled", xwayland.enabled, None, comment_style)?;
    write_boolean_option(writer, " use_nearest_neighbor", xwayland.use_nearest_neighbor, None, comment_style)?;
    write_boolean_option(writer, " force_zero_scaling", xwayland.force_zero_scaling, None, comment_style)?;
    write_boolean_option(writer, " create_abstract_socket", xwayland.create_abstract_socket, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}