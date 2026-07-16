//! An **export style** — a format-agnostic bundle of typography, structure, content
//! inclusion, and localization choices. The output *format* is chosen separately at export
//! time; a style applies wherever the format supports each knob (free text formats ignore
//! the typography fields; DOCX/PDF honour them).
//!
//! Presets are the user's stable data: built-ins live in [`builtin_presets`] (read-only),
//! user presets are persisted by `bastyde_ui`, and both round-trip as JSON for
//! import/export.

use serde::{Deserialize, Serialize};

/// One output format an exporter can render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Djot,
    PlainText,
    Markdown,
    Html,
    Latex,
    Docx,
    Epub,
    Pdf,
}

impl ExportFormat {
    /// The conventional file extension (no dot).
    pub fn extension(self) -> &'static str {
        match self {
            ExportFormat::Djot => "dj",
            ExportFormat::PlainText => "txt",
            ExportFormat::Markdown => "md",
            ExportFormat::Html => "html",
            ExportFormat::Latex => "tex",
            ExportFormat::Docx => "docx",
            ExportFormat::Epub => "epub",
            ExportFormat::Pdf => "pdf",
        }
    }

    /// Whether the format is rendered synchronously to a `String` (vs. written to a file).
    pub fn is_text(self) -> bool {
        matches!(
            self,
            ExportFormat::Djot
                | ExportFormat::PlainText
                | ExportFormat::Markdown
                | ExportFormat::Html
                | ExportFormat::Latex
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum LineSpacing {
    Single,
    OneAndHalf,
    #[default]
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PageSize {
    #[default]
    A4,
    Letter,
    A5,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Margins {
    pub top_in: f32,
    pub bottom_in: f32,
    pub left_in: f32,
    pub right_in: f32,
}

impl Default for Margins {
    fn default() -> Self {
        Margins { top_in: 1.0, bottom_in: 1.0, left_in: 1.0, right_in: 1.0 }
    }
}

/// How to separate two scenes within a chapter (generated furniture, never the author's
/// own prose).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SceneBreak {
    /// A centered glyph line, e.g. `#` (Shunn) or `* * *`.
    Glyph(String),
    /// A single blank line, no glyph.
    #[default]
    BlankLine,
    /// No separator at all.
    None,
}

/// How a Book / Part / Chapter heading is generated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum HeadingScheme {
    /// No heading emitted.
    None,
    /// The localized structural word + number only ("Chapter 3").
    Numbered,
    /// The item's own title only.
    TitleOnly,
    /// Both ("Chapter 3 — The Storm").
    #[default]
    NumberAndTitle,
}

/// Which language drives the generated structural words.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum HeadingLanguage {
    /// Follow each scene's own resolved language (the default; a mixed-language book gets
    /// mixed heading words).
    #[default]
    Auto,
    /// Force one BCP-47 language for every generated word.
    Fixed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DigitStyle {
    #[default]
    Western,
    /// Eastern Arabic-Indic digits (٠١٢…). Note the Arabic world forks Mashriq (Eastern)
    /// vs. Maghreb (Western) — this picks Eastern explicitly.
    EasternArabic,
}

/// Text direction for the whole export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DirectionMode {
    /// Derive per scene from its language (RTL for ar/he/…).
    #[default]
    Auto,
    ForceLtr,
    ForceRtl,
}

/// A named export style. Every field is part of the JSON contract; the free text formats
/// use the structure + localization fields, DOCX/PDF additionally honour the typography.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub id: String,
    pub name: String,
    /// Built-in (read-only) marker. Set only by [`builtin_presets`]; user copies are `false`.
    #[serde(default)]
    pub builtin: bool,
    /// The built-in id this was duplicated from (provenance), if any.
    #[serde(default)]
    pub source: Option<String>,

    // Typography (applied by DOCX / PDF).
    pub font_family: String,
    pub font_size_pt: f32,
    #[serde(default)]
    pub line_spacing: LineSpacing,
    #[serde(default)]
    pub page_size: PageSize,
    #[serde(default)]
    pub margin: Margins,
    #[serde(default)]
    pub first_line_indent_in: f32,
    #[serde(default)]
    pub paragraph_spacing_pt: f32,
    #[serde(default)]
    pub justify: bool,

    // Structure (generated furniture — never rewrites prose).
    #[serde(default)]
    pub scene_break: SceneBreak,
    #[serde(default)]
    pub chapter_heading: HeadingScheme,
    #[serde(default)]
    pub part_heading: HeadingScheme,
    #[serde(default)]
    pub book_title_page: bool,

    // Content inclusion.
    #[serde(default)]
    pub include_synopses: bool,
    #[serde(default)]
    pub include_notes: bool,
    #[serde(default)]
    pub include_scene_titles: bool,

    // Localization.
    #[serde(default)]
    pub heading_language: HeadingLanguage,
    #[serde(default)]
    pub digit_style: DigitStyle,
    #[serde(default)]
    pub direction: DirectionMode,

    /// Which formats this style is offered for (a UI hint). Empty ⇒ all.
    #[serde(default)]
    pub formats: Vec<ExportFormat>,
}

impl Preset {
    /// A neutral base a builder starts from — sensible defaults, language-derived
    /// everything. Not itself a shipped preset (see [`builtin_presets`]).
    fn base(id: &str, name: &str) -> Self {
        Preset {
            id: id.to_string(),
            name: name.to_string(),
            builtin: true,
            source: None,
            font_family: "Literata".to_string(),
            font_size_pt: 12.0,
            line_spacing: LineSpacing::Double,
            page_size: PageSize::A4,
            margin: Margins::default(),
            first_line_indent_in: 0.5,
            paragraph_spacing_pt: 0.0,
            justify: false,
            scene_break: SceneBreak::BlankLine,
            chapter_heading: HeadingScheme::NumberAndTitle,
            part_heading: HeadingScheme::NumberAndTitle,
            book_title_page: false,
            include_synopses: false,
            include_notes: false,
            include_scene_titles: false,
            heading_language: HeadingLanguage::Auto,
            digit_style: DigitStyle::Western,
            direction: DirectionMode::Auto,
            formats: Vec::new(),
        }
    }
}

/// The read-only styles shipped with the app. User presets are duplicated from these and
/// then editable. Kept as Rust values (type-safe, can't fail to parse at runtime); a test
/// asserts each round-trips through JSON so the import/export format stays valid.
pub fn builtin_presets() -> Vec<Preset> {
    vec![
        Preset::base("neutral", "Neutral"),
        // The manuscript presets differ by typography + scene break only; heading words stay
        // `HeadingLanguage::Auto` (base default) so "Chapter"/"Chapitre"/"Kapitel" follow each
        // scene's own resolved language rather than being forced by the chosen style.
        Preset {
            id: "manuscript-shunn".to_string(),
            name: "Standard Manuscript (Shunn)".to_string(),
            font_family: "Times New Roman".to_string(),
            line_spacing: LineSpacing::Double,
            first_line_indent_in: 0.5,
            scene_break: SceneBreak::Glyph("#".to_string()),
            book_title_page: true,
            ..Preset::base("manuscript-shunn", "")
        },
        Preset {
            id: "manuscript-fr".to_string(),
            name: "Manuscrit (français)".to_string(),
            font_family: "Times New Roman".to_string(),
            scene_break: SceneBreak::Glyph("*".to_string()),
            ..Preset::base("manuscript-fr", "")
        },
        Preset {
            id: "manuscript-de".to_string(),
            name: "Manuskript (Normseite)".to_string(),
            font_family: "Courier New".to_string(),
            ..Preset::base("manuscript-de", "")
        },
        Preset {
            id: "ebook-clean".to_string(),
            name: "Clean e-book".to_string(),
            font_family: "Source Serif 4".to_string(),
            line_spacing: LineSpacing::Single,
            first_line_indent_in: 0.0,
            scene_break: SceneBreak::Glyph("* * *".to_string()),
            chapter_heading: HeadingScheme::TitleOnly,
            formats: vec![ExportFormat::Epub, ExportFormat::Html],
            ..Preset::base("ebook-clean", "")
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtins_round_trip_through_json() {
        for p in builtin_presets() {
            let json = serde_json::to_string_pretty(&p).unwrap();
            let back: Preset = serde_json::from_str(&json).unwrap();
            assert_eq!(p, back, "preset {} must survive a JSON round-trip", p.id);
            assert!(back.builtin);
        }
    }

    #[test]
    fn builtin_ids_are_unique() {
        let mut ids: Vec<String> = builtin_presets().into_iter().map(|p| p.id).collect();
        ids.sort();
        let n = ids.len();
        ids.dedup();
        assert_eq!(ids.len(), n, "built-in preset ids must be unique");
    }

    #[test]
    fn a_missing_field_defaults_rather_than_failing() {
        // Forward/backward compatibility: a preset JSON with only the required fields loads.
        let json = r#"{ "id": "x", "name": "X", "font_family": "Serif", "font_size_pt": 12.0 }"#;
        let p: Preset = serde_json::from_str(json).unwrap();
        assert_eq!(p.id, "x");
        assert!(!p.builtin);
        assert_eq!(p.line_spacing, LineSpacing::Double);
        assert_eq!(p.scene_break, SceneBreak::BlankLine);
    }
}
