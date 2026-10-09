// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The converter end to end: a Scrivener project of each generation written as a `.skrib`,
//! read back by the format's own reader, turned into the object graph `load_work` builds,
//! loaded into a real store and saved back out, and every text put through the editor's
//! own save.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use chrono::{DateTime, Utc};
use common::entities::{BinderItemSubRole as SubRole, CommentAnchorKind, ContentRole};
use scrivener_fixtures::{Generation, Item, Kind, Platform, Spec, TINY_PNG};
use scrivener_import::structure::Choices;
use scrivener_import::{
    Answers, ConvertError, ProseChoices, ProseNotice, Request, Summary, TITLE_SLOT, Words, convert,
};
use skrib_format::{BundledItem, WorkBundle};

use crate::support::{options, temp, write_files};

pub(crate) fn words() -> Words {
    Words {
        manuscript_binder: "W-Manuscript".to_string(),
        notes_binder: "W-Notes".to_string(),
        notes_of: format!("{TITLE_SLOT} W-notes"),
        project_notes: "W-ProjectNotes".to_string(),
        recovered: "W-Recovered".to_string(),
        untitled_template: "W-Untitled".to_string(),
        research_folder: format!("{TITLE_SLOT} W-research"),
        list_separator: " W+ ".to_string(),
        label_details: format!("W-field {TITLE_SLOT}"),
        status_details: format!("W-status-field {TITLE_SLOT}"),
        unnamed_label: format!("W-label {TITLE_SLOT}"),
        unnamed_status: format!("W-status {TITLE_SLOT}"),
        label_tag: format!("{TITLE_SLOT} W-(label)"),
        untitled_tag: "W-untitled-tag".to_string(),
        default_statuses: vec![
            scrivener_import::DefaultRung {
                name: "W-planned".to_string(),
                category: common::entities::StatusCategory::Planned,
            },
            scrivener_import::DefaultRung {
                name: "W-final".to_string(),
                category: common::entities::StatusCategory::Final,
            },
        ],
    }
}

pub(crate) fn instant() -> DateTime<Utc> {
    DateTime::from_timestamp(1_790_000_000, 0).unwrap_or_default()
}

pub(crate) fn request<'a>(
    source: &'a Path,
    output: &'a str,
    choices: &'a Choices,
    words: &'a Words,
    answers: &'a Answers,
    options: &'a scrivener_import::Options,
) -> Request<'a> {
    Request {
        source,
        answers,
        options,
        choices,
        prose: ProseChoices::default(),
        fingerprint: None,
        output,
        overwrite: false,
        words,
        title: None,
        author: None,
        language: Some("en-US"),
        copy_research_files: true,
        now: instant(),
    }
}

/// Convert the project at `source` into `out`, or fail naming what happened.
pub(crate) fn converted(source: &Path, out: &Path) -> Summary {
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let output = out.to_string_lossy().into_owned();
    let request = request(source, &output, &choices, &words, &answers, &options);
    match convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        Ok(summary) => summary,
        Err(e) => panic!("the conversion failed: {e}"),
    }
}

pub(crate) fn read(out: &Path) -> WorkBundle {
    match skrib_format::read_bundle(&out.to_string_lossy()) {
        Ok(bundle) => bundle,
        Err(e) => panic!("the written project does not read back: {e:#}"),
    }
}

/// The written bundle becomes the object graph `load_work` builds, every id resolving.
pub(crate) fn loads(bundle: WorkBundle, out: &Path) {
    if let Err(e) = skrib_format::bundle_to_loaded(bundle, &out.to_string_lossy()) {
        panic!("the bundle does not become a work: {e:#}");
    }
}

pub(crate) fn items(bundle: &WorkBundle) -> impl Iterator<Item = &BundledItem> {
    bundle.binders.iter().flat_map(|b| b.items.iter())
}

pub(crate) fn item<'b>(bundle: &'b WorkBundle, title: &str) -> &'b BundledItem {
    match items(bundle).find(|i| i.item.title == title) {
        Some(item) => item,
        None => panic!("no row titled {title}"),
    }
}

/// The Djot of a row's content of `role`.
pub(crate) fn prose(item: &BundledItem, role: ContentRole) -> String {
    item.item
        .prose_refs
        .iter()
        .find(|p| p.role == role)
        .and_then(|p| item.prose.get(&p.file_id))
        .cloned()
        .unwrap_or_default()
}

/// What the editor reads a Djot text as.
pub(crate) fn addressable(djot: &str) -> String {
    let doc = text_document::TextDocument::new();
    if let Err(e) = doc.set_djot_sync(djot) {
        panic!("the editor does not read {djot:?}: {e}");
    }
    doc.to_addressable_text().unwrap_or_default()
}

/// Every text a bundle stores keeps its words through the editor's own save: read, written
/// back with `to_djot`, read again.
pub(crate) fn survives_the_editor(bundle: &WorkBundle) {
    for item in items(bundle) {
        for (id, djot) in &item.prose {
            let doc = text_document::TextDocument::new();
            if let Err(e) = doc.set_djot_sync(djot) {
                panic!(
                    "'{}' {id}: the editor does not read its text: {e}",
                    item.item.title
                );
            }
            let before = doc.to_addressable_text().unwrap_or_default();
            let saved = match doc.to_djot() {
                Ok(saved) => saved,
                Err(e) => panic!("'{}': the editor does not write it: {e}", item.item.title),
            };
            assert_eq!(
                addressable(&saved),
                before,
                "'{}': the editor's save changed the text of {djot:?} into {saved:?}",
                item.item.title
            );
        }
    }
}

/// The novel fixture with some of its texts replaced.
fn novel_with(
    generation: Generation,
    platform: Platform,
    parent: &Path,
    replace: &[(&str, &str, String)],
) -> PathBuf {
    let spec = Spec::novel(generation, platform);
    let mut files = spec.build();
    for (title, file, bytes) in replace {
        let Some(index) = spec.index_of(title) else {
            panic!("no fixture item {title}");
        };
        let path = if file.is_empty() {
            spec.text_path(index)
        } else {
            format!("{}{file}", spec.payload_prefix(index))
        };
        files.put(path, bytes.clone().into_bytes());
    }
    write_files(&files, parent, &spec.folder())
}

// ── every generation ─────────────────────────────────────────────────────────────

#[test]
fn every_generation_converts_into_a_project_that_loads() {
    for (generation, platform) in [
        (Generation::Format10, Platform::Mac),
        (Generation::Format10, Platform::Windows),
        (Generation::Format15, Platform::Mac),
        (Generation::Format15, Platform::Windows),
        (Generation::Format20, Platform::Mac),
        (Generation::Format20, Platform::Windows),
        (Generation::IosStub, Platform::Ios),
    ] {
        let dir = temp();
        let spec = Spec::novel(generation, platform);
        let folder = write_files(&spec.build(), dir.path(), &spec.folder());
        let out = dir.path().join("novel.skrib");
        let summary = converted(&folder, &out);
        let bundle = read(&out);
        let label = format!("{generation:?} {platform:?}");

        // The words the import added are the caller's, never English.
        let names: Vec<&str> = bundle
            .binders
            .iter()
            .map(|b| b.binder.name.as_str())
            .collect();
        assert_eq!(names[0], "W-Manuscript", "{label}");
        let arrival = item(&bundle, "Arrival");
        assert_eq!(arrival.item.sub_role, SubRole::Scene, "{label}");
        let text = prose(arrival, ContentRole::SceneText);
        assert!(
            text.contains(scrivener_fixtures::text::sentence(0)),
            "{label}: the scene's words are there: {text:?}"
        );
        let synopsis = prose(arrival, ContentRole::SynopsisText);
        assert_eq!(
            synopsis.trim(),
            scrivener_fixtures::text::sentence(7),
            "{label}: the synopsis, as plain text"
        );
        if generation != Generation::IosStub {
            let notes = item(&bundle, "Arrival W-notes");
            assert!(
                !prose(notes, ContentRole::NoteText).is_empty(),
                "{label}: the document notes, beside the scene"
            );
            // The picture in Research is an asset shown on its note.
            assert_eq!(summary.totals.pictures, 1, "{label}");
            assert!(
                prose(item(&bundle, "Harbour map"), ContentRole::NoteText).contains("](assets/"),
                "{label}"
            );
            // The PDF is copied beside the project, and its note names where.
            assert_eq!(summary.totals.research_files, 1, "{label}");
            assert_eq!(
                summary.research_folder.as_deref(),
                Some("novel W-research"),
                "{label}"
            );
            let note = addressable(&prose(item(&bundle, "Tide tables"), ContentRole::NoteText));
            assert_eq!(note, "novel W-research/Tide tables.pdf", "{label}");
            assert_eq!(
                std::fs::read(dir.path().join("novel W-research/Tide tables.pdf"))
                    .unwrap_or_default(),
                scrivener_fixtures::TINY_PDF,
                "{label}"
            );
            assert_eq!(bundle.note_templates.len(), 1, "{label}: the template");
            assert_eq!(bundle.trash_infos.len(), 1, "{label}: one trash entry");
        }
        // The rows carry the dates of the items they came from.
        assert!(
            arrival.item.created_at.starts_with("2024-03-14"),
            "{label}: {}",
            arrival.item.created_at
        );
        assert_eq!(
            summary.totals.rows as usize,
            items(&bundle).count(),
            "{label}"
        );
        survives_the_editor(&bundle);
        loads(bundle, &out);
    }
}

// ── the dialect, end to end ─────────────────────────────────────────────────────

fn png_hex() -> String {
    TINY_PNG.iter().map(|b| format!("{b:02x}")).collect()
}

/// Everything Scrivener 3 for Mac writes into a scene: an inline footnote, an inline
/// annotation, a linked comment and a linked footnote, a picture, and text the Djot parser
/// would rewrite if it were not escaped.
fn dialect_scene() -> String {
    [
        r"{\rtf1\ansi\ansicpg1252\cocoartf2761
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
\pard\f0\fs24 The ferry was late\{\\Scrv_fn=It always was.\\end_Scrv_fn\} again.\
The keeper \{\\Scrv_annot \\color=\{\\R=1\\G=0\\B=0\} \\text=Check the date.\\end_Scrv_annot\} counted the ships {\field{\*\fldinst{HYPERLINK ",
        "\"scrivcmt://C0FFEE00-0000-0000-0000-000000000001\"",
        r"}}{\fldrslt before the tide}} turned.\
Mara climbed the stairs {\field{\*\fldinst{HYPERLINK ",
        "\"scrivcmt://C0FFEE00-0000-0000-0000-000000000002\"",
        r"}}{\fldrslt with the oil}}.\
{\*\shppict{\pict{\*\nisusfilename lamp}\picw1\pich1\picwgoal300\pichgoal300\pngblip ",
        &png_hex(),
        r"}}\
At 10:30:45 she said 'no' -- and... I. Roman numerals stay.}",
    ]
    .concat()
}

fn dialect_comments() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
<Comments>
    <Comment ID="C0FFEE00-0000-0000-0000-000000000001" Color="0.99 0.95 0.75"><![CDATA[{\rtf1\ansi A remark on the tide.}]]></Comment>
    <Comment ID="C0FFEE00-0000-0000-0000-000000000002" Footnote="Yes" Color="0.85 0.85 0.85"><![CDATA[{\rtf1\ansi Lamp oil was scarce.}]]></Comment>
</Comments>
"#
    .to_string()
}

#[test]
fn a_scenes_notes_comments_and_picture_arrive_on_their_words() {
    let dir = temp();
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[
            ("Arrival", "", dialect_scene()),
            ("Arrival", "content.comments", dialect_comments()),
        ],
    );
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let bundle = read(&out);
    let arrival = item(&bundle, "Arrival");
    let djot = prose(arrival, ContentRole::SceneText);
    let text = addressable(&djot);
    assert!(
        text.contains("At 10:30:45 she said 'no' -- and... I. Roman numerals stay."),
        "the text reads back as written: {text:?}"
    );
    assert!(!text.contains("Scrv_"), "no marker survives: {text:?}");
    assert!(
        !text.contains("Check the date"),
        "the annotation left the prose"
    );

    let content_id = arrival
        .item
        .prose_refs
        .iter()
        .find(|p| p.role == ContentRole::SceneText)
        .map(|p| p.file_id)
        .unwrap_or_default();
    let notes = arrival
        .footnotes
        .get(&content_id)
        .cloned()
        .unwrap_or_default();
    let bodies: Vec<String> = notes.iter().map(|n| addressable(&n.body)).collect();
    assert_eq!(bodies, ["It always was.", "Lamp oil was scarce."]);
    for note in &notes {
        assert!(
            djot.contains(&format!("[^{}]", note.label)),
            "{} is cited",
            note.label
        );
    }
    // The linked footnote is cited after its words.
    let oil = text.find("with the oil").unwrap_or_default();
    assert_eq!(
        text[oil + "with the oil".len()..].chars().next(),
        Some('\u{FFFC}'),
        "{text:?}"
    );

    let comments = arrival
        .comments
        .get(&content_id)
        .cloned()
        .unwrap_or_default();
    let mut quotes: Vec<(String, String)> = comments
        .iter()
        .map(|c| (c.quote_exact.clone(), addressable(&c.body)))
        .collect();
    quotes.sort();
    // The fixture's own sidecar comment is gone: this sidecar replaced it.
    assert_eq!(
        quotes,
        [
            (
                "before the tide".to_string(),
                "A remark on the tide.".to_string()
            ),
            ("keeper".to_string(), "Check the date.".to_string()),
        ]
    );
    for comment in &comments {
        assert!(!comment.orphaned, "{comment:?}");
        assert_eq!(comment.kind, CommentAnchorKind::Range);
        assert!(comment.author_name.is_empty(), "never the importing writer");
        // The stored offsets point at the quoted words in the text the editor reads.
        let at: String = text
            .chars()
            .skip(comment.range_start as usize)
            .take(comment.range_length as usize)
            .collect();
        assert_eq!(at, comment.quote_exact);
    }

    // The picture: one asset, named by its place in the project, at its display size.
    assert!(djot.contains("{width=20 height=20}"), "{djot}");
    // The scene's picture is the same image as the Research picture: one asset, which both
    // name.
    assert_eq!(bundle.assets.len(), 1);
    let path = &bundle.assets[0].path;
    assert!(djot.contains(path.as_str()), "{djot}");
    assert!(prose(item(&bundle, "Harbour map"), ContentRole::NoteText).contains(path.as_str()));
    assert!(
        summary
            .prose
            .iter()
            .any(|n| matches!(n, ProseNotice::AnnotationsBecameComments { count: 1, .. }))
    );
    // The summary crosses a long operation's boundary as JSON (G5).
    let json = serde_json::to_string(&summary).unwrap_or_default();
    let back: Result<Summary, _> = serde_json::from_str(&json);
    assert!(
        back.is_ok_and(|b| b == summary),
        "the summary survives JSON"
    );
    survives_the_editor(&bundle);
    loads(bundle, &out);
}

/// A picture a text holds with no name of its own is named by its number, in no language:
/// the name is kept with the picture in the project, and English is not the writer's.
#[test]
fn a_picture_with_no_name_is_named_in_no_language() {
    let dir = temp();
    let scene = [
        r"{\rtf1 A lamp: {\pict\picw1\pich1\pngblip ",
        &png_hex(),
        r"}\par}",
    ]
    .concat();
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[("Arrival", "", scene)],
    );
    let out = dir.path().join("novel.skrib");
    converted(&folder, &out);
    let bundle = read(&out);
    let names: Vec<&str> = bundle.assets.iter().map(|a| a.file_name.as_str()).collect();
    assert!(names.contains(&"1.png"), "{names:?}");
}

/// A note template's body is Djot alone: a footnote written in the template document is
/// reported as not carried, and leaves no reference to a note that is not there.
#[test]
fn a_templates_footnote_leaves_no_reference_behind() {
    let dir = temp();
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[(
            "Scene sheet",
            "",
            r"{\rtf1 Goal\{\\Scrv_fn=What the scene wants.\\end_Scrv_fn\}: \par}".to_string(),
        )],
    );
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let bundle = read(&out);
    let bodies: Vec<&String> = bundle.note_template_bodies.values().collect();
    assert_eq!(bodies.len(), 1);
    assert!(!bodies[0].contains("[^"), "{bodies:?}");
    assert!(bodies[0].contains("Goal"), "{bodies:?}");
    assert!(
        summary
            .prose
            .iter()
            .any(|n| n.key() == "footnote-not-carried"),
        "{:?}",
        summary.prose
    );
}

/// Scrivener for Windows 1.9 (format 1.5): Qt RTF, a note written one pair per paragraph,
/// comments in `<ID>.comments`, the field without inner braces.
#[test]
fn a_windows_format_1_5_project_brings_its_notes_and_comments() {
    let scene = r#"{\rtf1\ansi\ansicpg1252\uc1\deff0
{\fonttbl{\f0\fmodern\fcharset0\fprq2 Cochineal;}}
\pard\plain \ltrch\loch {\f0\fs24\b0\i0 He sat at his desk\{\\Scrv_fn=}{\f0\fs20\b0\i0 ENDNOTES\\end_Scrv_fn\}}
\par\pard\plain {\f0\fs20 \{\\Scrv_fn=A soaked copy of the letter.\\end_Scrv_fn\}}
\par\pard\plain {\f0\fs24 The morning {\field{\*\fldinst HYPERLINK "scrivcmt://D0000000-0000-0000-0000-000000000001"}{\fldrslt\f0\fs24\b0\i0 came late}}.}
\par}"#;
    let comments = r#"<?xml version="1.0" encoding="UTF-8"?>
<Comments Version="1.0">
    <Comment ID="D0000000-0000-0000-0000-000000000001" Color="0.99 0.95 0.75" Collapsed="No"><![CDATA[{\rtf1\ansi\uc1 Too slow.}]]></Comment>
</Comments>
"#;
    let dir = temp();
    let folder = novel_with(
        Generation::Format15,
        Platform::Windows,
        dir.path(),
        &[
            ("Arrival", "", scene.to_string()),
            ("Arrival", ".comments", comments.to_string()),
        ],
    );
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let bundle = read(&out);
    let arrival = item(&bundle, "Arrival");
    let text = addressable(&prose(arrival, ContentRole::SceneText));
    assert_eq!(text, "He sat at his desk\u{FFFC}\nThe morning came late.");
    let notes: Vec<String> = arrival
        .footnotes
        .values()
        .flatten()
        .map(|n| addressable(&n.body))
        .collect();
    assert_eq!(notes, ["ENDNOTES\nA soaked copy of the letter."]);
    let comments: Vec<&str> = arrival
        .comments
        .values()
        .flatten()
        .map(|c| c.quote_exact.as_str())
        .collect();
    assert_eq!(comments, ["came late"]);
    assert!(
        summary.prose.iter().any(|n| matches!(
            n,
            ProseNotice::InlineNotesJoinedAcrossParagraphs { count: 1, .. }
        )),
        "{:?}",
        summary.prose
    );
    survives_the_editor(&bundle);
    loads(bundle, &out);
}

/// Format 1.0 (Scrivener for Mac 2.0 to 2.4): comments anchored by a range into the text.
#[test]
fn a_format_1_0_range_comment_lands_on_its_words() {
    let scene = r"{\rtf1\ansi\ansicpg1252\cocoartf1038
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
\pard\f0\fs24 Light with match and lantern.}";
    let links = r#"<?xml version="1.0" encoding="UTF-8"?>
<Links Version="1.0">
    <CommentsAndFootnotes>
        <Comment IsFootnote="No" Collapsed="No"><Range>6,10</Range><Color>0.992 0.929 0.525</Color><RTF><![CDATA[{\rtf1\ansi Which match?}]]></RTF></Comment>
        <Comment IsFootnote="Yes" Collapsed="No"><Range>21,7</Range><Color>0.85 0.85 0.85</Color><RTF><![CDATA[{\rtf1\ansi An oil lantern.}]]></RTF></Comment>
    </CommentsAndFootnotes>
</Links>
"#;
    let dir = temp();
    let folder = novel_with(
        Generation::Format10,
        Platform::Mac,
        dir.path(),
        &[
            ("Arrival", "", scene.to_string()),
            ("Arrival", ".links", links.to_string()),
        ],
    );
    let out = dir.path().join("novel.skrib");
    converted(&folder, &out);
    let bundle = read(&out);
    let arrival = item(&bundle, "Arrival");
    let text = addressable(&prose(arrival, ContentRole::SceneText));
    assert_eq!(text, "Light with match and lantern\u{FFFC}.");
    let comments: Vec<&str> = arrival
        .comments
        .values()
        .flatten()
        .map(|c| c.quote_exact.as_str())
        .collect();
    assert_eq!(comments, ["with match"]);
    let notes: Vec<String> = arrival
        .footnotes
        .values()
        .flatten()
        .map(|n| addressable(&n.body))
        .collect();
    assert_eq!(notes, ["An oil lantern."]);
    loads(bundle, &out);
}

/// A comment and a footnote inside a document's notes, and a comment over two paragraphs.
#[test]
fn notes_hold_their_own_footnotes_and_a_comment_may_span_paragraphs() {
    let notes = r"{\rtf1\ansi\ansicpg1252\cocoartf2761
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
\pard\f0\fs24 Research on tides\{\\Scrv_fn=Tables of 1901.\\end_Scrv_fn\}.}";
    let scene = [
        r"{\rtf1\ansi\ansicpg1252\cocoartf2761
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
\pard\f0\fs24 One {\field{\*\fldinst{HYPERLINK ",
        "\"scrivcmt://E0000000-0000-0000-0000-000000000001\"",
        r"}}{\fldrslt first line.\
Second} line} ends.}",
    ]
    .concat();
    let comments = r#"<?xml version="1.0" encoding="UTF-8"?>
<Comments>
    <Comment ID="E0000000-0000-0000-0000-000000000001" Color="0.99 0.95 0.75"><![CDATA[{\rtf1\ansi Across the break.}]]></Comment>
</Comments>
"#;
    let dir = temp();
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[
            ("Arrival", "notes.rtf", notes.to_string()),
            ("Arrival", "", scene),
            ("Arrival", "content.comments", comments.to_string()),
        ],
    );
    let out = dir.path().join("novel.skrib");
    converted(&folder, &out);
    let bundle = read(&out);
    let note_row = item(&bundle, "Arrival W-notes");
    let note_notes: Vec<String> = note_row
        .footnotes
        .values()
        .flatten()
        .map(|n| addressable(&n.body))
        .collect();
    assert_eq!(note_notes, ["Tables of 1901."]);
    let arrival = item(&bundle, "Arrival");
    let comment = arrival.comments.values().flatten().next().cloned();
    let Some(comment) = comment else {
        panic!("the comment arrived");
    };
    assert_eq!(comment.quote_exact, "first line.\nSecond");
    assert!(!comment.orphaned);
    loads(bundle, &out);
}

/// A one-document manuscript holding its chapters as headings is split at them (A2): each
/// chapter a row titled with its heading, its text what follows it.
#[test]
fn a_one_document_manuscript_is_split_at_its_chapter_headings() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.items = vec![
        Item::new(0, Kind::Draft, "Manuscript").compiled(),
        Item {
            text: vec![
                "Foreword words.".to_string(),
                "Chapter One".to_string(),
                "It began at the harbour.".to_string(),
                "Chapter Two".to_string(),
                "It went on at sea.".to_string(),
                "Chapter Three".to_string(),
                "It ended at the lighthouse.".to_string(),
            ],
            ..Item::new(1, Kind::Text, "Whole book").compiled()
        },
        Item::new(0, Kind::Research, "Research"),
        Item::new(0, Kind::Trash, "Trash"),
    ];
    spec.template_folder = None;
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("book.skrib");
    converted(&folder, &out);
    let bundle = read(&out);
    let texts: BTreeMap<String, String> = items(&bundle)
        .filter(|i| i.item.sub_role == SubRole::ChapterScene)
        .map(|i| {
            (
                i.item.title.clone(),
                addressable(&prose(i, ContentRole::SceneText)),
            )
        })
        .collect();
    assert_eq!(
        texts.get("Chapter Two").map(String::as_str),
        Some("It went on at sea.")
    );
    assert_eq!(
        texts.get("Chapter Three").map(String::as_str),
        Some("It ended at the lighthouse.")
    );
    assert!(
        items(&bundle).all(|i| !prose(i, ContentRole::SceneText).contains("Chapter Two")),
        "a heading is a title, not text"
    );
    loads(bundle, &out);
}

/// A text split at its chapter headings whose opening holds no words of its own (a blank
/// line before the first heading) gets no row for that opening; what the import put there
/// all the same, a comment the text no longer anchors, goes with the first chapter, and so
/// does what the whole text reports.
#[test]
fn what_a_split_text_holds_before_its_first_heading_is_never_left_behind() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    spec.items = vec![
        Item::new(0, Kind::Draft, "Manuscript").compiled(),
        Item {
            text: vec!["Placeholder.".to_string()],
            ..Item::new(1, Kind::Text, "Whole book").compiled()
        },
        Item::new(0, Kind::Research, "Research"),
        Item::new(0, Kind::Trash, "Trash"),
    ];
    spec.template_folder = None;
    let mut files = spec.build();
    let Some(index) = spec.index_of("Whole book") else {
        panic!("no book");
    };
    files.put(
        spec.text_path(index),
        br"{\rtf1 \par Chapter One\par It began at the harbour.\par Chapter Two\par It went on at sea for <$wc> words.\par Chapter Three\par It ended at the lighthouse.\par}"
            .to_vec(),
    );
    files.put(
        format!("{}content.comments", spec.payload_prefix(index)),
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Comments><Comment ID="C0FFEE00-0000-0000-0000-000000000009"><![CDATA[{\rtf1 Check the whole book.}]]></Comment></Comments>"#
            .as_bytes()
            .to_vec(),
    );
    let folder = write_files(&files, dir.path(), &spec.folder());
    let out = dir.path().join("book.skrib");
    let summary = converted(&folder, &out);
    let bundle = read(&out);
    let comments: Vec<(String, String)> = items(&bundle)
        .flat_map(|i| {
            i.comments
                .values()
                .flatten()
                .map(|c| (i.item.title.clone(), addressable(&c.body)))
        })
        .collect();
    assert_eq!(
        comments,
        [(
            "Chapter One".to_string(),
            "Check the whole book.".to_string()
        )],
        "{:?}",
        summary.prose
    );
    // What the whole text reports is reported, whichever part it came with.
    assert!(
        summary.prose.iter().any(|n| matches!(
            n,
            ProseNotice::PlaceholdersInExportedText { kind, count: 1, .. } if kind == "<$wc>"
        )),
        "{:?}",
        summary.prose
    );
    loads(bundle, &out);
}

// ── the backend ──────────────────────────────────────────────────────────────────

/// The written project loads into a real store through `load_work`, and saves back out
/// with its texts, footnotes, comments and pictures whole.
#[test]
fn the_project_loads_into_the_store_and_saves_back_whole() {
    use common::database::db_context::DbContext;
    use common::event::EventHub;
    use common::long_operation::{LongOperationManager, OperationStatus};
    use work_management::{LoadWorkDto, SaveWorkDto, work_management_controller as controller};

    let dir = temp();
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[
            ("Arrival", "", dialect_scene()),
            ("Arrival", "content.comments", dialect_comments()),
        ],
    );
    let out = dir.path().join("novel.skrib");
    converted(&folder, &out);
    let imported = read(&out);

    let db = match DbContext::new() {
        Ok(db) => db,
        Err(e) => panic!("a store: {e}"),
    };
    let hub = Arc::new(EventHub::new());
    let media = dir.path().join("media");
    if let Err(e) = controller::load_work(
        &db,
        &hub,
        &LoadWorkDto {
            file_name: out.to_string_lossy().into_owned(),
            media_root: media.to_string_lossy().into_owned(),
        },
    ) {
        panic!("load_work refuses the project: {e:#}");
    }
    let work_id = db
        .get_store()
        .works
        .read()
        .ok()
        .and_then(|works| works.keys().next().copied())
        .unwrap_or_default();
    let resaved = dir.path().join("resaved.skrib");
    let mut long_ops = LongOperationManager::new();
    let op = match controller::save_work(
        &db,
        &hub,
        &mut long_ops,
        &SaveWorkDto {
            work_id,
            file_name: resaved.to_string_lossy().into_owned(),
            overwrite: true,
            media_root: media.to_string_lossy().into_owned(),
        },
    ) {
        Ok(op) => op,
        Err(e) => panic!("save_work does not start: {e:#}"),
    };
    assert!(long_ops.wait_for_operation(&op, Some(Duration::from_secs(120))));
    assert_eq!(
        long_ops.get_operation_status(&op),
        Some(OperationStatus::Completed)
    );
    let saved = read(&resaved);

    // Every row, its texts, its notes and its comments, by title.
    let shape = |bundle: &WorkBundle| -> BTreeMap<String, (Vec<String>, usize, usize)> {
        items(bundle)
            .map(|i| {
                let mut texts: Vec<String> = i.prose.values().map(|d| addressable(d)).collect();
                texts.sort();
                (
                    format!("{} {:?}", i.item.title, i.item.sub_role),
                    (
                        texts,
                        i.footnotes.values().map(Vec::len).sum(),
                        i.comments.values().map(Vec::len).sum(),
                    ),
                )
            })
            .collect()
    };
    assert_eq!(shape(&saved), shape(&imported));
    assert_eq!(saved.assets.len(), imported.assets.len());
    assert_eq!(saved.tags.len(), imported.tags.len());
    assert_eq!(saved.note_templates.len(), imported.note_templates.len());
}

// ── what is never left behind ────────────────────────────────────────────────────

#[test]
fn a_cancelled_conversion_leaves_nothing() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    let output = out.to_string_lossy().into_owned();
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let request = request(&folder, &output, &choices, &words, &answers, &options);
    let cancelled = AtomicBool::new(true);
    assert_eq!(
        convert(&request, &|_, _| {}, &cancelled).err(),
        Some(ConvertError::Cancelled)
    );
    assert!(!out.exists());
    assert!(!dir.path().join("novel.skrib.importing").exists());
    let beside: Vec<String> = std::fs::read_dir(dir.path())
        .map(|d| {
            d.filter_map(Result::ok)
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(beside, [spec.folder()], "no research folder, staged or not");
}

/// The folder beside the project taken while the import ran (another program made it):
/// the research files cannot be put there, so the import fails whole and says where,
/// rather than leaving a project whose notes name files that are not in that folder.
#[test]
fn a_research_folder_that_cannot_be_put_in_place_fails_the_import_whole() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    let output = out.to_string_lossy().into_owned();
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let request = request(&folder, &output, &choices, &words, &answers, &options);
    let taken = dir.path().join("novel W-research");
    let report = |_: f32, phase: &str| {
        if phase == "import.writing" {
            let _ = std::fs::create_dir_all(&taken);
            let _ = std::fs::write(taken.join("mine.txt"), b"the writer's own");
        }
    };
    let result = convert(&request, &report, &AtomicBool::new(false));
    assert!(
        matches!(result, Err(ConvertError::Output(_))),
        "{:?}",
        result.map(|s| s.prose.len())
    );
    assert!(!out.exists(), "no project whose notes name missing files");
    let mut beside: Vec<String> = std::fs::read_dir(dir.path())
        .map(|d| {
            d.filter_map(Result::ok)
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    beside.sort();
    assert_eq!(beside, [spec.folder(), "novel W-research".to_string()]);
    let held: Vec<String> = std::fs::read_dir(&taken)
        .map(|d| {
            d.filter_map(Result::ok)
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    assert_eq!(held, ["mine.txt"], "the other folder is left as it was");
}

/// A research picture in a format Skribisto cannot show is still a research file it cannot
/// open: copied beside the project with the others, its note naming where.
#[test]
fn a_research_picture_the_editor_cannot_show_is_copied_beside_the_project() {
    let dir = temp();
    let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="4" height="4"/>"#.to_string();
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[("Harbour map", "content.png", svg.clone())],
    );
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let copied: Vec<&String> = summary
        .prose
        .iter()
        .filter_map(|n| match n {
            ProseNotice::MediaFileCopied { path, .. } => Some(path),
            _ => None,
        })
        .collect();
    assert!(
        copied.iter().any(|p| p.ends_with("/Harbour map.png")),
        "{:?}",
        summary.prose
    );
    let research = summary.research_folder.clone().unwrap_or_default();
    assert_eq!(
        std::fs::read(dir.path().join(&research).join("Harbour map.png")).unwrap_or_default(),
        svg.as_bytes()
    );
    let bundle = read(&out);
    let note = addressable(&prose(item(&bundle, "Harbour map"), ContentRole::NoteText));
    assert_eq!(note, format!("{research}/Harbour map.png"));
}

/// Off, the research file stays in the Scrivener project and its note names where.
#[test]
fn research_files_left_in_the_scrivener_project_are_named_there() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    let output = out.to_string_lossy().into_owned();
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let mut request = request(&folder, &output, &choices, &words, &answers, &options);
    request.copy_research_files = false;
    let summary = match convert(&request, &|_, _| {}, &AtomicBool::new(false)) {
        Ok(summary) => summary,
        Err(e) => panic!("{e}"),
    };
    assert_eq!(summary.research_folder, None);
    assert!(
        summary
            .prose
            .iter()
            .any(|n| matches!(n, ProseNotice::MediaFileNotCarried { .. }))
    );
    let bundle = read(&out);
    let note = addressable(&prose(item(&bundle, "Tide tables"), ContentRole::NoteText));
    assert!(note.starts_with("Lighthouse.scriv/Files/Data/"), "{note}");
}

#[test]
fn a_project_changed_since_the_review_is_refused() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    let output = out.to_string_lossy().into_owned();
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let mut request = request(&folder, &output, &choices, &words, &answers, &options);
    request.fingerprint = Some("not the fingerprint reviewed");
    assert_eq!(
        convert(&request, &|_, _| {}, &AtomicBool::new(false)).err(),
        Some(ConvertError::ChangedSinceReview)
    );
    assert!(!out.exists());
}

#[test]
fn an_existing_target_is_refused_before_anything_is_read() {
    let dir = temp();
    let out = dir.path().join("novel.skrib");
    if let Err(e) = std::fs::write(&out, b"PK") {
        panic!("{e}");
    }
    let output = out.to_string_lossy().into_owned();
    let missing = dir.path().join("nothing-here.scriv");
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let request = request(&missing, &output, &choices, &words, &answers, &options);
    assert!(matches!(
        convert(&request, &|_, _| {}, &AtomicBool::new(false)),
        Err(ConvertError::Output(_))
    ));
    assert_eq!(std::fs::read(&out).unwrap_or_default(), b"PK");
}

// ── what the plan adds around the binder ────────────────────────────────────────

/// A sync service's copy of a scene is recovered into a notes folder the import names in the
/// writer's language; a sheet's index-card picture opens its note; format 1.x project notes
/// have a folder of their own. No word the import adds is English.
#[test]
fn recovered_texts_card_images_and_project_notes_have_their_rows() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let mut files = spec.build();
    let arrival = spec.index_of("Arrival").unwrap_or_default();
    let mara = spec.index_of("Mara").unwrap_or_default();
    let copy = scrivener_fixtures::traps::copy_name(
        "content.rtf",
        scrivener_fixtures::traps::CopyScheme::Dropbox,
    );
    files.put(
        format!("{}{copy}", spec.payload_prefix(arrival)),
        scrivener_fixtures::rtf::document(
            scrivener_fixtures::rtf::Dialect::Cocoa,
            &["A version the sync service kept aside.".to_string()],
        )
        .into_bytes(),
    );
    files.put(
        format!("{}card-image.png", spec.payload_prefix(mara)),
        TINY_PNG.to_vec(),
    );
    let folder = write_files(&files, dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    converted(&folder, &out);
    let bundle = read(&out);
    let recovered = item(&bundle, "W-Recovered");
    assert_eq!(recovered.item.sub_role, SubRole::Note);
    let kept_aside = items(&bundle)
        .map(|i| addressable(&prose(i, ContentRole::NoteText)))
        .any(|t| t == "A version the sync service kept aside.");
    assert!(kept_aside, "the recovered text has a row");
    let sheet = prose(item(&bundle, "Mara"), ContentRole::NoteText);
    assert!(
        sheet.starts_with("![](assets/"),
        "the card's picture opens the sheet: {sheet}"
    );
    loads(bundle, &out);

    // Format 1.5 keeps project notes in a folder of their own.
    let dir = temp();
    let spec = Spec::novel(Generation::Format15, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("novel.skrib");
    converted(&folder, &out);
    let bundle = read(&out);
    assert_eq!(item(&bundle, "W-ProjectNotes").item.sub_role, SubRole::Note);
    // The fixture's own "Notes" folder is the writer's title; what the import adds is not.
    for i in items(&bundle) {
        let title = &i.item.title;
        assert!(
            !title.ends_with("(notes)")
                && title != "Project notes"
                && title != "Recovered from Scrivener",
            "an English word the import added: {title}"
        );
    }
    let binders: Vec<&str> = bundle
        .binders
        .iter()
        .map(|b| b.binder.name.as_str())
        .collect();
    assert_eq!(binders, ["W-Manuscript", "W-Notes"]);
    loads(bundle, &out);
}

/// A link from one note to another becomes a reference the note's *Links* show; one from
/// a scene to a note that is not a story-bible entry has nowhere to show, so its words stay
/// and it is reported (X-35).
#[test]
fn document_links_become_references_where_skribisto_shows_them() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let ideas = spec.uuid(spec.index_of("Ideas").unwrap_or_default());
    let mara = spec.uuid(spec.index_of("Mara").unwrap_or_default());
    let link = |uuid: &str, words: &str| {
        [
            r"{\rtf1\ansi\ansicpg1252\cocoartf2761
{\fonttbl\f0\fswiss\fcharset0 Helvetica;}
\pard\f0\fs24 See {\field{\*\fldinst{HYPERLINK ",
            &format!("\"scrivlnk://{uuid}\""),
            r"}}{\fldrslt ",
            words,
            r"}} for more.}",
        ]
        .concat()
    };
    let folder = novel_with(
        Generation::Format20,
        Platform::Mac,
        dir.path(),
        &[
            ("Mara", "", link(&ideas, "the ideas")),
            ("Arrival", "", link(&mara, "Mara")),
        ],
    );
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let bundle = read(&out);
    let ideas_id = item(&bundle, "Ideas").item.file_id;
    assert_eq!(item(&bundle, "Mara").item.reference_ids, [ideas_id]);
    assert!(item(&bundle, "Arrival").item.reference_ids.is_empty());
    assert_eq!(
        addressable(&prose(item(&bundle, "Arrival"), ContentRole::SceneText)),
        "See Mara for more.",
        "the words stay, the link goes"
    );
    assert!(
        summary
            .prose
            .iter()
            .any(|n| matches!(n, ProseNotice::DocumentLinksDropped { count: 1, .. }))
    );
    assert!(
        summary
            .prose
            .iter()
            .any(|n| matches!(n, ProseNotice::DocumentLinksKept { count: 1, .. }))
    );
    loads(bundle, &out);
}

// ── hostile shapes ────────────────────────────────────────────────────────────────

/// The whole conversion of texts built to be deep (a hundred thousand nested groups,
/// twenty thousand nested notes, two hundred nested tables, a comment body and its XML
/// nested as far) finishes on a small thread stack: nothing on the way recurses on the
/// input. 384 KiB leaves room for the debug frames of a platform about five times Linux's.
#[test]
fn hostile_depth_converts_on_a_small_stack() {
    let deep_groups = format!(
        r"{{\rtf1 Before.{}deep{} After.\par}}",
        "{".repeat(100_000),
        "}".repeat(100_000)
    );
    let mut nested_notes = String::from(r"{\rtf1 Start ");
    nested_notes.push_str(&r"\{\\Scrv_fn=".repeat(20_000));
    nested_notes.push_str("inner");
    nested_notes.push_str(&r"\\end_Scrv_fn\}".repeat(20_000));
    nested_notes.push_str(r" end.\par}");
    let mut tables = String::from(r"{\rtf1 Before.\par ");
    for depth in 1..=200 {
        tables.push_str(&format!(r"\pard\intbl\itap{depth} cell {depth}\nestcell "));
    }
    for depth in (1..=200).rev() {
        tables.push_str(&format!(
            r"{{\*\nesttableprops\trowd\cellx1000\nestrow}}\itap{depth} "
        ));
    }
    tables.push_str(r"\pard After.\par}");
    let comments = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Comments>{}<Comment ID="C0FFEE00-0000-0000-0000-000000000001"><![CDATA[{{\rtf1 {}x{}}}]]></Comment>{}</Comments>"#,
        "<Wrap>".repeat(5_000),
        "{".repeat(50_000),
        "}".repeat(50_000),
        "</Wrap>".repeat(5_000)
    );
    let scene = r#"{\rtf1 He left {\field{\*\fldinst{HYPERLINK "scrivcmt://C0FFEE00-0000-0000-0000-000000000001"}}{\fldrslt at dawn}}.\par}"#.to_string();
    let worker = std::thread::Builder::new()
        .stack_size(384 << 10)
        .spawn(move || {
            let dir = temp();
            let folder = novel_with(
                Generation::Format20,
                Platform::Mac,
                dir.path(),
                &[
                    ("Arrival", "", deep_groups),
                    ("The Harbour", "", nested_notes),
                    ("Night Watch", "", tables),
                    ("Ideas", "", scene),
                    ("Ideas", "content.comments", comments),
                ],
            );
            let out = dir.path().join("novel.skrib");
            let summary = converted(&folder, &out);
            let bundle = read(&out);
            let keys: Vec<String> = summary.prose.iter().map(|n| n.key().to_string()).collect();
            let harbour = addressable(&prose(item(&bundle, "The Harbour"), ContentRole::SceneText));
            (keys, harbour)
        });
    let (keys, harbour) = match worker.map(|h| h.join()) {
        Ok(Ok(done)) => done,
        _ => panic!("the conversion did not finish on a 384 KiB stack"),
    };
    // Too deep to read is said, never guessed; the notes that do not pair stay as text.
    assert!(
        keys.iter().any(|k| k == "scrivener-prose-unreadable"),
        "{keys:?}"
    );
    assert!(
        keys.iter()
            .any(|k| k == "scrivener-prose-inline-note-unmatched"),
        "{keys:?}"
    );
    assert!(
        harbour.starts_with("Start"),
        "{}",
        &harbour[..harbour.len().min(40)]
    );
    assert!(
        keys.iter()
            .any(|k| k == "scrivener-prose-comments-unreadable"),
        "{keys:?}"
    );
}

/// A cancel at any point of the conversion leaves nothing beside the project it would
/// have made: no `.skrib`, no staged file, no research folder staged or placed. Past the
/// last point it can be heard, the import is whole.
#[test]
fn a_cancel_at_any_point_leaves_nothing() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let mut completed = false;
    for point in 0..60usize {
        let out = dir.path().join("novel.skrib");
        let output = out.to_string_lossy().into_owned();
        let request = request(&folder, &output, &choices, &words, &answers, &options);
        let cancelled = AtomicBool::new(false);
        let calls = std::cell::Cell::new(0usize);
        let report = |_: f32, _: &str| {
            if calls.get() == point {
                cancelled.store(true, std::sync::atomic::Ordering::Relaxed);
            }
            calls.set(calls.get() + 1);
        };
        let result = convert(&request, &report, &cancelled);
        let beside = || -> Vec<String> {
            let mut names: Vec<String> = std::fs::read_dir(dir.path())
                .map(|d| {
                    d.filter_map(Result::ok)
                        .map(|e| e.file_name().to_string_lossy().into_owned())
                        .collect()
                })
                .unwrap_or_default();
            names.sort();
            names
        };
        match result {
            Err(ConvertError::Cancelled) => {
                assert_eq!(beside(), [spec.folder()], "cancelled at report {point}");
            }
            Ok(summary) => {
                completed = true;
                let _ = std::fs::remove_file(&out);
                if let Some(research) = summary.research_folder {
                    let _ = std::fs::remove_dir_all(dir.path().join(research));
                }
                assert_eq!(beside(), [spec.folder()]);
                break;
            }
            Err(e) => panic!("report {point}: {e}"),
        }
    }
    assert!(
        completed,
        "a cancel after the last report never stops a whole import"
    );
}

/// A document written to while the conversion reads it (a sync service catching up) has the
/// whole project read again; the progress the writer watches still never goes back, and
/// ends at the end.
#[test]
fn progress_never_goes_back_when_the_project_is_read_again() {
    let dir = temp();
    let spec = Spec::novel(Generation::Format20, Platform::Mac);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let index = spec.index_of("Arrival").expect("the scene");
    let text = folder.join(spec.text_path(index));
    let out = dir.path().join("novel.skrib");
    let output = out.to_string_lossy().into_owned();
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let request = request(&folder, &output, &choices, &words, &answers, &options);
    let seen = std::cell::RefCell::new(Vec::<(f32, String)>::new());
    let touched = std::cell::Cell::new(false);
    let converting = std::cell::Cell::new(false);
    let report = |percent: f32, label: &str| {
        seen.borrow_mut().push((percent, label.to_string()));
        if label == "import.converting" {
            converting.set(true);
        }
        // The first row converted: its title, after the converting phase began.
        let a_row = converting.get() && !label.starts_with("import.");
        if a_row && !touched.get() {
            touched.set(true);
            let mut rtf = std::fs::read(&text).expect("the scene reads");
            let at = rtf.iter().rposition(|b| *b == b'}').expect("an RTF group");
            rtf.splice(at..at, b"\\par One more line.".iter().copied());
            std::fs::write(&text, rtf).expect("the scene writes");
        }
    };
    let summary = convert(&request, &report, &AtomicBool::new(false));
    assert!(summary.is_ok(), "{:?}", summary.err());
    let seen = seen.into_inner();
    let readings = seen
        .iter()
        .filter(|(_, label)| label == "import.outline")
        .count();
    assert_eq!(readings, 2, "the project was read again: {seen:?}");
    for pair in seen.windows(2) {
        assert!(pair[1].0 >= pair[0].0, "progress went back: {seen:?}");
    }
    assert_eq!(seen.last(), Some(&(100.0, "import.done".to_string())));
}

/// A large project's conversion reports its progress by the percent, not by the row: each
/// report is a notice redrawn on the interface's thread, and three thousand of them kept
/// that thread too busy to take the Cancel the notice offers before the import was over.
#[test]
fn a_large_conversion_reports_progress_by_the_percent() {
    let dir = temp();
    let mut spec = Spec::novel(Generation::Format20, Platform::Mac);
    let at = spec.index_of("Research").unwrap_or(spec.items.len());
    let mut more = vec![Item::new(2, Kind::Folder, "Chapter 3").compiled()];
    more.extend((0..600).map(|i| {
        Item::new(3, Kind::Text, &format!("Crossing {i}"))
            .compiled()
            .with_text(i, 1)
    }));
    spec.items.splice(at..at, more);
    let folder = write_files(&spec.build(), dir.path(), &spec.folder());
    let out = dir.path().join("large.skrib");
    let output = out.to_string_lossy().into_owned();
    let (choices, words, answers, options) =
        (Choices::default(), words(), Answers::default(), options());
    let request = request(&folder, &output, &choices, &words, &answers, &options);
    let seen = std::cell::RefCell::new(Vec::<(f32, String)>::new());
    let report = |percent: f32, label: &str| seen.borrow_mut().push((percent, label.to_string()));
    let summary = convert(&request, &report, &AtomicBool::new(false));
    assert!(summary.is_ok(), "{:?}", summary.err());
    let seen = seen.into_inner();
    // The converting phase, then the titles of the rows it reports, until writing begins.
    let start = seen
        .iter()
        .position(|(_, label)| label == "import.converting")
        .expect("the conversion says it is converting");
    let end = seen
        .iter()
        .position(|(_, label)| label == "import.writing")
        .expect("and then that it is writing");
    let converting = &seen[start..end];
    // The start, then at most one per percent of the rows.
    assert!(
        converting.len() <= 102,
        "{} reports for one conversion",
        converting.len()
    );
    assert_eq!(
        converting.last().map(|(percent, _)| *percent),
        Some(90.0),
        "the last row is reported: {:?}",
        converting.last()
    );
}

// ── what the report names rows by ────────────────────────────────────────────────────────

/// Every row written is in the summary with the uid the project holds for it, in the
/// plan's order, and every placement points at a row written.
#[test]
fn every_row_written_carries_the_uid_the_project_holds() {
    let dir = temp();
    let folder = novel_with(Generation::Format20, Platform::Mac, dir.path(), &[]);
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let bundle = read(&out);
    let held: Vec<uuid::Uuid> = items(&bundle).map(|i| i.item.uid).collect();
    let written: Vec<uuid::Uuid> = summary.rows.iter().map(|r| r.uid).collect();
    assert_eq!(written, held, "one per row, in the plan's order");
    let at: std::collections::BTreeSet<_> = summary
        .rows
        .iter()
        .map(|r| (format!("{:?}", r.at.binder), r.at.row))
        .collect();
    for placement in &summary.placements {
        for row in [
            placement.item,
            placement.text.as_ref().map(|(row, _)| *row),
            placement.notes,
            placement.synopsis,
        ]
        .into_iter()
        .flatten()
        {
            assert!(at.contains(&(format!("{:?}", row.binder), row.row)));
        }
    }
}

/// An item is named where the writer knew it: its titles from the top of the binder.
#[test]
fn an_item_is_named_by_its_place_in_the_binder() {
    let dir = temp();
    let folder = novel_with(Generation::Format15, Platform::Windows, dir.path(), &[]);
    let out = dir.path().join("novel.skrib");
    let summary = converted(&folder, &out);
    let spec = Spec::novel(Generation::Format15, Platform::Windows);
    let Some(arrival) = spec.index_of("Arrival") else {
        panic!("no fixture item Arrival");
    };
    assert_eq!(
        summary.names.items[arrival],
        ["Manuscript", "Part One", "Chapter 1", "Arrival"]
    );
    assert_eq!(summary.names.items.len(), spec.items.len());
    // The untitled scene keeps an empty title: named by nothing the import invented.
    assert!(
        summary
            .names
            .items
            .iter()
            .any(|path| path.last().is_some_and(String::is_empty))
    );
    // Format 1.x project notes are named by their own titles.
    assert_eq!(summary.names.project_notes, ["General"]);
}

/// An analysis the writer stopped says nothing, and one they did not stop reads the project.
#[test]
fn a_cancelled_analysis_is_no_analysis() {
    let dir = temp();
    let folder = novel_with(Generation::Format20, Platform::Mac, dir.path(), &[]);
    let answers = Answers::default();
    let stopped = scrivener_import::analyze_cancellable(
        &folder,
        &answers,
        &options(),
        &AtomicBool::new(true),
    );
    assert!(stopped.is_none());
    let read = scrivener_import::analyze_cancellable(
        &folder,
        &answers,
        &options(),
        &AtomicBool::new(false),
    );
    assert!(matches!(read, Some(scrivener_import::Analysis::Read(_))));
}
