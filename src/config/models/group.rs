use std::io::{self, Write};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupSection {
    pub auto_group: bool,
    pub insert_after_current: bool,
    pub focus_removed_window: bool,
    pub drag_into_group: i32,
    pub merge_groups_on_drag: bool,
    pub merge_groups_on_groupbar: bool,
    pub merge_floated_into_tiled_on_groupbar: bool,
    pub group_on_movetoworkspace: bool,
    pub col_border_active: String,
    pub col_border_inactive: String,
    pub col_border_locked_active: String,
    pub col_border_locked_inactive: String,
    pub groupbar: GroupbarSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupbarSection {
    pub enabled: bool,
    pub font_family: String,
    pub font_size: i32,
    pub gradients: bool,
    pub height: i32,
    pub indicator_height: i32,
    pub stacked: bool,
    pub priority: i32,
    pub render_titles: bool,
    pub text_offset: i32,
    pub scrolling: bool,
    pub rounding: i32,
    pub gradient_rounding: i32,
    pub round_only_edges: bool,
    pub gradient_round_only_edges: bool,
    pub text_color: String,
    pub col_active: String,
    pub col_inactive: String,
    pub col_locked_active: String,
    pub col_locked_inactive: String,
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub keep_upper_gap: bool,
}

pub fn parse_group_section(section: &mut GroupSection, content: &str) -> Result<()> {
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
                "auto_group" => section.auto_group = parse_bool(value),
                "insert_after_current" => section.insert_after_current = parse_bool(value),
                "focus_removed_window" => section.focus_removed_window = parse_bool(value),
                "drag_into_group" => section.drag_into_group = value.parse().unwrap_or(1),
                "merge_groups_on_drag" => section.merge_groups_on_drag = parse_bool(value),
                "merge_groups_on_groupbar" => section.merge_groups_on_groupbar = parse_bool(value),
                "merge_floated_into_tiled_on_groupbar" => section.merge_floated_into_tiled_on_groupbar = parse_bool(value),
                "group_on_movetoworkspace" => section.group_on_movetoworkspace = parse_bool(value),
                "col.border_active" => section.col_border_active = value.to_string(),
                "col.border_inactive" => section.col_border_inactive = value.to_string(),
                "col.border_locked_active" => section.col_border_locked_active = value.to_string(),
                "col.border_locked_inactive" => section.col_border_locked_inactive = value.to_string(),
                _ => debug!("Unknown group setting: {}", key),
            }
        }
    }
    Ok(())
}

pub fn parse_groupbar_section(section: &mut GroupbarSection, content: &str) -> Result<()> {
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
                "font_family" => section.font_family = value.to_string(),
                "font_size" => section.font_size = value.parse().unwrap_or(8),
                "gradients" => section.gradients = parse_bool(value),
                "height" => section.height = value.parse().unwrap_or(14),
                "indicator_height" => section.indicator_height = value.parse().unwrap_or(3),
                "stacked" => section.stacked = parse_bool(value),
                "priority" => section.priority = value.parse().unwrap_or(3),
                "render_titles" => section.render_titles = parse_bool(value),
                "text_offset" => section.text_offset = value.parse().unwrap_or(0),
                "scrolling" => section.scrolling = parse_bool(value),
                "rounding" => section.rounding = value.parse().unwrap_or(1),
                "gradient_rounding" => section.gradient_rounding = value.parse().unwrap_or(2),
                "round_only_edges" => section.round_only_edges = parse_bool(value),
                "gradient_round_only_edges" => section.gradient_round_only_edges = parse_bool(value),
                "text_color" => section.text_color = value.to_string(),
                "col.active" => section.col_active = value.to_string(),
                "col.inactive" => section.col_inactive = value.to_string(),
                "col.locked_active" => section.col_locked_active = value.to_string(),
                "col.locked_inactive" => section.col_locked_inactive = value.to_string(),
                "gaps_in" => section.gaps_in = value.parse().unwrap_or(2),
                "gaps_out" => section.gaps_out = value.parse().unwrap_or(2),
                "keep_upper_gap" => section.keep_upper_gap = parse_bool(value),
                _ => debug!("Unknown groupbar setting: {}", key),
            }
        }
    }
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    group: &GroupSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "group {", comment_style)?;
    write_boolean_option(writer, " auto_group", group.auto_group, None, comment_style)?;
    write_boolean_option(writer, " insert_after_current", group.insert_after_current, None, comment_style)?;
    write_boolean_option(writer, " focus_removed_window", group.focus_removed_window, None, comment_style)?;
    write_option(writer, " drag_into_group", &group.drag_into_group, None, comment_style)?;
    write_boolean_option(writer, " merge_groups_on_drag", group.merge_groups_on_drag, None, comment_style)?;
    write_boolean_option(writer, " merge_groups_on_groupbar", group.merge_groups_on_groupbar, None, comment_style)?;
    write_boolean_option(writer, " merge_floated_into_tiled_on_groupbar", group.merge_floated_into_tiled_on_groupbar, None, comment_style)?;
    write_boolean_option(writer, " group_on_movetoworkspace", group.group_on_movetoworkspace, None, comment_style)?;
    write_option(writer, " col.border_active", &group.col_border_active, None, comment_style)?;
    write_option(writer, " col.border_inactive", &group.col_border_inactive, None, comment_style)?;
    write_option(writer, " col.border_locked_active", &group.col_border_locked_active, None, comment_style)?;
    write_option(writer, " col.border_locked_inactive", &group.col_border_locked_inactive, None, comment_style)?;
    writeln!(writer, " groupbar {{")?;
    write_boolean_option(writer, "  enabled", group.groupbar.enabled, None, comment_style)?;
    write_option(writer, "  font_family", &group.groupbar.font_family, None, comment_style)?;
    write_option(writer, "  font_size", &group.groupbar.font_size, None, comment_style)?;
    write_boolean_option(writer, "  gradients", group.groupbar.gradients, None, comment_style)?;
    write_option(writer, "  height", &group.groupbar.height, None, comment_style)?;
    write_option(writer, "  indicator_height", &group.groupbar.indicator_height, None, comment_style)?;
    write_boolean_option(writer, "  stacked", group.groupbar.stacked, None, comment_style)?;
    write_option(writer, "  priority", &group.groupbar.priority, None, comment_style)?;
    write_boolean_option(writer, "  render_titles", group.groupbar.render_titles, None, comment_style)?;
    write_option(writer, "  text_offset", &group.groupbar.text_offset, None, comment_style)?;
    write_boolean_option(writer, "  scrolling", group.groupbar.scrolling, None, comment_style)?;
    write_option(writer, "  rounding", &group.groupbar.rounding, None, comment_style)?;
    write_option(writer, "  gradient_rounding", &group.groupbar.gradient_rounding, None, comment_style)?;
    write_boolean_option(writer, "  round_only_edges", group.groupbar.round_only_edges, None, comment_style)?;
    write_boolean_option(writer, "  gradient_round_only_edges", group.groupbar.gradient_round_only_edges, None, comment_style)?;
    write_option(writer, "  text_color", &group.groupbar.text_color, None, comment_style)?;
    write_option(writer, "  col.active", &group.groupbar.col_active, None, comment_style)?;
    write_option(writer, "  col.inactive", &group.groupbar.col_inactive, None, comment_style)?;
    write_option(writer, "  col.locked_active", &group.groupbar.col_locked_active, None, comment_style)?;
    write_option(writer, "  col.locked_inactive", &group.groupbar.col_locked_inactive, None, comment_style)?;
    write_option(writer, "  gaps_in", &group.groupbar.gaps_in, None, comment_style)?;
    write_option(writer, "  gaps_out", &group.groupbar.gaps_out, None, comment_style)?;
    write_boolean_option(writer, "  keep_upper_gap", group.groupbar.keep_upper_gap, None, comment_style)?;
    writeln!(writer, " }}")?;
    writeln!(writer, "}}")?;
    Ok(())
}
