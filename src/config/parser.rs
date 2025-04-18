// src/config/parser.rs
use std::fs;
use std::path::Path;
use anyhow::{Result, Context, anyhow};
use log::{debug, warn, info};
use crate::config::models::*;

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
        
        // Track current section and any nested sections
        let mut current_section: Option<String> = None;
        let mut in_block = false;
        let mut block_content = String::new();
        
        // Process each line
        for (i, line) in content.lines().enumerate() {
            let line_num = i + 1;
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
                    // TODO: Store variables for later use
                }
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
            
            // Check for section start
            if line.contains('{') && !in_block {
                let section_name = line.split('{').next().unwrap().trim().to_string();
                current_section = Some(section_name.clone());
                in_block = true;
                debug!("Entering section: {}", section_name);
                continue;
            }
            
            // Check for section end
            if line.contains('}') && in_block {
                if let Some(section) = &current_section {
                    debug!("Processing section block: {}", section);
                    Self::process_section_block(&mut config, section, &block_content)?;
                }
                in_block = false;
                block_content.clear();
                current_section = None;
                continue;
            }
            
            // If we're in a block, add the line to the block content
            if in_block {
                block_content.push_str(line);
                block_content.push('\n');
                continue;
            }
            
            // If we're not in a block, this could be a top-level setting
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                debug!("Found top-level setting: {} = {}", key, value);
                // TODO: Handle top-level settings
            } else if !line.contains('=') && !line.contains('{') && !line.contains('}') {
                debug!("Found non-standard line: {}", line);
                // Some configs might have non-standard lines
            }
        }
        
        Ok(config)
    }
    
    /// Process a section block of configuration
    fn process_section_block(
        config: &mut HyprlandConfig, 
        section: &str, 
        content: &str
    ) -> Result<()> {
        match section {
            "general" => Self::parse_general_section(&mut config.general, content)?,
            "decoration" => Self::parse_decoration_section(&mut config.decoration, content)?,
            "animations" => Self::parse_animations_section(&mut config.animations, content)?,
            "input" => Self::parse_input_section(&mut config.input, content)?,
            "gestures" => Self::parse_gestures_section(&mut config.gestures, content)?,
            "misc" => Self::parse_misc_section(&mut config.misc, content)?,
            "dwindle" => Self::parse_dwindle_section(content)?, // Store in appropriate place
            "master" => Self::parse_master_section(content)?,   // Store in appropriate place
            "device" => Self::parse_device_section(content)?,   // Store in appropriate place
            _ => {
                debug!("Unknown section: {}", section);
                // We don't want to fail parsing if we encounter an unknown section
            }
        }
        
        Ok(())
    }
    
    /// Parse the general section
    fn parse_general_section(section: &mut GeneralSection, content: &str) -> Result<()> {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                match key {
                    "gaps_in" => section.gaps_in = value.parse()
                        .context(format!("Invalid gaps_in: {}", value))?,
                    "gaps_out" => section.gaps_out = value.parse()
                        .context(format!("Invalid gaps_out: {}", value))?,
                    "border_size" => section.border_size = value.parse()
                        .context(format!("Invalid border_size: {}", value))?,
                    "col.active_border" => section.col_active_border = value.to_string(),
                    "col.inactive_border" => section.col_inactive_border = value.to_string(),
                    "resize_on_border" => { /* Parse boolean */ },
                    "allow_tearing" => { /* Parse boolean */ },
                    "layout" => { /* Store layout */ },
                    _ => debug!("Unknown general setting: {}", key),
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse the decoration section
    fn parse_decoration_section(section: &mut DecorationSection, content: &str) -> Result<()> {
        let mut in_nested_block = false;
        let mut nested_block_name = String::new();
        let mut nested_block_content = String::new();
        
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Nested block handling
            if line.contains('{') && !in_nested_block {
                nested_block_name = line.split('{').next().unwrap().trim().to_string();
                in_nested_block = true;
                continue;
            }
            
            if line.contains('}') && in_nested_block {
                // Process the nested block (shadow or blur)
                match nested_block_name.as_str() {
                    "shadow" => Self::parse_shadow_section(&nested_block_content)?,
                    "blur" => Self::parse_blur_section(section, &nested_block_content)?,
                    _ => debug!("Unknown nested block in decoration: {}", nested_block_name),
                }
                
                in_nested_block = false;
                nested_block_content.clear();
                nested_block_name.clear();
                continue;
            }
            
            if in_nested_block {
                nested_block_content.push_str(line);
                nested_block_content.push('\n');
                continue;
            }
            
            // Regular settings
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                match key {
                    "rounding" => section.rounding = value.parse()
                        .context(format!("Invalid rounding: {}", value))?,
                    "rounding_power" => { /* Parse value */ },
                    "active_opacity" => { /* Parse value */ },
                    "inactive_opacity" => { /* Parse value */ },
                    _ => debug!("Unknown decoration setting: {}", key),
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse the shadow subsection
    fn parse_shadow_section(content: &str) -> Result<()> {
        // Parse shadow settings
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                match key {
                    "enabled" => { /* Parse boolean */ },
                    "range" => { /* Parse value */ },
                    "render_power" => { /* Parse value */ },
                    "color" => { /* Parse color */ },
                    _ => debug!("Unknown shadow setting: {}", key),
                }
            }
        }
        
        Ok(())
    }
    
    /// Parse the blur subsection
    fn parse_blur_section(section: &mut DecorationSection, content: &str) -> Result<()> {
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                match key {
                    "enabled" => section.blur = value.parse()
                        .context(format!("Invalid blur enabled: {}", value))?,
                    "size" => section.blur_size = value.parse()
                        .context(format!("Invalid blur size: {}", value))?,
                    "passes" => section.blur_passes = value.parse()
                        .context(format!("Invalid blur passes: {}", value))?,
                    "vibrancy" => { /* Parse value */ },
                    _ => debug!("Unknown blur setting: {}", key),
                }
            }
        }
        
        Ok(())
    }
    
    // Add placeholders for other section parsers
    fn parse_animations_section(section: &mut AnimationsSection, content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
    
    fn parse_input_section(section: &mut InputSection, content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
    
    fn parse_gestures_section(section: &mut GesturesSection, content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
    
    fn parse_misc_section(section: &mut MiscSection, content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
    
    fn parse_dwindle_section(content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
    
    fn parse_master_section(content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
    
    fn parse_device_section(content: &str) -> Result<()> {
        // Placeholder - will implement details later
        Ok(())
    }
}

/// Find the default Hyprland config file
pub fn find_default_config() -> Option<String> {
    // Check common locations
    let possible_paths = [
        // User config directory
        dirs::config_dir().map(|p| p.join("hypr/hyprland.conf")),
        // Home directory
        dirs::home_dir().map(|p| p.join(".config/hypr/hyprland.conf")),
        // System config
        Some(Path::new("/etc/hypr/hyprland.conf").to_path_buf()),
    ];
    
    for path in possible_paths.iter().flatten() {
        if path.exists() {
            return path.to_str().map(String::from);
        }
    }
    
    None
}