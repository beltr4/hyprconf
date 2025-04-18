use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, HeaderBar, Button, Stack, StackSwitcher, Label, Box as GtkBox};
use std::rc::Rc;
use std::cell::RefCell;
use std::path::Path;

use crate::app::{self, AppState};
//use crate::safety::backup::BackupManager;
use crate::ui::tabs::{
    general::GeneralTab,
    //decoration::DecorationTab,
    //animations::AnimationsTab,
    //input::InputTab,
    //gestures::GesturesTab,
};

pub struct AppWindow {
    window: ApplicationWindow,
    app_state: Rc<RefCell<AppState>>,
    save_button: Button,
}

impl AppWindow {
    pub fn new(app: &Application) -> Self {
        // Application state
        let app_state = Rc::new(RefCell::new(AppState::new()));
        // Load default config
        let default_path = app::get_default_config_path();
        if default_path.exists() {
            if let Ok(state) = AppState::from_file(default_path.to_string_lossy().as_ref()) {
                *app_state.borrow_mut() = state;
            }
        }

        // Window setup
        let window = ApplicationWindow::new(app);
        window.set_title("Hyprland Configuration");
        window.set_default_size(900, 600);

        // Backup manager (unused here, but created for later use)
        let backup_dir = app::get_config_backup_dir();
        //let _backup_manager = BackupManager::new(backup_dir);

        // Header bar with buttons
        let header = HeaderBar::new();
        header.set_show_close_button(true);
        header.set_title(Some("Hyprland Configuration"));
        let open_button = Button::with_label("Open");
        let apply_button = Button::with_label("Apply");
        let save_button = Button::with_label("Save");
        header.pack_start(&open_button);
        header.pack_end(&apply_button);
        header.pack_end(&save_button);

        // Stack and switcher
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(200);
        let switcher = StackSwitcher::new();
        switcher.set_stack(Some(&stack));

        // Tabs
        let general_tab   = GeneralTab::new(app_state.clone());
        //let decoration_tab= DecorationTab::new(app_state.clone());
        //let animations_tab= AnimationsTab::new(app_state.clone());
        //let input_tab     = InputTab::new(app_state.clone());
        //let gestures_tab  = GesturesTab::new(app_state.clone());

        stack.add_titled(general_tab.get_widget(), "general", "General");
        //stack.add_titled(decoration_tab.get_widget(), "decoration", "Decoration");
        //stack.add_titled(animations_tab.get_widget(), "animations", "Animations");
        //stack.add_titled(input_tab.get_widget(), "input", "Input");
        //stack.add_titled(gestures_tab.get_widget(), "gestures", "Gestures");

        // Main container
        let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&header, false, false, 0);
        let tabbox = GtkBox::new(gtk::Orientation::Horizontal, 0);
        tabbox.set_halign(gtk::Align::Center);
        tabbox.pack_start(&switcher, true, true, 0);
        vbox.pack_start(&tabbox, false, false, 10);
        vbox.pack_start(&stack, true, true, 0);

        // Set as window child
        window.set_child(Some(&vbox));

        // Save button handler
        {
            let state = app_state.clone();
            save_button.connect_clicked(move |_| {
                let st = state.borrow();
                if let Some(p) = st.get_current_path() {
                    if let Err(e) = st.save() {
                        eprintln!("Error saving: {}", e);
                    }
                } else {
                    // TODO: prompt for path
                }
            });
        }

        // Open button handler
        {
            let state = app_state.clone();
            let win   = window.clone();
            open_button.connect_clicked(move |_| {
                let chooser = gtk::FileChooserDialog::with_buttons(
                    Some("Open Hyprland Configuration"),
                    Some(&win),
                    gtk::FileChooserAction::Open,
                    &[
                        ("Cancel", gtk::ResponseType::Cancel),
                        ("Open",   gtk::ResponseType::Accept),
                    ],
                );
                
                if let Some(parent) = app::get_default_config_path().parent() {
                    chooser.set_current_folder(parent);
                }
                let filter = gtk::FileFilter::new();
                filter.set_name(Some(".conf files"));
                filter.add_pattern("*.conf");
                chooser.add_filter(&filter);

                let st = state.clone();
                chooser.connect_response(move |d, resp| {
                    if resp == gtk::ResponseType::Accept {
                        if let Some(f) = d.file().and_then(|f| f.path()) {
                            if let Some(p) = f.to_str() {
                                if let Ok(new_st) = AppState::from_file(p) {
                                    *st.borrow_mut() = new_st;
                                }
                            }
                        }
                    }
                    d.close();
                });
                chooser.show_all();
            });
        }

        // Apply button handler
        {
            let state = app_state.clone();
            apply_button.connect_clicked(move |_| {
                let st = state.borrow();
                if let Err(e) = st.create_backup() {
                    eprintln!("Backup failed: {}", e);
                    return;
                }
                if let Some(p) = st.get_current_path() {
                    if let Err(e) = st.save() {
                        eprintln!("Save failed: {}", e);
                    } else if let Err(e) = Self::reload_hyprland_config() {
                        eprintln!("Reload failed: {}", e);
                    }
                }
            });
        }

        window.show_all();
        Self { window, app_state, save_button }
    }

    pub fn run(&self) {
        self.window.show_all();
    }

    fn reload_hyprland_config() -> Result<(), std::io::Error> {
        let out = std::process::Command::new("hyprctl").arg("reload").output()?;
        if !out.status.success() {
            let err = String::from_utf8_lossy(&out.stderr);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, err.to_string()));
        }
        Ok(())
    }
}
