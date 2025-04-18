pub mod models;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::io;

/// Manages loading, editing, and saving a Hyprland configuration
pub struct ConfigManager {
    config: models::HyprlandConfig,
    current_path: Option<String>,
}

impl ConfigManager {
    /// Creates a new, empty manager with default config
    pub fn new() -> Self {
        Self {
            config: models::HyprlandConfig::default(),
            current_path: None,
        }
    }

    /// Loads config from a file path
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let config = parser::core::ConfigParser::parse_file(&path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(Self {
            config,
            current_path: Some(path.as_ref().to_string_lossy().into_owned()),
        })
    }

    /// Loads config from a raw string
    pub fn from_string(content: &str) -> io::Result<Self> {
        let config = parser::core::ConfigParser::parse_string(content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(Self {
            config,
            current_path: None,
        })
    }

    /// Returns a reference to the current config
    pub fn get_config(&self) -> &models::HyprlandConfig {
        &self.config
    }

    /// Returns a mutable reference to the current config
    pub fn get_config_mut(&mut self) -> &mut models::HyprlandConfig {
        &mut self.config
    }

    /// Returns the path from which the config was loaded, if any
    pub fn get_current_path(&self) -> Option<&String> {
        self.current_path.as_ref()
    }

    /// Saves the config back to the original path
    pub fn save(&self) -> io::Result<()> {
        if let Some(path) = &self.current_path {
            self.save_to(path)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No path set for saving"))
        }
    }

    /// Saves the config to a specific file path
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let writer = writer::core::ConfigWriter::new(self.config.clone());
        writer.write_to_file(path)
    }

    /// Sets or changes the path for saving
    pub fn set_path<P: AsRef<Path>>(&mut self, path: P) {
        self.current_path = Some(path.as_ref().to_string_lossy().into_owned());
    }

    /// Generates the config content as a string
    pub fn generate_content(&self) -> io::Result<String> {
        let writer = writer::core::ConfigWriter::new(self.config.clone());
        writer.generate_config_content()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}
