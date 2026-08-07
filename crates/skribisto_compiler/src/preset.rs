// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! An **export style** — a format-agnostic bundle of typography, structure, content
//! inclusion, and localization choices. The output *format* is chosen separately at export
//! time; a style applies wherever the format supports each knob (free text formats ignore
//! the typography fields; DOCX/PDF honour them).
//!
//! Presets are the user's stable data: built-ins live in [`builtin_presets`] (read-only),
//! user presets are persisted by `teksilo_ui`, and both round-trip as JSON for
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

    /// Whether an image in this format is a *path* to a file that has to exist
    /// beside the output, rather than bytes carried inside it.
    ///
    /// The distinction is what decides whether an export writes a sidecar
    /// folder, and it does not line up with [`Self::is_text`]: plain text is a
    /// text format that has no images at all, and DOCX/EPUB/PDF are containers
    /// that need no help.
    pub fn references_images(self) -> bool {
        matches!(
            self,
            ExportFormat::Djot | ExportFormat::Markdown | ExportFormat::Html | ExportFormat::Latex
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
        Margins {
            top_in: 1.0,
            bottom_in: 1.0,
            left_in: 1.0,
            right_in: 1.0,
        }
    }
}

/// How a scene break the author marked in their prose is *rendered*.
///
/// The author owns **where** a break goes (a marker paragraph — see
/// `skribisto_model::scene_break`); a preset owns **what it looks like**. That
/// split is what lets the same manuscript export as Shunn `#` for a submission
/// and a dinkus for an ebook without retyping anything.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SceneBreak {
    /// A centered glyph line, e.g. `#` (Shunn) or `* * *` (the dinkus).
    Glyph(String),
    /// No glyph — extra leading, and no first-line indent on the paragraph that
    /// follows. The dominant *minor*-break convention outside the anglophone
    /// world: French, German, Spanish (the RAE prefers it to asterisks
    /// outright), Russian and Italian publishing, and Japanese print all use a
    /// bare gap rather than a mark.
    #[default]
    BlankLine,
    /// No separator at all — the scenes run straight together.
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

/// The spaced em dash between a generated number and a title — this compiler's
/// long-standing separator, and the `#[serde(default)]` companion for
/// [`Preset::heading_separator`] so a preset written before the field existed keeps
/// rendering exactly as it did.
fn default_heading_separator() -> String {
    " — ".to_string()
}

/// Where a chapter's epigraph sits relative to the heading that opens the chapter.
///
/// Every editorial convention that actually writes the rule down says **after**: Chicago
/// ("typically follows the chapter number … and any chapter title"), French practice (the
/// epigraph "suit le titre du chapitre"), German (a chapter's Motto stands *zwischen*
/// Kapitelüberschrift und Textbeginn), and the Russian publishing dictionary («эпиграф к
/// главе завёрстывается после заголовка главы перед текстом»).
///
/// Putting it above the title is nonetheless a real and long-standing designer's choice —
/// LaTeX's `epigraph` package ships `\epigraphhead` for exactly that, offset above the
/// chapter heading — and enough published fiction does it that a writer who remembers it
/// that way is not misremembering. So it is a choice, not a rule, and the default is the
/// documented convention.
/// Where a note's text is set — at the foot of the page, or gathered as endnotes.
///
/// A note that leaves the foot of the page stops being a footnote and becomes an
/// **endnote**; the marker in the prose does not move, only the text does. That is
/// why this is one option rather than two features — and why the variants say
/// "endnotes" rather than describing a position. A writer looking for endnotes
/// should find them under that word, which is the one every style guide uses.
///
/// All three placements are ordinary editorial choices, not a scale from best to
/// worst: Chicago offers exactly these, and the back-of-book "Notes" section is
/// what most trade non-fiction ships.
///
/// What a given format can honour differs, and the writer's choice is honoured as
/// far as the format allows rather than refused:
///
/// * PDF, DOCX and LaTeX set a real page-bottom footnote.
/// * EPUB and HTML have no page bottom — a reflowable book has no pages — so the
///   `noteref`/`aside` pair becomes a pop-up. That is the format's own idiom, not a
///   compromise.
/// * Markdown and plain text can only list them at the end.
///
/// **Nothing reads this yet**: every value renders identically, notes landing once
/// at the end of the compiled document. It is named and documented ahead of the
/// behaviour deliberately, so the vocabulary is settled before anything depends on
/// it — see the note in `settings/panes/export_styles.rs` on why it has no control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FootnotePlacement {
    /// At the foot of the page the reference falls on. What "footnote" means.
    #[default]
    PageBottom,
    /// Endnotes, gathered at the end of each chapter.
    EndnotesPerChapter,
    /// Endnotes, gathered at the end of the book — the back-of-book "Notes"
    /// section.
    ///
    /// Named for what it *is* rather than where it sits, because "book end" is
    /// already taken twice over in this codebase for something else entirely:
    /// `BinderItemSubRole::BookEnd` is a book's closing matter, and
    /// `CreateType::EndOfBook` is labelled `create-book-end`. A third meaning
    /// would have been one too many — and "bookend" is a shelf ornament.
    EndnotesAtEnd,
}

/// Where a footnote's numbering restarts. Mirrors
/// [`skribisto_model::footnote_numbering::FootnoteRestart`], which does the counting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FootnoteNumbering {
    /// One run through the whole book — the non-fiction convention, where a reader
    /// may cite "note 214" and expect to find it.
    #[default]
    Continuous,
    /// Restart at each chapter — the fiction convention, and what keeps a long
    /// novel's markers out of four digits.
    PerChapter,
    /// Restart at each book.
    PerBook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum EpigraphPlacement {
    /// After the chapter number/title, before the body. The editorial standard.
    #[default]
    AfterHeading,
    /// Above the chapter heading, opening the page.
    BeforeHeading,
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

/// What an export does with the manuscript's images.
///
/// The choice only arises for the formats that *reference* a picture by path —
/// Djot, Markdown, HTML, LaTeX. DOCX, EPUB and PDF are containers: the bytes
/// travel inside the file, so there is nothing for the writer to decide and
/// nothing this setting can change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ImageHandling {
    /// Write the image files beside the exported document, in the folder the
    /// prose already names.
    ///
    /// The default, and the only one of the three that loses nothing. The
    /// document's references are left exactly as written and the layout on disk
    /// is built to match them, so correctness does not depend on rewriting a
    /// path — which is what would have to be got right for every format's own
    /// escaping rules.
    #[default]
    CopyBeside,
    /// Inline the bytes into the document itself, as `data:` URIs.
    ///
    /// HTML only — it is the one referencing format with a syntax for carrying
    /// bytes. Produces a single file that can be mailed or dropped anywhere,
    /// at about a third more than the images' own size. Every other format
    /// treats this as [`Self::CopyBeside`].
    Embed,
    /// Emit no images at all.
    ///
    /// For sending a manuscript as prose alone. Nothing dangles: the reference
    /// is dropped along with the file, rather than left pointing at something
    /// that was never written.
    Omit,
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
    /// How an ordinary scene break renders — a shift of time, place or viewpoint.
    #[serde(default)]
    pub scene_break: SceneBreak,
    /// How a *major* break renders — a large time skip or decisive viewpoint
    /// change. Shunn codifies `#` vs `# # #` for exactly this distinction, and
    /// it recurs independently in novelWriter and Russian editorial practice.
    #[serde(default)]
    pub major_scene_break: SceneBreak,
    #[serde(default)]
    pub chapter_heading: HeadingScheme,
    #[serde(default)]
    pub part_heading: HeadingScheme,
    /// What sits between the generated number and the title under
    /// [`HeadingScheme::NumberAndTitle`] — "Chapter 3`<sep>`The Storm".
    ///
    /// A style choice, not a constant: published fiction uses a spaced em dash, a colon, a
    /// period, and a plain line break, and which one a book takes is exactly the sort of
    /// decision an export style exists to hold. Defaults to the spaced em dash this
    /// compiler has always emitted, so an existing preset renders byte-identically.
    ///
    /// Carries its own spacing rather than being spliced with hardcoded spaces around it,
    /// because a style wanting `" : "` — with the no-break space French requires before a
    /// colon — cannot express that if the surrounding spaces are not the style's to choose.
    ///
    /// **Single-line only.** A heading is emitted as one `#`-prefixed line, so a newline in
    /// here would end the heading early and leave the title behind as a stray paragraph in
    /// every backend. Any newline is folded to a space rather than rejected: a preset is
    /// hand-editable JSON, and quietly rendering "3 The Storm" beats corrupting the
    /// document structure. Putting the number on its own line above the title is a real
    /// layout, but it needs the number and the title to reach the renderer as separate
    /// things — a different feature, not a separator string.
    #[serde(default = "default_heading_separator")]
    pub heading_separator: String,
    #[serde(default)]
    pub book_title_page: bool,
    /// Open the book with its cover picture, when the export includes the book's
    /// opening and the Work has one.
    ///
    /// `default = "yes"`, for the reason its pagination neighbours give below: a
    /// preset saved before the field existed has no such key, and nobody *chose*
    /// a coverless book — there was no cover to choose. EPUB ignores this; it has
    /// a real cover slot in the package and fills it either way.
    #[serde(default = "yes")]
    pub book_cover: bool,
    /// Print the rounded word count on the title page — the manuscript-submission
    /// convention, where an editor reads it before anything else. Off elsewhere: a trade
    /// title page carries the title and the byline and nothing more.
    #[serde(default)]
    pub title_page_word_count: bool,

    // Pagination. Only the formats that have pages honour these — DOCX, PDF, LaTeX, and
    // print/EPUB through CSS; plain text and Markdown say it if asked and otherwise not
    // at all.
    //
    // All four default to **on**, and they need `default = "yes"` to do it. A bool's bare
    // `#[serde(default)]` is `false`, and every user preset saved before these fields
    // existed has no such key — but nobody *chose* an unbroken book, because until now
    // there was no page break to choose. Reading their absence as "off" would leave every
    // custom style silently stuck with the behaviour that prompted this, while the
    // built-ins moved on.
    /// A new page at each Book opener. Only reachable when the title page is off — with
    /// one on, the Book's own heading is suppressed and the title page does the breaking.
    #[serde(default = "yes")]
    pub book_starts_page: bool,
    #[serde(default = "yes")]
    pub part_starts_page: bool,
    /// A new page at each chapter. The most load-bearing of the four: it is the rule that
    /// makes a manuscript readable as chapters rather than as one unbroken column, and
    /// Shunn requires it outright.
    #[serde(default = "yes")]
    pub chapter_starts_page: bool,
    /// A new page at each paratext. A dedication that shares a page with the end of the
    /// copyright notice is not a dedication.
    #[serde(default = "yes")]
    pub paratext_starts_page: bool,

    // Content inclusion.
    #[serde(default)]
    pub include_synopses: bool,
    #[serde(default)]
    pub include_notes: bool,
    #[serde(default)]
    pub include_scene_titles: bool,
    /// Where a chapter's epigraph sits relative to its heading. Plain `#[serde(default)]`
    /// is right here, unlike its bool neighbours: the enum's own default *is* the
    /// convention, so a preset saved before this field existed keeps doing what it did.
    #[serde(default)]
    pub epigraph_placement: EpigraphPlacement,
    /// Keep the epigraphs. Unlike its three neighbours this defaults to **true**, and it
    /// needs `default = "yes"` to do it: an epigraph is finished-book content, not a
    /// working note, so "I wrote it, publish it" is the right answer — but a bare
    /// `#[serde(default)]` gives `false` for a bool, and every user preset saved before
    /// this field existed has no such key. Those presets would silently start dropping
    /// epigraphs while the built-ins kept them, which is exactly the kind of divergence
    /// nobody would think to look for. The toggle exists for the clean-submission case,
    /// where quoted matter is stripped from the manuscript.
    #[serde(default = "yes")]
    pub include_epigraphs: bool,
    /// Keep the footnotes. Defaults to **true**, and needs `default = "yes"` for
    /// exactly the reason epigraphs do — a bare `#[serde(default)]` reads `false`
    /// for every preset saved before this field existed, and those presets would
    /// silently start dropping notes while the built-ins kept them.
    ///
    /// More sharply here than for an epigraph, in fact: an epigraph dropped from a
    /// clean submission is a deliberate omission the writer would notice on the
    /// first page. A footnote dropped from the middle of chapter nine is text
    /// missing from the book, and nothing in the output says so.
    #[serde(default = "yes")]
    pub include_footnotes: bool,
    /// Where the note's text is set. The enum's own default is the convention.
    #[serde(default)]
    pub footnote_placement: FootnotePlacement,
    /// Where the numbering restarts.
    #[serde(default)]
    pub footnote_numbering: FootnoteNumbering,
    /// Keep the paratexts — prefaces, dedications, afterwords. Defaults to **true** for
    /// the same reason epigraphs do, and needs `default = "yes"` for the same reason:
    /// a bare `#[serde(default)]` is `false` for a bool, and every preset saved before
    /// this field existed has no such key, so custom styles would silently start
    /// dropping the author's front matter while the built-ins kept it.
    #[serde(default = "yes")]
    pub include_paratexts: bool,

    // Localization.
    #[serde(default)]
    pub heading_language: HeadingLanguage,
    #[serde(default)]
    pub digit_style: DigitStyle,
    #[serde(default)]
    pub direction: DirectionMode,

    /// What the referencing formats do with the manuscript's images. Plain
    /// `#[serde(default)]`, because the enum's own default is what every export
    /// did before the field existed.
    #[serde(default)]
    pub image_handling: ImageHandling,

    /// Which formats this style is offered for (a UI hint). Empty ⇒ all.
    #[serde(default)]
    pub formats: Vec<ExportFormat>,
}

/// `#[serde(default)]` for a bool is `false`; this is for the fields whose absence must
/// read as *on*, so a preset saved before the field existed keeps the shipped behaviour
/// instead of silently opting out of it.
fn yes() -> bool {
    true
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
            major_scene_break: SceneBreak::Glyph("* * *".to_string()),
            chapter_heading: HeadingScheme::NumberAndTitle,
            part_heading: HeadingScheme::NumberAndTitle,
            heading_separator: default_heading_separator(),
            book_title_page: false,
            book_cover: true,
            title_page_word_count: false,
            image_handling: ImageHandling::CopyBeside,
            include_footnotes: true,
            footnote_placement: FootnotePlacement::default(),
            footnote_numbering: FootnoteNumbering::default(),
            book_starts_page: true,
            part_starts_page: true,
            chapter_starts_page: true,
            paratext_starts_page: true,
            include_synopses: false,
            include_notes: false,
            include_scene_titles: false,
            include_epigraphs: true,
            epigraph_placement: EpigraphPlacement::AfterHeading,
            include_paratexts: true,
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
        // Shunn is explicit about the two tiers: `#` for an ordinary break, and
        // `# # #` for a stronger one, because `#` is the typesetter's mark for
        // space and tripling it reads as a higher-level division.
        // https://www.shunn.net/format/scene_breaks/
        Preset {
            id: "manuscript-shunn".to_string(),
            name: "Standard Manuscript (Shunn)".to_string(),
            font_family: "Times New Roman".to_string(),
            line_spacing: LineSpacing::Double,
            first_line_indent_in: 0.5,
            scene_break: SceneBreak::Glyph("#".to_string()),
            major_scene_break: SceneBreak::Glyph("# # #".to_string()),
            book_title_page: true,
            // "Round off the word count ... to the nearest thousand for longer works" —
            // upper right of the title page, the first thing an editor looks at.
            // https://www.shunn.net/format/novel/
            title_page_word_count: true,
            ..Preset::base("manuscript-shunn", "")
        },
        Preset {
            id: "manuscript-fr".to_string(),
            name: "Manuscrit (français)".to_string(),
            font_family: "Times New Roman".to_string(),
            // French practice grades a blank line (ordinary) against one to
            // three centred asterisks (stronger). The astérisme ⁂ is the
            // Imprimerie nationale's formal mark but is vanishingly rare in
            // modern fiction, so it is offered, not defaulted.
            scene_break: SceneBreak::BlankLine,
            major_scene_break: SceneBreak::Glyph("*".to_string()),
            ..Preset::base("manuscript-fr", "")
        },
        Preset {
            id: "manuscript-de".to_string(),
            name: "Manuskript (Normseite)".to_string(),
            font_family: "Courier New".to_string(),
            // No DIN or Duden rule governs this; German trade books largely
            // avoid asterisks and leave the choice to house layout, so a blank
            // line is the ordinary break and `***` the manuscript-stage stronger one.
            scene_break: SceneBreak::BlankLine,
            major_scene_break: SceneBreak::Glyph("***".to_string()),
            ..Preset::base("manuscript-de", "")
        },
        Preset {
            id: "manuscript-es".to_string(),
            name: "Manuscrito (español)".to_string(),
            // The RAE's Ortografía states that three centred asterisks
            // traditionally marked a section end but that blank lines are used
            // today, calling them "more elegant and less cumbersome" — so the
            // ordinary break is the gap and the asterisks are the stronger tier.
            // https://www.rae.es/ortografía/con-función-delimitadora
            scene_break: SceneBreak::BlankLine,
            major_scene_break: SceneBreak::Glyph("* * *".to_string()),
            ..Preset::base("manuscript-es", "")
        },
        Preset {
            id: "manuscript-ru".to_string(),
            name: "Рукопись (русский)".to_string(),
            // Russian editorial practice grades a silent gap against a graphic
            // separator. Asterisks read as poetry-coded in Russian literary
            // convention, so a row of dots is the idiomatic stronger mark.
            //
            // A *row* of dots, spaced — not the single `…` this used to emit.
            // One ellipsis alone on a line is an ordinary beat of silence in
            // fiction, and the app's own smart punctuation turns a typed `...`
            // into exactly that character, so a mark spelled that way could not
            // be told apart from prose: the recogniser would strip a writer's
            // silence from their word count and replace it with this glyph on
            // export. `. . .` matches the spacing of `* * *` and cannot be
            // mistaken for a sentence.
            scene_break: SceneBreak::BlankLine,
            major_scene_break: SceneBreak::Glyph(". . .".to_string()),
            ..Preset::base("manuscript-ru", "")
        },
        Preset {
            id: "manuscript-it".to_string(),
            name: "Manoscritto (italiano)".to_string(),
            // Same gradient as French/Spanish: paragraph gap < `* * *` < new
            // chapter. The triangular asterismo is almost never used in novels.
            scene_break: SceneBreak::BlankLine,
            major_scene_break: SceneBreak::Glyph("* * *".to_string()),
            ..Preset::base("manuscript-it", "")
        },
        Preset {
            id: "manuscript-ja-print".to_string(),
            name: "原稿（印刷）".to_string(),
            // Japanese commercial print fiction typically uses no symbol at all:
            // vertical typesetting makes a bare gap plainly visible, so symbols
            // are added only where clarity would otherwise suffer. Both tiers
            // are therefore gaps — this is a real convention, not an oversight.
            scene_break: SceneBreak::BlankLine,
            major_scene_break: SceneBreak::BlankLine,
            ..Preset::base("manuscript-ja-print", "")
        },
        Preset {
            id: "manuscript-ja-web".to_string(),
            name: "原稿（Web小説）".to_string(),
            // Web novels are read horizontally by scrolling, where a lone blank
            // line is easy to miss, so a symbol is conventional. `◇` is
            // preferred over rule-like characters, which break mobile layouts.
            scene_break: SceneBreak::Glyph("＊".to_string()),
            major_scene_break: SceneBreak::Glyph("◇".to_string()),
            ..Preset::base("manuscript-ja-web", "")
        },
        Preset {
            id: "ebook-clean".to_string(),
            name: "Clean e-book".to_string(),
            font_family: "Source Serif 4".to_string(),
            line_spacing: LineSpacing::Single,
            first_line_indent_in: 0.0,
            // Anglophone trade: the dinkus is what published fiction prints. A
            // bare blank line is avoided here on purpose — Shunn and CMOS 1.58
            // both warn it vanishes when it lands at a page boundary.
            scene_break: SceneBreak::Glyph("* * *".to_string()),
            major_scene_break: SceneBreak::Glyph("# # #".to_string()),
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

    /// Every glyph a preset can *write* must be a glyph the app can *read back*.
    ///
    /// The two vocabularies are deliberately separate — `skribisto_model::scene_break`
    /// is fixed and preset-independent, so changing a preset never un-recognises a
    /// mark the author already typed — but separate is not the same as unrelated.
    /// A preset that emits a mark the recogniser has never heard of tells the writer
    /// their own house convention is ordinary prose: the mark is counted as words,
    /// never stripped on export, and invisible to every consumer that asks "is this
    /// a break?". That gap was real for `…`, `＊` and `◇` until this test existed.
    ///
    /// This is the pin, not the derivation: the vocabulary stays a hand-written
    /// constant, and adding a preset glyph that is not in it fails here rather than
    /// making the recogniser depend on the export layer.
    #[test]
    fn every_preset_glyph_is_a_recognised_scene_break() {
        for p in builtin_presets() {
            for (tier_name, brk) in [
                ("scene_break", &p.scene_break),
                ("major_scene_break", &p.major_scene_break),
            ] {
                let SceneBreak::Glyph(glyph) = brk else {
                    continue; // BlankLine writes no text, so there is nothing to read back
                };
                assert!(
                    skribisto_model::scene_break::tier_of_plain_line(glyph).is_some(),
                    "preset '{}' emits {tier_name} '{glyph}', which \
                     skribisto_model::scene_break does not recognise — add it to \
                     MINOR_MARKERS or MAJOR_MARKERS",
                    p.id
                );
            }
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

    /// The pagination fields must read as ON when absent, not off. Every user preset saved
    /// before they existed lacks the keys, and nobody chose an unbroken book — there was
    /// nothing to choose. Getting this wrong is invisible until someone exports.
    #[test]
    fn pagination_defaults_to_on_for_a_preset_that_predates_it() {
        let json = r#"{ "id": "x", "name": "X", "font_family": "Serif", "font_size_pt": 12.0 }"#;
        let p: Preset = serde_json::from_str(json).unwrap();
        assert!(p.chapter_starts_page);
        assert!(p.part_starts_page);
        assert!(p.book_starts_page);
        assert!(p.paratext_starts_page);
        // …but the word count is a submission convention, not a default.
        assert!(!p.title_page_word_count);
    }

    /// The convention, in every tradition that writes it down, is after the heading — so
    /// that is what a preset with nothing to say about it gets, including every preset
    /// saved before the field existed.
    #[test]
    fn every_builtin_follows_the_documented_convention() {
        for p in builtin_presets() {
            assert_eq!(
                p.epigraph_placement,
                EpigraphPlacement::AfterHeading,
                "preset {} must follow its tradition's own rule",
                p.id
            );
        }
        let json = r#"{ "id": "x", "name": "X", "font_family": "Serif", "font_size_pt": 12.0 }"#;
        let old: Preset = serde_json::from_str(json).unwrap();
        assert_eq!(old.epigraph_placement, EpigraphPlacement::AfterHeading);
    }

    /// …and the other placement round-trips, so a writer who chooses it keeps it.
    #[test]
    fn the_other_placement_round_trips() {
        let p = Preset {
            epigraph_placement: EpigraphPlacement::BeforeHeading,
            ..Preset::base("x", "X")
        };
        let back: Preset = serde_json::from_str(&serde_json::to_string(&p).unwrap()).unwrap();
        assert_eq!(back.epigraph_placement, EpigraphPlacement::BeforeHeading);
    }

    /// An explicit `false` still means false — the default only fills a *missing* key.
    #[test]
    fn an_explicit_no_is_honoured() {
        let json = r#"{ "id": "x", "name": "X", "font_family": "Serif", "font_size_pt": 12.0,
                        "chapter_starts_page": false }"#;
        let p: Preset = serde_json::from_str(json).unwrap();
        assert!(!p.chapter_starts_page);
        assert!(p.part_starts_page, "the others are untouched");
    }
}
