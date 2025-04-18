use std::io::{self, Write};
use crate::config::models::ShadowSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    shadow: &ShadowSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "shadow {", comment_style)?;
    
    write_boolean_option(writer, " enabled", shadow.enabled, None, comment_style)?;
    write_option(writer, " range", &shadow.range, None, comment_style)?;
    write_option(writer, " render_power", &shadow.render_power, None, comment_style)?;
    write_boolean_option(writer, " sharp", shadow.sharp, None, comment_style)?;
    write_boolean_option(writer, " ignore_window", shadow.ignore_window, None, comment_style)?;
    write_option(writer, " color", &shadow.color, None, comment_style)?;
    write_option(writer, " color_inactive", &shadow.color_inactive, None, comment_style)?;
    
    // Handle the offset tuple
    write_option(writer, " offset", &format!("{} {}", shadow.offset.0, shadow.offset.1), None, comment_style)?;
    
    write_option(writer, " scale", &shadow.scale, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}