use crate::config::models::KeyBind;
use anyhow::Result;
use log::debug;

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
    if let Some((arg_part, meta)) = arg.split_once('#') {
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