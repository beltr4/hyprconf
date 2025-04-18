use std::io::{self, Write};
use crate::config::models::GroupSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_boolean_option, write_option};

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
    
    // Write groupbar subsection
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