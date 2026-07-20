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

use std::cell::Cell;
use std::rc::Rc;

use bastyde::prelude::Signal;
use bastyde::text_document::Alignment;
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

/// A blockquote's nesting depth is not queryable through `EditorHandle`, so
/// [`FormatViewModel::clear_formatting`] unwraps one level at a time and stops
/// here. Deeper than this in a manuscript is a corrupt document, not a style.
const MAX_BLOCKQUOTE_UNWRAP: usize = 16;

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

/// Resolves the editor the formatting commands should act on, at the moment
/// they are invoked. `None` when nothing formattable is focused.
type ResolveEditor = Rc<dyn Fn() -> Option<EditorHandle>>;

/// The state and command surface behind the format dock, the Format menu and
/// the context-menu row. Cloneable; every clone shares one set of signals.
#[derive(Clone)]
pub struct FormatViewModel {
    /// How to find the current editor. Called fresh on every command and every
    /// [`Self::refresh`] — see the module docs on handle staleness.
    resolve: ResolveEditor,
    /// Which groups apply. Written by `App` as focus moves; read by the dock to
    /// decide what to show and by the menu to decide what to enable.
    surface: Signal<FormatSurface>,

    bold: Signal<bool>,
    italic: Signal<bool>,
    underline: Signal<bool>,
    strikethrough: Signal<bool>,
    blockquote: Signal<bool>,
    /// The caret sits inside a table, so the row/column commands are meaningful.
    in_table: Signal<bool>,
    /// `0` = normal paragraph, `1..=6` = H1..H6. Doubles as the radio index.
    heading: Signal<usize>,
    /// [`ALIGN_LEFT`], [`ALIGN_CENTER`] or [`ALIGN_OTHER`].
    alignment: Signal<usize>,
    can_undo: Signal<bool>,
    can_redo: Signal<bool>,

    /// Last `(format_version, cursor_position)` seen by [`Self::refresh`], so a
    /// per-frame call is nearly free when nothing has moved.
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
    pub fn new(resolve: ResolveEditor) -> Self {
        Self {
            resolve,
            surface: Signal::new(FormatSurface::None),
            bold: Signal::new(false),
            italic: Signal::new(false),
            underline: Signal::new(false),
            strikethrough: Signal::new(false),
            blockquote: Signal::new(false),
            in_table: Signal::new(false),
            heading: Signal::new(0),
            alignment: Signal::new(ALIGN_LEFT),
            can_undo: Signal::new(false),
            can_redo: Signal::new(false),
            last_seen: Rc::new(Cell::new((0, 0))),
        }
    }

    // ── The current editor ────────────────────────────────────────────────

    /// The editor to act on right now, or `None`.
    fn handle(&self) -> Option<EditorHandle> {
        (self.resolve)()
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
    pub fn can_undo(&self) -> Signal<bool> {
        self.can_undo.clone()
    }
    pub fn can_redo(&self) -> Signal<bool> {
        self.can_redo.clone()
    }

    /// The current surface. `App` is the only writer — it knows which pane and
    /// which tab the focus landed in; this view-model deliberately does not.
    pub fn set_surface(&self, surface: FormatSurface) {
        if self.surface.get() != surface {
            self.surface.set(surface);
        }
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
        let Some(handle) = self.handle() else {
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
        set_if_changed(&self.blockquote, handle.is_in_blockquote());
        set_if_changed(&self.in_table, handle.is_in_table());
        set_if_changed(&self.heading, handle.get_heading_level() as usize);
        set_if_changed(&self.alignment, alignment_index(&handle.get_alignment()));
        set_if_changed(&self.can_undo, handle.can_undo().get());
        set_if_changed(&self.can_redo, handle.can_redo().get());
    }

    fn clear_mirrors(&self) {
        self.last_seen.set((0, 0));
        set_if_changed(&self.bold, false);
        set_if_changed(&self.italic, false);
        set_if_changed(&self.underline, false);
        set_if_changed(&self.strikethrough, false);
        set_if_changed(&self.blockquote, false);
        set_if_changed(&self.in_table, false);
        set_if_changed(&self.heading, 0);
        set_if_changed(&self.alignment, ALIGN_LEFT);
        set_if_changed(&self.can_undo, false);
        set_if_changed(&self.can_redo, false);
    }

    // ── Character marks ───────────────────────────────────────────────────
    //
    // Each runs the real command then re-syncs, so the mirrors carry the
    // editor's answer rather than an assumption. Superscript and subscript are
    // absent on purpose: `MergeTextFormatDto` does not carry
    // `vertical_alignment`, so applying them today succeeds and changes
    // nothing. They arrive with that upstream fix, not before.

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

    /// Strip formatting back to plain prose.
    ///
    /// Clears the four character marks over the selection, then flattens the
    /// caret's blocks: heading to normal, alignment to left, blockquote
    /// unwrapped to depth zero. Without a selection the character half is
    /// inherently a no-op — there is no range to re-format — so this degrades
    /// to the block half, which is still worth having with the caret parked in
    /// a centred H2.
    ///
    /// Each property is read before it is written, so clearing already-clean
    /// text neither pushes undo entries nor marks the document modified.
    ///
    /// Three things it does **not** clear, each because the API cannot:
    /// list membership (`outdent` bottoms out at depth 0 and the real primitive
    /// is not public), font family (no layer can express "unset" — `None` means
    /// "leave unchanged" everywhere), and superscript/subscript (unreachable,
    /// as above). The tooltip says so rather than implying a clean sweep.
    ///
    /// Each cleared property is its own undo entry until
    /// `begin_edit_block`/`end_edit_block` reach `EditorHandle`; wrapping this
    /// body in that pair is the whole change when they do.
    pub fn clear_formatting(&self) {
        let Some(handle) = self.handle() else {
            return;
        };

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
        if handle.get_heading_level() != 0 {
            handle.set_heading_level(0);
        }
        if handle.get_alignment() != Alignment::Left {
            handle.set_alignment(Alignment::Left);
        }
        // Depth is not queryable, so unwrap one level at a time and bound the
        // loop — a command that cannot make progress must still terminate.
        let mut unwrapped = 0;
        while handle.is_in_blockquote() && unwrapped < MAX_BLOCKQUOTE_UNWRAP {
            handle.decrease_blockquote_depth();
            unwrapped += 1;
        }

        self.sync_now();
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
        let doc = TextDocument::new();
        doc.set_markdown(text)
            .expect("parse")
            .wait()
            .expect("import");
        let editor = RichTextEditor::editor(doc);
        let handle = editor.handle();
        let vm = FormatViewModel::new(Rc::new(move || Some(handle.clone())));
        (vm, editor)
    }

    /// A view-model with nothing focused.
    fn vm_detached() -> FormatViewModel {
        FormatViewModel::new(Rc::new(|| None))
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
        let vm = FormatViewModel::new(Rc::new(move || gate.get().then(|| handle.clone())));

        vm.refresh();
        assert!(vm.bold().get());

        flip.set(false);
        vm.refresh();
        assert!(!vm.bold().get(), "mirrors clear when the editor goes away");
    }
}
