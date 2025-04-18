use crate::config::models::MiscSection;
use crate::config::parser::utils::parse_bool;
use anyhow::Result;
use log::debug;

pub fn parse_misc_section(section: &mut MiscSection, content: &str) -> Result<()> {
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            
            match key {
                "disable_hyprland_logo" => section.disable_hyprland_logo = parse_bool(value),
                "disable_splash_rendering" => section.disable_splash_rendering = parse_bool(value),
                "col.splash" => section.col_splash = value.to_string(),
                "font_family" => section.font_family = value.to_string(),
                "splash_font_family" => section.splash_font_family = value.to_string(),
                "force_default_wallpaper" => section.force_default_wallpaper = value.parse().unwrap_or(-1),
                "vfr" => section.vfr = parse_bool(value),
                "vrr" => section.vrr = value.parse().unwrap_or(0),
                "mouse_move_enables_dpms" => section.mouse_move_enables_dpms = parse_bool(value),
                "key_press_enables_dpms" => section.key_press_enables_dpms = parse_bool(value),
                "always_follow_on_dnd" => section.always_follow_on_dnd = parse_bool(value),
                "layers_hog_keyboard_focus" => section.layers_hog_keyboard_focus = parse_bool(value),
                "animate_manual_resizes" => section.animate_manual_resizes = parse_bool(value),
                "animate_mouse_windowdragging" => section.animate_mouse_windowdragging = parse_bool(value),
                "disable_autoreload" => section.disable_autoreload = parse_bool(value),
                "enable_swallow" => section.enable_swallow = parse_bool(value),
                "swallow_regex" => section.swallow_regex = value.to_string(),
                "swallow_exception_regex" => section.swallow_exception_regex = value.to_string(),
                "focus_on_activate" => section.focus_on_activate = parse_bool(value),
                "mouse_move_focuses_monitor" => section.mouse_move_focuses_monitor = parse_bool(value),
                "render_ahead_of_time" => section.render_ahead_of_time = parse_bool(value),
                "render_ahead_safezone" => section.render_ahead_safezone = value.parse().unwrap_or(1),
                "allow_session_lock_restore" => section.allow_session_lock_restore = parse_bool(value),
                "background_color" => section.background_color = value.to_string(),
                "close_special_on_empty" => section.close_special_on_empty = parse_bool(value),
                "new_window_takes_over_fullscreen" => section.new_window_takes_over_fullscreen = value.parse().unwrap_or(0),
                "exit_window_retains_fullscreen" => section.exit_window_retains_fullscreen = parse_bool(value),
                "initial_workspace_tracking" => section.initial_workspace_tracking = value.parse().unwrap_or(1),
                "middle_click_paste" => section.middle_click_paste = parse_bool(value),
                "render_unfocused_fps" => section.render_unfocused_fps = value.parse().unwrap_or(15),
                "disable_xdg_env_checks" => section.disable_xdg_env_checks = parse_bool(value),
                "disable_hyprland_qtutils_check" => section.disable_hyprland_qtutils_check = parse_bool(value),
                "lockdead_screen_delay" => section.lockdead_screen_delay = value.parse().unwrap_or(1000),
                "enable_anr_dialog" => section.enable_anr_dialog = parse_bool(value),
                "anr_missed_pings" => section.anr_missed_pings = value.parse().unwrap_or(1),
                _ => debug!("Unknown misc setting: {}", key),
            }
        }
    }
    
    Ok(())
}