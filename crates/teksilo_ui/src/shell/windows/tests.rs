// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

/// Every window must name the **running edition**, not the community build.
///
/// Asserts all three branches — a Work with a title, one still loading, and a
/// second window on the same Work — because each resolves a different Fluent
/// key and a key whose arguments do not match its value resolves to an error
/// placeholder rather than failing to compile. Building the string is the only
/// thing that proves the arguments are actually wired.
#[test]
fn a_window_title_names_the_registered_edition() {
    let _serial = crate::identity::lock_for_test();
    let _h = crate::identity::register(
        crate::identity::AppIdentity::new("eu", "acme-writer", "Acme Writer")
            .with_display_name("Acme Writer"),
    );

    let work_title = Signal::new(String::new());
    let ordinal = Signal::new(1usize);
    let title = window_title_text(work_title.clone(), &ordinal);

    // Before a Work has loaded there is no title, so the window is just the app.
    assert!(
        title.get().contains("Acme Writer"),
        "an untitled window must still name the running edition, got {:?}",
        title.get()
    );
    assert!(
        !title.get().contains("Skribisto"),
        "an edition's window must not claim to be the community build, got {:?}",
        title.get()
    );

    work_title.set("The Lighthouse".to_string());
    let titled = title.get();
    assert!(
        titled.contains("The Lighthouse") && titled.contains("Acme Writer"),
        "a loaded Work must name both itself and the edition, got {titled:?}"
    );

    ordinal.set(2);
    let numbered = title.get();
    assert!(
        numbered.contains("The Lighthouse") && numbered.contains('2'),
        "a sibling window must stay distinguishable from the first, got {numbered:?}"
    );
    assert_ne!(
        numbered, titled,
        "two windows on one Work must not share a title — a KWin rule keyed on it \
             could not tell them apart"
    );
}

#[test]
fn window_id_for_a_missing_path_is_stable_and_not_shared() {
    // A New Work target that hasn't been written yet falls back to
    // hashing the raw string — still deterministic, still distinct from
    // the launcher's fixed id.
    let a = window_id_for("/tmp/skribisto-window-id-test-does-not-exist-a.skrib");
    let b = window_id_for("/tmp/skribisto-window-id-test-does-not-exist-a.skrib");
    let c = window_id_for("/tmp/skribisto-window-id-test-does-not-exist-b.skrib");
    assert_eq!(a, b, "the same path always yields the same id");
    assert_ne!(a, c, "different paths yield different ids");
    assert_ne!(a, LAUNCHER_WINDOW_ID);
    assert!(a.starts_with("work-"));
}

#[test]
fn window_id_for_collapses_different_spellings_of_one_path() {
    let dir = std::env::temp_dir().join(format!("sk-window-id-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("project.skrib");
    std::fs::write(&file, b"x").unwrap();

    let direct = window_id_for(file.to_str().unwrap());
    let via_dotdot = window_id_for(&format!(
        "{}/../{}/project.skrib",
        dir.to_str().unwrap(),
        dir.file_name().unwrap().to_str().unwrap()
    ));
    assert_eq!(
        direct, via_dotdot,
        "canonicalization must collapse a `..`-spelled path onto the same id"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// F4(a): the persisted id must not depend on `std::hash::DefaultHasher`
/// (whose algorithm std explicitly does not guarantee stable across Rust
/// releases) — swapped for blake3. This can't directly prove
/// cross-release stability (that would require pinning a specific
/// toolchain), but it does pin the two properties a caller actually
/// relies on: the id is deterministic for a given input in this process,
/// and its shape is the fixed `work-{16 hex chars}` this module's other
/// callers (and `window_state.toml`'s existing rows) expect.
#[test]
fn window_id_for_is_deterministic_and_16_hex_chars() {
    let path = "/tmp/skribisto-window-id-format-test-does-not-exist.skrib";
    let a = window_id_for(path);
    let b = window_id_for(path);
    assert_eq!(a, b, "hashing the same path twice must agree");

    let hex = a.strip_prefix("work-").expect("id must start with work-");
    assert_eq!(hex.len(), 16, "id must be work- + exactly 16 hex chars");
    assert!(
        hex.chars().all(|c| c.is_ascii_hexdigit()),
        "id suffix must be lowercase/uppercase hex, got {hex:?}"
    );
}

// ── Second windows on one project (Work ▸ New Window) ────────────────

/// The first window on a project must keep the id its geometry has always
/// been saved under — anything else silently orphans every existing
/// `window_state.toml` row the day this ships.
#[test]
fn the_first_window_on_a_project_keeps_the_plain_id() {
    let path = "/tmp/skribisto-attached-id-test-does-not-exist.skrib";
    assert_eq!(attached_window_id_for(path, 1), window_id_for(path));
    // Defensive: an ordinal of 0 is not reachable (they start at 1), but it
    // must degrade to the base id rather than producing `-w0`.
    assert_eq!(attached_window_id_for(path, 0), window_id_for(path));
}

/// Every further window is a distinct identity — distinct from the first
/// and from each other. teksilo's `WindowManager` overwrites its
/// `string_to_id` entry rather than rejecting a duplicate, so a collision
/// here would not fail loudly: the newer window would silently steal the
/// older one's identity, and `find_window` (hence `open_or_focus_project`,
/// the IPC raise path and the switcher) would resolve to the wrong one.
#[test]
fn every_further_window_on_one_project_gets_its_own_id() {
    let path = "/tmp/skribisto-attached-id-test-does-not-exist.skrib";
    let base = window_id_for(path);
    let second = attached_window_id_for(path, 2);
    let third = attached_window_id_for(path, 3);

    assert_eq!(second, format!("{base}-w2"));
    assert_ne!(
        second, base,
        "the second window must not claim the first's id"
    );
    assert_ne!(third, second, "two further windows must not share one id");
    assert!(
        second.starts_with(&base),
        "a second window's id must stay recognisably derived from its project's"
    );
}

/// Two different projects' second windows must not collide either — the
/// suffix disambiguates *within* a project, never across them.
#[test]
fn second_windows_of_different_projects_do_not_collide() {
    let a = attached_window_id_for("/tmp/skribisto-attached-a-does-not-exist.skrib", 2);
    let b = attached_window_id_for("/tmp/skribisto-attached-b-does-not-exist.skrib", 2);
    assert_ne!(a, b);
}

/// The id is a pure function of (project, ordinal): reopening a second
/// window on the same project at the same ordinal must land on the same
/// remembered geometry.
#[test]
fn an_attached_window_id_is_deterministic() {
    let path = "/tmp/skribisto-attached-id-test-does-not-exist.skrib";
    assert_eq!(
        attached_window_id_for(path, 2),
        attached_window_id_for(path, 2)
    );
}

// ── `attached_window_config` (the Work ▸ New Window factory path) ──────

/// A factory over a caller-supplied registry, so a test can register a Work
/// and then ask for a second window on it. (`welcome::welcome_vm`'s own
/// helper builds its registry internally, which is fine there and useless
/// here.)
fn test_factory(app_ctx: Rc<AppContext>, registry: WorkRegistry) -> ProjectWindowFactory {
    use crate::models::{BackupSettingsService, TreeExpansionService, WorkspaceLayoutService};
    use crate::spellcheck::SpellcheckService;
    ProjectWindowFactory::new(
        app_ctx,
        registry,
        SpellcheckService::new(),
        BackupSettingsViewModel::new(BackupSettingsService::in_memory_default()),
        WorkspaceLayoutService::in_memory_default(),
        TreeExpansionService::in_memory_default(),
        Signal::new(false),
        Signal::new(true),
        Signal::new(true),
    )
}

/// The race the menu item cannot rule out on its own: the Work was closed
/// between the menu opening and the click. No window, rather than a window
/// onto nothing — and, critically, rather than falling back to *loading the
/// file again*, which would give two independent `Work`s for one project.
#[test]
fn attaching_to_a_work_that_is_not_open_yields_no_window() {
    let app_ctx = Rc::new(AppContext::new());
    let registry = WorkRegistry::new();
    let factory = test_factory(app_ctx, registry);

    assert!(
        factory
            .attached_window_config(404, "/tmp/skribisto-attach-test.skrib")
            .is_none()
    );
}

/// The heart of the feature: a second window on an open Work shares that
/// Work's live session — one `AppIds`, one set of singles, one undo stack —
/// rather than minting a second one, and takes its own identity (ordinal,
/// string id) so the two windows never collide in teksilo's window map.
#[test]
fn a_second_window_shares_the_works_session_and_takes_its_own_identity() {
    let app_ctx = Rc::new(AppContext::new());
    let registry = WorkRegistry::new();
    let session = crate::sessions::WorkSession::for_test();
    session.ids.work_id.set(Some(1));
    registry.register(1, session.clone());
    // The Work's first window, exactly as its own `LoadWork` subscriber
    // binds it — the state Work ▸ New Window is always invoked from.
    registry.register_window(
        teksilo::prelude::TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        Rc::new(|| {}),
    );
    let factory = test_factory(app_ctx, registry.clone());

    let path = "/tmp/skribisto-attach-test.skrib";
    let (config, state) = factory
        .attached_window_config(1, path)
        .expect("the Work is open, so a second window on it must be buildable");

    assert_eq!(
        config.string_id.as_deref(),
        Some(attached_window_id_for(path, 2).as_str()),
        "the second window must carry its own persistence id, never the first's"
    );
    // Shared, not copied: a write through the registered session is visible
    // through the new window's — they are one object.
    session.ids.work_info_id.set(Some(99));
    assert_eq!(
        state.session.ids.work_info_id.get(),
        Some(99),
        "the attached window must share the Work's live session, not a fresh one"
    );
    assert_eq!(
        registry.window_count_for(1),
        2,
        "attaching must take a reference the new window's close will drop"
    );
}

/// Each further window gets its own ordinal and its own id — the mechanism
/// is not limited to a second window, and none of them may collide.
#[test]
fn a_third_window_does_not_reuse_the_seconds_identity() {
    let app_ctx = Rc::new(AppContext::new());
    let registry = WorkRegistry::new();
    registry.register(1, crate::sessions::WorkSession::for_test());
    registry.register_window(
        teksilo::prelude::TeksiloWindowId::new(1),
        1,
        None,
        Rc::new(|_| {}),
        Rc::new(|| {}),
    );
    let factory = test_factory(app_ctx, registry.clone());

    let path = "/tmp/skribisto-attach-test.skrib";
    let (second, _) = factory
        .attached_window_config(1, path)
        .expect("second window");
    let (third, _) = factory
        .attached_window_config(1, path)
        .expect("third window");

    assert_eq!(
        second.string_id.as_deref(),
        Some(attached_window_id_for(path, 2).as_str())
    );
    assert_eq!(
        third.string_id.as_deref(),
        Some(attached_window_id_for(path, 3).as_str())
    );
    assert_ne!(second.string_id, third.string_id);
    assert_eq!(registry.window_count_for(1), 3);
}

/// A window that *loads* a project keeps the plain id its geometry has
/// always been saved under — the suffix is only ever an addition.
#[test]
fn a_loading_window_keeps_the_plain_project_id() {
    let app_ctx = Rc::new(AppContext::new());
    let factory = test_factory(app_ctx, WorkRegistry::new());

    let path = "/tmp/skribisto-attach-test.skrib";
    let (config, _) = factory.window_config(PendingAction::Load(path.to_string()));

    assert_eq!(
        config.string_id.as_deref(),
        Some(window_id_for(path).as_str())
    );
}

// ── Menu mnemonics ────────────────────────────────────────────────────
//
// A mnemonic must be unique *within* one keyboard namespace, and each open
// menu is its own namespace — `Alt+F` opens File, and `F` may then address
// an item inside it without ambiguity. So the check is per-scope, not
// global, and the same letter may be reused freely across scopes.
//
// The menu bar itself is the scope that bites hardest: `MenuBar::build`
// fills a `HashMap<char, usize>` behind a `debug_assert!`, so a duplicate
// there is a debug-build panic, and in release the later entry silently
// wins and the earlier menu becomes unreachable by keyboard. That is not
// hypothetical — fr-FR shipped `F&ormat` against `&Outils` (both `O`)
// until this test was written.
//
// Translators pick mnemonics per language, so a locale that reads clean in
// English can collide in French. Every supported locale is checked.

/// Menu scopes, mirroring the `MenuModel` built in
/// [`ProjectWindowFactory::window_config`]. Add an entry to that menu, add its
/// key here — an unlisted key is simply unchecked, which is the one
/// failure mode this table has.
const MENU_MNEMONIC_SCOPES: &[(&str, &[&str])] = &[
    (
        "menu bar",
        &[
            "menu-work",
            "menu-view",
            "menu-format",
            "menu-go",
            "menu-tools",
            "menu-help",
        ],
    ),
    (
        "Work",
        &[
            "menu-new-work",
            "menu-open-work",
            "menu-new-window",
            "menu-import-from",
            "menu-export",
            "menu-save",
            "menu-save-as-file",
            "menu-save-as-folder",
            "menu-backup",
            "menu-backups-list",
            "menu-close-work",
            "menu-welcome",
            "menu-settings",
            "menu-quit",
        ],
    ),
    ("Work > Import from", &["menu-import-plume"]),
    // The export scopes are labelled from `ExportScopeKind` at runtime and
    // deliberately carry no mnemonics; they are listed so the table stays a
    // complete picture of the menu, and the uniqueness check skips them.
    (
        "Work > Export",
        &[
            "menu-export-book",
            "menu-export-part",
            "menu-export-chapter",
            "menu-export-scene",
            "menu-export-note",
            "menu-export-folder",
            "menu-export-choose",
            "menu-export-none",
        ],
    ),
    (
        "View",
        &[
            "menu-outline",
            "menu-search",
            "menu-trash",
            "menu-search-preview",
            "menu-fullscreen",
            "menu-focus-mode",
        ],
    ),
    (
        "Format",
        &[
            "menu-format-marks-bold",
            "menu-format-marks-italic",
            "menu-format-marks-underline",
            "menu-format-marks-strike",
            "menu-format-marks-superscript",
            "menu-format-marks-subscript",
            "menu-format-marks-clear",
            "menu-format-heading",
            "menu-format-alignment",
            "menu-format-blockquote",
            "menu-format-lists",
            "menu-format-table",
            "menu-format-undo",
            "menu-format-redo",
            "menu-scene-break",
            "menu-major-scene-break",
        ],
    ),
    (
        "Format > Heading",
        &[
            "menu-format-heading-normal",
            "menu-format-heading-1",
            "menu-format-heading-2",
            "menu-format-heading-3",
            "menu-format-heading-4",
            "menu-format-heading-5",
            "menu-format-heading-6",
        ],
    ),
    (
        "Format > Alignment",
        &["menu-format-align-left", "menu-format-align-center"],
    ),
    (
        "Format > Lists",
        &[
            "menu-format-list-bullet",
            "menu-format-list-numbered",
            "menu-format-indent",
            "menu-format-outdent",
        ],
    ),
    (
        "Format > Table",
        &[
            "menu-format-table-insert",
            "menu-format-table-row-above",
            "menu-format-table-row-below",
            "menu-format-table-col-before",
            "menu-format-table-col-after",
            "menu-format-table-row-delete",
            "menu-format-table-col-delete",
            "menu-format-table-remove",
        ],
    ),
    (
        "Format > Table > Insert",
        &[
            "menu-format-table-2x2",
            "menu-format-table-3x3",
            "menu-format-table-4x4",
        ],
    ),
    (
        "Go",
        &[
            "menu-go-next-scene",
            "menu-go-prev-scene",
            "menu-go-next-chapter",
            "menu-go-prev-chapter",
            "menu-go-next-note",
            "menu-go-prev-note",
        ],
    ),
    ("Tools", &["menu-spellcheck"]),
    ("Help", &["menu-about"]),
];

/// Every locale whose menu labels carry mnemonics, as the `.ftl` source.
/// Mirrors the `compile_in` list in `main.rs`.
const MENU_LOCALES: &[(&str, &str)] = &[
    ("en-US", include_str!("../../../locales/en-US/main.ftl")),
    ("fr-FR", include_str!("../../../locales/fr-FR/main.ftl")),
];

/// The mnemonic a label declares, lower-cased to match `MenuBar`'s own
/// `key_lower` table. `&&` is an escaped literal ampersand and is skipped,
/// per the convention documented at the top of each `.ftl`.
fn mnemonic_of(label: &str) -> Option<char> {
    let mut chars = label.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '&' {
            continue;
        }
        match chars.peek() {
            Some('&') => {
                chars.next();
            }
            Some(&marked) => return marked.to_lowercase().next().or(Some(marked)),
            None => return None,
        }
    }
    None
}

/// `key = value` pairs from one `.ftl`. Continuation lines are indented and
/// comments start with `#`, so both are skipped; menu labels are always
/// single-line, which is all this needs to see.
fn ftl_labels(ftl: &str) -> std::collections::HashMap<&str, &str> {
    ftl.lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with(char::is_whitespace))
        .filter_map(|line| line.split_once(" = "))
        .map(|(key, value)| (key.trim(), value.trim()))
        .collect()
}

#[test]
fn menu_mnemonics_are_unique_within_every_scope_and_locale() {
    let mut collisions = Vec::new();

    for (locale, ftl) in MENU_LOCALES {
        let labels = ftl_labels(ftl);

        for (scope, keys) in MENU_MNEMONIC_SCOPES {
            let mut claimed: std::collections::HashMap<char, &str> =
                std::collections::HashMap::new();

            for key in *keys {
                let label = labels.get(key).unwrap_or_else(|| {
                    panic!(
                        "{locale}: `{key}` is listed in MENU_MNEMONIC_SCOPES \
                             but absent from main.ftl"
                    )
                });
                // No mnemonic is legitimate (see the export scopes above).
                let Some(mnemonic) = mnemonic_of(label) else {
                    continue;
                };
                if let Some(previous) = claimed.insert(mnemonic, key) {
                    collisions.push(format!(
                        "{locale} / {scope}: '{mnemonic}' is claimed by both \
                             `{previous}` and `{key}`"
                    ));
                }
            }
        }
    }

    assert!(
        collisions.is_empty(),
        "menu mnemonics must be unique within a scope:\n  {}",
        collisions.join("\n  ")
    );
}

#[test]
fn every_menu_bar_entry_declares_a_mnemonic() {
    // A top-level menu with no mnemonic is unreachable by `Alt+letter`,
    // which for the Format menu is the whole keyboard path to the
    // formatting commands.
    let bar = MENU_MNEMONIC_SCOPES
        .iter()
        .find(|(scope, _)| *scope == "menu bar")
        .expect("the menu-bar scope is listed")
        .1;

    for (locale, ftl) in MENU_LOCALES {
        let labels = ftl_labels(ftl);
        for key in bar {
            let label = labels[key];
            assert!(
                mnemonic_of(label).is_some(),
                "{locale}: menu-bar entry `{key}` = {label:?} declares no mnemonic"
            );
        }
    }
}

/// The labels of the platform-standard (macOS) menus, which are declared in
/// `project_menus::{app_standard_menu, window_standard_menu}`.
const NATIVE_MENU_LABELS: &[&str] = &[
    "native-menu-about",
    "native-menu-hide",
    "native-menu-quit",
    "native-menu-window",
    "native-menu-minimize",
    "native-menu-zoom",
];

/// macOS has no `Alt`+letter mnemonics, and the native bridge resolves a
/// standard menu's labels **without** stripping one — unlike every other menu
/// label, which goes through `parse_mnemonic` on its way into the snapshot. So
/// an `&` copied from a neighbouring key here does not quietly do nothing: it
/// prints, and "Quit &Skribisto" ships to the one platform this whole path
/// exists for. Nothing on Linux would ever show it.
#[test]
fn the_native_menu_labels_carry_no_mnemonic() {
    for (locale, ftl) in MENU_LOCALES {
        let labels = ftl_labels(ftl);
        for key in NATIVE_MENU_LABELS {
            let label = labels.get(key).unwrap_or_else(|| {
                panic!("{locale}: `{key}` is a native menu label but is absent from main.ftl")
            });
            assert!(
                mnemonic_of(label).is_none(),
                "{locale}: native menu label `{key}` = {label:?} must not declare a mnemonic"
            );
        }
    }
}

/// The three App-menu labels name the running edition, and the name is data —
/// it arrives as `{ $app }` rather than being written into the value, the same
/// rule the window titles follow. A locale that spells "Skribisto" out loses
/// the name of any other edition built on this tree.
#[test]
fn the_app_menu_labels_take_the_application_name_as_an_argument() {
    for (locale, ftl) in MENU_LOCALES {
        let labels = ftl_labels(ftl);
        for key in ["native-menu-about", "native-menu-hide", "native-menu-quit"] {
            let label = labels[key];
            assert!(
                label.contains("{ $app }"),
                "{locale}: `{key}` = {label:?} must name the app through the argument"
            );
        }
    }
}

#[test]
fn mnemonic_of_reads_markers_and_skips_escaped_ampersands() {
    assert_eq!(mnemonic_of("&Fichier"), Some('f'));
    assert_eq!(mnemonic_of("Fo&rmat"), Some('r'));
    assert_eq!(mnemonic_of("E&xporter"), Some('x'));
    // An escaped ampersand is literal text, not a marker.
    assert_eq!(mnemonic_of("Search && Replace"), None);
    assert_eq!(mnemonic_of("Search && &Replace"), Some('r'));
    assert_eq!(mnemonic_of("no marker here"), None);
    // A trailing lone '&' marks nothing.
    assert_eq!(mnemonic_of("dangling &"), None);
}
