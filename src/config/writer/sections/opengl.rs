use std::io::{self, Write};
use crate::config::models::OpenGLSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    opengl: &OpenGLSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "opengl {", comment_style)?;
    
    write_boolean_option(writer, " nvidia_anti_flicker", opengl.nvidia_anti_flicker, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}