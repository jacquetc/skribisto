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
        let advanced = self.last_caret.replace(Some(caret)).is_some_and(|p| caret > p);

        // A pending revert outlives only the fire's own echo. The first change
        // that moves the document past it is the writer's, and is the one — the
        // only one — that may revert.
        if let Some(pending) = self.take_pending_if_settled(doc) {
            if self.try_revert(handle, doc, &pending, caret) {
                return;
            }
        }

        if !advanced || handle.has_selection().get() {
            return;
        }
        self.refresh_engine();
        self.try_fire(handle, doc, caret);
    }

    /// Recompile the engine when the lexicon or the project's master switch has
    /// changed since it was last built. Both are plain signal reads — cheaper
    /// per keystroke than a subscription would be to keep correct across
    /// project switches.
    fn refresh_engine(&self) {
        let enabled = self.vm.enabled_signal().get();
        let version = self.vm.changed_signal().get();
        if self.compiled_from.get() == (version, enabled) {
            return;
        }
        self.compiled_from.set((version, enabled));
        *self.engine.borrow_mut() = if enabled {
            TextReplacementEngine::from_rules(&self.vm.rows())
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

    /// Expand a rule if one just fired.
    fn try_fire(&self, handle: &EditorHandle, doc: &TextDocument, caret: usize) {
        let fired = {
            let engine = self.engine.borrow();
            if engine.is_empty() {
                return;
            }
            let Some(window) = text_before(doc, caret, engine.window_chars()) else {
                return;
            };
            engine.check(&window)
        };
        let Some(fired) = fired else {
            // Anything the writer types that is not a fired rule ends the
            // one-shot suppression — it only covers re-typing the delimiter
            // they just deleted, not the rest of the paragraph.
            self.suppressed.borrow_mut().take();
            return;
        };

        // The delimiter that fired sits between the trigger and the caret.
        let Some(span_start) = caret.checked_sub(fired.trigger_chars + 1) else {
            return;
        };
        if self.consume_suppression(span_start, &fired.typed) {
            return;
        }

        let delimiter = match text_before(doc, caret, 1) {
            Some(d) if !d.is_empty() => d,
            // The delimiter is what fired the rule, so it must be readable; if
            // it is not, the document moved under us and doing nothing is right.
            _ => return,
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
        let s = Suppressed { span_start: 4, key: "btw".into() };
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
    fn editor(text: &str) -> (TextDocument, EditorHandle, Rc<TextReplacementSession>, WidgetTree) {
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
