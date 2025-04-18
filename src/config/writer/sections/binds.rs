use std::io::{self, Write};
use crate::config::models::{BindsSection, KeyBind};
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

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
            write_bind(writer, bind, comment_style)?;
        }
        writeln!(writer, "}}")?;
    }
    
    // Write submaps if they exist
    for (submap_name, submap_binds) in &binds.submaps {
        if !submap_binds.is_empty() {
            write_section_header(writer, &format!("submap {} {{", submap_name), comment_style)?;
            for bind in submap_binds {
                write_bind(writer, bind, comment_style)?;
            }
            writeln!(writer, "}}")?;
        }
    }
    
    Ok(())
}

fn write_bind<W: Write>(
    writer: &mut W,
    bind: &KeyBind,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Format is: MOD, key, dispatchers, arg
    write!(writer, " {}, {}, ", bind.modifiers, bind.key)?;
    
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