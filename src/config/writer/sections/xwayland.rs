use std::io::{self, Write};
use crate::config::models::XWaylandSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_boolean_option};

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