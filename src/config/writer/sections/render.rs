use std::io::{self, Write};
use crate::config::models::RenderSection;
use crate::config::writer::utils::{CommentStyle, write_section_header, write_option, write_boolean_option};

pub fn write_section<W: Write>(
    writer: &mut W,
    render: &RenderSection,
    comment_style: &CommentStyle,
) -> io::Result<()> {
    write_section_header(writer, "render {", comment_style)?;
    
    write_option(writer, " explicit_sync", &render.explicit_sync, None, comment_style)?;
    write_option(writer, " explicit_sync_kms", &render.explicit_sync_kms, None, comment_style)?;
    write_option(writer, " direct_scanout", &render.direct_scanout, None, comment_style)?;
    write_boolean_option(writer, " expand_undersized_textures", render.expand_undersized_textures, None, comment_style)?;
    write_boolean_option(writer, " xp_mode", render.xp_mode, None, comment_style)?;
    write_option(writer, " ctm_animation", &render.ctm_animation, None, comment_style)?;
    write_option(writer, " cm_fs_passthrough", &render.cm_fs_passthrough, None, comment_style)?;
    write_boolean_option(writer, " cm_enabled", render.cm_enabled, None, comment_style)?;
    
    writeln!(writer, "}}")?;
    Ok(())
}