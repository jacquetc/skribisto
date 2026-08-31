// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `StatsModel` — live word/char counts derived from the open documents.
//!
//! A thin, cloneable Layer-A handle over [`OpenDocsStore`] + the pure
//! `skribisto_model::counting` policy. It computes counts on demand rather than holding
//! a cached signal: the status-bar word count is one scene's text, cheap to recount, and
//! the widget owns the reactivity (bind the focus + edit signals, debounce, call
//! [`focused_word_count`](StatsModel::focused_word_count) fresh).
//!
//! Written **once** — no `#[cfg]` seam. It touches no backend command directly; it reads
//! the already-seamed `OpenDocsStore` (whose real/mock split lives in `SingleContent`
//! below it) and the pure counting module, so under `--features mocks` it counts the
//! mock documents' fabricated text for real. Same "no-seam" shape as
//! `backup_settings_file` / `examples_list_model`.

use std::rc::Rc;

use teksilo::prelude::Signal;

use skribisto_model::counting::{self, CountMethod, CountingMethodSetting};
use skribisto_model::language;

use crate::models::OpenDocsStore;

struct Inner {
    open_docs: OpenDocsStore,
    /// The focused editor tab's item id (`EditorsViewModel::active_item`).
    active_item: Signal<Option<u64>>,
    /// The user's counting-method preference (the Goals settings pane's
    /// `goals.counting_method`, live). Resolved per scene from the effective language
    /// at count time; `Auto` picks CJK-smart for zh/ja and Unicode words elsewhere.
    method: Signal<CountingMethodSetting>,
}

/// Cloneable handle over the open-documents counting surface.
#[derive(Clone)]
pub struct StatsModel {
    inner: Rc<Inner>,
}

impl StatsModel {
    pub fn new(
        open_docs: OpenDocsStore,
        active_item: Signal<Option<u64>>,
        method: Signal<CountingMethodSetting>,
    ) -> Self {
        Self {
            inner: Rc::new(Inner {
                open_docs,
                active_item,
                method,
            }),
        }
    }

    /// The focused editor item id — bind a widget's rebuild to it.
    pub fn active_item(&self) -> Signal<Option<u64>> {
        self.inner.active_item.clone()
    }

    /// The aggregate "an edit happened" counter — bind (debounced) so the count
    /// refreshes as the writer types.
    pub fn edited_signal(&self) -> Signal<u64> {
        self.inner.open_docs.edited_any()
    }

    /// The live counting-method preference — bind it so the count re-derives when the
    /// writer changes the method in Settings (the status bar is behind that modal, so
    /// nothing else would trigger a rebuild).
    pub fn method_signal(&self) -> Signal<CountingMethodSetting> {
        self.inner.method.clone()
    }

    /// The live word count of the focused item's main prose, or `None` when nothing
    /// prose-bearing is focused (no project, a container tab, an unopened item).
    ///
    /// Counts the open document's **plain text** (Djot already stripped, cached until
    /// the next edit) with the method resolved from the item's effective language —
    /// so the number tracks the writer's current typing, not the last-saved text.
    pub fn focused_word_count(&self) -> Option<usize> {
        Some(self.focused_counts()?.words)
    }

    /// Word + character counts of the focused item's main prose (`None` if not
    /// prose-bearing / not open).
    pub fn focused_counts(&self) -> Option<counting::WordCharCounts> {
        let id = self.inner.active_item.get()?;
        let doc = self.inner.open_docs.peek(id)?;
        // Only prose-bearing items (Scene / ChapterScene / Note) own a `main` field.
        let field = doc.main.as_ref()?;
        let text = field.doc.to_plain_text().ok()?;
        // This is the one counter that sees *parsed* text rather than raw Djot,
        // so it needs the plain-text entry point — the markers read `* * *`
        // here, not `\* \* \*`. Without this the status bar would disagree with
        // the pace history the moment the author inserts a break.
        let text = skribisto_model::scene_break::strip_markers_plain(&text);
        let lang = self.inner.open_docs.effective_language(id);
        let method = counting::resolve_method(
            self.inner.method.get(),
            CountMethod::UnicodeWords,
            language::primary(&lang),
        );
        Some(counting::count(&text, method))
    }
}
