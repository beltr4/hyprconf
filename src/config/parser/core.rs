use std::fs;
use std::path::Path;
use std::collections::HashMap;
use anyhow::{Result, Context};
use log::{debug, info};

use crate::config::models::*;
use crate::config::parser::sections::{
    general, decoration, animations, input, gestures, group, misc, binds,
    xwayland, opengl, render, cursor, dwindle, master, debug,
    snap, blur, shadow, touch, keybinds,
    ecosystem, experimental, permissions,
};

pub struct ConfigParser;

impl ConfigParser {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<HyprlandConfig> {
        let content = fs::read_to_string(path)
            .context("Failed to read Hyprland config file")?;
        Self::parse_string(&content)
    }

    pub fn parse_string(content: &str) -> Result<HyprlandConfig> {
        debug!("Parsing Hyprland configuration");

        let mut config = HyprlandConfig::default();
        let mut variables = HashMap::new();
        let mut env_vars = HashMap::new();
        let mut autostart = Vec::new();

        let mut current_section: Option<String> = None;
        let mut nested_section: Option<String> = None;
        let mut in_block = false;
        let mut block_content = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }

            if line.starts_with('$') {
                if let Some((n,v)) = line.split_once('=') {
                    variables.insert(n.trim().to_string(), v.trim().to_string());
                }
                continue;
            }
            if line.starts_with("env = ") {
                if let Some((n,v)) = line[6..].split_once(',') {
                    env_vars.insert(n.trim().to_string(), v.trim().to_string());
                }
                continue;
            }
            if line.starts_with("exec-once = ") {
                autostart.push(line[13..].trim().to_string());
                continue;
            }
            if line.starts_with("source") { continue; }

            if line.contains('}') && in_block && nested_section.is_some() {
                if let (Some(sec), Some(sub)) = (&current_section, &nested_section) {
                    Self::process_nested_section_block(&mut config, sec, sub, &block_content)?;
                }
                nested_section = None;
                block_content.clear();
                continue;
            }
            if line.contains('}') && in_block {
                if let Some(sec) = &current_section {
                    Self::process_section_block(&mut config, sec, &block_content)?;
                }
                in_block = false;
                current_section = None;
                block_content.clear();
                continue;
            }
            if line.contains('{') && in_block {
                nested_section = Some(line.split('{').next().unwrap().trim().to_string());
                debug!("Entering nested section: {}:{}",
                    current_section.as_ref().unwrap_or(&String::new()),
                    nested_section.as_ref().unwrap());
                continue;
            }
            if line.contains('{') {
                current_section = Some(line.split('{').next().unwrap().trim().to_string());
                in_block = true;
                debug!("Entering section: {}", current_section.as_ref().unwrap());
                continue;
            }
            if in_block {
                block_content.push_str(line);
                block_content.push_str("\n");
                continue;
            }

            if line.starts_with("monitor=") {
                let parts: Vec<&str> = line[8..].split(',').collect();
                if parts.len() >= 4 {
                    let mut m = MonitorConfig::default();
                    m.name = parts[0].trim().to_string();
                    m.resolution = parts[1].trim().to_string();
                    m.position = parts[2].trim().to_string();
                    m.scale = parts[3].trim().parse().unwrap_or(1.0);
                    if let Some(t)  = parts.get(4)  { m.transform = t.trim().parse().ok(); }
                    if let Some(mi) = parts.get(5)  { m.mirror = Some(mi.trim().to_string()); }
                    if let Some(bd) = parts.get(6)  { m.bitdepth = bd.trim().parse().ok(); }
                    if let Some(cm) = parts.get(7)  { m.color_management = Some(cm.trim().to_string()); }
                    if let Some(sb) = parts.get(8)  { m.sdr_brightness = sb.trim().parse().ok(); }
                    if let Some(ss) = parts.get(9)  { m.sdr_saturation = ss.trim().parse().ok(); }
                    if let Some(v)  = parts.get(10) { m.vrr = v.trim().parse().ok(); }
                    if let Some(d)  = parts.get(11) { m.disable = d.trim().parse::<i32>().unwrap_or(0)!=0; }
                    if parts.len() >= 16 {
                        m.reserved_area = Some((
                            parts[12].trim().parse().unwrap_or(0),
                            parts[13].trim().parse().unwrap_or(0),
                            parts[14].trim().parse().unwrap_or(0),
                            parts[15].trim().parse().unwrap_or(0),
                        ));
                    }
                    config.monitors.push(m);
                }
                continue;
            }
            if line.starts_with("windowrule = ") {
                let mut it = line[14..].split(',').map(str::trim);
                if let (Some(r), Some(v)) = (it.next(), it.next()) {
                    let params = it.map(|s| s.to_string()).collect();
                    config.window_rules.push(WindowRule { rule: r.to_string(), value: v.to_string(), parameters: params });
                }
                continue;
            }
            if line.starts_with("workspace = ") {
                let mut it = line[12..].split(',').map(str::trim);
                if let Some(ws) = it.next() {
                    let mut map = HashMap::new();
                    for kv in it {
                        if let Some((k,v)) = kv.split_once('=') {
                            map.insert(k.trim().to_string(), v.trim().to_string());
                        }
                    }
                    config.workspace_rules.push(WorkspaceRule { workspace: ws.to_string(), rules: map });
                }
                continue;
            }
            if line.starts_with("bind") { continue; }
            if let Some((k,v)) = line.split_once('=') {
                debug!("Top-level setting: {} = {}", k.trim(), v.trim());
            }
        }

        config.variables = variables;
        config.environment_variables = env_vars;
        config.autostart_programs = autostart;
        Ok(config)
    }

    fn process_section_block(
        config: &mut HyprlandConfig,
        section: &str,
        content: &str
    ) -> Result<()> {
        match section {
            "general"    => general::parse_general_section(&mut config.general, content)?,
            "decoration" => decoration::parse_decoration_section(&mut config.decoration, content)?,
            "animations" => animations::parse_animations_section(&mut config.animations, content)?,
            "input"      => input::parse_input_section(&mut config.input, content)?,
            "gestures"   => gestures::parse_gestures_section(&mut config.gestures, content)?,
            "group"      => group::parse_group_section(&mut config.group, content)?,
            "misc"       => misc::parse_misc_section(&mut config.misc, content)?,
            "binds"      => binds::parse_binds_section(&mut config.binds, content)?,
            "xwayland"   => xwayland::parse_xwayland_section(&mut config.xwayland, content)?,
            "opengl"     => opengl::parse_opengl_section(&mut config.opengl, content)?,
            "render"     => render::parse_render_section(&mut config.render, content)?,
            "cursor"     => cursor::parse_cursor_section(&mut config.cursor, content)?,
            "dwindle"    => dwindle::parse_dwindle_section(&mut config.dwindle, content)?,
            "master"     => master::parse_master_section(&mut config.master, content)?,
            "debug"      => debug::parse_debug_section(&mut config.debug, content)?,
            "ecosystem"  => ecosystem::parse_ecosystem_section(&mut config.ecosystem, content)?,
            "experimental"=> experimental::parse_experimental_section(&mut config.experimental, content)?,
            "bezier_curves" => {
                config.bezier_curves = content.lines()
                    .filter_map(|l| l.trim().split_once('=').map(|(k,v)| (k.trim().to_string(), v.trim().to_string())))
                    .collect();
            }
            "submap_definitions" => {
                let mut m = HashMap::new();
                for l in content.lines().map(str::trim).filter(|l| !l.is_empty()) {
                    if let Some((name, binds_str)) = l.split_once('=') {
                        let vec = binds_str.split(';')
                            .filter_map(|e| keybinds::parse_key_bind(e.trim()).ok())
                            .collect::<Vec<_>>();
                        m.insert(name.trim().to_string(), vec);
                    }
                }
                config.submap_definitions = m;
            }
            "permission" => {
                let p = permissions::parse_permission_section(content)?;
                config.permissions.push(p);
            }
            _ => debug!("Unknown section: {}", section),
        }
        Ok(())
    }

    fn process_nested_section_block(
        config: &mut HyprlandConfig,
        section: &str,
        subsection: &str,
        content: &str
    ) -> Result<()> {
        match (section, subsection) {
            ("general", "snap") => snap::parse_snap_section(&mut config.general.snap, content)?,
            ("decoration", "blur") => blur::parse_blur_section(&mut config.decoration.blur, content)?,
            ("decoration", "shadow") => shadow::parse_shadow_section(&mut config.decoration.shadow, content)?,
            ("input", "touchpad") => touch::parse_touchpad_section(&mut config.input.touchpad, content)?,
            ("input", "touchdevice") => touch::parse_touchdevice_section(&mut config.input.touchdevice, content)?,
            ("input", "tablet") => touch::parse_tablet_section(&mut config.input.tablet, content)?,
            ("group", "groupbar") => group::parse_groupbar_section(&mut config.group.groupbar, content)?,
            _ => debug!("Unknown nested section: {}:{}", section, subsection),
        }
        Ok(())
    }
}
