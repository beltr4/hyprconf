use crate::config::models::RenderSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

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