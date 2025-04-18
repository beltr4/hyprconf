use gtk::prelude::*;
use gtk::{self, ApplicationWindow, HeaderBar, Stack, StackSwitcher};
use gtk::{Box as GtkBox, Button};
use glib::clone;
use log::{debug, info, warn};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config::models::HyprlandConfig;
use crate::config::parser::{ConfigParser, find_default_config};
use crate::ui::tabs::GeneralTab;
use anyhow::Result;

pub struct MainWindow {
    pub window: ApplicationWindow,
    config: Rc<RefCell<HyprlandConfig>>,
    tabs: TabControls,
}

struct TabControls {
    general: GeneralTab,
    // Add more tabs as they are implemented
}

impl MainWindow {
    pub fn new(app: &gtk::Application) -> Self {
        debug!("Creating main window");
        
        // Create the window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hyprland Configuration")
            .default_width(1000)
            .default_height(750)
            .build();
        
        // Create a header bar with a stack switcher for tabs
        let header = HeaderBar::new();
        
        // Add save and load buttons to the header
        let save_button = Button::with_label("Save");
        let load_button = Button::with_label("Load");
        header.pack_start(&load_button);
        header.pack_end(&save_button);
        
        // Create a stack for our tab content
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(200);
        
        // Create a stack switcher (tabs) linked to our stack
        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(Some(&stack));
        header.set_title_widget(Some(&stack_switcher));
        
        window.set_titlebar(Some(&header));
        
        // Create shared config
        let config = Rc::new(RefCell::new(HyprlandConfig::default()));
        
        // Initialize tab controls
        let tabs = TabControls {
            general: GeneralTab::new(),
            // Add more tabs as they are implemented
        };
        
        // Build tab UI
        tabs.general.build_ui(&stack);
        // Add more tabs as they are implemented
        
        // Add the stack to the window
        window.set_child(Some(&stack));
        
        let main_window = Self {
            window,
            config,
            tabs,
        };
        
        // Connect signals
        main_window.connect_signals(save_button, load_button);
        
        main_window
    }
    
    fn connect_signals(&self, save_button: Button, load_button: Button) {
        // Connect save button
        let config_clone = self.config.clone();
        save_button.connect_clicked(move |_| {
            let config = config_clone.borrow();
            // TODO: Implement save functionality
            debug!("Save button clicked");
        });
        
        // Connect load button
        let config_clone = self.config.clone();
        let window_clone = self.window.clone();
        let tabs = &self.tabs;
        
        load_button.connect_clicked(move |_| {
            if let Some(path) = find_default_config() {
                match ConfigParser::parse_file(path) {
                    Ok(new_config) => {
                        // Update config
                        *config_clone.borrow_mut() = new_config;
                        
                        // Update UI with new config
                        tabs.general.update_from_config(&config_clone.borrow().general);
                        // Update other tabs as needed
                        
                        debug!("Config loaded successfully");
                    },
                    Err(e) => {
                        warn!("Failed to load config: {}", e);
                        // TODO: Show error dialog
                    }
                }
            }
        });
        
        // Connect change handlers from tabs to update config
        let config_clone = self.config.clone();
        self.tabs.general.connect_signals(move |field, value| {
            let mut config = config_clone.borrow_mut();
            match field {
                "border_size" => if let Some(val) = value.downcast_ref::<i32>() {
                    config.general.border_size = *val;
                },
                "gaps_in" => if let Some(val) = value.downcast_ref::<String>() {
                    config.general.gaps_in = val.clone();
                },
                // Define other field handlers
                _ => warn!("Unknown field: {}", field),
            }
        });
    }
    
    /// Load configuration from a file path
    pub fn load_config(&self, path: &str) -> Result<()> {
        let new_config = ConfigParser::parse_file(path)?;
        *self.config.borrow_mut() = new_config;
        self.update_ui_from_config();
        
        Ok(())
    }
    
    /// Update the UI widgets from the loaded config
    fn update_ui_from_config(&self) {
        let config = self.config.borrow();
        
        // Update general tab
        self.tabs.general.update_from_config(&config.general);
        // Update other tabs as they are implemented
    }
    
    /// Show the window
    pub fn present(&self) {
        self.window.present();
    }
}