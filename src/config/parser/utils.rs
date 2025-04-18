use std::path::Path;

/// Find the default Hyprland config file
pub fn find_default_config() -> Option<String> {
    // Check common locations
    let possible_paths = [
        // User config directory
        dirs::config_dir().map(|p| p.join("hypr/hyprland.conf")),
        // Home directory
        dirs::home_dir().map(|p| p.join(".config/hypr/hyprland.conf")),
        // System config
        Some(Path::new("/etc/hypr/hyprland.conf").to_path_buf()),
    ];
    
    for path in possible_paths.iter().flatten() {
        if path.exists() {
            return path.to_str().map(String::from);
        }
    }
    
    None
}
/// Helper function to parse a boolean value
pub fn parse_bool(value: &str) -> bool {
    match value.to_lowercase().as_str() {
        "true" | "yes" | "on" | "1" => true,
        // Special case for animations section which uses "yes, please" or similar
        s if s.contains("yes") => true,
        _ => false,
    }
}
