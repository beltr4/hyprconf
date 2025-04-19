use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use crate::config::utils::{CommentStyle};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WindowRule {
    pub rule: String,
    pub value: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceRule {
    pub workspace: String,
    pub rules: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayerRule {
    pub rule: String,
    pub target: String,
    pub value: Option<String>,
}

pub fn parse_layer_rule(content: &str) -> Result<LayerRule> {
    let parts: Vec<&str> = content.split(',').collect();
    let mut rule = LayerRule::default();
    if parts.len() >= 2 {
        rule.rule = parts[0].trim().to_string();
        rule.target = parts[1].trim().to_string();
        if parts.len() > 2 {
            rule.value = Some(parts[2].trim().to_string());
        }
    }
    Ok(rule)
}

pub fn parse_window_rule(content: &str) -> Result<WindowRule> {
    let parts: Vec<&str> = content.splitn(3, ',').collect();
    let mut rule = WindowRule::default();
    if parts.len() >= 2 {
        rule.rule = parts[0].trim().to_string();
        rule.value = parts[1].trim().to_string();
        if parts.len() > 2 {
            rule.parameters = parts[2]
                .trim()
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
        }
    }
    Ok(rule)
}

pub fn parse_workspace_rule(content: &str) -> Result<WorkspaceRule> {
    let parts: Vec<&str> = content.split(',').collect();
    let mut rule = WorkspaceRule::default();
    if !parts.is_empty() {
        rule.workspace = parts[0].trim().to_string();
        for part in &parts[1..] {
            if let Some((key, value)) = part.trim().split_once(':') {
                rule.rules.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    Ok(rule)
}

/// Write a window rule to the provided writer
pub fn write_window_rule<W: Write>(
    writer: &mut W,
    rule: &WindowRule,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    let mut rule_str = format!("windowrule = {},{}", rule.rule, rule.value);
    
    // Add any additional parameters
    for param in &rule.parameters {
        rule_str.push_str(&format!(",{}", param));
    }
    
    // Simply write the rule without comment handling
    writeln!(writer, "{}", rule_str)?;
    
    Ok(())
}

/// Write a workspace rule to the provided writer
pub fn write_workspace_rule<W: Write>(
    writer: &mut W,
    rule: &WorkspaceRule,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    let mut workspace_str = format!("workspace = {}", rule.workspace);
    
    // Add rules
    for (key, value) in &rule.rules {
        workspace_str.push_str(&format!(",{}:{}", key, value));
    }
    
    // Simply write the rule without comment handling
    writeln!(writer, "{}", workspace_str)?;
    
    Ok(())
}

/// Write a layer rule to the provided writer
pub fn write_layer_rule<W: Write>(
    writer: &mut W,
    rule: &LayerRule,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    // Handle the Option<String> value correctly with the correct field names
    let value_str = match &rule.value {
        Some(v) => format!(",{}", v),
        None => String::new(),
    };
    
    let layer_str = format!("layerrule = {},{}{}", rule.rule, rule.target, value_str);
    
    // Simply write the rule without comment handling
    writeln!(writer, "{}", layer_str)?;
    
    Ok(())
}

/// Implement ConfigSection for Vec<WindowRule>
impl crate::config::models::core::ConfigSection for Vec<WindowRule> {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for rule in self {
            write_window_rule(writer, rule, comment_style)?;
        }
        Ok(())
    }
}

/// Implement ConfigSection for Vec<WorkspaceRule>
impl crate::config::models::core::ConfigSection for Vec<WorkspaceRule> {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for rule in self {
            write_workspace_rule(writer, rule, comment_style)?;
        }
        Ok(())
    }
}

/// Implement ConfigSection for Vec<LayerRule>
impl crate::config::models::core::ConfigSection for Vec<LayerRule> {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for rule in self {
            write_layer_rule(writer, rule, comment_style)?;
        }
        Ok(())
    }
}