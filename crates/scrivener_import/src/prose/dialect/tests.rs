// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The dialect pass, construct by construct, on RTF written here the way Scrivener's two
//! writers write it. Every fixture is a Rust raw string: `\{\\Scrv_fn=` is RTF for the
//! characters `{\Scrv_fn=`.
//!
//! A paragraph is rendered for the assertions with its objects spelled out: `{fn0}` a
//! footnote's reference, `{c0}` a comment's place, `{[}` and `{]}` a comment range's ends,
//! `{pic0}` a picture.

use super::*;
use crate::prose::sidecar::{Entry, Sidecar};
use crate::prose::styles::{StreamStyles, StyleDef, StyleTable, StyleType};

fn parse(rtf: &str) -> Document {
    match rtf_reader::parse(rtf.as_bytes()) {
        Ok(doc) => doc,
        Err(e) => panic!("the fixture is RTF: {e}"),
    }
}

fn input() -> Input<'static> {
    Input {
        role: Role::Stream,
        styles: StreamStyles::default(),
        sidecar: None,
        title: None,
        footnote_marker: None,
        scene_breaks: SceneBreaks::default(),
        annotations_as_prose: false,
    }
}

fn render(paragraph: &Paragraph) -> String {
    let mut out = String::new();
    for inline in &paragraph.inlines {
        match inline {
            Inline::Text(run) => out.push_str(&run.text),
            Inline::NoteReference(n) => out.push_str(&format!("{{fn{n}}}")),
            Inline::AnnotationReference(n) => out.push_str(&format!("{{c{n}}}")),
            Inline::AnnotationRangeStart(_) => out.push_str("{[}"),
            Inline::AnnotationRangeEnd(_) => out.push_str("{]}"),
            Inline::Picture(n) => out.push_str(&format!("{{pic{n}}}")),
            _ => {}
        }
    }
    out
}

/// The top-level paragraphs, rendered, blank ones left out as the mapping leaves them.
fn texts(doc: &Document) -> Vec<String> {
    doc.body
        .iter()
        .filter_map(|b| match b {
            Block::Paragraph(p) => Some(render(p)),
            Block::Table(_) => None,
        })
        .filter(|t| !t.trim().is_empty())
        .collect()
}

fn blocks_text(blocks: &[Block]) -> Vec<String> {
    rtf_reader::paragraphs(blocks)
        .map(render)
        .filter(|t| !t.trim().is_empty())
        .collect()
}

fn note(doc: &Document, n: usize) -> Vec<String> {
    blocks_text(&doc.notes[n].blocks)
}

fn comment(doc: &Document, n: usize) -> Vec<String> {
    blocks_text(&doc.annotations[n].blocks)
}

fn run(rtf: &str) -> Rewritten {
    rewrite(parse(rtf), &input())
}

// ── inline footnotes ─────────────────────────────────────────────────────────────

/// Scrivener for Windows writes the opener and the closer in different groups from the
/// note's text: the pass reads the characters, whatever groups hold them.
#[test]
fn a_footnote_split_across_groups_becomes_a_note_cited_where_it_opened() {
    let out = run(
        r"{\rtf1 {\f0 The ferry was late\{\\Scrv_fn=}{\f1 It always was.\\end_Scrv_fn\}}{\f0  again.}\par}",
    );
    assert_eq!(texts(&out.doc), ["The ferry was late{fn0} again."]);
    assert_eq!(note(&out.doc, 0), ["It always was."]);
    assert_eq!(out.found.unmatched, 0);
}

/// A note whose formatting changes is written as two pairs with nothing between: one note,
/// each part keeping its formatting (`20` + superscript `th`).
#[test]
fn pairs_split_by_formatting_are_one_note_with_their_formatting() {
    let out = run(
        r"{\rtf1 Born in the \{\\Scrv_fn=20\\end_Scrv_fn\}{\super \{\\Scrv_fn=th\\end_Scrv_fn\}} century.\par}",
    );
    assert_eq!(texts(&out.doc), ["Born in the {fn0} century."]);
    assert_eq!(note(&out.doc, 0), ["20th"]);
    assert_eq!(out.found.merged, 1);
    let raised: Vec<&str> = rtf_reader::paragraphs(&out.doc.notes[0].blocks)
        .flat_map(|p| p.runs())
        .filter(|r| r.props.vertical == rtf_reader::VerticalPosition::Superscript)
        .map(|r| r.text.as_str())
        .collect();
    assert_eq!(raised, ["th"]);
}

/// Scrivener for Windows writes a note of several paragraphs one pair per paragraph: only
/// paragraph breaks between the pairs, so they are one note, its breaks its own (D5.3).
#[test]
fn one_pair_per_paragraph_is_one_note_of_several_paragraphs() {
    let out = run(
        r"{\rtf1 A desk\{\\Scrv_fn=ENDNOTES\\end_Scrv_fn\}\par\par \{\\Scrv_fn=The first remark.\\end_Scrv_fn\}\par \{\\Scrv_fn=The second remark.\\end_Scrv_fn\}\par After it.\par}",
    );
    assert_eq!(texts(&out.doc), ["A desk{fn0}", "After it."]);
    assert_eq!(
        note(&out.doc, 0),
        ["ENDNOTES", "The first remark.", "The second remark."]
    );
    assert_eq!(out.found.joined_across, 2);
    assert_eq!(out.doc.notes.len(), 1);
}

/// A space between two notes keeps them apart: they are two remarks.
#[test]
fn notes_a_space_apart_stay_two_notes() {
    let out = run(
        r"{\rtf1 See\{\\Scrv_fn=Brown 2008, 4.\\end_Scrv_fn\} \{\\Scrv_fn=Another remark.\\end_Scrv_fn\}\par}",
    );
    assert_eq!(texts(&out.doc), ["See{fn0} {fn1}"]);
    assert_eq!(note(&out.doc, 0), ["Brown 2008, 4."]);
    assert_eq!(note(&out.doc, 1), ["Another remark."]);
}

/// A break just inside the closer is the text's own (the Mac writer closes a note after
/// the paragraph mark): the text around the note keeps its two paragraphs (D5.4).
#[test]
fn a_break_at_a_notes_edge_belongs_to_the_text_around_it() {
    let out = run(
        r"{\rtf1 More generally.\{\\Scrv_fn=A source.\par \\end_Scrv_fn\}That is, the next one.\par}",
    );
    assert_eq!(
        texts(&out.doc),
        ["More generally.{fn0}", "That is, the next one."]
    );
    assert_eq!(note(&out.doc, 0), ["A source."]);
}

/// A break inside a note, away from its edges, is the note's: the text around it joins.
#[test]
fn a_break_inside_a_note_is_the_notes_and_the_text_around_it_joins() {
    let out = run(r"{\rtf1 Before\{\\Scrv_fn=One.\par Two.\\end_Scrv_fn\} after.\par}");
    assert_eq!(texts(&out.doc), ["Before{fn0} after."]);
    assert_eq!(note(&out.doc, 0), ["One.", "Two."]);
}

#[test]
fn a_blank_note_is_dropped_and_counted() {
    let out = run(r"{\rtf1 Text\{\\Scrv_fn=  \\end_Scrv_fn\} goes on.\par}");
    assert_eq!(texts(&out.doc), ["Text goes on."]);
    assert!(out.doc.notes.is_empty());
    assert_eq!(out.found.blank_notes, 1);
}

// ── markers that do not pair ─────────────────────────────────────────────────────

/// What does not pair stays the text it is, for the writer to see, and is counted (D5.2).
#[test]
fn markers_that_do_not_pair_stay_literal_and_are_counted() {
    let out = run(
        r"{\rtf1 A stray \\end_Scrv_fn\} closer.\par An \{\\Scrv_fn=open note never closed.\par}",
    );
    assert_eq!(
        texts(&out.doc),
        [
            r"A stray \end_Scrv_fn} closer.",
            r"An {\Scrv_fn=open note never closed."
        ]
    );
    assert_eq!(out.found.unmatched, 2);
    assert!(out.doc.notes.is_empty());
}

/// Scrivener's notes do not nest: an opener inside an open note is literal, and the first
/// closer closes the outer note.
#[test]
fn a_note_inside_a_note_is_literal() {
    let out = run(
        r"{\rtf1 X\{\\Scrv_fn=outer \{\\Scrv_fn=inner\\end_Scrv_fn\} tail\\end_Scrv_fn\}.\par}",
    );
    assert_eq!(out.doc.notes.len(), 1);
    assert_eq!(note(&out.doc, 0), [r"outer {\Scrv_fn=inner"]);
    assert_eq!(texts(&out.doc), [r"X{fn0} tail\end_Scrv_fn}."]);
    assert_eq!(out.found.unmatched, 2);
}

#[test]
fn an_unknown_scrivener_word_stays_literal() {
    let out = run(r"{\rtf1 A \{\\Scrv_comm=x\} b.\par}");
    assert_eq!(texts(&out.doc), [r"A {\Scrv_comm=x} b."]);
    assert_eq!(out.found.unmatched, 1);
}

// ── inline annotations ───────────────────────────────────────────────────────────

/// An annotation inside a paragraph becomes a comment on the nearest word, the one before
/// it; its text leaves the prose (D5.6).
#[test]
fn an_annotation_inside_a_paragraph_marks_the_word_before_it() {
    let out = run(
        r"{\rtf1 The keeper \{\\Scrv_annot \\color=\{\\R=1.0\\G=0.0\\B=0.0\} \\text=check this date\\end_Scrv_annot\} counted.\par}",
    );
    assert_eq!(texts(&out.doc), ["The {[}{c0}keeper{]}  counted."]);
    assert_eq!(comment(&out.doc, 0), ["check this date"]);
    assert_eq!(out.found.annotations, 1);
    assert_eq!(out.found.annotation_colours, 1);
    assert!(out.doc.annotations[0].reference.is_some());
}

/// Opening its paragraph, an annotation marks the word after it.
#[test]
fn an_annotation_opening_a_paragraph_marks_the_word_after_it() {
    let out = run(
        r"{\rtf1 \{\\Scrv_annot \\color=\{#AF1C00\} \\text=iOS colour\\end_Scrv_annot\}Nightfall came.\par}",
    );
    assert_eq!(texts(&out.doc), ["{[}{c0}Nightfall{]} came."]);
    assert_eq!(comment(&out.doc, 0), ["iOS colour"]);
    assert_eq!(out.found.annotation_colours, 1);
}

/// An annotation filling whole paragraphs becomes a comment on the next paragraph, and the
/// paragraph it filled goes: kept blank, it would read as a blank line.
#[test]
fn a_whole_paragraph_annotation_goes_to_the_next_paragraph() {
    let out = run(
        r"{\rtf1 First scene.\par \{\\Scrv_annot \\text=Move this later.\\end_Scrv_annot\}\par Second scene.\par}",
    );
    assert_eq!(texts(&out.doc), ["First scene.", "{c0}Second scene."]);
    assert_eq!(
        out.doc
            .body
            .iter()
            .filter(|b| matches!(b, Block::Paragraph(_)))
            .count(),
        2,
        "the emptied paragraph is gone, not left blank"
    );
    assert_eq!(comment(&out.doc, 0), ["Move this later."]);
}

/// With nothing after it, the comment goes to the paragraph before.
#[test]
fn a_last_whole_paragraph_annotation_goes_to_the_paragraph_before() {
    let out =
        run(r"{\rtf1 The only scene.\par \{\\Scrv_annot \\text=A remark.\\end_Scrv_annot\}\par}");
    assert_eq!(texts(&out.doc), ["The only scene{c0}."]);
}

/// Annotations kept as prose (a document holding nothing else, X-03): markers gone, words
/// kept where they are.
#[test]
fn annotations_as_prose_keep_their_words() {
    let doc = parse(
        r"{\rtf1 \{\\Scrv_annot \\color=\{\\R=1\\G=0\\B=0\} \\text=Endnotes will go here.\\end_Scrv_annot\}\par}",
    );
    let out = rewrite(
        doc,
        &Input {
            annotations_as_prose: true,
            ..input()
        },
    );
    assert_eq!(texts(&out.doc), ["Endnotes will go here."]);
    assert!(out.doc.annotations.is_empty());
}

#[test]
fn preserve_formatting_markers_go_and_the_text_stays() {
    let out = run(r"{\rtf1 A \{\\Scrv_ps=quoted {\i passage}\\end_Scrv_ps\} ends.\par}");
    assert_eq!(texts(&out.doc), ["A quoted passage ends."]);
    assert_eq!(out.found.preserve_formatting, 1);
}

// ── style markers ────────────────────────────────────────────────────────────────

fn style_table() -> StyleTable {
    let mut table = StyleTable::default();
    for (id, name, kind, heading) in [
        ("S0", "Heading 1", StyleType::Both, Some(1)),
        ("S1", "Block Quote", StyleType::Paragraph, None),
        ("S2", "Code Span", StyleType::Character, None),
        ("S3", "Verse", StyleType::Paragraph, None),
        ("S4", "Emphasis", StyleType::Character, None),
    ] {
        table.insert(
            id,
            StyleDef {
                name: name.to_string(),
                kind,
                heading,
                compile: None,
            },
        );
    }
    table
}

/// Markers always go; a heading is a heading inside the text, a block quote a quote, code
/// code, and every other style only a name the report lists (D5.8). The Mac writer closes
/// a paragraph style at the start of the next paragraph, the Windows one before the mark.
#[test]
fn style_markers_give_paragraphs_their_kind_whichever_writer_placed_them() {
    let table = style_table();
    let list: Vec<String> = ["S0", "S1", "S2", "S3", "S4"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let doc = parse(
        r"{\rtf1 <$Scr_H::1><$Scr_Ps::0>A Heading\par <!$Scr_H::1><!$Scr_Ps::0><$Scr_Ps::1>Quoted words.<!$Scr_Ps::1>\par Type <$Scr_Cs::2>make<!$Scr_Cs::2> and <$Scr_Cs::4>stress<!$Scr_Cs::4>.\par <$Scr_Ps::3>A verse line.<!$Scr_Ps::3>\par <$Scr_Ps::9>Unknown.<!$Scr_Ps::9>\par}",
    );
    let out = rewrite(
        doc,
        &Input {
            styles: StreamStyles {
                list: Some(&list),
                table: Some(&table),
            },
            ..input()
        },
    );
    assert_eq!(
        texts(&out.doc),
        [
            "A Heading",
            "Quoted words.",
            "Type make and stress.",
            "A verse line.",
            "Unknown."
        ]
    );
    let style_of = |n: usize| -> Option<String> {
        let Block::Paragraph(p) = &out.doc.body[n] else {
            return None;
        };
        let number = p.props.style?;
        out.doc
            .styles
            .iter()
            .find(|s| s.number == number)
            .map(|s| s.name.clone())
    };
    assert_eq!(style_of(0).as_deref(), Some("heading 1"));
    assert_eq!(style_of(1).as_deref(), Some("Block Quote"));
    assert_eq!(style_of(2), None);
    assert_eq!(style_of(3), None, "a style with no meaning here gives none");
    let code: Vec<String> = match &out.doc.body[2] {
        Block::Paragraph(p) => p
            .runs()
            .filter(|r| r.props.font == out.code_font)
            .map(|r| r.text.clone())
            .collect(),
        Block::Table(_) => Vec::new(),
    };
    assert_eq!(code, ["make"]);
    assert_eq!(out.found.styles_not_carried.get("Verse"), Some(&1));
    assert_eq!(out.found.styles_not_carried.get("Emphasis"), Some(&6));
    assert_eq!(out.found.styles_unresolved, 1);
}

/// With no style list, a marker names nothing: removed and counted, never guessed.
#[test]
fn a_marker_without_a_style_list_is_removed_and_counted() {
    let out = run(r"{\rtf1 <$Scr_Ps::0>Plain.<!$Scr_Ps::0><$ScrKeepWithNext>\par}");
    assert_eq!(texts(&out.doc), ["Plain."]);
    assert_eq!(out.found.styles_unresolved, 1);
}

// ── fields: comments, linked footnotes, links ────────────────────────────────────

fn sidecar() -> Sidecar {
    Sidecar {
        entries: vec![
            Entry {
                id: Some("AAAAAAAA-0000-0000-0000-000000000001".to_string()),
                footnote: false,
                author: None,
                coloured: true,
                body: r"{\rtf1 Too soon for gulls.\par Pradel Ianto, 8/24/16, 3:55 PM}".to_string(),
                range: None,
            },
            Entry {
                id: Some("AAAAAAAA-0000-0000-0000-000000000002".to_string()),
                footnote: true,
                author: None,
                coloured: true,
                body: r"{\rtf1 Smith 2001, p. 4.}".to_string(),
                range: None,
            },
            Entry {
                id: Some("AAAAAAAA-0000-0000-0000-000000000003".to_string()),
                footnote: false,
                author: Some("Ann".to_string()),
                coloured: false,
                body: r"{\rtf1 Never anchored.}".to_string(),
                range: None,
            },
        ],
        links: Vec::new(),
    }
}

/// A comment field is a range over its words, its edge spaces left out; a linked footnote
/// is cited after its words; an entry the text never anchors lands on the first paragraph
/// (X-03). A typed "Name, date, time" last line names the author and stays in the body.
#[test]
fn comment_and_footnote_fields_become_ranges_and_references() {
    let sidecar = sidecar();
    let doc = parse(
        r#"{\rtf1 He left {\field{\*\fldinst{HYPERLINK "scrivcmt://aaaaaaaa-0000-0000-0000-000000000001"}}{\fldrslt  at dawn }}for the coast{\field{\*\fldinst HYPERLINK "scrivcmt://AAAAAAAA-0000-0000-0000-000000000002"}{\fldrslt harbour}}.\par Second.\par}"#,
    );
    let out = rewrite(
        doc,
        &Input {
            sidecar: Some(&sidecar),
            ..input()
        },
    );
    assert_eq!(
        texts(&out.doc),
        [
            "{c1}He left  {[}{c0}at dawn{]} for the coastharbour{fn0}.",
            "Second."
        ]
    );
    assert_eq!(
        comment(&out.doc, 0),
        ["Too soon for gulls.", "Pradel Ianto, 8/24/16, 3:55 PM"]
    );
    assert_eq!(
        out.doc.annotations[0].author.as_deref(),
        Some("Pradel Ianto")
    );
    assert_eq!(out.doc.annotations[1].author.as_deref(), Some("Ann"));
    assert_eq!(note(&out.doc, 0), ["Smith 2001, p. 4."]);
    assert_eq!(out.found.comments_without_anchor, 1);
    assert_eq!(out.found.comment_colours, 1);
    assert!(
        rtf_reader::paragraphs(&out.doc.body)
            .flat_map(|p| p.runs())
            .all(|r| r.field.is_none()),
        "the fields' runs are plain text now"
    );
}

#[test]
fn a_field_naming_no_entry_keeps_its_words_and_is_counted() {
    let out = run(
        r#"{\rtf1 A {\field{\*\fldinst{HYPERLINK "scrivcmt://AAAAAAAA-0000-0000-0000-00000000000F"}}{\fldrslt word}} here.\par}"#,
    );
    assert_eq!(texts(&out.doc), ["A word here."]);
    assert_eq!(out.found.anchors_without_entry, 1);
}

/// A project that anchors new footnotes to a marker glyph: the glyph is its mark, not
/// words, and the reference takes its place.
#[test]
fn a_footnote_marker_glyph_is_replaced_by_the_reference() {
    let sidecar = sidecar();
    let doc = parse(
        r#"{\rtf1 Claim{\field{\*\fldinst{HYPERLINK "scrivcmt://AAAAAAAA-0000-0000-0000-000000000002"}}{\fldrslt *}}.\par}"#,
    );
    let out = rewrite(
        doc,
        &Input {
            sidecar: Some(&sidecar),
            footnote_marker: Some("*"),
            ..input()
        },
    );
    // The two comments the text never anchored sit on the first paragraph.
    assert_eq!(texts(&out.doc)[0], "{c1}{c0}Claim{fn0}.");
}

/// A link to another document keeps its words, loses its link, and hands its target up;
/// a web link stays a link for the mapping.
#[test]
fn document_links_keep_their_words_and_report_their_targets() {
    let out = run(
        r#"{\rtf1 See {\field{\*\fldinst{HYPERLINK "scrivlnk://bbbbbbbb-0000-0000-0000-000000000001"}}{\fldrslt Mara}} and {\field{\*\fldinst{HYPERLINK "https://example.org"}}{\fldrslt the site}}.\par}"#,
    );
    assert_eq!(texts(&out.doc), ["See Mara and the site."]);
    assert_eq!(out.links, ["BBBBBBBB-0000-0000-0000-000000000001"]);
    let linked: Vec<&str> = rtf_reader::paragraphs(&out.doc.body)
        .flat_map(|p| p.runs())
        .filter(|r| r.field.is_some())
        .map(|r| r.text.as_str())
        .collect();
    assert_eq!(linked, ["the site"]);
}

// ── format 1.0 ranges ────────────────────────────────────────────────────────────

/// A format 1.0 comment placed by its range, counted in UTF-16 units over the text with
/// the inline notes' markers left out and their text in (D5.11).
#[test]
fn a_range_comment_lands_on_its_words_with_inline_markers_left_out() {
    // "Hi\{\\Scrv_fn=ab\\end_Scrv_fn\} brave world": the note's text counts, its markers
    // do not, so "brave" starts at 2 + 2 + 1 = 5.
    let sidecar = Sidecar {
        entries: vec![Entry {
            id: None,
            footnote: false,
            author: None,
            coloured: true,
            body: r"{\rtf1 Braver still.}".to_string(),
            range: Some((5, 5)),
        }],
        links: Vec::new(),
    };
    let doc = parse(r"{\rtf1 Hi\{\\Scrv_fn=ab\\end_Scrv_fn\} brave world.\par}");
    let out = rewrite(
        doc,
        &Input {
            sidecar: Some(&sidecar),
            ..input()
        },
    );
    assert_eq!(texts(&out.doc), ["Hi{fn0} {[}{c0}brave{]} world."]);
    assert!(
        out.found.range_readings.is_empty(),
        "the first reading fits"
    );
    assert_eq!(out.found.ranges_approximate, 0);
}

/// A range that only fits with the markers counted is placed by that reading, and the
/// report says which.
#[test]
fn a_range_that_fits_another_reading_says_which() {
    let sidecar = Sidecar {
        entries: vec![Entry {
            id: None,
            footnote: true,
            author: None,
            coloured: false,
            body: r"{\rtf1 A note.}".to_string(),
            // "Hi{\Scrv_fn=ab\end_Scrv_fn} brave": with the 10 + 13 marker characters
            // counted, "brave" starts at 2 + 10 + 2 + 13 + 1 = 28.
            range: Some((28, 5)),
        }],
        links: Vec::new(),
    };
    let doc = parse(r"{\rtf1 Hi\{\\Scrv_fn=ab\\end_Scrv_fn\} brave world.\par}");
    let out = rewrite(
        doc,
        &Input {
            sidecar: Some(&sidecar),
            ..input()
        },
    );
    assert_eq!(texts(&out.doc), ["Hi{fn0} brave{fn1} world."]);
    assert_eq!(
        out.found
            .range_readings
            .get(&RangeReadingKey::MarkersCounted),
        Some(&1)
    );
}

// ── pictures, placeholders ───────────────────────────────────────────────────────

#[test]
fn a_picture_linked_to_a_project_item_becomes_a_picture_and_one_on_disk_is_named() {
    let out = run(
        r"{\rtf1 Map: \{$SCRImageLink[w:120;h:80]=$PROJECT://682DB6B7-3F22-4ED6-8E67-DF568697A79F.png\} and \{$SCRImageLink[w:14;h:14]=C:/Users/someone/Pictures/logo.svg\}.\par}",
    );
    let picture = out.image_links.first().map(|l| l.picture);
    assert_eq!(
        out.image_links
            .iter()
            .map(|l| l.uuid.as_str())
            .collect::<Vec<_>>(),
        ["682DB6B7-3F22-4ED6-8E67-DF568697A79F"]
    );
    let id = picture.unwrap_or(usize::MAX);
    assert_eq!(texts(&out.doc), [format!("Map: {{pic{id}}} and .")]);
    assert_eq!(out.doc.pictures[id].goal_width, Some(120 * 15));
    assert_eq!(
        out.found.linked_on_disk,
        ["logo.svg"],
        "a file name, never a path"
    );
}

#[test]
fn placeholders_stay_as_written_and_are_counted_by_name() {
    let out = run(r"{\rtf1 Chapter <$n> of <$projecttitle> by <$custom:Author>.\par}");
    assert_eq!(
        texts(&out.doc),
        ["Chapter <$n> of <$projecttitle> by <$custom:Author>."]
    );
    assert_eq!(out.found.placeholders.get("n"), Some(&1));
    assert_eq!(out.found.placeholders.get("projecttitle"), Some(&1));
    assert_eq!(out.found.placeholders.get("custom"), Some(&1));
}

// ── tables, scene breaks ─────────────────────────────────────────────────────────

/// A cell is a story of its own: a note inside it is read there.
#[test]
fn a_note_inside_a_table_cell_is_read_in_the_cell() {
    let out = run(
        r"{\rtf1 Before.\par\trowd\cellx2000\cellx4000\pard\intbl Cell one\{\\Scrv_fn=in a cell\\end_Scrv_fn\}\cell\pard\intbl Two\cell\row\pard After.\par}",
    );
    assert_eq!(texts(&out.doc), ["Before.", "After."]);
    assert_eq!(note(&out.doc, 0), ["in a cell"]);
    let cells: Vec<String> = rtf_reader::paragraphs(&out.doc.body).map(render).collect();
    assert!(cells.contains(&"Cell one{fn0}".to_string()), "{cells:?}");
    assert_eq!(out.found.tables, 1);
}

fn prose_line(seed: usize) -> String {
    format!(
        "{} The keeper counted the ships before the tide turned, and wrote the weather down.",
        ["One.", "Two.", "Three.", "Four.", "Five.", "Six."][seed % 6]
    )
}

/// In a manuscript document of prose, an isolated blank line between two paragraphs is a
/// scene break (D5.20, UQ-33: converted by default).
#[test]
fn an_isolated_blank_line_between_prose_becomes_a_scene_break() {
    let mut rtf = String::from(r"{\rtf1 ");
    for i in 0..3 {
        rtf.push_str(&prose_line(i));
        rtf.push_str(r"\par ");
    }
    rtf.push_str(r"\par ");
    for i in 3..6 {
        rtf.push_str(&prose_line(i));
        rtf.push_str(r"\par ");
    }
    rtf.push('}');
    let breaks = SceneBreaks {
        manuscript: true,
        blank_lines: true,
        ornaments: false,
    };
    let out = rewrite(
        parse(&rtf),
        &Input {
            scene_breaks: breaks,
            ..input()
        },
    );
    let lines = texts(&out.doc);
    assert_eq!(lines.len(), 7);
    assert_eq!(lines[3], "* * *");
    assert_eq!(out.found.blank_lines_breaks, 1);

    // Off, the line is removed with the others.
    let out = rewrite(
        parse(&rtf),
        &Input {
            scene_breaks: SceneBreaks {
                blank_lines: false,
                ..breaks
            },
            ..input()
        },
    );
    assert_eq!(texts(&out.doc).len(), 6);
    assert_eq!(out.found.blank_lines_removed, 1);
}

#[test]
fn a_signature_line_names_its_author_and_nothing_else_does() {
    assert_eq!(
        signed_name("Pradel Ianto, 8/24/16, 3:55 PM").as_deref(),
        Some("Pradel Ianto")
    );
    assert_eq!(signed_name("TW, 11/09/2025 16:52").as_deref(), Some("TW"));
    assert_eq!(signed_name("Jane, 16.04.2019").as_deref(), Some("Jane"));
    assert_eq!(signed_name("See chapter 3, 12/3/4 above."), None);
    assert_eq!(signed_name("Remember, the tide turns."), None);
}

/// Tokens never cross a paragraph's end, and a lone `<` or `{` is text.
#[test]
fn scanning_ignores_what_only_looks_like_a_marker() {
    let text: Vec<char> = "a < b {x} <$ c\n<$Scr_Ps::0>".chars().collect();
    let tokens = scan(&text);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].kind, TokenKind::StyleOpen(Family::Paragraph, 0));
}

/// The Mac writer closes an annotation filling a paragraph after the paragraph's mark: the
/// comment still goes to the paragraph after it, not to the one after that.
#[test]
fn a_whole_paragraph_annotation_closed_after_its_mark_goes_to_the_next_paragraph() {
    let out = run(
        r"{\rtf1 First scene.\par \{\\Scrv_annot \\text=Move this later.\par \\end_Scrv_annot\}Second scene.\par Third scene.\par}",
    );
    assert_eq!(
        texts(&out.doc),
        ["First scene.", "{c0}Second scene.", "Third scene."]
    );
    assert_eq!(comment(&out.doc, 0), ["Move this later."]);
}

/// A poem's blank lines between stanzas are removed like any other, and said so apart;
/// short lines each a paragraph of their own, as a character sheet has them, are no poem.
#[test]
fn a_poems_stanza_gaps_are_counted() {
    let out = run(
        r"{\rtf1 The tide came in,\line the lamp was lit,\par\par the keeper slept,\line the gulls did not.\par\par Morning.\par}",
    );
    assert_eq!(
        out.found.stanza_gaps, 1,
        "a one-line paragraph is not a stanza"
    );
    assert_eq!(texts(&out.doc).len(), 3);

    // The same poem in one paragraph, its stanzas apart by an empty line.
    let one = run(
        r"{\rtf1 The tide came in,\line the lamp was lit,\line\line the keeper slept,\line the gulls did not.\par}",
    );
    assert_eq!(one.found.stanza_gaps, 1);
    // Prose typed with soft breaks between its paragraphs is no poem.
    let long = run(
        r"{\rtf1 The keeper climbed the stairs again and counted every one of them aloud.\line\line He reached the lamp before the light had gone from the western sky.\par}",
    );
    assert_eq!(long.found.stanza_gaps, 0);

    let sheet = run(
        r"{\rtf1 Name: Ada Keeper\par\par Age: forty\par\par Lives: the lighthouse\par\par Fears: the fog\par}",
    );
    assert_eq!(sheet.found.stanza_gaps, 0);
}

/// A blank line between stanzas never becomes a scene break, whatever the prose around.
#[test]
fn a_blank_line_between_stanzas_is_not_a_scene_break() {
    let prose = "The keeper climbed the stairs again and counted every one of them aloud.";
    let mut rtf = String::from(r"{\rtf1 ");
    for _ in 0..5 {
        rtf.push_str(prose);
        rtf.push_str(r"\par ");
    }
    // Each stanza long enough, its lines run together, to pass for prose.
    rtf.push_str(r"The tide came in across the stones,\line the lamp was lit above the bay,\par\par the keeper slept beside the glass,\line the gulls did not, and nor did he.\par ");
    for _ in 0..5 {
        rtf.push_str(prose);
        rtf.push_str(r"\par ");
    }
    rtf.push('}');
    let out = rewrite(
        parse(&rtf),
        &Input {
            scene_breaks: SceneBreaks {
                manuscript: true,
                blank_lines: true,
                ornaments: false,
            },
            ..input()
        },
    );
    assert_eq!(out.found.blank_lines_breaks, 0);
    assert_eq!(out.found.stanza_gaps, 1);
}

/// Should Scrivener write a comment over two paragraphs as one field in each, it is still
/// one comment, over both.
#[test]
fn fields_naming_one_comment_are_one_comment() {
    let sidecar = sidecar();
    let doc = parse(
        r#"{\rtf1 {\field{\*\fldinst{HYPERLINK "scrivcmt://AAAAAAAA-0000-0000-0000-000000000001"}}{\fldrslt First part}} ends.\par Then {\field{\*\fldinst{HYPERLINK "scrivcmt://AAAAAAAA-0000-0000-0000-000000000001"}}{\fldrslt second part}}.\par}"#,
    );
    let out = rewrite(
        doc,
        &Input {
            sidecar: Some(&sidecar),
            ..input()
        },
    );
    // Comment 0 is the fields'; comment 1 and footnote 0 are the entries the text never
    // anchored.
    assert_eq!(
        texts(&out.doc),
        ["{c1}{[}{c0}First part ends.", "Then second part{]}.{fn0}"]
    );
    assert_eq!(
        comment(&out.doc, 0),
        ["Too soon for gulls.", "Pradel Ianto, 8/24/16, 3:55 PM"]
    );
}

/// A crafted `.links` file stating a range past any text, or one that overflows its own
/// arithmetic, is never a crash: the entry is placed with the others the text never
/// anchored, and the text is left whole.
#[test]
fn a_range_past_any_text_is_placed_with_the_unanchored() {
    for range in [
        (u64::MAX - 1, 5),
        (u64::MAX, u64::MAX),
        (3, u64::MAX),
        (30, 4),
    ] {
        let sidecar = Sidecar {
            entries: vec![Entry {
                id: None,
                footnote: false,
                author: None,
                coloured: false,
                body: r"{\rtf1 A remark.}".to_string(),
                range: Some(range),
            }],
            links: vec![crate::prose::sidecar::RangeLink {
                range,
                target: "3".into(),
            }],
        };
        // An astral character first: units 0 and 1 are one character.
        let doc = parse(r"{\rtf1\uc1 \u-10179?\u-8704? brave world.\par}");
        let out = rewrite(
            doc,
            &Input {
                sidecar: Some(&sidecar),
                ..input()
            },
        );
        assert_eq!(texts(&out.doc), ["{c0}\u{1F600} brave world."], "{range:?}");
        assert_eq!(out.found.comments_without_anchor, 1, "{range:?}");
    }
}

/// Thousands of comment fields in one text are read in time proportional to their number:
/// each field's comment was looked for through every other.
#[test]
fn many_comment_fields_are_read_in_linear_time() {
    let count = 50_000;
    let mut entries = Vec::with_capacity(count);
    let mut rtf = String::from(r"{\rtf1 ");
    for i in 0..count {
        let id = format!("AAAAAAAA-0000-0000-0000-{i:012}");
        rtf.push_str(&format!(
            r#"Word {{\field{{\*\fldinst{{HYPERLINK "scrivcmt://{id}"}}}}{{\fldrslt marked}}}} here. "#
        ));
        if i % 10 == 9 {
            rtf.push_str(r"\par ");
        }
        entries.push(Entry {
            id: Some(id),
            footnote: i % 2 == 0,
            author: None,
            coloured: false,
            body: r"{\rtf1 A remark.}".to_string(),
            range: None,
        });
    }
    rtf.push('}');
    let sidecar = Sidecar {
        entries,
        links: Vec::new(),
    };
    let doc = parse(&rtf);
    let started = std::time::Instant::now();
    let out = rewrite(
        doc,
        &Input {
            sidecar: Some(&sidecar),
            ..input()
        },
    );
    assert_eq!(out.doc.annotations.len() + out.doc.notes.len(), count);
    assert_eq!(out.found.comments_without_anchor, 0);
    assert!(
        started.elapsed() < std::time::Duration::from_secs(15),
        "{:?}",
        started.elapsed()
    );
}

/// A format 1.0 range the writer selected from the blank line above a paragraph (a
/// triple-click often takes the line break too): the comment is on the paragraph's words,
/// and the blank line still becomes the scene break it was, the comment kept.
#[test]
fn a_range_starting_on_a_blank_line_lands_on_the_words_after_it() {
    let len = |i: usize| prose_line(i).encode_utf16().count() as u64;
    let start: u64 = (0..3).map(|i| len(i) + 1).sum();
    let sidecar = Sidecar {
        entries: vec![Entry {
            id: None,
            footnote: false,
            author: None,
            coloured: false,
            body: r"{\rtf1 A remark on the scene.}".to_string(),
            // The blank line, then the fourth paragraph and its break.
            range: Some((start, 1 + len(3) + 1)),
        }],
        links: Vec::new(),
    };
    let mut rtf = String::from(r"{\rtf1 ");
    for i in 0..3 {
        rtf.push_str(&prose_line(i));
        rtf.push_str(r"\par ");
    }
    rtf.push_str(r"\par ");
    for i in 3..6 {
        rtf.push_str(&prose_line(i));
        rtf.push_str(r"\par ");
    }
    rtf.push('}');
    let out = rewrite(
        parse(&rtf),
        &Input {
            sidecar: Some(&sidecar),
            scene_breaks: SceneBreaks {
                manuscript: true,
                blank_lines: true,
                ornaments: false,
            },
            ..input()
        },
    );
    let lines = texts(&out.doc);
    assert_eq!(lines[3], "* * *");
    assert_eq!(lines[4], format!("{{[}}{{c0}}{}{{]}}", prose_line(3)));
    assert_eq!(out.found.ranges_approximate, 0);
}

/// A linked footnote whose format 1.0 range takes its paragraph's break too is cited after
/// the paragraph's last word, not at the start of the next paragraph.
#[test]
fn a_footnote_range_taking_its_line_break_is_cited_in_its_paragraph() {
    let sidecar = Sidecar {
        entries: vec![Entry {
            id: None,
            footnote: true,
            author: None,
            coloured: false,
            body: r"{\rtf1 A source.}".to_string(),
            // "First line." and its break.
            range: Some((0, 12)),
        }],
        links: Vec::new(),
    };
    let out = rewrite(
        parse(r"{\rtf1 First line.\par Second line.\par}"),
        &Input {
            sidecar: Some(&sidecar),
            ..input()
        },
    );
    assert_eq!(texts(&out.doc), ["First line.{fn0}", "Second line."]);
}

/// A blank line that becomes a scene break keeps its spaces only: a page break it held
/// goes to the paragraph after it rather than with them.
#[test]
fn a_blank_line_made_a_break_hands_on_what_it_held() {
    let mut rtf = String::from(r"{\rtf1 ");
    for i in 0..3 {
        rtf.push_str(&prose_line(i));
        rtf.push_str(r"\par ");
    }
    rtf.push_str(r"\page\par ");
    for i in 3..6 {
        rtf.push_str(&prose_line(i));
        rtf.push_str(r"\par ");
    }
    rtf.push('}');
    let out = rewrite(
        parse(&rtf),
        &Input {
            scene_breaks: SceneBreaks {
                manuscript: true,
                blank_lines: true,
                ornaments: false,
            },
            ..input()
        },
    );
    let paragraphs: Vec<&Paragraph> = rtf_reader::paragraphs(&out.doc.body).collect();
    assert_eq!(render(paragraphs[3]), "* * *");
    assert_eq!(
        paragraphs[4].inlines.first(),
        Some(&Inline::Break(BreakKind::Page)),
        "{:?}",
        paragraphs[4].inlines
    );
}

/// Two annotations side by side at a paragraph's start both mark the word after them: the
/// first never takes the second's markers for words.
#[test]
fn annotations_side_by_side_both_mark_the_word_after_them() {
    let out = run(
        r"{\rtf1 \{\\Scrv_annot \\text=One.\\end_Scrv_annot\} \{\\Scrv_annot \\text=Two.\\end_Scrv_annot\} Harbour lights.\par}",
    );
    assert_eq!(texts(&out.doc), ["  {[}{c0}{[}{c1}Harbour{]}{]} lights."]);
}

/// Thousands of annotations in one paragraph are placed in time proportional to the text:
/// each once scanned past all the others, and ten thousand took minutes.
#[test]
fn many_annotations_in_one_paragraph_are_placed_in_linear_time() {
    let mut rtf = String::from(r"{\rtf1 A word first.\par ");
    for i in 0..10_000 {
        rtf.push_str(&format!(
            r"\{{\\Scrv_annot \\text=Remark {i}.\\end_Scrv_annot\}} "
        ));
    }
    rtf.push_str(r"\par}");
    let doc = parse(&rtf);
    let started = std::time::Instant::now();
    let out = rewrite(doc, &input());
    assert_eq!(out.doc.annotations.len(), 10_000);
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "{:?}",
        started.elapsed()
    );
}
