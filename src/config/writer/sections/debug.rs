use std::io::{self, Write};
use crate::config::models::DebugSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_boolean_option, write_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    debug: &DebugSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "debug {", comment_style)?;
    
    // Write all debug settings
    write_boolean_option(writer, " overlay", debug.overlay, None, comment_style)?;
    write_boolean_option(writer, " damage_blink", debug.damage_blink, None, comment_style)?;
    write_boolean_option(writer, " disable_logs", debug.disable_logs, None, comment_style)?;
    write_boolean_option(writer, " disable_time", debug.disable_time, None, comment_style)?;
    write_option(writer, " damage_tracking", &debug.damage_tracking.to_string(), None, comment_style)?;
    write_boolean_option(writer, " enable_stdout_logs", debug.enable_stdout_logs, None, comment_style)?;
    write_option(writer, " manual_crash", &debug.manual_crash.to_string(), None, comment_style)?;
    write_boolean_option(writer, " suppress_errors", debug.suppress_errors, None, comment_style)?;
    write_option(writer, " watchdog_timeout", &debug.watchdog_timeout.to_string(), None, comment_style)?;
    write_boolean_option(writer, " disable_scale_checks", debug.disable_scale_checks, None, comment_style)?;
    write_option(writer, " error_limit", &debug.error_limit.to_string(), None, comment_style)?;
    write_option(writer, " error_position", &debug.error_position.to_string(), None, comment_style)?;
    write_boolean_option(writer, " colored_stdout_logs", debug.colored_stdout_logs, None, comment_style)?;
    write_boolean_option(writer, " pass", debug.pass, None, comment_style)?;
    write_boolean_option(writer, " full_cm_proto", debug.full_cm_proto, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}