use std::io::{self, Write};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneralSection {
    pub border_size: i32,
    pub no_border_on_floating: bool,
    pub gaps_in: String,
    pub gaps_out: String,
    pub gaps_workspaces: i32,
    pub col_inactive_border: String,
    pub col_active_border: String,
    pub col_nogroup_border: String,
    pub col_nogroup_border_active: String,
    pub layout: String,
    pub no_focus_fallback: bool,
    pub resize_on_border: bool,
    pub extend_border_grab_area: i32,
    pub hover_icon_on_border: bool,
    pub allow_tearing: bool,
    pub resize_corner: i32,
    pub snap: SnapSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapSection {
    pub enabled: bool,
    pub window_gap: i32,
    pub monitor_gap: i32,
    pub border_overlap: bool,
}

pub fn parse_general_section(section: &mut GeneralSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.contains('{') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "border_size" => section.border_size = value.parse().unwrap_or(1),
                "no_border_on_floating" => section.no_border_on_floating = parse_bool(value),
                "gaps_in" => section.gaps_in = value.to_string(),
                "gaps_out" => section.gaps_out = value.to_string(),
                "gaps_workspaces" => section.gaps_workspaces = value.parse().unwrap_or(0),
                "col.inactive_border" => section.col_inactive_border = value.to_string(),
                "col.active_border" => section.col_active_border = value.to_string(),
                "col.nogroup_border" => section.col_nogroup_border = value.to_string(),
                "col.nogroup_border_active" => section.col_nogroup_border_active = value.to_string(),
                "layout" => section.layout = value.to_string(),
                "no_focus_fallback" => section.no_focus_fallback = parse_bool(value),
                "resize_on_border" => section.resize_on_border = parse_bool(value),
                "extend_border_grab_area" => section.extend_border_grab_area = value.parse().unwrap_or(15),
                "hover_icon_on_border" => section.hover_icon_on_border = parse_bool(value),
                "allow_tearing" => section.allow_tearing = parse_bool(value),
                "resize_corner" => section.resize_corner = value.parse().unwrap_or(0),
                _ => debug!("Unknown general setting: {}", key),
            }
        }
    }
    Ok(())
}

pub fn write_general_section<W: Write>(
    writer: &mut W,
    general: &GeneralSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "general {", comment_style)?;
    write_option(writer, " border_size", &general.border_size, None, comment_style)?;
    write_boolean_option(writer, " no_border_on_floating", general.no_border_on_floating, None, comment_style)?;
    write_option(writer, " gaps_in", &general.gaps_in, None, comment_style)?;
    write_option(writer, " gaps_out", &general.gaps_out, None, comment_style)?;
    write_option(writer, " gaps_workspaces", &general.gaps_workspaces, None, comment_style)?;
    write_option(writer, " col.inactive_border", &general.col_inactive_border, None, comment_style)?;
    write_option(writer, " col.active_border", &general.col_active_border, None, comment_style)?;
    write_option(writer, " col.nogroup_border", &general.col_nogroup_border, None, comment_style)?;
    write_option(writer, " col.nogroup_border_active", &general.col_nogroup_border_active, None, comment_style)?;
    write_option(writer, " layout", &general.layout, None, comment_style)?;
    write_boolean_option(writer, " no_focus_fallback", general.no_focus_fallback, None, comment_style)?;
    write_boolean_option(writer, " resize_on_border", general.resize_on_border, None, comment_style)?;
    write_option(writer, " extend_border_grab_area", &general.extend_border_grab_area, None, comment_style)?;
    write_boolean_option(writer, " hover_icon_on_border", general.hover_icon_on_border, None, comment_style)?;
    write_boolean_option(writer, " allow_tearing", general.allow_tearing, None, comment_style)?;
    write_option(writer, " resize_corner", &general.resize_corner, None, comment_style)?;
    writeln!(writer, " snap {{")?;
    write_boolean_option(writer, "  enabled", general.snap.enabled, None, comment_style)?;
    write_option(writer, "  window_gap", &general.snap.window_gap, None, comment_style)?;
    write_option(writer, "  monitor_gap", &general.snap.monitor_gap, None, comment_style)?;
    write_boolean_option(writer, "  border_overlap", general.snap.border_overlap, None, comment_style)?;
    writeln!(writer, " }}")?;
    writeln!(writer, "}}")?;
    Ok(())
}

pub fn parse_snap_section(section: &mut SnapSection, content: &str) -> Result<()> {
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
                "window_gap" => section.window_gap = value.parse().unwrap_or(10),
                "monitor_gap" => section.monitor_gap = value.parse().unwrap_or(10),
                "border_overlap" => section.border_overlap = parse_bool(value),
                _ => debug!("Unknown snap setting: {}", key),
            }
        }
    }
    Ok(())
}
pub fn write_snap_section<W: Write>(
    writer: &mut W,
    snap: &SnapSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "snap {", comment_style)?;
    write_boolean_option(writer, " enabled", snap.enabled, None, comment_style)?;
    write_option(writer, " window_gap", &snap.window_gap, None, comment_style)?;
    write_option(writer, " monitor_gap", &snap.monitor_gap, None, comment_style)?;
    write_boolean_option(writer, " border_overlap", snap.border_overlap, None, comment_style)?;
    writeln!(writer, "}}")?;
    Ok(())
}


