use gtk::prelude::*;
use gtk::{self, Application};
use log::{info, error};

use crate::config::parser;
use crate::ui::window::MainWindow;

pub struct HyprconfApp {
    app: Application,
}

impl HyprconfApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("org.hyprland.hyprconf")
            .build();
            
        let instance = Self { app };
        instance.connect_signals();
        instance
    }
    
    fn connect_signals(&self) {
        self.app.connect_activate(move |app| {
            info!("Application activated");
            
            // Create main window
            let mut main_window = MainWindow::new(app);
            
            // Try to load default config if available
            if let Some(config_path) = parser::find_default_config() {
                info!("Loading default config from: {}", config_path);
                if let Err(err) = main_window.load_config(&config_path) {
                    error!("Failed to load config: {}", err);
                    // TODO: Show error dialog
                }
            } else {
                info!("No default config found, using defaults");
            }
            
            // Show the window
            main_window.present();
        });
    }
    
    pub fn run(&self) -> i32 {
        self.app.run().into()
    }
}