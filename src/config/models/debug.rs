use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugSection {
    pub overlay: bool,
    pub damage_blink: bool,
    pub disable_logs: bool,
    pub disable_time: bool,
    pub damage_tracking: i32,
    pub enable_stdout_logs: bool,
    pub manual_crash: i32,
    pub suppress_errors: bool,
    pub watchdog_timeout: i32,
    pub disable_scale_checks: bool,
    pub error_limit: i32,
    pub error_position: i32,
    pub colored_stdout_logs: bool,
    pub pass: bool,
    pub full_cm_proto: bool,
}

pub fn parse_debug_section(section: &mut DebugSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "overlay" => section.overlay = parse_bool(value),
                "damage_blink" => section.damage_blink = parse_bool(value),
                "disable_logs" => section.disable_logs = parse_bool(value),
                "disable_time" => section.disable_time = parse_bool(value),
                "damage_tracking" => section.damage_tracking = value.parse().unwrap_or(2),
                "enable_stdout_logs" => section.enable_stdout_logs = parse_bool(value),
                "manual_crash" => section.manual_crash = value.parse().unwrap_or(0),
                "suppress_errors" => section.suppress_errors = parse_bool(value),
                "watchdog_timeout" => section.watchdog_timeout = value.parse().unwrap_or(5),
                "disable_scale_checks" => section.disable_scale_checks = parse_bool(value),
                "error_limit" => section.error_limit = value.parse().unwrap_or(5),
                "error_position" => section.error_position = value.parse().unwrap_or(0),
                "colored_stdout_logs" => section.colored_stdout_logs = parse_bool(value),
                "pass" => section.pass = parse_bool(value),
                "full_cm_proto" => section.full_cm_proto = parse_bool(value),
                _ => debug!("Unknown debug setting: {}", key),
            }
        }
    }
    Ok(())
}

pub fn write_section<W: Write>(writer: &mut W, debug: &DebugSection, comment_style: &CommentStyle) -> io::Result<()> {
    write_section_header(writer, "debug {", comment_style)?;
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
