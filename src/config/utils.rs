use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
pub enum CommentStyle {
    Hash,      // # comment
    SlashSlash, // // comment
}

impl CommentStyle {
    pub fn prefix(&self) -> &'static str {
        match self {
            CommentStyle::Hash => "#",
            CommentStyle::SlashSlash => "//",
        }
    }
}

pub fn write_section_header<W: Write>(
    writer: &mut W,
    section_name: &str,
    _comment_style: &CommentStyle,
) -> io::Result<()> {
    writeln!(writer)?;
    writeln!(writer, "{}", section_name)?;
    Ok(())
}

pub fn write_option<W: Write, T: fmt::Display>(
    writer: &mut W,
    option_name: &str,
    value: &T,
    comment: Option<&str>,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    match comment {
        Some(c) => writeln!(writer, "{} = {} {} {}", option_name, value, comment_style.prefix(), c),
        None => writeln!(writer, "{} = {}", option_name, value),
    }
}

pub fn write_boolean_option<W: Write>(
    writer: &mut W,
    option_name: &str,
    value: bool,
    comment: Option<&str>,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    let value_str = if value { "yes" } else { "no" };
    write_option(writer, option_name, &value_str, comment, comment_style)
}

pub fn write_rgb_color<W: Write>(
    writer: &mut W,
    option_name: &str,
    r: u8,
    g: u8,
    b: u8,
    comment: Option<&str>,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    let rgb = format!("rgb({},{},{})", r, g, b);
    write_option(writer, option_name, &rgb, comment, comment_style)
}

pub fn write_rgba_color<W: Write>(
    writer: &mut W,
    option_name: &str,
    r: u8,
    g: u8,
    b: u8,
    a: f32,
    comment: Option<&str>,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    let rgba = format!("rgba({},{},{},{})", r, g, b, a);
    write_option(writer, option_name, &rgba, comment, comment_style)
}

use std::path::Path;
use dirs;

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
/// Helper function to parse a boolean value
pub fn parse_bool(value: &str) -> bool {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" => true,
        // Special case for animations section which uses "yes, please" or similar
        s if s.contains("yes") => true,
        _ => false,
    }
}

