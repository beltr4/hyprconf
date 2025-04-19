use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DwindleSection {
    pub pseudotile: bool,
    pub preserve_split: bool,
    pub smart_split: bool,
    pub force_split: i32,
    pub permanent_direction_override: bool,
    pub special_scale_factor: f32,
    pub split_width_multiplier: f32,
    pub use_active_for_splits: bool,
    pub default_split_ratio: f32,
    pub split_bias: i32,
    pub smart_resizing: bool,
}

pub fn parse_dwindle_section(section: &mut DwindleSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "pseudotile" => section.pseudotile = parse_bool(value),
                "preserve_split" => section.preserve_split = parse_bool(value),
                "smart_split" => section.smart_split = parse_bool(value),
                "force_split" => section.force_split = value.parse().unwrap_or(0),
                "permanent_direction_override" => section.permanent_direction_override = parse_bool(value),
                "special_scale_factor" => section.special_scale_factor = value.parse().unwrap_or(0.8),
                "split_width_multiplier" => section.split_width_multiplier = value.parse().unwrap_or(1.0),
                "use_active_for_splits" => section.use_active_for_splits = parse_bool(value),
                "default_split_ratio" => section.default_split_ratio = value.parse().unwrap_or(1.0),
                "split_bias" => section.split_bias = value.parse().unwrap_or(0),
                "smart_resizing" => section.smart_resizing = parse_bool(value),
                _ => debug!("Unknown dwindle setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    dwindle: &DwindleSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "dwindle {", comment_style)?;
    
    // Write all dwindle settings - all fields are now required (non-Option)
    write_boolean_option(writer, " pseudotile", dwindle.pseudotile, None, comment_style)?;
    write_boolean_option(writer, " preserve_split", dwindle.preserve_split, None, comment_style)?;
    write_boolean_option(writer, " smart_split", dwindle.smart_split, None, comment_style)?;
    write_option(writer, " force_split", &dwindle.force_split.to_string(), None, comment_style)?;
    write_boolean_option(writer, " permanent_direction_override", dwindle.permanent_direction_override, None, comment_style)?;
    write_option(writer, " special_scale_factor", &dwindle.special_scale_factor.to_string(), None, comment_style)?;
    write_option(writer, " split_width_multiplier", &dwindle.split_width_multiplier.to_string(), None, comment_style)?;
    write_boolean_option(writer, " use_active_for_splits", dwindle.use_active_for_splits, None, comment_style)?;
    write_option(writer, " default_split_ratio", &dwindle.default_split_ratio.to_string(), None, comment_style)?;
    write_option(writer, " split_bias", &dwindle.split_bias.to_string(), None, comment_style)?;
    write_boolean_option(writer, " smart_resizing", dwindle.smart_resizing, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}