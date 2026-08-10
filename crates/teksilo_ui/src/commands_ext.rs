// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! App-lifetime commands: how an extension adds a verb to the application —
//! an action, a global shortcut, and a row in the Tools menu.
//!
//! ## Why an extension cannot just call `register_action_global`
//!
//! It can — but ownership makes it useless.
//! `BuildContext::register_action_global` ties the action to the **calling
//! widget**: teksilo's own doc says "the action is torn down when this widget
//! rebuilds or is destroyed", and `WidgetTree` really does
//! `global_actions.retain(|(owner, _)| *owner != widget_id)` on both. An
//! extension has no always-mounted widget — its dock can be closed, its segment
//! is only built when its container tab is open — so a command it registered
//! from inside its own panel dies the moment that panel does. The keyboard
//! shortcut then silently stops working, which reads as "the extension is
//! broken" rather than "the panel is closed".
//!
//! Nothing special-cases `App`, though: it *is* a widget, it is the window's
//! stable root, and anything registered from inside its `build()` outlives every
//! panel in the window. So this module is the arrangement for an extension to
//! register **through** `App` rather than through its own widget.
//!
//! ## The menu row is built with the menu, not synced into it afterwards
//!
//! Every other registry in the seam is a snapshot read when its surface is built;
//! this one is no different. `project_menus::build_project_menu` reads
//! [`menu_rows`] while it is assembling the Tools menu, so a row cannot be pushed
//! twice by a later `App` rebuild, and there is no menu id two call sites have to
//! agree on with no handshake between them.
//!
//! The consequence is that a row's *enabled* state is a `Signal` the extension
//! owns and updates from its own views, rather than a predicate over a context
//! the menu build does not have (`EditorsViewModel` — and therefore
//! [`SeamContext`] — does not exist yet when the menu is assembled). That is the
//! honest split: the app owns the menu, the extension owns whether its verb
//! currently applies.
//!
//! ## Registration
//!
//! ```ignore
//! let _handle = teksilo_ui::commands_ext::register_command(
//!     "acme",
//!     ExtensionCommand {
//!         intent: "acme.next_beat",
//!         build_action: Rc::new(|ctx, cx| {
//!             let plan = …;                      // the extension's own state
//!             let work = cx.work.clone();         // …and the Work it belongs to
//!             // `ctx` is App's own — hang the row's `enabled` upkeep on it here.
//!             Action::new("acme.next_beat")
//!                 .on_invoke(move |_i, _c| work.mark_changed(plan.advance()))
//!         }),
//!         shortcut: Some(ShortcutSpec {
//!             name: "acme.next_beat",
//!             label: "Next beat",
//!             primary: KeyStroke::new(Key::F8, Modifiers::NONE),
//!         }),
//!         menu_row: Some(MenuRow { label: Rc::new(|| tr!(acme_next_beat())), enabled: None }),
//!     },
//! )?;
//! ```
//!
//! ⚠ Like every registry in the extension seam, this is read as a **snapshot** —
//! by `App::build` for the actions and shortcuts, and by each window's menu build
//! for the rows. Register at startup, on the main thread, before `run()`.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::{Action, BuildContext, KeyStroke, Shortcut, Signal};
use teksilo::widgets::{MenuEntry, MenuItems};

/// The same live per-window context a dock already receives — reused rather than
/// duplicated, so an extension learns one shape instead of two. See
/// [`crate::docks::DockContext`] for what is on it and why.
pub type SeamContext = crate::docks::DockContext;

/// Builds an extension's action from the app handles it is given. Called once
/// per window, from `App::build`, so the action is owned by that window's `App`
/// and outlives every panel in it.
///
/// It is handed `App`'s own `BuildContext` as well as the seam context, so a
/// command may register effects there — `ctx.effect(..)`,
/// `ReadSignal::on_change(ctx, ..)` — that live as long as the action does. That
/// is the only place an extension can keep a [`MenuRow::enabled`] signal in step
/// with the writer's focus: the menu is assembled before this window's
/// view-models exist, and a panel's own `build` only runs while the panel is
/// open.
pub type CommandBuildFn = Rc<dyn Fn(&mut BuildContext, &SeamContext) -> Action>;

/// A global shortcut for an extension command.
///
/// Declared, not hardcoded as a chord label: teksilo renders the accelerator per
/// platform and locale (⌘ on macOS, "Strg" in German) from this one declaration.
#[derive(Clone)]
pub struct ShortcutSpec {
    /// The shortcut registry id. Conventionally the same string as the intent,
    /// and what a [`MenuRow`] displays its accelerator from.
    pub name: &'static str,
    /// Human-readable name, for the shortcuts list.
    pub label: &'static str,
    pub primary: KeyStroke,
}

/// A row in the **Tools** menu.
#[derive(Clone)]
pub struct MenuRow {
    /// Resolved per menu build, so a runtime locale switch reaches the label —
    /// the same reason [`crate::docks::ExtensionDock::title`] stores a closure.
    pub label: crate::docks::LabelFn,
    /// Whether the verb currently applies. `None` is always enabled.
    ///
    /// A `Signal` the extension owns, not a predicate over a [`SeamContext`]: the
    /// menu is assembled before this window's view-models exist, so no context is
    /// available there. Keep it in step from [`ExtensionCommand::build_action`],
    /// which *is* handed one — along with a `BuildContext` to hang the effects on.
    pub enabled: Option<Signal<bool>>,
}

/// One verb an extension adds to the application.
#[derive(Clone)]
pub struct ExtensionCommand {
    /// The intent name, e.g. `"acme.next_beat"`. Namespaced by convention and
    /// unique by enforcement — see [`register_command`].
    ///
    /// No `AppIntent` variant is needed or possible: a command fired only *by
    /// name* never needs one, which is already true of most of the app's own
    /// commands (`work.open`, `editor.save`, `app.quit`, …).
    pub intent: &'static str,
    pub build_action: CommandBuildFn,
    pub shortcut: Option<ShortcutSpec>,
    pub menu_row: Option<MenuRow>,
}

struct Registered {
    namespace: String,
    command: ExtensionCommand,
}

// Thread-local for the same reason [`crate::docks`]' registry is: an
// `ExtensionCommand` holds `Rc` closures and `Signal`s, neither of which is
// `Send`. Both readers below run on the UI thread — `App::build` and the menu
// build.
thread_local! {
    static EXTENSION_COMMANDS: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// Add an extension's command to the app.
///
/// Returns `Err` when `intent` is already claimed by a **different** namespace,
/// or collides with one of the application's own command names. Both are
/// refusals rather than warnings, for the same reason a dock id collision is:
/// two actions on one intent do not fail visibly, they make whichever the
/// framework reaches first the only one that ever runs.
///
/// Registering the same namespace twice replaces its earlier entry rather than
/// stacking a second copy. The returned handle unregisters on drop.
pub fn register_command(
    namespace: impl Into<String>,
    command: ExtensionCommand,
) -> Result<CommandHandle, String> {
    let namespace = namespace.into();
    let intent = command.intent;
    if intent.is_empty() {
        return Err("an extension command needs an intent name".to_string());
    }
    if is_app_intent(intent) {
        return Err(format!(
            "intent '{intent}' is one of the application's own commands; namespace yours \
             (e.g. '{namespace}.{intent}')"
        ));
    }
    if let Some(s) = &command.shortcut {
        if s.name.is_empty() {
            return Err(format!("command '{intent}' declares a shortcut with no id"));
        }
        if is_app_shortcut(s.name) {
            return Err(format!(
                "shortcut id '{}' is one of the application's own; namespace yours",
                s.name
            ));
        }
    }
    EXTENSION_COMMANDS.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.command.intent == intent && r.namespace != namespace)
        {
            return Err(format!(
                "intent '{intent}' is already registered by '{}'",
                other.namespace
            ));
        }
        // The shortcut id is a field of its own, so a clash on it is possible
        // between two commands whose intents do not clash at all. Unchecked, the
        // later registration silently wins the keystroke.
        if let Some(s) = &command.shortcut
            && let Some(other) = reg.iter().find(|r| {
                r.namespace != namespace
                    && r.command
                        .shortcut
                        .as_ref()
                        .is_some_and(|o| o.name == s.name)
            })
        {
            return Err(format!(
                "shortcut id '{}' is already registered by '{}'",
                s.name, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            command,
        });
        Ok(CommandHandle { namespace })
    })
}

/// Unregisters its command when dropped.
#[derive(Debug)]
pub struct CommandHandle {
    namespace: String,
}

impl Drop for CommandHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = EXTENSION_COMMANDS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// The application's own command namespaces, which an extension may not shadow.
///
/// A prefix test rather than an enumeration of every intent: the real set is
/// large and evolving (`work.open`, `editor.save`, `go.next_scene`, …), and
/// copying all of it here would rot immediately.
///
/// ⚠ **But this list is hand-written and the app's commands are not**, which is
/// the same rot one level up — it shipped missing `numbering.`, so an extension
/// could have claimed `numbering.tidy_titles` and silently taken over Binder ▸
/// Tidy titles. `tests::every_app_command_namespace_is_listed` is what keeps the
/// two in step: it walks the crate for `Action::new("…")` / `Shortcut::new("…")`
/// and fails if any namespace is missing here. Same technique, and the same
/// reason, as `settings_keys`' own drift test.
const APP_INTENT_NAMESPACES: &[&str] = &[
    "app.",
    "backup.",
    "backups.",
    "binder.",
    "comments.",
    "editor.",
    "export.",
    "footnotes.",
    "format.",
    "go.",
    "image.",
    "numbering.",
    "outline.",
    "preview.",
    "search.",
    "session.",
    "spellcheck.",
    "templates.",
    "timeline.",
    "trash.",
    "view.",
    "welcome.",
    "window.",
    "work.",
];

fn is_app_intent(intent: &str) -> bool {
    APP_INTENT_NAMESPACES
        .iter()
        .any(|ns| intent.starts_with(ns))
}

/// The shortcut ids the application itself has claimed.
///
/// Checked separately from the intent because [`ShortcutSpec::name`] is a
/// **separate field**, only conventionally equal to the intent. Two extensions
/// with perfectly namespaced, non-colliding intents could still both name the
/// shortcut `"editor.save"`, and whichever registered last would silently take
/// the keystroke — the exact failure this module refuses for intents.
///
/// Same drift test, same reason.
fn is_app_shortcut(name: &str) -> bool {
    is_app_intent(name)
}

/// Register every extension command's action and shortcut onto **`App`'s own**
/// `BuildContext`.
///
/// Call from `App::build`, right after the app's own `commands::register_all` —
/// that is what makes the ownership right (see the module docs). Re-running it on
/// a rebuild is correct and expected: the framework drops the previous build's
/// registrations first, so this restores exactly one of each.
pub fn register_all_extension_commands(ctx: &mut BuildContext, cx: &SeamContext) {
    let commands: Vec<ExtensionCommand> =
        EXTENSION_COMMANDS.with(|reg| reg.borrow().iter().map(|r| r.command.clone()).collect());
    for command in commands {
        if let Some(s) = &command.shortcut {
            ctx.register_shortcut_global(
                Shortcut::new(s.name)
                    .name(s.label.to_string())
                    .primary(s.primary)
                    .build(),
            );
        }
        // Two statements, not one: `register_action_global` takes `&mut ctx`, and
        // so does the builder. Nesting the calls holds two mutable borrows.
        let action = (command.build_action)(ctx, cx);
        ctx.register_action_global(action);
    }
}

/// Every registered command's Tools-menu row, appended to `items` in
/// registration order.
///
/// Called by `project_menus::build_project_menu` while it assembles the Tools
/// menu — a snapshot, like every other registry in the seam.
pub fn menu_rows(items: MenuItems) -> MenuItems {
    EXTENSION_COMMANDS.with(|reg| {
        let reg = reg.borrow();
        let mut items = items;
        let rows = reg
            .iter()
            .filter_map(|r| r.command.menu_row.as_ref().map(|m| (&r.command, m)));
        for (command, row) in rows {
            let mut entry = MenuEntry::new((row.label)()).intent(command.intent);
            if let Some(enabled) = &row.enabled {
                entry = entry.enabled(enabled.clone());
            }
            if let Some(s) = &command.shortcut {
                entry = entry.shortcut(s.name);
            }
            items = items.item(entry);
        }
        items
    })
}

/// Whether any registered command asks for a Tools row — so the menu build can
/// skip the separator that would otherwise dangle under the app's own rows.
pub fn has_menu_rows() -> bool {
    EXTENSION_COMMANDS.with(|reg| reg.borrow().iter().any(|r| r.command.menu_row.is_some()))
}

#[cfg(test)]
mod tests {
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
            work: crate::view_models::WorkHandle::detached(app_ctx, AppIds::new()),
            active: crate::active_context::ActiveContext::detached(),
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

        let _h =
            register_command("test.cmd.rebuild", with_shortcut("t11.rebuild")).expect("register");

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
}
