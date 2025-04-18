use std::io::{self, Write};
use crate::config::models::BlurSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_boolean_option, write_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    blur: &BlurSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "blur {", comment_style)?;
    
    // All fields are now required (non-Option) according to the model
    write_boolean_option(writer, " enabled", blur.enabled, None, comment_style)?;
    write_option(writer, " size", &blur.size.to_string(), None, comment_style)?;
    write_option(writer, " passes", &blur.passes.to_string(), None, comment_style)?;
    write_boolean_option(writer, " ignore_opacity", blur.ignore_opacity, None, comment_style)?;
    write_boolean_option(writer, " new_optimizations", blur.new_optimizations, None, comment_style)?;
    write_boolean_option(writer, " xray", blur.xray, None, comment_style)?;
    write_option(writer, " noise", &blur.noise.to_string(), None, comment_style)?;
    write_option(writer, " contrast", &blur.contrast.to_string(), None, comment_style)?;
    write_option(writer, " brightness", &blur.brightness.to_string(), None, comment_style)?;
    
    // New fields
    write_option(writer, " vibrancy", &blur.vibrancy.to_string(), None, comment_style)?;
    write_option(writer, " vibrancy_darkness", &blur.vibrancy_darkness.to_string(), None, comment_style)?;
    write_boolean_option(writer, " special", blur.special, None, comment_style)?;
    write_boolean_option(writer, " popups", blur.popups, None, comment_style)?;
    write_option(writer, " popups_ignorealpha", &blur.popups_ignorealpha.to_string(), None, comment_style)?;
    write_boolean_option(writer, " input_methods", blur.input_methods, None, comment_style)?;
    write_option(writer, " input_methods_ignorealpha", &blur.input_methods_ignorealpha.to_string(), None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}