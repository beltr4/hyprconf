use gtk::prelude::*;
use gtk::{
    Box, Grid, Switch, Label, SpinButton, ColorButton, Adjustment, Entry, ComboBoxText,
};
use std::rc::Rc;
use std::cell::RefCell;

use crate::app::AppState;

pub struct GeneralTab {
    widget: Box,
    app_state: Rc<RefCell<AppState>>,
}

impl GeneralTab {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        let widget = Box::new(gtk::Orientation::Vertical, 10);
        widget.set_margin_start(10);
        widget.set_margin_end(10);
        widget.set_margin_top(10);
        widget.set_margin_bottom(10);
        widget.set_spacing(10);
        
        let grid = Grid::new();
        grid.set_column_spacing(10);
        grid.set_row_spacing(10);
        
        // Get the current config
        let config = {
            let state = app_state.borrow();
            state.get_config().clone()
        };
        
        // Row counter
        let mut row = 0;
        
        // Sensitivity setting
        if let Some(general) = &config.general {
            // Sensitivity control
            let sens_label = Label::new(Some("Mouse Sensitivity:"));
            sens_label.set_halign(gtk::Align::End);
            
            let sens_adjustment = Adjustment::new(
                general.sensitivity.unwrap_or(1.0),
                0.0, 10.0, 0.1, 1.0, 0.0
            );
            let sens_spinner = SpinButton::new(Some(&sens_adjustment), 0.1, 1);
            
            grid.attach(&sens_label, 0, row, 1, 1);
            grid.attach(&sens_spinner, 1, row, 1, 1);
            
            row += 1;
            
            // Apply sensitivity to raw input
            let apply_sens_label = Label::new(Some("Apply Sensitivity to Raw Input:"));
            apply_sens_label.set_halign(gtk::Align::End);
            
            let apply_sens_switch = Switch::new();
            apply_sens_switch.set_active(general.apply_sens_to_raw.unwrap_or(false));
            
            grid.attach(&apply_sens_label, 0, row, 1, 1);
            grid.attach(&apply_sens_switch, 1, row, 1, 1);
            
            row += 1;
            
            // Border size
            let border_label = Label::new(Some("Border Size:"));
            border_label.set_halign(gtk::Align::End);
            
            let border_adjustment = Adjustment::new(
                general.border_size.unwrap_or(1) as f64,
                0.0, 10.0, 1.0, 1.0, 0.0
            );
            let border_spinner = SpinButton::new(Some(&border_adjustment), 1.0, 0);
            
            grid.attach(&border_label, 0, row, 1, 1);
            grid.attach(&border_spinner, 1, row, 1, 1);
            
            row += 1;
            
            // No border on floating
            let no_border_label = Label::new(Some("No Border on Floating Windows:"));
            no_border_label.set_halign(gtk::Align::End);
            
            let no_border_switch = Switch::new();
            no_border_switch.set_active(general.no_border_on_floating.unwrap_or(false));
            
            grid.attach(&no_border_label, 0, row, 1, 1);
            grid.attach(&no_border_switch, 1, row, 1, 1);
            
            row += 1;
            
            // Gaps in
            let gaps_in_label = Label::new(Some("Inner Gaps:"));
            gaps_in_label.set_halign(gtk::Align::End);
            
            let gaps_in_adjustment = Adjustment::new(
                general.gaps_in.unwrap_or(5) as f64,
                0.0, 50.0, 1.0, 5.0, 0.0
            );
            let gaps_in_spinner = SpinButton::new(Some(&gaps_in_adjustment), 1.0, 0);
            
            grid.attach(&gaps_in_label, 0, row, 1, 1);
            grid.attach(&gaps_in_spinner, 1, row, 1, 1);
            
            row += 1;
            
            // Gaps out
            let gaps_out_label = Label::new(Some("Outer Gaps:"));
            gaps_out_label.set_halign(gtk::Align::End);
            
            let gaps_out_adjustment = Adjustment::new(
                general.gaps_out.unwrap_or(20) as f64,
                0.0, 100.0, 1.0, 5.0, 0.0
            );
            let gaps_out_spinner = SpinButton::new(Some(&gaps_out_adjustment), 1.0, 0);
            
            grid.attach(&gaps_out_label, 0, row, 1, 1);
            grid.attach(&gaps_out_spinner, 1, row, 1, 1);
            
            row += 1;
            
            // Active border color
            let active_border_label = Label::new(Some("Active Border Color:"));
            active_border_label.set_halign(gtk::Align::End);
            
            let active_border_button = ColorButton::new();
            // We'd need to parse the color string from the config here
            // For now, just set a default color
            
            grid.attach(&active_border_label, 0, row, 1, 1);
            grid.attach(&active_border_button, 1, row, 1, 1);
            
            row += 1;
            
            // Inactive border color
            let inactive_border_label = Label::new(Some("Inactive Border Color:"));
            inactive_border_label.set_halign(gtk::Align::End);
            
            let inactive_border_button = ColorButton::new();
            // We'd need to parse the color string from the config here
            
            grid.attach(&inactive_border_label, 0, row, 1, 1);
            grid.attach(&inactive_border_button, 1, row, 1, 1);
            
            row += 1;
            
            // Resize on border
            let resize_border_label = Label::new(Some("Resize on Border:"));
            resize_border_label.set_halign(gtk::Align::End);
            
            let resize_border_switch = Switch::new();
            resize_border_switch.set_active(general.resize_on_border.unwrap_or(false));
            
            grid.attach(&resize_border_label, 0, row, 1, 1);
            grid.attach(&resize_border_switch, 1, row, 1, 1);
            
            row += 1;
            
            // Hover icon on border
            let hover_icon_label = Label::new(Some("Hover Icon on Border:"));
            hover_icon_label.set_halign(gtk::Align::End);
            
            let hover_icon_switch = Switch::new();
            hover_icon_switch.set_active(general.hover_icon_on_border.unwrap_or(false));
            
            grid.attach(&hover_icon_label, 0, row, 1, 1);
            grid.attach(&hover_icon_switch, 1, row, 1, 1);
            
            row += 1;
            
            // Layout
            let layout_label = Label::new(Some("Layout:"));
            layout_label.set_halign(gtk::Align::End);
            
            let layout_combo = ComboBoxText::new();
            layout_combo.append(Some("dwindle"), "Dwindle");
            layout_combo.append(Some("master"), "Master");
            
            if let Some(layout) = &general.layout {
                layout_combo.set_active_id(Some(layout));
            } else {
                layout_combo.set_active_id(Some("dwindle"));
            }
            
            grid.attach(&layout_label, 0, row, 1, 1);
            grid.attach(&layout_combo, 1, row, 1, 1);
            
            row += 1;
            
            // Allow tearing
            let tearing_label = Label::new(Some("Allow Tearing:"));
            tearing_label.set_halign(gtk::Align::End);
            
            let tearing_switch = Switch::new();
            tearing_switch.set_active(general.allow_tearing.unwrap_or(false));
            
            // Create a row for tearing
            let tearing_row = Box::new(gtk::Orientation::Horizontal, 10);
            tearing_row.pack_start(&tearing_label, false, false, 0);
            tearing_row.pack_start(&tearing_switch, false, false, 0);
            
            grid.attach(&tearing_label, 0, row, 1, 1);
            grid.attach(&tearing_switch, 1, row, 1, 1);
            
            row += 1;
        }
        
        widget.pack_start(&grid, true, true, 0);
        
        Self {
            widget,
            app_state,
        }
    }
    
    pub fn get_widget(&self) -> &Box {
        &self.widget
    }
}