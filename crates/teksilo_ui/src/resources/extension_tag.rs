// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The small coloured tag a file's row shows before its size: its extension, `pdf`,
//! `docx`, `jpg`. The title is the file's name without it, so two files called "Map"
//! looked alike but for their icon.
//!
//! **The colour comes from the extension.** The formats a writer knows by their colour
//! keep it: a PDF is Adobe's red, a Word document Word's blue. Every other extension
//! takes a hue from its letters, through a named hash (FNV-1a) rather than
//! `DefaultHasher`, whose output std does not promise across releases: an extension must
//! keep its colour from one build to the next.
//!
//! **Every colour is then brought into one band of luminance**, [`LUMINANCE_LOW`] to
//! [`LUMINANCE_HIGH`], keeping its hue. Inside it a tag stands out at 3:1 or more from
//! the plain surfaces of both the light and the dark theme, so it needs no outline and
//! no knowledge of the theme. Its text is derived as a tag's is,
//! [`contrast::text_on`], which clears 4.5:1 on any fill.
//!
//! The tag says the extension of the file that opens ([`name_to_judge`]): a linked
//! "map.pdf" whose path is a script shows the script's.

use teksilo::prelude::*;
use teksilo::tokens::Color;
use teksilo::widgets::Badge;

use crate::models::ResourceRow;
use crate::resources::file_kind;
use crate::resources::name_to_judge;
use crate::tags::contrast;

/// The most characters a tag shows. A longer extension (`numbers`, `scrivx`) keeps its
/// first ones and ends with "…", so the tag stays small beside the size.
pub const MAX_CHARS: usize = 6;

/// The darkest a tag's fill may be. Against the dark theme's surface (`#2B2D30`) it
/// gives 3.0:1.
pub const LUMINANCE_LOW: f32 = 0.18;

/// The lightest a tag's fill may be. Against the light theme's surface (`#F7F8FA`) it
/// gives 3.0:1.
pub const LUMINANCE_HIGH: f32 = 0.27;

/// The extension of `row`'s file, lowercase and without its dot. `None` for a web link,
/// a file with no extension, or a symbolic link to nothing.
pub fn extension_of(row: &ResourceRow) -> Option<String> {
    if row.is_web_link() {
        return None;
    }
    let ext = file_kind::extension(&name_to_judge(row));
    (!ext.is_empty()).then_some(ext)
}

/// What the tag reads for `ext`: the whole extension, or its first characters and "…".
pub fn label_for(ext: &str) -> String {
    if ext.chars().count() <= MAX_CHARS {
        return ext.to_string();
    }
    let mut short: String = ext.chars().take(MAX_CHARS - 1).collect();
    short.push('…');
    short
}

/// The colour a format is known by, for the few that have one.
fn known_colour(ext: &str) -> Option<Color> {
    let hex = match ext {
        "pdf" => "#FA0F00",
        "doc" | "docx" | "docm" | "dot" | "dotx" | "odt" | "ott" | "rtf" => "#2B579A",
        "xls" | "xlsx" | "xlsm" | "ods" | "csv" | "tsv" => "#217346",
        "ppt" | "pptx" | "pptm" | "odp" => "#D24726",
        "html" | "htm" => "#E34F26",
        "txt" | "md" | "markdown" | "djot" => "#6B7280",
        _ => return None,
    };
    Some(Color::from_hex(hex))
}

/// FNV-1a over `bytes`: named, so that its output never changes.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// A colour from `ext`'s letters: its hue from the hash, and a saturation from four
/// steps, so two extensions whose hues fall close apart still differ.
fn colour_from_letters(ext: &str) -> Color {
    let hash = fnv1a(ext.as_bytes());
    let hue = (hash % 360) as f32;
    let saturation = 0.55 + ((hash >> 32) % 4) as f32 * 0.1;
    Color::from_hsl(hue, saturation, 0.45)
}

/// `colour`, darkened toward black or lightened toward white until its luminance is in
/// the band. Stepping rather than solving, as `margin_lane::resolve_slot` does: the
/// result is checked against the band anyway.
fn into_band(colour: Color) -> Color {
    let luminance = colour.relative_luminance();
    if (LUMINANCE_LOW..=LUMINANCE_HIGH).contains(&luminance) {
        return colour;
    }
    let darken = luminance > LUMINANCE_HIGH;
    let mut best = colour;
    for step in 1..=100 {
        let amount = step as f32 / 100.0;
        best = if darken {
            colour.darken(amount)
        } else {
            colour.lighten(amount)
        };
        let l = best.relative_luminance();
        if (darken && l <= LUMINANCE_HIGH) || (!darken && l >= LUMINANCE_LOW) {
            break;
        }
    }
    best
}

/// The fill of `ext`'s tag: its known colour or one from its letters, in the band.
pub fn fill_for(ext: &str) -> Color {
    into_band(known_colour(ext).unwrap_or_else(|| colour_from_letters(ext)))
}

/// The tag of `row`'s file, or `None` when it has no extension to show.
///
/// Hidden from assistive technologies: the row's accessible name already says what kind
/// of file it is ("PDF document"), and would otherwise be followed by the letters again.
pub fn extension_tag(row: &ResourceRow) -> Option<impl Widget + use<>> {
    let ext = extension_of(row)?;
    let fill = fill_for(&ext);
    Some(
        Badge::new(lit!(label_for(&ext)))
            .background(fill)
            .text_role(contrast::text_on(fill))
            .access_hidden(true),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ResourceKind;

    fn stored(file_name: &str) -> ResourceRow {
        ResourceRow {
            kind: ResourceKind::StoredFile,
            file_name: file_name.into(),
            ..Default::default()
        }
    }

    #[test]
    fn a_file_shows_its_extension_lowercase_and_a_web_link_none() {
        assert_eq!(extension_of(&stored("Map.pdf")).as_deref(), Some("pdf"));
        assert_eq!(extension_of(&stored("SCAN.JPG")).as_deref(), Some("jpg"));
        assert_eq!(extension_of(&stored("notes.txt.")).as_deref(), Some("txt"));
        assert_eq!(extension_of(&stored("README")), None);
        assert_eq!(extension_of(&stored(".hidden")), None);
        let linked = ResourceRow {
            kind: ResourceKind::LinkedFile,
            ..stored("Interview.mp3")
        };
        assert_eq!(extension_of(&linked).as_deref(), Some("mp3"));
        let web = ResourceRow {
            kind: ResourceKind::WebLink,
            location: "https://example.org/map.pdf".into(),
            ..stored("")
        };
        assert_eq!(extension_of(&web), None);
    }

    /// A linked file is named by the file that opens, not by the name the project gave
    /// it: a "map.pdf" whose path is a launcher says so.
    #[test]
    fn a_linked_file_shows_the_extension_of_the_file_that_opens() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("run.desktop");
        std::fs::write(&path, b"[Desktop Entry]").unwrap();
        let row = ResourceRow {
            kind: ResourceKind::LinkedFile,
            file_name: "map.pdf".into(),
            location: path.display().to_string(),
            path: Some(path),
            ..Default::default()
        };
        assert_eq!(extension_of(&row).as_deref(), Some("desktop"));
    }

    #[test]
    fn a_long_extension_is_cut_with_an_ellipsis() {
        assert_eq!(label_for("pdf"), "pdf");
        assert_eq!(label_for("scrivx"), "scrivx");
        assert_eq!(label_for("markdown"), "markd…");
        assert_eq!(label_for("markdown").chars().count(), MAX_CHARS);
    }

    #[test]
    fn a_colour_never_changes_and_differs_between_extensions() {
        assert_eq!(fill_for("zip"), fill_for("zip"));
        // FNV-1a's published values, pinned: a tag keeps its colour from one build to
        // the next only while the hash gives these.
        assert_eq!(fnv1a(b""), 0xcbf2_9ce4_8422_2325);
        assert_eq!(fnv1a(b"a"), 0xaf63_dc4c_8601_ec8c);
        let hashed = ["zip", "mp3", "mp4", "png", "jpg", "svg", "epub", "scrivx"];
        for (i, a) in hashed.iter().enumerate() {
            for b in &hashed[i + 1..] {
                assert_ne!(fill_for(a), fill_for(b), "{a} and {b}");
            }
        }
    }

    /// The colours people know stay those colours where the band allows: Adobe's red
    /// is already in it.
    #[test]
    fn a_pdf_keeps_adobe_red_and_a_word_document_stays_blue() {
        assert_eq!(fill_for("pdf"), Color::from_hex("#FA0F00"));
        let word = fill_for("docx");
        assert!(word.b() > word.r() && word.b() > word.g(), "{word:?}");
        assert_eq!(fill_for("docx"), fill_for("odt"));
    }

    /// The guarantee the band exists for: every tag, known or hashed, stands out from
    /// the plain surfaces of both themes at 3:1 (WCAG 1.4.11), and its text reads on it
    /// at 4.5:1 (WCAG AA).
    #[test]
    fn every_tag_stands_out_on_both_themes_and_its_text_reads() {
        let mut exts: Vec<String> = [
            "pdf", "doc", "docx", "odt", "rtf", "xls", "xlsx", "ods", "csv", "ppt", "pptx", "odp",
            "html", "htm", "txt", "md",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        // A sweep of made-up extensions reaches every hue the hash can give.
        for a in b'a'..=b'z' {
            for b in b'a'..=b'z' {
                exts.push(String::from_utf8(vec![a, b, b'x']).unwrap());
            }
        }
        for colors in [
            teksilo::tokens::ColorTokens::light_default(),
            teksilo::tokens::ColorTokens::dark_default(),
        ] {
            let surfaces = [
                colors.surface_main,
                colors.surface_content,
                colors.surface_alt_row,
            ];
            for ext in &exts {
                let fill = fill_for(ext);
                let l = fill.relative_luminance();
                assert!(
                    (LUMINANCE_LOW..=LUMINANCE_HIGH).contains(&l),
                    "{ext}: luminance {l:.3} out of the band"
                );
                for surface in surfaces {
                    let ratio = fill.contrast_ratio(surface);
                    assert!(
                        ratio >= 3.0,
                        "{ext}: {fill:?} at {ratio:.2}:1 on {surface:?}"
                    );
                }
                let text = fill.contrast_ratio(contrast::text_on(fill));
                assert!(text >= 4.5, "{ext}: text at {text:.2}:1");
            }
        }
    }

    /// The tag sits on the size's line, and a screen reader does not hear it: the row's
    /// own name says the file's kind already.
    #[test]
    fn the_tag_is_on_the_size_line_and_hidden_from_screen_readers() {
        let row = ResourceRow {
            byte_size: 2048,
            ..stored("Map.pdf")
        };
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.add(crate::resources::row::detail_widget(&row));
        tree.layout(SizeProposal::exact(240.0, 40.0));
        assert!(tree.find_by_label("pdf").is_some(), "the tag is built");
        let snapshot = tree.accessibility_tree_snapshot();
        assert!(
            snapshot
                .nodes
                .iter()
                .filter(|(_, node)| node.label() == Some("pdf"))
                .all(|(_, node)| node.is_hidden()),
            "a screen reader must not hear the tag"
        );
    }

    #[test]
    fn a_tag_is_made_for_a_file_and_not_for_a_web_link() {
        assert!(extension_tag(&stored("Map.pdf")).is_some());
        let web = ResourceRow {
            kind: ResourceKind::WebLink,
            location: "https://example.org".into(),
            ..Default::default()
        };
        assert!(extension_tag(&web).is_none());
    }
}
