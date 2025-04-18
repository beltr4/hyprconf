use std::io::{self, Write};
use crate::config::models::MiscSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

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