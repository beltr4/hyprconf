mod app;
mod config;
mod safety;
mod ui;

#[cfg(test)]
mod tests;

use std::path::PathBuf;
use std::process;
use gio::prelude::*;

use clap::{Parser, Subcommand};
use config::parser::core::ConfigParser;
use config::writer::core::ConfigWriter;
use ui::window::AppWindow;

#[derive(Parser)]
#[command(name = "hyprconf")]
#[command(about = "A GUI configuration tool for Hyprland")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Parse and validate a Hyprland config file
    Validate {
        /// Path to the Hyprland config file
        #[arg(default_value = "~/.config/hypr/hyprland.conf")]
        path: String,
    },
    /// Generate a default Hyprland config file
    Generate {
        /// Path to output the generated config file
        #[arg(default_value = "~/.config/hypr/hyprland.conf")]
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Validate { path }) => {
            let path = shellexpand::tilde(&path).to_string();
            println!("Validating config file: {}", path);
            
            match ConfigParser::parse_file(&path) {
                Ok(_) => {
                    println!("Configuration file is valid!");
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error validating configuration: {}", e);
                    process::exit(1);
                }
            }
            
        }
        Some(Commands::Generate { path }) => {
            let path = shellexpand::tilde(&path).to_string();
            println!("Generating default config file: {}", path);
            
            // Generate a default config
            let default_config = config::models::HyprlandConfig::default();
            
            // Write the config to the specified path
            let writer = ConfigWriter::new(default_config);
            match writer.write_to_file(&path) {
                Ok(_) => {
                    println!("Default configuration file generated!");
                    process::exit(0);
                }
                Err(e) => {
                    eprintln!("Error generating configuration: {}", e);
                    process::exit(1);
                }
            }
        }
        None => {
            // Run the GUI application
            let app = app::build_app();
            app.run();
        }
    }
}