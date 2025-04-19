use std::collections::HashMap;
use std::io::{self, Write};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};
use crate::config::models::core::ConfigSection;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BindsSection {
    pub pass_mouse_when_bound: bool,
    pub scroll_event_delay: i32,
    pub workspace_back_and_forth: bool,
    pub hide_special_on_workspace_change: bool,
    pub allow_workspace_cycles: bool,
    pub workspace_center_on: i32,
    pub focus_preferred_method: i32,
    pub ignore_group_lock: bool,
    pub movefocus_cycles_fullscreen: bool,
    pub movefocus_cycles_groupfirst: bool,
    pub disable_keybind_grabbing: bool,
    pub window_direction_monitor_fallback: bool,
    pub allow_pin_fullscreen: bool,
    pub keybinds: Vec<KeyBind>,
    pub submaps: HashMap<String, Vec<KeyBind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBind {
    pub modifiers: String,
    pub key: String,
    pub dispatchers: Vec<String>,
    pub arg: String,
    pub flags: Option<String>,
    pub description: Option<String>,
}

/// Parse the binds section
pub fn parse_binds_section(section: &mut BindsSection, content: &str) -> Result<()> {
    let mut current_submap: Option<String> = None;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Check for bind or submap definition
        if line.starts_with("bind") || line.starts_with("unbind") {
            let bind_parts: Vec<&str> = line.splitn(2, '=').collect();
            if bind_parts.len() == 2 {
                let bind_value = bind_parts[1].trim();
                
                match parse_key_bind(bind_value) {
                    Ok(keybind) => {
                        if let Some(submap_name) = &current_submap {
                            // Add to current submap
                            section.submaps
                                .entry(submap_name.clone())
                                .or_insert_with(Vec::new)
                                .push(keybind);
                        } else {
                            // Add to main keybinds
                            section.keybinds.push(keybind);
                        }
                    },
                    Err(e) => debug!("Error parsing keybind: {}", e),
                }
            }
        } else if line.starts_with("submap") {
            let submap_parts: Vec<&str> = line.splitn(2, '=').collect();
            if submap_parts.len() == 2 {
                let submap_name = submap_parts[1].trim();
                if submap_name == "reset" {
                    current_submap = None;
                } else {
                    current_submap = Some(submap_name.to_string());
                }
            }
        } else if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "pass_mouse_when_bound" => section.pass_mouse_when_bound = parse_bool(value),
                "scroll_event_delay" => section.scroll_event_delay = value.parse().unwrap_or(300),
                "workspace_back_and_forth" => section.workspace_back_and_forth = parse_bool(value),
                "hide_special_on_workspace_change" => section.hide_special_on_workspace_change = parse_bool(value),
                "allow_workspace_cycles" => section.allow_workspace_cycles = parse_bool(value),
                "workspace_center_on" => section.workspace_center_on = value.parse().unwrap_or(0),
                "focus_preferred_method" => section.focus_preferred_method = value.parse().unwrap_or(0),
                "ignore_group_lock" => section.ignore_group_lock = parse_bool(value),
                "movefocus_cycles_fullscreen" => section.movefocus_cycles_fullscreen = parse_bool(value),
                "movefocus_cycles_groupfirst" => section.movefocus_cycles_groupfirst = parse_bool(value),
                "disable_keybind_grabbing" => section.disable_keybind_grabbing = parse_bool(value),
                "window_direction_monitor_fallback" => section.window_direction_monitor_fallback = parse_bool(value),
                "allow_pin_fullscreen" => section.allow_pin_fullscreen = parse_bool(value),
                _ => debug!("Unknown binds setting: {}", key),
            }
        }
    }
    
    Ok(())
}

/// Parse a key bind from line
pub fn parse_key_bind(content: &str) -> Result<KeyBind> {
    let parts: Vec<&str> = content.splitn(4, ',').collect();
    
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid keybind format"));
    }
    
    let modifiers = parts[0].trim().to_string();
    let key = parts[1].trim().to_string();
    
    // Parse dispatchers and args
    let dispatcher_part = parts[2].trim();
    let dispatchers: Vec<String> = dispatcher_part.split_whitespace()
        .map(|s| s.trim().to_string())
        .collect();
    
    // Get the argument (everything after the dispatcher)
    let arg = if parts.len() > 3 {
        parts[3].trim().to_string()
    } else {
        String::new()
    };
    
    // Optional fields
    let mut flags = None;
    let mut description = None;
    
    // Check for flags and description at the end of arg
    if let Some((_arg_part, meta)) = arg.split_once('#') {
        let meta_parts: Vec<&str> = meta.split(',').collect();
        
        for part in meta_parts {
            let part = part.trim();
            if part.starts_with("flags:") {
                flags = Some(part.strip_prefix("flags:").unwrap_or("").trim().to_string());
            } else if part.starts_with("desc:") {
                description = Some(part.strip_prefix("desc:").unwrap_or("").trim().to_string());
            } else if !part.is_empty() {
                // If it's not empty and doesn't have a prefix, assume it's a description
                description = Some(part.to_string());
            }
        }
    }
    
    Ok(KeyBind {
        modifiers,
        key,
        dispatchers,
        arg,
        flags,
        description,
    })
}

/// Write the binds section to the provided writer
pub fn write_section<W: Write>(
    writer: &mut W,
    binds: &BindsSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Write the binds section configuration
    write_section_header(writer, "binds {", comment_style)?;
    
    // Write all the configuration options
    write_boolean_option(writer, " pass_mouse_when_bound", binds.pass_mouse_when_bound, None, comment_style)?;
    write_option(writer, " scroll_event_delay", &binds.scroll_event_delay.to_string(), None, comment_style)?;
    write_boolean_option(writer, " workspace_back_and_forth", binds.workspace_back_and_forth, None, comment_style)?;
    write_boolean_option(writer, " hide_special_on_workspace_change", binds.hide_special_on_workspace_change, None, comment_style)?;
    write_boolean_option(writer, " allow_workspace_cycles", binds.allow_workspace_cycles, None, comment_style)?;
    write_option(writer, " workspace_center_on", &binds.workspace_center_on.to_string(), None, comment_style)?;
    write_option(writer, " focus_preferred_method", &binds.focus_preferred_method.to_string(), None, comment_style)?;
    write_boolean_option(writer, " ignore_group_lock", binds.ignore_group_lock, None, comment_style)?;
    write_boolean_option(writer, " movefocus_cycles_fullscreen", binds.movefocus_cycles_fullscreen, None, comment_style)?;
    write_boolean_option(writer, " movefocus_cycles_groupfirst", binds.movefocus_cycles_groupfirst, None, comment_style)?;
    write_boolean_option(writer, " disable_keybind_grabbing", binds.disable_keybind_grabbing, None, comment_style)?;
    write_boolean_option(writer, " window_direction_monitor_fallback", binds.window_direction_monitor_fallback, None, comment_style)?;
    write_boolean_option(writer, " allow_pin_fullscreen", binds.allow_pin_fullscreen, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    
    // Write the keybinds
    if !binds.keybinds.is_empty() {
        write_section_header(writer, "bind {", comment_style)?;
        for bind in &binds.keybinds {
            write_key_bind(writer, bind, comment_style)?;
        }
        writeln!(writer, "}}")?;
    }
    
    // Write submaps if they exist
    for (submap_name, submap_binds) in &binds.submaps {
        if !submap_binds.is_empty() {
            write_section_header(writer, &format!("submap {} {{", submap_name), comment_style)?;
            for bind in submap_binds {
                write_key_bind(writer, bind, comment_style)?;
            }
            writeln!(writer, "}}")?;
        }
    }
    
    Ok(())
}

/// Write a key bind to the provided writer
pub fn write_key_bind<W: Write>(
    writer: &mut W,
    bind: &KeyBind,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Format is: MOD, key, dispatchers, arg
    write!(writer, " bind = {}, {}, ", bind.modifiers, bind.key)?;
    
    // Join all dispatchers with space
    write!(writer, "{}", bind.dispatchers.join(" "))?;
    
    // Add arg if not empty
    if !bind.arg.is_empty() {
        write!(writer, ", {}", bind.arg)?;
    }
    
    // Add flags if present
    if let Some(flags) = &bind.flags {
        write!(writer, " {}", flags)?;
    }
    
    // Add description if present
    if let Some(description) = &bind.description {
        write!(writer, " {} {}", comment_style.prefix(), description)?;
    }
    
    writeln!(writer)?;
    Ok(())
}

/// Implement ConfigSection for BindsSection
impl ConfigSection for BindsSection {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        write_section(writer, self, comment_style)
    }
}