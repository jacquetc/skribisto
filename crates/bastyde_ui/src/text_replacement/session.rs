// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The per-document state machine around [`engine`](super::engine): decide when
//! to ask it, apply what it answers, and let the writer take it back.
//!
//! One session per open document, held beside the document itself and shared by
//! every editor showing it (a scene's prose and its synopsis get their own).
//!
//! ## Driven from the frame tick, NOT from `on_change`
//!
//! [`TextReplacementSession::tick`] runs from a frame-tick effect, exactly as
//! [`SpellSession`](crate::spellcheck::SpellSession) does, and notices an edit by
//! comparing the document's revision against the last one it saw.
//!
//! It must not run from the editor's `on_change`. That callback is invoked from
//! inside `frame_loop::tick`, which holds a `borrow_mut` on the editor's state
//! for its whole duration — so **every** `EditorHandle` method panics there:
//! `cursor_position`, `has_selection`, `is_composing`, `replace_range`. Doing the
//! work from `on_change` crashed the app on the first character typed. A frame
//! tick fires after that borrow is released, which is why the spell session has
//! always been wired this way.
//!
//! ## Why the caret has to have moved forward
//!
//! `on_change` says only "the content changed", not what changed, and it is
//! **batched per frame** — a fast typist can land two characters in one call.
//! The session therefore gates on the caret having advanced since the previous
//! call rather than on it having advanced by exactly one: an insertion moves it
//! forward, a deletion or a replaced selection does not. Without that gate,
//! deleting the "x" out of "btwx " would leave text that ends in a fired
//! trigger and expand it, which is not something the writer typed.
//!
//! ## Applying the rule
//!
//! The delimiter is replaced along with the trigger and immediately re-added
//! (`"btw "` → `"by the way "`) rather than the trigger alone. Two reasons:
//! [`EditorHandle::replace_range`] leaves the caret after the inserted text, so
//! replacing just the trigger would strand the caret *before* the space the
//! writer typed; and one call is one undo entry, so a single Ctrl+Z takes the
//! whole expansion back.
//!
//! ## Backspace-revert
//!
//! After a rule fires, a `Backspace` pressed immediately after it puts the
//! trigger back — the escape hatch for "that is not what I meant". It is
//! deliberately narrow: only the very next content change is eligible, and only
//! if the document is in exactly the state the fire left it in minus the
//! delimiter. See [`PendingRevert`].

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use bastyde::text_document::TextDocument;
use bastyde::widgets::rich_text::EditorHandle;

use super::engine::TextReplacementEngine;
use super::typography::{SmartPunctuationFlags, TypographyEngine};
use crate::view_models::TextReplacementRulesViewModel;

/// What a fired rule left behind, so the next keystroke can undo it.
///
/// Held for exactly one content change. The `revision` is the document's
/// revision *after* the fire was applied: a callback that observes the same
/// revision is the fire's own echo and changes nothing, while the first
/// callback observing a different one is the writer's next edit — the only one
/// that may revert.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PendingRevert {
    /// Character position where the replacement starts.
    span_start: usize,
    /// Character count of the inserted replacement.
    replacement_chars: usize,
    /// The replacement text, re-checked against the document before reverting
    /// so an unrelated edit of the same length cannot be mistaken for the fire.
    replacement: String,
    /// The trigger exactly as the writer typed it — what goes back.
    typed: String,
    /// `TextDocument::content_revision` immediately after the fire.
    revision: u64,
}

/// A trigger that must not fire again at this exact spot.
///
/// Set by a revert: the writer has just said "no" to this expansion, so
/// re-typing the delimiter they deleted must give them the delimiter, not the
/// expansion back. One-shot — it is consumed by the fire it suppresses, and
/// dropped as soon as anything else happens.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Suppressed {
    /// Character position the trigger starts at.
    span_start: usize,
    /// Lowercased trigger, matching the engine's own case-insensitive key.
    key: String,
}

/// Per-document replace-while-typing state.
///
/// Cloneable by `Rc` at the call site (`Option<Rc<TextReplacementSession>>`,
/// mirroring how the spell session is threaded), so every editor over one
/// document shares one state machine.
pub struct TextReplacementSession {
    vm: TextReplacementRulesViewModel,
    /// Recompiled when the lexicon or the master switch changes; see
    /// [`refresh_engine`](Self::refresh_engine).
    engine: RefCell<TextReplacementEngine>,
    /// The `(lexicon version, master switch)` the engine was compiled from.
    compiled_from: Cell<(u64, bool)>,
    /// The document's BCP-47 language tag, pushed in by the open-docs store and
    /// re-pushed when the writer changes an item's language. Decides how case is
    /// folded and propagated — it is what makes a Turkish trigger match at all
    /// (see `skribisto_model::casing`).
    locale: RefCell<String>,
    /// The locale the current engine was compiled for, so a language change
    /// forces a recompile the same way a lexicon edit does.
    compiled_locale: RefCell<String>,
    /// The locale's punctuation rules — quotes, dashes, the ellipsis, French
    /// spacing, Arabic's own marks. Rebuilt whenever the locale or the project's
    /// flags change; see [`set_punctuation`](Self::set_punctuation).
    typography: RefCell<TypographyEngine>,
    /// Which punctuation rules this project wants on, pushed in from the
    /// `SmartPunctuation` row. `None` until the store has resolved it, which is
    /// **not** the same as "all off" — it means "do not substitute anything
    /// yet", so a document opened before the row loads is never silently
    /// rewritten under a guess.
    punctuation: RefCell<Option<SmartPunctuationFlags>>,
    /// The `(flags, locale)` the typography engine was compiled from.
    ///
    /// The **locale is always part of the key**, even when the flags are still
    /// `None`. It has to be: a paragraph rule that gates on the language rather
    /// than on a flag — Spanish's `¿`, Arabic's mirrored marks — fires from an
    /// engine built with only the locale, so a locale change while the flags are
    /// unresolved must still force a rebuild. A key that dropped the locale
    /// alongside the flags left the engine pinned to whatever locale it was
    /// first built with (`""`), and Spanish never fired in a project whose
    /// punctuation row had not yet loaded when its language was set.
    compiled_punctuation: RefCell<Option<(Option<SmartPunctuationFlags>, String)>>,
    pending: RefCell<Option<PendingRevert>>,
    suppressed: RefCell<Option<Suppressed>>,
    /// Caret position at the previous tick; `None` until the first one.
    last_caret: Cell<Option<usize>>,
    /// Document revision at the previous tick. Most frames change nothing, so
    /// this is the early-out that keeps the per-frame cost to one field read.
    last_revision: Cell<Option<u64>>,
    /// Set while this session is itself mutating the document, so the change
    /// notification that mutation provokes cannot re-enter the machine. The
    /// revision check would catch a re-entrant call anyway, but only after it
    /// had already read the document back mid-edit.
    applying: Cell<bool>,
}

impl TextReplacementSession {
    /// A session over `vm`'s lexicon. The engine is compiled lazily on the
    /// first change rather than here, so opening a document costs nothing when
    /// the writer never types in it.
    pub fn new(vm: TextReplacementRulesViewModel) -> Rc<Self> {
        Rc::new(Self {
            vm,
            engine: RefCell::new(TextReplacementEngine::default()),
            // A version the lexicon cannot be at, so the first change always
            // compiles. `(0, false)` would not do: 0 IS the version a freshly
            // opened project's lexicon sits at, so an engine that had never been
            // built would look current and the first rule would never fire.
            compiled_from: Cell::new((u64::MAX, false)),
            locale: RefCell::new(String::new()),
            // Deliberately not equal to `locale`'s initial value, so the very
            // first refresh compiles rather than believing itself current.
            compiled_locale: RefCell::new("\u{0}unset".to_string()),
            // Explicitly all-off, NOT `TypographyEngine::default()`. That
            // default carries `SmartPunctuationFlags::default()`, which is
            // everything **on** — the right default for the app-level
            // preference, and exactly wrong here: it would substitute
            // punctuation in the window between the document opening and its
            // settings resolving, in a project that may want none.
            typography: RefCell::new(TypographyEngine::new("", SmartPunctuationFlags::all_off())),
            punctuation: RefCell::new(None),
            compiled_punctuation: RefCell::new(None),
            pending: RefCell::new(None),
            suppressed: RefCell::new(None),
            last_caret: Cell::new(None),
            last_revision: Cell::new(None),
            applying: Cell::new(false),
        })
    }

    /// One frame. Cheap and safe to call unconditionally — it returns
    /// immediately unless the document actually changed since the last tick.
    ///
    /// Called from a frame-tick effect (see `tabs::shared::editor`), never from
    /// `on_change`; see the module documentation for why that distinction is
    /// load-bearing rather than stylistic.
    pub fn tick(&self, handle: &EditorHandle, doc: &TextDocument) {
        // Our own edit, coming back around. Nothing to decide.
        if self.applying.get() {
            return;
        }
        // The early-out: nothing has been typed since the last frame.
        let revision = doc.content_revision();
        if self.last_revision.replace(Some(revision)) == Some(revision) {
            return;
        }
        // Mid-composition text is provisional — a Japanese or Korean writer is
        // still choosing characters, and expanding one out from under them would
        // break the IME's own editing model.
        if handle.is_composing() {
            return;
        }

        let caret = handle.cursor_position();
        let advanced = self
            .last_caret
            .replace(Some(caret))
            .is_some_and(|p| caret > p);

        // A pending revert outlives only the fire's own echo. The first change
        // that moves the document past it is the writer's, and is the one — the
        // only one — that may revert.
        if let Some(pending) = self.take_pending_if_settled(doc)
            && self.try_revert(handle, doc, &pending, caret)
        {
            return;
        }

        if !advanced || handle.has_selection().get() {
            return;
        }
        self.refresh_engine();
        self.refresh_typography();
        // The lexicon first, and at most one of the two per keystroke.
        //
        // Both can match the same character — typing `dbl.` ends a lexicon
        // trigger *and* could begin an ellipsis. The lexicon wins because it is
        // the writer's own explicit instruction, where typography is a house
        // convention; and only one fires because each leaves a pending revert
        // behind, and two in one tick would make Backspace undo the wrong half.
        if self.try_fire(handle, doc, caret) {
            return;
        }
        // Paragraph rules before the stateless ones. They know strictly more —
        // a dialogue dash has to be the whole paragraph so far, Spanish's `¿`
        // spans back to where the clause began — so when both could match, the
        // one with more context is the one that should decide.
        if self.try_paragraph(handle, doc, caret) {
            return;
        }
        self.try_typography(handle, doc, caret);
    }

    /// Set the document's language tag. Cheap and idempotent; a real change
    /// invalidates the compiled engine so the next tick rebuilds it.
    ///
    /// Pushed rather than pulled because the effective language of an item is
    /// resolved against the whole binder (item's own tag, else the Work's), and
    /// the open-docs store already does that work for spell-check.
    pub fn set_locale(&self, tag: &str) {
        let mut locale = self.locale.borrow_mut();
        if *locale != tag {
            *locale = tag.to_string();
        }
    }

    /// Set which punctuation rules this project wants, from its
    /// `SmartPunctuation` row. Cheap and idempotent, like [`set_locale`].
    ///
    /// `None` means "not resolved yet" and substitutes nothing. It is
    /// deliberately distinct from `Some(everything off)`: a document whose row
    /// has not loaded must not be rewritten under a guess, and once the row
    /// arrives with `override_app_default` false the caller passes the
    /// app-level default rather than this row's inert flags.
    ///
    /// [`set_locale`]: Self::set_locale
    pub fn set_punctuation(&self, flags: Option<SmartPunctuationFlags>) {
        let mut current = self.punctuation.borrow_mut();
        if *current != flags {
            *current = flags;
        }
    }

    /// Rebuild the typography engine when the flags or the locale have moved.
    ///
    /// Separate from [`refresh_engine`](Self::refresh_engine) because the two
    /// have different inputs — the lexicon's version counter versus a pushed
    /// flag set — even though both are recompiled on the same tick.
    fn refresh_typography(&self) {
        let flags = self.punctuation.borrow().clone();
        let locale = self.locale.borrow().clone();
        // The locale is in the key unconditionally — see the field's own note
        // for why collapsing it into the flags was a bug.
        let wanted = (flags.clone(), locale.clone());
        if self.compiled_punctuation.borrow().as_ref() == Some(&wanted) {
            return;
        }
        *self.compiled_punctuation.borrow_mut() = Some(wanted);
        *self.typography.borrow_mut() = match flags {
            Some(flags) => TypographyEngine::new(&locale, flags),
            // Not resolved: an engine with every rule off — but built with the
            // real locale, so a language-only rule (Spanish's `¿`, Arabic's
            // marks) still fires while the flags are pending.
            None => TypographyEngine::new(&locale, SmartPunctuationFlags::all_off()),
        };
    }

    /// The lexicon this session reads. Test-only: the live-editor tests need to
    /// add a rule to the very view-model the session compiles from, and going
    /// through the session keeps them from having to thread a second handle.
    #[cfg(test)]
    pub fn vm_for_test(&self) -> &TextReplacementRulesViewModel {
        &self.vm
    }

    /// The locale the session currently holds — the one the next tick compiles
    /// the engines against. Test-only, for pinning that a language pushed
    /// through the open-docs store actually reaches here.
    #[cfg(test)]
    pub fn locale_for_test(&self) -> String {
        self.locale.borrow().clone()
    }

    /// Recompile the engine when the lexicon, the project's master switch, or
    /// the document's language has changed since it was last built. All plain
    /// reads — cheaper per keystroke than a subscription would be to keep
    /// correct across project switches.
    fn refresh_engine(&self) {
        let enabled = self.vm.enabled_signal().get();
        let version = self.vm.changed_signal().get();
        let locale = self.locale.borrow().clone();
        if self.compiled_from.get() == (version, enabled)
            && *self.compiled_locale.borrow() == locale
        {
            return;
        }
        self.compiled_from.set((version, enabled));
        *self.compiled_locale.borrow_mut() = locale.clone();
        *self.engine.borrow_mut() = if enabled {
            TextReplacementEngine::from_rules_for_locale(&self.vm.rows(), &locale)
        } else {
            // Switched off: an empty engine, so the per-keystroke path is a
            // single `is_empty` check rather than a branch on the flag.
            TextReplacementEngine::default()
        };
    }

    /// Take the pending revert if the document has moved past the fire that
    /// created it, leaving it in place if this callback is that fire's echo.
    fn take_pending_if_settled(&self, doc: &TextDocument) -> Option<PendingRevert> {
        let is_echo = {
            let pending = self.pending.borrow();
            let pending = pending.as_ref()?;
            doc.content_revision() == pending.revision
        };
        if is_echo {
            return None;
        }
        self.pending.borrow_mut().take()
    }

    /// Put the trigger back if this change was exactly "the writer deleted the
    /// delimiter the fire re-added". Returns whether it did.
    fn try_revert(
        &self,
        handle: &EditorHandle,
        doc: &TextDocument,
        pending: &PendingRevert,
        caret: usize,
    ) -> bool {
        if caret != pending.span_start + pending.replacement_chars {
            return false;
        }
        // The caret being in the right place is not enough — an edit elsewhere
        // could leave it there. Read the replacement back and require it to
        // still be exactly what was inserted.
        let before = text_before(doc, caret, pending.replacement_chars);
        if before.as_deref() != Some(pending.replacement.as_str()) {
            return false;
        }
        self.apply(|| handle.replace_range(pending.span_start, caret, &pending.typed));
        // Re-typing the delimiter now must give the writer the delimiter, not
        // the expansion they just rejected.
        *self.suppressed.borrow_mut() = Some(Suppressed {
            span_start: pending.span_start,
            key: pending.typed.to_lowercase(),
        });
        self.last_caret.set(Some(handle.cursor_position()));
        self.last_revision.set(Some(doc.content_revision()));
        true
    }

    /// Expand a lexicon rule if one just fired. Returns whether it did, so the
    /// caller knows not to also run a typography rule over the same keystroke.
    fn try_fire(&self, handle: &EditorHandle, doc: &TextDocument, caret: usize) -> bool {
        let fired = {
            let engine = self.engine.borrow();
            if engine.is_empty() {
                return false;
            }
            let Some(window) = text_before(doc, caret, engine.window_chars()) else {
                return false;
            };
            engine.check(&window)
        };
        let Some(fired) = fired else {
            // Anything the writer types that is not a fired rule ends the
            // one-shot suppression — it only covers re-typing the delimiter
            // they just deleted, not the rest of the paragraph.
            self.suppressed.borrow_mut().take();
            return false;
        };

        // The delimiter that fired sits between the trigger and the caret.
        let Some(span_start) = caret.checked_sub(fired.trigger_chars + 1) else {
            return false;
        };
        if self.consume_suppression(span_start, &fired.typed) {
            // Suppressed still counts as handled: the writer just rejected this
            // expansion, and letting typography have the same keystroke would
            // substitute something else in its place.
            return true;
        }

        let delimiter = match text_before(doc, caret, 1) {
            Some(d) if !d.is_empty() => d,
            // The delimiter is what fired the rule, so it must be readable; if
            // it is not, the document moved under us and doing nothing is right.
            _ => return false,
        };
        let replacement_chars = fired.replacement.chars().count();
        let text = format!("{}{delimiter}", fired.replacement);
        self.apply(|| handle.replace_range(span_start, caret, &text));

        *self.pending.borrow_mut() = Some(PendingRevert {
            span_start,
            replacement_chars,
            replacement: fired.replacement,
            typed: fired.typed,
            revision: doc.content_revision(),
        });
        self.last_caret.set(Some(handle.cursor_position()));
        self.last_revision.set(Some(doc.content_revision()));
        true
    }

    /// The paragraph-aware rules — a dialogue dash, Spanish's opening marks.
    ///
    /// Reads the current paragraph up to the caret, bounded by
    /// `position_in_block()`. That bound is the whole reason this is cheap and
    /// safe: it cannot reach into the paragraph above, so there is no scanning
    /// for newlines, and the read is proportional to the line rather than the
    /// document.
    ///
    /// No pending revert, for the same reason [`try_typography`] leaves none:
    /// these substitute rather than swallow, so Backspace should delete what is
    /// there and Ctrl+Z restores the literal in one step.
    ///
    /// [`try_typography`]: Self::try_typography
    fn try_paragraph(&self, handle: &EditorHandle, doc: &TextDocument, caret: usize) -> bool {
        let in_block = doc.cursor_at(caret).position_in_block();
        if in_block == 0 {
            return false;
        }
        let Some(block_before) = text_before(doc, caret, in_block) else {
            return false;
        };
        let fired = self.typography.borrow().check_paragraph(&block_before);
        let Some(fired) = fired else {
            return false;
        };
        let Some(span_start) = caret.checked_sub(fired.replace_chars) else {
            return false;
        };
        self.apply(|| handle.replace_range(span_start, caret, &fired.replacement));
        self.last_caret.set(Some(handle.cursor_position()));
        self.last_revision.set(Some(doc.content_revision()));
        true
    }

    /// Substitute the locale's own punctuation if the character just typed calls
    /// for it.
    ///
    /// Unlike a lexicon rule this replaces text ending *at* the caret rather
    /// than one character before it: the trigger here IS the character the
    /// writer just typed, not a delimiter following a word.
    ///
    /// ## And unlike a lexicon rule, it leaves no pending revert
    ///
    /// The backspace-revert exists because a lexicon expansion swallows the
    /// delimiter the writer typed, so Backspace has to mean "put my word back"
    /// rather than "delete a character". A punctuation substitution swallows
    /// nothing — one glyph in, one glyph out — so Backspace should delete it
    /// exactly as it deletes any character, and pressing it twice to remove one
    /// quotation mark would be a bug, not an escape hatch.
    ///
    /// The escape hatch is Ctrl+Z, and it already works: the substitution goes
    /// through a single [`EditorHandle::replace_range`], which is one undo
    /// entry, so undoing it restores the literal `"` or `--` the writer typed.
    /// That is also what Word and LibreOffice do.
    fn try_typography(&self, handle: &EditorHandle, doc: &TextDocument, caret: usize) {
        let fired = {
            let typography = self.typography.borrow();
            let Some(window) = text_before(doc, caret, typography.window_chars()) else {
                return;
            };
            typography.check(&window)
        };
        let Some(fired) = fired else {
            return;
        };
        let Some(span_start) = caret.checked_sub(fired.replace_chars) else {
            return;
        };
        self.apply(|| handle.replace_range(span_start, caret, &fired.replacement));
        self.last_caret.set(Some(handle.cursor_position()));
        self.last_revision.set(Some(doc.content_revision()));
    }

    /// Whether a fire at `span_start` for `typed` is the one a revert just
    /// suppressed. Consumes the suppression either way — it covers the one
    /// keystroke that follows the revert, not the rest of the session.
    fn consume_suppression(&self, span_start: usize, typed: &str) -> bool {
        let Some(s) = self.suppressed.borrow_mut().take() else {
            return false;
        };
        s.span_start == span_start && s.key == typed.to_lowercase()
    }

    /// Run a document mutation with the re-entrancy guard held.
    fn apply(&self, edit: impl FnOnce()) {
        self.applying.set(true);
        edit();
        self.applying.set(false);
    }
}

/// The last `count` characters before `at`, or `None` if the document could not
/// be read (it moved under us, or holds a structure the fast path rejects and
/// the slow path failed too).
fn text_before(doc: &TextDocument, at: usize, count: usize) -> Option<String> {
    if count == 0 {
        return Some(String::new());
    }
    doc.cursor_at(at).text_before(count).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The revert gate, stated as the pure predicate the impure path applies.
    /// `try_revert` is not directly testable headless (it needs a live
    /// `EditorHandle`), but its decision is exactly these two comparisons, and
    /// they are what a wrong caret or an unrelated same-length edit has to fail.
    fn revert_applies(pending: &PendingRevert, caret: usize, before: &str) -> bool {
        caret == pending.span_start + pending.replacement_chars && before == pending.replacement
    }

    fn pending() -> PendingRevert {
        PendingRevert {
            span_start: 4,
            replacement_chars: 10,
            replacement: "by the way".into(),
            typed: "btw".into(),
            revision: 7,
        }
    }

    /// The case the feature exists for: "say btw " expanded, the writer hit
    /// Backspace, so the delimiter is gone and the caret sits at the end of the
    /// replacement.
    #[test]
    fn deleting_the_delimiter_right_after_a_fire_reverts() {
        assert!(revert_applies(&pending(), 14, "by the way"));
    }

    /// The writer kept typing instead — the caret is past where a revert could
    /// apply, so the expansion stands.
    #[test]
    fn typing_on_after_a_fire_does_not_revert() {
        assert!(!revert_applies(&pending(), 15, "by the way "));
    }

    /// A caret in the right place is not enough on its own: an edit elsewhere
    /// can leave it there, and reverting then would rewrite text the fire never
    /// touched.
    #[test]
    fn a_matching_caret_over_different_text_does_not_revert() {
        assert!(!revert_applies(&pending(), 14, "by the wax"));
    }

    /// Deleting further back moves the caret out of the span.
    #[test]
    fn deleting_into_the_replacement_does_not_revert() {
        assert!(!revert_applies(&pending(), 13, "by the wa"));
    }

    /// The suppression is keyed case-insensitively, matching the engine: the
    /// writer who reverted "BTW" must not have "btw" re-expand at that spot.
    #[test]
    fn the_suppression_key_is_case_insensitive() {
        let s = Suppressed {
            span_start: 4,
            key: "btw".into(),
        };
        assert_eq!(s.key, "BTW".to_lowercase());
    }

    /// The revert restores the trigger *without* the delimiter the backspace
    /// consumed. That is what keeps the restore from re-firing: the text now
    /// ends in a word character, so the engine has nothing to match.
    #[test]
    fn the_restored_text_cannot_immediately_re_fire() {
        use crate::models::TextReplacementRuleRow;
        let engine = TextReplacementEngine::from_rules(&[TextReplacementRuleRow {
            id: 0,
            trigger: "btw".into(),
            replacement: "by the way".into(),
            enabled: true,
        }]);
        // What the document reads as after a revert of "say btw ".
        assert_eq!(engine.check("say btw"), None);
    }
}

/// The session against a **live editor** — the integration the pure tests above
/// cannot reach.
///
/// Everything here goes through the same `EditorHandle` the writing surfaces
/// hand `tick`, over a real `RichTextEditor` in a laid-out
/// `WidgetTree` (the shape `tabs::shared::dictionary_menu`'s tests established).
/// This is what actually proves the feature works: the pure engine tests say a
/// rule *matches*, and these say the document *changes*.
///
/// `mocks` because the lexicon comes from a `TextReplacementRulesViewModel`,
/// whose list model only fabricates rows in that feature set.
#[cfg(all(test, feature = "mocks"))]
mod live_editor_tests {
    use bastyde::core::widget_tree::WidgetTree;
    use bastyde::prelude::SizeProposal;
    use bastyde::text_document::TextDocument;
    use bastyde::widgets::rich_text::{EditorHandle, RichTextEditor};
    use frontend::AppContext;

    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::TextReplacementRuleListModel;
    use crate::singles::SingleWork;

    /// A live editor over `text`, plus a session whose lexicon is the mock one
    /// (`--` → `—`, `btw` → `by the way`, and a disabled `teh` → `the`) with the
    /// project's master switch turned on.
    ///
    /// The `WidgetTree` is returned and must be kept alive — the handle reads
    /// the editor's state, and the tree owns the editor.
    fn editor(
        text: &str,
    ) -> (
        TextDocument,
        EditorHandle,
        Rc<TextReplacementSession>,
        WidgetTree,
    ) {
        let doc = TextDocument::new();
        doc.set_plain_text(text).unwrap();
        let ed = RichTextEditor::editor(doc.clone());
        let handle = ed.handle();
        let mut tree = WidgetTree::new();
        tree.add(ed);
        tree.layout(SizeProposal::exact(600.0, 400.0));

        let ctx = Rc::new(AppContext::new());
        let work = SingleWork::new(ctx.clone());
        work.set_custom_replacement_rules_enabled(true);
        let vm = TextReplacementRulesViewModel::new(
            TextReplacementRuleListModel::new(ctx),
            work,
            AppIds::new(),
        );
        let session = TextReplacementSession::new(vm);
        (doc, handle, session, tree)
    }

    /// Type `s` one character at a time at the caret, running the session after
    /// each — the same cadence `on_change` delivers real typing in.
    fn type_text(
        handle: &EditorHandle,
        doc: &TextDocument,
        session: &TextReplacementSession,
        s: &str,
    ) {
        // Seed the caret baseline before typing, which is what the real app
        // does for free: the frame-tick effect runs from the moment the editor
        // is built, so by the time anyone types, `tick` has already observed a
        // caret. Without this the FIRST character of each test is swallowed by
        // the caret-advanced gate — invisible to a multi-character lexicon
        // trigger, but fatal to a single-character punctuation rule.
        session.tick(handle, doc);
        for c in s.chars() {
            handle.insert_text(&c.to_string());
            session.tick(handle, doc);
        }
    }

    fn plain(doc: &TextDocument) -> String {
        doc.to_plain_text().unwrap_or_default()
    }

    /// The headline behaviour: typing the trigger and then a space expands it,
    /// and the space the writer typed is still there afterwards.
    #[test]
    fn typing_a_trigger_then_a_space_expands_it() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "I saw btw ");
        assert_eq!(plain(&doc), "I saw by the way ");
        assert_eq!(
            handle.cursor_position(),
            "I saw by the way ".chars().count(),
            "the caret must end up after the delimiter, not before it"
        );
    }

    /// Still mid-word, nothing has fired — otherwise the expansion would happen
    /// under the writer's fingers as they typed the trigger's last letter.
    #[test]
    fn typing_the_trigger_alone_does_not_expand() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "I saw btw");
        assert_eq!(plain(&doc), "I saw btw");
    }

    /// The word-start guard, against a real document.
    #[test]
    fn a_trigger_inside_a_longer_word_does_not_expand() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "a xbtw ");
        assert_eq!(plain(&doc), "a xbtw ");
    }

    /// Case propagation end to end.
    #[test]
    fn the_typed_case_reaches_the_document() {
        let (doc, handle, session, _tree) = editor("");
        // `--` → `—` has no letters, so it also proves a caseless rule fires.
        type_text(&handle, &doc, &session, "a -- ");
        assert_eq!(plain(&doc), "a — ");
    }

    /// A disabled rule must not fire even with the project switch on.
    #[test]
    fn a_disabled_rule_does_not_expand() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "teh ");
        assert_eq!(plain(&doc), "teh ");
    }

    /// The document's language must actually reach the matcher.
    ///
    /// The mock lexicon ships `teh` → `the` (disabled) and `btw` → `by the way`.
    /// Under `tr`, typing the SHOUTED form of `btw` is `BTW` either way — `b`,
    /// `t` and `w` have no dotted-I problem — so this uses a rule that does:
    /// it adds one, then checks that the Turkish fold matches where the default
    /// fold would not.
    #[test]
    fn the_documents_locale_reaches_the_matcher() {
        let (doc, handle, session, _tree) = editor("");
        // A trigger whose uppercase differs between Turkish and the default.
        session.vm_for_test().create("ii", "iyi", true);
        session.set_locale("tr-TR");
        // `İİ` is the Turkish uppercase of `ii`; under the default fold it would
        // carry a combining dot and never match.
        type_text(&handle, &doc, &session, "İİ ");
        assert_eq!(plain(&doc), "İYİ ", "got {:?}", plain(&doc));
    }

    /// And the same input under a non-Turkish locale must NOT expand — `İİ` is
    /// not the uppercase of `ii` anywhere else.
    #[test]
    fn a_non_turkish_locale_does_not_get_the_turkish_fold() {
        let (doc, handle, session, _tree) = editor("");
        session.vm_for_test().create("ii", "iyi", true);
        session.set_locale("en-US");
        type_text(&handle, &doc, &session, "İİ ");
        assert_eq!(plain(&doc), "İİ ", "got {:?}", plain(&doc));
    }

    /// Backspace immediately after an expansion puts the writer's own spelling
    /// back — the escape hatch, driven through the real handle.
    #[test]
    fn backspace_right_after_an_expansion_reverts_it() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "I saw btw ");
        assert_eq!(plain(&doc), "I saw by the way ");

        // The delimiter the expansion re-added is what Backspace removes.
        let end = handle.cursor_position();
        handle.replace_range(end - 1, end, "");
        session.tick(&handle, &doc);
        assert_eq!(plain(&doc), "I saw btw");
    }

    /// And having reverted, re-typing the delimiter must NOT expand it again —
    /// the writer already said no. (The one-shot suppression.)
    #[test]
    fn re_typing_the_delimiter_after_a_revert_does_not_re_expand() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "I saw btw ");
        let end = handle.cursor_position();
        handle.replace_range(end - 1, end, "");
        session.tick(&handle, &doc);
        assert_eq!(plain(&doc), "I saw btw");

        type_text(&handle, &doc, &session, " ");
        assert_eq!(
            plain(&doc),
            "I saw btw ",
            "the rule the writer just rejected must not fire again at that spot"
        );
    }

    /// The suppression is one-shot: a *different* occurrence later still fires,
    /// or rejecting one expansion would disable the rule for the session.
    #[test]
    fn a_later_occurrence_still_expands_after_a_revert() {
        let (doc, handle, session, _tree) = editor("");
        type_text(&handle, &doc, &session, "I saw btw ");
        let end = handle.cursor_position();
        handle.replace_range(end - 1, end, "");
        session.tick(&handle, &doc);
        type_text(&handle, &doc, &session, " and btw ");
        assert!(
            plain(&doc).ends_with("and by the way "),
            "got {:?}",
            plain(&doc)
        );
    }

    /// Deleting text that happens to leave a fired trigger behind the caret is
    /// not typing, and must not expand — the caret-advanced gate.
    #[test]
    fn a_deletion_that_exposes_a_trigger_does_not_expand() {
        let (doc, handle, session, _tree) = editor("say btw x ");
        // Put the caret after the "x " and delete the "x", leaving "say btw  ".
        handle.select_range(9, 9);
        session.tick(&handle, &doc); // seed the caret baseline
        handle.replace_range(8, 9, "");
        session.tick(&handle, &doc);
        assert!(
            !plain(&doc).contains("by the way"),
            "a deletion must not fire a rule, got {:?}",
            plain(&doc)
        );
    }

    // ── Typography, through the same live editor ─────────────────────────────

    /// Turn on the punctuation rules for `locale` on an existing session.
    fn punctuate(session: &TextReplacementSession, locale: &str) {
        session.set_locale(locale);
        session.set_punctuation(Some(SmartPunctuationFlags::default()));
    }

    #[test]
    fn typing_three_dots_produces_an_ellipsis() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "en-US");
        type_text(&handle, &doc, &session, "wait...");
        assert_eq!(plain(&doc), "wait…");
    }

    /// The chained case, through a real document: two hyphens become an en dash
    /// and the third has to upgrade it rather than sit beside it.
    #[test]
    fn typing_three_hyphens_climbs_to_an_em_dash() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "en-US");
        type_text(&handle, &doc, &session, "a--");
        assert_eq!(plain(&doc), "a–", "two hyphens make an en dash");
        type_text(&handle, &doc, &session, "-");
        assert_eq!(plain(&doc), "a—", "the third upgrades it");
    }

    #[test]
    fn quotes_curl_by_side_against_a_real_document() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "en-US");
        type_text(&handle, &doc, &session, "he said \"yes\"");
        assert_eq!(plain(&doc), "he said “yes”");
    }

    /// French takes guillemets and a narrow no-break space before its question
    /// mark — both rules on one line of prose.
    #[test]
    fn french_gets_its_guillemets_and_its_thin_space() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("fr-FR");
        session.set_punctuation(Some(SmartPunctuationFlags {
            pre_punctuation_spacing: true,
            ..SmartPunctuationFlags::default()
        }));
        type_text(&handle, &doc, &session, "\"Quoi ?");
        assert_eq!(plain(&doc), "«Quoi\u{202F}?");
    }

    /// Arabic mirroring, end to end.
    #[test]
    fn arabic_punctuation_is_mirrored_in_the_document() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "ar");
        type_text(&handle, &doc, &session, "كيف?");
        assert_eq!(plain(&doc), "كيف؟");
    }

    /// The reason mirroring is gated on script rather than direction — Hebrew is
    /// right-to-left and keeps its ASCII question mark.
    #[test]
    fn hebrew_keeps_its_ascii_question_mark_in_the_document() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "he-IL");
        type_text(&handle, &doc, &session, "מה?");
        assert_eq!(plain(&doc), "מה?");
    }

    /// Both engines can match the same keystroke. The lexicon wins, because it
    /// is the writer's own instruction where typography is a house convention —
    /// and only one fires, or Backspace would undo the wrong half.
    #[test]
    fn the_lexicon_wins_a_keystroke_both_engines_could_claim() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "en-US");
        // `btw` + `.` ends a lexicon trigger; the `.` could also begin an
        // ellipsis. Only the expansion may happen.
        type_text(&handle, &doc, &session, "I saw btw.");
        assert_eq!(plain(&doc), "I saw by the way.");
    }

    /// Typography must not disturb the lexicon's own backspace-revert.
    #[test]
    fn backspace_revert_still_works_with_punctuation_on() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "en-US");
        type_text(&handle, &doc, &session, "I saw btw ");
        assert_eq!(plain(&doc), "I saw by the way ");
        let end = handle.cursor_position();
        handle.replace_range(end - 1, end, "");
        session.tick(&handle, &doc);
        assert_eq!(plain(&doc), "I saw btw");
    }

    /// A punctuation substitution leaves NO pending revert: one glyph replaced
    /// one glyph, so Backspace must delete it like any character rather than
    /// restoring the literal and needing a second press.
    #[test]
    fn backspace_after_a_substitution_just_deletes_it() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "en-US");
        // A space before it, so the quote opens rather than closes — the side
        // is decided by what precedes, and `a"` would legitimately give `a”`.
        type_text(&handle, &doc, &session, "a \"");
        assert_eq!(plain(&doc), "a “");
        let end = handle.cursor_position();
        handle.replace_range(end - 1, end, "");
        session.tick(&handle, &doc);
        assert_eq!(
            plain(&doc),
            "a ",
            "the quote is gone, not turned back into a literal one"
        );
    }

    /// Until the project's row resolves, nothing is substituted — a document
    /// must never be rewritten under a guess about settings still loading.
    #[test]
    fn nothing_is_substituted_before_the_settings_resolve() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("en-US");
        // `set_punctuation` deliberately not called.
        type_text(&handle, &doc, &session, "wait... \"no\"");
        assert_eq!(plain(&doc), "wait... \"no\"");
    }

    /// And an explicitly all-off row substitutes nothing either, while the
    /// lexicon carries on working — the two switches are independent.
    #[test]
    fn punctuation_off_leaves_the_lexicon_running() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("en-US");
        session.set_punctuation(Some(SmartPunctuationFlags::all_off()));
        type_text(&handle, &doc, &session, "wait... btw ");
        assert_eq!(plain(&doc), "wait... by the way ");
    }

    /// **The document's language picks the quotation marks**, through the same
    /// live editor a writer types into.
    ///
    /// This is the test that answers "does it adapt to the project's language,
    /// or is it really just English and French?" — every row here is a locale
    /// the ruleset table carries, and each opens with its own glyph. Dashes and
    /// the ellipsis are deliberately absent: they are locale-independent, which
    /// is exactly why the feature can *look* English-only until someone types a
    /// quotation mark.
    #[test]
    fn the_documents_language_picks_the_quotation_marks() {
        for (locale, want) in [
            ("en-US", "\u{201C}"), // “
            ("fr-FR", "\u{00AB}"), // «
            ("de-DE", "\u{201E}"), // „
            ("de-CH", "\u{00AB}"), // « — Switzerland departs from German
            ("es-ES", "\u{00AB}"),
            ("it-IT", "\u{00AB}"),
            ("pt-PT", "\u{00AB}"),
            ("pt-BR", "\u{201C}"), // Brazil departs from Portugal
            ("nl-NL", "\u{201C}"),
            ("pl-PL", "\u{201E}"),
            ("ru-RU", "\u{00AB}"),
            ("sv-SE", "\u{201D}"), // ” at BOTH ends
            ("tr-TR", "\u{201C}"),
            ("ar", "\u{00AB}"),
        ] {
            let (doc, handle, session, _tree) = editor("");
            punctuate(&session, locale);
            type_text(&handle, &doc, &session, "x \"");
            assert_eq!(
                plain(&doc),
                format!("x {want}"),
                "{locale} must open its quotation with {want}"
            );
        }
    }

    /// A region with no row of its own inherits its language's typography
    /// rather than falling back to English — `fr-CA` is French.
    #[test]
    fn an_unlisted_region_inherits_its_language() {
        let (doc, handle, session, _tree) = editor("");
        punctuate(&session, "fr-CA");
        type_text(&handle, &doc, &session, "il dit \"");
        assert_eq!(plain(&doc), "il dit \u{00AB}");
    }

    // ── The paragraph/clause subsystem ───────────────────────────────────────

    fn spanish(session: &TextReplacementSession) {
        session.set_locale("es-ES");
        session.set_punctuation(Some(SmartPunctuationFlags::default()));
    }

    /// **The rule that cannot work from the tail.** Spanish opens a question
    /// where the *clause* began, which is most of a line behind the caret.
    #[test]
    fn spanish_opens_its_question_at_the_start_of_the_clause() {
        let (doc, handle, session, _tree) = editor("");
        spanish(&session);
        type_text(&handle, &doc, &session, "Que hora es?");
        assert_eq!(plain(&doc), "\u{00BF}Que hora es?");
        assert_eq!(
            handle.cursor_position(),
            "\u{00BF}Que hora es?".chars().count(),
            "the caret stays after the `?` — it must NOT jump back to the mark"
        );
    }

    #[test]
    fn spanish_opens_an_exclamation_too() {
        let (doc, handle, session, _tree) = editor("");
        spanish(&session);
        type_text(&handle, &doc, &session, "Que bien!");
        assert_eq!(plain(&doc), "\u{00A1}Que bien!");
    }

    /// The case that makes this a *clause* scan and not a sentence one: Spanish
    /// re-opens mid sentence.
    #[test]
    fn spanish_reopens_after_a_comma_mid_sentence() {
        let (doc, handle, session, _tree) = editor("");
        spanish(&session);
        type_text(&handle, &doc, &session, "Si puedes, vienes?");
        assert_eq!(plain(&doc), "Si puedes, \u{00BF}vienes?");
    }

    /// A second question in the same paragraph opens its own clause, not the
    /// first one again.
    #[test]
    fn a_second_question_opens_its_own_clause() {
        let (doc, handle, session, _tree) = editor("");
        spanish(&session);
        type_text(&handle, &doc, &session, "Vienes? Cuando?");
        assert_eq!(plain(&doc), "\u{00BF}Vienes? \u{00BF}Cuando?");
    }

    /// A writer who typed the mark themselves must not get a second one.
    #[test]
    fn an_already_opened_question_is_left_alone() {
        let (doc, handle, session, _tree) = editor("");
        spanish(&session);
        type_text(&handle, &doc, &session, "\u{00BF}Vienes?");
        assert_eq!(plain(&doc), "\u{00BF}Vienes?");
    }

    /// Neighbours that do NOT invert. Catalan and Portuguese sit next to Spanish
    /// in the locale table and share its guillemets — inserting `¿` into either
    /// would be a character no reader of them expects.
    #[test]
    fn the_neighbouring_languages_do_not_invert() {
        for locale in ["ca", "pt-PT", "pt-BR", "fr-FR", "it-IT", "en-US"] {
            let (doc, handle, session, _tree) = editor("");
            session.set_locale(locale);
            session.set_punctuation(Some(SmartPunctuationFlags::default()));
            type_text(&handle, &doc, &session, "Que tal?");
            assert_eq!(plain(&doc), "Que tal?", "{locale} must not invert");
        }
    }

    /// **The bug this pins.** The order the real app pushes state in is: the
    /// document's language first, its punctuation flags later (they come from a
    /// row that loads asynchronously). Between the two, the session's flags are
    /// `None` — and Spanish's `¿` does not need them, only the locale. A cache
    /// key that dropped the locale while the flags were `None` left the engine
    /// pinned to locale `""`, so `¿` never fired in exactly this window.
    #[test]
    fn spanish_fires_when_the_language_arrives_before_the_flags() {
        let (doc, handle, session, _tree) = editor("");
        // Language known; flags NOT yet resolved — `set_punctuation` never
        // called, so `punctuation` is `None`.
        session.set_locale("es-ES");
        type_text(&handle, &doc, &session, "Hola?");
        assert_eq!(
            plain(&doc),
            "\u{00BF}Hola?",
            "the opening mark must fire on the locale alone, before any flags load"
        );
    }

    /// And a language *change* while the flags stay unresolved must re-reach the
    /// engine — the same key bug, in its other guise.
    #[test]
    fn a_language_change_reaches_the_engine_with_no_flags_set() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("en-US");
        type_text(&handle, &doc, &session, "Hola?");
        assert_eq!(plain(&doc), "Hola?", "English does not invert");

        session.set_locale("es-ES");
        type_text(&handle, &doc, &session, " Que?");
        assert!(
            plain(&doc).ends_with("\u{00BF}Que?"),
            "the switch to Spanish must take effect, got {:?}",
            plain(&doc)
        );
    }

    /// The dialogue dash opens a paragraph typed as `- `.
    #[test]
    fn a_dialogue_dash_opens_the_paragraph() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("fr-FR");
        session.set_punctuation(Some(SmartPunctuationFlags {
            dialogue_marker: true,
            ..SmartPunctuationFlags::default()
        }));
        type_text(&handle, &doc, &session, "- ");
        assert_eq!(plain(&doc), "\u{2014}\u{00A0}");
    }

    /// And a hyphen anywhere else is a hyphen. A rule that fired mid-line would
    /// mangle ordinary prose, which is why it matches the whole paragraph so far
    /// rather than just the two characters behind the caret.
    #[test]
    fn a_hyphen_mid_paragraph_is_left_alone() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("fr-FR");
        session.set_punctuation(Some(SmartPunctuationFlags {
            dialogue_marker: true,
            ..SmartPunctuationFlags::default()
        }));
        type_text(&handle, &doc, &session, "eh bien - ");
        assert_eq!(plain(&doc), "eh bien - ");
    }

    /// The dialogue dash IS behind a flag, unlike Spanish's marks — it is a
    /// convention some books follow and others do not, where writing `¿` is
    /// simply writing the language.
    #[test]
    fn the_dialogue_dash_is_off_unless_asked_for() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("fr-FR");
        session.set_punctuation(Some(SmartPunctuationFlags::default()));
        type_text(&handle, &doc, &session, "- ");
        assert_eq!(plain(&doc), "- ", "default is off");
    }

    /// A language with no dialogue dash in its table gets none even when asked.
    #[test]
    fn a_language_without_a_dialogue_dash_gets_none() {
        let (doc, handle, session, _tree) = editor("");
        session.set_locale("en-US");
        session.set_punctuation(Some(SmartPunctuationFlags {
            dialogue_marker: true,
            ..SmartPunctuationFlags::default()
        }));
        type_text(&handle, &doc, &session, "- ");
        assert_eq!(plain(&doc), "- ");
    }

    // ── Nested quotes ────────────────────────────────────────────────────────

    fn quotes_for(session: &TextReplacementSession, locale: &str) {
        session.set_locale(locale);
        session.set_punctuation(Some(SmartPunctuationFlags::default()));
    }

    /// French switches to curly doubles inside its guillemets, and the writer
    /// types `"` for every level.
    #[test]
    fn french_quotes_nest_from_guillemets_to_curly_doubles() {
        let (doc, handle, session, _tree) = editor("");
        quotes_for(&session, "fr-FR");
        type_text(&handle, &doc, &session, "\"a \"b\" c\"");
        assert_eq!(plain(&doc), "\u{00AB}a \u{201C}b\u{201D} c\u{00BB}");
    }

    /// Russian nests guillemets into low-high doubles — a different inner mark,
    /// proving the rule reads the locale's own secondary rather than a constant.
    #[test]
    fn russian_quotes_nest_from_guillemets_to_low_high() {
        let (doc, handle, session, _tree) = editor("");
        quotes_for(&session, "ru-RU");
        type_text(&handle, &doc, &session, "\"a \"b\" c\"");
        assert_eq!(plain(&doc), "\u{00AB}a \u{201E}b\u{201C} c\u{00BB}");
    }

    /// Three levels deep, the marks alternate back to the outer style — the
    /// same as every word processor.
    #[test]
    fn a_third_level_alternates_back_to_the_primary() {
        let (doc, handle, session, _tree) = editor("");
        quotes_for(&session, "fr-FR");
        type_text(&handle, &doc, &session, "\"a \"b \"c\"");
        // « then “ then « again at depth 2.
        assert_eq!(plain(&doc), "\u{00AB}a \u{201C}b \u{00AB}c\u{00BB}");
    }

    /// **The reason nesting is gated on double-width secondaries.** English's
    /// inner mark is a single curly quote, which the writer reaches with `'`,
    /// not `"`. Typing `"` inside a quote must stay a double, curled by context
    /// — exactly what Word does — not silently turn into a `’`.
    #[test]
    fn english_quotes_do_not_switch_to_single_when_nested() {
        let (doc, handle, session, _tree) = editor("");
        quotes_for(&session, "en-US");
        type_text(&handle, &doc, &session, "\"a \"b\" c\"");
        assert_eq!(plain(&doc), "\u{201C}a \u{201C}b\u{201D} c\u{201D}");
    }

    /// And the apostrophe hazard the gate exists to avoid: an elision inside a
    /// French quotation must not be counted as a closing mark and throw the
    /// depth off. `l'ami` carries an apostrophe; the closing `"` must still land
    /// on the guillemet.
    #[test]
    fn an_apostrophe_inside_a_quote_does_not_corrupt_the_depth() {
        let (doc, handle, session, _tree) = editor("");
        quotes_for(&session, "fr-FR");
        type_text(&handle, &doc, &session, "\"l'ami\"");
        // « … » — the apostrophe curled to ’, and the close is still a guillemet.
        assert_eq!(plain(&doc), "\u{00AB}l\u{2019}ami\u{00BB}");
    }

    /// With the project's master switch off, nothing expands at all.
    #[test]
    fn the_master_switch_off_disables_every_rule() {
        let doc = TextDocument::new();
        let ed = RichTextEditor::editor(doc.clone());
        let handle = ed.handle();
        let mut tree = WidgetTree::new();
        tree.add(ed);
        tree.layout(SizeProposal::exact(600.0, 400.0));

        let ctx = Rc::new(AppContext::new());
        let work = SingleWork::new(ctx.clone());
        // Left at its default: off.
        let vm = TextReplacementRulesViewModel::new(
            TextReplacementRuleListModel::new(ctx),
            work,
            AppIds::new(),
        );
        let session = TextReplacementSession::new(vm);
        type_text(&handle, &doc, &session, "I saw btw ");
        assert_eq!(plain(&doc), "I saw btw ");
        drop(tree);
    }
}
