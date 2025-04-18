use gtk::prelude::*;
use gtk::{self, ApplicationWindow, HeaderBar, Stack, StackSwitcher, Box as GtkBox, Orientation};
use log::debug;

use crate::config::models::HyprlandConfig;

pub struct MainWindow {
    pub window: ApplicationWindow,
    config: HyprlandConfig,
}

impl MainWindow {
    pub fn new(app: &gtk::Application) -> Self {
        debug!("Creating main window");
        
        // Create the window
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hyprland Configuration")
            .default_width(900)
            .default_height(700)
            .build();
        
        // Create a header bar with a stack switcher for tabs
        let header = HeaderBar::new();
        
        // Create a stack for our tab content
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        stack.set_transition_duration(200);
        
        // Create a stack switcher (tabs) linked to our stack
        let stack_switcher = StackSwitcher::new();
        stack_switcher.set_stack(Some(&stack));
        header.set_title_widget(Some(&stack_switcher));
        
        window.set_titlebar(Some(&header));
        
        // Create tab content
        Self::create_general_tab(&stack);
        Self::create_decoration_tab(&stack);
        Self::create_input_tab(&stack);
        Self::create_animations_tab(&stack);
        // Add more tabs as needed
        
        // Add the stack to the window
        window.set_child(Some(&stack));
        
        Self {
            window,
            config: HyprlandConfig::default(), // Start with defaults
        }
    }
    
    /// Create the General tab
    fn create_general_tab(stack: &Stack) {
        let content = GtkBox::new(Orientation::Vertical, 10);
        content.set_margin_top(20);
        content.set_margin_bottom(20);
        content.set_margin_start(20);
        content.set_margin_end(20);

        // Add a placeholder label
        let label = gtk::Label::new(Some("General Settings (placeholder)"));
        content.append(&label);
        
        stack.add_titled(&content, Some("general"), "General");
    }
    
    /// Create the Decoration tab
    fn create_decoration_tab(stack: &Stack) {
        let content = GtkBox::new(Orientation::Vertical, 10);
        content.set_margin_top(20);
        content.set_margin_bottom(20);
        content.set_margin_start(20);
        content.set_margin_end(20);
        
        // Add a placeholder label
        let label = gtk::Label::new(Some("Decoration Settings (placeholder)"));
        content.append(&label);
        
        stack.add_titled(&content, Some("decoration"), "Decoration");
    }
    
    /// Create the Input tab
    fn create_input_tab(stack: &Stack) {
        let content = GtkBox::new(Orientation::Vertical, 10);
        content.set_margin_top(20);
        content.set_margin_bottom(20);
        content.set_margin_start(20);
        content.set_margin_end(20);
        
        // Add a placeholder label
        let label = gtk::Label::new(Some("Input Settings (placeholder)"));
        content.append(&label);
        
        stack.add_titled(&content, Some("input"), "Input");
    }
    
    /// Create the Animations tab
    fn create_animations_tab(stack: &Stack) {
        let content = GtkBox::new(Orientation::Vertical, 10);
        content.set_margin_top(20);
        content.set_margin_bottom(20);
        content.set_margin_start(20);
        content.set_margin_end(20);
        
        // Add a placeholder label
        let label = gtk::Label::new(Some("Animation Settings (placeholder)"));
        content.append(&label);
        
        stack.add_titled(&content, Some("animations"), "Animations");
    }
    
    /// Load configuration from a file path
    pub fn load_config(&mut self, path: &str) -> anyhow::Result<()> {
        use crate::config::parser::ConfigParser;
        
        self.config = ConfigParser::parse_file(path)?;
        // TODO: Update UI with loaded values
        
        Ok(())
    }
    
    /// Show the window
    pub fn present(&self) {
        self.window.present();
    }
}