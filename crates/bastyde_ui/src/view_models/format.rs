// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `FormatViewModel` — single-instance live state behind every formatting
//! surface: the trailing Format dock, the Format menu, and the four-button row
//! on the editor's right-click menu.
//!
//! Three rules shape this file.
//!
//! **Never cache an `EditorHandle`.** `RichTextEditor::construct()` mints a
//! fresh `Rc<RefCell<EditorState>>` every time it runs, and a theme or locale
//! switch is enough to re-run it — a stored handle would keep addressing the
//! editor the user *used* to be typing in. So the current editor is resolved on
//! demand, through a closure `App` supplies, exactly as `insert_scene_break`
//! already resolves its target. The same reason [`FindViewModel`] is re-attached
//! on every rebuild rather than held.
//!
//! The editor **registry** below does hold handles, and does not break that
//! rule: an entry is created in a widget's `build` and withdrawn in its `Drop`,
//! so a handle is never reachable for longer than the editor it addresses is
//! mounted, and a rebuild re-points the entry at the fresh handle. The registry
//! exists because the resolver reaches one editor per tab, while a Full
//! Chapter/Part/Book stream builds one per row and a corkboard builds one per
//! card — the editors the writer is most often actually typing in.
//!
//! **The mirror signals are pushed, not derived.** `IconButton::toggle` and
//! `MenuEntry::checked` both want a plain `Signal<bool>` they can read (and, for
//! the button, write), and a derived signal is read-only. So the state is
//! mirrored into ordinary signals by [`FormatViewModel::refresh`], which the
//! dock calls from a frame tick. It is deliberately *not* an effect on the
//! editor's `format_version`: that signal is written from inside the editor's
//! own `state.borrow_mut()`, and observers fire synchronously there, so reading
//! the state back would panic on an already-borrowed cell. A frame tick fires
//! outside any borrow.
//!
//! **The dock and the menu do not each roll their own state.** They bind to the
//! same signals from this one view-model, so a checkmark in the menu and a lit
//! button in the dock can never disagree.
//!
//! Peer view-models are not imported here (see the DAG rule in
//! [`crate::view_models`]) — `App` injects the editor resolver, which keeps this
//! type headless-testable against a plain `RichTextEditor` with no `WidgetTree`.
//!
//! **Every surface must call `ctx.request_frame()` after invoking a command.**
//! The commands here deliberately take no `EventContext` (threading one through
//! would cost the headless testability above). But an edit made while the
//! pointer is on a dock button or a menu overlay leaves the editor unfocused,
//! and under bastyde's draw-when-needed contract nothing then schedules the
//! frame that drains the document's events and repaints — the formatting
//! simply does not appear. So each surface funnels its buttons through one
//! local constructor that runs the command and requests the frame together,
//! rather than repeating the pair at every call site where one can be
//! forgotten.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bastyde::prelude::{Signal, WidgetId};
use bastyde::text_document::{Alignment, TextDirection};
use bastyde::widgets::rich_text::EditorHandle;

/// Index of "align left" in the alignment radio group.
pub const ALIGN_LEFT: usize = 0;
/// Index of "align centre".
pub const ALIGN_CENTER: usize = 1;
/// What an alignment Skribisto does not offer collapses to.
///
/// Right and Justify are deliberately not on the control surface — Justify
/// duplicates the export preset's own setting and is a typesetting decision
/// rather than a drafting one, and Right has no manuscript use. But a document
/// can still *carry* them: Djot round-trips block alignment, so an imported or
/// hand-edited file may arrive justified. Collapsing those to a third index
/// leaves no button lit, which is honest — the state is real but not one of
/// ours — and choosing Left or Center replaces it.
pub const ALIGN_OTHER: usize = 2;

/// Index of "direction follows the text" in the direction radio group.
///
/// The default, and a genuinely distinct state from an explicit
/// left-to-right: with no direction stored, the bidi algorithm reads the
/// paragraph's first strong character. That is right almost always, and
/// wrong in the cases worth having a control for — an Arabic paragraph
/// opening with a Latin acronym or a quoted English title reads as
/// left-to-right and lays itself out backwards.
pub const DIR_AUTO: usize = 0;
/// Index of "left to right", set explicitly.
pub const DIR_LTR: usize = 1;
/// Index of "right to left", set explicitly.
pub const DIR_RTL: usize = 2;

/// A blockquote's nesting depth is not queryable through `EditorHandle`, so
/// [`FormatViewModel::clear_formatting`] unwraps one level at a time and stops
/// here. Deeper than this in a manuscript is a corrupt document, not a style.
const MAX_BLOCKQUOTE_UNWRAP: usize = 16;

/// The "nothing compared yet" value for [`FormatViewModel`]'s change gate.
///
/// Deliberately unreachable rather than `(0, 0)`: a freshly-focused editor
/// reports version 0 with the caret at 0, so a zero sentinel *matches* and the
/// gate skips the one sync that mattered — leaving the dock showing plain text
/// over a document that opens bold, or the previous editor's state after a
/// switch between two documents that happen to agree on both numbers.
const NEVER_SEEN: (u64, usize) = (u64::MAX, usize::MAX);

/// What the caret is sitting in, and therefore which control groups make sense.
///
/// This is about the *kind* of text, not about which widget holds focus: the
/// dock hides groups that would be meaningless where the writer is working,
/// rather than showing a wall of dead buttons. Ordering matters only in that
/// [`Self::None`] is the empty state.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FormatSurface {
    /// A scene's own prose — the full toolkit, scene breaks included.
    Scene,
    /// A note's prose. Everything a scene gets except scene breaks: the
    /// compiler never scans notes for markers, so a break placed here would be
    /// a mark the exporter silently ignores.
    Note,
    /// A synopsis box or a corkboard card. Real prose, so the character marks
    /// and lists apply; but a synopsis is not chapter-structured, so headings,
    /// alignment, blockquote, tables and scene breaks do not.
    ///
    /// Reached two ways: through `EditorsViewModel::format_target`, which
    /// prefers the tab's prose editor and falls back to its synopsis; and
    /// through the editor registry, which is how a **stream row's synopsis** and
    /// a **corkboard card** get here. Those build many editors per tab, so no
    /// single per-tab handle can say which one the caret is in — the registry
    /// asks the editors themselves and lets focus decide.
    Synopsis,
    /// Nothing formattable has focus — the binder, a dock, the title field
    /// (a plain `TextInput`, not a rich editor). The dock shows its empty
    /// state; the menu disables.
    #[default]
    None,
}

impl FormatSurface {
    /// Undo / redo.
    pub fn shows_history(self) -> bool {
        self != Self::None
    }

    /// Bold, italic, underline, strikethrough, clear formatting.
    pub fn shows_marks(self) -> bool {
        self != Self::None
    }

    /// Heading level, alignment, blockquote.
    pub fn shows_block(self) -> bool {
        matches!(self, Self::Scene | Self::Note)
    }

    /// Bullet and numbered lists, indent, outdent.
    pub fn shows_lists(self) -> bool {
        self != Self::None
    }

    /// Insert table and the seven table operations. The operations themselves
    /// gate further on [`FormatViewModel::in_table`] — the group is present
    /// wherever a table *could* live, its row/column commands only where one
    /// actually does.
    pub fn shows_tables(self) -> bool {
        matches!(self, Self::Scene | Self::Note)
    }

    /// Minor and major scene breaks. Scene prose only, matching the predicate
    /// `skribisto_compiler` uses to decide what it scans, so the command
    /// surface and the exporter cannot disagree about where a break means
    /// something.
    pub fn shows_scene_breaks(self) -> bool {
        self == Self::Scene
    }

    /// Whether the dock should render its placeholder instead of any controls.
    pub fn is_empty(self) -> bool {
        self == Self::None
    }
}

/// Resolves the editor to act on **and** what kind of text it is, together.
///
/// One closure rather than two because both answers come from the same walk of
/// the focused pane's tab list, and this runs on every pumped frame — i.e.
/// continuously while the writer types. Injected by `App`, the only layer that
/// can see both the pane/tab structure and the editors.
type ResolveTarget = Rc<dyn Fn() -> (Option<EditorHandle>, FormatSurface)>;

/// What a **registered** editor knows about its own content.
///
/// Deliberately coarser than [`FormatSurface`]: an editor holding prose cannot
/// tell a scene from a note — that is a property of the *item* the tab was
/// opened on, which only the resolver can see. [`FormatViewModel::target`]
/// refines this against the resolver's answer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EditorKind {
    /// A manuscript-prose editor (`writing_column`).
    Prose,
    /// A synopsis editor — the tab's own, a stream row's, or a corkboard card's.
    Synopsis,
}

/// One live editor widget, registered while it is mounted.
///
/// The registry is what lets the formatting surfaces reach the editors the
/// per-tab resolver structurally cannot: a **stream row** (Full Chapter/Part/
/// Book builds one editor per row) and a **corkboard card** (one per card).
/// A single per-tab handle cannot say *which* of those the writer is in, so
/// each editor registers itself and focus picks the winner.
///
/// Registration is owned by `TypographyBoundEditor` — every writing editor in
/// the app is wrapped in one — which registers in `build` and unregisters in
/// `Drop`. That is why entries never go stale: the widget's own lifetime is the
/// entry's lifetime, so a handle is dropped from here the moment the editor it
/// addresses stops existing.
struct RegisteredEditor {
    /// The wrapper's `self_id` — the same identity `wire_spell` uses as its
    /// per-view token.
    ///
    /// Keying on it is safe against the build/drop interleaving, which is worth
    /// stating because getting it wrong would silently unregister a live editor:
    /// on the default rebuild path `rebuild_single_widget` destroys the old
    /// children *before* `build()` runs, so the old entry is withdrawn before
    /// the new one arrives; on the preserve path a re-attached child keeps its
    /// id and is not rebuilt at all. A `WidgetId` is a slotmap key, so even a
    /// reused slot comes back with a bumped version — a freed id can never
    /// collide with a live one.
    id: WidgetId,
    handle: EditorHandle,
    kind: EditorKind,
}

/// One gate per control group, for the dock to hang `visible_when` on.
///
/// The dock **hides** groups that do not apply rather than greying them out: a
/// synopsis has no headings and no tables, and eleven dead buttons teach a
/// writer nothing. Contrast the Format menu, which keeps its rows and disables
/// them — a menu is a map of what exists, a dock is a set of what applies.
#[derive(Clone, Debug)]
pub struct GroupVisibility {
    pub history: Signal<bool>,
    pub marks: Signal<bool>,
    pub block: Signal<bool>,
    pub lists: Signal<bool>,
    pub tables: Signal<bool>,
    pub scene_breaks: Signal<bool>,
    /// The placeholder replaces the controls entirely.
    pub empty: Signal<bool>,
}

impl GroupVisibility {
    fn new(surface: FormatSurface) -> Self {
        Self {
            history: Signal::new(surface.shows_history()),
            marks: Signal::new(surface.shows_marks()),
            block: Signal::new(surface.shows_block()),
            lists: Signal::new(surface.shows_lists()),
            tables: Signal::new(surface.shows_tables()),
            scene_breaks: Signal::new(surface.shows_scene_breaks()),
            empty: Signal::new(surface.is_empty()),
        }
    }

    fn apply(&self, surface: FormatSurface) {
        set_if_changed(&self.history, surface.shows_history());
        set_if_changed(&self.marks, surface.shows_marks());
        set_if_changed(&self.block, surface.shows_block());
        set_if_changed(&self.lists, surface.shows_lists());
        set_if_changed(&self.tables, surface.shows_tables());
        set_if_changed(&self.scene_breaks, surface.shows_scene_breaks());
        set_if_changed(&self.empty, surface.is_empty());
    }
}

/// The state and command surface behind the format dock, the Format menu and
/// the context-menu row. Cloneable; every clone shares one set of signals.
#[derive(Clone)]
pub struct FormatViewModel {
    /// How to find the current editor. Called fresh on every command and every
    /// [`Self::refresh`] — see the module docs on handle staleness.
    /// How to find the editor and classify it. Wired by `App` on every build —
    /// the view-model is created before `EditorsViewModel` exists, because the
    /// menu bar is built alongside `App` rather than inside it and needs these
    /// signals at that moment. Same shape as `WorkspaceLayoutViewModel`, which
    /// starts editor-less and is re-pointed on each build.
    ///
    /// Inert until then: no target, nothing to format, every group hidden.
    resolve: Rc<RefCell<Option<ResolveTarget>>>,
    /// Every mounted writing editor, whether or not the resolver can see it.
    /// See [`RegisteredEditor`].
    registry: Rc<RefCell<Vec<RegisteredEditor>>>,
    /// The last registered editor to hold focus, kept so the **menu** still has
    /// a target after opening it moved focus to the overlay.
    ///
    /// Written by [`Self::target`], which [`Self::refresh`] calls on every
    /// pumped frame — so it is current by the time focus moves anywhere, a
    /// focus change being itself a repaint.
    ///
    /// The resolver provides this stickiness for the editors it can reach; a
    /// stream row or a corkboard card has no per-tab slot to be sticky in, so
    /// the latch does it for them. Cleared when that editor unregisters, so it
    /// can never outlive the widget.
    sticky: Rc<RefCell<Option<(WidgetId, EditorHandle, EditorKind)>>>,
    /// Which groups apply. Read by the dock to decide what to show and by the
    /// menu to decide what to enable.
    surface: Signal<FormatSurface>,

    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
    strikethrough: Signal<bool>,
    /// Superscript and subscript are one tri-state property in the document,
    /// mirrored as two signals because the toolbar shows two buttons. They are
    /// never both true.
    superscript: Signal<bool>,
    subscript: Signal<bool>,
    blockquote: Signal<bool>,
    /// The caret sits inside a table, so the row/column commands are meaningful.
    in_table: Signal<bool>,
    /// `0` = normal paragraph, `1..=6` = H1..H6. Doubles as the radio index.
    heading: Signal<usize>,
    /// [`ALIGN_LEFT`], [`ALIGN_CENTER`] or [`ALIGN_OTHER`]. The menu's radio
    /// group binds to this directly.
    alignment: Signal<usize>,
    /// The same value as two booleans, because `IconButton::toggle` wants a
    /// `Signal<bool>` per button. Both false when the block carries an
    /// alignment Skribisto does not offer — see [`ALIGN_OTHER`].
    align_left: Signal<bool>,
    align_center: Signal<bool>,
    /// [`DIR_AUTO`], [`DIR_LTR`] or [`DIR_RTL`] — the direction radio
    /// group's index.
    direction: Signal<usize>,
    /// The same value as one boolean for the dock's toggle button, which
    /// wants a `Signal<bool>`. Lit only for an explicit right-to-left, so
    /// an auto-detected RTL paragraph leaves it dark — the button reports
    /// what is *stored*, which is what pressing it changes.
    dir_rtl: Signal<bool>,
    can_undo: Signal<bool>,
    can_redo: Signal<bool>,
    /// Whether there is an editor to act on at all — the Format menu's
    /// enablement.
    ///
    /// Tracks the *sticky* target, not [`Self::surface`]: opening the menu
    /// blurs the editor, so an enablement keyed on live focus would grey out
    /// every item at the instant the user reached for one.
    has_target: Signal<bool>,

    /// Per-group visibility, pushed by [`Self::set_surface`].
    ///
    /// Plain signals rather than maps over [`Self::surface`] for the same
    /// reason the mirrors are: `visible_when` takes a `Prop`, which *observes*,
    /// and observing a derived signal panics. Deriving these would look tidier
    /// and blow up the first time a group changed.
    group_visible: GroupVisibility,

    /// Last `(format_version, cursor_position)` seen by [`Self::refresh`], so a
    /// per-frame call is nearly free when nothing has moved. [`NEVER_SEEN`]
    /// when the mirrors hold nothing worth comparing against.
    last_seen: Rc<Cell<(u64, usize)>>,
}

impl std::fmt::Debug for FormatViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FormatViewModel")
            .field("surface", &self.surface.get())
            .finish_non_exhaustive()
    }
}

impl FormatViewModel {
    /// `resolve` is called every time a command runs or the mirrors refresh; it
    /// must walk to the currently-focused editor rather than returning a stored
    /// handle. `App` wires it to `EditorsViewModel::focused_prose_handle`.
    pub fn new(resolve: ResolveTarget) -> Self {
        let vm = Self::detached();
        vm.attach(resolve);
        vm
    }

    /// A view-model with nothing to format yet. `App` calls [`Self::attach`]
    /// once the editors exist.
    pub fn detached() -> Self {
        Self {
            resolve: Rc::new(RefCell::new(None)),
            registry: Rc::new(RefCell::new(Vec::new())),
            sticky: Rc::new(RefCell::new(None)),
            surface: Signal::new(FormatSurface::None),
            bold: Signal::new(false),
            italic: Signal::new(false),
            underline: Signal::new(false),
            strikethrough: Signal::new(false),
            superscript: Signal::new(false),
            subscript: Signal::new(false),
            blockquote: Signal::new(false),
            in_table: Signal::new(false),
            heading: Signal::new(0),
            alignment: Signal::new(ALIGN_LEFT),
            align_left: Signal::new(true),
            align_center: Signal::new(false),
            direction: Signal::new(DIR_AUTO),
            dir_rtl: Signal::new(false),
            can_undo: Signal::new(false),
            can_redo: Signal::new(false),
            has_target: Signal::new(false),
            group_visible: GroupVisibility::new(FormatSurface::None),
            last_seen: Rc::new(Cell::new(NEVER_SEEN)),
        }
    }

    // ── The current editor ────────────────────────────────────────────────

    /// The editor to act on right now, or `None`.
    /// Point this view-model at the editors. Idempotent — `App` calls it on
    /// every build, and re-pointing is the whole intent.
    pub fn attach(&self, resolve: ResolveTarget) {
        *self.resolve.borrow_mut() = Some(resolve);
    }

    /// Announce a mounted writing editor. Called from `TypographyBoundEditor`'s
    /// `build`, which wraps every writing editor in the app; paired with
    /// [`Self::unregister`] from its `Drop`.
    ///
    /// Re-registering the same `id` replaces the entry — a rebuild mints a fresh
    /// handle for the same widget slot, and the new one must win.
    pub fn register(&self, id: WidgetId, handle: EditorHandle, kind: EditorKind) {
        let mut registry = self.registry.borrow_mut();
        match registry.iter_mut().find(|e| e.id == id) {
            Some(entry) => {
                entry.handle = handle.clone();
                entry.kind = kind;
            }
            None => registry.push(RegisteredEditor {
                id,
                handle: handle.clone(),
                kind,
            }),
        }
        // A rebuild of the editor the menu is sticky on must re-point the latch
        // too, or the next command would address the previous widget's state.
        let mut sticky = self.sticky.borrow_mut();
        if let Some((sticky_id, sticky_handle, sticky_kind)) = sticky.as_mut()
            && *sticky_id == id
        {
            *sticky_handle = handle;
            *sticky_kind = kind;
        }
    }

    /// Drop a torn-down editor. Also clears the sticky latch when it named this
    /// editor, so the menu can never act through a handle whose widget is gone.
    pub fn unregister(&self, id: WidgetId) {
        self.registry.borrow_mut().retain(|e| e.id != id);
        let mut sticky = self.sticky.borrow_mut();
        if sticky.as_ref().is_some_and(|(s, _, _)| *s == id) {
            *sticky = None;
        }
    }

    /// The registered editor holding keyboard focus, if any. At most one widget
    /// in the window has focus, so this never has to break a tie.
    fn focused_registered(&self) -> Option<(WidgetId, EditorHandle, EditorKind)> {
        self.registry
            .borrow()
            .iter()
            .find(|e| e.handle.focused_signal().get())
            .map(|e| (e.id, e.handle.clone(), e.kind))
    }

    /// The current target and what kind of text it is. `(None, None)` before
    /// `App` has attached a resolver.
    ///
    /// Three sources, most specific first:
    ///   1. a **registered editor with focus** — the only answer that can name a
    ///      stream row or a corkboard card, and always the right one when it
    ///      exists (it is literally where the caret is);
    ///   2. the **sticky latch** — that same editor, remembered across the focus
    ///      loss that opening the Format menu causes (target only: the surface
    ///      stays live, so the dock still empties);
    ///   3. the **resolver** — the focused pane's active tab, which stays correct
    ///      for the scene/note/synopsis tabs it can see and is the honest `None`
    ///      when nothing is open.
    fn target(&self) -> (Option<EditorHandle>, FormatSurface) {
        let fallback = match self.resolve.borrow().as_ref() {
            Some(resolve) => resolve(),
            None => (None, FormatSurface::None),
        };
        let live = self.focused_registered();
        if let Some((id, handle, kind)) = live {
            *self.sticky.borrow_mut() = Some((id, handle.clone(), kind));
            return (Some(handle), self.classify(kind, fallback.1));
        }
        // Nothing focused: keep the latched *target*, so a card or a stream row
        // survives the trip through the menu bar — but take the resolver's
        // *surface* verbatim. It reports `None` the moment focus leaves the
        // editors, and that liveness is what empties the dock. Sticky target,
        // live surface: the same split `App`'s resolver already makes, extended
        // to the editors it cannot see.
        let sticky = self.sticky.borrow().clone();
        if let Some((_, handle, _)) = sticky {
            return (Some(handle), fallback.1);
        }
        fallback
    }

    /// Widen a registered editor's coarse [`EditorKind`] into a [`FormatSurface`].
    ///
    /// Synopsis is self-describing. Prose is not: only the tab knows whether the
    /// item is a note, so the resolver's classification is kept when it says so,
    /// and anything else (including a stream row, whose tab is a container) is a
    /// scene — manuscript prose, where scene breaks belong.
    fn classify(&self, kind: EditorKind, resolved: FormatSurface) -> FormatSurface {
        match kind {
            EditorKind::Synopsis => FormatSurface::Synopsis,
            EditorKind::Prose if resolved == FormatSurface::Note => FormatSurface::Note,
            EditorKind::Prose => FormatSurface::Scene,
        }
    }

    /// Return keyboard focus to the editor a command just acted on.
    ///
    /// The Format **menu** needs this and the dock does not: reaching a menu
    /// item moves focus to the menu overlay, so without this the writer is left
    /// with no caret and has to click back into the prose before typing. Same
    /// reason — and same fix — as `EditorsViewModel::insert_scene_break`.
    pub fn refocus(&self, ctx: &mut bastyde::prelude::EventContext) {
        if let Some(handle) = self.handle() {
            handle.focus(ctx);
        }
    }

    fn handle(&self) -> Option<EditorHandle> {
        self.target().0
    }

    /// The editor a template should be inserted into — the same resolution every format
    /// command uses, exposed for the note-template commands, which live outside this module
    /// because templates are Work-scoped *data* rather than formatting state.
    pub fn handle_for_commands(&self) -> Option<EditorHandle> {
        self.handle()
    }

    // ── Signals the views bind to ─────────────────────────────────────────

    pub fn surface_signal(&self) -> Signal<FormatSurface> {
        self.surface.clone()
    }
    pub fn bold(&self) -> Signal<bool> {
        self.bold.clone()
    }
    pub fn italic(&self) -> Signal<bool> {
        self.italic.clone()
    }
    pub fn underline(&self) -> Signal<bool> {
        self.underline.clone()
    }
    pub fn strikethrough(&self) -> Signal<bool> {
        self.strikethrough.clone()
    }
    pub fn superscript(&self) -> Signal<bool> {
        self.superscript.clone()
    }
    pub fn subscript(&self) -> Signal<bool> {
        self.subscript.clone()
    }
    pub fn blockquote(&self) -> Signal<bool> {
        self.blockquote.clone()
    }
    pub fn in_table(&self) -> Signal<bool> {
        self.in_table.clone()
    }
    pub fn heading(&self) -> Signal<usize> {
        self.heading.clone()
    }
    pub fn alignment(&self) -> Signal<usize> {
        self.alignment.clone()
    }
    pub fn align_left(&self) -> Signal<bool> {
        self.align_left.clone()
    }
    pub fn direction(&self) -> Signal<usize> {
        self.direction.clone()
    }
    pub fn dir_rtl(&self) -> Signal<bool> {
        self.dir_rtl.clone()
    }
    pub fn align_center(&self) -> Signal<bool> {
        self.align_center.clone()
    }
    pub fn can_undo(&self) -> Signal<bool> {
        self.can_undo.clone()
    }
    pub fn can_redo(&self) -> Signal<bool> {
        self.can_redo.clone()
    }
    /// Whether any editor is available to format. The Format menu's enablement
    /// gate — the dock hides groups instead, so it does not read this.
    ///
    /// Sticky by construction (see [`Self::target`]): it must not drop to false
    /// when opening the menu blurs the editor, or every row would grey out at
    /// the instant the writer reached for one.
    ///
    /// This gate once disabled the *entire* Format menu, and the reason is worth
    /// keeping: before the editor registry, the target came only from the
    /// focused tab's prose/synopsis handle, and a Full Chapter/Part/Book or
    /// corkboard tab attaches neither (its rows pass `None` for both the find
    /// and the synopsis sink). So in exactly the views a writer drafts in, there
    /// was no target, and every row was correctly-but-uselessly disabled. The
    /// registry is what makes this signal true there.
    pub fn has_target(&self) -> Signal<bool> {
        self.has_target.clone()
    }

    /// The current surface. `App` is the only writer — it knows which pane and
    /// which tab the focus landed in; this view-model deliberately does not.
    pub fn set_surface(&self, surface: FormatSurface) {
        if self.surface.get() == surface {
            return;
        }
        self.surface.set(surface);
        self.group_visible.apply(surface);
    }

    /// Per-group visibility gates for the dock. See [`GroupVisibility`].
    pub fn groups(&self) -> &GroupVisibility {
        &self.group_visible
    }

    // ── Mirroring editor state ────────────────────────────────────────────

    /// Pull the editor's current formatting into the mirror signals.
    ///
    /// Call once per frame from the dock. Cheap when nothing has changed: the
    /// editor's `format_version` and cursor position are compared against the
    /// last values seen, and an unchanged pair returns without touching the
    /// document. When no editor is focused the mirrors are cleared, so a stale
    /// "bold" never lingers over an empty state.
    pub fn refresh(&self) {
        // Reclassified before the version gate below, and on every frame rather
        // than only when the caret moves: focus can move between editors — or
        // out of them entirely — without the document changing at all, and the
        // dock would otherwise keep showing the previous surface's groups.
        let (handle, surface) = self.target();
        self.set_surface(surface);
        set_if_changed(&self.has_target, handle.is_some());

        let Some(handle) = handle else {
            self.clear_mirrors();
            return;
        };

        let version = handle.format_version().get();
        let caret = handle.cursor_position_signal().get();
        if (version, caret) == self.last_seen.get() {
            return;
        }
        self.last_seen.set((version, caret));
        self.sync_from(&handle);
    }

    /// Re-read every mirror from `handle`, unconditionally.
    ///
    /// Used straight after a command so a toggle button shows the result
    /// immediately rather than waiting a frame — and, for the toggles, so the
    /// editor's answer overrides `IconButton::toggle`'s optimistic flip. That
    /// matters on a mixed selection, where "toggle bold" over half-bold text is
    /// not a pure negation and the optimistic value would be wrong.
    pub fn sync_now(&self) {
        if let Some(handle) = self.handle() {
            self.last_seen.set((
                handle.format_version().get(),
                handle.cursor_position_signal().get(),
            ));
            self.sync_from(&handle);
        } else {
            self.clear_mirrors();
        }
    }

    fn sync_from(&self, handle: &EditorHandle) {
        set_if_changed(&self.bold, handle.is_bold());
        set_if_changed(&self.italic, handle.is_italic());
        set_if_changed(&self.underline, handle.is_underline());
        set_if_changed(&self.strikethrough, handle.is_strikethrough());
        set_if_changed(&self.superscript, handle.is_superscript());
        set_if_changed(&self.subscript, handle.is_subscript());
        set_if_changed(&self.blockquote, handle.is_in_blockquote());
        set_if_changed(&self.in_table, handle.is_in_table());
        set_if_changed(&self.heading, handle.get_heading_level() as usize);
        let alignment = alignment_index(&handle.get_alignment());
        set_if_changed(&self.alignment, alignment);
        set_if_changed(&self.align_left, alignment == ALIGN_LEFT);
        set_if_changed(&self.align_center, alignment == ALIGN_CENTER);
        let direction = direction_index(handle.get_direction());
        set_if_changed(&self.direction, direction);
        set_if_changed(&self.dir_rtl, direction == DIR_RTL);
        set_if_changed(&self.can_undo, handle.can_undo().get());
        set_if_changed(&self.can_redo, handle.can_redo().get());
    }

    fn clear_mirrors(&self) {
        self.last_seen.set(NEVER_SEEN);
        set_if_changed(&self.bold, false);
        set_if_changed(&self.italic, false);
        set_if_changed(&self.underline, false);
        set_if_changed(&self.strikethrough, false);
        set_if_changed(&self.superscript, false);
        set_if_changed(&self.subscript, false);
        set_if_changed(&self.blockquote, false);
        set_if_changed(&self.in_table, false);
        set_if_changed(&self.heading, 0);
        set_if_changed(&self.alignment, ALIGN_LEFT);
        set_if_changed(&self.align_left, true);
        set_if_changed(&self.align_center, false);
        set_if_changed(&self.direction, DIR_AUTO);
        set_if_changed(&self.dir_rtl, false);
        set_if_changed(&self.can_undo, false);
        set_if_changed(&self.can_redo, false);
    }

    // ── Character marks ───────────────────────────────────────────────────
    //
    // Each runs the real command then re-syncs, so the mirrors carry the
    // editor's answer rather than an assumption.

    pub fn toggle_bold(&self) {
        self.with_editor(|h| h.toggle_bold());
    }
    pub fn toggle_italic(&self) {
        self.with_editor(|h| h.toggle_italic());
    }
    pub fn toggle_underline(&self) {
        self.with_editor(|h| h.toggle_underline());
    }
    pub fn toggle_strikethrough(&self) {
        self.with_editor(|h| h.toggle_strikethrough());
    }

    /// Raise the selection. Turning superscript on clears subscript — the
    /// document holds one property, and both buttons lit would be a lie.
    pub fn toggle_superscript(&self) {
        self.with_editor(|h| h.toggle_superscript());
    }

    /// Lower the selection. The mirror image of
    /// [`toggle_superscript`](Self::toggle_superscript).
    pub fn toggle_subscript(&self) {
        self.with_editor(|h| h.toggle_subscript());
    }

    /// Strip formatting back to plain prose.
    ///
    /// Clears the six character marks over the selection, then flattens the
    /// caret's blocks: heading to normal, alignment to left, list membership
    /// dropped, blockquote unwrapped to depth zero. Without a selection the
    /// character half is inherently a no-op — there is no range to re-format —
    /// so this degrades to the block half, which is still worth having with the
    /// caret parked in a centred H2.
    ///
    /// Every property is read before it is written, so clearing already-clean
    /// text neither pushes undo entries nor marks the document modified — with
    /// one exception: list membership, which `EditorHandle` offers no query for
    /// (`is_in_list` does not exist, unlike `is_in_blockquote`). That one call
    /// is therefore unconditional, and clearing an already-plain paragraph may
    /// cost one no-op entry. Guard it as soon as a query exists.
    ///
    /// The whole sweep is **one undo entry**. A writer who clears a heading
    /// that was also bold and centred means one action, and should not have to
    /// press Ctrl+Z five times to get back — worse, a single press would
    /// otherwise leave the paragraph half-cleared.
    ///
    /// One thing it does **not** clear: font family. No layer can express
    /// "unset" — `None` means "leave unchanged" at every level down to the DTO,
    /// so the closest available move would pin the run to a literal family
    /// rather than restoring it to the editor's typography default, which is
    /// worse than leaving it alone. The tooltip says so rather than implying a
    /// clean sweep. (See the tri-state work in the plan; it is the fix.)
    pub fn clear_formatting(&self) {
        let Some(handle) = self.handle() else {
            self.clear_mirrors();
            return;
        };

        handle.edit_block(|| {
            if handle.is_bold() {
                handle.set_bold(false);
            }
            if handle.is_italic() {
                handle.set_italic(false);
            }
            if handle.is_underline() {
                handle.set_underline(false);
            }
            if handle.is_strikethrough() {
                handle.set_strikethrough(false);
            }
            if handle.is_superscript() || handle.is_subscript() {
                handle.set_superscript(false);
            }
            if handle.get_heading_level() != 0 {
                handle.set_heading_level(0);
            }
            if handle.get_alignment() != Alignment::Left {
                handle.set_alignment(Alignment::Left);
            }
            // Unset rather than pin left-to-right. Every other property
            // here clears by writing its default, which works because
            // "default" and "unset" render alike — but a paragraph
            // pinned left-to-right lays Arabic out backwards, so for
            // direction the two are not interchangeable.
            if handle.get_direction().is_some() {
                handle.clear_direction();
            }
            handle.remove_from_list();
            // Depth is not queryable, so unwrap one level at a time and bound
            // the loop — a command that cannot make progress must still
            // terminate.
            let mut unwrapped = 0;
            while handle.is_in_blockquote() && unwrapped < MAX_BLOCKQUOTE_UNWRAP {
                handle.decrease_blockquote_depth();
                unwrapped += 1;
            }
        });

        self.sync_now();
    }

    /// Take the caret's block out of its list, leaving a plain paragraph.
    ///
    /// Distinct from [`outdent`](Self::outdent), which steps one nesting level
    /// and deliberately stops at the outermost rather than destroying the list.
    pub fn remove_from_list(&self) {
        self.with_editor(|h| h.remove_from_list());
    }

    // ── Block structure ───────────────────────────────────────────────────

    /// `0` = normal paragraph, `1..=6` = H1..H6. Out-of-range values are
    /// ignored rather than clamped: a caller asking for H9 has a bug, and
    /// silently giving it H6 would hide that.
    pub fn set_heading(&self, level: usize) {
        if level > 6 {
            return;
        }
        self.with_editor(|h| h.set_heading_level(level as u8));
    }

    /// [`ALIGN_LEFT`] or [`ALIGN_CENTER`]. [`ALIGN_OTHER`] is a state a document
    /// can be *in*, never one the user can ask for, so it is ignored here.
    pub fn set_alignment(&self, index: usize) {
        let alignment = match index {
            ALIGN_LEFT => Alignment::Left,
            ALIGN_CENTER => Alignment::Center,
            _ => return,
        };
        self.with_editor(|h| h.set_alignment(alignment.clone()));
    }

    /// Set the paragraph's base reading direction.
    ///
    /// [`DIR_AUTO`] unsets it, which is genuinely different from pinning
    /// [`DIR_LTR`]: an explicit direction overrides the bidi algorithm,
    /// so a paragraph pinned left-to-right keeps laying out that way
    /// even after the writer replaces its text with Arabic.
    pub fn set_direction(&self, index: usize) {
        match index {
            DIR_LTR => self.with_editor(|h| h.set_direction(TextDirection::LeftToRight)),
            DIR_RTL => self.with_editor(|h| h.set_direction(TextDirection::RightToLeft)),
            DIR_AUTO => self.with_editor(|h| h.clear_direction()),
            _ => (),
        }
    }

    /// Flip the paragraph between right-to-left and automatic.
    ///
    /// What the dock's single toggle button does. Turning it off returns
    /// the paragraph to automatic rather than pinning left-to-right: the
    /// writer is undoing a choice, not making the opposite one, and
    /// automatic is right for almost every paragraph.
    ///
    /// Reads the block's current direction from the editor rather than
    /// the cached `dir_rtl` mirror — the mirror only refreshes on
    /// `sync_now`, so deciding from it would flip based on whichever
    /// block the caret was in last.
    pub fn toggle_direction(&self) {
        self.with_editor(|h| {
            if h.get_direction() == Some(TextDirection::RightToLeft) {
                h.clear_direction();
            } else {
                h.set_direction(TextDirection::RightToLeft);
            }
        });
    }

    pub fn toggle_blockquote(&self) {
        self.with_editor(|h| h.toggle_blockquote());
    }

    // ── Lists ─────────────────────────────────────────────────────────────

    pub fn insert_bullet_list(&self) {
        self.with_editor(|h| h.insert_list(false));
    }
    pub fn insert_numbered_list(&self) {
        self.with_editor(|h| h.insert_list(true));
    }
    pub fn indent(&self) {
        self.with_editor(|h| h.indent());
    }
    pub fn outdent(&self) {
        self.with_editor(|h| h.outdent());
    }

    // ── Tables ────────────────────────────────────────────────────────────

    /// A zero-sized table is not a table; the request is dropped rather than
    /// producing a degenerate one.
    pub fn insert_table(&self, rows: usize, columns: usize) {
        if rows == 0 || columns == 0 {
            return;
        }
        self.with_editor(|h| h.insert_table(rows, columns));
    }

    pub fn insert_row_above(&self) {
        self.with_editor(|h| h.insert_row_above());
    }
    pub fn insert_row_below(&self) {
        self.with_editor(|h| h.insert_row_below());
    }
    pub fn insert_column_before(&self) {
        self.with_editor(|h| h.insert_column_before());
    }
    pub fn insert_column_after(&self) {
        self.with_editor(|h| h.insert_column_after());
    }
    pub fn remove_row(&self) {
        self.with_editor(|h| h.remove_current_row());
    }
    pub fn remove_column(&self) {
        self.with_editor(|h| h.remove_current_column());
    }
    pub fn remove_table(&self) {
        self.with_editor(|h| h.remove_current_table());
    }

    // ── History ───────────────────────────────────────────────────────────
    //
    // The editor's own undo stack, not the app's Work-level trunk: these step
    // through the prose edits in the focused document.

    pub fn undo(&self) {
        self.with_editor(|h| h.undo());
    }
    pub fn redo(&self) {
        self.with_editor(|h| h.redo());
    }

    /// Run `command` against the current editor, then re-sync the mirrors.
    /// A no-op when nothing is focused — every command is safe to invoke from a
    /// menu that outlived the editor it was opened over.
    fn with_editor(&self, command: impl FnOnce(&EditorHandle)) {
        let Some(handle) = self.handle() else {
            // Not simply `return`. `IconButton::toggle` flips its bound signal
            // *before* the activation closure runs, so leaving here would strand
            // that flip: the button would sit lit, claiming a formatting that
            // was never applied and that nothing later corrects.
            self.clear_mirrors();
            return;
        };
        command(&handle);
        self.sync_now();
    }
}

/// Write only on a real change. `Signal::set` fires observers unconditionally,
/// so an unguarded per-frame write would wake every bound widget every frame.
fn set_if_changed<T: Clone + PartialEq + 'static>(signal: &Signal<T>, value: T) {
    if signal.get() != value {
        signal.set(value);
    }
}

/// Map the document's alignment onto the radio index. See [`ALIGN_OTHER`].
/// Map a block's stored direction onto its radio index.
///
/// `None` is [`DIR_AUTO`] — the paragraph carries no direction and the
/// bidi algorithm decides. That is deliberately not folded into
/// [`DIR_LTR`]: they lay out identically for ordinary Latin prose but
/// differ exactly where the control earns its place.
fn direction_index(direction: Option<TextDirection>) -> usize {
    match direction {
        None => DIR_AUTO,
        Some(TextDirection::LeftToRight) => DIR_LTR,
        Some(TextDirection::RightToLeft) => DIR_RTL,
    }
}

fn alignment_index(alignment: &Alignment) -> usize {
    match alignment {
        Alignment::Left => ALIGN_LEFT,
        Alignment::Center => ALIGN_CENTER,
        _ => ALIGN_OTHER,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::text_document::TextDocument;
    use bastyde::widgets::rich_text::RichTextEditor;

    /// A view-model over one live editor, plus the editor itself so the test can
    /// assert against the document directly. No `WidgetTree` — the whole point
    /// of keeping the resolver injectable.
    fn vm_over(text: &str) -> (FormatViewModel, RichTextEditor) {
        let (vm, editor, _doc) = vm_over_doc(text);
        (vm, editor)
    }

    /// The document as well, for the assertions `EditorHandle` cannot express —
    /// list membership has no `is_in_list()` query, so it has to be read off
    /// the block itself.
    fn vm_over_doc(text: &str) -> (FormatViewModel, RichTextEditor, TextDocument) {
        let doc = TextDocument::new();
        doc.set_markdown(text)
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc.clone());
        let handle = editor.handle();
        // Scene rather than None so the target reads as real prose; the tests
        // that care about grouping drive `set_surface` themselves.
        let vm = FormatViewModel::new(Rc::new(move || {
            (Some(handle.clone()), FormatSurface::Scene)
        }));
        (vm, editor, doc)
    }

    /// A view-model with nothing focused.
    fn vm_detached() -> FormatViewModel {
        FormatViewModel::new(Rc::new(|| (None, FormatSurface::None)))
    }

    /// Two distinct `WidgetId`s. It is a slotmap key type, so the only way to
    /// mint one is from a slotmap — a throwaway one does fine, and this keeps
    /// the registry tests free of a `WidgetTree`.
    fn two_ids() -> (WidgetId, WidgetId) {
        let mut keys: slotmap::SlotMap<WidgetId, ()> = slotmap::SlotMap::with_key();
        (keys.insert(()), keys.insert(()))
    }

    /// A standalone editor over `text`, and its handle.
    ///
    /// Everything is selected up front: `is_bold` and friends probe the format
    /// at the **selection start**, so without a selection a toggle only changes
    /// the typing format and the character there is still unmarked — the round
    /// trip would be invisible. Same reason `toggling_bold_mirrors_the_editors_answer`
    /// selects before asserting.
    fn loose_editor(text: &str) -> (RichTextEditor, EditorHandle) {
        let doc = TextDocument::new();
        doc.set_markdown(text)
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        let handle = editor.handle();
        editor.select_all();
        (editor, handle)
    }

    /// The registry's whole reason to exist: a corkboard card and a stream row
    /// build editors the per-tab resolver structurally cannot name, so without
    /// this the formatting surfaces would act on the wrong document — or on
    /// nothing at all.
    ///
    /// Asserted by *effect* rather than by comparing handles: `EditorHandle` has
    /// no identity API, and "the command reached this document and not that one"
    /// is the property that actually matters.
    #[test]
    fn a_focused_registered_editor_outranks_the_resolvers_answer() {
        let (_tab_editor, tab_handle) = loose_editor("tab prose");
        let (_card_editor, card_handle) = loose_editor("card synopsis");
        let resolved = tab_handle.clone();
        let vm = FormatViewModel::new(Rc::new(move || {
            (Some(resolved.clone()), FormatSurface::Scene)
        }));
        let (card_id, _) = two_ids();
        vm.register(card_id, card_handle.clone(), EditorKind::Synopsis);

        // Not focused yet: the resolver still owns the answer.
        assert_eq!(vm.target().1, FormatSurface::Scene);
        vm.toggle_bold();
        assert!(
            tab_handle.is_bold(),
            "the resolver's editor took the command"
        );
        assert!(!card_handle.is_bold());

        // The writer clicks into the card. Now the card wins, and it is
        // classified as a synopsis however the resolver classified the tab.
        card_handle.focused_signal().set(true);
        assert_eq!(vm.target().1, FormatSurface::Synopsis);
        vm.toggle_italic();
        assert!(card_handle.is_italic(), "the focused card took the command");
        assert!(!tab_handle.is_italic());
    }

    /// Opening the Format menu blurs the editor. For a tab the resolver stays
    /// sticky on its own; a card has no per-tab slot to be sticky in, so the
    /// latch has to carry it — otherwise every Format command would land on the
    /// tab's prose instead of the card the writer was editing.
    #[test]
    fn a_card_stays_the_target_after_the_menu_takes_focus() {
        let (_tab_editor, tab_handle) = loose_editor("tab prose");
        let (_card_editor, card_handle) = loose_editor("card synopsis");
        let resolved = tab_handle.clone();
        // The live resolver reports `None` for the surface once focus has left
        // the editors — exactly what `App` wires.
        let vm = FormatViewModel::new(Rc::new(move || {
            (Some(resolved.clone()), FormatSurface::None)
        }));
        let (card_id, _) = two_ids();
        vm.register(card_id, card_handle.clone(), EditorKind::Synopsis);

        card_handle.focused_signal().set(true);
        // The per-frame refresh `App` drives off the frame tick. This is what
        // writes the latch, and a focus change always pumps a frame (the caret
        // has to appear), so in the app it has always run by this point.
        vm.refresh();
        // The menu bar takes focus away.
        card_handle.focused_signal().set(false);

        vm.toggle_bold();
        assert!(
            card_handle.is_bold(),
            "the command must still reach the card the writer was in"
        );
        assert!(!tab_handle.is_bold());
        assert_eq!(
            vm.target().1,
            FormatSurface::None,
            "the surface stays live so the dock empties — only the target is sticky"
        );
    }

    /// The registry entry lives exactly as long as the widget. A stream row
    /// scrolled out of existence must not stay formattable.
    #[test]
    fn unregistering_drops_the_entry_and_the_latch() {
        let (_tab_editor, tab_handle) = loose_editor("tab prose");
        let (_row_editor, row_handle) = loose_editor("row prose");
        let resolved = tab_handle.clone();
        let vm = FormatViewModel::new(Rc::new(move || {
            (Some(resolved.clone()), FormatSurface::Scene)
        }));
        let (row_id, _) = two_ids();
        vm.register(row_id, row_handle.clone(), EditorKind::Prose);
        row_handle.focused_signal().set(true);
        row_handle.focused_signal().set(false);

        // The row is torn down while still holding the latch.
        vm.unregister(row_id);
        assert!(vm.registry.borrow().is_empty());
        vm.toggle_bold();
        assert!(
            tab_handle.is_bold(),
            "with the row gone the resolver's tab editor is the answer again"
        );
        assert!(!row_handle.is_bold());
    }

    /// A rebuild mints a fresh `EditorState` for the same widget slot, so the
    /// registry must re-point rather than accumulate — and the latch with it,
    /// or a command would address the widget's previous state.
    #[test]
    fn rebuilding_repoints_the_entry_instead_of_duplicating_it() {
        let (_first, first_handle) = loose_editor("before rebuild");
        let (_second, second_handle) = loose_editor("after rebuild");
        let vm = vm_detached();
        let (id, _) = two_ids();

        vm.register(id, first_handle.clone(), EditorKind::Prose);
        first_handle.focused_signal().set(true);
        vm.refresh(); // latches, as the frame tick would
        first_handle.focused_signal().set(false);

        // Same widget id, fresh handle — a theme or locale switch is enough.
        vm.register(id, second_handle.clone(), EditorKind::Prose);
        assert_eq!(vm.registry.borrow().len(), 1, "one slot, one entry");
        vm.toggle_bold();
        assert!(
            second_handle.is_bold(),
            "the latch must follow the rebuild, not keep the dead state"
        );
        assert!(!first_handle.is_bold());
    }

    /// A prose editor cannot tell a scene from a note; the tab can. The
    /// registry must not flatten a note into a scene and start offering scene
    /// breaks the exporter would ignore.
    #[test]
    fn a_registered_prose_editor_keeps_the_tabs_note_classification() {
        let (_editor, handle) = loose_editor("a note");
        let resolved = handle.clone();
        let vm = FormatViewModel::new(Rc::new(move || {
            (Some(resolved.clone()), FormatSurface::Note)
        }));
        let (id, _) = two_ids();
        vm.register(id, handle.clone(), EditorKind::Prose);
        handle.focused_signal().set(true);
        assert_eq!(vm.target().1, FormatSurface::Note);
    }

    #[test]
    fn commands_are_inert_when_no_editor_is_focused() {
        // Every command must survive being invoked from a menu that outlived
        // its editor — the menu bar blurs the editor to open, and a tab can
        // close underneath an open overlay.
        let vm = vm_detached();
        vm.toggle_bold();
        vm.toggle_italic();
        vm.toggle_underline();
        vm.toggle_strikethrough();
        vm.clear_formatting();
        vm.set_heading(3);
        vm.set_alignment(ALIGN_CENTER);
        vm.toggle_blockquote();
        vm.insert_bullet_list();
        vm.insert_numbered_list();
        vm.indent();
        vm.outdent();
        vm.insert_table(3, 3);
        vm.insert_row_above();
        vm.remove_table();
        vm.undo();
        vm.redo();
        vm.refresh();

        assert!(!vm.bold().get(), "no editor means no state to mirror");
        assert_eq!(vm.heading().get(), 0);
        assert_eq!(vm.alignment().get(), ALIGN_LEFT);
        assert!(vm.surface_signal().get().is_empty());
    }

    #[test]
    fn toggling_bold_mirrors_the_editors_answer() {
        let (vm, editor) = vm_over("Hello world");
        // `is_bold` probes the format at the selection start, so a selection is
        // what makes the round trip observable — without one, a toggle changes
        // the typing format and the character at the caret is still unbolded.
        editor.select_all();
        vm.sync_now();
        assert!(!vm.bold().get());

        vm.toggle_bold();
        assert!(
            vm.bold().get(),
            "the mirror follows the editor, not a guess"
        );
        assert!(editor.handle().is_bold());

        vm.toggle_bold();
        assert!(!vm.bold().get());
        assert!(!editor.handle().is_bold());
    }

    #[test]
    fn heading_and_alignment_round_trip_through_the_radio_indices() {
        let (vm, editor) = vm_over("Hello");
        assert_eq!(vm.heading().get(), 0);

        vm.set_heading(2);
        assert_eq!(vm.heading().get(), 2);
        assert_eq!(editor.handle().get_heading_level(), 2);

        vm.set_alignment(ALIGN_CENTER);
        assert_eq!(vm.alignment().get(), ALIGN_CENTER);
        assert_eq!(editor.handle().get_alignment(), Alignment::Center);

        vm.set_alignment(ALIGN_LEFT);
        assert_eq!(vm.alignment().get(), ALIGN_LEFT);
    }

    #[test]
    fn an_out_of_range_heading_is_refused_rather_than_clamped() {
        let (vm, editor) = vm_over("Hello");
        vm.set_heading(2);
        vm.set_heading(9);
        assert_eq!(
            editor.handle().get_heading_level(),
            2,
            "H9 is a caller bug; silently giving it H6 would hide it"
        );
    }

    #[test]
    fn an_alignment_we_do_not_offer_lights_no_button() {
        let (vm, editor) = vm_over("Hello");
        // Djot round-trips block alignment, so an imported document can arrive
        // justified even though Skribisto offers no way to ask for it.
        editor.handle().set_alignment(Alignment::Justify);
        vm.sync_now();
        assert_eq!(vm.alignment().get(), ALIGN_OTHER);

        // And it is not a state the user can request.
        vm.set_alignment(ALIGN_OTHER);
        assert_eq!(editor.handle().get_alignment(), Alignment::Justify);

        // Choosing a real alignment replaces it.
        vm.set_alignment(ALIGN_LEFT);
        assert_eq!(vm.alignment().get(), ALIGN_LEFT);
    }

    #[test]
    fn clear_formatting_flattens_marks_and_block_structure() {
        let (vm, editor) = vm_over("Hello world");
        let handle = editor.handle();
        editor.select_all();
        handle.set_bold(true);
        handle.set_italic(true);
        handle.set_heading_level(2);
        handle.set_alignment(Alignment::Center);
        vm.sync_now();
        assert!(vm.bold().get() && vm.italic().get());

        vm.clear_formatting();

        assert!(!handle.is_bold(), "bold cleared");
        assert!(!handle.is_italic(), "italic cleared");
        assert_eq!(handle.get_heading_level(), 0, "heading flattened");
        assert_eq!(handle.get_alignment(), Alignment::Left, "alignment reset");
        assert!(!vm.bold().get(), "and the mirrors followed");
        assert_eq!(vm.heading().get(), 0);
    }

    #[test]
    fn superscript_and_subscript_are_mutually_exclusive_in_the_mirrors() {
        let (vm, editor) = vm_over("H2O");
        editor.handle().select_range(1, 2);
        vm.sync_now();

        vm.toggle_subscript();
        assert!(vm.subscript().get() && !vm.superscript().get());

        vm.toggle_superscript();
        assert!(
            vm.superscript().get() && !vm.subscript().get(),
            "one property, two buttons — both lit would be a lie about the document"
        );

        vm.toggle_superscript();
        assert!(!vm.superscript().get() && !vm.subscript().get());
    }

    #[test]
    fn clear_formatting_is_a_single_undo_entry() {
        // A writer clearing a bold, centred heading means one action. Without
        // the edit block this took five Ctrl+Z presses, and the first one left
        // the paragraph half-cleared.
        let (vm, editor) = vm_over("Hello world");
        let handle = editor.handle();
        editor.select_all();
        handle.set_bold(true);
        handle.set_italic(true);
        handle.set_heading_level(2);
        handle.set_alignment(Alignment::Center);
        vm.sync_now();

        vm.clear_formatting();
        assert!(!handle.is_bold() && handle.get_heading_level() == 0);

        handle.undo();
        vm.sync_now();
        assert!(handle.is_bold(), "one undo restores the marks");
        assert!(handle.is_italic(), "...all of them");
        assert_eq!(handle.get_heading_level(), 2, "...and the block format too");
        assert_eq!(handle.get_alignment(), Alignment::Center);
    }

    #[test]
    fn clear_formatting_takes_the_block_out_of_a_list() {
        // `outdent` bottoms out at depth 0 by design, so before
        // `remove_from_list` existed a cleared paragraph stayed a list item.
        let (vm, editor, doc) = vm_over_doc("item");
        let handle = editor.handle();
        handle.insert_list(false);
        vm.sync_now();
        assert!(
            doc.block_at_position(0).expect("block").list().is_some(),
            "precondition: the block is a list item"
        );

        vm.clear_formatting();
        assert!(
            doc.block_at_position(0).expect("block").list().is_none(),
            "clearing formatting must leave a plain paragraph"
        );
    }

    #[test]
    fn remove_from_list_is_reachable_on_its_own() {
        // The dock offers it as its own control, not only via clear-formatting.
        let (vm, editor, doc) = vm_over_doc("item");
        editor.handle().insert_list(true);
        assert!(doc.block_at_position(0).expect("block").list().is_some());

        vm.remove_from_list();
        assert!(doc.block_at_position(0).expect("block").list().is_none());

        // Outside a list it is a no-op, not an error.
        vm.remove_from_list();
        assert!(doc.block_at_position(0).expect("block").list().is_none());
    }

    #[test]
    fn clear_formatting_flattens_superscript() {
        let (vm, editor) = vm_over("E=mc2");
        let handle = editor.handle();
        handle.select_range(4, 5);
        vm.toggle_superscript();
        assert!(vm.superscript().get());

        vm.clear_formatting();
        assert!(
            !handle.is_superscript(),
            "superscript is a character mark and goes with the rest"
        );
        assert!(!vm.superscript().get());
    }

    #[test]
    fn clear_formatting_terminates_on_a_blockquote() {
        // `decrease_blockquote_depth` is driven blind (depth is not queryable),
        // so the loop must be bounded — this test would hang, not fail, on a
        // regression that let it spin.
        let (vm, editor) = vm_over("Hello");
        editor.handle().toggle_blockquote();
        vm.sync_now();

        vm.clear_formatting();
        assert!(!editor.handle().is_in_blockquote());
        assert!(!vm.blockquote().get());
    }

    #[test]
    fn refresh_re_reads_only_when_the_editor_reports_a_change() {
        let (vm, editor) = vm_over("Hello world");
        let handle = editor.handle();
        editor.select_all();
        vm.refresh();
        assert!(!vm.bold().get());

        // Toggle behind the view-model's back, the way typing Ctrl+B in the
        // editor does. `format_version` is bumped by the editor's *batched*
        // event drain, which the frame loop runs — so with no frame loop it
        // stays put, and `refresh` correctly declines to re-read the document.
        // That is the whole point of the gate: it runs every frame, and an
        // unguarded version would wake every bound widget continuously.
        handle.toggle_bold();
        vm.refresh();
        assert!(
            !vm.bold().get(),
            "no version bump yet, so nothing to re-read"
        );

        // Stand in for the drain the frame loop would have done. In the app the
        // dock calls `refresh` from a frame-tick effect, which fires *after*
        // the editor's own tick closure has drained and released its borrow.
        let version = handle.format_version();
        version.set(version.get() + 1);
        vm.refresh();
        assert!(vm.bold().get(), "a version bump makes refresh re-read");
    }

    #[test]
    fn a_command_updates_the_mirrors_without_waiting_for_a_frame() {
        // Commands routed through the view-model re-sync unconditionally, so a
        // button lights up on click rather than a frame later — and, on a mixed
        // selection, shows the editor's answer rather than `IconButton`'s
        // optimistic flip.
        let (vm, editor) = vm_over("Hello world");
        editor.select_all();
        vm.refresh();

        vm.toggle_bold();
        assert!(
            vm.bold().get(),
            "no format_version bump happened, yet the mirror is current"
        );
        assert!(editor.handle().is_bold());
    }

    #[test]
    fn a_degenerate_table_is_refused() {
        let (vm, editor) = vm_over("Hello");
        vm.insert_table(0, 3);
        vm.insert_table(3, 0);
        assert!(
            !editor.handle().is_in_table(),
            "a zero-sized table is not a table"
        );

        vm.insert_table(2, 2);
        assert!(editor.handle().is_in_table());
        assert!(vm.in_table().get(), "and the table group unlocks");
    }

    /// A synopsis is a real formatting target, not a second-class one: the
    /// commands must reach it, and the surface must say what it is so the dock
    /// drops the groups a synopsis has no use for.
    ///
    /// This is the gap that shipped first time round — `App` could only ever
    /// resolve a tab's *main prose* handle, so the caret sitting in a synopsis
    /// produced `None` and the dock showed its empty state over perfectly
    /// formattable text.
    #[test]
    fn a_synopsis_is_a_formattable_target_of_its_own_kind() {
        let doc = TextDocument::new();
        doc.set_markdown("a synopsis line")
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        editor.select_all();
        let handle = editor.handle();
        let vm = FormatViewModel::new(Rc::new(move || {
            (Some(handle.clone()), FormatSurface::Synopsis)
        }));

        vm.refresh();
        assert_eq!(vm.surface_signal().get(), FormatSurface::Synopsis);

        let g = vm.groups();
        assert!(
            g.marks.get() && g.lists.get() && g.history.get(),
            "a synopsis is prose: marks, lists and history all apply"
        );
        assert!(
            !g.block.get() && !g.tables.get() && !g.scene_breaks.get(),
            "but it is not chapter-structured, so those go"
        );
        assert!(!g.empty.get(), "and it is emphatically not the empty state");

        // The commands reach it like any other editor.
        vm.toggle_bold();
        assert!(vm.bold().get(), "bold must apply to a synopsis");
        assert!(editor.handle().is_bold());
    }

    /// A note's synopsis classifies as `Synopsis`, never `Note` — the structural reason the
    /// insert command cannot reach a synopsis box.
    #[test]
    fn a_notes_synopsis_classifies_as_synopsis_not_note() {
        let vm = FormatViewModel::new(Rc::new(|| (None, FormatSurface::None)));
        assert_eq!(
            vm.classify(EditorKind::Synopsis, FormatSurface::Note),
            FormatSurface::Synopsis,
            "the editor kind wins over the tab's own note-ness"
        );
        assert_eq!(
            vm.classify(EditorKind::Prose, FormatSurface::Note),
            FormatSurface::Note
        );
    }

    #[test]
    fn surface_decides_which_groups_appear() {
        use FormatSurface::*;

        // Scene prose is the only place a scene break means anything — the same
        // predicate the compiler uses to decide what it scans.
        assert!(Scene.shows_scene_breaks());
        assert!(!Note.shows_scene_breaks());
        assert!(!Synopsis.shows_scene_breaks());

        // A synopsis is real prose but is not chapter-structured.
        assert!(Synopsis.shows_marks());
        assert!(Synopsis.shows_lists());
        assert!(!Synopsis.shows_block());
        assert!(!Synopsis.shows_tables());

        // History and marks are the high-frequency groups: they never vanish
        // while there is anywhere to type, so moving between a scene and its
        // synopsis does not make the dock flicker.
        for surface in [Scene, Note, Synopsis] {
            assert!(surface.shows_history());
            assert!(surface.shows_marks());
            assert!(surface.shows_lists());
            assert!(!surface.is_empty());
        }

        // And nothing focused shows nothing at all.
        assert!(None.is_empty());
        assert!(!None.shows_history());
        assert!(!None.shows_marks());
        assert!(!None.shows_lists());
    }

    #[test]
    fn the_surface_signal_only_fires_on_a_real_change() {
        let vm = vm_detached();
        assert_eq!(vm.surface_signal().get(), FormatSurface::None);
        vm.set_surface(FormatSurface::Scene);
        assert_eq!(vm.surface_signal().get(), FormatSurface::Scene);
        vm.set_surface(FormatSurface::Scene);
        assert_eq!(vm.surface_signal().get(), FormatSurface::Scene);
    }

    #[test]
    fn losing_focus_clears_the_mirrors() {
        // A stale "bold" lingering over the empty state would be a lie about a
        // document the user is no longer in.
        let flip: Rc<Cell<bool>> = Rc::new(Cell::new(true));
        let doc = TextDocument::new();
        doc.set_markdown("Hello")
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        editor.select_all();
        editor.handle().set_bold(true);

        let handle = editor.handle();
        let gate = flip.clone();
        let vm = FormatViewModel::new(Rc::new(move || {
            if gate.get() {
                (Some(handle.clone()), FormatSurface::Scene)
            } else {
                (None, FormatSurface::None)
            }
        }));

        vm.refresh();
        assert!(vm.bold().get());

        flip.set(false);
        vm.refresh();
        assert!(!vm.bold().get(), "mirrors clear when the editor goes away");
    }

    #[test]
    fn a_fresh_paragraph_reads_as_automatic_direction() {
        let (vm, _editor) = vm_over("Hello");
        vm.refresh();
        assert_eq!(vm.direction().get(), DIR_AUTO);
        assert!(!vm.dir_rtl().get());
    }

    #[test]
    fn pinning_a_direction_is_reported_back() {
        let (vm, _editor) = vm_over("Hello");
        vm.refresh();

        vm.set_direction(DIR_RTL);
        vm.refresh();
        assert_eq!(vm.direction().get(), DIR_RTL);
        assert!(vm.dir_rtl().get(), "the dock toggle should light up");

        vm.set_direction(DIR_LTR);
        vm.refresh();
        assert_eq!(vm.direction().get(), DIR_LTR);
        assert!(
            !vm.dir_rtl().get(),
            "an explicit left-to-right is not right-to-left"
        );
    }

    #[test]
    fn automatic_is_a_state_the_writer_can_get_back_to() {
        // The reason `clear_direction` had to exist: pinning
        // left-to-right is *not* the same as never having chosen, and
        // only an unset direction lets Arabic auto-detect as RTL.
        let (vm, _editor) = vm_over("Hello");
        vm.refresh();

        vm.set_direction(DIR_RTL);
        vm.refresh();
        assert_eq!(vm.direction().get(), DIR_RTL);

        vm.set_direction(DIR_AUTO);
        vm.refresh();
        assert_eq!(
            vm.direction().get(),
            DIR_AUTO,
            "choosing Automatic must unset the direction, not pin LTR"
        );
        assert!(!vm.dir_rtl().get());
    }

    #[test]
    fn the_dock_toggle_flips_between_rtl_and_automatic() {
        let (vm, _editor) = vm_over("Hello");
        vm.refresh();

        vm.toggle_direction();
        vm.refresh();
        assert_eq!(vm.direction().get(), DIR_RTL);

        // Off returns to automatic rather than pinning left-to-right —
        // the writer is undoing a choice, not making the opposite one.
        vm.toggle_direction();
        vm.refresh();
        assert_eq!(vm.direction().get(), DIR_AUTO);
    }

    #[test]
    fn clearing_formatting_also_unsets_the_direction() {
        let (vm, _editor) = vm_over("Hello");
        vm.refresh();
        vm.set_direction(DIR_RTL);
        vm.refresh();

        vm.clear_formatting();
        assert_eq!(
            vm.direction().get(),
            DIR_AUTO,
            "clear formatting must unset the direction, not leave the \
             paragraph pinned right-to-left"
        );
    }
}
