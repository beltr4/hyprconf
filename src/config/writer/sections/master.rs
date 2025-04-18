use std::io::{self, Write};
use crate::config::models::MasterSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    master: &MasterSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "master {", comment_style)?;
    
    write_boolean_option(writer, " allow_small_split", master.allow_small_split, None, comment_style)?;
    write_option(writer, " special_scale_factor", &master.special_scale_factor, None, comment_style)?;
    write_option(writer, " mfact", &master.mfact, None, comment_style)?;
    write_option(writer, " new_status", &master.new_status, None, comment_style)?;
    write_boolean_option(writer, " new_on_top", master.new_on_top, None, comment_style)?;
    write_option(writer, " new_on_active", &master.new_on_active, None, comment_style)?;
    write_option(writer, " orientation", &master.orientation, None, comment_style)?;
    write_boolean_option(writer, " inherit_fullscreen", master.inherit_fullscreen, None, comment_style)?;
    write_option(writer, " slave_count_for_center_master", &master.slave_count_for_center_master, None, comment_style)?;
    write_boolean_option(writer, " center_master_slaves_on_right", master.center_master_slaves_on_right, None, comment_style)?;
    write_boolean_option(writer, " smart_resizing", master.smart_resizing, None, comment_style)?;
    write_boolean_option(writer, " drop_at_cursor", master.drop_at_cursor, None, comment_style)?;
    write_boolean_option(writer, " always_keep_position", master.always_keep_position, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}