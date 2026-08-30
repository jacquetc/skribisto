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
//! already resolves its target. The same reason [`crate::search::FindViewModel`] is re-attached
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
//! Peer view-models are not imported here (see the DAG rule the project's
//! view-model conventions define) — `App` injects the editor resolver, which
//! keeps this type headless-testable against a plain `RichTextEditor` with no
//! `WidgetTree`.
//!
//! **Every surface must call `ctx.request_frame()` after invoking a command.**
//! The commands here deliberately take no `EventContext` (threading one through
//! would cost the headless testability above). But an edit made while the
//! pointer is on a dock button or a menu overlay leaves the editor unfocused,
//! and under teksilo's draw-when-needed contract nothing then schedules the
//! frame that drains the document's events and repaints — the formatting
//! simply does not appear. So each surface funnels its buttons through one
//! local constructor that runs the command and requests the frame together,
//! rather than repeating the pair at every call site where one can be
//! forgotten.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use common::types::EntityId;
use teksilo::prelude::{Signal, WidgetId};
use teksilo::text_document::{Alignment, TextDirection};
use teksilo::widgets::rich_text::EditorHandle;

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
///
/// The third component is "may this editor step through its history" — the
/// editor's command filter, folded in because a writing game switches it while
/// the document and the caret both stand still. Without it the dedup gate below
/// skips the only sync that mattered, and the Edit menu keeps offering an Undo
/// row that does nothing.
const NEVER_SEEN: (u64, usize, bool) = (u64::MAX, usize::MAX, false);

/// May this editor step back through its own history?
///
/// `false` while a writing game freezes it — see `WritingGamesViewModel`. Read
/// live from the editor rather than from the game, because the Format surfaces
/// follow *whichever* editor holds the caret and only that editor knows the
/// filter it was given.
fn history_allowed(handle: &EditorHandle) -> bool {
    handle
        .command_filter()
        .accepts(teksilo::widgets::rich_text::EditCommandKind::Undo)
}

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
    /// A synopsis box or a corkboard card. Real prose, and treated as such:
    /// everything a note gets. A synopsis is where a writer sketches — an
    /// outline wants headings, a beat sheet wants a table, a quoted line of
    /// research wants a blockquote — and it compiles through the same
    /// `push_prose` as any other prose, so nothing downstream cares. Only
    /// scene breaks go, for the reason in [`Self::shows_scene_breaks`].
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
    /// Bold, italic, underline, strikethrough, clear formatting.
    pub fn shows_marks(self) -> bool {
        self != Self::None
    }

    /// Heading level, alignment, blockquote.
    ///
    /// Available in a synopsis too. These were once scene-and-note only, on the
    /// reasoning that a synopsis "is not chapter-structured" — but that argues
    /// about what a heading *means* in the finished book, not about what the
    /// writer is doing with it. A synopsis is planning text, and planning text
    /// is exactly where an outline's levels earn their keep.
    pub fn shows_block(self) -> bool {
        self != Self::None
    }

    /// Bullet and numbered lists, indent, outdent.
    pub fn shows_lists(self) -> bool {
        self != Self::None
    }

    /// Insert table and the seven table operations. The operations themselves
    /// gate further on [`FormatViewModel::in_table`] — the group is present
    /// wherever a table *could* live, its row/column commands only where one
    /// actually does.
    ///
    /// A table could live in a synopsis: a beat sheet or a cast grid is
    /// planning material, and the synopsis is where planning material goes.
    pub fn shows_tables(self) -> bool {
        self != Self::None
    }

    /// Minor and major scene breaks. Scene prose only, matching the predicate
    /// `skribisto_compiler` uses to decide what it scans, so the command
    /// surface and the exporter cannot disagree about where a break means
    /// something.
    ///
    /// The one group with a *mechanical* reason to narrow, and therefore the
    /// only one that does: `render.rs` passes `scan: false` for a note's and a
    /// synopsis's prose, so a break placed in either is a mark the exporter
    /// silently drops. Every other group is offered wherever there is prose.
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
    /// This editor's door to the footnote feature, when its surface has one.
    ///
    /// Carried here rather than resolved per command because this registry is
    /// the only place that can answer **which** editor the caret is in — a
    /// stream shows one per row — and a footnote has to annotate the row being
    /// typed into, not the one the tab happens to be named after. The binding
    /// rather than a bare id, because the row may not exist yet and the binding
    /// knows how to mint it.
    footnotes: Option<crate::footnotes::FootnoteBinding>,
    /// The typography bundle this editor is dressed in, so Ctrl+= / Ctrl+− /
    /// Ctrl+0 can resize *the editor the writer is in* rather than guessing from
    /// the tab. `EditorKind` cannot answer this — it only separates prose from
    /// synopsis, while a `Prose` editor may be dressed by Scene, Notes or
    /// Distraction-free, and a `Synopsis` one by three bundles again.
    ///
    /// `None` for a registered surface with no size preference of its own: the
    /// search preview band registers here for the formatting commands but
    /// deliberately bypasses `TypographyBoundEditor` and honours no typography
    /// setting, so there is nothing there for the size commands to move.
    typo: Option<crate::settings::EditorTypography>,
    /// The `BinderItem` whose text this editor is showing.
    ///
    /// Not something the formatting commands ever ask for — they act on the
    /// editor the caret is in and do not care which item it is. It is here
    /// because **this registry is the only correct place for it**, and the
    /// argument is lifetime rather than convenience.
    ///
    /// A margin lane has to reach *a named item's* editor to convert an offset
    /// into a position, which is the one question this registry could not answer:
    /// its reads are all focus-shaped, though its contents never were. Keeping a
    /// second index would mean re-earning [`TypographyBoundEditor`]'s `Drop`
    /// discipline, and the failure mode of getting that wrong is a lane drawing
    /// marks through a handle whose editor is gone. One registry, one lifetime.
    ///
    /// `None` for a surface that is not showing one item's text — the search
    /// preview band, and the editors the widget tests build with no project
    /// around them.
    ///
    /// [`TypographyBoundEditor`]: crate::tabs::shared::editor
    item: Option<EntityId>,
    /// Which writing surface built this editor — see
    /// [`LaneScope`](crate::margin_lane::LaneScope), where the whole argument
    /// lives.
    ///
    /// Set with `item` and never apart from it: several surfaces can show the
    /// same item at once, so the item alone does not identify an editor, and a
    /// lane asking by item alone was answered by whichever had registered first.
    ///
    /// `None` for the editors the widget tests build with no surface around
    /// them, which is also the only case
    /// [`handle_for_item`](Self::handle_for_item) still answers by order.
    scope: Option<crate::margin_lane::LaneScope>,
}

/// One gate per control group, for the dock to hang `visible_when` on.
///
/// The dock **hides** groups that do not apply rather than greying them out:
/// a note has nowhere to put a scene break, and dead buttons teach a writer
/// nothing. Contrast the Format menu, which keeps its rows and disables them —
/// a menu is a map of what exists, a dock is a set of what applies.
///
/// Hiding is only ever a *last* resort, and the bar for it is "the command
/// would do nothing here", not "I cannot picture wanting it here". Only
/// [`FormatSurface::shows_scene_breaks`] clears that bar; the rest of these
/// gates separate "there is prose" from "there is not".
/// What the Link dialog opens with, resolved from the editor before the modal
/// is built.
///
/// Resolved by the view-model rather than by the panel, so the panel never
/// holds an `EditorHandle`: by the time a modal is on screen the editor has
/// lost focus, and a handle grabbed then can be the wrong one — or stale, since
/// a rebuild mints a fresh editor state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkRequest {
    /// The words the link will cover. Pre-filled from the existing link's text,
    /// or from whatever the writer selected.
    pub name: String,
    /// Where it points. Empty when making a new link.
    pub href: String,
    /// The caret was already inside a link, so this is an edit — which is what
    /// decides whether the dialog offers "Remove link".
    pub editing: bool,
}

#[derive(Clone, Debug)]
pub struct GroupVisibility {
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
            marks: Signal::new(surface.shows_marks()),
            block: Signal::new(surface.shows_block()),
            lists: Signal::new(surface.shows_lists()),
            tables: Signal::new(surface.shows_tables()),
            scene_breaks: Signal::new(surface.shows_scene_breaks()),
            empty: Signal::new(surface.is_empty()),
        }
    }

    fn apply(&self, surface: FormatSurface) {
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
    /// Set while a control the **dock itself** owns holds an overlay open.
    ///
    /// Every dock button is `focusable(false)` precisely so that pressing one
    /// never blurs the editor out from under itself — but a popover is not a
    /// button. `PopoverWidget` moves keyboard focus into its content (it has
    /// to: the list is arrow-key navigable), and from the resolver's side that
    /// is indistinguishable from clicking into the binder. The live surface
    /// dropped to [`FormatSurface::None`], every group hid — *including the
    /// group holding the trigger*, whose subtree went dormant and took the
    /// just-opened list with it. Pressing the heading picker blanked the whole
    /// dock, which is how this was found.
    ///
    /// While this is set, [`Self::refresh`] keeps the surface it was already
    /// showing rather than collapsing to the empty state. It is the
    /// surface-side twin of [`Self::sticky`]: the *commands* survive that focus
    /// loss because the target is latched, and now the *dock* survives it
    /// because the surface is too — but only for focus this dock took, and only
    /// for as long as it holds it. Nothing has to put focus back: the overlay
    /// manager records the pre-overlay focus and replays it on every dismiss
    /// path, so the surface goes live again the moment the popover closes.
    dock_overlay_open: Signal<bool>,
    /// Which groups apply. Read by the dock to decide what to show and by the
    /// menu to decide what to enable.
    surface: Signal<FormatSurface>,
    /// The inline image the writer last clicked, as `(character offset, src)`.
    ///
    /// A click on an image does not move the caret — that is the editor's
    /// documented behaviour, shared with links — so the caret cannot say which
    /// picture the writer meant. This can, and it is per-window for the same
    /// reason the rest of this view-model is: two windows on one project have
    /// two carets and two selections.
    ///
    /// Cleared on every other click, so "Describe the image…" is offered only
    /// while there is actually an image in hand.
    active_image: Signal<Option<(usize, String)>>,
    /// Paths just dropped on an editor, for the command that turns them into
    /// pictures.
    ///
    /// The editor widget reports a drop; inserting needs the project's media
    /// directory, an `Asset` row and an undo stack, none of which an editor
    /// builder has. So the paths are parked here — per-window, like everything
    /// else on this view-model — and a global command picks them up, reusing the
    /// same pipeline `Insert image…` runs.
    dropped_files: Signal<Vec<std::path::PathBuf>>,

    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
    strikethrough: Signal<bool>,
    /// Superscript and subscript are one tri-state property in the document,
    /// mirrored as two signals because the toolbar shows two buttons. They are
    /// never both true.
    superscript: Signal<bool>,
    subscript: Signal<bool>,
    /// The caret sits on a hyperlink, so the Link command edits one rather
    /// than making one. Mirrored like every other mark, and read the same way:
    /// from the editor after each command, never flipped optimistically.
    link: Signal<bool>,
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
    /// Whether a *caret* is in an editor, as opposed to merely a document tab
    /// being open.
    ///
    /// [`Self::has_target`] answers the broader question and is right for the
    /// commands that act on a whole document (Save as template…, the Format
    /// menu): it includes the resolver's tab-scoped answer, which reports the
    /// active tab's prose handle whether or not anyone has ever clicked into it.
    ///
    /// Inserting *at the caret* needs more than that. This one is true only
    /// when a registered editor is live-focused, or was — the sticky latch,
    /// never the bare resolver fallback. So it survives the focus loss that
    /// opening the menu causes (the whole reason `has_target` is sticky, and the
    /// regression commit 2d4890b0 fixed), while a tab the writer has opened and
    /// never typed in still reads false.
    ///
    /// It cannot distinguish "the menu took focus" from "the binder took focus"
    /// — both look like *nothing* focused from here — so it stays true once the
    /// writer has been in the editor. That is the same limit `has_target` and
    /// the note gate live with, and for the same reason.
    has_caret_target: Signal<bool>,

    /// Per-group visibility, pushed by [`Self::set_surface`].
    ///
    /// Plain signals rather than maps over [`Self::surface`] for the same
    /// reason the mirrors are: `visible_when` takes a `Prop`, which *observes*,
    /// and observing a derived signal panics. Deriving these would look tidier
    /// and blow up the first time a group changed.
    group_visible: GroupVisibility,

    /// Last `(format_version, cursor_position, history_allowed)` seen by
    /// [`Self::refresh`], so a per-frame call is nearly free when nothing has
    /// moved. [`NEVER_SEEN`] when the mirrors hold nothing worth comparing
    /// against.
    last_seen: Rc<Cell<(u64, usize, bool)>>,
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

    /// The inline image the writer last clicked — see [`Self::active_image`].
    pub fn active_image(&self) -> Signal<Option<(usize, String)>> {
        self.active_image.clone()
    }

    /// Paths just dropped on an editor — see [`Self::dropped_files`].
    pub fn dropped_files(&self) -> Signal<Vec<std::path::PathBuf>> {
        self.dropped_files.clone()
    }

    /// Record a click on an inline image, or clear the record.
    pub fn set_active_image(&self, image: Option<(usize, String)>) {
        self.active_image.set(image);
    }

    /// A view-model with nothing to format yet. `App` calls [`Self::attach`]
    /// once the editors exist.
    pub fn detached() -> Self {
        Self {
            resolve: Rc::new(RefCell::new(None)),
            registry: Rc::new(RefCell::new(Vec::new())),
            sticky: Rc::new(RefCell::new(None)),
            dock_overlay_open: Signal::new(false),
            surface: Signal::new(FormatSurface::None),
            active_image: Signal::new(None),
            dropped_files: Signal::new(Vec::new()),
            bold: Signal::new(false),
            italic: Signal::new(false),
            underline: Signal::new(false),
            strikethrough: Signal::new(false),
            superscript: Signal::new(false),
            subscript: Signal::new(false),
            link: Signal::new(false),
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
            has_caret_target: Signal::new(false),
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
                footnotes: None,
                typo: None,
                item: None,
                scope: None,
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
    pub fn refocus(&self, ctx: &mut teksilo::prelude::EventContext) {
        if let Some(handle) = self.handle() {
            handle.focus(ctx);
        }
    }

    fn handle(&self) -> Option<EditorHandle> {
        self.target().0
    }

    /// Give an already-registered editor its footnote door. Separate from
    /// [`register`](Self::register) because only some surfaces have one, and
    /// they learn it from a different source than their handle.
    pub fn set_registered_footnotes(
        &self,
        id: WidgetId,
        binding: crate::footnotes::FootnoteBinding,
    ) {
        if let Some(entry) = self.registry.borrow_mut().iter_mut().find(|e| e.id == id) {
            entry.footnotes = Some(binding);
        }
    }

    /// Give an already-registered editor its typography bundle, so the size
    /// commands can resize it. Separate from [`register`](Self::register) for the
    /// same reason as the footnote door above: only some registered surfaces have
    /// one, and forcing a parameter would make the search preview — which has no
    /// typography at all — invent a bundle it does not use.
    pub fn set_registered_typography(&self, id: WidgetId, typo: crate::settings::EditorTypography) {
        if let Some(entry) = self.registry.borrow_mut().iter_mut().find(|e| e.id == id) {
            entry.typo = Some(typo);
        }
    }

    /// Tell an already-registered editor which `BinderItem`'s text it is showing,
    /// and which writing surface built it.
    ///
    /// Separate from [`register`](Self::register) for the same reason as the two
    /// above: not every registered surface is showing one item's text, and the
    /// widget tests would have to invent an anchor they do not have.
    ///
    /// **Both halves or neither.** This used to take the item alone, and the
    /// surface was inferred by [`handle_for_item`](Self::handle_for_item) from
    /// registration order — which is wrong wherever more than one live editor
    /// shows the same item, and three ordinary arrangements do. See
    /// [`LaneAnchor`](crate::margin_lane::LaneAnchor).
    pub fn set_registered_anchor(&self, id: WidgetId, anchor: crate::margin_lane::LaneAnchor) {
        if let Some(entry) = self.registry.borrow_mut().iter_mut().find(|e| e.id == id) {
            entry.item = Some(anchor.item);
            entry.scope = Some(anchor.scope);
        }
    }

    /// The mounted editor showing `item`'s text, if one is.
    ///
    /// **Not focus-shaped**, unlike every other read here, and that is the point:
    /// a margin lane converts offsets for every row on screen at once, most of
    /// which do not have focus and never will.
    ///
    /// `kind` disambiguates the surfaces that show the same item twice in different
    /// roles — a scene tab has both a prose column and a synopsis box, and a lane on
    /// one must not resolve its offsets against the other.
    ///
    /// **`scope` is the answer, not a hint.** One item can have several editors of
    /// the same kind mounted at once, and it is not one exotic arrangement but
    /// three ordinary ones: the dual-pane tab builds its prose column for both the
    /// Top and the Side synopsis layout; a scene open in a tab is also a row of the
    /// Full Chapter in the other half of the split editor; the search preview band
    /// is a third. All of them are live, all of them are laid out, and all of them
    /// share one `FormatViewModel` — it is per window.
    ///
    /// So when the caller names a scope, only an editor from *that* surface may
    /// answer, and `None` is the honest reply when it has not been built yet. A
    /// lane already handles `None` correctly — it is the normal state of every row
    /// below the fold, and the one thing it must never do is substitute a position
    /// from somewhere else. Falling back across scopes is precisely the bug: the
    /// Side page's lane spent the life of the tab converting offsets against the
    /// Top arm's frozen geometry, and the gap above the first paragraph moved with
    /// a column nobody was looking at.
    ///
    /// **Within a scope, a laid-out editor still wins over one merely registered.**
    /// That tie-break was the whole of the old rule and it was never wrong, only
    /// insufficient: a handle with no geometry is not a wrong position but no
    /// position at all, silently and for the life of the tab. It is kept, scoped.
    ///
    /// `scope: None` is the unscoped legacy answer — first laid-out registration
    /// wins — and exists for the editors the widget tests build with no surface
    /// around them. Nothing in the application passes it.
    /// A view of `item`'s **document**, for a caller that wants the text rather than a
    /// position in it.
    ///
    /// Scope-free, deliberately, and safe only for that. Every mounted view of one row is a
    /// view on the same `TextDocument`: `OpenDocsStore` holds one refcounted `OpenDoc` per
    /// item, and the tab page, the Top and Side arms, a stream row and the search preview
    /// band are all handed that same document. So the text and its version are identical
    /// whichever registration answers, and naming a surface would be answering a question
    /// that has no bearing on the result.
    ///
    /// A dock has no surface to name in any case. [`LaneScope`](crate::margin_lane::LaneScope)
    /// asks "which editor is this lane sitting beside", and a dock sits beside none of them.
    ///
    /// ⚠ Geometry is **not** shared, and this is the wrong door for it. Anything converting
    /// an offset into a rectangle has to name its own scope through
    /// [`Self::handle_for_item`], or it reads one column's layout while sitting against
    /// another, which is the drift that made scopes necessary in the first place.
    pub fn document_view(&self, item: EntityId, kind: EditorKind) -> Option<EditorHandle> {
        self.registry
            .borrow()
            .iter()
            .find(|e| e.item == Some(item) && e.kind == kind)
            .map(|e| e.handle.clone())
    }

    pub fn handle_for_item(
        &self,
        item: EntityId,
        kind: EditorKind,
        scope: Option<crate::margin_lane::LaneScope>,
    ) -> Option<EditorHandle> {
        let registry = self.registry.borrow();
        let mut matching = registry
            .iter()
            .filter(|e| e.item == Some(item) && e.kind == kind)
            .filter(|e| scope.is_none() || e.scope == scope)
            .map(|e| &e.handle);
        let first = matching.next()?;
        if first.content_height().is_some() {
            return Some(first.clone());
        }
        Some(
            matching
                .find(|h| h.content_height().is_some())
                .unwrap_or(first)
                .clone(),
        )
    }

    /// Every mounted editor of `kind`, by the item it is showing.
    ///
    /// The bulk read a surface mapping many rows at once could take, one call rather
    /// than one lookup per row. Nothing in the application uses it: the margin lane
    /// asks per row through [`handle_for_item`](Self::handle_for_item), because it
    /// only ever asks about rows that have been *placed* — which is a much smaller
    /// set than every registered editor, and because the answer has to be narrowed
    /// to the asking surface, which this cannot do.
    pub fn handles_by_item(&self, kind: EditorKind) -> Vec<(EntityId, EditorHandle)> {
        self.registry
            .borrow()
            .iter()
            .filter(|e| e.kind == kind)
            .filter_map(|e| e.item.map(|item| (item, e.handle.clone())))
            .collect()
    }

    /// The typography bundle Ctrl+= / Ctrl+− / Ctrl+0 should resize: the one
    /// dressing the focused editor, or — reaching the command through the View
    /// menu, which takes focus away before the action runs — the last editor that
    /// had it.
    ///
    /// Built on [`resolved_registration`](Self::resolved_registration) rather
    /// than a second latch of its own, so the size commands and the mark commands
    /// can never disagree about which editor the writer is in. `None` when
    /// nothing has ever been focused, or when the resolved surface carries no
    /// size preference (the search preview band) — both mean "nothing to
    /// resize", and the commands no-op.
    pub fn focused_typography(&self) -> Option<crate::settings::EditorTypography> {
        let (id, _, _) = self.resolved_registration()?;
        self.registry
            .borrow()
            .iter()
            .find(|e| e.id == id)
            .and_then(|e| e.typo.clone())
    }

    /// The editor a footnote reference goes into, and the `Content` row behind it.
    ///
    /// Resolved through the **registry**, not through the focused tab, and that
    /// distinction is the whole of it: a tab's own handle is whatever
    /// `writing_column` last attached to the find banner, which in a stream is
    /// the container's prose rather than the row the writer is in — and which
    /// reports `focused = false` and a caret of 0 for an editor nobody is
    /// typing in. Inserting through it put every marker at the top of the
    /// document regardless of where the caret was.
    ///
    /// `target()`'s sticky latch is what makes this survive the menu: reaching
    /// Document ▸ Insert footnote moves focus to the overlay, so "the focused
    /// editor" is momentarily nothing at all.
    ///
    /// `None` for a **synopsis**: it is planning text, and a note attached there
    /// prints into a synopsis export and nowhere in the book.
    pub fn footnote_target(&self) -> Option<(EditorHandle, crate::footnotes::FootnoteBinding)> {
        let (id, handle, kind) = self.resolved_registration()?;
        if kind == EditorKind::Synopsis {
            return None;
        }
        let binding = self
            .registry
            .borrow()
            .iter()
            .find(|e| e.id == id)
            .and_then(|e| e.footnotes.clone())?;
        Some((handle, binding))
    }

    /// The registered editor a command should act through: the focused one, else
    /// the sticky latch. The `WidgetId` is kept, unlike in [`target`](Self::target),
    /// so a caller can look the rest of its registration up.
    fn resolved_registration(&self) -> Option<(WidgetId, EditorHandle, EditorKind)> {
        if let Some(live) = self.focused_registered() {
            // Latch it, exactly as `target` does. Without this the menu path
            // would work only when some *other* format query happened to run
            // while the editor still had focus — true today, because the dock
            // polls, and a silent dependency on that is how a command comes to
            // work in one window arrangement and not another.
            *self.sticky.borrow_mut() = Some(live.clone());
            return Some(live);
        }
        self.sticky.borrow().clone()
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
    pub fn link(&self) -> Signal<bool> {
        self.link.clone()
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
    /// Is a registered editor holding the keyboard focus **right now**?
    ///
    /// Deliberately the live answer, not [`has_caret_target`](Self::has_caret_target)'s
    /// latched one. The latch exists so a *command* still knows what to act on
    /// after opening a menu blurs the editor; deciding which undo domain is
    /// active is the opposite question, and a latch there would keep routing
    /// Ctrl+Z to prose after the writer has clicked into the binder.
    pub fn editor_focused(&self) -> bool {
        self.focused_registered().is_some()
    }

    /// Is the target editor refusing to step through its history — the
    /// "Always forward" writing game?
    ///
    /// Distinct from `!can_undo()`, which folds this together with "there is
    /// nothing to undo". A router has to tell them apart: an empty history may
    /// fall through to another domain, a frozen one must not, or the game would
    /// simply redirect Ctrl+Z instead of refusing it.
    pub fn history_frozen(&self) -> bool {
        self.handle().is_some_and(|h| !history_allowed(&h))
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

    /// Whether a caret is in an editor — see [`Self::has_caret_target`].
    pub fn has_caret_target(&self) -> Signal<bool> {
        self.has_caret_target.clone()
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

    /// Report that an overlay the dock owns has opened or closed — wired to
    /// `PopoverWidget::on_open` / `on_close` by the control that owns it, and
    /// reset to `false` as that control is (re)built so a dock torn down with
    /// its popover up cannot leave the surface latched.
    ///
    /// See [`Self::dock_overlay_open`] for what it buys and why a `focusable`
    /// button is not enough.
    pub fn set_dock_overlay_open(&self, open: bool) {
        set_if_changed(&self.dock_overlay_open, open);
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
        let (handle, resolved) = self.target();
        // A popover the dock itself opened holds the keyboard focus, so the
        // resolver truthfully answers "nothing focused". Keep showing what we
        // were showing rather than hiding the group the writer just reached
        // into. Only the *empty* answer is overridden — a different live
        // surface is a real move and has to win.
        let surface = if resolved.is_empty() && self.dock_overlay_open.get() {
            self.surface.get()
        } else {
            resolved
        };
        self.set_surface(surface);
        set_if_changed(&self.has_target, handle.is_some());
        // Narrower than `has_target` by exactly the resolver fallback: the latch
        // is only ever written by a genuinely focused registered editor, so its
        // presence means the writer has put a caret in one.
        set_if_changed(&self.has_caret_target, self.sticky.borrow().is_some());

        let Some(handle) = handle else {
            self.clear_mirrors();
            return;
        };

        let version = handle.format_version().get();
        let caret = handle.cursor_position_signal().get();
        let history = history_allowed(&handle);
        if (version, caret, history) == self.last_seen.get() {
            return;
        }
        self.last_seen.set((version, caret, history));
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
                history_allowed(&handle),
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
        set_if_changed(&self.link, handle.is_link());
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
        // Undo/redo are gated on the editor's own command filter as well as on
        // whether there is history to step through: a writing game
        // ("Always forward") refuses them. These two mirrors are read by
        // `edit::ProseDomain`, and through it by the Edit menu's rows and the
        // Ctrl+Z router — so folding the rule in here is what makes the menu go
        // quiet the moment the focused editor is frozen, rather than offering a
        // step the editor will silently refuse.
        let history = history_allowed(handle);
        set_if_changed(&self.can_undo, handle.can_undo().get() && history);
        set_if_changed(&self.can_redo, handle.can_redo().get() && history);
    }

    fn clear_mirrors(&self) {
        self.last_seen.set(NEVER_SEEN);
        set_if_changed(&self.bold, false);
        set_if_changed(&self.italic, false);
        set_if_changed(&self.underline, false);
        set_if_changed(&self.strikethrough, false);
        set_if_changed(&self.superscript, false);
        set_if_changed(&self.subscript, false);
        set_if_changed(&self.link, false);
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

    // ── Hyperlinks ────────────────────────────────────────────────────────
    //
    // The Link command is the only formatting command that needs data from the
    // writer, so it is the only one split in two: the dialog asks, and these
    // apply. What the dialog needs to *ask* is resolved by
    // [`link_request`](Self::link_request), so the panel never has to reach for
    // an editor handle itself.

    /// What the Link dialog should open with.
    ///
    /// Three cases, in the order they take precedence: the caret is inside a
    /// link (edit it, whole extent), the writer has selected some text (link
    /// that, pre-filling the name), or neither (insert a fresh link).
    ///
    /// `None` when there is no editor to act on at all — a menu can outlive
    /// the editor it was opened over.
    pub fn link_request(&self) -> Option<LinkRequest> {
        let handle = self.handle()?;
        if let Some(extent) = handle.link_at_caret() {
            return Some(LinkRequest {
                name: extent.text,
                href: extent.href,
                editing: true,
            });
        }
        Some(LinkRequest {
            name: handle.selected_text(),
            href: String::new(),
            editing: false,
        })
    }

    /// Write the link the dialog collected.
    ///
    /// Applied as a character format over a range rather than by inserting
    /// `[name](href)` markup: that keeps any bold or italic already on the
    /// words, and means neither the name nor the destination has to be escaped
    /// for a markup parser that never sees them.
    ///
    /// The text is only rewritten when the writer actually changed the name —
    /// re-typing the same string would otherwise churn the document and cost
    /// the run its other formatting for nothing.
    pub fn apply_link(&self, name: &str, href: &str) {
        let Some(handle) = self.handle() else {
            self.clear_mirrors();
            return;
        };

        // Where the link goes: over the link already there, else over the
        // selection, else at the bare caret.
        let (start, end, current) = match handle.link_at_caret() {
            Some(extent) => (extent.start, extent.end, extent.text),
            None => {
                let (anchor, position) = handle.selection();
                (
                    anchor.min(position),
                    anchor.max(position),
                    handle.selected_text(),
                )
            }
        };

        // One undo entry for the pair, so a writer who changed both the name
        // and the destination undoes one link edit rather than two halves.
        handle.edit_block(|| {
            let end = if name == current {
                end
            } else {
                // `replace_range` on an empty range is an insert, which is
                // exactly what a bare caret needs — no separate branch.
                handle.replace_range(start, end, name);
                start + name.chars().count()
            };
            handle.select_range(start, end);
            handle.set_link(href);
        });
        self.sync_now();
    }

    /// Take the link off the caret's link, leaving its words.
    ///
    /// Selects the extent first: a collapsed caret formats nothing, so
    /// clearing without selecting is a silent no-op — the trap the image
    /// commands already work around the same way.
    pub fn remove_link(&self) {
        let Some(handle) = self.handle() else {
            self.clear_mirrors();
            return;
        };
        let Some(extent) = handle.link_at_caret() else {
            return;
        };
        handle.select_range(extent.start, extent.end);
        handle.clear_link();
        self.sync_now();
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
            // A link is formatting too, and "plain prose" is exactly what a
            // writer stripping formatting is asking for. Unlike the marks it
            // clears over the *link's* own extent, not the selection: the
            // caret may be sitting in a link without selecting all of it.
            if let Some(extent) = handle.link_at_caret() {
                let (anchor, position) = handle.selection();
                handle.select_range(extent.start, extent.end);
                handle.clear_link();
                handle.select_range(anchor, position);
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

    /// Step the **focused editor's own** history.
    ///
    /// `edit::ProseDomain`'s implementation, and the **only** door this
    /// view-model has onto undo. Nothing in the Format surfaces offers Undo of
    /// its own: routing is the undo group's job, because only a surface that
    /// can render a label may take a fall-through step (see
    /// `UndoGroupViewModel::route`).
    ///
    /// The `command_filter` gate is therefore load-bearing rather than
    /// belt-and-braces: it is what makes `ProseDomain::frozen()` mean something,
    /// and what stops the writing game being escapable through the router.
    pub fn undo_editor(&self) {
        self.with_editor(|h| {
            if h.command_filter()
                .accepts(teksilo::widgets::rich_text::EditCommandKind::Undo)
            {
                h.undo();
            }
        });
    }

    /// See [`undo_editor`](Self::undo_editor).
    pub fn redo_editor(&self) {
        self.with_editor(|h| {
            if h.command_filter()
                .accepts(teksilo::widgets::rich_text::EditCommandKind::Redo)
            {
                h.redo();
            }
        });
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
mod tests;
