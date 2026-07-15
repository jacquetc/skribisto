//! Compile an export scope into one `text_document::TextDocument` and render it.
//!
//! The single place a `TextDocument` is built for export. Given a frozen [`Gathered`] tree
//! (from `skrib_format::tree_read::gather`), an ordered set of item ids to include (from
//! `skribisto_model::compile::resolve_scope` or the Choose… tree), and a [`Preset`] style,
//! it assembles one document — localized headings + scene breaks around each scene's Djot —
//! and renders it to the chosen [`ExportFormat`]. Shared by the backend `export_work` use
//! case (commit) and `bastyde_ui` (client-side live preview), so neither reimplements the
//! compile.
//!
//! [`Gathered`]: skrib_format::Gathered

mod headings;
mod preset;
mod render;

pub use preset::{
    DigitStyle, DirectionMode, ExportFormat, HeadingLanguage, HeadingScheme, LineSpacing, Margins,
    PageSize, Preset, SceneBreak, builtin_presets,
};
pub use render::{
    RenderRequest, RenderStats, render_preview_document, render_preview_html, render_to_file,
    render_to_string,
};

use skrib_format::Gathered;
use skribisto_model::compile::ItemMeta;

/// The flat, ordered `ItemMeta` stream from a gathered tree — the input both
/// [`skribisto_model::compile::resolve_scope`] and [`render::render_to_file`]'s scope
/// filter walk. Shared by the backend `export_work` use case and the UI's client-side
/// preview so the two resolve byte-identical scopes from the same tree.
pub fn item_metas(g: &Gathered) -> Vec<ItemMeta> {
    let mut v = Vec::new();
    for bwi in &g.binders {
        for iwc in &bwi.items {
            let it = &iwc.item;
            v.push(ItemMeta {
                id: it.id,
                role: it.role.clone(),
                sub_role: it.sub_role.clone(),
                indent: it.indent as i32,
                activated: it.activated,
                is_exportable: it.is_exportable,
            });
        }
    }
    v
}
