// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use crate::app::wiring::punctuation::punctuation_flags;
use crate::settings::SettingsViewModel;
use crate::singles::SingleSmartPunctuation;

// ── Work ▸ New Window: may this window replace its project in place? ────
//
// The predicate every switch door consults. It is the whole protection for
// the shared `AppIds`: get it wrong in the permissive direction and File ▸
// Open Work from either of two windows on one Work deletes the backend
// subtree the other one is displaying.

/// A window alone on its project switches in place, exactly as before this
/// feature existed — the common case must not regress.
#[test]
fn a_window_alone_on_its_project_may_switch_in_place() {
    let registry = WorkRegistry::new();
    let ids = AppIds::new();
    registry.register(1, crate::sessions::WorkSession::for_test());
    ids.work_id.set(Some(1));

    assert!(may_switch_project_in_place(
        &registry,
        &ids,
        WindowRole::Owner
    ));
}

/// With no project open there is nothing to replace and nothing to protect.
#[test]
fn a_window_with_no_project_may_switch_in_place() {
    let registry = WorkRegistry::new();
    let ids = AppIds::new();
    assert!(may_switch_project_in_place(
        &registry,
        &ids,
        WindowRole::Owner
    ));
}

/// The live condition: a sibling window is showing this very Work, and the
/// two share one `AppIds`.
#[test]
fn a_window_sharing_its_work_with_a_sibling_may_not_switch_in_place() {
    let registry = WorkRegistry::new();
    let ids = AppIds::new();
    registry.register(1, crate::sessions::WorkSession::for_test());
    registry.attach(1).expect("a second window on Work 1");
    ids.work_id.set(Some(1));

    assert!(!may_switch_project_in_place(
        &registry,
        &ids,
        WindowRole::Owner
    ));
}

/// …and it becomes permitted again once that sibling closes: the reason was
/// the sharing, not a one-way door.
#[test]
fn the_last_window_left_on_a_work_may_switch_in_place_again() {
    let registry = WorkRegistry::new();
    let ids = AppIds::new();
    registry.register(1, crate::sessions::WorkSession::for_test());
    registry.attach(1).expect("a second window on Work 1");
    ids.work_id.set(Some(1));
    assert!(!may_switch_project_in_place(
        &registry,
        &ids,
        WindowRole::Owner
    ));

    registry.unregister(1); // the sibling closed
    assert!(may_switch_project_in_place(
        &registry,
        &ids,
        WindowRole::Owner
    ));
}

/// The durable condition, and the one that is easy to miss: a window opened
/// BY Work ▸ New Window may never switch in place, even once it is the only
/// window left on the Work. It does not own the project's saved desk — the
/// shared `workspace_layout` still holds the *original* window's
/// `DockingModel` and editors — so an in-place switch there would persist a
/// dead window's desk under the incoming project's key.
#[test]
fn an_attached_window_may_never_switch_in_place_even_when_left_alone() {
    let registry = WorkRegistry::new();
    let ids = AppIds::new();
    registry.register(1, crate::sessions::WorkSession::for_test());
    ids.work_id.set(Some(1));
    // No sibling at all: only `attached` stands in the way.
    assert_eq!(registry.window_count_for(1), 1);

    assert!(!may_switch_project_in_place(
        &registry,
        &ids,
        WindowRole::Attached
    ));
}

/// Work ▸ New Window carries the ordinal that decides both the window's
/// title suffix and its persistence id, so `PendingAction` has to surface
/// the Work it attaches to — and must not claim one for the actions that
/// have no `work_id` until the backend mints it.
#[test]
fn only_an_attaching_action_names_a_work_up_front() {
    let attach = PendingAction::AttachExisting {
        work_id: 7,
        path: "/tmp/x.skrib".into(),
        ordinal: 2,
    };
    assert_eq!(attach.attached_work_id(), Some(7));
    assert_eq!(attach.target_path(), "/tmp/x.skrib");

    let load = PendingAction::Load("/tmp/y.skrib".into());
    assert_eq!(load.attached_work_id(), None);
    assert_eq!(load.target_path(), "/tmp/y.skrib");
}

/// The two-tier punctuation resolution, which is the whole point of
/// `override_app_default` being a stored flag rather than an implied one.
///
/// Driven through real handles rather than a reimplementation of the rule:
/// a test that restated the `if` would pass no matter which way round it
/// was written.
#[test]
fn a_project_without_an_override_follows_the_application_preference() {
    let ctx = std::rc::Rc::new(frontend::AppContext::new());
    let sp = SingleSmartPunctuation::new(ctx);
    let app = settings_vm_for_test();

    // The app tier says dashes on, spacing off.
    app.punct_dashes().set(true);
    app.punct_spacing().set(false);
    app.punct_quote_style()
        .set(frontend::common::entities::QuoteStyle::Guillemets);

    // The project disagrees on every count — and must be ignored while its
    // override is off.
    sp.set_override_app_default(false);
    sp.set_dashes(false);
    sp.set_pre_punctuation_spacing(true);
    sp.set_quote_style(frontend::common::entities::QuoteStyle::LowHigh);

    let f = punctuation_flags(&sp, &app);
    assert!(f.dashes, "the app tier decides");
    assert!(!f.pre_punctuation_spacing);
    assert_eq!(
        f.quote_style,
        frontend::common::entities::QuoteStyle::Guillemets
    );
}

/// And with the override on, the project's own row wins outright — not
/// merged with the app tier. A house style is a whole system; a
/// half-inherited one belongs to no language at all.
#[test]
fn an_overriding_project_wins_outright() {
    let ctx = std::rc::Rc::new(frontend::AppContext::new());
    let sp = SingleSmartPunctuation::new(ctx);
    let app = settings_vm_for_test();

    app.punct_dashes().set(true);
    app.punct_ellipsis().set(true);

    sp.set_override_app_default(true);
    sp.set_dashes(false);
    sp.set_ellipsis(false);

    let f = punctuation_flags(&sp, &app);
    assert!(!f.dashes, "the project decides, even to switch a rule OFF");
    assert!(
        !f.ellipsis,
        "an app-level rule is not inherited through an override"
    );
}

/// A throwaway store, so these read the real signal plumbing rather than a
/// stand-in for it.
fn settings_vm_for_test() -> SettingsViewModel {
    use std::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "skribisto_punct_tier_{}_{n}.toml",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let store = teksilo::settings::SettingsStore::open(path).expect("open temp store");
    SettingsViewModel::new(&store)
}

/// The four states the Save affordances gate on. Nothing to save, or a
/// read-only backup window → disabled.
#[test]
fn can_save_only_when_dirty_and_not_in_backup_mode() {
    for (dirty, backup, want) in [
        (false, false, false), // clean project — nothing to write
        (true, false, true),   // the only case that saves
        (false, true, false),  // backup window, clean
        (true, true, false),   // backup window with edits: Save As / Restore, not Save
    ] {
        let unsaved = Signal::new(dirty);
        let backup_mode = Signal::new(backup);
        assert_eq!(
            can_save(&unsaved, &backup_mode).get(),
            want,
            "dirty={dirty} backup_mode={backup}"
        );
    }
}

/// The signal is *derived*, not sampled: flipping either input after the fact
/// must move it (otherwise the menu item would freeze at its build-time value).
#[test]
fn can_save_tracks_later_input_changes() {
    let unsaved = Signal::new(false);
    let backup_mode = Signal::new(false);
    let can = can_save(&unsaved, &backup_mode);
    assert!(!can.get());

    unsaved.set(true); // an edit lands
    assert!(can.get());

    backup_mode.set(true); // …in a backup window: still nothing Save can do
    assert!(!can.get());

    backup_mode.set(false); // Save As turned it back into a normal project
    assert!(can.get());

    unsaved.set(false); // saved to disk
    assert!(!can.get());
}

/// Phase 3 regression: `backup_mode` used to be one process-wide `Signal`
/// shared by every open Work (see `sessions::WorkSession`'s module doc's
/// "Phase 3 correction" section). Entering backup mode in Work A's window
/// must leave Work B's own `can_save` (hence its Save menu item, Ctrl+S,
/// and `editor.save`) fully enabled — this is the "backup-mode on Work A
/// leaves Work B writable" guarantee the migration exists to restore.
#[test]
fn backup_mode_on_one_work_never_disables_saving_on_another() {
    let session_a = crate::sessions::WorkSession::for_test();
    let session_b = crate::sessions::WorkSession::for_test();
    // Both Works have unsaved edits. `unsaved` is ALSO per-Work now (Scope
    // E's own fix, `WorkSession::unsaved`) — using plain local `Signal`s
    // here instead keeps this test isolated to what it actually names:
    // `backup_mode`.
    let unsaved_a = Signal::new(true);
    let unsaved_b = Signal::new(true);

    let can_save_a = can_save(&unsaved_a, &session_a.backup_mode);
    let can_save_b = can_save(&unsaved_b, &session_b.backup_mode);
    assert!(can_save_a.get(), "Work A starts writable");
    assert!(can_save_b.get(), "Work B starts writable");

    // Work A opens a backup file — enters backup mode.
    session_a.backup_mode.set(true);

    assert!(!can_save_a.get(), "Work A's Save must now be disabled");
    assert!(
        can_save_b.get(),
        "Work B must stay writable — a sibling Work's backup mode must never disable it"
    );
}

/// The shortcut and the action must *both* follow the signal — the keystroke
/// and the intent are two independent entry points into `save_to_disk`.
#[test]
fn save_shortcut_and_action_follow_can_save() {
    let unsaved = Signal::new(false);
    let backup_mode = Signal::new(false);
    let can = can_save(&unsaved, &backup_mode);

    let shortcut = Shortcut::new("editor.save")
        .name("Save")
        .primary(KeyStroke::ctrl(Key::S))
        .enabled_when(can.clone())
        .build();
    let action = Action::new("editor.save")
        .enabled_when(can)
        .on_invoke(|_i, _c| {});

    assert!(!shortcut.is_enabled(), "Ctrl+S inert on a clean project");
    assert!(!action.is_enabled(), "editor.save inert on a clean project");

    unsaved.set(true);
    assert!(shortcut.is_enabled());
    assert!(action.is_enabled());

    backup_mode.set(true);
    assert!(!shortcut.is_enabled(), "Ctrl+S inert in backup mode");
    assert!(!action.is_enabled(), "editor.save inert in backup mode");
}

/// F3: `build_window_teardown` — the closure `WorkRegistry::remove_window`
/// runs once teksilo's `on_removed` hook confirms a window is really
/// gone — must tell the toast registry to forget that window too, or
/// `window_audiences`/`window_versions` leak one entry per window ever
/// opened over the process's lifetime (see the function's own doc).
/// `set_window_audience(id, None)` alone is NOT the fix (it only clears
/// the signal's *value*, leaving the map entry) — this proves the actual
/// teardown path removes the entry, by observing the framework's own
/// documented post-`forget_window` behaviour: re-deriving the audience
/// signal for a forgotten window starts fresh (`None`), not merely
/// "whatever it was last set to".
#[test]
fn window_teardown_forgets_the_toast_registry_entry_too() {
    use teksilo::widgets::{ToastAudience, ToastInstallOptions};

    let app_ctx = Rc::new(frontend::AppContext::new());
    let session = crate::sessions::WorkSession::for_test();
    let editors = test_editors_view_model(&app_ctx);
    let window_id = TeksiloWindowId::new(1);

    let registry = ToastRegistry::new(ToastInstallOptions {
        archive: None,
        ..ToastInstallOptions::default()
    });
    registry.set_window_audience(window_id, Some(ToastAudience::new(42)));
    assert_eq!(
        registry.window_audience_signal(window_id).get(),
        Some(ToastAudience::new(42)),
        "sanity: the audience is really set before teardown runs"
    );

    let teardown = build_window_teardown(
        editors,
        session.backup_scheduler.clone(),
        Some(registry.clone()),
        window_id,
        None,
    );
    teardown();

    assert_eq!(
        registry.window_audience_signal(window_id).get(),
        None,
        "a forgotten window's audience must start fresh, not resurrect whatever \
             `set_window_audience` last wrote — proving the map entry (not just the \
             signal's value) was actually removed"
    );
}

/// A minimal `EditorsViewModel` for wiring-only tests that just need a real
/// instance to pass through `build_window_teardown` — mirrors
/// `editors.rs`'s own private test helper (not reachable from here), kept
/// deliberately small since nothing here exercises editor behaviour.
fn test_editors_view_model(app_ctx: &Rc<frontend::AppContext>) -> EditorsViewModel {
    let bundle = || crate::settings::EditorTypography {
        font_family: Signal::new("Literata".to_string()),
        size: Signal::new(1.0),
        line_height: Signal::new(1.5),
        first_line_indent: Signal::new(0.0),
        para_spacing_before: Signal::new(0.0),
        para_spacing_after: Signal::new(0.0),
        size_range: crate::settings::TypographySizeRange::default(),
    };
    let typography = crate::settings::EditorTypographySet {
        scene: bundle(),
        synopsis: bundle(),
        notes: bundle(),
        corkboard: bundle(),
        distraction_free: bundle(),
    };
    let ids = crate::app_ids::AppIds::new();
    let save_state = crate::save::SaveStateViewModel::new(app_ctx.clone(), ids.clone());
    EditorsViewModel::new(
        app_ctx.clone(),
        Signal::new(700.0),
        Signal::new(true),
        Signal::new(crate::shared::SynopsisPlacement::default()),
        Signal::new(crate::SYNOPSIS_SIDE_WIDTH_DEFAULT),
        typography,
        crate::shared::TypewriterSettings::off(),
        crate::shared::CaretHighlightSettings::off(),
        crate::settings::EditorViewMemory::detached(false),
        crate::settings::CorkboardDefaults::detached(),
        ids.clone(),
        crate::models::OpenDocsStore::new(app_ctx.clone()),
        Signal::new(false),
        save_state,
        Signal::new(false),
        crate::settings::TreeExpansionViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            crate::models::TreeExpansionService::in_memory_default(),
        ),
        Signal::new(false),
        Signal::new(620.0),
        crate::go::GoAvailability::new(),
        crate::format::FormatViewModel::detached(),
        crate::writing_session::WritingGamesViewModel::detached(),
        Signal::new(frontend::common::entities::GoalUnit::default()),
    )
}

// ── The dirty-marking whitelist ─────────────────────────────────────────

/// Every entity kind whose edits commit outside the manuscript editors must
/// be a dirty-marking event, or its edits are silently discarded by a
/// close-without-prompt. `Comment` was missing at first — a comment typed
/// into the margin card never showed the unsaved dot, never armed autosave,
/// and Close threw it away without asking. This pins each kind the whitelist
/// must carry so the next one added to the model fails a test instead of a
/// writer.
#[test]
fn every_non_editor_edit_surface_is_a_dirty_marking_event() {
    let origins = mutation_origins();
    use DirectAccessEntity::{Comment, CommentReply, Footnote};
    for ent in [
        Comment(EntityEvent::Created),
        Comment(EntityEvent::Updated),
        Comment(EntityEvent::Removed),
        CommentReply(EntityEvent::Created),
        CommentReply(EntityEvent::Updated),
        CommentReply(EntityEvent::Removed),
        Footnote(EntityEvent::Created),
        Footnote(EntityEvent::Updated),
        Footnote(EntityEvent::Removed),
    ] {
        assert!(
            origins.contains(&Origin::DirectAccess(ent.clone())),
            "{ent:?} must mark the work unsaved — its edits commit outside \
                 the editors' `edited` signal, so this event is the only trace"
        );
    }
}

/// The deliberate exclusion stays excluded: `Content` fires only on flush,
/// and a save's own flush marking the project dirty again is the debounce
/// loop the whitelist exists to avoid.
#[test]
fn content_events_stay_off_the_dirty_marking_whitelist() {
    for ev in [
        EntityEvent::Created,
        EntityEvent::Updated,
        EntityEvent::Removed,
    ] {
        assert!(
            !mutation_origins().contains(&Origin::DirectAccess(DirectAccessEntity::Content(ev))),
            "Content events fire on flush — whitelisting them loops autosave"
        );
    }
}
