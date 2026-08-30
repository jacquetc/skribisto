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
//! ```text
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
    "edit.",
    "editor.",
    "export.",
    "footnotes.",
    "format.",
    "go.",
    "help.",
    "image.",
    "numbering.",
    "outline.",
    "pace.",
    "preview.",
    "search.",
    "session.",
    "spellcheck.",
    "statuses.",
    "story_bible.",
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

/// Every shortcut id the command palette should offer: the application's own, plus
/// whatever an extension registered.
///
/// The palette reads the tree's whole `ShortcutRegistry`, which also carries bindings
/// the *framework* registers for itself (the debug inspector's `Toggle Picker`,
/// `Cycle Bounds Overlay`, `Next Tab`, …). Those are development tools, not commands a
/// writer is looking for by name, and in a debug build they otherwise sort to the top of
/// an empty query because they carry no category.
///
/// An allow-list rather than a block-list, and deliberately built from the same
/// `APP_INTENT_NAMESPACES` the seam already guards: a block-list of framework ids would
/// rot the first time teksilo added one, silently and invisibly.
pub(crate) fn is_palette_command(id: &str) -> bool {
    if is_app_intent(id) {
        return true;
    }
    EXTENSION_COMMANDS.with(|reg| {
        reg.borrow().iter().any(|r| {
            r.command.intent == id || r.command.shortcut.as_ref().is_some_and(|s| s.name == id)
        })
    })
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
mod tests;
