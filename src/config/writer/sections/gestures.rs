use std::io::{self, Write};
use crate::config::models::GesturesSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    gestures: &GesturesSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "gestures {", comment_style)?;
    
    write_boolean_option(writer, " workspace_swipe", gestures.workspace_swipe, None, comment_style)?;
    write_option(writer, " workspace_swipe_fingers", &gestures.workspace_swipe_fingers, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_min_fingers", gestures.workspace_swipe_min_fingers, None, comment_style)?;
    write_option(writer, " workspace_swipe_distance", &gestures.workspace_swipe_distance, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_touch", gestures.workspace_swipe_touch, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_invert", gestures.workspace_swipe_invert, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_touch_invert", gestures.workspace_swipe_touch_invert, None, comment_style)?;
    write_option(writer, " workspace_swipe_min_speed_to_force", &gestures.workspace_swipe_min_speed_to_force, None, comment_style)?;
    write_option(writer, " workspace_swipe_cancel_ratio", &gestures.workspace_swipe_cancel_ratio, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_create_new", gestures.workspace_swipe_create_new, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_direction_lock", gestures.workspace_swipe_direction_lock, None, comment_style)?;
    write_option(writer, " workspace_swipe_direction_lock_threshold", &gestures.workspace_swipe_direction_lock_threshold, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_forever", gestures.workspace_swipe_forever, None, comment_style)?;
    write_boolean_option(writer, " workspace_swipe_use_r", gestures.workspace_swipe_use_r, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}