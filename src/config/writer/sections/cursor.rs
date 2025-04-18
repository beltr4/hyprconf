use std::io::{self, Write};
use crate::config::models::CursorSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_boolean_option, write_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    cursor: &CursorSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "cursor {", comment_style)?;
    
    // Write all cursor settings
    write_boolean_option(writer, " sync_gsettings_theme", cursor.sync_gsettings_theme, None, comment_style)?;
    write_option(writer, " no_hardware_cursors", &cursor.no_hardware_cursors.to_string(), None, comment_style)?;
    write_option(writer, " no_break_fs_vrr", &cursor.no_break_fs_vrr.to_string(), None, comment_style)?;
    write_option(writer, " min_refresh_rate", &cursor.min_refresh_rate.to_string(), None, comment_style)?;
    write_option(writer, " hotspot_padding", &cursor.hotspot_padding.to_string(), None, comment_style)?;
    write_option(writer, " inactive_timeout", &cursor.inactive_timeout.to_string(), None, comment_style)?;
    write_boolean_option(writer, " no_warps", cursor.no_warps, None, comment_style)?;
    write_boolean_option(writer, " persistent_warps", cursor.persistent_warps, None, comment_style)?;
    write_option(writer, " warp_on_change_workspace", &cursor.warp_on_change_workspace.to_string(), None, comment_style)?;
    write_option(writer, " warp_on_toggle_special", &cursor.warp_on_toggle_special.to_string(), None, comment_style)?;
    write_option(writer, " default_monitor", &cursor.default_monitor, None, comment_style)?;
    write_option(writer, " zoom_factor", &cursor.zoom_factor.to_string(), None, comment_style)?;
    write_boolean_option(writer, " zoom_rigid", cursor.zoom_rigid, None, comment_style)?;
    write_boolean_option(writer, " enable_hyprcursor", cursor.enable_hyprcursor, None, comment_style)?;
    write_boolean_option(writer, " hide_on_key_press", cursor.hide_on_key_press, None, comment_style)?;
    write_boolean_option(writer, " hide_on_touch", cursor.hide_on_touch, None, comment_style)?;
    write_option(writer, " use_cpu_buffer", &cursor.use_cpu_buffer.to_string(), None, comment_style)?;
    write_boolean_option(writer, " warp_back_after_non_mouse_input", cursor.warp_back_after_non_mouse_input, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}