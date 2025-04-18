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