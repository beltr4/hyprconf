use std::io::{self, Write};
use crate::config::models::AnimationsSection;
use crate::config::models::Animation;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    animations: &AnimationsSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "animations {", comment_style)?;
    
    // Write enabled state
    write_boolean_option(writer, " enabled", animations.enabled, None, comment_style)?;
    
    // Write first launch animation
    write_boolean_option(writer, " first_launch_animation", animations.first_launch_animation, None, comment_style)?;
    
    // Write bezier curves
    for (name, curve) in &animations.beziers {
        write_option(writer, &format!(" bezier = {}", name), curve, None, comment_style)?;
    }
    
    // Write animations
    for anim in &animations.animations {
        let mut parts = vec![
            anim.name.as_str(),
            &anim.enabled.to_string(),
            &anim.speed.to_string(),
            &anim.curve
        ];
        
        // Add style if present
        if let Some(style) = &anim.style {
            parts.push(style);
        }
        
        write_option(writer, " animation", &parts.join(", "), None, comment_style)?;
    }
    
    writeln!(writer, "}}")?;
    Ok(())
}