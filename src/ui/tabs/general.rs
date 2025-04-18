use gtk::prelude::*;
use gtk::{Stack, Box as GtkBox, Orientation, Label, Scale, Switch, ColorButton, Frame, ScrolledWindow, DropDown};
use crate::config::models::GeneralSection;

/// Struct to hold all widgets for the general tab
pub struct GeneralTab {
    pub border_size: Scale,
    pub gaps_in: Scale,
    pub gaps_out: Scale,
    pub gaps_workspaces: Scale,
    pub active_border_color: ColorButton,
    pub inactive_border_color: ColorButton,
    pub no_border_floating: Switch,
    pub resize_on_border: Switch,
    pub no_focus_fallback: Switch,
    pub allow_tearing: Switch,
    pub layout_combo: DropDown,
}

impl GeneralTab {
    /// Create a new instance of the general tab
    pub fn new() -> Self {
        // Initialize layout dropdown
        let layout_model = gtk::StringList::new(&["dwindle", "master"]);
        
        Self {
            border_size: Scale::new(Orientation::Horizontal, Some(&gtk::Adjustment::new(1.0, 0.0, 10.0, 1.0, 1.0, 0.0))),
            gaps_in: Scale::new(Orientation::Horizontal, Some(&gtk::Adjustment::new(5.0, 0.0, 50.0, 1.0, 5.0, 0.0))),
            gaps_out: Scale::new(Orientation::Horizontal, Some(&gtk::Adjustment::new(20.0, 0.0, 100.0, 1.0, 5.0, 0.0))),
            gaps_workspaces: Scale::new(Orientation::Horizontal, Some(&gtk::Adjustment::new(0.0, 0.0, 50.0, 1.0, 5.0, 0.0))),
            active_border_color: ColorButton::new(),
            inactive_border_color: ColorButton::new(),
            no_border_floating: Switch::new(),
            resize_on_border: Switch::new(),
            no_focus_fallback: Switch::new(),
            allow_tearing: Switch::new(),
            layout_combo: DropDown::new(Some(&layout_model), None),
        }
    }
    
    /// Build the general tab UI
    pub fn build_ui(&self, stack: &Stack) {
        let content = GtkBox::new(Orientation::Vertical, 10);
        content.set_margin_top(20);
        content.set_margin_bottom(20);
        content.set_margin_start(20);
        content.set_margin_end(20);
        
        // Add some description text
        let heading = Label::new(Some("General Settings"));
        heading.set_halign(gtk::Align::Start);
        heading.add_css_class("title-3");
        content.append(&heading);
        
        let description = Label::new(Some("Configure general window behavior, borders, and gaps."));
        description.set_halign(gtk::Align::Start);
        description.set_margin_bottom(10);
        content.append(&description);
        
        // Create a scrolled window to contain all settings
        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        
        let settings_box = GtkBox::new(Orientation::Vertical, 15);
        settings_box.set_margin_top(10);
        settings_box.set_margin_bottom(10);
        settings_box.set_margin_start(10);
        settings_box.set_margin_end(10);
        
        // Add borders section
        settings_box.append(&self.create_borders_section());
        
        // Add gaps section
        settings_box.append(&self.create_gaps_section());
        
        // Add behavior section
        settings_box.append(&self.create_behavior_section());
        
        // Add to scrolled window
        scrolled.set_child(Some(&settings_box));
        content.append(&scrolled);
        
        stack.add_titled(&content, Some("general"), "General");
    }
    
    /// Create the borders section
    fn create_borders_section(&self) -> Frame {
        let borders_frame = Frame::new(Some("Borders"));
        let borders_box = GtkBox::new(Orientation::Vertical, 10);
        borders_box.set_margin_top(10);
        borders_box.set_margin_bottom(10);
        borders_box.set_margin_start(10);
        borders_box.set_margin_end(10);
        
        // Border size
        let border_row = GtkBox::new(Orientation::Horizontal, 10);
        let border_label = Label::new(Some("Border Size:"));
        border_label.set_halign(gtk::Align::Start);
        border_label.set_hexpand(true);
        border_row.append(&border_label);
        border_row.append(&self.border_size);
        self.border_size.set_draw_value(true);
        self.border_size.set_value_pos(gtk::PositionType::Right);
        self.border_size.set_size_request(150, -1);
        borders_box.append(&border_row);
        
        // No border on floating
        let floating_border_row = GtkBox::new(Orientation::Horizontal, 10);
        let floating_border_label = Label::new(Some("No Border on Floating Windows:"));
        floating_border_label.set_halign(gtk::Align::Start);
        floating_border_label.set_hexpand(true);
        floating_border_row.append(&floating_border_label);
        floating_border_row.append(&self.no_border_floating);
        borders_box.append(&floating_border_row);
        
        // Active border color
        let active_color_row = GtkBox::new(Orientation::Horizontal, 10);
        let active_color_label = Label::new(Some("Active Border Color:"));
        active_color_label.set_halign(gtk::Align::Start);
        active_color_label.set_hexpand(true);
        active_color_row.append(&active_color_label);
        active_color_row.append(&self.active_border_color);
        borders_box.append(&active_color_row);
        
        // Inactive border color
        let inactive_color_row = GtkBox::new(Orientation::Horizontal, 10);
        let inactive_color_label = Label::new(Some("Inactive Border Color:"));
        inactive_color_label.set_halign(gtk::Align::Start);
        inactive_color_label.set_hexpand(true);
        inactive_color_row.append(&inactive_color_label);
        inactive_color_row.append(&self.inactive_border_color);
        borders_box.append(&inactive_color_row);
        
        // Add to frame
        borders_frame.set_child(Some(&borders_box));
        borders_frame
    }
    
    /// Create the gaps section
    fn create_gaps_section(&self) -> Frame {
        let gaps_frame = Frame::new(Some("Gaps"));
        let gaps_box = GtkBox::new(Orientation::Vertical, 10);
        gaps_box.set_margin_top(10);
        gaps_box.set_margin_bottom(10);
        gaps_box.set_margin_start(10);
        gaps_box.set_margin_end(10);
        
        // Gaps in
        let gaps_in_row = GtkBox::new(Orientation::Horizontal, 10);
        let gaps_in_label = Label::new(Some("Inner Gaps:"));
        gaps_in_label.set_halign(gtk::Align::Start);
        gaps_in_label.set_hexpand(true);
        gaps_in_row.append(&gaps_in_label);
        gaps_in_row.append(&self.gaps_in);
        self.gaps_in.set_draw_value(true);
        self.gaps_in.set_value_pos(gtk::PositionType::Right);
        self.gaps_in.set_size_request(150, -1);
        gaps_box.append(&gaps_in_row);
        
        // Gaps out
        let gaps_out_row = GtkBox::new(Orientation::Horizontal, 10);
        let gaps_out_label = Label::new(Some("Outer Gaps:"));
        gaps_out_label.set_halign(gtk::Align::Start);
        gaps_out_label.set_hexpand(true);
        gaps_out_row.append(&gaps_out_label);
        gaps_out_row.append(&self.gaps_out);
        self.gaps_out.set_draw_value(true);
        self.gaps_out.set_value_pos(gtk::PositionType::Right);
        self.gaps_out.set_size_request(150, -1);
        gaps_box.append(&gaps_out_row);
        
        // Gaps between workspaces
        let gaps_workspaces_row = GtkBox::new(Orientation::Horizontal, 10);
        let gaps_workspaces_label = Label::new(Some("Workspace Gaps:"));
        gaps_workspaces_label.set_halign(gtk::Align::Start);
        gaps_workspaces_label.set_hexpand(true);
        gaps_workspaces_row.append(&gaps_workspaces_label);
        gaps_workspaces_row.append(&self.gaps_workspaces);
        self.gaps_workspaces.set_draw_value(true);
        self.gaps_workspaces.set_value_pos(gtk::PositionType::Right);
        self.gaps_workspaces.set_size_request(150, -1);
        gaps_box.append(&gaps_workspaces_row);
        
        // Add to frame
        gaps_frame.set_child(Some(&gaps_box));
        gaps_frame
    }
    
    /// Create the behavior section
    fn create_behavior_section(&self) -> Frame {
        let behavior_frame = Frame::new(Some("Behavior"));
        let behavior_box = GtkBox::new(Orientation::Vertical, 10);
        behavior_box.set_margin_top(10);
        behavior_box.set_margin_bottom(10);
        behavior_box.set_margin_start(10);
        behavior_box.set_margin_end(10);
        
        // Layout
        let layout_row = GtkBox::new(Orientation::Horizontal, 10);
        let layout_label = Label::new(Some("Layout:"));
        layout_label.set_halign(gtk::Align::Start);
        layout_label.set_hexpand(true);
        layout_row.append(&layout_label);
        layout_row.append(&self.layout_combo);
        behavior_box.append(&layout_row);
        
        // Resize on border
        let resize_row = GtkBox::new(Orientation::Horizontal, 10);
        let resize_label = Label::new(Some("Resize on Border:"));
        resize_label.set_halign(gtk::Align::Start);
        resize_label.set_hexpand(true);
        resize_row.append(&resize_label);
        resize_row.append(&self.resize_on_border);
        behavior_box.append(&resize_row);
        
        // No focus fallback
        let focus_fallback_row = GtkBox::new(Orientation::Horizontal, 10);
        let focus_fallback_label = Label::new(Some("Disable Focus Fallback:"));
        focus_fallback_label.set_halign(gtk::Align::Start);
        focus_fallback_label.set_hexpand(true);
        focus_fallback_row.append(&focus_fallback_label);
        focus_fallback_row.append(&self.no_focus_fallback);
        behavior_box.append(&focus_fallback_row);
        
        // Allow tearing
        let tearing_row = GtkBox::new(Orientation::Horizontal, 10);
        let tearing_label = Label::new(Some("Allow Tearing:"));
        tearing_label.set_halign(gtk::Align::Start);
        tearing_label.set_hexpand(true);
        tearing_row.append(&tearing_label);
        tearing_row.append(&self.allow_tearing);
        behavior_box.append(&tearing_row);
        
        // Add to frame
        behavior_frame.set_child(Some(&behavior_box));
        behavior_frame
    }
    
    /// Update the UI from a configuration section
    pub fn update_from_config(&self, section: &GeneralSection) {
        // Update border size
        self.border_size.set_value(section.border_size as f64);
        
        // Parse gaps values - handle both int and CSS style
        let gaps_in = if let Ok(val) = section.gaps_in.parse::<i32>() {
            val as f64
        } else {
            // If it's a CSS style value, just use the first number
            let first_num = section.gaps_in.split_whitespace()
                .next()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(5);
            first_num as f64
        };
        
        let gaps_out = if let Ok(val) = section.gaps_out.parse::<i32>() {
            val as f64
        } else {
            // If it's a CSS style value, just use the first number
            let first_num = section.gaps_out.split_whitespace()
                .next()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(20);
            first_num as f64
        };
        
        self.gaps_in.set_value(gaps_in);
        self.gaps_out.set_value(gaps_out);
        self.gaps_workspaces.set_value(section.gaps_workspaces as f64);
        
        // Set colors
        set_color_from_hyprland_string(&self.active_border_color, &section.col_active_border);
        set_color_from_hyprland_string(&self.inactive_border_color, &section.col_inactive_border);
        
        // Set switches
        self.no_border_floating.set_active(section.no_border_on_floating);
        self.resize_on_border.set_active(section.resize_on_border);
        self.no_focus_fallback.set_active(section.no_focus_fallback);
        self.allow_tearing.set_active(section.allow_tearing);
        
        // Set dropdown for layout
        let layout_index = match section.layout.as_str() {
            "master" => 1,
            _ => 0, // Default to dwindle
        };
        self.layout_combo.set_selected(layout_index as u32);
    }
    
    /// Connect the UI signals to update the config
    pub fn connect_signals(&self, section: &mut GeneralSection) {
        // Border size
        self.border_size.connect_value_changed(clone!(@strong section => move |scale| {
            let value = scale.value() as i32;
            section.border_size = value;
        }));
        
        // Gaps in
        self.gaps_in.connect_value_changed(clone!(@strong section => move |scale| {
            let value = scale.value() as i32;
            section.gaps_in = value.to_string();
        }));
        
        // Gaps out
        self.gaps_out.connect_value_changed(clone!(@strong section => move |scale| {
            let value = scale.value() as i32;
            section.gaps_out = value.to_string();
        }));
        
        // Gaps workspaces
        self.gaps_workspaces.connect_value_changed(clone!(@strong section => move |scale| {
            let value = scale.value() as i32;
            section.gaps_workspaces = value;
        }));
        
        // Active border color
        self.active_border_color.connect_rgba_notify(clone!(@strong section => move |button| {
            let rgba = button.rgba();
            section.col_active_border = format!(
                "rgba({:02x}{:02x}{:02x}{:02x})",
                (rgba.red() * 255.0) as u8,
                (rgba.green() * 255.0) as u8,
                (rgba.blue() * 255.0) as u8,
                (rgba.alpha() * 255.0) as u8
            );
        }));
        
        // Inactive border color
        self.inactive_border_color.connect_rgba_notify(clone!(@strong section => move |button| {
            let rgba = button.rgba();
            section.col_inactive_border = format!(
                "rgba({:02x}{:02x}{:02x}{:02x})",
                (rgba.red() * 255.0) as u8,
                (rgba.green() * 255.0) as u8,
                (rgba.blue() * 255.0) as u8,
                (rgba.alpha() * 255.0) as u8
            );
        }));
        
        // No border on floating
        self.no_border_floating.connect_state_notify(clone!(@strong section => move |switch| {
            section.no_border_on_floating = switch.is_active();
        }));
        
        // Resize on border
        self.resize_on_border.connect_state_notify(clone!(@strong section => move |switch| {
            section.resize_on_border = switch.is_active();
        }));
        
        // No focus fallback
        self.no_focus_fallback.connect_state_notify(clone!(@strong section => move |switch| {
            section.no_focus_fallback = switch.is_active();
        }));
        
        // Allow tearing
        self.allow_tearing.connect_state_notify(clone!(@strong section => move |switch| {
            section.allow_tearing = switch.is_active();
        }));
        
        // Layout
        self.layout_combo.connect_selected_notify(clone!(@strong section => move |dropdown| {
            let selected = dropdown.selected();
            section.layout = match selected {
                1 => "master".to_string(),
                _ => "dwindle".to_string(),
            };
        }));
    }
}

/// Helper to set ColorButton from Hyprland color format
fn set_color_from_hyprland_string(button: &ColorButton, color_str: &str) {
    // Parse RGBA from Hyprland format like 'rgba(33ccffee)'
    if color_str.starts_with("rgba(") && color_str.ends_with(')') {
        let inner = &color_str[5..color_str.len()-1];
        
        // Handle different formats
        let rgba = if inner.contains(',') {
            // Parse decimal format: rgba(179, 255, 26, 0.933)
            let parts: Vec<&str> = inner.split(',').collect();
            if parts.len() >= 4 {
                let r = parts[0].trim().parse::<f64>().unwrap_or(0.0) / 255.0;
                let g = parts[1].trim().parse::<f64>().unwrap_or(0.0) / 255.0;
                let b = parts[2].trim().parse::<f64>().unwrap_or(0.0) / 255.0;
                let a = parts[3].trim().parse::<f64>().unwrap_or(1.0);
                gdk::RGBA::new(r as f32, g as f32, b as f32, a as f32)
            } else {
                gdk::RGBA::new(1.0, 1.0, 1.0, 1.0) // Default white
            }
        } else {
            // Parse hex format: rgba(b3ff1aee)
            let r = u8::from_str_radix(&inner[0..2], 16).unwrap_or(255) as f32 / 255.0;
            let g = u8::from_str_radix(&inner[2..4], 16).unwrap_or(255) as f32 / 255.0;
            let b = u8::from_str_radix(&inner[4..6], 16).unwrap_or(255) as f32 / 255.0;
            let a = u8::from_str_radix(&inner[6..8], 16).unwrap_or(255) as f32 / 255.0;
            gdk::RGBA::new(r, g, b, a)
        };
        
        button.set_rgba(&rgba);
    } else if color_str.starts_with("rgb(") && color_str.ends_with(')') {
        // Handle RGB format without alpha
        // Implementation similar to above but without alpha
    } else if color_str.starts_with("0x") {
        // Handle legacy ARGB format like 0xeeb3ff1a
        let hex = &color_str[2..];
        let a = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255) as f32 / 255.0;
        let r = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255) as f32 / 255.0;
        let g = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255) as f32 / 255.0;
        let b = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0;
        
        let rgba = gdk::RGBA::new(r, g, b, a);
        button.set_rgba(&rgba);
    }
}