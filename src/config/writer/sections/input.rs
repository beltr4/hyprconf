use std::io::{self, Write};
use crate::config::models::InputSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    input: &InputSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "input {", comment_style)?;
    
    write_option(writer, " kb_model", &input.kb_model, None, comment_style)?;
    write_option(writer, " kb_layout", &input.kb_layout, None, comment_style)?;
    write_option(writer, " kb_variant", &input.kb_variant, None, comment_style)?;
    write_option(writer, " kb_options", &input.kb_options, None, comment_style)?;
    write_option(writer, " kb_rules", &input.kb_rules, None, comment_style)?;
    write_option(writer, " kb_file", &input.kb_file, None, comment_style)?;
    write_boolean_option(writer, " numlock_by_default", input.numlock_by_default, None, comment_style)?;
    write_boolean_option(writer, " resolve_binds_by_sym", input.resolve_binds_by_sym, None, comment_style)?;
    write_option(writer, " repeat_rate", &input.repeat_rate, None, comment_style)?;
    write_option(writer, " repeat_delay", &input.repeat_delay, None, comment_style)?;
    write_option(writer, " sensitivity", &input.sensitivity, None, comment_style)?;
    write_option(writer, " accel_profile", &input.accel_profile, None, comment_style)?;
    write_boolean_option(writer, " force_no_accel", input.force_no_accel, None, comment_style)?;
    write_boolean_option(writer, " left_handed", input.left_handed, None, comment_style)?;
    write_option(writer, " scroll_points", &input.scroll_points, None, comment_style)?;
    write_option(writer, " scroll_method", &input.scroll_method, None, comment_style)?;
    write_option(writer, " scroll_button", &input.scroll_button, None, comment_style)?;
    write_boolean_option(writer, " scroll_button_lock", input.scroll_button_lock, None, comment_style)?;
    write_option(writer, " scroll_factor", &input.scroll_factor, None, comment_style)?;
    write_boolean_option(writer, " natural_scroll", input.natural_scroll, None, comment_style)?;
    write_option(writer, " follow_mouse", &input.follow_mouse, None, comment_style)?;
    write_option(writer, " follow_mouse_threshold", &input.follow_mouse_threshold, None, comment_style)?;
    write_option(writer, " focus_on_close", &input.focus_on_close, None, comment_style)?;
    write_boolean_option(writer, " mouse_refocus", input.mouse_refocus, None, comment_style)?;
    write_option(writer, " float_switch_override_focus", &input.float_switch_override_focus, None, comment_style)?;
    write_boolean_option(writer, " special_fallthrough", input.special_fallthrough, None, comment_style)?;
    write_option(writer, " off_window_axis_events", &input.off_window_axis_events, None, comment_style)?;
    write_option(writer, " emulate_discrete_scroll", &input.emulate_discrete_scroll, None, comment_style)?;
    write_option(writer, " drag_threshold", &input.drag_threshold, None, comment_style)?;
    
    // Write touchpad subsection
    writeln!(writer, " touchpad {{")?;
    write_boolean_option(writer, "  disable_while_typing", input.touchpad.disable_while_typing, None, comment_style)?;
    write_boolean_option(writer, "  natural_scroll", input.touchpad.natural_scroll, None, comment_style)?;
    write_option(writer, "  scroll_factor", &input.touchpad.scroll_factor, None, comment_style)?;
    write_boolean_option(writer, "  middle_button_emulation", input.touchpad.middle_button_emulation, None, comment_style)?;
    write_option(writer, "  tap_button_map", &input.touchpad.tap_button_map, None, comment_style)?;
    write_boolean_option(writer, "  clickfinger_behavior", input.touchpad.clickfinger_behavior, None, comment_style)?;
    write_boolean_option(writer, "  tap_to_click", input.touchpad.tap_to_click, None, comment_style)?;
    write_boolean_option(writer, "  drag_lock", input.touchpad.drag_lock, None, comment_style)?;
    write_boolean_option(writer, "  tap_and_drag", input.touchpad.tap_and_drag, None, comment_style)?;
    write_boolean_option(writer, "  flip_x", input.touchpad.flip_x, None, comment_style)?;
    write_boolean_option(writer, "  flip_y", input.touchpad.flip_y, None, comment_style)?;
    writeln!(writer, " }}")?;
    
    // Write touchdevice subsection
    writeln!(writer, " touchdevice {{")?;
    write_option(writer, "  transform", &input.touchdevice.transform, None, comment_style)?;
    write_option(writer, "  output", &input.touchdevice.output, None, comment_style)?;
    write_boolean_option(writer, "  enabled", input.touchdevice.enabled, None, comment_style)?;
    writeln!(writer, " }}")?;
    
    // Write tablet subsection
    writeln!(writer, " tablet {{")?;
    write_option(writer, "  transform", &input.tablet.transform, None, comment_style)?;
    write_option(writer, "  output", &input.tablet.output, None, comment_style)?;
    write_option(writer, "  region_position", &format!("{} {}", input.tablet.region_position.0, input.tablet.region_position.1), None, comment_style)?;
    write_boolean_option(writer, "  absolute_region_position", input.tablet.absolute_region_position, None, comment_style)?;
    write_option(writer, "  region_size", &format!("{} {}", input.tablet.region_size.0, input.tablet.region_size.1), None, comment_style)?;
    write_boolean_option(writer, "  relative_input", input.tablet.relative_input, None, comment_style)?;
    write_boolean_option(writer, "  left_handed", input.tablet.left_handed, None, comment_style)?;
    write_option(writer, "  active_area_size", &format!("{} {}", input.tablet.active_area_size.0, input.tablet.active_area_size.1), None, comment_style)?;
    write_option(writer, "  active_area_position", &format!("{} {}", input.tablet.active_area_position.0, input.tablet.active_area_position.1), None, comment_style)?;
    writeln!(writer, " }}")?;
    
    writeln!(writer, "}}")?;
    Ok(())
}