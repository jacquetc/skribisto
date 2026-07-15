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
pub use render::{RenderRequest, RenderStats, render_preview_html, render_to_file, render_to_string};
