use std::io::{self, Write};
use crate::config::models::GeneralSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
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
    
    // Write snap section
    writeln!(writer, " snap {{")?;
    write_boolean_option(writer, "  enabled", general.snap.enabled, None, comment_style)?;
    write_option(writer, "  window_gap", &general.snap.window_gap, None, comment_style)?;
    write_option(writer, "  monitor_gap", &general.snap.monitor_gap, None, comment_style)?;
    write_boolean_option(writer, "  border_overlap", general.snap.border_overlap, None, comment_style)?;
    writeln!(writer, " }}")?;
    
    writeln!(writer, "}}")?;
    Ok(())
}