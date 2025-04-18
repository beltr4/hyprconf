use crate::config::models::DebugSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_debug_section(section: &mut DebugSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
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