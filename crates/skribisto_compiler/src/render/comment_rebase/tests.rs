// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::super::*;
use super::*;
use crate::preset::builtin_presets;
use common::entities::{Binder, BinderItem, BinderItemRole, BinderItemSubRole as SR, Work};
use skrib_format::{BinderWithItems, ItemWithContents};

fn c(id: u64, role: ContentRole, data: &str) -> Content {
    Content {
        id,
        activated: true,
        role,
        data: data.to_string(),
        ..Default::default()
    }
}

fn iwc(id: u64, sub_role: SR, contents: Vec<Content>) -> ItemWithContents {
    ItemWithContents {
        item: BinderItem {
            id,
            role: BinderItemRole::Item,
            sub_role,
            dict_language: language::parse_legacy_list("en"),
            is_exportable: true,
            activated: true,
            ..Default::default()
        },
        contents,
    }
}

fn gathered(items: Vec<ItemWithContents>) -> Gathered {
    Gathered {
        assets: Vec::new(),
        footnotes: Vec::new(),
        work: Work {
            id: 1,
            number_chapters: true,
            ..Default::default()
        },
        tags: vec![],
        dict_words: vec![],
        text_replacement_rules: vec![],
        note_templates: vec![],
        smart_punctuation: None,
        trash_infos: vec![],
        paces: vec![],
        progress_snapshots: vec![],
        comments: vec![],
        binders: vec![BinderWithItems {
            binder: Binder {
                id: 10,
                ..Default::default()
            },
            items,
        }],
        work_info: None,
    }
}

fn preset_of(id: &str) -> Preset {
    builtin_presets()
        .into_iter()
        .find(|p| p.id == id)
        .expect("preset")
}

fn req<'a>(g: &'a Gathered, include: &'a [u64], p: &'a Preset) -> RenderRequest<'a> {
    RenderRequest {
        media_dir: std::path::Path::new(""),
        gathered: g,
        include,
        preset: p,
        format: ExportFormat::Html,
        work_lang: "en",
        explicit_selection: false,
    }
}

/// Capture an anchor exactly as the editor does: against the row's own plain text.
fn anchor_on(djot: &str, phrase: &str) -> Anchor {
    let (text, starts) = djot_plain_text(djot).expect("plain");
    let chars: Vec<char> = text.chars().collect();
    let needle: Vec<char> = phrase.chars().collect();
    let start = find_from(&chars, &needle, 0)
        .unwrap_or_else(|| panic!("fixture phrase {phrase:?} is not in {text:?}"));
    let ordinal = comment_anchor::block_of(&starts, start);
    comment_anchor::capture(&text, start, start + needle.len(), ordinal)
}

/// Assemble, then place — the whole pipeline, returning the compiled text alongside so a
/// test can assert on the words a placement actually lands on.
fn run(
    g: &Gathered,
    include: &[u64],
    p: &Preset,
    comments: &[CommentToPlace],
) -> (String, Vec<CommentPlacement>) {
    let built = assemble(&req(g, include, p), &|_| {}, &AtomicBool::new(false)).expect("assemble");
    // The ADDRESSABLE text, paired with the block starts that index it — never
    // `to_plain_text()`, which omits each table's U+FFFC anchor and would put every offset
    // after a table two characters out.
    let text = built.doc.to_addressable_text().expect("addressable");
    let starts: Vec<usize> = built
        .doc
        .blocks()
        .into_iter()
        .map(|b| b.position())
        .collect();
    let placed = place_comments(&text, &starts, &built.emitted, comments);
    (text, placed)
}

fn words_at(text: &str, p: &CommentPlacement) -> String {
    match p.resolution {
        Resolution::Anchored { start, length } => text.chars().skip(start).take(length).collect(),
        Resolution::Orphan(ref r) => format!("<orphan: {r:?}>"),
    }
}

const S1: &str = "The wind rose over the hills and the salt-bleached door rattled all night.";
const S2: &str = "She counted the lamps again. The harbour was quiet, and the boats were still.";
const S3: &str = "The wind rose over the hills once more, but nobody was left to hear it.";

fn three_scene_book() -> Gathered {
    gathered(vec![
        iwc(
            100,
            SR::BookBegin,
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        ),
        iwc(
            101,
            SR::ChapterScene,
            vec![
                c(2, ContentRole::ChapterTitle, "Storms"),
                c(3, ContentRole::SceneText, S1),
            ],
        ),
        iwc(102, SR::Scene, vec![c(4, ContentRole::SceneText, S2)]),
        iwc(
            103,
            SR::ChapterScene,
            vec![
                c(5, ContentRole::ChapterTitle, "After"),
                c(6, ContentRole::SceneText, S3),
            ],
        ),
    ])
}

fn to_place(comment_id: u64, content_id: u64, djot: &str, phrase: &str) -> CommentToPlace {
    CommentToPlace {
        comment_id,
        content_id,
        anchor: anchor_on(djot, phrase),
        is_paragraph: false,
    }
}

#[test]
fn a_comment_in_each_scene_rebases_onto_the_right_words() {
    let g = three_scene_book();
    let p = preset_of("neutral");
    let include = [100u64, 101, 102, 103];
    let comments = vec![
        to_place(1, 3, S1, "salt-bleached door"),
        to_place(2, 4, S2, "the boats were still"),
        to_place(3, 6, S3, "nobody was left"),
    ];

    let (text, placed) = run(&g, &include, &p, &comments);
    assert_eq!(placed.len(), 3, "every comment's row is in the export");
    for (want, got) in [
        "salt-bleached door",
        "the boats were still",
        "nobody was left",
    ]
    .iter()
    .zip(&placed)
    {
        assert_eq!(&words_at(&text, got), want, "placement {got:?}");
    }
}

/// The case that defeats a whole-document quote search: S1 and S3 open with the *same*
/// sentence. Only the monotone window can tell them apart, and getting it wrong means one
/// scene's note silently moves to another scene.
#[test]
fn a_sentence_repeated_in_two_scenes_lands_in_the_right_one() {
    let g = three_scene_book();
    let p = preset_of("neutral");
    let include = [100u64, 101, 102, 103];
    let comments = vec![
        to_place(1, 3, S1, "The wind rose over the hills"),
        to_place(2, 6, S3, "The wind rose over the hills"),
    ];

    let (_, placed) = run(&g, &include, &p, &comments);
    let starts: Vec<usize> = placed
        .iter()
        .map(|p| match p.resolution {
            Resolution::Anchored { start, .. } => start,
            ref o => panic!("did not anchor: {o:?}"),
        })
        .collect();
    assert!(
        starts[0] < starts[1],
        "the third scene's comment must land later in the compiled document than the \
             first's — got {starts:?}. Equal or reversed means the window failed to \
             disambiguate and a note has moved to another scene."
    );
}

/// `push_prose` rewrites a scene-break marker into the preset's own rendering, so a row's
/// compiled text is not character-identical to its stored text. The window search has to
/// tolerate that.
#[test]
fn a_comment_after_a_scene_break_still_rebases() {
    let with_break = "Before the storm broke.\n\n* * *\n\nAfter it had passed for good.";
    let g = gathered(vec![
        iwc(
            100,
            SR::BookBegin,
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        ),
        iwc(
            101,
            SR::ChapterScene,
            vec![
                c(2, ContentRole::ChapterTitle, "Storms"),
                c(3, ContentRole::SceneText, with_break),
            ],
        ),
    ]);
    let p = preset_of("neutral");
    let include = [100u64, 101];
    let comments = vec![to_place(1, 3, with_break, "After it had passed")];

    let (text, placed) = run(&g, &include, &p, &comments);
    assert_eq!(words_at(&text, &placed[0]), "After it had passed");
}

/// A comment on a row outside the export scope is **absent**, not orphaned. Warning about
/// it would cry wolf on every "Export Chapter 5".
#[test]
fn a_comment_on_an_out_of_scope_row_is_absent_rather_than_orphaned() {
    let g = three_scene_book();
    let p = preset_of("neutral");
    // Chapter 3 (item 103, content 6) is excluded from the scope.
    let include = [100u64, 101, 102];
    let comments = vec![
        to_place(1, 3, S1, "salt-bleached door"),
        to_place(2, 6, S3, "nobody was left"),
    ];

    let (_, placed) = run(&g, &include, &p, &comments);
    assert_eq!(
        placed.len(),
        1,
        "only the in-scope comment is reported at all: {placed:?}"
    );
    assert_eq!(placed[0].comment_id, 1);
    assert!(orphan_reason(&placed[0]).is_none());
}

/// A comment whose quoted words are gone comes back as a typed orphan, never as a
/// plausible-looking guess.
#[test]
fn a_comment_whose_text_is_gone_reports_a_typed_orphan() {
    let g = three_scene_book();
    let p = preset_of("neutral");
    let include = [100u64, 101, 102, 103];
    let mut c1 = to_place(1, 3, S1, "salt-bleached door");
    c1.anchor.exact = "a sentence that was deleted entirely".into();
    c1.anchor.prefix = String::new();
    c1.anchor.suffix = String::new();

    let (_, placed) = run(&g, &include, &p, &[c1]);
    assert_eq!(
        placed.len(),
        1,
        "the row IS in scope, so it must be reported"
    );
    assert_eq!(
        orphan_reason(&placed[0]),
        Some(CommentOrphanReason::TextNotFound),
        "got {:?}",
        placed[0].resolution
    );
}

/// A synopsis is its own window. Two remarks quoting the same sentence — one on the scene,
/// one on the summary of it — must not be able to claim each other's position.
#[test]
fn a_synopsis_comment_is_placed_in_the_synopsis_not_the_scene() {
    let shared = "The lamp guttered in the hallway.";
    let g = gathered(vec![
        iwc(
            100,
            SR::BookBegin,
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        ),
        iwc(
            101,
            SR::ChapterScene,
            vec![
                c(2, ContentRole::ChapterTitle, "Storms"),
                c(3, ContentRole::SceneText, shared),
                c(4, ContentRole::SynopsisText, shared),
            ],
        ),
    ]);
    let mut p = preset_of("neutral");
    p.include_synopses = true;
    let include = [100u64, 101];
    let comments = vec![
        to_place(1, 3, shared, "lamp guttered"),
        to_place(2, 4, shared, "lamp guttered"),
    ];

    let (_, placed) = run(&g, &include, &p, &comments);
    assert_eq!(placed.len(), 2);
    let (a, b) = (&placed[0].resolution, &placed[1].resolution);
    match (a, b) {
        (Resolution::Anchored { start: sa, .. }, Resolution::Anchored { start: sb, .. }) => {
            assert_ne!(
                sa, sb,
                "the scene's comment and the synopsis's must land at different offsets"
            )
        }
        _ => panic!("both should anchor: {a:?} {b:?}"),
    }
}

/// **Regression.** A paragraph comment covering a paragraph *and* the scene-break marker
/// after it must never widen to the paragraph beyond.
///
/// Under `SceneBreak::BlankLine` — the minor break nearly every built-in preset uses — the
/// marker renders to no characters at all, so the row's third block sits immediately after
/// its first in the compiled document. Deriving the extent from the stored `block_span`
/// then counted one block too far and bracketed text the writer never selected, silently:
/// no orphan, and `comments_written` reported it as a success.
#[test]
fn a_paragraph_comment_does_not_swallow_the_paragraph_after_a_vanished_scene_break() {
    let with_break = "Before the storm broke.\n\n\\* \\* \\*\n\nAfter it had passed for good.";
    let g = gathered(vec![
        iwc(
            100,
            SR::BookBegin,
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        ),
        iwc(
            101,
            SR::ChapterScene,
            vec![
                c(2, ContentRole::ChapterTitle, "Storms"),
                c(3, ContentRole::SceneText, with_break),
            ],
        ),
    ]);
    let p = preset_of("neutral");
    let include = [100u64, 101];

    // Captured as the editor captures a selection covering the first paragraph and the
    // marker's own block — but NOT the paragraph after it.
    let (row_text, row_starts) = djot_plain_text(with_break).expect("plain");
    let end = row_starts
        .get(2)
        .copied()
        .unwrap_or(row_text.chars().count());
    let mut anchor = comment_anchor::capture(&row_text, 0, end.saturating_sub(1), 0);
    anchor.block_span = 2;

    let comments = vec![CommentToPlace {
        comment_id: 1,
        content_id: 3,
        anchor,
        is_paragraph: true,
    }];

    let (text, placed) = run(&g, &include, &p, &comments);
    let got = words_at(&text, &placed[0]);
    assert!(
        !got.contains("After it had passed"),
        "the comment must not reach the paragraph after the break — got {got:?}"
    );
    assert!(
        got.starts_with("Before the storm broke."),
        "…and must still cover the paragraph it was made on — got {got:?}"
    );
}

/// **Regression.** A comment with no quote at all — the shape a legacy
/// `CommentAnchorKind::Document` row stores — must orphan, not resolve to a zero-width
/// range at offset 0.
///
/// An empty needle matches the empty slice anywhere, so without an explicit guard this
/// "succeeded" and was written into the exported file as a comment bracketing nothing,
/// pinned to the start of its row.
#[test]
fn a_comment_with_no_quote_orphans_instead_of_pinning_itself_at_the_start() {
    let g = three_scene_book();
    let p = preset_of("neutral");
    let include = [100u64, 101, 102, 103];
    let comments = vec![CommentToPlace {
        comment_id: 1,
        content_id: 3,
        anchor: Anchor::default(),
        is_paragraph: false,
    }];

    let (_, placed) = run(&g, &include, &p, &comments);
    assert_eq!(
        placed.len(),
        1,
        "the row is in scope, so it must be reported"
    );
    assert_eq!(
        orphan_reason(&placed[0]),
        Some(CommentOrphanReason::TextNotFound),
        "got {:?}",
        placed[0].resolution
    );
}

/// **Regression.** A row's block must be located at a compiled *block boundary*, never
/// mid-paragraph.
///
/// A short, common block matched wherever those characters first occurred, which could drag
/// the window into an unrelated row and take every later row's cursor with it.
#[test]
fn a_block_is_never_located_in_the_middle_of_another_paragraph() {
    // Scene 1 contains the words "Yes." inside a longer sentence; scene 2 IS "Yes.".
    let s1 = "He asked whether it was true. Yes. she said, and turned away.";
    let s2 = "Yes.";
    let g = gathered(vec![
        iwc(
            100,
            SR::BookBegin,
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        ),
        iwc(101, SR::Scene, vec![c(3, ContentRole::SceneText, s1)]),
        iwc(102, SR::Scene, vec![c(4, ContentRole::SceneText, s2)]),
    ]);
    let p = preset_of("neutral");
    let include = [100u64, 101, 102];
    let comments = vec![to_place(1, 4, s2, "Yes.")];

    let (text, placed) = run(&g, &include, &p, &comments);
    assert_eq!(placed.len(), 1);
    let start = match placed[0].resolution {
        Resolution::Anchored { start, .. } => start,
        ref o => panic!("did not anchor: {o:?}"),
    };
    let before: String = text.chars().take(start).collect();
    assert!(
        before.contains("turned away"),
        "scene 2's comment landed inside scene 1's sentence instead of on scene 2 — \
             text before it was {before:?}"
    );
}

/// With synopses switched off, a comment on one is out of scope — absent, not orphaned.
#[test]
fn a_synopsis_comment_is_absent_when_the_preset_omits_synopses() {
    let shared = "The lamp guttered in the hallway.";
    let g = gathered(vec![
        iwc(
            100,
            SR::BookBegin,
            vec![c(1, ContentRole::BookTitle, "My Novel")],
        ),
        iwc(
            101,
            SR::ChapterScene,
            vec![
                c(2, ContentRole::ChapterTitle, "Storms"),
                c(3, ContentRole::SceneText, shared),
                c(4, ContentRole::SynopsisText, shared),
            ],
        ),
    ]);
    let mut p = preset_of("neutral");
    p.include_synopses = false;
    let include = [100u64, 101];
    let comments = vec![to_place(2, 4, shared, "lamp guttered")];

    let (_, placed) = run(&g, &include, &p, &comments);
    assert!(
        placed.is_empty(),
        "a synopsis the preset omits is not in the document, so its comment has not \
             failed — it is simply not here: {placed:?}"
    );
}
