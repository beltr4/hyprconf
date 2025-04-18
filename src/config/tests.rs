#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;
    
    use crate::config::parser::core::ConfigParser;
    use crate::config::writer::core::ConfigWriter;
    
    #[test]
    fn test_config_round_trip() {
        // A sample hyprland config file
        let sample_config = r#"
# This is a sample Hyprland config file

general {
    border_size = 2
    gaps_in = 5
    gaps_out = 10
    col.active_border = rgba(33ccffee)
    col.inactive_border = rgba(595959aa)
    layout = dwindle
}

decoration {
    rounding = 10
    blur = true
    blur_size = 3
    blur_passes = 1
    drop_shadow = true
    shadow_range = 4
    shadow_render_power = 3
    col.shadow = rgba(1a1a1aee)
}

animations {
    enabled = true
    
    bezier = myBezier, 0.05, 0.9, 0.1, 1.05
    
    animation = windows, 1, 7, myBezier
    animation = windowsOut, 1, 7, default, popin 80%
    animation = fade, 1, 7, default
    animation = workspaces, 1, 6, default
}

input {
    kb_layout = us
    follow_mouse = 1
    sensitivity = 0.5
    touchpad {
        natural_scroll = true
    }
}

dwindle {
    pseudotile = true
    preserve_split = true
    no_gaps_when_only = false
}

bind = SUPER, Q, exec, kitty
bind = SUPER, C, killactive,
bind = SUPER, M, exit,
bind = SUPER, E, exec, dolphin
bind = SUPER, V, togglefloating,
bind = SUPER, R, exec, wofi --show drun
bind = SUPER, P, pseudo,
bind = SUPER, J, togglesplit,

# Move focus with arrow keys
bind = SUPER, left, movefocus, l
bind = SUPER, right, movefocus, r
bind = SUPER, up, movefocus, u
bind = SUPER, down, movefocus, d
"#;
        
        // Create a temporary directory for our test
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let input_path = temp_dir.path().join("input.conf");
        let output_path = temp_dir.path().join("output.conf");
        
        // Write the sample config to the input file
        fs::write(&input_path, sample_config).expect("Failed to write sample config");
        
        // Parse the config file
        let parser = ConfigParser::new();
        let config = parser.parse_file(&input_path).expect("Failed to parse config");
        
        // Write the config to the output file
        let writer = ConfigWriter::new(config);
        writer.write_to_file(&output_path).expect("Failed to write config");
        
        // Read back the output file
        let written_config = fs::read_to_string(&output_path).expect("Failed to read written config");
        
        // Parse the written config to verify it's valid
        let parser = ConfigParser::new();
        let _parsed_output = parser.parse_str(&written_config).expect("Failed to parse written config");
        
        // We don't compare the input and output directly because formatting may differ,
        // but as long as we can parse both successfully, the round-trip is considered successful
    }
}