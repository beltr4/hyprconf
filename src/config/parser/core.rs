
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{debug, info};

use crate::config::models::*;
use crate::config::parser::sections::*;


pub struct ConfigParser;

impl ConfigParser {
    /// Parse a Hyprland configuration file
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<HyprlandConfig> {
        let content = fs::read_to_string(path)
            .context("Failed to read Hyprland config file")?;
            
        Self::parse_string(&content)
    }
    
    /// Parse a Hyprland configuration from a string
    pub fn parse_string(content: &str) -> Result<HyprlandConfig> {
        debug!("Parsing Hyprland configuration");
        
        // Start with default config
        let mut config = HyprlandConfig::default();
        let mut variables = HashMap::new();
        let mut env_vars = HashMap::new();
        let mut autostart = Vec::new();
        
        // Track current section and any nested sections
        let mut current_section: Option<String> = None;
        let mut nested_section: Option<String> = None;
        let mut in_block = false;
        let mut block_content = String::new();
        
        // Process each line
        for (_i, line) in content.lines().enumerate() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Check if this is a variable definition
            if line.starts_with('$') {
                if let Some((name, value)) = line.split_once('=') {
                    let name = name.trim();
                    let value = value.trim();
                    debug!("Found variable: {} = {}", name, value);
                    variables.insert(name.to_string(), value.to_string());
                }
                continue;
            }
            
            // Check if this is an environment variable
            if line.starts_with("env = ") {
                let parts: Vec<&str> = line["env = ".len()..].splitn(2, ',').collect();
                if parts.len() == 2 {
                    let name = parts[0].trim();
                    let value = parts[1].trim();
                    debug!("Found env var: {} = {}", name, value);
                    env_vars.insert(name.to_string(), value.to_string());
                }
                continue;
            }
            
            // Check if this is an autostart command
            if line.starts_with("exec-once = ") {
                let command = line["exec-once = ".len()..].trim();
                debug!("Found autostart: {}", command);
                autostart.push(command.to_string());
                continue;
            }
            
            // Check if this is a source directive
            if line.starts_with("source") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    debug!("Found source directive: {}", parts[1]);
                    // TODO: Handle including other files
                }
                continue;
            }
            
            // Check for nested section end
            if line.contains('}') && in_block && nested_section.is_some() {
                if let (Some(section), Some(subsection)) = (&current_section, &nested_section) {
                    debug!("Processing nested section block: {}:{}", section, subsection);
                    Self::process_nested_section_block(&mut config, section, subsection, &block_content)?;
                }
                nested_section = None;
                block_content.clear();
                continue;
            }
            
            // Check for section end
            if line.contains('}') && in_block && nested_section.is_none() {
                if let Some(section) = &current_section {
                    debug!("Processing section block: {}", section);
                    Self::process_section_block(&mut config, section, &block_content)?;
                }
                in_block = false;
                current_section = None;
                block_content.clear();
                continue;
            }
            
            // Check for nested section start within a section
            if line.contains('{') && in_block && nested_section.is_none() {
                let subsection_name = line.split('{').next().unwrap().trim().to_string();
                nested_section = Some(subsection_name.clone());
                debug!("Entering nested section: {}:{}", current_section.as_ref().unwrap_or(&"".to_string()), subsection_name);
                continue;
            }
            
            // Check for section start
            if line.contains('{') && !in_block {
                let section_name = line.split('{').next().unwrap().trim().to_string();
                current_section = Some(section_name.clone());
                in_block = true;
                debug!("Entering section: {}", section_name);
                continue;
            }
            
            // If we're in a block, add the line to the block content
            if in_block {
                block_content.push_str(line);
                block_content.push('\n');
                continue;
            }
            
            // Monitor configuration
            if line.starts_with("monitor=") {
                let parts: Vec<&str> = line["monitor=".len()..].split(',').collect();
                if parts.len() >= 4 {
                    let monitor = MonitorConfig {
                        name: parts[0].trim().to_string(),
                        resolution: parts[1].trim().to_string(),
                        position: parts[2].trim().to_string(),
                        scale: parts[3].trim().parse().unwrap_or(1.0),
                    };
                    config.monitors.push(monitor);
                }
                continue;
            }
            
            // Window rules
            if line.starts_with("windowrule = ") {
                let parts: Vec<&str> = line["windowrule = ".len()..].splitn(2, ',').collect();
                if parts.len() == 2 {
                    let rule = WindowRule {
                        rule: parts[0].trim().to_string(),
                        value: parts[1].trim().to_string(),
                    };
                    config.window_rules.push(rule);
                }
                continue;
            }
            
            // Workspace rules
            if line.starts_with("workspace = ") {
                let parts: Vec<&str> = line["workspace = ".len()..].splitn(2, ',').collect();
                if parts.len() == 2 {
                    let rule = WorkspaceRule {
                        name: parts[0].trim().to_string(),
                        params: parts[1].trim().to_string(),
                    };
                    config.workspace_rules.push(rule);
                }
                continue;
            }
            
            // Keybindings
            if line.starts_with("bind = ") || line.starts_with("bindl = ") || 
               line.starts_with("bindr = ") || line.starts_with("bindm = ") || 
               line.starts_with("bindle = ") || line.starts_with("bindel = ") {
                // Process keybindings
                // TODO: Implement detailed keybinding parsing
                continue;
            }
            
            // If we're not in a block, this could be a top-level setting
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                debug!("Found top-level setting: {} = {}", key, value);
                // TODO: Handle top-level settings
            }
        }
        
        // Store the parsed variables, env vars, and autostart commands
        config.variables = variables;
        config.environment_variables = env_vars;
        config.autostart_programs = autostart;
        
        Ok(config)
    }
    
    /// Process a section block of configuration
    fn process_section_block(
        config: &mut HyprlandConfig, 
        section: &str, 
        content: &str
    ) -> Result<()> {
        match section {
            "general" => general::parse_general_section(&mut config.general, content)?,
            "decoration" => decoration::parse_decoration_section(&mut config.decoration, content)?,
            "animations" => animations::parse_animations_section(&mut config.animations, content)?,
            "input" => input::parse_input_section(&mut config.input, content)?,
            "gestures" => gestures::parse_gestures_section(&mut config.gestures, content)?,
            "group" => group::parse_group_section(&mut config.group, content)?,
            "misc" => misc::parse_misc_section(&mut config.misc, content)?,
            "binds" => binds::parse_binds_section(&mut config.binds, content)?,
            "xwayland" => xwayland::parse_xwayland_section(&mut config.xwayland, content)?,
            "opengl" => opengl::parse_opengl_section(&mut config.opengl, content)?,
            "render" => render::parse_render_section(&mut config.render, content)?,
            "cursor" => cursor::parse_cursor_section(&mut config.cursor, content)?,
            "dwindle" => dwindle::parse_dwindle_section(&mut config.dwindle, content)?,
            "master" => master::parse_master_section(&mut config.master, content)?,
            "debug" => debug::parse_debug_section(&mut config.debug, content)?,
            _ => {
                debug!("Unknown section: {}", section);
                // We don't want to fail parsing if we encounter an unknown section
            }
        }
        
        Ok(())
    }
    
    /// Process a nested section block of configuration
    fn process_nested_section_block(
        config: &mut HyprlandConfig,
        section: &str,
        subsection: &str,
        content: &str
    ) -> Result<()> {
        match (section, subsection) {
            ("general", "snap") => snap::parse_snap_section(&mut config.general.snap, content)?,
            ("decoration", "blur") => blur::parse_blur_section(&mut config.decoration.blur, content)?,
            ("decoration", "shadow") => shadow::parse_shadow_section(&mut config.decoration.shadow, content)?,
            ("input", "touchpad") => touch::parse_touchpad_section(&mut config.input.touchpad, content)?,
            ("input", "touchdevice") => touch::parse_touchdevice_section(&mut config.input.touchdevice, content)?,
            ("input", "tablet") => touch::parse_tablet_section(&mut config.input.tablet, content)?,
            ("group", "groupbar") => group::parse_groupbar_section(&mut config.group.groupbar, content)?,
            _ => {
                debug!("Unknown nested section: {}:{}", section, subsection);
                // We don't want to fail parsing if we encounter an unknown nested section
            }
        }
        
        Ok(())
    }
    
    
}
