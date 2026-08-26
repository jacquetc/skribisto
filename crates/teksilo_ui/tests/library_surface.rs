// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The crate is reachable from outside itself.
//!
//! An integration test compiles as its **own crate** linked against `teksilo_ui`,
//! which is what makes this a real check rather than a restatement of the unit
//! tests: everything named below has to be reachable the way a downstream
//! extension crate would reach it. While this was a `[[bin]]`-only target none of
//! it was — not one type — and that is the single fact that blocked the whole UI
//! half of the extension seam.
//!
//! If someone reverts the split, or turns a `pub mod` back into `mod`, this file
//! stops compiling. That is the entire point; there is deliberately very little
//! runtime assertion here.

/// The entry point the binary is now a one-line wrapper around.
#[test]
fn run_is_callable_from_outside_the_crate() {
    // Not invoked — it opens windows and elects a single instance. Taking its
    // address is what proves it is public and correctly typed.
    let entry: fn() = teksilo_ui::run;
    assert!(
        !std::ptr::eq(entry as *const (), std::ptr::null()),
        "run() must be a real, externally-callable entry point"
    );
}

/// The modules an extension has to reach to contribute anything at all.
///
/// Named individually rather than with a glob so that losing one is a compile
/// error naming *that* module, instead of a silent narrowing nobody notices.
#[test]
fn the_extension_facing_modules_are_public() {
    // Docks: the roster and the stable ids an extension dock must not collide with.
    let _: u64 = teksilo_ui::docks::OUTLINE_DOCK_ID;
    let roster_len = teksilo_ui::docks::APP_DOCKS.len();
    assert!(roster_len > 0, "the app must declare at least one dock");

    // The rest of the seam's surface, reached as a downstream crate would.
    #[allow(unused_imports)]
    use teksilo_ui::{
        app, app_ids, docks, editors, export, icons, intents, models, panels, sessions, settings,
        settings_keys, shell, singles, statusbar, tabs, tags, widgets,
    };
}

/// **A whole lane provider, built naming nothing but `ext`.**
///
/// `ext`'s own drift test walks `pub fn register…` declarations and asserts each
/// is re-exported. A **type** carried through one of those signatures is
/// invisible to it — which is how `WorkHandle` went a release without being
/// reachable, and how four of the margin lane's own types did: a spec is
/// declared with a `LaneColumn` and a `LaneShape`, and its closure returns
/// `LaneMark`s built on `LaneSpan`s. Ten of the lane's fourteen names were
/// exported and the four that a provider cannot be *written* without were not.
///
/// So this is the check the walk cannot be: a downstream edition's registration,
/// compiled from outside the crate, importing `ext` and nothing else. It asserts
/// almost nothing at runtime on purpose. If a type leaves `ext`, this file stops
/// compiling, and the error names it.
#[test]
fn an_extension_can_build_a_lane_provider_naming_only_ext() {
    use std::rc::Rc;
    use teksilo_ui::ext::{
        LaneColumn, LaneMark, LaneProviderSpec, LaneRefresh, LaneShape, LaneSpan, LaneSurface,
        register_lane_provider,
    };

    let spec = LaneProviderSpec {
        id: "library-surface-probe".to_string(),
        label: Rc::new(|| teksilo::prelude::lit!("Probe")),
        hint: Rc::new(|| teksilo::prelude::lit!("What the probe marks")),
        column: LaneColumn::Right,
        shape: LaneShape::Diamond,
        palette_slot: 2,
        surfaces: &[LaneSurface::Stream],
        default_on: false,
        refresh: LaneRefresh::Manual,
        marks: Rc::new(|ctx| {
            let Some(at) = (ctx.locate)(0) else {
                return Vec::new();
            };
            vec![LaneMark {
                id: ctx.item_id,
                span: LaneSpan::at(at),
                column: LaneColumn::Right,
                shape: LaneShape::Diamond,
                color: ctx.color,
                label: teksilo::prelude::lit!("a probe"),
                group: ctx.group,
            }]
        }),
    };

    let handle = register_lane_provider("test.library.surface", spec).expect("register");
    // The handle's type is nameable too — an edition has to store it in a struct
    // field for the life of the process, and one it cannot spell it cannot keep.
    let _kept: teksilo_ui::ext::LaneProviderHandle = handle;
}

/// **A chart sized the way this crate's own two charts are sized.**
///
/// `ext`'s drift test walks `pub fn register…` declarations (`src/ext/tests.rs`), and a
/// plain function or a `const` never matches that prefix, so renaming or reshaping any of
/// these five would not fail there. It fails here instead, compiled from outside the crate
/// against nothing but `ext`, which is the same check that already guards the margin
/// lane's types above.
///
/// The point of the guarantee is in `tabs::shared::charts`'s own module doc: two charts of
/// one manuscript that size themselves differently make one book look like two shapes. A
/// third chart, wherever it is built, has to be able to reach the same formula.
#[test]
fn an_extension_can_size_a_chart_naming_only_ext() {
    use teksilo_ui::ext::{BAR_PITCH, CHART_HEIGHT, STRIP_HEIGHT, content_width, wide_chart};

    let n = 12;
    assert!(
        content_width(n) >= n as f32 * BAR_PITCH,
        "every datum keeps its full pitch, however long the series is"
    );
    // A compile-time check rather than a runtime one: both sides are constants, so an
    // `assert!` here folds to a literal and clippy is right to say so.
    const _: () = assert!(CHART_HEIGHT > STRIP_HEIGHT);

    // Building one is the real proof: `wide_chart` has to take a real widget and hand one
    // back, naming nothing this module has not exported.
    let probe = || teksilo::widgets::TextWidget::new(teksilo::prelude::lit!("probe"));
    let _primary = wide_chart(n, CHART_HEIGHT, probe());
    let _strip = wide_chart(n, STRIP_HEIGHT, probe());
}

/// **A dock can be placed naming only `ext`.**
///
/// `ExtensionDock::placement` is an `AppDock` and its id has to fall inside a band the
/// registry enforces at runtime, so an extension that cannot name those three from the
/// façade has to reach past it for the one field of the registration it cannot avoid.
#[test]
fn an_extension_can_place_a_dock_naming_only_ext() {
    use teksilo_ui::ext::{APP_DOCK_ID_CEILING, AppDock, EXTENSION_DOCK_ID_FLOOR};

    const ID: u64 = EXTENSION_DOCK_ID_FLOOR + 1;
    // The check an extension wants to make at compile time rather than at first launch.
    const _: () = assert!(ID > EXTENSION_DOCK_ID_FLOOR);
    const _: () = assert!(EXTENSION_DOCK_ID_FLOOR > APP_DOCK_ID_CEILING);

    let placement = AppDock {
        id: ID,
        side: teksilo::widgets::DockSide::Trailing,
        own_tab: true,
    };
    assert_eq!(placement.id, ID);
}

/// **An extension can read what the writer is typing, naming only `ext`.**
///
/// The ordinary read commands answer with the stored text, and typing does not reach the
/// store until a flush. A surface reporting on the focused scene that reads the store shows
/// the text as of the last save, with nothing on screen saying so.
///
/// The capability arrives on `DockContext`, not through `app_state`: it is per window, like
/// `active`, and `app_state` is write-once at builder time besides.
#[test]
fn an_extension_can_read_a_rows_live_prose_naming_only_ext() {
    use std::rc::Rc;
    use teksilo_ui::ext::{LiveProse, LiveProseFn};

    // What an extension holds: the reader off its own `DockContext`, and the two types it
    // needs to name to use the result.
    let reader: LiveProseFn = Rc::new(|item| {
        (item == 7).then(|| LiveProse {
            text: "the ferry, as it stands right now".to_string(),
            version: teksilo::prelude::Signal::new(3),
        })
    });

    let live = reader(7).expect("the row this stand-in knows about");
    assert_eq!(live.text, "the ferry, as it stands right now");
    assert_eq!(
        live.version.get(),
        3,
        "the counter has to be readable and bindable, or the reader gets one snapshot \
         and never hears about the next keystroke"
    );
    assert!(
        reader(8).is_none(),
        "a row with no mounted editor is None, never an empty string a caller would \
         render as an empty scene"
    );
}

/// **The mention index, named and its rows destructured, naming only `ext`.**
///
/// `MentionIndex` is not behind a `register_…` slot at all; it is handed out
/// through `ctx.app_state::<MentionIndex>()`, the same way `WorkHandle` is
/// reached through a context type. That makes it invisible to `ext`'s drift
/// walk twice over: the walk only ever sees `pub fn register…` declarations,
/// and this type is not the payload of one of those either, it is a bare `pub
/// use`. Nothing about it would trip the walk if it were ever quietly dropped.
///
/// There is no `AppContext`/`AppIds` reachable through `ext` to build a real
/// `MentionIndex` with, which is deliberate: the seam hands out the live
/// instance itself through `app_state`, never the means to construct a second
/// one (the same reasoning `ext.rs` gives for never capturing an `AppContext`
/// at registration). So this proves what a downstream edition actually needs
/// from outside the crate: that the type can be named as a parameter a reading
/// function takes, and that `MentionRow`, what such a function hands back, can
/// be built and pattern-matched field by field. If a field were renamed or
/// hidden, this stops compiling and the error names it.
#[test]
fn an_extension_can_name_the_mention_index_and_destructure_its_rows() {
    use teksilo_ui::ext::{MentionIndex, MentionRow};

    // A downstream reading function, shaped like `story_bible/read.rs` would
    // write it: takes whatever `ctx.app_state::<MentionIndex>()` handed the
    // window, hands back rows. Compiling this is the assertion; it is never
    // called with a real index, because building one needs types this seam
    // deliberately does not publish.
    fn backlink_titles(index: &MentionIndex, item_id: u64) -> Vec<String> {
        index
            .backlinks_for(item_id)
            .into_iter()
            .map(|row| row.title)
            .collect()
    }
    let _typed: fn(&MentionIndex, u64) -> Vec<String> = backlink_titles;

    let row = MentionRow {
        owner_id: 10,
        target_id: 20,
        title: "Elena".to_string(),
        matched_names: vec!["Elena".to_string()],
        is_title_match: true,
        hit_count: 3,
        is_confirmed: false,
        is_point_of_view: true,
        evidence: "Elena walked onto the dock.".to_string(),
    };
    let MentionRow {
        owner_id,
        target_id,
        title,
        matched_names,
        is_title_match,
        hit_count,
        is_confirmed,
        is_point_of_view,
        evidence,
    } = row;
    assert_eq!(owner_id, 10);
    assert_eq!(target_id, 20);
    assert_eq!(title, "Elena");
    assert_eq!(matched_names, vec!["Elena".to_string()]);
    assert!(is_title_match);
    assert_eq!(hit_count, 3);
    assert!(!is_confirmed);
    assert!(is_point_of_view);
    assert_eq!(evidence, "Elena walked onto the dock.");
}
