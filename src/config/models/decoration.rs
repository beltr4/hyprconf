use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


/// Decoration section with blur and shadow subcategories
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecorationSection {
    pub rounding: i32,
    pub rounding_power: f32,
    pub active_opacity: f32,
    pub inactive_opacity: f32,
    pub fullscreen_opacity: f32,
    pub dim_inactive: bool,
    pub dim_strength: f32,
    pub dim_special: f32,
    pub dim_around: f32,
    pub screen_shader: String,
    pub border_part_of_window: bool,
    pub blur: BlurSection,
    pub shadow: ShadowSection,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShadowSection {
    pub enabled: bool,
    pub range: i32,
    pub render_power: i32,
    pub sharp: bool,
    pub ignore_window: bool,
    pub color: String,
    pub color_inactive: String,
    pub offset: (f32, f32),
    pub scale: f32,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlurSection {
    pub enabled: bool,
    pub size: i32,
    pub passes: i32,
    pub ignore_opacity: bool,
    pub new_optimizations: bool,
    pub xray: bool,
    pub noise: f32,
    pub contrast: f32,
    pub brightness: f32,
    pub vibrancy: f32,
    pub vibrancy_darkness: f32,
    pub special: bool,
    pub popups: bool,
    pub popups_ignorealpha: f32,
    pub input_methods: bool,
    pub input_methods_ignorealpha: f32,
}

pub fn parse_decoration_section(section: &mut DecorationSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if line.contains('{') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "rounding" => section.rounding = value.parse().unwrap_or(0),
                "rounding_power" => section.rounding_power = value.parse().unwrap_or(2.0),
                "active_opacity" => section.active_opacity = value.parse().unwrap_or(1.0),
                "inactive_opacity" => section.inactive_opacity = value.parse().unwrap_or(1.0),
                "fullscreen_opacity" => section.fullscreen_opacity = value.parse().unwrap_or(1.0),
                "dim_inactive" => section.dim_inactive = parse_bool(value),
                "dim_strength" => section.dim_strength = value.parse().unwrap_or(0.5),
                "dim_special" => section.dim_special = value.parse().unwrap_or(0.2),
                "dim_around" => section.dim_around = value.parse().unwrap_or(0.4),
                "screen_shader" => section.screen_shader = value.to_string(),
                "border_part_of_window" => section.border_part_of_window = parse_bool(value),
                _ => debug!("Unknown decoration setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_decoration_section<W: Write>(
    writer: &mut W,
    decoration: &DecorationSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "decoration {", comment_style)?;
    
    // Write main decoration options
    write_option(writer, " rounding", &decoration.rounding.to_string(), None, comment_style)?;
    write_option(writer, " rounding_power", &decoration.rounding_power.to_string(), None, comment_style)?;
    write_option(writer, " active_opacity", &decoration.active_opacity.to_string(), None, comment_style)?;
    write_option(writer, " inactive_opacity", &decoration.inactive_opacity.to_string(), None, comment_style)?;
    write_option(writer, " fullscreen_opacity", &decoration.fullscreen_opacity.to_string(), None, comment_style)?;
    write_boolean_option(writer, " dim_inactive", decoration.dim_inactive, None, comment_style)?;
    write_option(writer, " dim_strength", &decoration.dim_strength.to_string(), None, comment_style)?;
    write_option(writer, " dim_special", &decoration.dim_special.to_string(), None, comment_style)?;
    write_option(writer, " dim_around", &decoration.dim_around.to_string(), None, comment_style)?;
    write_option(writer, " screen_shader", &decoration.screen_shader, None, comment_style)?;
    write_boolean_option(writer, " border_part_of_window", decoration.border_part_of_window, None, comment_style)?;
    
    // Write the blur subsection using the dedicated blur writer
    write_blur_section(writer, &decoration.blur, comment_style)?;
    
    // Write the shadow subsection using the dedicated shadow writer
    write_shadow_section(writer, &decoration.shadow, comment_style)?;
    
    writeln!(writer, "}}")?;
    
    Ok(())
}



pub fn parse_blur_section(section: &mut BlurSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "enabled" => section.enabled = parse_bool(value),
                "size" => section.size = value.parse().unwrap_or(8),
                "passes" => section.passes = value.parse().unwrap_or(1),
                "ignore_opacity" => section.ignore_opacity = parse_bool(value),
                "new_optimizations" => section.new_optimizations = parse_bool(value),
                "xray" => section.xray = parse_bool(value),
                "noise" => section.noise = value.parse().unwrap_or(0.0117),
                "contrast" => section.contrast = value.parse().unwrap_or(0.8916),
                "brightness" => section.brightness = value.parse().unwrap_or(0.8172),
                "vibrancy" => section.vibrancy = value.parse().unwrap_or(0.1696),
                "vibrancy_darkness" => section.vibrancy_darkness = value.parse().unwrap_or(0.0),
                "special" => section.special = parse_bool(value),
                "popups" => section.popups = parse_bool(value),
                "popups_ignorealpha" => section.popups_ignorealpha = value.parse().unwrap_or(0.2),
                "input_methods" => section.input_methods = parse_bool(value),
                "input_methods_ignorealpha" => section.input_methods_ignorealpha = value.parse().unwrap_or(0.2),
                _ => debug!("Unknown blur setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_blur_section<W: Write>(
    writer: &mut W,
    blur: &BlurSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "blur {", comment_style)?;
    
    // All fields are now required (non-Option) according to the model
    write_boolean_option(writer, " enabled", blur.enabled, None, comment_style)?;
    write_option(writer, " size", &blur.size.to_string(), None, comment_style)?;
    write_option(writer, " passes", &blur.passes.to_string(), None, comment_style)?;
    write_boolean_option(writer, " ignore_opacity", blur.ignore_opacity, None, comment_style)?;
    write_boolean_option(writer, " new_optimizations", blur.new_optimizations, None, comment_style)?;
    write_boolean_option(writer, " xray", blur.xray, None, comment_style)?;
    write_option(writer, " noise", &blur.noise.to_string(), None, comment_style)?;
    write_option(writer, " contrast", &blur.contrast.to_string(), None, comment_style)?;
    write_option(writer, " brightness", &blur.brightness.to_string(), None, comment_style)?;
    
    // New fields
    write_option(writer, " vibrancy", &blur.vibrancy.to_string(), None, comment_style)?;
    write_option(writer, " vibrancy_darkness", &blur.vibrancy_darkness.to_string(), None, comment_style)?;
    write_boolean_option(writer, " special", blur.special, None, comment_style)?;
    write_boolean_option(writer, " popups", blur.popups, None, comment_style)?;
    write_option(writer, " popups_ignorealpha", &blur.popups_ignorealpha.to_string(), None, comment_style)?;
    write_boolean_option(writer, " input_methods", blur.input_methods, None, comment_style)?;
    write_option(writer, " input_methods_ignorealpha", &blur.input_methods_ignorealpha.to_string(), None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}


pub fn parse_shadow_section(section: &mut ShadowSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "enabled" => section.enabled = parse_bool(value),
                "range" => section.range = value.parse().unwrap_or(4),
                "render_power" => section.render_power = value.parse().unwrap_or(3),
                "sharp" => section.sharp = parse_bool(value),
                "ignore_window" => section.ignore_window = parse_bool(value),
                "color" => section.color = value.to_string(),
                "color_inactive" => section.color_inactive = value.to_string(),
                "scale" => section.scale = value.parse().unwrap_or(1.0),
                "offset" => {
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse().unwrap_or(0.0);
                        let y = parts[1].parse().unwrap_or(0.0);
                        section.offset = (x, y);
                    }
                },
                _ => debug!("Unknown shadow setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_shadow_section<W: Write>(
    writer: &mut W,
    shadow: &ShadowSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "shadow {", comment_style)?;
    
    write_boolean_option(writer, " enabled", shadow.enabled, None, comment_style)?;
    write_option(writer, " range", &shadow.range.to_string(), None, comment_style)?;
    write_option(writer, " render_power", &shadow.render_power.to_string(), None, comment_style)?;
    write_boolean_option(writer, " sharp", shadow.sharp, None, comment_style)?;
    write_boolean_option(writer, " ignore_window", shadow.ignore_window, None, comment_style)?;
    write_option(writer, " color", &shadow.color, None, comment_style)?;
    write_option(writer, " color_inactive", &shadow.color_inactive, None, comment_style)?;
    write_option(writer, " offset", &format!("{} {}", shadow.offset.0, shadow.offset.1), None, comment_style)?;
    write_option(writer, " scale", &shadow.scale.to_string(), None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}