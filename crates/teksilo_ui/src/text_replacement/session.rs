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

use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::EditorHandle;

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
    /// A different revert shape, for a substitution that inserted **more**
    /// characters than the writer typed — the French guillemet plus its inner
    /// no-break space (`"` → `«\u{202F}` / `\u{202F}»`). A lexicon revert fires
    /// when the writer deletes the *delimiter* the fire re-added; there is no
    /// delimiter here, so instead: the moment the writer backspaces into the
    /// substitution (deleting its last character), collapse the whole thing back
    /// to the one `typed` glyph. Without this, one Backspace removes only the
    /// visible mark and strands the invisible no-break space.
    collapse: bool,
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
        // Collapse revert: the writer backspaced into a substitution that
        // inserted more characters than they typed (a spaced guillemet). One
        // Backspace has just removed its last character; finish the job by
        // restoring the single `typed` glyph, so the companion no-break space is
        // not left stranded and one press fully undoes the auto-substitution.
        if pending.collapse && caret + 1 == pending.span_start + pending.replacement_chars {
            let kept = pending.replacement_chars - 1;
            let before = text_before(doc, caret, kept);
            let expected: String = pending.replacement.chars().take(kept).collect();
            if before.as_deref() == Some(expected.as_str()) {
                self.apply(|| handle.replace_range(pending.span_start, caret, &pending.typed));
                self.last_caret.set(Some(handle.cursor_position()));
                self.last_revision.set(Some(doc.content_revision()));
                return true;
            }
            return false;
        }
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
                // No lexicon to fire — but a suppression left by an earlier
                // revert must not outlive the keystroke it was meant for, or it
                // would silently swallow a real fire once the lexicon comes back.
                self.suppressed.borrow_mut().take();
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
            collapse: false,
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
        if fired.prepend {
            // Insert the mark at `span_start`, delete nothing (so the clause
            // between keeps its formatting), then put the caret back where the
            // writer left it — shifted right by the one glyph we inserted.
            let shift = fired.replacement.chars().count();
            self.apply(|| {
                handle.replace_range(span_start, span_start, &fired.replacement);
                handle.select_range(caret + shift, caret + shift);
            });
        } else {
            self.apply(|| handle.replace_range(span_start, caret, &fired.replacement));
            self.note_expanded_substitution(span_start, &fired, doc);
        }
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
    /// ## Revert, only where the substitution grew
    ///
    /// A punctuation substitution that is one-glyph-in-one-glyph-out (a curled
    /// quote, an em dash) leaves no pending revert: Backspace deletes it like any
    /// character, and Ctrl+Z restores the literal since it is one
    /// [`EditorHandle::replace_range`] / one undo entry — the same as Word.
    ///
    /// The exception is a substitution that inserted MORE characters than the
    /// writer typed — the French guillemet plus its inner no-break space. There,
    /// a plain per-character Backspace would remove the visible mark and strand
    /// the invisible space, so [`note_expanded_substitution`](Self::note_expanded_substitution)
    /// records a collapse revert that restores the single typed glyph on the
    /// first Backspace instead.
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
        self.note_expanded_substitution(span_start, &fired, doc);
        self.last_caret.set(Some(handle.cursor_position()));
        self.last_revision.set(Some(doc.content_revision()));
    }

    /// Record a collapse revert for a punctuation substitution that inserted
    /// **more** characters than the writer typed — the only such case is the
    /// French guillemet plus its inner no-break space. Everything else (one glyph
    /// in for one-or-more typed) leaves no revert and is deleted char-by-char.
    /// See [`PendingRevert::collapse`].
    fn note_expanded_substitution(
        &self,
        span_start: usize,
        fired: &super::typography::Fired,
        doc: &TextDocument,
    ) {
        if fired.replacement.chars().count() <= fired.typed.chars().count() {
            return;
        }
        *self.pending.borrow_mut() = Some(PendingRevert {
            span_start,
            replacement_chars: fired.replacement.chars().count(),
            replacement: fired.replacement.clone(),
            typed: fired.typed.clone(),
            revision: doc.content_revision(),
            collapse: true,
        });
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
mod tests;

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
mod live_editor_tests;
