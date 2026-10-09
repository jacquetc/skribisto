// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One test per construct: each builds the shape a real writer produces, from synthetic
//! English text, and checks what the model holds.

mod common;

use common::{body, cocoa, first, parse, parse_bytes, rtf, run_with, runs, texts, windows};
use rtf_reader::{
    Alignment, Block, BreakKind, Diagnostic, Direction, FieldKind, FontFamily, Inline, NoteKind,
    ParagraphEnd, PictureFormat, PictureOrigin, Refusal, Rgb, StyleKind, VerticalPosition,
    paragraphs,
};

// ── the two Scrivener writers ────────────────────────────────────────────────

#[test]
fn cocoa_paragraphs_end_with_a_backslash_before_the_line_feed() {
    let doc = parse(&cocoa(
        "The ferry was late.\\\nThe harbour was not.\\\n\\\nEnd.",
    ));
    assert_eq!(
        body(&doc),
        ["The ferry was late.", "The harbour was not.", "", "End."]
    );
    assert_eq!(doc.header.cocoa_version, Some(2822));
    assert_eq!(first(&doc).end, ParagraphEnd::Paragraph);
    // The expanded colour table is macOS's and holds no text.
    assert_eq!(doc.colors.len(), 3);
}

#[test]
fn windows_runs_are_groups_and_paragraph_formatting_carries_across_par() {
    let doc = parse(&windows(
        "\\qc {\\f0\\fs24\\b0\\i0 A centred title}\n\\par\\plain {\\f0\\fs24\\b1\\i0 Still centred}\n\\par\\pard\\plain {\\f0\\fs24 Back to the left}",
    ));
    assert_eq!(
        body(&doc),
        ["A centred title", "Still centred", "Back to the left"]
    );
    let aligned: Vec<Option<Alignment>> =
        paragraphs(&doc.body).map(|p| p.props.alignment).collect();
    assert_eq!(
        aligned,
        [Some(Alignment::Center), Some(Alignment::Center), None]
    );
    assert!(run_with(&doc, "Still").props.bold);
    assert_eq!(doc.fonts.len(), 2);
    assert_eq!(doc.fonts[0].family, FontFamily::Modern);
    assert_eq!(doc.font(1).map(|f| f.name.as_str()), Some("Calibri"));
}

#[test]
fn the_properties_in_force_at_the_mark_govern_the_paragraph() {
    let doc = parse(r"{\rtf1 {\qc inside a group}\par {\qc closed on the mark\par}}");
    let aligned: Vec<Option<Alignment>> =
        paragraphs(&doc.body).map(|p| p.props.alignment).collect();
    assert_eq!(aligned, [None, Some(Alignment::Center)]);
}

// ── code pages ───────────────────────────────────────────────────────────────

#[test]
fn hex_escapes_are_read_in_the_document_code_page() {
    let doc = parse(r"{\rtf1\ansi\ansicpg1252 \'93I can\'92t\'97\'94 caf\'e9}");
    assert_eq!(
        body(&doc),
        ["\u{201C}I can\u{2019}t\u{2014}\u{201D} caf\u{E9}"]
    );
}

#[test]
fn mac_roman_documents() {
    let doc = parse(r"{\rtf1\mac\ansicpg10000 caf\'8e}");
    assert_eq!(body(&doc), ["caf\u{E9}"]);
}

#[test]
fn a_font_character_set_selects_the_code_page_of_its_text() {
    // An arrow typed in a Japanese font: two escapes, one Shift_JIS character.
    let doc = parse(
        r"{\rtf1\ansi\ansicpg1252{\fonttbl\f0\fnil\fcharset0 Helvetica;\f2\fnil\fcharset128 HiraginoSans;}\f0 l\'92empire \f2 \'81\'cb\f0 \'a0inverser}",
    );
    assert_eq!(body(&doc), ["l\u{2019}empire \u{21D2}\u{A0}inverser"]);
}

#[test]
fn character_set_zero_follows_the_document_code_page_as_macos_means_it() {
    let doc = parse(
        r"{\rtf1\ansi\ansicpg1251{\fonttbl\f0\fnil\fcharset0 Helvetica;}\f0 \'cf\'f0\'e8\'e2\'e5\'f2}",
    );
    assert_eq!(body(&doc), ["\u{41F}\u{440}\u{438}\u{432}\u{435}\u{442}"]);
}

#[test]
fn a_cpg_in_the_font_table_wins_over_the_character_set() {
    let doc = parse(r"{\rtf1\ansi{\fonttbl{\f0\fcharset0\cpg1251 Font;}}\f0 \'e0}");
    assert_eq!(body(&doc), ["\u{430}"]);
}

#[test]
fn the_default_font_decides_text_before_any_font_switch() {
    let doc = parse(r"{\rtf1\ansi\deff1{\fonttbl{\f0 A;}{\f1\fcharset204 B;}}\'e0\plain \'e0}");
    assert_eq!(body(&doc), ["\u{430}\u{430}"]);
    assert_eq!(doc.header.default_font, Some(1));
}

#[test]
fn an_unsupported_code_page_keeps_ascii_and_counts_the_rest() {
    let doc = parse(r"{\rtf1\ansi\ansicpg10001 ok\'c0}");
    assert_eq!(body(&doc), ["ok\u{FFFD}"]);
    assert_eq!(
        doc.diagnostics
            .count(&Diagnostic::UnsupportedCodePage(10001)),
        1
    );
}

#[test]
fn raw_eight_bit_text_is_read_in_the_code_page() {
    let mut bytes = rtf(r"{\rtf1\ansi\ansicpg1252 caf");
    bytes.push(0xE9);
    bytes.push(b'}');
    assert_eq!(body(&parse_bytes(&bytes)), ["caf\u{E9}"]);
}

#[test]
fn raw_double_byte_text_keeps_a_trail_byte_that_is_a_backslash() {
    // 0x83 0x5C is one Shift_JIS character (katakana so).
    let mut bytes = rtf(r"{\rtf1\ansi\ansicpg932 ");
    bytes.extend_from_slice(&[0x83, 0x5C, b'!', b'}']);
    assert_eq!(body(&parse_bytes(&bytes)), ["\u{30BD}!"]);
}

#[test]
fn a_symbol_font_is_read_through_its_own_table() {
    let doc = parse(
        r"{\rtf1\ansi{\fonttbl{\f0 Times;}{\f1\fcharset2 Symbol;}}\f1 a\f0  and \f1 @u-3999?}",
    );
    assert_eq!(body(&doc), ["\u{3B1} and \u{3B1}"]);
}

// ── Unicode ──────────────────────────────────────────────────────────────────

#[test]
fn windows_fallback_bytes_after_a_unicode_escape_are_skipped() {
    let doc = parse(r"{\rtf1\ansi\uc1 it@u8217\'92s @u8220?quoted@u8221?}");
    assert_eq!(body(&doc), ["it\u{2019}s \u{201C}quoted\u{201D}"]);
}

#[test]
fn a_zero_fallback_count_holds_for_later_escapes_in_the_group() {
    let doc = parse(r"{\rtf1 to @uc0@u966  or not to @u966 . end}");
    assert_eq!(body(&doc), ["to \u{3C6} or not to \u{3C6}. end"]);
}

#[test]
fn the_fallback_count_is_scoped_by_group() {
    let doc = parse(r"{\rtf1\uc2 {\uc0 a@u233 b}c@u233 xyd}");
    assert_eq!(body(&doc), ["a\u{E9}bc\u{E9}d"]);
}

#[test]
fn a_brace_ends_the_characters_a_unicode_escape_skips() {
    let doc = parse(r"{\rtf1\uc3 {x@u233}yz}");
    assert_eq!(body(&doc), ["x\u{E9}yz"]);
}

#[test]
fn a_control_word_and_binary_data_each_count_as_one_skipped_character() {
    let doc = parse(r"{\rtf1\uc1 a@u233\loch b@u233\bin2 xyc}");
    assert_eq!(body(&doc), ["a\u{E9}b\u{E9}c"]);
}

#[test]
fn negative_and_large_values_name_the_same_characters() {
    let doc = parse(r"{\rtf1\uc0 @u-3913 @u61623 @u40960 @u128512 }");
    // -3913 is U+F0B7 as a signed 16-bit number, the same as 61623.
    assert_eq!(body(&doc), ["\u{F0B7}\u{F0B7}\u{A000}\u{1F600}"]);
}

#[test]
fn surrogate_pairs_are_joined_in_either_sign() {
    let doc = parse(r"{\rtf1\uc1 a@u55357?@u56832?b@u-10179?@u-8704?c}");
    assert_eq!(body(&doc), ["a\u{1F600}b\u{1F600}c"]);
}

#[test]
fn a_lone_surrogate_is_a_replacement_character_and_is_counted() {
    let doc = parse(r"{\rtf1\uc0 a@u55357 b@u56832 c}");
    assert_eq!(body(&doc), ["a\u{FFFD}b\u{FFFD}c"]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::LoneSurrogate), 2);
}

#[test]
fn a_value_out_of_range_is_a_replacement_character() {
    let bytes = format!("{{\\rtf1\\uc0 a{}u99999999999 b}}", '\\');
    let doc = parse_bytes(bytes.as_bytes());
    assert_eq!(body(&doc), ["a\u{FFFD}b"]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::InvalidUnicode), 1);
}

#[test]
fn upr_is_read_through_its_unicode_half() {
    let doc = parse(r"{\rtf1 {\upr{\f0 ascii text}{\*\ud{\f0 unicode text}}} after}");
    assert_eq!(body(&doc), ["unicode text after"]);
}

#[test]
fn text_already_in_unicode_is_read_as_such() {
    let doc = match rtf_reader::parse_str("{\\rtf1\\ansi caf\u{E9} \\'e9\\uc1\\u233?}") {
        Ok(doc) => doc,
        Err(refusal) => panic!("{refusal}"),
    };
    assert_eq!(body(&doc), ["caf\u{E9} \u{E9}\u{E9}"]);
}

// ── destinations ─────────────────────────────────────────────────────────────

#[test]
fn an_ignorable_destination_is_skipped_and_counted_by_name() {
    let doc = parse(r"{\rtf1 before{\*\madeup text {nested} more}after}");
    assert_eq!(body(&doc), ["beforeafter"]);
    assert_eq!(
        doc.diagnostics
            .count(&Diagnostic::IgnorableDestinationSkipped("madeup".into())),
        1
    );
}

#[test]
fn an_unknown_destination_without_a_star_keeps_its_text() {
    let doc = parse(r"{\rtf1 before {\madeup kept} after}");
    assert_eq!(body(&doc), ["before kept after"]);
}

#[test]
fn a_star_in_the_middle_of_a_group_is_not_a_destination_mark() {
    // Scrivener for Windows writes `\*\scrrevlvl3` inside a run.
    let doc = parse(r"{\rtf1 {\f0 one\*\scrrevlvl3 two}}");
    assert_eq!(body(&doc), ["onetwo"]);
}

#[test]
fn info_and_headers_are_not_body_text() {
    let doc = parse(
        r"{\rtf1{\info{\title The Lighthouse}{\author A. Writer}{\*\company X}}{\header Running head}{\*\generator Riched20 10.0;}Body.}",
    );
    assert_eq!(body(&doc), ["Body."]);
    assert_eq!(doc.info.title.as_deref(), Some("The Lighthouse"));
    assert_eq!(doc.info.author.as_deref(), Some("A. Writer"));
    assert_eq!(doc.header.generator.as_deref(), Some("Riched20 10.0"));
    assert_eq!(doc.diagnostics.count(&Diagnostic::HeaderOrFooterSkipped), 1);
}

#[test]
fn index_entries_and_bookmarks_are_not_body_text() {
    let doc = parse(r"{\rtf1 A{\xe {\v entry}}B{\*\bkmkstart mark}C{\*\bkmkend mark}}");
    assert_eq!(body(&doc), ["ABC"]);
}

// ── tables of the document ───────────────────────────────────────────────────

#[test]
fn the_colour_table_starts_with_an_automatic_entry() {
    let doc = parse(
        r"{\rtf1{\colortbl;\red255\green255\blue255;\red200\green0\blue0;}\cb1 white\cb2 red\cb0 none}",
    );
    assert_eq!(doc.colors[0], None);
    assert_eq!(
        doc.color(2),
        Some(Rgb {
            red: 200,
            green: 0,
            blue: 0
        })
    );
    assert_eq!(run_with(&doc, "white").props.background, Some(1));
    assert!(!doc.background_shows(1), "a white background is none");
    assert!(doc.background_shows(2));
    assert_eq!(run_with(&doc, "none").props.background, None);
}

#[test]
fn the_style_sheet_names_its_styles_and_their_outline_levels() {
    let doc = parse(
        r"{\rtf1{\stylesheet{\ql Normal;}{\s1\ql\outlinelevel0\b heading 1;}{\*\cs10 \additive Default Paragraph Font;}{\s2\sbasedon1 Quote;}}\pard\s1\outlinelevel0 Chapter One\par\pard\s2 Quoted.}",
    );
    let heading = doc.paragraph_style(1);
    assert_eq!(heading.map(|s| s.name.as_str()), Some("heading 1"));
    assert_eq!(heading.and_then(|s| s.outline_level), Some(0));
    assert_eq!(doc.paragraph_style(2).and_then(|s| s.based_on), Some(1));
    assert!(
        doc.styles
            .iter()
            .any(|s| s.kind == StyleKind::Character && s.number == 10)
    );
    let paragraphs: Vec<_> = paragraphs(&doc.body).collect();
    assert_eq!(paragraphs[0].props.style, Some(1));
    assert_eq!(paragraphs[0].props.outline_level, Some(0));
    assert_eq!(paragraphs[1].props.style, Some(2));
    assert_eq!(body(&doc), ["Chapter One", "Quoted."]);
}

// ── paragraph and character formatting ───────────────────────────────────────

#[test]
fn paragraph_formatting_is_read_and_pard_resets_it() {
    let doc = parse(
        r"{\rtf1\pard\qr\li720\ri360\fi-360\sb120\sa240\sl360\slmult1\pagebb\keepn\rtlpar One\par\pard Two\par\qnatural Three\par\qj Four}",
    );
    let all: Vec<_> = paragraphs(&doc.body).collect();
    let one = &all[0].props;
    assert_eq!(one.alignment, Some(Alignment::Right));
    assert_eq!(
        (one.left_indent, one.right_indent, one.first_line_indent),
        (720, 360, -360)
    );
    assert_eq!(
        (one.space_before, one.space_after, one.line_spacing),
        (120, 240, 360)
    );
    assert!(one.line_spacing_multiple && one.page_break_before && one.keep_with_next);
    assert_eq!(one.direction, Some(Direction::RightToLeft));
    assert_eq!(all[1].props, rtf_reader::ParagraphProps::default());
    assert_eq!(all[2].props.alignment, Some(Alignment::Natural));
    assert_eq!(all[3].props.alignment, Some(Alignment::Justify));
}

#[test]
fn character_toggles_take_zero_as_off() {
    let doc = parse(
        r"{\rtf1 {\b bold\b0  plain} {\i\ul it and ul\ulnone  it} {\strike gone\strike0  back}}",
    );
    assert!(run_with(&doc, "bold").props.bold);
    assert!(!run_with(&doc, " plain").props.bold);
    let both = run_with(&doc, "it and ul").props;
    assert!(both.italic && both.underline);
    let italic_only = run_with(&doc, " it").props;
    assert!(italic_only.italic && !italic_only.underline);
    assert!(run_with(&doc, "gone").props.strike);
    assert!(!run_with(&doc, "back").props.strike);
}

#[test]
fn an_underline_or_strike_colour_is_not_a_toggle() {
    let doc = parse(r"{\rtf1 \ul \ulc0 lined\strikec0  still}");
    assert!(run_with(&doc, "lined").props.underline);
    assert!(!run_with(&doc, "still").props.strike);
}

#[test]
fn raised_and_lowered_text_share_one_axis() {
    let doc = parse(
        r"{\rtf1 3\super rd\nosupersub  H\sub 2\nosupersub O d\dn3 T \up0 c 2\up10 k\up0  e\super x\sub y\super0 z}",
    );
    let vertical = |needle: &str| run_with(&doc, needle).props.vertical;
    assert_eq!(vertical("rd"), VerticalPosition::Superscript);
    assert_eq!(vertical("2"), VerticalPosition::Subscript);
    assert_eq!(vertical("T "), VerticalPosition::Subscript);
    assert_eq!(
        vertical("c 2"),
        VerticalPosition::Baseline,
        "\\up0 ends \\dn too"
    );
    assert_eq!(vertical("k"), VerticalPosition::Superscript);
    assert_eq!(vertical("y"), VerticalPosition::Subscript);
    assert_eq!(vertical("z"), VerticalPosition::Baseline);
}

#[test]
fn capitals_hidden_text_size_colour_and_plain() {
    let doc = parse(
        r"{\rtf1{\colortbl;\red0\green0\blue0;\red0\green0\blue255;} \scaps tiny\scaps0 \caps loud\caps0 \v hidden\v0 \fs36\cf2 big blue\plain  plain}",
    );
    assert!(run_with(&doc, "tiny").props.small_caps);
    assert!(run_with(&doc, "loud").props.all_caps);
    assert!(run_with(&doc, "hidden").props.hidden);
    let big = run_with(&doc, "big blue").props;
    assert_eq!((big.font_size, big.color), (Some(36), Some(2)));
    assert_eq!(
        run_with(&doc, " plain").props,
        rtf_reader::CharProps::default()
    );
}

#[test]
fn right_to_left_runs_and_paragraphs() {
    let doc = parse(r"{\rtf1\pard\rtlpar\rtlch shalom\par\ltrpar\ltrch hello}");
    let all: Vec<_> = paragraphs(&doc.body).collect();
    assert_eq!(all[0].props.direction, Some(Direction::RightToLeft));
    assert!(run_with(&doc, "shalom").props.right_to_left);
    assert_eq!(all[1].props.direction, Some(Direction::LeftToRight));
    assert!(!run_with(&doc, "hello").props.right_to_left);
}

// ── breaks and special characters ────────────────────────────────────────────

#[test]
fn breaks_are_inline_items_and_tabs_are_characters() {
    let mut bytes = rtf(r"{\rtf1 a\line b\page c\column d\tab e");
    bytes.push(b'\t');
    bytes.extend_from_slice(b"f}");
    let doc = parse_bytes(&bytes);
    let kinds: Vec<BreakKind> = first(&doc)
        .inlines
        .iter()
        .filter_map(|i| match i {
            Inline::Break(kind) => Some(*kind),
            _ => None,
        })
        .collect();
    assert_eq!(kinds, [BreakKind::Line, BreakKind::Page, BreakKind::Column]);
    assert_eq!(body(&doc), ["a\nb\nc\nd\te\tf"]);
}

#[test]
fn control_symbols_and_named_characters() {
    let doc = parse(
        r"{\rtf1 a\~b\-c\_d \emdash\endash\lquote\rquote\ldblquote\rdblquote\bullet\emspace\enspace\qmspace \{\}\\}",
    );
    assert_eq!(
        body(&doc),
        [
            "a\u{A0}b\u{AD}c\u{2011}d \u{2014}\u{2013}\u{2018}\u{2019}\u{201C}\u{201D}\u{2022}\u{2003}\u{2002}\u{2005}{}\\"
        ]
    );
}

#[test]
fn stray_control_bytes_are_removed_and_counted_but_the_object_mark_is_kept() {
    let mut bytes = rtf(r"{\rtf1 a");
    bytes.push(0x16);
    bytes.extend_from_slice(&rtf(r"b\'0bc\uc1@u65532?d}"));
    let doc = parse_bytes(&bytes);
    assert_eq!(body(&doc), ["abc\u{FFFC}d"]);
    assert_eq!(
        doc.diagnostics.count(&Diagnostic::ControlCharacterRemoved),
        2
    );
}

#[test]
fn computed_characters_have_no_text() {
    let doc = parse(r"{\rtf1 Page \chpgn  of today \chdate .}");
    assert_eq!(body(&doc), ["Page  of today ."]);
    assert_eq!(
        doc.diagnostics.count(&Diagnostic::DynamicCharacterSkipped),
        2
    );
}

// ── fields ───────────────────────────────────────────────────────────────────

#[test]
fn a_cocoa_hyperlink_with_a_braced_instruction() {
    let doc = parse(&cocoa(
        r#"Some {\field{\*\fldinst{HYPERLINK "scrivcmt://4069A064-5921-4685-815D-06944089FD67"}}{\fldrslt anchored words}} after.\
"#,
    ));
    assert_eq!(body(&doc), ["Some anchored words after."]);
    assert_eq!(doc.fields.len(), 1);
    let field = &doc.fields[0];
    assert!(field.has_result);
    assert_eq!(
        field.kind(),
        FieldKind::Hyperlink {
            target: Some("scrivcmt://4069A064-5921-4685-815D-06944089FD67".into()),
            fragment: None
        }
    );
    assert_eq!(run_with(&doc, "anchored").field, Some(0));
    assert_eq!(run_with(&doc, "Some").field, None);
    let markers: Vec<&Inline> = first(&doc)
        .inlines
        .iter()
        .filter(|i| matches!(i, Inline::FieldStart(_) | Inline::FieldEnd(_)))
        .collect();
    assert_eq!(markers, [&Inline::FieldStart(0), &Inline::FieldEnd(0)]);
}

#[test]
fn a_windows_hyperlink_with_a_bare_instruction_keeps_its_formatted_result() {
    let doc = parse(&windows(
        r#"{\f0\fs24 See }{\field{\*\fldinst HYPERLINK "scrivlnk://D3796A92-2893-4B70-8AEA-4A8B308FB7E8"}{\fldrslt\f0\fs24\b1\i0 split}}{\f0\fs24  there.}"#,
    ));
    assert_eq!(body(&doc), ["See split there."]);
    let linked = run_with(&doc, "split");
    assert!(linked.props.bold);
    assert_eq!(linked.field, Some(0));
    assert!(matches!(
        doc.fields[0].kind(),
        FieldKind::Hyperlink { target: Some(t), .. } if t.starts_with("scrivlnk://")
    ));
}

#[test]
fn field_instructions_other_than_links() {
    let doc = parse(
        r#"{\rtf1 {\field{\*\fldinst PAGE \\* MERGEFORMAT}{\fldrslt 12}} {\field{\*\fldinst {HYPERLINK \\l "chapter-two"}}{\fldrslt jump}} {\field{\*\fldinst DATE}}.}"#,
    );
    assert_eq!(body(&doc), ["12 jump ."]);
    assert_eq!(
        doc.fields[0].kind(),
        FieldKind::Other {
            name: "PAGE".into()
        }
    );
    assert_eq!(
        doc.fields[1].kind(),
        FieldKind::Hyperlink {
            target: None,
            fragment: Some("chapter-two".into())
        }
    );
    assert!(!doc.fields[2].has_result);
}

#[test]
fn a_field_result_may_run_over_several_paragraphs() {
    let doc = parse(
        r#"{\rtf1 {\field{\*\fldinst HYPERLINK "https://example.org"}{\fldrslt one\par two}} three}"#,
    );
    assert_eq!(body(&doc), ["one", "two three"]);
    assert_eq!(run_with(&doc, "one").field, Some(0));
    assert_eq!(run_with(&doc, "two").field, Some(0));
    assert_eq!(run_with(&doc, " three").field, None);
}

// ── pictures ─────────────────────────────────────────────────────────────────

#[test]
fn a_shape_picture_keeps_its_format_sizes_name_and_bytes() {
    let doc = parse(&cocoa(
        r"Before {{\*\shppict{\pict {\*\nisusfilename Screenshot at 7.05\uc0@u8239 PM}\picw394\pich80\picwgoal3940\pichgoal800\pngblip 89504e47 0d0a1a0a}}} after",
    ));
    assert_eq!(doc.pictures.len(), 1);
    let picture = &doc.pictures[0];
    assert_eq!(picture.format, PictureFormat::Png);
    assert_eq!(picture.origin, PictureOrigin::ShapePicture);
    assert_eq!(
        picture.name.as_deref(),
        Some("Screenshot at 7.05\u{202F}PM")
    );
    assert_eq!((picture.width, picture.height), (Some(394), Some(80)));
    assert_eq!(
        (picture.goal_width, picture.goal_height),
        (Some(3940), Some(800))
    );
    assert_eq!(
        picture.data,
        [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
    );
    assert!(first(&doc).inlines.contains(&Inline::Picture(0)));
    assert_eq!(body(&doc), ["Before  after"]);
}

#[test]
fn the_fallback_picture_for_old_readers_is_not_a_second_picture() {
    let doc =
        parse(r"{\rtf1 {\*\shppict{\pict\jpegblip ffd8}}{\nonshppict{\pict\wmetafile8 0102}}.}");
    assert_eq!(doc.pictures.len(), 1);
    assert_eq!(doc.pictures[0].format, PictureFormat::Jpeg);
    assert_eq!(doc.pictures[0].data, [0xFF, 0xD8]);
}

#[test]
fn a_bare_picture_with_binary_data_and_an_odd_hex_picture() {
    let mut bytes = rtf(r"{\rtf1 {\pict\pngblip\bin3 ");
    bytes.extend_from_slice(&[b'{', 0x00, b'}']);
    bytes.extend_from_slice(&rtf(r"}{\pict\emfblip abc}.}"));
    let doc = parse_bytes(&bytes);
    assert_eq!(doc.pictures[0].origin, PictureOrigin::Bare);
    assert_eq!(doc.pictures[0].data, [b'{', 0x00, b'}']);
    assert_eq!(doc.pictures[1].format, PictureFormat::Emf);
    assert_eq!(doc.pictures[1].data, [0xAB]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::OddPictureHex), 1);
    assert_eq!(body(&doc), ["."]);
}

#[test]
fn a_drawing_shape_gives_its_picture() {
    let doc = parse(
        r"{\rtf1 x{\shp{\*\shpinst\shpleft0{\sp{\sn shapeType}{\sv 75}}{\sp{\sn pib}{\sv {\pict\pngblip 89}}}}{\shprslt {\*\do nothing}}}y{\shp{\*\shpinst{\sp{\sn fillColor}{\sv 1}}{\shptxt boxed text}}}z}",
    );
    assert_eq!(doc.pictures.len(), 1);
    assert_eq!(doc.pictures[0].origin, PictureOrigin::Shape);
    assert_eq!(body(&doc), ["xyz"]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::ShapeSkipped), 1);
    assert_eq!(doc.diagnostics.count(&Diagnostic::TextBoxSkipped), 1);
}

#[test]
fn an_embedded_object_is_read_as_the_result_it_shows() {
    let doc = parse(
        r"{\rtf1 a{\object\objemb{\*\objclass Equation.3}{\*\objdata 0102}{\result {\pict\pngblip 89}}}b{\object{\*\objdata 03}}c}",
    );
    assert_eq!(body(&doc), ["abc"]);
    assert_eq!(doc.pictures.len(), 1);
    assert_eq!(doc.pictures[0].origin, PictureOrigin::ObjectResult);
    assert_eq!(doc.diagnostics.count(&Diagnostic::ObjectReadAsResult), 1);
    assert_eq!(doc.diagnostics.count(&Diagnostic::ObjectWithoutResult), 1);
}

#[test]
fn an_rtfd_attachment_is_a_picture_by_name_and_its_marker_byte_is_not_text() {
    let mut bytes = rtf(
        r"{\rtf1\mac\ansicpg10000 Look: {{\NeXTGraphic Pasted Graphic.tif \width2560 \height1920
}",
    );
    bytes.push(0xAC);
    bytes.extend_from_slice(b"} done}");
    let doc = parse_bytes(&bytes);
    assert_eq!(body(&doc), ["Look:  done"]);
    let picture = &doc.pictures[0];
    assert_eq!(picture.format, PictureFormat::NextAttachment);
    assert_eq!(picture.name.as_deref(), Some("Pasted Graphic.tif"));
    assert_eq!(picture.goal_width, Some(2560));
}

#[test]
fn a_pdf_written_into_the_text_is_a_picture_where_it_stands() {
    // Scrivener 2 for macOS writes a PDF dropped into a document whole, in hexadecimal,
    // with its file name before the bytes.
    let doc = parse(&cocoa(
        r"See {\*\scrivenerpdf {{\*\pdffilename Harbour survey} 255044462d312e33}} here.",
    ));
    assert_eq!(body(&doc), ["See  here."]);
    assert_eq!(doc.pictures.len(), 1);
    let pdf = &doc.pictures[0];
    assert_eq!(pdf.format, PictureFormat::Pdf);
    assert_eq!(pdf.origin, PictureOrigin::EmbeddedPdf);
    assert_eq!(pdf.name.as_deref(), Some("Harbour survey"));
    assert_eq!(pdf.data, b"%PDF-1.3");
    assert!(first(&doc).inlines.contains(&Inline::Picture(0)));
    assert!(doc.diagnostics.is_empty(), "{:?}", doc.diagnostics);
}

// ── tables ───────────────────────────────────────────────────────────────────

#[test]
fn a_cocoa_table_becomes_rows_of_cells() {
    let doc = parse(&cocoa(
        "Before\\\n\\itap1\\trowd \\taflags1 \\trgaph108\\trleft-108 \\cellx4320\\cellx8640\n\\pard\\intbl\\itap1 First cell\\cell \n\\pard\\intbl\\itap1 Second\\\nline\\cell \\lastrow\\row\n\\pard After",
    ));
    assert_eq!(
        body(&doc),
        ["Before", "First cell", "Second", "line", "After"]
    );
    let Block::Table(table) = &doc.body[1] else {
        panic!("expected a table, got {:?}", doc.body[1]);
    };
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0].cells.len(), 2);
    assert_eq!(table.rows[0].cells[1].blocks.len(), 2);
    let last = paragraphs(&table.rows[0].cells[1].blocks).last();
    assert_eq!(last.map(|p| p.end), Some(ParagraphEnd::Cell));
}

#[test]
fn a_windows_table_repeats_its_row_definition_after_the_cells() {
    let doc = parse(&windows(
        "\\trowd\\trgaph108 \\cellx4320\\cellx8640\n\\pard\\plain \\intbl\\itap1 {\\f0 a}\\cell\\pard\\plain \\intbl\\itap1 {\\f0 b}\\cell{\n\\trowd\\trgaph108 \\cellx4320\\cellx8640 \\row}\n\\pard\\plain \\intbl\\itap1 {\\f0 c}\\cell\\pard\\plain \\intbl\\itap1 {\\f0 d}\\cell{\\trowd \\row}\n\\pard\\plain {\\f0 after}",
    ));
    assert_eq!(body(&doc), ["a", "b", "c", "d", "after"]);
    let Block::Table(table) = &doc.body[0] else {
        panic!("expected a table");
    };
    assert_eq!(table.rows.len(), 2);
}

#[test]
fn a_nested_table_is_a_block_of_its_cell() {
    let doc = parse(
        r"{\rtf1\trowd\cellx5000\pard\intbl outer\par\pard\intbl\itap2 inner\nestcell{\nonesttables \par}{\*\nesttableprops\trowd\cellx2000\nestrow}{\nonesttables \par}\pard\intbl\cell\row\pard after}",
    );
    assert_eq!(body(&doc), ["outer", "inner", "", "after"]);
    let Block::Table(outer) = &doc.body[0] else {
        panic!("expected a table");
    };
    let cell = &outer.rows[0].cells[0];
    let Block::Table(inner) = &cell.blocks[1] else {
        panic!("expected a nested table, got {:?}", cell.blocks[1]);
    };
    assert_eq!(texts(&inner.rows[0].cells[0].blocks), ["inner"]);
}

#[test]
fn a_table_nested_past_the_limit_is_kept_at_the_deepest_level() {
    let doc = parse(r"{\rtf1\pard\intbl\itap40 deep\nestcell\pard after}");
    assert_eq!(body(&doc), ["deep", "after"]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::TableNestingFlattened), 1);
}

// ── lists ────────────────────────────────────────────────────────────────────

#[test]
fn a_list_paragraph_keeps_its_label_and_reaches_its_level() {
    let mut bytes = rtf(
        r"{\rtf1{\*\listtable{\list\listtemplateid1\listhybrid{\listlevel\levelnfc0\levelstartat1{\*\levelmarker \{decimal\}.}{\leveltext\leveltemplateid1\'02\'00.;}{\levelnumbers\'01;}}{\listlevel\levelnfc23{\*\levelmarker \{disc\}}{\leveltext\'01\uc0@u8226 ;}{\levelnumbers;}}{\listname ;}\listid7}}{\*\listoverridetable{\listoverride\listid7\listoverridecount0\ls1}}\pard\ls1\ilvl0 {\listtext",
    );
    bytes.push(b'\t');
    bytes.extend_from_slice(b"1.");
    bytes.push(b'\t');
    bytes.extend_from_slice(&rtf(
        r"}Select the text.\par\pard\ilvl1\ls1 {\listtext\uc0@u8226 }A point.}",
    ));
    let doc = parse_bytes(&bytes);
    assert_eq!(body(&doc), ["Select the text.", "A point."]);
    let all: Vec<_> = paragraphs(&doc.body).collect();
    assert_eq!(all[0].label.as_deref(), Some("\t1.\t"));
    let first_list = all[0].props.list.map(|l| (l.override_number, l.level));
    assert_eq!(first_list, Some((1, 0)));
    let second = all[1].props.list;
    assert_eq!(
        second.map(|l| l.level),
        Some(1),
        "\\ilvl before \\ls still counts"
    );
    let numbered = all
        .iter()
        .map(|p| p.props.list.and_then(|l| doc.list_level(l)?.is_numbered()))
        .collect::<Vec<_>>();
    assert_eq!(numbered, [Some(true), Some(false)]);
    assert_eq!(doc.lists[0].levels[0].marker.as_deref(), Some("{decimal}."));
}

#[test]
fn word_95_numbering() {
    let doc = parse(r"{\rtf1\pard{\*\pn\pnlvlblt\pnf1{\pntxtb \'b7}}{\pntext\f1\'b7\tab}Item\par}");
    let paragraph = first(&doc);
    let numbering = paragraph.props.legacy_numbering;
    assert!(numbering.is_some_and(|n| n.bullet));
    assert_eq!(paragraph.label.as_deref(), Some("\u{B7}\t"));
    assert_eq!(body(&doc), ["Item"]);
}

// ── notes and comments ───────────────────────────────────────────────────────

#[test]
fn a_footnote_is_a_story_of_its_own_cited_from_the_text() {
    let doc = parse(
        r"{\rtf1\pard\plain\b Mead's study.{\cs16\super \chftn {\footnote \pard\plain \s15 {\up6\chftn }See the bibliography.\par Second paragraph.}} It was her work.\par}",
    );
    assert_eq!(body(&doc), ["Mead's study. It was her work."]);
    assert_eq!(doc.notes.len(), 1);
    assert_eq!(doc.notes[0].kind, NoteKind::Footnote);
    assert_eq!(
        texts(&doc.notes[0].blocks),
        ["See the bibliography.", "Second paragraph."]
    );
    assert!(first(&doc).inlines.contains(&Inline::NoteReference(0)));
    // A note does not take the formatting of the text around it.
    let note_run = paragraphs(&doc.notes[0].blocks)
        .next()
        .and_then(|p| p.runs().next().cloned());
    assert!(note_run.is_some_and(|r| !r.props.bold));
    // The text after the note is still bold.
    assert!(run_with(&doc, "It was").props.bold);
}

#[test]
fn an_endnote_is_a_footnote_marked_ftnalt() {
    let doc = parse(r"{\rtf1 Text.{\footnote\ftnalt At the end.}}");
    assert_eq!(doc.notes[0].kind, NoteKind::Endnote);
    assert_eq!(texts(&doc.notes[0].blocks), ["At the end."]);
}

#[test]
fn a_comment_has_its_author_its_range_and_its_own_text() {
    let doc = parse(
        r"{\rtf1 An example of {\*\atrfstart 1}a paradigm{\*\atrfend 1} might be biology.{\cs15\v\fs16{\*\atnid JD}{\*\atnauthor John Doe}\chatn {\*\annotation{\*\atndate 1180187342}{\*\atnref 1}\pard\plain {\cs15\fs16 \chatn }How about social science?}}\par}",
    );
    assert_eq!(body(&doc), ["An example of a paradigm might be biology."]);
    assert_eq!(doc.annotations.len(), 1);
    let comment = &doc.annotations[0];
    assert_eq!(comment.initials.as_deref(), Some("JD"));
    assert_eq!(comment.author.as_deref(), Some("John Doe"));
    assert_eq!(comment.date.as_deref(), Some("1180187342"));
    assert_eq!(comment.reference.as_deref(), Some("1"));
    assert_eq!(texts(&comment.blocks), ["How about social science?"]);
    let inlines = &first(&doc).inlines;
    assert!(inlines.contains(&Inline::AnnotationRangeStart("1".into())));
    assert!(inlines.contains(&Inline::AnnotationRangeEnd("1".into())));
    assert!(inlines.contains(&Inline::AnnotationReference(0)));
}

// ── what a dialect layer above reads ─────────────────────────────────────────

#[test]
fn inline_notes_written_as_text_stay_text_and_keep_their_groups_apart() {
    // Scrivener for Windows' split form: the opening marker ends one group, the body and
    // the closing marker sit in the next.
    let doc = parse(&windows(
        r"{\f0\fs24\b0\i0 The lamp burned low beside a map.\{\\Scrv_fn=}{\f0\fs24\b0\i0 A note on the lamp.\\end_Scrv_fn\}}",
    ));
    assert_eq!(
        body(&doc),
        ["The lamp burned low beside a map.{\\Scrv_fn=A note on the lamp.\\end_Scrv_fn}"]
    );
    let runs = runs(&doc);
    assert_eq!(runs.len(), 2);
    assert_ne!(runs[0].group, runs[1].group);
    assert!(runs[0].text.ends_with("{\\Scrv_fn="));
    assert!(runs[1].text.ends_with("\\end_Scrv_fn}"));
}

#[test]
fn text_is_one_run_per_group_and_formatting() {
    let doc = parse(r"{\rtf1 {a}{a}{\b b}{\b b\b0 c}}");
    let pieces: Vec<(&str, bool)> = runs(&doc)
        .iter()
        .map(|r| (r.text.as_str(), r.props.bold))
        .collect();
    assert_eq!(
        pieces,
        [
            ("a", false),
            ("a", false),
            ("b", true),
            ("b", true),
            ("c", false)
        ]
    );
}

// ── damaged and hostile input ────────────────────────────────────────────────

#[test]
fn a_file_that_is_not_rtf_is_refused() {
    assert_eq!(rtf_reader::parse(b"Hello"), Err(Refusal::NotRtf));
    assert_eq!(rtf_reader::parse(b""), Err(Refusal::NotRtf));
    assert!(rtf_reader::parse(b"\xEF\xBB\xBF \n{\\rtf1 ok}").is_ok());
}

#[test]
fn missing_braces_are_closed_at_the_end_and_counted() {
    let doc = parse(r"{\rtf1 {\b open\par unclosed");
    assert_eq!(body(&doc), ["open", "unclosed"]);
    assert!(run_with(&doc, "unclosed").props.bold);
    assert_eq!(doc.diagnostics.count(&Diagnostic::UnclosedGroup), 2);
}

#[test]
fn a_field_left_open_at_the_end_closes_in_the_paragraph_it_opened_in() {
    // A file cut short inside a link's text: the link ends where the text does, not in a
    // paragraph of its own after it.
    let doc = parse(
        r#"{\rtf1 {\qc See {\field{\*\fldinst HYPERLINK "scrivcmt://4069A064"}{\fldrslt the harbour"#,
    );
    let all: Vec<_> = paragraphs(&doc.body).collect();
    assert_eq!(all.len(), 1, "{all:?}");
    assert_eq!(all[0].inlines.last(), Some(&Inline::FieldEnd(0)));
    assert_eq!(all[0].props.alignment, Some(Alignment::Center));
    assert_eq!(body(&doc), ["See the harbour"]);
}

#[test]
fn a_richedit_trailer_is_not_counted_but_other_trailing_bytes_are() {
    let doc = parse_bytes(b"{\\rtf1 text}\r\n\0");
    assert!(doc.diagnostics.is_empty(), "{:?}", doc.diagnostics);
    let doc = parse_bytes(b"{\\rtf1 text}} more");
    assert_eq!(body(&doc), ["text"]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::BytesAfterDocument), 2);
}

#[test]
fn a_truncated_hex_escape_and_a_lying_bin() {
    let doc = parse(r"{\rtf1 x\'a}");
    assert_eq!(body(&doc), ["xa"]);
    assert_eq!(doc.diagnostics.count(&Diagnostic::BadHexEscape), 1);
    let doc = parse(r"{\rtf1 {\pict\bin99 ab}}");
    assert_eq!(doc.diagnostics.count(&Diagnostic::BinaryTruncated), 1);
}

#[test]
fn the_last_paragraph_keeps_the_formatting_of_its_document() {
    let doc = parse(r"{\rtf1\pard\qc centred to the end}");
    assert_eq!(first(&doc).props.alignment, Some(Alignment::Center));
    assert_eq!(first(&doc).end, ParagraphEnd::Implicit);
}
