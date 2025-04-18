use std::io::{self, Write};
use crate::config::models::SnapSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
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