// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Settings modal — Skribisto's full preferences window.
//!
//! Presented as an in-tree modal (see the `app.settings` action in `app.rs`). A
//! two-pane layout mirroring the design (IntelliJ Int UI vocabulary): a left
//! **category [`TreeView`](teksilo::widgets::TreeView)** (with a
//! [`SearchField`](teksilo::widgets::SearchField) on top) and a right
//! **settings pane** that switches with the tree selection, breadcrumb header +
//! action footer. The header/footer chrome matches the Welcome and New Work
//! panels.
//!
//! All state lives on [`SettingsViewModel`] (persisted signals) plus the
//! framework's drop-in [`ThemeSwitcher`] / [`LanguageSwitcher`] / [`TextScaleControl`]
//! (theme / interface language / text scale — each applies live and persists).
//! The panes bind those signals directly; this view is thin.
//!
//! A per-project page with no project open renders an **empty placeholder** (the
//! `Work:` section's rows are absent from the tree in that state, but the pages
//! stay in the `Switcher`). Keymap embeds Teksilo's `ShortcutSettings`;
//! Notifications embeds the toast archive `NotificationLog`.
//!
//! A **parent** — a section, or the nested Typography group — is a page in its own
//! right: its title, what it is for, and a link to each thing under it
//! (`panes::overview`). Selecting one used to switch nothing at all, so the right
//! pane went on showing whichever leaf was open last and the window disagreed with
//! its own tree.
//! **Instant-apply** (the macOS / GNOME convention): every change takes effect and
//! persists immediately, so there is no Apply/Cancel/OK staged model. The footer
//! carries only *Reset to defaults* (left — enabled only while something differs
//! from the factory defaults, and guarded by a confirmation since there is no undo)
//! and *Done* (right — closes the window). The header ✕ closes it too.
//!
//! ## Three ways out, all kept, and why
//!
//! Escape (the modal's [`ModalCloseBehavior::EscapeKey`]), the header ✕ and the
//! footer's *Done* all do the identical thing — `ctx.dismiss_modal()` — and that
//! is not a redundancy to trim. Instant-apply is what makes them safe: there is
//! nothing staged to discard, so "close" carries no decision and no keystroke
//! can lose work. Each serves a different reader: **Escape** is what a keyboard
//! user reaches for and what this window owed and did not have; the **✕** is the
//! chrome every other panel in the app carries in the same corner; **Done** is
//! the affordance a mouse user looks for at the end of a dialog, and it is a
//! button rather than a 16 px glyph in the far corner. Deleting either of the
//! latter two to leave "one way" would remove the discoverable one, not the
//! duplicate.
//!
//! Deliberately **not** [`ModalCloseBehavior::EscapeOrClickOutside`]: the window
//! hosts uncommitted text (the rail's search field, inline tag/status names, the
//! Paratext TOML overlay), and a stray click beside a card this size is easy to
//! land by accident. Escape is what is owed; click-outside is not.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::core::widget::WidgetPlacement;
use teksilo::i18n::{LocalizedString, current_locale};
use teksilo::prelude::*;
use teksilo::settings::{SettingsExt, TEXT_SCALE_KEY};
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, FontPicker, FormLayout, HStack, IconButton,
    LanguageSwitcher, MessageBox, MessageBoxButton, MessageBoxButtons, Padding, Panel, RadioButton,
    RadioGroup, Slider, Spacer, StandardButton, TextScaleControl, TextWidget, ThemeSwitcher,
    Toggle, VStack,
};

mod content;
mod defaults;
mod fields;
mod nav;
pub(crate) mod panes;
mod settings_vm;
mod text_replacement_rules_vm;
mod tree;
mod tree_expansion_vm;
mod work_settings_vm;

// Re-bound here, not merely `use`d, because every pane module reaches this file
// with `use super::super::*` — a glob that carries a private import along with
// the rest. Naming them explicitly is what keeps that glob honest: the list
// below is the settings window's whole internal vocabulary.
pub(crate) use defaults::build_not_defaults;
pub(crate) use fields::{
    Crumbs, empty_pane, field_label, group, hint, index_to_method, method_to_index, pane_frame,
    section_title, slider_field, slider_field_tipped,
};
pub(crate) use nav::{Navigator, Pane, Sec, tree_spec};

pub use settings_vm::{
    CorkboardDefaults, EditorTypography, EditorTypographySet, EditorViewMemory, SettingsViewModel,
    TypographyKind, TypographySizeRange,
};
pub use text_replacement_rules_vm::TextReplacementRulesViewModel;
pub use tree_expansion_vm::TreeExpansionViewModel;
pub use work_settings_vm::WorkSettingsViewModel;

use crate::sessions::WorkSession;
use crate::shared::{HighlightScope, TypewriterAnchor};
use skribisto_model::ChapterMode;

/// The card's **preferred** dimensions (a compact two-pane preferences window).
///
/// Preferred, not fixed: [`SettingsPanel::card_size`] shrinks the card into
/// whatever the window actually offers. Project windows declare a
/// `.min_size(800, 600)` (`shell/windows.rs`), and a 620-tall card in a 600-tall
/// window used to lose the bottom 20 px of the footer band — *Done* clipped in
/// half, on every 1366×768 @125 % and 1920×1080 @175 % desktop as well.
const CARD_W: f32 = 920.0;
const CARD_H: f32 = 620.0;
/// The floor the card will not shrink past: below this the two-pane layout stops
/// being a two-pane layout, and clipping is preferable to a 40 px content column.
const MIN_CARD_W: f32 = 560.0;
const MIN_CARD_H: f32 = 360.0;
/// Breathing room left between the card and the window edge once the card has had
/// to shrink at all — the gutter that says "this is a window over a window".
const VIEWPORT_MARGIN: f32 = 32.0;
const HEADER_H: f32 = 44.0;
const FOOTER_H: f32 = 56.0;
const TREE_W: f32 = 262.0;
/// The vertical rule between the rail and the pane.
const RULE_W: f32 = 1.0;
/// The default text-scale factor (framework `TEXT_SCALE_KEY` baseline).
const TEXT_SCALE_DEFAULT: f32 = 1.0;

/// The pane column's width for a card `card_w` wide — everything the rail and the
/// rule between them leave.
///
/// Bound explicitly onto the pane's `FixedSize` rather than left to an
/// `Expand::horizontal`, which forwards `width: None`: `ScrollArea` measured with
/// no width offered takes its `natural_content_width` branch and lays the *whole*
/// page subtree out unbounded (a user-shaped pane measures 1472 px, a
/// paratext-shaped one 1808 px). Those pages placed correctly all the same only
/// because `Expand` reports a zero flex basis when its parent leaves width open —
/// so one `.respect_intrinsic()`, or one refactor dropping the `Expand`, would
/// have blown the card open, and every layout pass paid for a full extra
/// unbounded walk per pane in the meantime.
fn pane_width(card_w: f32) -> f32 {
    (card_w - TREE_W - RULE_W).max(0.0)
}

/// Present the settings window, however it was reached.
///
/// One door for all six entry points (Work ▸ Settings and its Ctrl+, shortcut,
/// the games dock, the Keymap sheet, the missing-dictionary toast, the unsigned-
/// comments toast, the no-backup nudge), so the presentation contract is stated
/// once. It used to be six copies of `.title("Settings").size(920, 620)
/// .close_behavior(Manual)` — a size that is dead under
/// [`ModalPresentation::InTree`] (and, now that the card sizes itself to the
/// window, actively misleading), and an English literal at five of the six.
///
/// [`ModalCloseBehavior::EscapeKey`], not `Manual`: see this module's "Three ways
/// out" section.
pub fn present(ctx: &mut EventContext, open: impl FnOnce() -> SettingsPanel + 'static) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| t.add(open()))
            .presentation(ModalPresentation::InTree)
            .title(tr!(settings_title()))
            .close_behavior(ModalCloseBehavior::EscapeKey),
    );
}

pub struct SettingsPanel {
    /// The active page — a panel field so it survives rebuilds (and seeds the
    /// tree selection + the content `Switcher`). Defaults to Editor ▸ Scene.
    selected_pane: Signal<Pane>,
    /// The rail's search field, captured for [`Widget::initial_focus_hint`] —
    /// see that method for why the header ✕ must not have it.
    search_field: Option<WidgetId>,
    /// The pane column's live width, written from [`Widget::layout_response`]
    /// (which is the only hook that learns how much room the window really has)
    /// and read by the pane's `FixedSize` in the same pass — `FixedSize` resolves
    /// its `Prop` at layout time, so there is no stale frame.
    ///
    /// A plain `Signal`, not a `Signal::map` of a card-width one: a derived signal
    /// is read-only and `observe()` panics on it, and a `Prop` observes.
    pane_w: Signal<f32>,
    /// The Tier-2 bundle for the Work the OPENING window shows, or `None` when
    /// the window that opened this one holds no project at all.
    ///
    /// Never resolve the equivalent state via `ctx.app_state::<SingleWork/
    /// SingleWorkInfo/AppIds/TagsViewModel/UserDictionaryViewModel>()` — that
    /// slot is one per process, seeded from the first window's session, so a
    /// second open Work would silently read/write the wrong project's
    /// settings. Same pattern as `SaveAsViewModel`/`BackupRestoreViewModel`.
    ///
    /// **`None` is the Launcher**, which is where this app *starts* on Linux and
    /// Windows: it is a real window with a real menu bar and no `WorkSession`
    /// behind it. Withholding the whole window from it — which is what a
    /// non-optional field forced — left theme, interface text scale,
    /// dictionaries, keybindings and the backup defaults unreachable until the
    /// writer had created or opened a project, i.e. exactly in the first-run
    /// state where they are most likely to be wanted. Every app-level page is
    /// live in that state; the Work section is absent from the tree
    /// (`tree_spec(has_work: false, ..)`) and its pages render their existing
    /// no-project placeholder.
    session: Option<WorkSession>,
    /// Held for as long as this panel is presented, which takes the window's
    /// Undo out of play.
    ///
    /// Ctrl+Z over a Settings checkbox must not reach the manuscript. A text
    /// *field* in here answers for itself — the framework's text-surface
    /// registry sees it like any other — but the rest of the panel is not a
    /// text surface at all, so without this the group would fall back to
    /// whatever the writer was last doing in the project behind the modal.
    ///
    /// `None` for the Launcher's window, which has no project behind it and so
    /// no undo group to suspend.
    ///
    /// Dropped with the panel, which is when the modal closes.
    _undo_suspend: Option<crate::edit::UndoSuspend>,
}

/// Where the window lands when nobody asked for a particular page:
/// **Appearance & Behavior ▸ Appearance**.
///
/// Not Editor ▸ Scene, which is where this window used to land. Scene sits two
/// levels down (`Editor ▸ Typography ▸ Scene`), so revealing it re-expands both
/// of its ancestors — and a 33-row rail into a ~519 px viewport puts the whole
/// `Work: <title>` section below the fold on every single open, which is the one
/// part of the tree nothing else in the app links to. Landing on a page whose
/// only ancestor is an already-open section keeps the rail at 21 rows, so the
/// Work section is visible without scrolling.
///
/// Appearance rather than User (the other 21-row candidate) because it carries
/// theme, interface language and text scale — what a writer actually comes here
/// to change, and the only page that is useful in the no-project state
/// [`SettingsPanel::without_project`] now opens from.
///
/// A named constant rather than the literal repeated at the two constructors
/// below, because the invariant that keeps the rail short is a *relation*
/// between this and [`tree::DEFAULT_COLLAPSED`] — every ancestor of this pane
/// has to be a row that starts open — and a test can only pin a relation whose
/// two halves it can name. See `the_landing_pane_has_no_collapsed_ancestor`.
pub(crate) const DEFAULT_PANE: Pane = Pane::Appearance;

impl SettingsPanel {
    /// Opens on [`DEFAULT_PANE`] — Appearance & Behavior ▸ Appearance.
    pub fn new(session: WorkSession, undo: &crate::edit::UndoGroupViewModel) -> Self {
        Self::opening_at(DEFAULT_PANE, Some(session), Some(undo))
    }

    /// The Launcher's settings window — every app-level page, no Work section.
    ///
    /// A constructor of its own rather than `new(None)`: the six project-window
    /// doors all genuinely hold a session, and passing `Some(..)` at each of
    /// them to accommodate the one caller that cannot would put the burden on
    /// the wrong side.
    ///
    /// Lands on [`DEFAULT_PANE`] like [`Self::new`] — and here that is not just
    /// consistency: Appearance is the one page that is fully live with no
    /// project open.
    pub fn without_project() -> Self {
        Self::opening_at(DEFAULT_PANE, None, None)
    }

    /// Open straight to Spelling ▸ Dictionaries — the target of the "install the
    /// missing dictionaries" toast (`offer_missing_dictionaries`). The tree seeds
    /// its selection to that page and expands the owning section on open.
    pub fn open_to_dictionaries(
        session: WorkSession,
        undo: &crate::edit::UndoGroupViewModel,
    ) -> Self {
        Self::opening_at(Pane::Dictionaries, Some(session), Some(undo))
    }

    /// Open straight to Editor ▸ Writing games — the target of the games dock's
    /// own "Writing game settings…" button, which promises that page by name.
    pub fn open_to_games(session: WorkSession, undo: &crate::edit::UndoGroupViewModel) -> Self {
        Self::opening_at(Pane::Games, Some(session), Some(undo))
    }

    /// Open straight to Backup & Sync ▸ Backup — the target of the
    /// "no backups configured" nudge toast.
    pub fn open_to_backup(session: WorkSession, undo: &crate::edit::UndoGroupViewModel) -> Self {
        Self::opening_at(Pane::Backup, Some(session), Some(undo))
    }

    /// Open straight to Settings ▸ User — the target of the "your comments are
    /// unsigned" toast (`crate::app::warn_unsigned_comments`), which promises
    /// that page by name.
    pub fn open_to_user(session: WorkSession, undo: &crate::edit::UndoGroupViewModel) -> Self {
        Self::opening_at(Pane::User, Some(session), Some(undo))
    }

    /// Open straight to Keymap — the target of the Help ▸ Keyboard shortcuts sheet's
    /// own "Change shortcuts…" button, which promises that page by name. The sheet
    /// itself is read-only, so this is where a reader who wanted to *change* a chord
    /// rather than look one up ends up.
    pub fn open_to_keymap(session: WorkSession, undo: &crate::edit::UndoGroupViewModel) -> Self {
        Self::opening_at(Pane::Keymap, Some(session), Some(undo))
    }

    /// Open straight to Editor ▸ Typography ▸ Distraction-free themes — the target
    /// of the focus-mode quick-settings popover's own "Manage themes…" button.
    ///
    /// That popover exists precisely so a writer in focus mode is *not* made to
    /// open the full preferences window over their manuscript and go hunting; it
    /// fired the generic `app.settings` all the same, which lands on Editor ▸
    /// Scene typography — the full modal, on the wrong page, from inside the one
    /// surface built to avoid it.
    pub fn open_to_df_themes(session: WorkSession, undo: &crate::edit::UndoGroupViewModel) -> Self {
        Self::opening_at(Pane::DistractionFreeThemes, Some(session), Some(undo))
    }

    fn opening_at(
        pane: Pane,
        session: Option<WorkSession>,
        undo: Option<&crate::edit::UndoGroupViewModel>,
    ) -> Self {
        Self {
            selected_pane: Signal::new(pane),
            search_field: None,
            pane_w: Signal::new(pane_width(CARD_W)),
            session,
            _undo_suspend: undo.map(|g| g.suspend()),
        }
    }

    /// The card's real size inside the slot the modal overlay handed it.
    ///
    /// `OverlayPlacement::Centered` clamps an overlay's bounds to the viewport and
    /// then lays its content out at exactly those bounds, so `slot` *is* the
    /// viewport once the card no longer fits — which is what makes this stable:
    /// [`Widget::layout_response`] always reports the full [`CARD_W`]×[`CARD_H`]
    /// aspiration, so the slot handed back is `min(preferred, viewport)` on every
    /// pass rather than a size that shrinks by a margin each time round.
    fn card_size(slot_w: f32, slot_h: f32) -> (f32, f32) {
        fn fit(slot: f32, preferred: f32, floor: f32) -> f32 {
            if slot >= preferred {
                preferred
            } else {
                // Clipped by the window: give the gutter back, but never shrink
                // past the floor and never past the slot itself.
                (slot - VIEWPORT_MARGIN).max(floor).min(slot)
            }
        }
        (
            fit(slot_w, CARD_W, MIN_CARD_W),
            fit(slot_h, CARD_H, MIN_CARD_H),
        )
    }
}

impl std::fmt::Debug for SettingsPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SettingsPanel").finish()
    }
}

impl Widget for SettingsPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let vm = SettingsViewModel::new(ctx.settings());
        // Which surfaces a game covers is an app setting; whether it is being
        // played is this project's session state. Paired here, exactly as
        // `App::build` pairs them for the editors.
        //
        // With no project open there is no activation to bind to — no manuscript
        // to play the game in — so the pane is handed a detached `false` and told
        // it is not playable, which is what disables the switch. Binding a live
        // switch to a signal nothing reads is the one thing that must not happen
        // here: it would report a game as being played in a window that has no
        // editor to enforce it in.
        let games = crate::writing_session::WritingGamesViewModel::new(
            self.session
                .as_ref()
                .map(|s| s.always_forward.clone())
                .unwrap_or_else(|| Signal::new(false)),
            crate::writing_session::WritingGameOptions::new(
                vm.games_forward_prose(),
                vm.games_forward_synopsis(),
            ),
        );
        let scale = ctx.settings().signal_for(&TEXT_SCALE_KEY);
        let theme_sig = ctx.theme_signal().clone();
        let locale_sig = current_locale();

        // Instant-apply: settings take effect + persist the moment they change, so
        // there is no Apply/Cancel/OK. The footer's Reset-to-defaults is enabled
        // only while something differs from the factory defaults.
        let not_defaults = build_not_defaults(&theme_sig, &locale_sig, &scale, &vm);

        // The OPENING WINDOW's own Work (never `ctx.app_state`, see this struct's
        // `session` field doc) backs the "Work: `<name>` ▸ Structure" page. When
        // no project is open yet — either because this window's Work has not
        // loaded, or because the window has no session at all (the Launcher) —
        // the page is present in the Switcher but its tree node isn't shown, so
        // it renders an empty placeholder.
        let work = self.session.as_ref().map(|s| s.single_work.clone());
        let work_title = work.as_ref().map(|w| w.title().get()).unwrap_or_default();

        // The shape of the whole window, resolved once: the tree walks it, the
        // parents' pages read their children off it, and the search index takes
        // its parents from it. Both variables it depends on are snapshots taken
        // as this window is built — a Work opened later gets its section when the
        // Settings window is next opened, and the extension registry is a
        // snapshot by design (see `settings_ext`).
        let extension_ids: Vec<&'static str> = crate::settings_ext::registered_pages()
            .iter()
            .map(|p| p.id)
            .collect();
        let spec = tree_spec(
            self.session
                .as_ref()
                .is_some_and(|s| s.single_work.id().is_some()),
            &extension_ids,
        );
        // The two Work pages edit the *entity*, not the settings store, so they go through
        // their own view-model rather than calling `SingleWork::set_*` + `save` from a pane.
        // THIS WINDOW's own session (never `ctx.app_state`), same reasoning as `work` above.
        let work_vm = self.session.as_ref().map(|s| {
            WorkSettingsViewModel::new(
                s.single_work.clone(),
                s.smart_punctuation.clone(),
                s.ids.stack_id.clone(),
            )
        });
        let (left, content, search_field) = content::build(
            ctx,
            self.session.as_ref(),
            self.selected_pane.clone(),
            &vm,
            scale.clone(),
            &games,
            spec,
            &work_vm,
            &work,
            work_title,
        );
        // The rail's search field, for `initial_focus_hint`.
        self.search_field = Some(search_field);

        let footer = self.footer(vm, scale, not_defaults);
        let right = VStack::new()
            .spacing(0.0)
            .child(Expand::vertical().child(content))
            .child(Expand::horizontal().child(Divider::new()))
            .child(FixedSize::new().height(FOOTER_H).child(footer));

        // ── Header strip (title + close), mirroring the Welcome panel ───────
        let header = FixedSize::new().height(HEADER_H).child(
            Padding::symmetric(8.0, 14.0).child(
                HStack::new()
                    .spacing(8.0)
                    .child(
                        Expand::horizontal().child(
                            TextWidget::new(tr!(settings_title()))
                                .style(TextStyleRole::Small)
                                .color(TextRole::Secondary),
                        ),
                    )
                    .child(
                        // Same `dismiss_modal` as Escape and as Done — see the
                        // module doc's "Three ways out", and note that this one
                        // deliberately does NOT take the initial focus.
                        IconButton::clear()
                            .tooltip(tr!(settings_close()))
                            .on_activate_fn(|ctx| ctx.dismiss_modal()),
                    ),
            ),
        );

        // No `FixedSize` around the card: `place_children` below hands the
        // Panel its size directly, which is how the card follows the window.
        // The body columns claim the height the header leaves rather than a
        // `BODY_H` constant derived from `CARD_H` — that constant is what
        // clipped the footer, since it stayed 575 however short the card got.
        let root = teksu!(ctx => Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    Expand::horizontal {
                        child: header
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::vertical {
                        HStack {
                            spacing: 0.0
                            Expand::vertical {
                                FixedSize {
                                    width: TREE_W
                                    child: left
                                }
                            }
                            Expand::vertical {
                                Divider::vertical()
                            }
                            Expand::vertical {
                                FixedSize {
                                    width: self.pane_w.clone()
                                    child: right
                                }
                            }
                        }
                    }
                }
            }
        );
        vec![root]
    }

    /// Report the card's **preferred** size, and note how much room the window
    /// actually left.
    ///
    /// Two passes reach here per frame. The overlay's intrinsic pass proposes
    /// `(None, None)` and takes the answer as the card's wanted size; the layout
    /// pass then proposes the *clamped* rect `OverlayPlacement::Centered` worked
    /// out from it. Answering the aspiration in both is what keeps that loop
    /// stable — a shrunken answer would be re-clamped and re-shrunk every frame —
    /// and the clamped proposal is the only place the real viewport is legible,
    /// so the pane width is written from here. Idempotent, so the per-pass layout
    /// memo is welcome to skip a repeat (see `Widget::cacheable_layout`), and
    /// guarded by `set_if_changed` so a steady window fans out to nobody.
    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        if let (Some(slot_w), Some(slot_h)) = (proposal.width, proposal.height) {
            let (card_w, _) = Self::card_size(slot_w, slot_h);
            self.pane_w.set_if_changed(pane_width(card_w));
        }
        Size::new(CARD_W, CARD_H).into()
    }

    /// Centre the card in the slot the overlay gave it, at whatever size fits.
    ///
    /// The slot is `min(preferred, viewport)`; the card is that less the
    /// [`VIEWPORT_MARGIN`] gutter once it has had to shrink at all. Centring is
    /// what turns the gutter into an even border rather than 32 px of dead space
    /// down one side.
    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let (w, h) = Self::card_size(bounds.width, bounds.height);
        let origin = Point::new(
            bounds.x + ((bounds.width - w) / 2.0).max(0.0),
            bounds.y + ((bounds.height - h) / 2.0).max(0.0),
        );
        for child in children.iter_mut() {
            child.origin = origin;
            child.size = Size::new(w, h);
        }
    }

    /// Open with the **rail's search field** focused, not the header ✕.
    ///
    /// The modal pipeline's fallback is `first_focusable_descendant`, and in tree
    /// order that is the ✕ in the title strip — so the window opened with a focus
    /// ring drawn around its own dismiss glyph. The search field is where a
    /// keyboard user's first keystroke is useful in a window of ~30 pages: type,
    /// and the tree filters. Same reasoning as `PaceSummaryPanel`'s footer Close,
    /// with a different answer because this card has somewhere better to put it.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.search_field
    }

    /// Announce the window as a named, modal dialog.
    ///
    /// Under [`ModalPresentation::InTree`] the `.title(..)` on the `ModalRequest`
    /// is consumed only on the native-window path, and `present_modal` does not
    /// wrap a hand-drawn panel in a `ModalContainer` — so without this the
    /// largest surface in the app had no accessible name, no `Role::Dialog` and
    /// no modal flag at all. Same gap the Pace summary, New Work and Import
    /// documents cards close the same way; the modal flag is this one's addition,
    /// because this card really does take the whole window hostage.
    fn accessibility(&self, builder: &mut AccessNodeBuilder) {
        builder.set_role(teksilo::core::accesskit::Role::Dialog);
        builder.set_name(tr!(settings_title()).resolve_now());
        builder.set_modal();
    }

    /// The name an enclosing shell should borrow — the window's own title.
    fn accessible_title_hint(&self) -> Option<String> {
        Some(tr!(settings_title()).resolve_now())
    }
}

impl SettingsPanel {
    /// The action bar: Reset to defaults · (spacer) · Done.
    ///
    /// Instant-apply — every setting already took effect and persisted as it was
    /// changed, so there is no Apply/Cancel/OK. *Reset to defaults* live-applies
    /// factory values (enabled only while something differs from them) behind a
    /// confirmation, since there is no undo; *Done* closes the window.
    fn footer(
        &self,
        vm: SettingsViewModel,
        scale: Signal<f32>,
        not_defaults: Signal<bool>,
    ) -> impl Widget {
        // Reset to defaults — confirm first (no undo), then live-apply the
        // factory values. Disabled while already at defaults.
        let reset_vm = vm.clone();
        let reset_scale = scale.clone();
        let reset = Button::new(tr!(settings_reset()))
            .variant(ButtonVariant::Plain)
            .enabled(not_defaults)
            .on_activate_fn(move |ctx| {
                let reset_vm = reset_vm.clone();
                let reset_scale = reset_scale.clone();
                MessageBox::warning(tr!(settings_reset_confirm_title()))
                    .text(tr!(settings_reset_confirm_body()))
                    .buttons(MessageBoxButtons::Custom(vec![
                        MessageBoxButton::standard(StandardButton::RestoreDefaults),
                        MessageBoxButton::standard(StandardButton::Cancel),
                    ]))
                    .default_button(StandardButton::Cancel)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |res, ctx| {
                        if res.button == StandardButton::RestoreDefaults {
                            // Theme mode back to *follow the system* and the
                            // interface language back to this machine's own
                            // default, in one call that owns both.
                            //
                            // It used to be `ctx.set_theme(style::light())`,
                            // which silently converted a System-theme reader
                            // into a manual-Light one: the factory state is
                            // "follow the system", and a reset that pins Light
                            // is not a reset. The locale half has the same
                            // shape — the factory language is whatever a fresh
                            // install on this machine would have picked, never
                            // a flat en-US.
                            settings_vm::reset_appearance(ctx);
                            reset_scale.set(TEXT_SCALE_DEFAULT);
                            reset_vm.reset_editor_defaults();
                        }
                    })
                    .present(ctx);
            });

        // Done — everything already applied + persisted; just close the window.
        //
        // Kept, though Escape and the header ✕ now do the identical thing: this
        // is the affordance a mouse user looks for at the end of a dialog, and
        // the ✕ is a 16 px glyph in the far corner. See this module's "Three
        // ways out" section for why none of the three is the redundant one.
        let done = Button::new(tr!(settings_done()))
            .variant(ButtonVariant::Filled)
            .on_activate_fn(|ctx| ctx.dismiss_modal());

        Padding::symmetric(10.0, 22.0).child(
            HStack::new()
                .spacing(9.0)
                .child(reset)
                .child(Spacer::new())
                .child(done),
        )
    }
}

#[cfg(test)]
mod tests;
