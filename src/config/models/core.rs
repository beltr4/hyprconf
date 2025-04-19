use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use crate::config::utils::CommentStyle;
use crate::config::models::{
    animations::AnimationsSection, binds::BindsSection, cursor::CursorSection, decoration::DecorationSection, debug::DebugSection, devices::DeviceConfig,
    dwindle::DwindleSection, ecosystem::EcosystemSection, experimental::ExperimentalSection, general::GeneralSection, gestures::GesturesSection,
    group::GroupSection, input::InputSection, binds::KeyBind, rules::LayerRule, master::MasterSection, misc::MiscSection, monitors::MonitorConfig,
    opengl::OpenGLSection, permissions::Permission, render::RenderSection, rules::WindowRule, rules::WorkspaceRule, xwayland::XWaylandSection,
};

/// Trait for all config sections that can be written to a writer
pub trait ConfigSection {
    fn write_section<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()>;
}

/// Represents a complete Hyprland configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HyprlandConfig {
    pub general: GeneralSection,
    pub decoration: DecorationSection,
    pub animations: AnimationsSection,
    pub input: InputSection,
    pub gestures: GesturesSection,
    pub group: GroupSection,
    pub misc: MiscSection,
    pub binds: BindsSection,
    pub xwayland: XWaylandSection,
    pub opengl: OpenGLSection,
    pub render: RenderSection,
    pub cursor: CursorSection,
    pub dwindle: DwindleSection,
    pub master: MasterSection,
    pub debug: DebugSection,
    pub monitors: Vec<MonitorConfig>,
    pub devices: Vec<DeviceConfig>,
    pub window_rules: Vec<WindowRule>,
    pub workspace_rules: Vec<WorkspaceRule>,
    pub layer_rules: Vec<LayerRule>,
    pub variables: HashMap<String, String>,
    pub environment_variables: HashMap<String, String>,
    pub autostart_programs: Vec<String>,
    pub bezier_curves: HashMap<String, String>,
    pub submap_definitions: HashMap<String, Vec<KeyBind>>,
    pub ecosystem: EcosystemSection,
    pub experimental: ExperimentalSection,
    pub permissions: Vec<Permission>,
}

impl HyprlandConfig {
    /// Writes the entire config to a writer
    pub fn write_to<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        // Write each section through its write_section function
        crate::config::models::animations::write_animations_section(writer, &self.animations, comment_style)?;
        crate::config::models::decoration::write_decoration_section(writer, &self.decoration, comment_style)?;
        crate::config::models::general::write_general_section(writer, &self.general, comment_style)?;
        crate::config::models::gestures::write_section(writer, &self.gestures, comment_style)?;
        crate::config::models::input::write_section(writer, &self.input, comment_style)?;
        crate::config::models::group::write_section(writer, &self.group, comment_style)?;
        crate::config::models::misc::write_section(writer, &self.misc, comment_style)?;
        crate::config::models::binds::write_section(writer, &self.binds, comment_style)?;
        crate::config::models::xwayland::write_section(writer, &self.xwayland, comment_style)?;
        crate::config::models::opengl::write_section(writer, &self.opengl, comment_style)?;
        crate::config::models::render::write_section(writer, &self.render, comment_style)?;
        crate::config::models::cursor::write_section(writer, &self.cursor, comment_style)?;
        crate::config::models::dwindle::write_section(writer, &self.dwindle, comment_style)?;
        crate::config::models::master::write_section(writer, &self.master, comment_style)?;
        crate::config::models::debug::write_section(writer, &self.debug, comment_style)?;
        crate::config::models::ecosystem::write_ecosystem_section(writer, &self.ecosystem, comment_style)?;
        crate::config::models::experimental::write_experimental_section(writer, &self.experimental, comment_style)?;
        
        // Write monitor configurations
        self.write_monitors(writer, comment_style)?;
        
        // Write device configurations
        self.write_devices(writer, comment_style)?;
        
        // Write rules
        self.write_window_rules(writer, comment_style)?;
        self.write_workspace_rules(writer, comment_style)?;
        self.write_layer_rules(writer, comment_style)?;
        
        // Write variables
        self.write_variables(writer, comment_style)?;
        
        // Write environment variables
        self.write_environment_variables(writer, comment_style)?;
        
        // Write autostart programs
        self.write_autostart_programs(writer, comment_style)?;
        
        // Write bezier curves
        self.write_bezier_curves(writer, comment_style)?;
        
        // Write submap definitions
        self.write_submap_definitions(writer, comment_style)?;
        
        // Write permissions
        self.write_permissions(writer, comment_style)?;
        
        Ok(())
    }
    
    /// Convenience method to write to a file
    pub fn write_to_file(&self, path: &str) -> io::Result<()> {
        // Use None to indicate no comments, or select a specific CommentStyle
        // variant if you want to include comments
        let comment_style = crate::config::utils::CommentStyle::Hash; 
        
        let mut file = File::create(path)?;
        self.write_to(&mut file, &comment_style)
    }
    
    // Helper methods for writing specific components
    
    fn write_monitors<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for monitor in &self.monitors {
            crate::config::models::monitors::write_monitor(writer, monitor, comment_style)?;
        }
        Ok(())
    }
    
    fn write_devices<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for device in &self.devices {
            crate::config::models::devices::write_device(writer, device, comment_style)?;
        }
        Ok(())
    }
    
    fn write_window_rules<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for rule in &self.window_rules {
            crate::config::models::rules::write_window_rule(writer, rule, comment_style)?;
        }
        Ok(())
    }
    
    fn write_workspace_rules<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for rule in &self.workspace_rules {
            crate::config::models::rules::write_workspace_rule(writer, rule, comment_style)?;
        }
        Ok(())
    }
    
    fn write_layer_rules<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for rule in &self.layer_rules {
            crate::config::models::rules::write_layer_rule(writer, rule, comment_style)?;
        }
        Ok(())
    }
    
    fn write_variables<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        if !self.variables.is_empty() {
            writeln!(writer)?;
            for (name, value) in &self.variables {
                writeln!(writer, "${} = {}", name, value)?;
            }
        }
        Ok(())
    }
    
    fn write_environment_variables<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        if !self.environment_variables.is_empty() {
            writeln!(writer)?;
            for (name, value) in &self.environment_variables {
                writeln!(writer, "env = {}={}", name, value)?;
            }
        }
        Ok(())
    }
    
    fn write_autostart_programs<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        if !self.autostart_programs.is_empty() {
            writeln!(writer)?;
            for program in &self.autostart_programs {
                writeln!(writer, "exec-once = {}", program)?;
            }
        }
        Ok(())
    }
    
    fn write_bezier_curves<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        if !self.bezier_curves.is_empty() {
            writeln!(writer)?;
            for (name, curve) in &self.bezier_curves {
                writeln!(writer, "bezier = {},{}", name, curve)?;
            }
        }
        Ok(())
    }
    
    fn write_submap_definitions<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        for (name, binds) in &self.submap_definitions {
            writeln!(writer, "\nsubmap = {}", name)?;
            for bind in binds {
                crate::config::models::binds::write_key_bind(writer, bind, comment_style)?;
            }
            writeln!(writer, "submap = reset")?;
        }
        Ok(())
    }
    
    fn write_permissions<W: Write>(&self, writer: &mut W, comment_style: &CommentStyle) -> io::Result<()> {
        if !self.permissions.is_empty() {
            writeln!(writer)?;
            for permission in &self.permissions {
                crate::config::models::permissions::write_permission(writer, permission, comment_style)?;
            }
        }
        Ok(())
    }
}

/// Hyprland socket event type for reactive scripting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HyprlandEvent {
    WorkspaceChanged(String),
    FocusedMon(String, String), // Monitor name, workspace name
    ActiveWindow(String, String), // Window address, window title
    FullscreenState(String), // Window address
    MonitorRemoved(String), // Monitor name
    MonitorAdded(String), // Monitor name
    CreateWorkspace(String), // Workspace name
    DestroyWorkspace(String), // Workspace name
    MoveWorkspace(String, String), // Workspace name, monitor name
    ActiveLayout(String, String), // Keyboard name, layout name
    ActiveSpecial(String), // Special workspace name
    UrgentWindow(String), // Window address
    Minimize(String, bool), // Window address, minimized state
    SubMap(String), // Submap name
    WindowClose(String), // Window address
    Unknown(String), // Any other events
}

/// Hyprctl batch command structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyprctlBatch {
    pub commands: Vec<String>,
}

impl HyprctlBatch {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }
    
    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }
    
    pub fn to_string(&self) -> String {
        self.commands.join(" ; ")
    }
}