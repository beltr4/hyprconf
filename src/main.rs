mod app;
mod config;
mod ui;
mod safety;

use anyhow::Result;
use log::{info, LevelFilter};
use env_logger::Builder;

fn main() -> Result<()> {
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .format_timestamp(None)
        .init();
    
    info!("Starting Hyprconf application");
    
    // Initialize GTK
    gtk::init()?;
    
    // Create and run the application
    let app = app::HyprconfApp::new();
    let exit_code = app.run();
    
    info!("Application exited with code {}", exit_code);
    Ok(())
}