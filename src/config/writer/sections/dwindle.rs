use std::io::{self, Write};
use crate::config::models::DwindleSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    dwindle: &DwindleSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "dwindle {", comment_style)?;
    
    // Write all dwindle settings - all fields are now required (non-Option)
    write_boolean_option(writer, " pseudotile", dwindle.pseudotile, None, comment_style)?;
    write_boolean_option(writer, " preserve_split", dwindle.preserve_split, None, comment_style)?;
    write_boolean_option(writer, " smart_split", dwindle.smart_split, None, comment_style)?;
    write_option(writer, " force_split", &dwindle.force_split.to_string(), None, comment_style)?;
    write_boolean_option(writer, " permanent_direction_override", dwindle.permanent_direction_override, None, comment_style)?;
    write_option(writer, " special_scale_factor", &dwindle.special_scale_factor.to_string(), None, comment_style)?;
    write_option(writer, " split_width_multiplier", &dwindle.split_width_multiplier.to_string(), None, comment_style)?;
    write_boolean_option(writer, " use_active_for_splits", dwindle.use_active_for_splits, None, comment_style)?;
    write_option(writer, " default_split_ratio", &dwindle.default_split_ratio.to_string(), None, comment_style)?;
    
    // New fields
    write_option(writer, " split_bias", &dwindle.split_bias.to_string(), None, comment_style)?;
    write_boolean_option(writer, " smart_resizing", dwindle.smart_resizing, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}