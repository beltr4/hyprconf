use std::io::{self, Write};
use crate::config::models::{DecorationSection, BlurSection, ShadowSection};
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};
use crate::config::writer::sections::blur; // Importing the blur writer
use crate::config::writer::sections::shadow; // Importing the shadow writer

pub fn write_section<W: Write>(
    writer: &mut W,
    decoration: &DecorationSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "decoration {", comment_style)?;
    
    // Write main decoration options
    write_option(writer, " rounding", &decoration.rounding.to_string(), None, comment_style)?;
    write_option(writer, " rounding_power", &decoration.rounding_power.to_string(), None, comment_style)?;
    write_option(writer, " active_opacity", &decoration.active_opacity.to_string(), None, comment_style)?;
    write_option(writer, " inactive_opacity", &decoration.inactive_opacity.to_string(), None, comment_style)?;
    write_option(writer, " fullscreen_opacity", &decoration.fullscreen_opacity.to_string(), None, comment_style)?;
    write_boolean_option(writer, " dim_inactive", decoration.dim_inactive, None, comment_style)?;
    write_option(writer, " dim_strength", &decoration.dim_strength.to_string(), None, comment_style)?;
    write_option(writer, " dim_special", &decoration.dim_special.to_string(), None, comment_style)?;
    write_option(writer, " dim_around", &decoration.dim_around.to_string(), None, comment_style)?;
    write_option(writer, " screen_shader", &decoration.screen_shader, None, comment_style)?;
    write_boolean_option(writer, " border_part_of_window", decoration.border_part_of_window, None, comment_style)?;
    
    // Write the blur subsection using the dedicated blur writer
    blur::write_section(writer, &decoration.blur, comment_style)?;
    
    // Write the shadow subsection using the dedicated shadow writer
    shadow::write_section(writer, &decoration.shadow, comment_style)?;
    
    writeln!(writer, "}}")?;
    
    Ok(())
}