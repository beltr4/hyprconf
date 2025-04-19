use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasterSection {
    pub allow_small_split: bool,
    pub special_scale_factor: f32,
    pub mfact: f32,
    pub new_status: String,
    pub new_on_top: bool,
    pub new_on_active: String,
    pub orientation: String,
    pub inherit_fullscreen: bool,
    pub slave_count_for_center_master: i32,
    pub center_master_slaves_on_right: bool,
    pub smart_resizing: bool,
    pub drop_at_cursor: bool,
    pub always_keep_position: bool,
}

pub fn parse_master_section(section: &mut MasterSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "allow_small_split" => section.allow_small_split = parse_bool(value),
                "special_scale_factor" => section.special_scale_factor = value.parse().unwrap_or(0.8),
                "mfact" => section.mfact = value.parse().unwrap_or(0.55),
                "new_status" => section.new_status = value.to_string(),
                "new_on_top" => section.new_on_top = parse_bool(value),
                "new_on_active" => section.new_on_active = value.to_string(),
                "orientation" => section.orientation = value.to_string(),
                "inherit_fullscreen" => section.inherit_fullscreen = parse_bool(value),
                "slave_count_for_center_master" => section.slave_count_for_center_master = value.parse().unwrap_or(2),
                "center_master_slaves_on_right" => section.center_master_slaves_on_right = parse_bool(value),
                "smart_resizing" => section.smart_resizing = parse_bool(value),
                "drop_at_cursor" => section.drop_at_cursor = parse_bool(value),
                "always_keep_position" => section.always_keep_position = parse_bool(value),
                _ => debug!("Unknown master setting: {}", key),
            }
        }
    }
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    master: &MasterSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "master {", comment_style)?;
    write_boolean_option(writer, " allow_small_split", master.allow_small_split, None, comment_style)?;
    write_option(writer, " special_scale_factor", &master.special_scale_factor, None, comment_style)?;
    write_option(writer, " mfact", &master.mfact, None, comment_style)?;
    write_option(writer, " new_status", &master.new_status, None, comment_style)?;
    write_boolean_option(writer, " new_on_top", master.new_on_top, None, comment_style)?;
    write_option(writer, " new_on_active", &master.new_on_active, None, comment_style)?;
    write_option(writer, " orientation", &master.orientation, None, comment_style)?;
    write_boolean_option(writer, " inherit_fullscreen", master.inherit_fullscreen, None, comment_style)?;
    write_option(writer, " slave_count_for_center_master", &master.slave_count_for_center_master, None, comment_style)?;
    write_boolean_option(writer, " center_master_slaves_on_right", master.center_master_slaves_on_right, None, comment_style)?;
    write_boolean_option(writer, " smart_resizing", master.smart_resizing, None, comment_style)?;
    write_boolean_option(writer, " drop_at_cursor", master.drop_at_cursor, None, comment_style)?;
    write_boolean_option(writer, " always_keep_position", master.always_keep_position, None, comment_style)?;
    writeln!(writer, "}}")?;
    Ok(())
}
