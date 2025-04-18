use gtk::{gio, glib};
use std::env;
use std::path::PathBuf;

pub fn build_app() -> gtk::Application {
    let application = gtk::Application::new(
        Some("com.github.hyprconf"),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    application.connect_activate(move |app| {
        // Create the UI window when the app is activated
        crate::ui::window::AppWindow::new(app);
    });

    application
}

pub fn get_default_config_path() -> PathBuf {
    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        let path = PathBuf::from(xdg_config_home).join("hypr/hyprland.conf");
        if path.exists() {
            return path;
        }
    }
    
    let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
    let path = PathBuf::from(home).join(".config/hypr/hyprland.conf");
    
    path
}

pub fn get_config_backup_dir() -> PathBuf {
    if let Ok(xdg_cache_home) = env::var("XDG_CACHE_HOME") {
        return PathBuf::from(xdg_cache_home).join("hyprconf/backups");
    }
    
    let home = env::var("HOME").unwrap_or_else(|_| String::from("."));
    PathBuf::from(home).join(".cache/hyprconf/backups")
}

pub struct AppState {
    config_manager: crate::config::ConfigManager,
    is_modified: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            config_manager: crate::config::ConfigManager::new(),
            is_modified: false,
        }
    }
    
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let config_manager = crate::config::ConfigManager::from_file(path)?;
        
        Ok(Self {
            config_manager,
            is_modified: false,
        })
    }
    
    pub fn get_config(&self) -> &crate::config::models::HyprlandConfig {
        self.config_manager.get_config()
    }
    
    pub fn get_config_mut(&mut self) -> &mut crate::config::models::HyprlandConfig {
        self.is_modified = true;
        self.config_manager.get_config_mut()
    }
    
    pub fn save(&self) -> Result<(), std::io::Error> {
        self.config_manager.save()
    }
    
    pub fn save_to(&self, path: &str) -> Result<(), std::io::Error> {
        self.config_manager.save_to(path)
    }
    
    pub fn is_modified(&self) -> bool {
        self.is_modified
    }
    
    pub fn set_modified(&mut self, modified: bool) {
        self.is_modified = modified;
    }
    
    pub fn get_current_path(&self) -> Option<&String> {
        self.config_manager.get_current_path()
    }
    
    pub fn set_path(&mut self, path: &str) {
        self.config_manager.set_path(path);
    }
    
    pub fn create_backup(&self) -> Result<PathBuf, std::io::Error> {
        use std::fs;
        use chrono::Local;
        
        let backup_dir = get_config_backup_dir();
        fs::create_dir_all(&backup_dir)?;
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let file_name = format!("hyprland_{}.conf.bak", timestamp);
        let backup_path = backup_dir.join(file_name);
        
        self.save_to(&backup_path.to_string_lossy())?;
        
        Ok(backup_path)
    }
}