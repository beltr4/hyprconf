use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::str::FromStr;
use anyhow::{Context, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::CommentStyle;
use crate::config::models::core::ConfigSection;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub path_regex: String,
    pub permission_type: String,
    pub mode: PermissionMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionMode {
    Allow,
    Ask,
    Deny,
}

impl Display for PermissionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PermissionMode::Allow => write!(f, "allow"),
            PermissionMode::Ask => write!(f, "ask"),
            PermissionMode::Deny => write!(f, "deny"),
        }
    }
}

impl FromStr for PermissionMode {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "allow" => Ok(PermissionMode::Allow),
            "ask" => Ok(PermissionMode::Ask),
            "deny" => Ok(PermissionMode::Deny),
            _ => Err(format!("Invalid permission mode: {}", s)),
        }
    }
}

/// Parse a permission block (TOML-style) into a Permission struct
pub fn parse_permission_section(content: &str) -> Result<Permission> {
    let mut path_regex = None;
    let mut permission_type = None;
    let mut mode = None;
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim();
            let val = val.trim().trim_matches('"');
            
            match key {
                "path_regex" => path_regex = Some(val.to_string()),
                "permission_type" => permission_type = Some(val.to_string()),
                "mode" => {
                    let parsed = PermissionMode::from_str(val)
                        .map_err(|e| anyhow::anyhow!("Invalid permission mode '{}': {}", val, e))?;
                    mode = Some(parsed);
                }
                other => debug!("Unknown permission key: {}", other),
            }
        }
    }
    
    let path_regex = path_regex.context("Missing 'path_regex' in permission block")?;
    let permission_type = permission_type.context("Missing 'permission_type' in permission block")?;
    let mode = mode.context("Missing or invalid 'mode' in permission block")?;
    
    Ok(Permission {
        path_regex,
        permission_type,
        mode,
    })
}

/// Write a permission to the provided writer in Hyprland's config format
pub fn write_permission<W: Write>(
    writer: &mut W,
    permission: &Permission,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    writeln!(writer, "permission {{")?;
    writeln!(writer, "    path_regex = \"{}\"", permission.path_regex)?;
    writeln!(writer, "    permission_type = \"{}\"", permission.permission_type)?;
    writeln!(writer, "    mode = {}", permission.mode)?;
    writeln!(writer, "}}")?;
    
    Ok(())
}

/// Implement ConfigSection for Vec<Permission>
impl ConfigSection for Vec<Permission> {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for permission in self {
            write_permission(writer, permission, comment_style)?;
        }
        Ok(())
    }
}