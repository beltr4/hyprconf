use crate::config::models::{WindowRule, WorkspaceRule, LayerRule};
use std::collections::HashMap;
use anyhow::Result;
use log::debug;

/// Parse a window rule
pub fn parse_window_rule(content: &str) -> Result<WindowRule> {
    let parts: Vec<&str> = content.splitn(3, ',').collect();
    
    let mut rule = WindowRule::default();
    
    if parts.len() >= 2 {
        rule.rule = parts[0].trim().to_string();
        rule.value = parts[1].trim().to_string();
        
        // If there are additional parameters
        if parts.len() > 2 {
            let params_str = parts[2].trim();
            rule.parameters = params_str.split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
    }
    
    Ok(rule)
}

/// Parse a workspace rule
pub fn parse_workspace_rule(content: &str) -> Result<WorkspaceRule> {
    let parts: Vec<&str> = content.split(',').collect();
    
    let mut rule = WorkspaceRule::default();
    
    if !parts.is_empty() {
        rule.workspace = parts[0].trim().to_string();
        
        // Parse rule key-value pairs
        for i in 1..parts.len() {
            let part = parts[i].trim();
            if let Some((key, value)) = part.split_once(':') {
                rule.rules.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    
    Ok(rule)
}

/// Parse a layer rule
pub fn parse_layer_rule(content: &str) -> Result<LayerRule> {
    let parts: Vec<&str> = content.split(',').collect();
    
    let mut rule = LayerRule::default();
    
    if parts.len() >= 2 {
        rule.rule = parts[0].trim().to_string();
        rule.target = parts[1].trim().to_string();
        
        // If there's a value
        if parts.len() > 2 {
            rule.value = Some(parts[2].trim().to_string());
        }
    }
    
    Ok(rule)
}