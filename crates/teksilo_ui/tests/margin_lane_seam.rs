// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The margin lane's provider seam, exercised **from outside the crate**.
//!
//! ## Why this is a `tests/` file and not a unit test
//!
//! A registry's own tests ask "does this hold what I put in it", and they pass
//! whether or not anything can actually use it. That question has been answered
//! green here before while three of four extension slots were broken: one could
//! not draw at all, one lived behind a `pub(crate)` module so nothing outside
//! could reach it, and one captured state at registration time — before the app
//! that state belongs to exists.
//!
//! The only thing that asks the useful question is a **consumer compiled as its
//! own crate**, which is exactly what a `tests/` file is. `pub(crate)` cannot
//! leak in here, so if this compiles, an extension can do the same.
//!
//! ## What it covers
//!
//! Registration, the refusals that protect a persisted id, the round trip from
//! a registered provider to marks on the widget, and — the part a registry test
//! structurally cannot see — that the lane has **height in two different parent
//! shapes**. That last one is not hypothetical: a registered Analysis category
//! shipped rendering as zero pixels because its body was measured only in the
//! parent that bounds height, never in the one that proposes unbounded.

use std::rc::Rc;

use teksilo::core::widget_tree::WidgetTree;
use teksilo::prelude::*;
use teksilo::widgets::{Expand, HStack};
use teksilo_ui::widgets::{LaneColumn, LaneMark, LaneShape, LaneSpan, MarginLane};

use teksilo_ui::margin_lane::{
    LaneProviderSpec, LaneRefresh, LaneSurface, builtin_ids, namespace_of, register_lane_provider,
    registered, registered_for, resolve_slot,
};

/// A provider an extension could plausibly write.
fn provider(id: &str, surfaces: &'static [LaneSurface]) -> LaneProviderSpec {
    LaneProviderSpec {
        id: id.to_string(),
        label: Rc::new(|| lit!("Beats")),
        hint: Rc::new(|| lit!("Where the structure puts them")),
        column: LaneColumn::Right,
        shape: LaneShape::Diamond,
        palette_slot: 2,
        surfaces,
        default_on: true,
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|_ctx| Vec::new()),
    }
}

#[test]
fn an_extension_can_register_a_provider_and_see_it_on_its_surface() {
    let _h = register_lane_provider("demo.beats", provider("beats", &[LaneSurface::Stream]))
        .expect("an out-of-crate caller can register");

    let on_stream = registered_for(LaneSurface::Stream);
    assert_eq!(on_stream.len(), 1, "the provider reaches its own surface");
    assert_eq!(on_stream[0].id, "beats");

    assert!(
        registered_for(LaneSurface::Editor).is_empty(),
        "and not the surfaces it did not list"
    );
}

/// The id becomes `editor.margin_lane.provider.<id>` in the writer's settings,
/// so two claimants would make a saved preference ambiguous rather than merely
/// crowded. The error has to name the holder, or the second author has nothing
/// to go on.
#[test]
fn a_taken_id_is_refused_and_the_error_names_who_holds_it() {
    let _h = register_lane_provider("demo.first", provider("collide", &[LaneSurface::Editor]))
        .expect("first registration");

    let err = register_lane_provider("demo.second", provider("collide", &[LaneSurface::Editor]))
        .expect_err("a second namespace must not claim it");

    assert!(
        err.contains("demo.first"),
        "the refusal must name the holder, got: {err}"
    );
}

#[test]
fn the_community_editions_own_ids_are_refused_to_an_extension() {
    for id in builtin_ids() {
        assert!(
            register_lane_provider("demo.greedy", provider(id, &[LaneSurface::Editor])).is_err(),
            "'{id}' is the community edition's and must be refused"
        );
    }
}

/// A drop-handle that outlives its registration is the shape every other door
/// here uses, and getting it wrong leaves a dead provider on a writer's settings
/// page after the extension is gone.
#[test]
fn dropping_the_handle_removes_the_provider() {
    {
        let _h = register_lane_provider("demo.temp", provider("temporary", &[LaneSurface::Editor]))
            .unwrap();
        assert!(registered().iter().any(|s| s.id == "temporary"));
    }
    assert!(
        !registered().iter().any(|s| s.id == "temporary"),
        "the provider outlived its handle"
    );
}

/// The round trip that matters: a provider's marks reach the widget, and the
/// widget resolves them to the geometry it will draw and speak.
#[test]
fn a_providers_marks_reach_the_widget_and_resolve_to_geometry() {
    let marks = vec![
        LaneMark {
            id: 1,
            span: LaneSpan::at(0.25),
            column: LaneColumn::Right,
            shape: LaneShape::Diamond,
            color: Color::from_hex("#009E73"),
            label: lit!("the first turn"),
            group: 7,
        },
        LaneMark {
            id: 2,
            span: LaneSpan::at(0.75),
            column: LaneColumn::Right,
            shape: LaneShape::DiamondHollow,
            color: Color::from_hex("#009E73"),
            label: lit!("the midpoint"),
            group: 7,
        },
    ];

    let lane = MarginLane::new(marks, Signal::new(0.0), Signal::new(1.0));

    let bounds = Rect::new(0.0, 0.0, 12.0, 1000.0);
    let resolved = lane.resolve_marks(bounds);

    assert_eq!(resolved.len(), 2, "both marks survived to geometry");
    assert!(
        resolved[0].top < resolved[1].top,
        "and kept their order down the strip"
    );
    // A quarter of the way down, less the half-height a point mark is grown by.
    assert!((resolved[0].top - (250.0 - 1.5)).abs() < 0.5);
}

/// A provider names a palette slot, never a colour, and the host resolves it.
/// This is what stops an extension painting a red stripe down the side of a
/// manuscript, or shipping a mark that fails contrast on a theme it never saw.
#[test]
fn a_palette_slot_resolves_to_a_colour_that_clears_contrast() {
    for colors in [
        teksilo::tokens::ColorTokens::light_default(),
        teksilo::tokens::ColorTokens::dark_default(),
    ] {
        let c = resolve_slot(&colors, provider("x", &[LaneSurface::Editor]).palette_slot);
        assert!(
            c.contrast_ratio(colors.surface_main) >= 3.0,
            "a provider's mark must be legible without the provider having to know the theme"
        );
    }
}

// ── the part a registry's own tests cannot see ────────────────────────────────

fn tree() -> WidgetTree {
    WidgetTree::new()
}

/// Height of the lane when mounted in `parent`.
fn height_in(parent: Box<dyn Widget>) -> f32 {
    let mut tree = tree();
    let id = tree.add_boxed(parent);
    tree.layout(SizeProposal::exact(700.0, 900.0));

    fn deepest(tree: &WidgetTree, id: WidgetId, depth: usize) -> f32 {
        if depth > 4 {
            return tree.bounds(id).height;
        }
        tree.children(id)
            .first()
            .map_or(tree.bounds(id).height, |c| deepest(tree, *c, depth + 1))
    }
    deepest(&tree, id, 0)
}

fn a_lane() -> MarginLane {
    MarginLane::new(
        vec![LaneMark {
            id: 1,
            span: LaneSpan::at(0.5),
            column: LaneColumn::Full,
            shape: LaneShape::Bar,
            color: Color::from_hex("#0072B2"),
            label: lit!("a mark"),
            group: 0,
        }],
        Signal::new(0.0),
        Signal::new(1.0),
    )
}

/// **A lane must have height in both parent shapes it will actually meet.**
///
/// A registered Analysis category shipped as zero pixels because its body was
/// only ever measured in the parent that bounds height, never in the one that
/// proposes unbounded — blank, with no error and nothing to grep for. A lane
/// sits beside a scroll area in one place and inside a flex row in another, so
/// it is measured both ways here.
#[test]
fn the_lane_has_height_beside_a_scroll_area_and_inside_a_flex_row() {
    let bounded = height_in(Box::new(
        HStack::new()
            .child(Expand::new().child(teksilo::widgets::Spacer::new()))
            .child(a_lane()),
    ));
    assert!(
        bounded > 0.0,
        "the lane collapsed to nothing beside an expanding sibling"
    );

    let alone = height_in(Box::new(HStack::new().child(a_lane())));
    assert!(alone > 0.0, "the lane collapsed to nothing on its own");
}

/// The lane takes a fixed width and gives the prose everything else.
///
/// Measured **beside an expanding sibling**, which is the only arrangement that
/// asks the question: at the root of a tree every widget is handed the exact
/// proposal, so a lane that wrongly claimed flex would look correct there. In a
/// real editor the lane's neighbour is the prose, and a lane that ate into the
/// measure the application centres would be a visible regression on every page.
#[test]
fn the_lane_takes_its_declared_width_beside_an_expanding_sibling() {
    let mut tree = tree();
    let root = tree.add_boxed(Box::new(
        HStack::new()
            .child(Expand::new().child(teksilo::widgets::Spacer::new()))
            .child(a_lane()),
    ));
    tree.layout(SizeProposal::exact(700.0, 900.0));

    // Asserted on the split rather than by walking to a particular node: the
    // wrapper shape around an `Expand` is an implementation detail, and a test
    // that hard-codes it breaks on an unrelated refactor while proving nothing.
    let widths: Vec<f32> = tree
        .children(root)
        .iter()
        .map(|c| tree.bounds(*c).width)
        .collect();

    assert!(
        widths.iter().any(|w| (w - 12.0).abs() < 0.01),
        "no child took the lane's declared 12 px; widths were {widths:?}"
    );
    assert!(
        widths.iter().any(|w| *w > 600.0),
        "the prose side did not get the rest; widths were {widths:?}"
    );
    let total: f32 = widths.iter().sum();
    assert!(
        (total - 700.0).abs() < 0.01,
        "the row did not fill its proposal; widths were {widths:?}"
    );
}

// ── the providers, against a real laid-out editor ─────────────────────────────
//
// Everything above asks whether the registry holds what it was given. These ask
// the question a registry structurally cannot: does a provider's mark land where
// the text actually is. The conversion runs through the editor's own geometry, so
// it needs an editor, laid out, with a document in it — which is exactly what a
// registry's own tests never have and why three of four extension slots shipped
// broken the last time nobody wrote this half.

use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::{EditorHandle, RichTextEditor};
use teksilo_ui::margin_lane::{
    CommentAnchor, LaneCall, LaneContext, LaneExtent, LaneQuery, active_query,
    install_builtin_providers, locate_offset, set_active_query,
};

/// A document with paragraphs of very unequal length, laid out narrow enough to
/// wrap, and its handle.
///
/// Unequal on purpose: with uniform paragraphs the character fraction and the pixel
/// fraction agree, and a lane computing the wrong one would pass.
fn laid_out(text: &str) -> (EditorHandle, WidgetTree) {
    let doc = TextDocument::new();
    doc.set_plain_text(text).unwrap();
    let editor = RichTextEditor::editor(doc);
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let _id = tree.add(editor);
    tree.layout(SizeProposal::exact(320.0, 240.0));
    let _ = tree.render();
    (handle, tree)
}

const SCENE: &str = "\
Short.

The ferry left before the light did, and for a long while the only thing anyone \
on the upper deck could hear was the water going past, which was not a sound so \
much as an absence of one, and which went on for as long as the crossing did.

\u{201C}Then go,\u{201D} she said.

Another very long paragraph, this one about the harbour and the way the cranes \
stood over it like something patient, and how the light came off the water in \
pieces that never quite joined up into anything a person could look at directly.

End.";

/// **The conversion the whole feature rests on.**
///
/// `offset / character_count` would be wrong here and this is what says so: the
/// The first offset is at the top of the page, the last at the bottom, and nothing
/// in between goes backwards. Ordering is what every mark on the strip depends on:
/// a lane whose fractions were not monotone would draw a later hit above an earlier
/// one, and merging would fold together marks that are nowhere near each other.
#[test]
fn a_document_offset_becomes_the_fraction_of_the_page_the_text_is_actually_at() {
    let (handle, _tree) = laid_out(SCENE);

    let at = |offset: usize| locate_offset(&handle, LaneExtent::WHOLE, offset).expect("laid out");

    let first = at(0);
    let last = at(SCENE.chars().count());
    assert!(first < 0.1, "the first line is at the top: {first}");
    assert!(last > 0.9, "and the last at the bottom: {last}");

    // And it is monotone, which is the property every mark's ordering depends on.
    let mut previous = -1.0_f32;
    for offset in (0..SCENE.chars().count()).step_by(37) {
        let f = at(offset);
        assert!(f >= previous, "fractions must not go backwards at {offset}");
        previous = f;
    }
}

/// The same conversion, on the document shape that makes the naive answer visibly
/// wrong: twenty one-word paragraphs, then one very long one.
///
/// Twenty short paragraphs are eighty characters and twenty lines; the long one is
/// two thousand characters and, wrapped, forty-odd. So the halfway *character* is
/// deep inside the long paragraph while the halfway *line* is nowhere near it, and
/// `offset / character_count` puts its mark a third of a page from where the text
/// it names actually sits.
#[test]
fn the_character_fraction_and_the_pixel_fraction_are_not_the_same_number() {
    let mut text = String::new();
    for i in 0..20 {
        text.push_str(&format!("Line {i}.\n\n"));
    }
    let long: String = std::iter::repeat_n("the water going past ", 100).collect();
    text.push_str(&long);

    let (handle, _tree) = laid_out(&text);
    let chars = text.chars().count();
    let middle = chars / 2;

    let by_characters = middle as f32 / chars as f32;
    let by_geometry = locate_offset(&handle, LaneExtent::WHOLE, middle).expect("laid out");

    assert!(
        by_geometry - by_characters > 0.08,
        "the halfway character sits well past halfway down the page: \
         geometry {by_geometry} against characters {by_characters}"
    );
}

/// A settings store of this test's own, so one test's toggles cannot reach another.
fn temp_store() -> teksilo::settings::SettingsStore {
    use std::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "skribisto_lane_seam_{}_{n}.toml",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    teksilo::settings::SettingsStore::open(path).expect("open temp store")
}

fn call<'a>(
    app_ctx: &'a std::rc::Rc<frontend::AppContext>,
    ids: &'a teksilo_ui::app_ids::AppIds,
    doc: &'a TextDocument,
    anchors: &'a [CommentAnchor],
    locate: &'a dyn Fn(usize) -> Option<f32>,
    surface: LaneSurface,
) -> LaneCall<'a> {
    LaneCall {
        kind: teksilo_ui::ext::EditorKind::Prose,
        app_ctx,
        ids,
        surface,
        doc,
        item_id: 1,
        comment_anchors: anchors,
        misspellings: &[],
        locate,
    }
}

fn spec_named(id: &str) -> LaneProviderSpec {
    registered()
        .into_iter()
        .find(|s| s.id == id)
        .unwrap_or_else(|| panic!("'{id}' is not registered"))
}

/// The search provider, end to end: a query published by whichever search the
/// writer last used, matched against the live document, placed by real geometry.
#[test]
fn the_search_provider_marks_every_hit_where_the_word_actually_is() {
    let _handles = install_builtin_providers();
    let doc = TextDocument::new();
    doc.set_plain_text(SCENE).unwrap();
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let _id = tree.add(editor);
    tree.layout(SizeProposal::exact(320.0, 240.0));
    let _ = tree.render();

    set_active_query(Some(LaneQuery {
        text: "the".into(),
        case_sensitive: false,
        whole_word: true,
        diacritic_sensitive: false,
        source: teksilo_ui::margin_lane::LaneQuerySource::Editor(1),
        current: Some(1),
    }));

    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let locate = teksilo_ui::margin_lane::locator(handle, LaneExtent::WHOLE);
    let anchors: Vec<CommentAnchor> = Vec::new();
    let c = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Editor);

    let marks = c.run(&spec_named("search"), Color::from_hex("#0072B2"), 5);
    assert!(
        marks.len() > 3,
        "'the' occurs many times in this scene, got {}",
        marks.len()
    );
    assert!(
        marks.windows(2).all(|w| w[0].span.start <= w[1].span.start),
        "hits must come out in document order"
    );

    // **The hit the writer is standing on is longer, not wider.** Every hit sits in
    // the same column; the current one is a `BarCurrent`, which draws at the column's
    // width and three times the length.
    //
    // It used to be the full width of the mark area against a third of it for its
    // siblings, so the column's occupied width was a function of the query -- wide
    // where the current hit was, narrow elsewhere, absent when nothing matched.
    // Typing "a", then "ar", then a word that matched nothing changed it three
    // times, which is what a reader sees long before they see which mark is current.
    // A difference of length rather than of colour, still, so it survives a reader
    // who cannot separate two hues.
    assert!(
        marks.iter().all(|m| m.column == LaneColumn::Center),
        "every hit shares one column, so the column's width says nothing about the query"
    );
    assert_eq!(
        marks
            .iter()
            .filter(|m| m.shape == LaneShape::BarCurrent)
            .count(),
        1,
        "exactly one current hit"
    );
    assert_eq!(
        marks.iter().filter(|m| m.shape == LaneShape::Bar).count(),
        marks.len() - 1,
        "and every other hit is an ordinary bar"
    );

    // Nothing searched for, nothing marked — not every word in the document.
    set_active_query(None);
    assert!(
        c.run(&spec_named("search"), Color::from_hex("#0072B2"), 5)
            .is_empty()
    );
    assert!(active_query().get().is_none());
}

/// A comment's mark covers the sentence it is on, not a point at the top of it.
#[test]
fn the_comments_provider_spans_the_text_a_thread_is_attached_to() {
    let _handles = install_builtin_providers();
    let doc = TextDocument::new();
    doc.set_plain_text(SCENE).unwrap();
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let _id = tree.add(editor);
    tree.layout(SizeProposal::exact(320.0, 240.0));
    let _ = tree.render();

    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let locate = teksilo_ui::margin_lane::locator(handle, LaneExtent::WHOLE);
    // Across the long second paragraph, which wraps to several lines.
    let anchors = vec![CommentAnchor {
        id: 42,
        start: 20,
        end: 240,
        label: lit!("“the water going past”"),
    }];
    let c = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Editor);

    let marks = c.run(&spec_named("comments"), Color::from_hex("#BD8300"), 9);
    assert_eq!(marks.len(), 1);
    let m = &marks[0];
    assert_eq!(m.id, 42, "the comment's own id, so the node holds still");
    assert!(
        m.span.end - m.span.start > 0.05,
        "a comment over a wrapped paragraph is a span, not a point: {:?}",
        m.span
    );
    assert_eq!(m.column, LaneColumn::Right);
}

/// **Do the built-ins compose?**
///
/// Every registry's own tests answer "does this hold what I put in it", which is
/// green whether or not the three that ship can actually all be present at once.
/// They cannot, if two take one namespace: a registration replaces its namespace's
/// entry, so the second would silently unregister the first with no error and
/// nothing to grep for.
#[test]
fn the_built_in_providers_all_install_together() {
    const BUILT_IN: [&str; 4] = ["comments", "search", "boundaries", "spelling"];

    let _handles = install_builtin_providers();
    let ids: Vec<String> = registered().into_iter().map(|s| s.id).collect();
    for expected in BUILT_IN {
        assert!(
            ids.iter().any(|id| id == expected),
            "'{expected}' is missing; got {ids:?}"
        );
    }

    // Distinct namespaces, which is what the previous assertion depends on and what
    // a shared one would silently break.
    let namespaces: std::collections::BTreeSet<String> =
        BUILT_IN.into_iter().filter_map(namespace_of).collect();
    assert_eq!(namespaces.len(), BUILT_IN.len(), "got {namespaces:?}");

    // Three columns and four providers, so one column is shared and the shape is
    // what keeps its occupants apart. That is the invariant now: a pair may share
    // a column, and then it may not share a shape.
    let mut seen: std::collections::BTreeSet<(String, String)> = Default::default();
    for spec in registered()
        .into_iter()
        .filter(|s| BUILT_IN.contains(&s.id.as_str()))
    {
        let key = (format!("{:?}", spec.column), format!("{:?}", spec.shape));
        assert!(
            seen.insert(key.clone()),
            "two built-ins draw the same shape in the same column: {key:?}"
        );
    }

    // Three on, one off. The budget is deliberate: a lane that lights up with
    // everything is a cockpit, and spelling marks a machine's opinion of the
    // writer's prose rather than anything they put there.
    let on: Vec<String> = registered()
        .into_iter()
        .filter(|s| BUILT_IN.contains(&s.id.as_str()) && s.default_on)
        .map(|s| s.id)
        .collect();
    assert_eq!(on.len(), 3, "got {on:?}");
    assert!(
        !on.contains(&"spelling".to_string()),
        "spelling must be off"
    );
}

/// Boundaries are a stream's business and nothing else's: a tab holds one document,
/// so a rule at the top of it would say what the tab title already says.
#[test]
fn document_boundaries_appear_in_a_stream_and_nowhere_else() {
    let _handles = install_builtin_providers();
    let on_stream: Vec<String> = registered_for(LaneSurface::Stream)
        .into_iter()
        .map(|s| s.id)
        .collect();
    assert!(on_stream.iter().any(|id| id == "boundaries"));

    for surface in [LaneSurface::Editor, LaneSurface::SearchPreview] {
        assert!(
            !registered_for(surface)
                .into_iter()
                .any(|s| s.id == "boundaries"),
            "boundaries must not appear on {surface:?}"
        );
    }
}

/// **The spelling provider marks what it is handed, and nothing else.**
///
/// It reads the set the editor is already showing rather than checking the
/// document itself, which is the whole reason it cannot flag the word the writer
/// is mid-way through typing: the caret exemption has already been applied by the
/// time this sees it. Handed nothing -- no session on this surface, or a language
/// with no dictionary -- it contributes nothing, and says nothing about why.
#[test]
fn spelling_marks_exactly_the_misspellings_it_is_given() {
    let _handles = install_builtin_providers();
    let (handle, _tree) = laid_out(SCENE);
    let locate = teksilo_ui::margin_lane::locator(handle, LaneExtent::WHOLE);

    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let doc = TextDocument::new();
    doc.set_plain_text(SCENE).expect("set_plain_text");
    let anchors: Vec<CommentAnchor> = Vec::new();

    // Nothing handed down: no marks, and emphatically not a document scan.
    let quiet = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Editor);
    assert!(
        quiet
            .run(&spec_named("spelling"), Color::from_hex("#009E73"), 3)
            .is_empty(),
        "with no session there is nothing to mark"
    );

    // Two flagged words, two marks, each a point at its word. Not a span: the
    // locator answers with the middle of the line an offset is on, so both ends
    // of a word inside one line are the same fraction.
    let flagged = [(0usize, 5usize), (20, 4)];
    let mut c = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Editor);
    c.misspellings = &flagged;
    let marks = c.run(&spec_named("spelling"), Color::from_hex("#009E73"), 3);
    assert_eq!(marks.len(), 2, "one mark per flagged word");
    assert!(
        marks[0].span.start < marks[1].span.start,
        "a word later in the prose marks lower down: {:?} then {:?}",
        marks[0].span,
        marks[1].span
    );
    // The offset is the id, so inserting a word above does not renumber every
    // mark below it -- which would hand a screen reader a tree that will not
    // hold still.
    assert_eq!(marks[0].id, 0);
    assert_eq!(marks[1].id, 20);
}

/// Each row of a stream maps its own document onto its own slice of one lane, and a
/// row's boundary lands at the top of that slice.
#[test]
fn a_stream_rows_marks_land_inside_that_rows_slice_of_the_lane() {
    let _handles = install_builtin_providers();
    let (handle, _tree) = laid_out(SCENE);

    // The second of three rows, occupying the middle half of the extent.
    let row = LaneExtent::slice(250.0, 500.0, 1000.0).unwrap();
    let locate = teksilo_ui::margin_lane::locator(handle, row);

    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let doc = TextDocument::new();
    let anchors: Vec<CommentAnchor> = Vec::new();
    let c = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Stream);

    let marks = c.run(&spec_named("boundaries"), Color::from_hex("#009E73"), 3);
    assert_eq!(marks.len(), 1, "one rule per row");
    let at = marks[0].span.start;
    assert!(
        (0.25..0.30).contains(&at),
        "the rule belongs at the top of the row's own slice, got {at}"
    );
    assert_eq!(marks[0].shape, LaneShape::Rule);
}

/// **What the host takes back from a provider, exercised through the real pipeline.**
///
/// A provider returning a red mark numbered zero must not be able to keep either.
/// Colour, because that is what stops an extension painting a red stripe down the
/// side of a manuscript or shipping a mark that fails contrast on a theme it never
/// saw. Group, because it is half of a mark's accessibility node identity, and two
/// providers each numbering their marks from one would hand a screen reader two
/// different things under one id.
#[test]
fn the_host_overwrites_what_a_provider_claimed_about_colour_and_group() {
    let red = Color::from_hex("#FF0000");
    let mut loud = provider("loud", &[LaneSurface::Editor]);
    loud.palette_slot = 4;
    loud.marks = Rc::new(|_ctx| {
        vec![LaneMark {
            id: 1,
            span: LaneSpan::at(0.5),
            column: LaneColumn::Left,
            shape: LaneShape::Dot,
            color: Color::from_hex("#FF0000"),
            label: lit!("loud"),
            group: 0,
        }]
    });
    let _h = register_lane_provider("demo.loud", loud).unwrap();

    let store = temp_store();
    let colors = teksilo::tokens::ColorTokens::light_default();
    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let doc = TextDocument::new();
    let anchors: Vec<CommentAnchor> = Vec::new();
    let locate = |_: usize| Some(0.5_f32);
    let c = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Editor);

    let marks = teksilo_ui::margin_lane::resolve::marks(
        &store,
        &colors,
        LaneSurface::Editor,
        |spec, color, group| c.run(spec, color, group),
    );

    let loud = marks
        .iter()
        .find(|m| m.shape == LaneShape::Dot)
        .expect("the provider's mark came through");
    assert_ne!(loud.color, red, "the red stripe did not survive");
    assert_eq!(
        loud.color,
        resolve_slot(&colors, 4),
        "it wears the slot it named, resolved against the live theme"
    );
    assert_eq!(
        loud.group,
        teksilo_ui::margin_lane::group_of("loud"),
        "and the group the host decided, not the zero it claimed"
    );
}

/// The three switches, checked in the order that makes each of the next irrelevant.
/// A writer who turned the lane off must not be paying for a document walk on every
/// keystroke to produce marks nothing will draw.
#[test]
fn each_of_the_three_switches_silences_the_lane_on_its_own() {
    let _handles = install_builtin_providers();
    let store = temp_store();
    let colors = teksilo::tokens::ColorTokens::light_default();
    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let doc = TextDocument::new();
    doc.set_plain_text(SCENE).unwrap();
    let anchors = vec![CommentAnchor {
        id: 42,
        start: 20,
        end: 40,
        label: lit!("a note"),
    }];
    let locate = |_: usize| Some(0.5_f32);
    let c = call(&app_ctx, &ids, &doc, &anchors, &locate, LaneSurface::Editor);
    let run = |store: &teksilo::settings::SettingsStore| {
        teksilo_ui::margin_lane::resolve::marks(store, &colors, LaneSurface::Editor, |s, k, g| {
            c.run(s, k, g)
        })
    };

    assert!(!run(&store).is_empty(), "the defaults show the comment");

    // 1. The lane itself, off the View menu.
    let enabled = store.signal("editor.margin_lane.enabled", true);
    enabled.set(false);
    assert!(run(&store).is_empty(), "the lane's own switch silences it");
    enabled.set(true);

    // 2. This surface.
    let surface = store.signal("editor.margin_lane.surface.editor", true);
    surface.set(false);
    assert!(run(&store).is_empty(), "the surface switch silences it");
    surface.set(true);

    // 3. This provider, and only this provider.
    let comments = store.signal("editor.margin_lane.provider.comments", true);
    comments.set(false);
    assert!(
        !run(&store).iter().any(|m| m.id == 42),
        "the provider switch silences its own marks"
    );
}

// ── the texture column ───────────────────────────────────────────────────────

/// **A ladder of speech against a wall of narration**, which is the one thing about
/// a scene's rhythm that survives being shrunk to twenty-eight pixels.
///
/// A bar per paragraph, its length the word count and its filled part the share a
/// reader hears as speech. Asserted on a scene built so the two are unmistakable:
/// one line of pure dialogue, one long paragraph of pure narration.
#[test]
fn the_texture_draws_dialogue_as_a_share_of_each_paragraph() {
    let scene = "\u{201C}Then go,\u{201D} she said.\n\n        The ferry left before the light did, and for a long while the only thing anyone \
        on the upper deck could hear was the water going past.";
    let doc = TextDocument::new();
    doc.set_plain_text(scene).unwrap();
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let _id = tree.add(editor);
    tree.layout(SizeProposal::exact(320.0, 240.0));
    let _ = tree.render();

    let span_of = |start: usize, end: usize| {
        teksilo_ui::margin_lane::locate_span(&handle, LaneExtent::WHOLE, start, end)
    };
    let markers = skribisto_model::analysis::prose_stats::markers_for(
        "en",
        common::entities::QuoteStyle::LocaleDefault,
    );

    let bars = teksilo_ui::margin_lane::texture::bars(&doc, markers, &span_of, 240.0);
    assert_eq!(bars.len(), 2, "one bar per paragraph: {bars:?}");

    let quoted = &bars[0];
    let narration = &bars[1];
    // Two of its four words are inside the quotation; "she said" is not, and a
    // texture that counted the attribution as speech would over-report every line
    // of dialogue in the book by the length of its tag.
    assert_eq!(quoted.filled, 0.5, "got {quoted:?}");
    assert_eq!(
        narration.filled, 0.0,
        "and the narration as none: {narration:?}"
    );
    assert!(
        narration.extent > quoted.extent,
        "the long paragraph is the long bar: {narration:?} against {quoted:?}"
    );
    assert!(
        (narration.extent - 1.0).abs() < 1e-6,
        "measured against the longest paragraph in this document, which it is"
    );
    assert!(
        quoted.span.end <= narration.span.start + 1e-3,
        "and they are placed in document order, not stacked"
    );
}

/// A bar's length is a fraction of the column, so it cannot exceed it.
///
/// Not a hypothetical: a merged bar's word count is the sum of its members', and
/// scaling those against the longest *unmerged* paragraph put bars past the end of
/// the column they are drawn in. A short lane is what forces the merging, so that
/// is what this measures on.
#[test]
fn no_bar_is_longer_than_the_column_however_much_merging_it_takes() {
    let mut scene = String::new();
    for i in 0..40 {
        if i % 4 == 0 {
            scene.push_str("\u{201C}Then go,\u{201D} she said.\n\n");
        } else {
            scene.push_str(
                "The ferry left before the light did, and for a long while the only \
                 thing anyone on the upper deck could hear was the water going past.\n\n",
            );
        }
    }
    let doc = TextDocument::new();
    doc.set_plain_text(&scene).unwrap();
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let _id = tree.add(editor);
    tree.layout(SizeProposal::exact(320.0, 240.0));
    let _ = tree.render();

    let span_of = |start: usize, end: usize| {
        teksilo_ui::margin_lane::locate_span(&handle, LaneExtent::WHOLE, start, end)
    };
    let markers = skribisto_model::analysis::prose_stats::markers_for(
        "en",
        common::entities::QuoteStyle::LocaleDefault,
    );

    // Sixty pixels for forty paragraphs: every bar has to merge with its neighbours.
    let bars = teksilo_ui::margin_lane::texture::bars(&doc, markers, &span_of, 60.0);
    assert!(!bars.is_empty());
    assert!(
        bars.len() < 40,
        "a 60 px column cannot show 40 bars; it showed {}",
        bars.len()
    );
    for bar in &bars {
        assert!(
            (0.0..=1.0).contains(&bar.extent),
            "a bar longer than the column: {bar:?}"
        );
        assert!(
            (0.0..=1.0).contains(&bar.filled),
            "a fill outside its own bar: {bar:?}"
        );
    }
}

/// **The bars must not touch.**
///
/// This is the regression the mockup caught by eye. Bars were run to the *next*
/// paragraph's start rather than to their own end, so the column tiled with no gaps
/// at all — and the moment two neighbours were a similar length they fused into one
/// grey slab. That is precisely the "uniform grey at any scale" a minimap fails at,
/// and the shape down the page is the only thing the texture is for.
#[test]
fn consecutive_bars_are_separated_rather_than_tiled_into_one_mass() {
    let mut scene = String::new();
    for i in 0..12 {
        scene.push_str(&format!(
            "Paragraph {i}: the ferry left before the light did, and the water went past.\n\n"
        ));
    }
    let doc = TextDocument::new();
    doc.set_plain_text(&scene).unwrap();
    let editor = RichTextEditor::editor(doc.clone());
    let handle = editor.handle();
    let mut tree = WidgetTree::new();
    let _id = tree.add(editor);
    tree.layout(SizeProposal::exact(320.0, 240.0));
    let _ = tree.render();

    let span_of = |start: usize, end: usize| {
        teksilo_ui::margin_lane::locate_span(&handle, LaneExtent::WHOLE, start, end)
    };
    let markers = skribisto_model::analysis::prose_stats::markers_for(
        "en",
        common::entities::QuoteStyle::LocaleDefault,
    );

    let lane_height = 600.0;
    let bars = teksilo_ui::margin_lane::texture::bars(&doc, markers, &span_of, lane_height);
    assert!(
        bars.len() > 4,
        "twelve paragraphs on a tall lane must not all merge"
    );

    for pair in bars.windows(2) {
        let gap = (pair[1].span.start - pair[0].span.end) * lane_height;
        assert!(
            gap >= teksilo_ui::margin_lane::texture::BAR_GAP - 0.01,
            "bars {:?} and {:?} are {gap:.2} px apart; they have to be separated",
            pair[0].span,
            pair[1].span
        );
    }

    // And every bar still has height a reader can see. A gap taken out of a bar
    // that was already at the minimum would be a gap, not a bar.
    for bar in &bars {
        let h = (bar.span.end - bar.span.start) * lane_height;
        assert!(
            h >= teksilo_ui::margin_lane::texture::MIN_BAR_HEIGHT - 0.01,
            "a bar {h:.2} px tall is not a bar: {bar:?}"
        );
    }
}

/// **The scale must not be re-litigated by whichever rows happen to be on screen.**
///
/// The bug a writer reported, and the last one standing: scrolling a Full Book made
/// the whole texture column redraw, bars a mile from the scene they had reached
/// changing length on nothing but a scroll. Bars are scaled against the longest
/// paragraph among the rows that could be **placed**, and a row below the fold has
/// no geometry and contributes nothing — so that set, and the number derived from
/// it, wandered with the viewport.
///
/// Monotone fixes it without giving up what the scale is for: still the longest
/// paragraph in this manuscript, just no longer up for renegotiation.
#[test]
fn a_row_arriving_late_does_not_resize_the_bars_already_drawn() {
    use skribisto_model::analysis::prose_stats::ParagraphStats;
    use std::cell::Cell;
    use teksilo_ui::margin_lane::texture::{Paragraph, bars_from};

    let para = |start: f32, end: f32, words: usize| Paragraph {
        span: LaneSpan::new(start, end),
        stats: ParagraphStats { words, spoken: 0 },
    };

    let scale = Cell::new(0);
    // First pass: only the rows near the top are placed, none of them long.
    let first = bars_from(
        vec![para(0.0, 0.10, 100), para(0.15, 0.25, 80)],
        600.0,
        &scale,
    );

    // The writer scrolls; a row with a much longer paragraph arrives, and then one
    // with a much shorter one as the first rows are unbuilt behind them.
    let _ = bars_from(vec![para(0.4, 0.5, 400)], 600.0, &scale);
    let again = bars_from(
        vec![para(0.0, 0.10, 100), para(0.15, 0.25, 80)],
        600.0,
        &scale,
    );

    assert_eq!(
        first.len(),
        again.len(),
        "the same paragraphs must still be the same bars"
    );
    // Not equal to the *first* answer — the arrival of a genuinely longer paragraph
    // is real information and the scale grows once to admit it. What must not happen
    // is the scale coming back down when that row leaves, which is what made every
    // bar in the book move on every scroll.
    let shrunk = bars_from(vec![para(0.0, 0.10, 100)], 600.0, &scale);
    assert!(
        (shrunk[0].extent - again[0].extent).abs() < 1e-6,
        "a row leaving must not lengthen the bars that stayed: {} then {}",
        again[0].extent,
        shrunk[0].extent
    );
    assert!(
        again[0].extent < first[0].extent,
        "and a longer paragraph arriving does shorten them, once: {} then {}",
        first[0].extent,
        again[0].extent
    );
}

/// **A guess is not a measurement.**
///
/// A language with no curated dialogue convention is not a language with no
/// dialogue. Drawing every bar with an empty fill would state a zero, which is the
/// most misleading thing this could say about a manuscript it cannot read.
#[test]
fn an_unmeasurable_language_draws_no_texture_rather_than_an_empty_one() {
    let doc = TextDocument::new();
    doc.set_plain_text("Ĉu vi venos? Mi ne scias.").unwrap();
    let span_of = |_: usize, _: usize| Some(LaneSpan::new(0.0, 0.5));
    let markers = skribisto_model::analysis::prose_stats::markers_for(
        "eo",
        common::entities::QuoteStyle::LocaleDefault,
    );
    assert!(!markers.is_measurable(), "the premise of this test");
    assert!(teksilo_ui::margin_lane::texture::bars(&doc, markers, &span_of, 240.0).is_empty());
}

/// A `LaneContext` is what an extension writes against, so it has to be buildable
/// from outside this crate — every field public, no private constructor.
#[test]
fn an_extension_can_build_a_context_of_its_own() {
    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let doc = TextDocument::new();
    let anchors: Vec<CommentAnchor> = Vec::new();
    let locate = |_: usize| Some(0.25_f32);
    let ctx = LaneContext {
        kind: teksilo_ui::ext::EditorKind::Prose,
        app_ctx: &app_ctx,
        ids: &ids,
        surface: LaneSurface::Stream,
        doc: &doc,
        item_id: 7,
        comment_anchors: &anchors,
        misspellings: &[],
        color: Color::from_hex("#009E73"),
        group: 3,
        locate: &locate,
    };
    assert_eq!((ctx.locate)(0), Some(0.25));
    assert!(ctx.comment_anchors.is_empty());
}

/// **An id that would make its settings key both a value and a table is refused
/// here, rather than panicking a window open later.**
///
/// A provider's key is `editor.margin_lane.provider.<id>` and the settings file
/// is TOML, where a dotted key is a path into nested tables. `pro.structure` and
/// `pro.structure.beats` therefore ask the store for one name as a boolean and
/// as a table, and it refuses by panicking -- on the first pass that resolves a
/// lane, which is a writer opening a Full book view.
///
/// Nothing had to think about it while every id was a single word; the first
/// dotted one came from an extension. Refusing at registration puts the error on
/// the main thread, before `run`, beside the extension that chose the id.
#[test]
fn an_id_that_would_collide_with_anothers_settings_path_is_refused() {
    let mut deep = provider("collide.parent.child", &[LaneSurface::Editor]);
    deep.id = "collide.parent.child".to_string();
    let _held = register_lane_provider("test.collide.deep", deep).expect("the first one is fine");

    let mut shallow = provider("collide.parent", &[LaneSurface::Editor]);
    shallow.id = "collide.parent".to_string();
    let err = register_lane_provider("test.collide.shallow", shallow)
        .expect_err("a dotted prefix of a registered id must be refused");
    assert!(
        err.contains("collide.parent") && err.contains("collide.parent.child"),
        "the error must name both ids so the author can see the pair: {err}"
    );

    // A shared *byte* prefix is not a shared path: these two nest perfectly well
    // and must both be accepted, or the check is a rename ban.
    let mut sibling = provider("collide.parent.children", &[LaneSurface::Editor]);
    sibling.id = "collide.parent.children".to_string();
    let _also = register_lane_provider("test.collide.sibling", sibling)
        .expect("'child' and 'children' are different segments");
}

/// **A provider can tell a Book's prose from its synopsis cards.**
///
/// A stream is one surface with two flavours: `Full book` maps each scene's
/// prose, `Full synopsis` maps the same scenes' notes, and both arrive as
/// `LaneSurface::Stream`. The host knew which -- it is how it finds the editor
/// handle for a row -- and did not say, so a provider whose marks mean something
/// about the *prose* had no way to notice it was being asked about a two-line
/// note instead.
///
/// Most providers should not branch on it: a position that came from `locate` is
/// already right on both. It is here so that marking both can be a decision.
#[test]
fn a_provider_is_told_which_field_of_the_item_it_is_mapping() {
    use std::cell::RefCell;

    let seen: Rc<RefCell<Vec<teksilo_ui::ext::EditorKind>>> = Rc::new(RefCell::new(Vec::new()));
    let recorder = seen.clone();
    let mut spec = provider("flavour-probe", &[LaneSurface::Stream]);
    spec.marks = Rc::new(move |ctx| {
        recorder.borrow_mut().push(ctx.kind);
        Vec::new()
    });

    let app_ctx = std::rc::Rc::new(frontend::AppContext::new());
    let ids = teksilo_ui::app_ids::AppIds::new();
    let doc = TextDocument::new();
    let anchors: Vec<CommentAnchor> = Vec::new();
    let locate = |_: usize| Some(0.5_f32);

    for kind in [
        teksilo_ui::ext::EditorKind::Prose,
        teksilo_ui::ext::EditorKind::Synopsis,
    ] {
        let call = teksilo_ui::margin_lane::resolve::LaneCall {
            app_ctx: &app_ctx,
            ids: &ids,
            surface: LaneSurface::Stream,
            doc: &doc,
            item_id: 7,
            kind,
            comment_anchors: &anchors,
            misspellings: &[],
            locate: &locate,
        };
        let _ = call.run(&spec, Color::from_hex("#009E73"), 3);
    }

    assert_eq!(
        *seen.borrow(),
        vec![
            teksilo_ui::ext::EditorKind::Prose,
            teksilo_ui::ext::EditorKind::Synopsis
        ],
        "the flavour must reach the provider, and reach it unchanged"
    );
}
