// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::app_ids::AppIds;
use frontend::AppContext;
use teksilo::prelude::{Key, Modifiers, lit};
use teksilo::widgets::{MenuModel, MenuNode};

// The registry is process-wide and tests run in parallel, so no assertion
// here may depend on the *total* number registered: a sibling test's handle
// moves that under this one's feet. Each test uses its own namespace and its
// own intent strings, and counts are taken as before/after deltas.
//
// What is deliberately NOT covered here: that
// `register_all_extension_commands` really reaches the framework's global
// action table. `WidgetTree::dispatch_intent` is crate-private to teksilo, so
// firing an intent at a headless tree is not expressible from this crate.
// That link is covered downstream, by an extension's own live automation —
// press the shortcut, close the panel that would have owned the action under
// the old arrangement, press it again.

fn command(intent: &'static str) -> ExtensionCommand {
    ExtensionCommand {
        intent,
        build_action: Rc::new(move |_ctx, _cx| Action::new(intent).on_invoke(|_i, _c| {})),
        shortcut: None,
        menu_row: None,
    }
}

fn registered(intent: &str) -> bool {
    EXTENSION_COMMANDS.with(|r| r.borrow().iter().any(|r| r.command.intent == intent))
}

/// How many rows the Tools menu would gain, read back off a real model —
/// `MenuItems` keeps its node list private, and asking the model is closer to
/// what `build_project_menu` actually does anyway.
fn rows_in_menu() -> usize {
    let model = MenuModel::new().menu(lit!("Tools".to_string()), menu_rows);
    let nodes = model.nodes();
    match nodes.first() {
        Some(MenuNode::Submenu { children, .. }) => children.len(),
        _ => panic!("the fixture builds exactly one top-level menu"),
    }
}

#[test]
fn a_registered_command_joins_the_roster_and_drop_removes_it() {
    assert!(!registered("t1.do"));
    {
        let _h = register_command("test.cmd.basic", command("t1.do")).expect("register");
        assert!(registered("t1.do"));
    }
    assert!(
        !registered("t1.do"),
        "a dropped handle must leave nothing behind"
    );
}

/// Two actions on one intent do not fail visibly — whichever the framework
/// reaches first becomes the only one that ever runs. So a clash is refused,
/// and the message names the holder or it is unfixable.
#[test]
fn a_second_extension_cannot_take_a_taken_intent() {
    let _first = register_command("test.cmd.first", command("t2.do")).expect("register");
    let err = register_command("test.cmd.second", command("t2.do")).expect_err("must refuse");
    assert!(err.contains("test.cmd.first"), "unhelpful message: {err}");
}

/// The app's own command names are off limits: shadowing `editor.save` would
/// make Ctrl+S do something else, with nothing to see.
#[test]
fn the_apps_own_command_names_are_refused() {
    for taken in ["editor.save", "work.open", "app.quit", "go.next_scene"] {
        let err = register_command("test.cmd.shadow", command(taken))
            .expect_err("must refuse an app command name");
        assert!(err.contains("namespace yours"), "unhelpful message: {err}");
    }
    // …and a namespaced one goes through.
    let _ok = register_command("test.cmd.shadow", command("acme.save")).expect("register");
}

/// Re-registering one namespace replaces rather than stacks. Two copies would
/// each register an action on the same intent — the very collision the check
/// above exists to prevent, reached by another route.
#[test]
fn re_registration_replaces() {
    let _a = register_command("test.cmd.same", command("t3.old")).expect("first");
    assert!(registered("t3.old"));
    let _b = register_command("test.cmd.same", command("t3.new")).expect("re-register");
    assert!(registered("t3.new"));
    assert!(
        !registered("t3.old"),
        "the earlier entry must be gone, not stacked beside it"
    );
}

/// A command may be shortcut-only: no Tools row appears for it.
#[test]
fn only_commands_that_ask_for_a_row_get_one() {
    let before = rows_in_menu();
    let _quiet = register_command("test.cmd.quiet", command("t4.quiet")).expect("register");
    assert_eq!(rows_in_menu(), before, "no row was asked for");

    let mut loud = command("t4.loud");
    loud.menu_row = Some(MenuRow {
        label: Rc::new(|| lit!("Loud".to_string())),
        enabled: None,
    });
    let _loud = register_command("test.cmd.loud", loud).expect("register");
    assert_eq!(rows_in_menu(), before + 1);
    assert!(has_menu_rows());
}

/// Reading the roster must not mutate it: the menu is assembled once per
/// window, and two windows must get the same rows.
#[test]
fn assembling_the_rows_twice_yields_the_same_menu() {
    let mut cmd = command("t6.twice");
    cmd.menu_row = Some(MenuRow {
        label: Rc::new(|| lit!("Twice".to_string())),
        enabled: Some(Signal::new(true)),
    });
    let _h = register_command("test.cmd.twice", cmd).expect("register");
    let first = rows_in_menu();
    assert_eq!(
        rows_in_menu(),
        first,
        "a second window must get the same rows"
    );
}

/// A row carries its declaration through to the model: one item, built from
/// the label closure (so a runtime locale switch reaches it) rather than a
/// value captured at registration.
#[test]
fn the_label_is_resolved_per_build() {
    use std::cell::Cell;
    let calls = Rc::new(Cell::new(0usize));
    let c = calls.clone();
    let mut cmd = command("t5.go");
    cmd.shortcut = Some(ShortcutSpec {
        name: "t5.go",
        label: "Go",
        primary: KeyStroke::new(Key::F8, Modifiers::NONE),
    });
    cmd.menu_row = Some(MenuRow {
        label: Rc::new(move || {
            c.set(c.get() + 1);
            lit!("Go".to_string())
        }),
        enabled: None,
    });
    let _h = register_command("test.cmd.label", cmd).expect("register");

    let before = calls.get();
    let _ = rows_in_menu();
    assert_eq!(
        calls.get(),
        before + 1,
        "the label closure must run per build"
    );
    let _ = rows_in_menu();
    assert_eq!(calls.get(), before + 2);
}

/// **The list of the app's own namespaces is hand-written; the app's commands
/// are not.** This is what keeps them in step.
///
/// It shipped missing `numbering.`, so an extension could have registered
/// `numbering.tidy_titles` and silently taken over Binder ▸ Tidy titles —
/// with nothing to see, since a shadowed action simply stops being reached.
/// The module doc says a hand-copied enumeration would rot; that was true one
/// level up too, and this is the drift test that answers it. Same technique
/// and same reason as `settings_keys`' own.
#[test]
fn every_app_command_namespace_is_listed() {
    let declared = declared_app_intents();
    assert!(
        declared.len() > 40,
        "the source scan found only {} app intents — the scan itself is broken",
        declared.len()
    );

    let missing: Vec<&String> = declared.iter().filter(|i| !is_app_intent(i)).collect();
    assert!(
        missing.is_empty(),
        "these commands are registered by the application but their namespace is missing \
             from APP_INTENT_NAMESPACES:\n{}\nAdd each namespace (with its trailing dot), or an \
             extension can shadow the command and nothing will say so.",
        missing
            .iter()
            .map(|i| format!("  {i}"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

/// Every intent this crate registers as a global action or shortcut.
///
/// A directory walk rather than a fixed file list: the point is to catch a
/// command added in a file nobody thought to look in, and a hard-coded list is
/// blind to exactly that.
fn declared_app_intents() -> Vec<String> {
    use std::path::{Path, PathBuf};

    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    walk(&src, &mut files);

    let mut found: Vec<String> = Vec::new();
    for file in files {
        // This file's own doc comment and tests contain the needles as
        // literals, and its example intents are deliberately NOT the app's.
        if file.file_name().is_some_and(|n| n == "commands_ext.rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for needle in ["Action::new(\"", "Shortcut::new(\""] {
            for (index, _) in text.match_indices(needle) {
                let tail = &text[index + needle.len()..];
                let Some(close) = tail.find('"') else {
                    continue;
                };
                let intent = &tail[..close];
                // A dotted name is a command; anything else is not one of ours
                // to protect (and would make the guard refuse whole words).
                if intent.contains('.') {
                    found.push(intent.to_string());
                }
            }
        }
    }
    found.sort();
    found.dedup();
    found
}

/// A shortcut id is a **separate field** from the intent, so two commands
/// whose intents do not clash at all can still both name the same keystroke —
/// and the later registration silently wins it, which is precisely what this
/// module refuses to let happen for intents.
#[test]
fn a_second_extension_cannot_take_a_taken_shortcut_id() {
    let mut first = command("t7.alpha");
    first.shortcut = Some(ShortcutSpec {
        name: "shared.chord",
        label: "Alpha",
        primary: KeyStroke::new(Key::F8, Modifiers::NONE),
    });
    let _a = register_command("test.cmd.chord.a", first).expect("register");

    // A different, perfectly legal intent — colliding only on the chord id.
    let mut second = command("t7.beta");
    second.shortcut = Some(ShortcutSpec {
        name: "shared.chord",
        label: "Beta",
        primary: KeyStroke::new(Key::F9, Modifiers::NONE),
    });
    let err = register_command("test.cmd.chord.b", second).expect_err("must refuse");
    assert!(
        err.contains("test.cmd.chord.a"),
        "the error must name the holder, or the clash is unfixable: {err}"
    );
}

/// …and the app's own shortcut ids are off limits too. Reusing `editor.save`
/// would put an extension's verb on Ctrl+S.
#[test]
fn the_apps_own_shortcut_ids_are_refused() {
    let mut cmd = command("t8.shadow");
    cmd.shortcut = Some(ShortcutSpec {
        name: "editor.save",
        label: "Not save",
        primary: KeyStroke::new(Key::F8, Modifiers::NONE),
    });
    let err = register_command("test.cmd.chord.app", cmd).expect_err("must refuse");
    assert!(
        err.contains("application's own"),
        "unhelpful message: {err}"
    );
}

/// Re-registering one namespace must not be refused by its **own** earlier
/// shortcut id — the replace path has to clear the old entry first.
#[test]
fn re_registering_a_namespace_keeps_its_own_shortcut_id() {
    let mut cmd = command("t9.same");
    cmd.shortcut = Some(ShortcutSpec {
        name: "t9.same",
        label: "Same",
        primary: KeyStroke::new(Key::F8, Modifiers::NONE),
    });
    let _a = register_command("test.cmd.chord.same", cmd.clone()).expect("first");
    let _b = register_command("test.cmd.chord.same", cmd)
        .expect("a namespace must not collide with its own previous registration");
}

// ── Ownership: the whole reason this module exists ───────────────────────
//
// A global registration belongs to the widget whose `build()` made it and is
// torn down when that widget rebuilds or is destroyed. An extension has no
// always-mounted widget, so registering from its own panel gives a command
// that dies when the panel closes.
//
// The *action* table is private to teksilo (`WidgetTree::global_actions`, no
// public reader, and `dispatch_intent` is crate-private), so these assert on
// the **shortcut** half — which `register_all_extension_commands` registers
// through the identical `ctx.register_*_global` call, in the same closure,
// owned by the same `self_id()`. `ShortcutRegistry::owner_of` makes that
// ownership directly observable. The end-to-end firing is covered downstream
// by `scripts/automation_pro_command.py`.

/// Stands in for `App`: a stable root that registers the extension commands,
/// with a child panel that comes and goes.
struct FakeApp {
    cx: SeamContext,
    /// Whether the panel is mounted. Flipping it rebuilds this widget, which
    /// destroys the panel — what a dock closing does.
    show_panel: Signal<bool>,
}

impl std::fmt::Debug for FakeApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FakeApp").finish()
    }
}

impl teksilo::prelude::Widget for FakeApp {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<teksilo::prelude::WidgetId> {
        use teksilo::core::BindingLevel;
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.show_panel.bind_to(sid, reg, BindingLevel::Rebuild);

        register_all_extension_commands(ctx, &self.cx);

        if self.show_panel.get() {
            vec![ctx.add(Panel)]
        } else {
            vec![ctx.add(teksilo::widgets::Spacer::new())]
        }
    }
    fn layout_response(
        &self,
        proposal: teksilo::prelude::SizeProposal,
        _ctx: &teksilo::prelude::LayoutContext,
    ) -> teksilo::prelude::LayoutResponse {
        proposal.resolve(0.0, 0.0).into()
    }
}

/// A panel that registers a shortcut **of its own**, the way an extension
/// would have had to before `commands_ext` existed.
#[derive(Debug)]
struct Panel;

/// The id that panel claims. Its fate is the counterfactual these tests turn on.
const PANEL_OWNED: &str = "t.panel.owned";

impl teksilo::prelude::Widget for Panel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<teksilo::prelude::WidgetId> {
        ctx.register_shortcut_global(
            Shortcut::new(PANEL_OWNED)
                .name("Owned by the panel".to_string())
                .primary(KeyStroke::new(Key::F7, Modifiers::NONE))
                .build(),
        );
        vec![ctx.add(teksilo::widgets::Spacer::new())]
    }
    fn layout_response(
        &self,
        proposal: teksilo::prelude::SizeProposal,
        _ctx: &teksilo::prelude::LayoutContext,
    ) -> teksilo::prelude::LayoutResponse {
        proposal.resolve(0.0, 0.0).into()
    }
}

fn seam_context() -> SeamContext {
    let app_ctx = Rc::new(AppContext::new());
    crate::docks::DockContext {
        app_ctx: app_ctx.clone(),
        ids: AppIds::new(),
        work: crate::save::WorkHandle::detached(app_ctx.clone(), AppIds::new()),
        active: crate::active_context::ActiveContext::detached(),
        mention_index: crate::mentions::MentionIndex::new(app_ctx, AppIds::new()),
        // Nothing mounted in a test tree, so no row has live prose.
        live_prose: Rc::new(|_| None),
    }
}

fn with_shortcut(intent: &'static str) -> ExtensionCommand {
    let mut cmd = command(intent);
    cmd.shortcut = Some(ShortcutSpec {
        name: intent,
        label: "Test",
        primary: KeyStroke::new(Key::F8, Modifiers::NONE),
    });
    cmd
}

/// **The door's central claim, with its counterfactual.**
///
/// A global registration belongs to the widget whose `build()` made it and is
/// torn down when that widget is destroyed. So the same panel closing that
/// takes away a shortcut the *panel* registered must leave one registered
/// through `App` untouched — otherwise an extension's command would silently
/// stop working the moment its dock was closed, which is exactly what this
/// module exists to prevent.
///
/// Asserted on the **shortcut** half: teksilo keeps its global *action* table
/// private (no public reader, and `dispatch_intent` is crate-private), while
/// `register_all_extension_commands` registers both through the identical
/// `ctx.register_*_global` pair, in one closure, owned by the same
/// `self_id()`. `scripts/automation_pro_command.py` drives the action itself
/// against the running app.
#[test]
fn a_command_outlives_the_panel_that_a_panel_owned_one_does_not() {
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::prelude::SizeProposal;

    let _h = register_command("test.cmd.own", with_shortcut("t10.own")).expect("register");

    let show_panel = Signal::new(true);
    let mut tree = WidgetTree::new();
    let root = tree.add_boxed(Box::new(FakeApp {
        cx: seam_context(),
        show_panel: show_panel.clone(),
    }));
    tree.layout(SizeProposal::exact(400.0, 300.0));

    assert_eq!(
        tree.shortcut_registry().owner_of("t10.own"),
        Some(root),
        "the command must be owned by the root that registered it, not by a panel"
    );
    let panel_owner = tree.shortcut_registry().owner_of(PANEL_OWNED);
    assert!(
        panel_owner.is_some() && panel_owner != Some(root),
        "the fixture panel must really own its own shortcut, or the counterfactual proves nothing"
    );

    // The panel closes.
    show_panel.set(false);
    tree.layout(SizeProposal::exact(400.0, 300.0));

    assert_eq!(
        tree.shortcut_registry().owner_of(PANEL_OWNED),
        None,
        "a panel-owned registration must die with the panel — if this ever stops \
             holding, the reason commands_ext exists has gone away"
    );
    assert_eq!(
        tree.shortcut_registry().owner_of("t10.own"),
        Some(root),
        "…and the extension's command, registered through App, must survive it"
    );
    assert!(tree.shortcut_registry().get_default("t10.own").is_some());
}

/// A rebuild of the owning root re-registers exactly one — neither losing the
/// command nor stacking a second copy on top of it.
#[test]
fn rebuilding_the_root_re_registers_exactly_one() {
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::prelude::SizeProposal;

    let _h = register_command("test.cmd.rebuild", with_shortcut("t11.rebuild")).expect("register");

    let show_panel = Signal::new(false);
    let mut tree = WidgetTree::new();
    let root = tree.add_boxed(Box::new(FakeApp {
        cx: seam_context(),
        show_panel: show_panel.clone(),
    }));
    tree.layout(SizeProposal::exact(400.0, 300.0));
    let before = tree.shortcut_registry().len();
    assert_eq!(tree.shortcut_registry().owner_of("t11.rebuild"), Some(root));

    // Any rebuild of the root re-runs `register_all_extension_commands`.
    show_panel.set(true);
    tree.layout(SizeProposal::exact(400.0, 300.0));
    show_panel.set(false);
    tree.layout(SizeProposal::exact(400.0, 300.0));

    assert_eq!(
        tree.shortcut_registry().owner_of("t11.rebuild"),
        Some(root),
        "a rebuild dropped the command"
    );
    assert_eq!(
        tree.shortcut_registry().len(),
        before,
        "a rebuild registered a second copy of every command"
    );
}
