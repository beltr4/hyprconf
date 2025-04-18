use crate::config::models::{Permission, PermissionMode};
use anyhow::{Result, Context};
use log::debug;
use std::str::FromStr;

/// Parse a permission block (TOML-style) into a Permission struct
pub fn parse_permission_section(content: &str) -> Result<Permission> {
    let mut path_regex = None;
    let mut permission_type = None;
    let mut mode = None;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim();
            let val = val.trim().trim_matches('"');
            match key {
                "path_regex"      => path_regex = Some(val.to_string()),
                "permission_type" => permission_type = Some(val.to_string()),
                "mode"            => {
                    // Directly map error into anyhow
                    let parsed = PermissionMode::from_str(val)
                        .map_err(|e| anyhow::anyhow!("Invalid permission mode '{}': {}", val, e))?;
                    mode = Some(parsed);
                }
                other => debug!("Unknown permission key: {}", other),
            }
        }
    }

    let path_regex = path_regex
        .context("Missing 'path_regex' in permission block")?;
    let permission_type = permission_type
        .context("Missing 'permission_type' in permission block")?;
    let mode = mode
        .context("Missing or invalid 'mode' in permission block")?;

    Ok(Permission { path_regex, permission_type, mode })
}