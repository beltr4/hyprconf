use std::io::{self, Write};
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};
use crate::config::utils::parse_bool;
use crate::config::utils::{CommentStyle, write_boolean_option, write_option, write_section_header};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MiscSection {
    pub disable_hyprland_logo: bool,
    pub disable_splash_rendering: bool,
    pub col_splash: String,
    pub font_family: String,
    pub splash_font_family: String,
    pub force_default_wallpaper: i32,
    pub vfr: bool,
    pub vrr: i32,
    pub mouse_move_enables_dpms: bool,
    pub key_press_enables_dpms: bool,
    pub always_follow_on_dnd: bool,
    pub layers_hog_keyboard_focus: bool,
    pub animate_manual_resizes: bool,
    pub animate_mouse_windowdragging: bool,
    pub disable_autoreload: bool,
    pub enable_swallow: bool,
    pub swallow_regex: String,
    pub swallow_exception_regex: String,
    pub focus_on_activate: bool,
    pub mouse_move_focuses_monitor: bool,
    pub render_ahead_of_time: bool,
    pub render_ahead_safezone: i32,
    pub allow_session_lock_restore: bool,
    pub background_color: String,
    pub close_special_on_empty: bool,
    pub new_window_takes_over_fullscreen: i32,
    pub exit_window_retains_fullscreen: bool,
    pub initial_workspace_tracking: i32,
    pub middle_click_paste: bool,
    pub render_unfocused_fps: i32,
    pub disable_xdg_env_checks: bool,
    pub disable_hyprland_qtutils_check: bool,
    pub lockdead_screen_delay: i32,
    pub enable_anr_dialog: bool,
    pub anr_missed_pings: i32,
}

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

pub fn write_section<W: Write>(
    writer: &mut W,
    misc: &MiscSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "misc {", comment_style)?;
    write_boolean_option(writer, " disable_hyprland_logo", misc.disable_hyprland_logo, None, comment_style)?;
    write_boolean_option(writer, " disable_splash_rendering", misc.disable_splash_rendering, None, comment_style)?;
    write_option(writer, " col.splash", &misc.col_splash, None, comment_style)?;
    write_option(writer, " font_family", &misc.font_family, None, comment_style)?;
    write_option(writer, " splash_font_family", &misc.splash_font_family, None, comment_style)?;
    write_option(writer, " force_default_wallpaper", &misc.force_default_wallpaper, None, comment_style)?;
    write_boolean_option(writer, " vfr", misc.vfr, None, comment_style)?;
    write_option(writer, " vrr", &misc.vrr, None, comment_style)?;
    write_boolean_option(writer, " mouse_move_enables_dpms", misc.mouse_move_enables_dpms, None, comment_style)?;
    write_boolean_option(writer, " key_press_enables_dpms", misc.key_press_enables_dpms, None, comment_style)?;
    write_boolean_option(writer, " always_follow_on_dnd", misc.always_follow_on_dnd, None, comment_style)?;
    write_boolean_option(writer, " layers_hog_keyboard_focus", misc.layers_hog_keyboard_focus, None, comment_style)?;
    write_boolean_option(writer, " animate_manual_resizes", misc.animate_manual_resizes, None, comment_style)?;
    write_boolean_option(writer, " animate_mouse_windowdragging", misc.animate_mouse_windowdragging, None, comment_style)?;
    write_boolean_option(writer, " disable_autoreload", misc.disable_autoreload, None, comment_style)?;
    write_boolean_option(writer, " enable_swallow", misc.enable_swallow, None, comment_style)?;
    write_option(writer, " swallow_regex", &misc.swallow_regex, None, comment_style)?;
    write_option(writer, " swallow_exception_regex", &misc.swallow_exception_regex, None, comment_style)?;
    write_boolean_option(writer, " focus_on_activate", misc.focus_on_activate, None, comment_style)?;
    write_boolean_option(writer, " mouse_move_focuses_monitor", misc.mouse_move_focuses_monitor, None, comment_style)?;
    write_boolean_option(writer, " render_ahead_of_time", misc.render_ahead_of_time, None, comment_style)?;
    write_option(writer, " render_ahead_safezone", &misc.render_ahead_safezone, None, comment_style)?;
    write_boolean_option(writer, " allow_session_lock_restore", misc.allow_session_lock_restore, None, comment_style)?;
    write_option(writer, " background_color", &misc.background_color, None, comment_style)?;
    write_boolean_option(writer, " close_special_on_empty", misc.close_special_on_empty, None, comment_style)?;
    write_option(writer, " new_window_takes_over_fullscreen", &misc.new_window_takes_over_fullscreen, None, comment_style)?;
    write_boolean_option(writer, " exit_window_retains_fullscreen", misc.exit_window_retains_fullscreen, None, comment_style)?;
    write_option(writer, " initial_workspace_tracking", &misc.initial_workspace_tracking, None, comment_style)?;
    write_boolean_option(writer, " middle_click_paste", misc.middle_click_paste, None, comment_style)?;
    write_option(writer, " render_unfocused_fps", &misc.render_unfocused_fps, None, comment_style)?;
    write_boolean_option(writer, " disable_xdg_env_checks", misc.disable_xdg_env_checks, None, comment_style)?;
    write_boolean_option(writer, " disable_hyprland_qtutils_check", misc.disable_hyprland_qtutils_check, None, comment_style)?;
    write_option(writer, " lockdead_screen_delay", &misc.lockdead_screen_delay, None, comment_style)?;
    write_boolean_option(writer, " enable_anr_dialog", misc.enable_anr_dialog, None, comment_style)?;
    write_option(writer, " anr_missed_pings", &misc.anr_missed_pings, None, comment_style)?;
    writeln!(writer, "}}")?;
    Ok(())
}
