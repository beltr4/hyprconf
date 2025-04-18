use gtk::prelude::*;
use gtk::{self, Application, ApplicationWindow, HeaderBar, Button, Stack, StackSwitcher, Label, Box};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

use crate::app::{self, AppState};
use crate::safety::backup::BackupManager;
use crate::ui::tabs::{general::GeneralTab, decoration::DecorationTab, animations::AnimationsTab, 
                     input::InputTab, gestures::GesturesTab};

pub struct AppWindow {
    window: ApplicationWindow,
    app_state: Rc<RefCell<AppState>>,
    save_button: Button,
}

impl AppWindow {
    pub fn new(app: &Application) -> Self {
        // Create the application state
        let app_state = Rc::new(RefCell::new(AppState::new()));
        
        // Try to load the default configuration
        let default_path = app::get_default_config_path();
        if default_path.exists() {
            if let Ok(loaded_state) = AppState::from_file(&default_path.to_string_lossy()) {
                *app_state.borrow_mut() = loaded_state;
            }
        }
        
        // Create the window
        let window = ApplicationWindow::new(app);
        window.set_title("Hyprland Configuration");
        window.set_default_size(900, 600);
        
        // Create a backup manager
        let backup_dir = app::get_config_backup_dir();
        let backup_manager = BackupManager::new(backup_dir);
        
        // Create the header bar
        let header_bar = HeaderBar::new();
        header_bar.set_show_close_button(true);
        header_bar.set_title(Some("Hyprland Configuration"));
        
        // Create buttons for the header bar
        let open_button = Button::with_label("Open");
        let save_button = Button::with_label("Save");
        let apply_button = Button::with_label("Apply");
        
        // Add buttons to the header bar
        header_bar.pack_start(&open_button);
        header_bar.pack_end(&apply_button);
        header_bar.pack_end(&save_button);
        
        // Create the stack for tab switching
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(200);
        
        // Create the stack switcher (tab bar)
        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(Some(&stack));
        
        // Create the tabs
        let general_tab = GeneralTab::new(app_state.clone());
        let decoration_tab = DecorationTab::new(app_state.clone());
        let animations_tab = AnimationsTab::new(app_state.clone());
        let input_tab = InputTab::new(app_state.clone());
        let gestures_tab = GesturesTab::new(app_state.clone());
        
        // Add the tabs to the stack
        stack.add_titled(general_tab.get_widget(), Some("general"), "General");
        stack.add_titled(decoration_tab.get_widget(), Some("decoration"), "Decoration");
        stack.add_titled(animations_tab.get_widget(), Some("animations"), "Animations");
        stack.add_titled(input_tab.get_widget(), Some("input"), "Input");
        stack.add_titled(gestures_tab.get_widget(), Some("gestures"), "Gestures");
        
        // Create the main vertical box
        let main_box = Box::new(gtk::Orientation::Vertical, 0);
        main_box.pack_start(&header_bar, false, false, 0);
        
        // Add the tab switcher to the main box
        let tab_box = Box::new(gtk::Orientation::Horizontal, 0);
        tab_box.set_halign(gtk::Align::Center);
        tab_box.pack_start(&stack_switcher, true, true, 0);
        main_box.pack_start(&tab_box, false, false, 10);
        
        // Add the stack to the main box
        main_box.pack_start(&stack, true, true, 0);
        
        // Add the main box to the window
        window.add(&main_box);
        
        // Connect the save button
        let app_state_clone = app_state.clone();
        save_button.connect_clicked(move |_| {
            let state = app_state_clone.borrow();
            
            if let Some(path) = state.get_current_path() {
                match state.save() {
                    Ok(_) => {
                        println!("Configuration saved to {}", path);
                    }
                    Err(e) => {
                        println!("Error saving configuration: {}", e);
                    }
                }
            } else {
                println!("No path set for saving");
                // Here you would typically open a file chooser dialog
            }
        });
        
        // Connect the open button
        let app_state_clone = app_state.clone();
        let window_clone = window.clone();
        open_button.connect_clicked(move |_| {
            let file_chooser = gtk::FileChooserDialog::new(
                Some("Open Hyprland Configuration"),
                Some(&window_clone),
                gtk::FileChooserAction::Open,
                &[
                    ("Cancel", gtk::ResponseType::Cancel),
                    ("Open", gtk::ResponseType::Accept),
                ],
            );
            
            // Set the current folder to the default hyprland config location
            let default_path = app::get_default_config_path();
            if let Some(parent) = default_path.parent() {
                file_chooser.set_current_folder(parent);
            }
            
            // Add a filter for .conf files
            let filter = gtk::FileFilter::new();
            filter.set_name(Some("Configuration Files"));
            filter.add_pattern("*.conf");
            file_chooser.add_filter(&filter);
            
            let app_state_clone2 = app_state_clone.clone();
            file_chooser.connect_response(move |dialog, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(file_path) = dialog.file() {
                        if let Some(path) = file_path.path() {
                            if let Some(path_str) = path.to_str() {
                                match AppState::from_file(path_str) {
                                    Ok(state) => {
                                        *app_state_clone2.borrow_mut() = state;
                                        println!("Loaded configuration from {}", path_str);
                                    }
                                    Err(e) => {
                                        println!("Error loading configuration: {}", e);
                                    }
                                }
                            }
                        }
                    }
                }
                dialog.close();
            });
            
            file_chooser.show_all();
        });
        
        // Connect the apply button
        let app_state_clone = app_state.clone();
        apply_button.connect_clicked(move |_| {
            let state = app_state_clone.borrow();
            
            // Create a backup before applying changes
            match state.create_backup() {
                Ok(backup_path) => {
                    println!("Created backup at {}", backup_path.display());
                },
                Err(e) => {
                    println!("Failed to create backup: {}", e);
                    // You might want to show a dialog here
                    return;
                }
            }
            
            // Save the configuration to the current path
            if let Some(path) = state.get_current_path() {
                match state.save() {
                    Ok(_) => {
                        println!("Configuration applied to {}", path);
                        
                        // Reload Hyprland configuration
                        if let Err(e) = Self::reload_hyprland_config() {
                            println!("Error reloading Hyprland configuration: {}", e);
                        }
                    }
                    Err(e) => {
                        println!("Error applying configuration: {}", e);
                    }
                }
            } else {
                println!("No path set for applying");
            }
        });
        
        // Show all widgets
        window.show_all();
        
        Self {
            window,
            app_state,
            save_button,
        }
    }
    
    pub fn run(&self) {
        self.window.show_all();
    }
    
    // Reload the Hyprland configuration using hyprctl
    fn reload_hyprland_config() -> Result<(), std::io::Error> {
        use std::process::Command;
        
        let output = Command::new("hyprctl")
            .arg("reload")
            .output()?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("hyprctl reload failed: {}", error),
            ));
        }
        
        Ok(())
    }
    
    // Method to update window title based on file path and modified status
    fn update_window_title(&self) {
        let state = self.app_state.borrow();
        let modified_marker = if state.is_modified() { "*" } else { "" };
        
        let title = if let Some(path) = state.get_current_path() {
            let filename = Path::new(path).file_name()
                .and_then(|f| f.to_str())
                .unwrap_or(path);
            format!("{}{} - Hyprland Configuration", filename, modified_marker)
        } else {
            format!("Untitled{} - Hyprland Configuration", modified_marker)
        };
        
        self.window.set_title(&title);
    }
    
    // Method to update the save button sensitivity based on modified status
    fn update_save_button(&self) {
        let state = self.app_state.borrow();
        self.save_button.set_sensitive(state.is_modified());
    }
}