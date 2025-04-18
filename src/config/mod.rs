pub mod models;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests;

use std::path::Path;
use std::io;

pub struct ConfigManager {
    config: models::HyprlandConfig,
    current_path: Option<String>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            config: models::HyprlandConfig::default(),
            current_path: None,
        }
    }
    
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let parser = parser::core::ConfigParser::new();
        let config = parser.parse_file(path.as_ref())?;
        
        Ok(Self {
            config,
            current_path: Some(path.as_ref().to_string_lossy().to_string()),
        })
    }
    
    pub fn from_string(content: &str) -> io::Result<Self> {
        let parser = parser::core::ConfigParser::new();
        let config = parser.parse_str(content)?;
        
        Ok(Self {
            config,
            current_path: None,
        })
    }
    
    pub fn get_config(&self) -> &models::HyprlandConfig {
        &self.config
    }
    
    pub fn get_config_mut(&mut self) -> &mut models::HyprlandConfig {
        &mut self.config
    }
    
    pub fn get_current_path(&self) -> Option<&String> {
        self.current_path.as_ref()
    }
    
    pub fn save(&self) -> io::Result<()> {
        if let Some(path) = &self.current_path {
            self.save_to(path)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No path set for saving"))
        }
    }
    
    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let writer = writer::core::ConfigWriter::new(self.config.clone());
        writer.write_to_file(path)
    }
    
    pub fn set_path<P: AsRef<Path>>(&mut self, path: P) {
        self.current_path = Some(path.as_ref().to_string_lossy().to_string());
    }
    
    pub fn generate_content(&self) -> io::Result<String> {
        let writer = writer::core::ConfigWriter::new(self.config.clone());
        writer.generate_config_content()
    }
}