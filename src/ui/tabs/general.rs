use gtk::prelude::*;
use gtk::{Box as GtkBox, Grid, Label, Switch, SpinButton, Adjustment, ComboBoxText};
use std::rc::Rc;
use std::cell::RefCell;

use crate::app::AppState;

pub struct GeneralTab {
    widget: GtkBox,
    app_state: Rc<RefCell<AppState>>,
}

impl GeneralTab {
    pub fn new(app_state: Rc<RefCell<AppState>>) -> Self {
        let widget = GtkBox::new(gtk::Orientation::Vertical, 10);
        Self { widget, app_state}
    }

    pub fn get_widget(&self) -> &GtkBox {
        &self.widget
    }
}
