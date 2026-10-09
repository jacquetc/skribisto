// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Scrivener 3's styles: what a `<$Scr_Ps::N>` or `<$Scr_Cs::N>` marker names.
//!
//! A marker's `N` is an index into the document's own style list (`content.styles` for its
//! text, `notes.styles` for its notes: comma-separated style UUIDs), and each UUID names a
//! style in the project's `Files/styles.xml`:
//!
//! ```text
//! <Styles>
//!   <Style Name="Heading 1" ID="640A541E-…" Type="Para+Char" …>
//!     <Format><![CDATA[{\rtf1 … <$Scr_H::1> … }]]></Format>
//!   </Style>
//! ```
//!
//! What a style *means* is read from as little as possible: a heading from the heading
//! marker in its own sample (language-independent, so `Encabezado 1` is a heading too), a
//! block quote or code by its name through the table the Word and OpenDocument importers
//! share (`document_ingest::sources::rich::styled_as`). Every other style is only a name the
//! report lists, since the formatting Scrivener baked into the text survives on its own.
//!
//! A missing list, an index past it or a UUID the project does not define is counted,
//! never guessed.

use std::collections::{HashMap, HashSet};

use document_ingest::sources::rich::{self, StyledAs};

use super::notice::CompileTreatment;

use crate::xml;

/// The most styles read from one `styles.xml`: far beyond any real project (the largest
/// measured defines a few dozen), and a bound on what a crafted one costs.
const MAX_STYLES: usize = 10_000;

/// The most characters of a style's name kept.
const MAX_NAME_CHARS: usize = 200;

/// Which of text a style applies to, as `Type` says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StyleType {
    Paragraph,
    Character,
    /// `Para+Char`: both.
    Both,
}

/// One style of `Files/styles.xml`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StyleDef {
    pub(crate) name: String,
    pub(crate) kind: StyleType,
    /// The heading level its own sample carries (`<$Scr_H::N>`), 1 to 6.
    pub(crate) heading: Option<u8>,
    /// What the project's compile does with the style's text, when it does not print it as
    /// prose.
    pub(crate) compile: Option<CompileTreatment>,
}

/// What a paragraph style means in Skribisto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParaMeaning {
    Heading(u8),
    Quote,
    CodeBlock,
    /// A style with no meaning here: only its name is reported.
    None,
}

impl StyleDef {
    /// What the style means for a paragraph it is applied to.
    pub(crate) fn paragraph_meaning(&self) -> ParaMeaning {
        if let Some(level) = self.heading {
            return ParaMeaning::Heading(level);
        }
        match rich::styled_as(&self.name) {
            Some(StyledAs::Quote) => ParaMeaning::Quote,
            Some(StyledAs::CodeBlock) => ParaMeaning::CodeBlock,
            // An epigraph is a row of its own in the document importer; a Scrivener style
            // named so stays in its prose, and its name is reported.
            Some(StyledAs::Epigraph) | None => ParaMeaning::None,
        }
    }

    /// Whether the style marks its characters as code.
    pub(crate) fn is_code_span(&self) -> bool {
        rich::char_style_is_code(&self.name)
    }
}

/// Every style of a project, by UUID (upper-cased: Scrivener writes either case).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct StyleTable {
    by_id: HashMap<String, StyleDef>,
}

impl StyleTable {
    /// Read `Files/styles.xml`. A file that does not parse yields no style, and every marker
    /// naming one is then counted as unresolved.
    pub(crate) fn read(bytes: &[u8]) -> Result<StyleTable, xml::XmlProblem> {
        let text = xml::text_of(bytes)?;
        let doc = xml::parse("Files/styles.xml", text)?;
        let mut table = StyleTable::default();
        for style in doc
            .descendants()
            .filter(|n| n.is_element() && n.tag_name().name() == "Style")
            .take(MAX_STYLES)
        {
            let Some(id) = style.attribute("ID") else {
                continue;
            };
            let name: String = style
                .attribute("Name")
                .unwrap_or_default()
                .chars()
                .take(MAX_NAME_CHARS)
                .collect();
            let kind = match style.attribute("Type").unwrap_or_default() {
                "Char" => StyleType::Character,
                "Para+Char" => StyleType::Both,
                _ => StyleType::Paragraph,
            };
            let heading = xml::child_text(style, "Format")
                .as_deref()
                .and_then(heading_in_sample);
            table.by_id.insert(
                id.trim().to_ascii_uppercase(),
                StyleDef {
                    name,
                    kind,
                    heading,
                    compile: None,
                },
            );
        }
        Ok(table)
    }

    pub(crate) fn get(&self, id: &str) -> Option<&StyleDef> {
        self.by_id.get(&id.trim().to_ascii_uppercase())
    }

    /// Mark the styles a compile format deletes or passes through raw. A compile format
    /// names a style by its name, as Scrivener matches them (the ids differ between a
    /// format and a project); the names are compared NFC and case-folded.
    pub(crate) fn mark_compiled(&mut self, deleted: &[String], raw: &[String]) {
        let key = |name: &str| crate::metadata::folded(name.trim());
        let deleted: HashSet<String> = deleted.iter().map(|n| key(n)).collect();
        let raw: HashSet<String> = raw.iter().map(|n| key(n)).collect();
        for def in self.by_id.values_mut() {
            let name = key(&def.name);
            if deleted.contains(&name) {
                def.compile = Some(CompileTreatment::Deleted);
            } else if raw.contains(&name) {
                def.compile = Some(CompileTreatment::RawMarkup);
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn insert(&mut self, id: &str, def: StyleDef) {
        self.by_id.insert(id.to_ascii_uppercase(), def);
    }
}

/// The styles one stream's markers index into.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct StreamStyles<'a> {
    /// The stream's own list (`content.styles`), when it has one.
    pub(crate) list: Option<&'a [String]>,
    pub(crate) table: Option<&'a StyleTable>,
}

impl<'a> StreamStyles<'a> {
    /// The style a marker's index names, or `None` when the list, the index or the UUID is
    /// missing.
    pub(crate) fn resolve(&self, index: u32) -> Option<&'a StyleDef> {
        let id = self.list?.get(usize::try_from(index).ok()?)?;
        self.table?.get(id)
    }
}

/// Read a `content.styles` list: comma-separated UUIDs, no newline.
pub(crate) fn read_list(bytes: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(bytes)
        .split(',')
        .map(|id| id.trim().to_ascii_uppercase())
        .take(MAX_STYLES)
        .collect()
}

/// The heading level a style's RTF sample carries: the `N` of its `<$Scr_H::N>` marker.
///
/// The sample is decoded rather than searched as bytes, since the marker's characters may be
/// written across RTF groups. Scrivener's own `Heading 1` carries level 1; the level is held
/// to the six Djot writes.
fn heading_in_sample(sample: &str) -> Option<u8> {
    let text: String = match rtf_reader::parse_str(sample) {
        Ok(doc) => rtf_reader::paragraphs(&doc.body)
            .flat_map(|p| p.runs())
            .map(|r| r.text.as_str())
            .collect(),
        // Not RTF: the sample as written.
        Err(_) => sample.to_string(),
    };
    let at = text.find("<$Scr_H::")?;
    let digits: String = text[at + "<$Scr_H::".len()..]
        .chars()
        .take_while(char::is_ascii_digit)
        .take(3)
        .collect();
    let level: u8 = digits.parse().ok()?;
    Some(level.clamp(1, 6))
}

#[cfg(test)]
mod tests {
    use super::*;

    const STYLES: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<Styles>
    <Style Name="Heading 1" ID="640a541e-95a4-4026-af24-014b25e855ae" Type="Para+Char">
        <Format><![CDATA[{\rtf1\ansi <$Scr_H::1><$Scr_Ps::0>\b Heading<!$Scr_H::1><!$Scr_Ps::0>}]]></Format>
    </Style>
    <Style Name="Encabezado 2" ID="11111111-2222-3333-4444-555555555555" Type="Para">
        <Format><![CDATA[{\rtf1\ansi {<$Scr_H::}{2>}Encabezado}]]></Format>
    </Style>
    <Style Name="Block Quote" ID="22222222-2222-3333-4444-555555555555" Type="Para"/>
    <Style Name="Code Span" ID="33333333-2222-3333-4444-555555555555" Type="Char"/>
    <Style Name="Title" ID="44444444-2222-3333-4444-555555555555" Type="Para"/>
</Styles>"#;

    #[test]
    fn a_style_sheet_reads_names_kinds_and_heading_levels() {
        let table = StyleTable::read(STYLES.as_bytes()).expect("parses");
        let heading = table
            .get("640A541E-95A4-4026-AF24-014B25E855AE")
            .expect("case-insensitive UUID");
        assert_eq!(heading.heading, Some(1));
        assert_eq!(heading.kind, StyleType::Both);
        assert_eq!(heading.paragraph_meaning(), ParaMeaning::Heading(1));
        let spanish = table
            .get("11111111-2222-3333-4444-555555555555")
            .expect("present");
        assert_eq!(
            spanish.paragraph_meaning(),
            ParaMeaning::Heading(2),
            "a marker split across groups still names its level"
        );
        let quote = table
            .get("22222222-2222-3333-4444-555555555555")
            .expect("present");
        assert_eq!(quote.paragraph_meaning(), ParaMeaning::Quote);
        assert_eq!(quote.compile, None);
        let span = table
            .get("33333333-2222-3333-4444-555555555555")
            .expect("present");
        assert!(span.is_code_span());
        assert_eq!(span.kind, StyleType::Character);
        let title = table
            .get("44444444-2222-3333-4444-555555555555")
            .expect("present");
        assert_eq!(
            title.paragraph_meaning(),
            ParaMeaning::None,
            "a title style is not a heading: its sample carries no heading marker"
        );
    }

    /// A compile format names the styles it leaves out by their names, which a project's
    /// style sheet matches whatever the ids and the letters' case.
    #[test]
    fn a_compile_format_marks_the_styles_it_leaves_out_by_name() {
        let mut table = StyleTable::read(STYLES.as_bytes()).expect("parses");
        table.mark_compiled(&["block quote".to_string()], &[" TITLE ".to_string()]);
        let quote = table
            .get("22222222-2222-3333-4444-555555555555")
            .expect("present");
        assert_eq!(quote.compile, Some(CompileTreatment::Deleted));
        let title = table
            .get("44444444-2222-3333-4444-555555555555")
            .expect("present");
        assert_eq!(title.compile, Some(CompileTreatment::RawMarkup));
        let heading = table
            .get("640A541E-95A4-4026-AF24-014B25E855AE")
            .expect("present");
        assert_eq!(heading.compile, None);
    }

    #[test]
    fn an_index_resolves_through_the_streams_own_list() {
        let table = StyleTable::read(STYLES.as_bytes()).expect("parses");
        let list =
            read_list(b"22222222-2222-3333-4444-555555555555,640A541E-95A4-4026-AF24-014B25E855AE");
        let styles = StreamStyles {
            list: Some(&list),
            table: Some(&table),
        };
        assert_eq!(
            styles.resolve(0).map(|s| s.name.as_str()),
            Some("Block Quote")
        );
        assert_eq!(
            styles.resolve(1).map(|s| s.name.as_str()),
            Some("Heading 1")
        );
        assert!(styles.resolve(2).is_none(), "past the list");
        let none = StreamStyles {
            list: None,
            table: Some(&table),
        };
        assert!(none.resolve(0).is_none(), "no list, nothing resolved");
    }
}
