use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RenderSection {
    pub explicit_sync: i32,
    pub explicit_sync_kms: i32,
    pub direct_scanout: i32,
    pub expand_undersized_textures: bool,
    pub xp_mode: bool,
    pub ctm_animation: i32,
    pub cm_fs_passthrough: i32,
    pub cm_enabled: bool,
}

pub fn parse_render_section(section: &mut RenderSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "explicit_sync" => section.explicit_sync = value.parse().unwrap_or(2),
                "explicit_sync_kms" => section.explicit_sync_kms = value.parse().unwrap_or(2),
                "direct_scanout" => section.direct_scanout = value.parse().unwrap_or(0),
                "expand_undersized_textures" => section.expand_undersized_textures = parse_bool(value),
                "xp_mode" => section.xp_mode = parse_bool(value),
                "ctm_animation" => section.ctm_animation = value.parse().unwrap_or(2),
                "cm_fs_passthrough" => section.cm_fs_passthrough = value.parse().unwrap_or(2),
                "cm_enabled" => section.cm_enabled = parse_bool(value),
                _ => debug!("Unknown render setting: {}", key),
            }
        }
    }
    
    Ok(())
}

pub fn write_section<W: Write>(
    writer: &mut W,
    render: &RenderSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "render {", comment_style)?;
    
    write_option(writer, " explicit_sync", &render.explicit_sync.to_string(), None, comment_style)?;
    write_option(writer, " explicit_sync_kms", &render.explicit_sync_kms.to_string(), None, comment_style)?;
    write_option(writer, " direct_scanout", &render.direct_scanout.to_string(), None, comment_style)?;
    write_boolean_option(writer, " expand_undersized_textures", render.expand_undersized_textures, None, comment_style)?;
    write_boolean_option(writer, " xp_mode", render.xp_mode, None, comment_style)?;
    write_option(writer, " ctm_animation", &render.ctm_animation.to_string(), None, comment_style)?;
    write_option(writer, " cm_fs_passthrough", &render.cm_fs_passthrough.to_string(), None, comment_style)?;
    write_boolean_option(writer, " cm_enabled", render.cm_enabled, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}