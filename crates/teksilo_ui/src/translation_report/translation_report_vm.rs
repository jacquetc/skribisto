// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Report a Translation Error window's state and actions.
//!
//! One per process (Tier 1): nothing in a report depends on an open project, and the
//! window is one window whichever project window opened it. Held in `app_state` like
//! the Help window's, so every door opens the same report and a writer who goes to
//! look at a menu and comes back finds the form as they left it.
//!
//! The steps, each a method the panel calls:
//!
//! 1. **The language** ([`TranslationReportViewModel::choose_language`]). It follows the interface language
//!    until the writer picks one, so a writer who switches the interface to check a
//!    string carries the report along.
//! 2. **The text** ([`TranslationReportViewModel::query`], [`TranslationReportViewModel::pick_row`], [`TranslationReportViewModel::cannot_find`]).
//!    Rows are groups of identical values ([`super::matching::group`]); a group of
//!    several places asks which one ([`TranslationReportViewModel::choose_place`]), or all of them.
//! 3. **What is wrong and how it should read** ([`TranslationReportViewModel::choose_kind`], the explanation
//!    and suggestion documents). Picking a text prefills the suggestion with the value
//!    as written, mnemonic and placeholders included, so the writer edits instead of
//!    retyping, and a fix to a plural keeps every form.
//! 4. **Continue on GitHub** ([`TranslationReportViewModel::prepare_continue`]): the link, and the whole
//!    report for the clipboard when the link could not carry all of it.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use teksilo::core::ObserverHandle;
use teksilo::data::{ListModel, SelectionMode, SelectionModel};
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;

use super::catalogue::{Area, Catalogue, CatalogueEntry, SOURCE_LOCALE, supported_locale};
use super::matching::{
    EnglishHit, Hit, LocaleIndex, ResultGroup, english_breakdown, english_hits, group,
};
use super::report::{BuildInfo, Link, ProblemKind, Report, Target};
use crate::startup::SUPPORTED_LOCALES;

/// How many rows the result list shows. The search keeps its best
/// [`super::matching::MAX_HITS`]; past this many groups the writer should type more.
const MAX_ROWS: usize = 80;

/// The result area's pages, in the order the panel's `Switcher` stacks them: an
/// invitation to type, a note that nothing matches, and the list.
pub(crate) const RESULTS_PROMPT: usize = 0;
pub(crate) const RESULTS_NONE: usize = 1;
pub(crate) const RESULTS_LIST: usize = 2;

/// One row of the result list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ResultRow {
    /// The text, as the screen shows it.
    pub title: String,
    /// Where it is, how many places, and the English it translates.
    pub subtitle: String,
}

/// What a row stands for.
#[derive(Clone, Debug)]
enum RowSource {
    Group(ResultGroup),
    English(EnglishHit),
}

/// A page the window was opened for, from the Help window's footer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scope {
    /// A Djot help topic: its paragraphs.
    HelpPage(String),
    /// A help topic built from a tooltip: the tooltip's text and its "more".
    Tooltip(String),
}

impl Scope {
    fn contains(&self, entry: &CatalogueEntry) -> bool {
        match self {
            Scope::HelpPage(key) => entry.catalogue == Catalogue::Help && entry.id == *key,
            Scope::Tooltip(key) => {
                entry.catalogue == Catalogue::App
                    && (entry.id == *key || entry.id.strip_suffix("-more") == Some(key.as_str()))
            }
        }
    }
}

/// One place a picked text appears.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Member {
    entry: usize,
    composed: Option<usize>,
}

/// The text the report is about.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Picked {
    Found {
        locale: &'static str,
        members: Vec<Member>,
    },
    NotFound,
}

/// Which of a group's places the report is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum PlaceChoice {
    All,
    One(usize),
}

/// Where the report stands after **Continue on GitHub**.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Phase {
    Editing,
    /// The browser was asked to open the link; `copied` when the report had to go
    /// through the clipboard too.
    Continued {
        copied: bool,
    },
    /// The browser could not be asked, with the error.
    OpenFailed(String),
}

/// What the last copy did, for the line beside the buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Notice {
    TextCopied,
    LinkCopied,
    CopyFailed,
}

/// What the window shows about the picked text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Card {
    /// The text as the screen shows it; for a composed label, the whole sentence.
    pub text: String,
    /// Where it appears.
    pub location: String,
    /// The English it translates, when there is one to show.
    pub english: Option<String>,
    /// The message id, for the writer who knows the catalogues.
    pub id: String,
    /// Where it is written: the file in its repository, and the line.
    pub path: String,
    /// What a fix to it changes, when that is not the obvious.
    pub note: Option<String>,
    /// For a group of several places: the choices, `All` first.
    pub places: Vec<(PlaceChoice, String)>,
}

/// What **Continue on GitHub** does: the link to open, and the report to put on the
/// clipboard first when the link could not carry all of it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ContinuePlan {
    pub link: Link,
    pub clipboard: Option<String>,
}

#[derive(Clone)]
pub struct TranslationReportViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    /// The language the writer picked; `None` follows the interface.
    chosen_language: Signal<Option<String>>,
    /// The interface language last seen, which the report follows until the writer
    /// picks one.
    interface_language: Signal<String>,
    /// [`TranslationReportViewModel::language`] as a tag, for the fields that hold
    /// text in it: kept in step by the two methods that change either input.
    target_language: Signal<Option<LanguageTag>>,
    query: Signal<String>,
    rows: ListModel<ResultRow>,
    /// Which of the result area's pages shows: see [`RESULTS_PROMPT`].
    results_page: Signal<usize>,
    sources: RefCell<Vec<RowSource>>,
    selection: SelectionModel,
    scope: Signal<Option<Scope>>,
    picked: Signal<Option<Picked>>,
    place: Signal<Option<PlaceChoice>>,
    kind: Signal<Option<ProblemKind>>,
    seen_at: Signal<String>,
    explanation: TextDocument,
    suggestion: TextDocument,
    /// Bumped by the panel on every edit of either document, so what depends on their
    /// text re-evaluates.
    edits: Signal<u64>,
    can_continue: Signal<bool>,
    phase: Signal<Phase>,
    notice: Signal<Option<Notice>>,
    card_version: Signal<u64>,
    last_link: RefCell<Option<String>>,
    indexes: RefCell<HashMap<&'static str, Rc<LocaleIndex>>>,
    /// Re-search when the query, the language or the scope changes; revalidate when
    /// anything Continue depends on changes.
    _observers: RefCell<Vec<ObserverHandle>>,
}

impl Default for TranslationReportViewModel {
    fn default() -> Self {
        Self::new()
    }
}

impl TranslationReportViewModel {
    pub fn new() -> Self {
        let inner = Rc::new(Inner {
            chosen_language: Signal::new(None),
            interface_language: Signal::new(interface_tag()),
            target_language: Signal::new(LanguageTag::parse(&interface_tag()).ok()),
            query: Signal::new(String::new()),
            rows: ListModel::new(),
            results_page: Signal::new(RESULTS_PROMPT),
            sources: RefCell::new(Vec::new()),
            selection: SelectionModel::new(SelectionMode::Single),
            scope: Signal::new(None),
            picked: Signal::new(None),
            place: Signal::new(None),
            kind: Signal::new(None),
            seen_at: Signal::new(String::new()),
            explanation: TextDocument::new(),
            suggestion: TextDocument::new(),
            edits: Signal::new(0),
            can_continue: Signal::new(false),
            phase: Signal::new(Phase::Editing),
            notice: Signal::new(None),
            card_version: Signal::new(0),
            last_link: RefCell::new(None),
            indexes: RefCell::new(HashMap::new()),
            _observers: RefCell::new(Vec::new()),
        });
        let vm = Self { inner };
        vm.observe();
        vm
    }

    /// Wire the re-search and the revalidation. Each closure holds a `Weak`, so the
    /// observers the view-model owns do not keep it alive.
    fn observe(&self) {
        let weak = Rc::downgrade(&self.inner);
        let research = move |w: &Weak<Inner>| {
            if let Some(inner) = w.upgrade() {
                Self { inner }.refresh();
            }
        };
        let revalidate = move |w: &Weak<Inner>| {
            if let Some(inner) = w.upgrade() {
                Self { inner }.revalidate();
            }
        };
        let i = &self.inner;
        let mut handles = Vec::new();
        {
            let w = weak.clone();
            handles.push(i.query.observe(move |_| research(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.chosen_language.observe(move |_| research(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.interface_language.observe(move |_| research(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.scope.observe(move |_| research(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.edits.observe(move |_| revalidate(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.kind.observe(move |_| revalidate(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.picked.observe(move |_| revalidate(&w)));
        }
        {
            let w = weak.clone();
            handles.push(i.seen_at.observe(move |_| revalidate(&w)));
        }
        *i._observers.borrow_mut() = handles;
    }

    // ── What the panel binds ────────────────────────────────────────────────

    pub(crate) fn query(&self) -> Signal<String> {
        self.inner.query.clone()
    }
    pub(crate) fn rows(&self) -> ListModel<ResultRow> {
        self.inner.rows.clone()
    }
    /// [`RESULTS_PROMPT`], [`RESULTS_NONE`] or [`RESULTS_LIST`].
    pub(crate) fn results_page(&self) -> Signal<usize> {
        self.inner.results_page.clone()
    }
    pub(crate) fn selection(&self) -> SelectionModel {
        self.inner.selection.clone()
    }
    pub(crate) fn kind(&self) -> Signal<Option<ProblemKind>> {
        self.inner.kind.clone()
    }
    pub(crate) fn place(&self) -> Signal<Option<PlaceChoice>> {
        self.inner.place.clone()
    }
    pub(crate) fn seen_at(&self) -> Signal<String> {
        self.inner.seen_at.clone()
    }
    pub(crate) fn explanation(&self) -> TextDocument {
        self.inner.explanation.clone()
    }
    pub(crate) fn suggestion(&self) -> TextDocument {
        self.inner.suggestion.clone()
    }
    pub(crate) fn can_continue(&self) -> Signal<bool> {
        self.inner.can_continue.clone()
    }
    pub(crate) fn phase(&self) -> Signal<Phase> {
        self.inner.phase.clone()
    }
    pub(crate) fn notice(&self) -> Signal<Option<Notice>> {
        self.inner.notice.clone()
    }
    pub(crate) fn scope(&self) -> Signal<Option<Scope>> {
        self.inner.scope.clone()
    }
    /// Bumped whenever the picked text or its place changes, for the card to rebuild on.
    pub(crate) fn card_version(&self) -> Signal<u64> {
        self.inner.card_version.clone()
    }
    /// Whether a text has been picked, or the writer said they could not find it.
    #[cfg(test)]
    pub(crate) fn has_target(&self) -> bool {
        self.inner.picked.get().is_some()
    }
    /// Whether the writer said they could not find the text.
    pub(crate) fn is_not_found(&self) -> bool {
        matches!(self.inner.picked.get(), Some(Picked::NotFound))
    }

    /// The language the report is about.
    pub(crate) fn language(&self) -> String {
        self.inner
            .chosen_language
            .get()
            .unwrap_or_else(|| self.inner.interface_language.get())
    }

    /// The languages the window offers: every shipped locale.
    pub(crate) fn languages() -> Vec<String> {
        SUPPORTED_LOCALES.iter().map(|s| s.to_string()).collect()
    }

    /// `français (fr-FR)`: a language as its own speakers write it, then its tag.
    pub(crate) fn language_label(tag: &str) -> String {
        let name = tag
            .parse()
            .ok()
            .and_then(|id| teksilo::i18n::language_endonym(&id));
        match name {
            Some(name) => format!("{name} ({tag})"),
            None => tag.to_string(),
        }
    }

    // ── Step 1: the language ─────────────────────────────────────────────────

    /// The writer picked a language; the report stops following the interface.
    pub(crate) fn choose_language(&self, tag: &str) {
        if supported_locale(tag).is_none() {
            return;
        }
        if self.inner.chosen_language.get().as_deref() != Some(tag) {
            self.clear_pick();
            self.inner.chosen_language.set(Some(tag.to_string()));
            self.sync_target_language();
        }
    }

    /// The language the report is about, as a tag, for `content_language`: the
    /// search field, the picked text and the suggestion hold text in it, while the
    /// window around them stays in the interface's.
    pub(crate) fn target_language(&self) -> Signal<Option<LanguageTag>> {
        self.inner.target_language.clone()
    }

    fn sync_target_language(&self) {
        let tag = LanguageTag::parse(&self.language()).ok();
        if self.inner.target_language.get() != tag {
            self.inner.target_language.set(tag);
        }
    }

    /// The interface language changed. The report follows it while the writer has not
    /// picked one.
    pub(crate) fn interface_language_changed(&self) {
        let tag = interface_tag();
        if self.inner.interface_language.get() != tag {
            if self.inner.chosen_language.get().is_none() {
                self.clear_pick();
            }
            self.inner.interface_language.set(tag);
            self.sync_target_language();
        }
    }

    // ── Step 2: the text ─────────────────────────────────────────────────────

    /// Open the window on one help page: the result list shows only its text.
    pub(crate) fn open_for(&self, scope: Option<Scope>) {
        self.clear_pick();
        self.inner.query.set(String::new());
        self.inner.scope.set(scope);
    }

    /// The index of `locale`, built on first use and kept.
    fn index(&self, locale: &str) -> Option<Rc<LocaleIndex>> {
        let tag = supported_locale(locale)?;
        if let Some(index) = self.inner.indexes.borrow().get(tag) {
            return Some(index.clone());
        }
        let index = Rc::new(LocaleIndex::build(tag)?);
        self.inner.indexes.borrow_mut().insert(tag, index.clone());
        Some(index)
    }

    /// Search again and refill the list. The pick stays: re-typing to compare with
    /// another text should not throw away a half-written suggestion.
    fn refresh(&self) {
        let language = self.language();
        let (Some(index), Some(english)) = (self.index(&language), self.index(SOURCE_LOCALE))
        else {
            self.set_rows(Vec::new());
            return;
        };
        let query = self.inner.query.get();
        let scope = self.inner.scope.get();
        let mut sources: Vec<RowSource> = Vec::new();

        let hits: Vec<Hit> = if query.trim().is_empty() {
            match &scope {
                // A scoped window lists its page before anything is typed.
                Some(scope) => (0..index.entries.len())
                    .filter(|i| scope.contains(&index.entries[*i]))
                    .map(|entry| Hit {
                        entry,
                        rank: super::matching::Rank::Whole,
                        composed: None,
                    })
                    .collect(),
                None => Vec::new(),
            }
        } else {
            index
                .search(&query)
                .into_iter()
                .filter(|h| {
                    scope
                        .as_ref()
                        .is_none_or(|s| s.contains(&index.entries[h.entry]))
                })
                .collect()
        };
        sources.extend(group(&index, &hits).into_iter().map(RowSource::Group));
        if scope.is_none() && !query.trim().is_empty() {
            sources.extend(
                english_hits(&index, &english, &query)
                    .into_iter()
                    .map(RowSource::English),
            );
        }
        sources.truncate(MAX_ROWS);

        let rows = sources
            .iter()
            .map(|s| row_for(&index, &english, s))
            .collect();
        *self.inner.sources.borrow_mut() = sources;
        self.set_rows(rows);
    }

    fn set_rows(&self, rows: Vec<ResultRow>) {
        let typed = !self.inner.query.get().trim().is_empty() || self.inner.scope.get().is_some();
        let page = match (rows.is_empty(), typed) {
            (true, false) => RESULTS_PROMPT,
            (true, true) => RESULTS_NONE,
            (false, _) => RESULTS_LIST,
        };
        self.inner.selection.clear();
        self.inner.rows.replace_all(rows);
        self.inner.results_page.set(page);
    }

    /// The writer picked row `i`.
    pub(crate) fn pick_row(&self, i: usize) {
        let Some(index) = self.index(&self.language()) else {
            return;
        };
        let Some(source) = self.inner.sources.borrow().get(i).cloned() else {
            return;
        };
        let members: Vec<Member> = match &source {
            RowSource::Group(g) => g
                .hits
                .iter()
                .map(|h| Member {
                    entry: h.entry,
                    composed: h.composed,
                })
                .collect(),
            RowSource::English(h) => match h.translation {
                Some(entry) => vec![Member {
                    entry,
                    composed: None,
                }],
                None => return,
            },
        };
        let Some(first) = members.first().copied() else {
            return;
        };
        self.inner.selection.select(i);
        self.inner.place.set(Some(PlaceChoice::One(0)));
        self.inner.picked.set(Some(Picked::Found {
            locale: index.locale,
            members,
        }));
        let entry = &index.entries[first.entry];
        let kind_hint = match &source {
            RowSource::English(h) if h.untranslated => Some(ProblemKind::NotTranslated),
            _ => None,
        };
        if let Some(kind) = kind_hint {
            self.inner.kind.set(Some(kind));
        }
        self.prefill(&index, entry, first.composed);
        self.bump_card();
    }

    /// The writer could not find the text: the report describes where they saw it.
    pub(crate) fn cannot_find(&self) {
        self.inner.selection.clear();
        self.inner.place.set(None);
        self.inner.picked.set(Some(Picked::NotFound));
        let _ = self.inner.suggestion.set_plain_text("");
        self.edited();
        self.bump_card();
    }

    fn bump_card(&self) {
        self.inner
            .card_version
            .set(self.inner.card_version.get().wrapping_add(1));
    }

    /// Which place of a group the report is about.
    pub(crate) fn choose_place(&self, choice: PlaceChoice) {
        let Some(Picked::Found { locale, members }) = self.inner.picked.get() else {
            return;
        };
        let valid = match choice {
            PlaceChoice::All => members.len() > 1,
            PlaceChoice::One(n) => n < members.len(),
        };
        if !valid || self.inner.place.get() == Some(choice) {
            return;
        }
        self.inner.place.set(Some(choice));
        self.bump_card();
        // Changing which place is reported changes what the suggestion should start
        // from only when it still reads as the old place's value.
        if let (PlaceChoice::One(n), Some(index)) = (choice, self.index(locale)) {
            let member = members[n];
            self.prefill(&index, &index.entries[member.entry], member.composed);
        }
    }

    /// Put the picked value in the suggestion, as written.
    fn prefill(&self, index: &LocaleIndex, entry: &CatalogueEntry, composed: Option<usize>) {
        let text = match composed {
            Some(c) => {
                let other = &index.entries[c];
                format!(
                    "{} = {}\n{} = {}",
                    entry.id,
                    entry.source.trim(),
                    other.id,
                    other.source.trim()
                )
            }
            None => entry.source.clone(),
        };
        let _ = self.inner.suggestion.set_plain_text(&text);
        self.edited();
    }

    fn clear_pick(&self) {
        self.inner.selection.clear();
        self.inner.picked.set(None);
        self.inner.place.set(None);
        self.bump_card();
    }

    /// The card for the picked text, `None` before a pick or for a text not found.
    pub(crate) fn card(&self) -> Option<Card> {
        let Some(Picked::Found { locale, members }) = self.inner.picked.get() else {
            return None;
        };
        let index = self.index(locale)?;
        let english = self.index(SOURCE_LOCALE)?;
        let place = self.inner.place.get().unwrap_or(PlaceChoice::One(0));
        let shown_member = match place {
            PlaceChoice::One(n) => members.get(n).copied(),
            PlaceChoice::All => members.first().copied(),
        }?;
        let entry = &index.entries[shown_member.entry];
        let text = composed_text(&index, shown_member);
        let english_text = english
            .counterpart(entry)
            .map(|e| match shown_member.composed {
                Some(c) => {
                    let other = english.counterpart(&index.entries[c]);
                    composed_english(&english, e, other)
                }
                None => english.entries[e].shown().to_string(),
            })
            .filter(|s| !s.is_empty());
        let note = note_for(entry, shown_member.composed.is_some());
        let places = if members.len() > 1 {
            std::iter::once((
                PlaceChoice::All,
                tr!(translation_report_place_all(count = members.len() as i64)).resolve_now(),
            ))
            .chain(members.iter().enumerate().map(|(n, m)| {
                let e = &index.entries[m.entry];
                let en = english
                    .counterpart(e)
                    .map(|i| english.entries[i].shown().to_string())
                    .unwrap_or_default();
                let label = if en.is_empty() {
                    format!("{} · {}", location_hint(e), e.id)
                } else {
                    format!("{} · \u{201C}{}\u{201D} · {}", location_hint(e), en, e.id)
                };
                (PlaceChoice::One(n), label)
            }))
            .collect()
        } else {
            Vec::new()
        };
        Some(Card {
            text,
            location: location_hint(entry),
            english: english_text,
            id: match shown_member.composed {
                Some(c) => format!("{} + {}", entry.id, index.entries[c].id),
                None => entry.id.clone(),
            },
            path: format!("{}:{}", entry.repository_path(), entry.line),
            note,
            places,
        })
    }

    /// The hints beside the suggestion, for what the picked value contains.
    pub(crate) fn suggestion_hints(&self) -> Vec<String> {
        let Some(Picked::Found { locale, members }) = self.inner.picked.get() else {
            return Vec::new();
        };
        let Some(index) = self.index(locale) else {
            return Vec::new();
        };
        let Some(member) = members.first() else {
            return Vec::new();
        };
        let entry = &index.entries[member.entry];
        let mut hints = Vec::new();
        if entry.catalogue != Catalogue::Help && entry.source.contains('&') {
            hints.push(tr!(translation_report_hint_mnemonic()).resolve_now());
        }
        if entry.has_selector {
            hints.push(tr!(translation_report_hint_select()).resolve_now());
        } else if entry.source.contains("{ $") || entry.source.contains("{$") {
            hints.push(tr!(translation_report_hint_placeholder()).resolve_now());
        }
        hints
    }

    // ── Step 3: what is wrong ────────────────────────────────────────────────

    pub(crate) fn choose_kind(&self, kind: ProblemKind) {
        self.inner.kind.set(Some(kind));
    }

    /// The panel calls this on every edit of the explanation or the suggestion.
    pub(crate) fn edited(&self) {
        self.inner.edits.set(self.inner.edits.get().wrapping_add(1));
    }

    fn revalidate(&self) {
        self.inner.can_continue.set(self.report().is_some());
    }

    // ── Step 4: the report ───────────────────────────────────────────────────

    /// The report as filled in so far, or `None` while something it needs is missing:
    /// a text (or where it was seen), what is wrong, and a better translation.
    pub(crate) fn report(&self) -> Option<Report> {
        let kind = self.inner.kind.get()?;
        let suggestion = self.inner.suggestion.to_plain_text().unwrap_or_default();
        if suggestion.trim().is_empty() {
            return None;
        }
        let explanation = self.inner.explanation.to_plain_text().unwrap_or_default();
        let build = BuildInfo::current();
        match self.inner.picked.get()? {
            Picked::NotFound => {
                let seen_at = self.inner.seen_at.get();
                if seen_at.trim().is_empty() {
                    return None;
                }
                Some(Report {
                    locale: self.language(),
                    target: Target::NotFound { seen_at },
                    current: String::new(),
                    english: String::new(),
                    kind,
                    explanation,
                    suggestion,
                    build,
                })
            }
            Picked::Found { locale, members } => {
                let index = self.index(locale)?;
                let english = self.index(SOURCE_LOCALE)?;
                let place = self.inner.place.get().unwrap_or(PlaceChoice::One(0));
                let (member, also) = match place {
                    PlaceChoice::One(n) => (*members.get(n)?, Vec::new()),
                    PlaceChoice::All => (
                        *members.first()?,
                        members[1..]
                            .iter()
                            .map(|m| index.entries[m.entry].id.clone())
                            .collect(),
                    ),
                };
                let entry = &index.entries[member.entry];
                let english_text = english
                    .counterpart(entry)
                    .map(|e| match member.composed {
                        Some(c) => {
                            composed_english(&english, e, english.counterpart(&index.entries[c]))
                        }
                        None => english.entries[e].shown().to_string(),
                    })
                    .unwrap_or_default();
                Some(Report {
                    locale: locale.to_string(),
                    target: Target::Message {
                        catalogue: entry.catalogue.slug(),
                        area: entry.area.slug(),
                        file: entry.file.clone(),
                        line: entry.line,
                        id: entry.id.clone(),
                        composed: member.composed.map(|c| index.entries[c].id.clone()),
                        also,
                        upstream: entry.is_upstream(),
                    },
                    current: composed_text(&index, member),
                    english: english_text,
                    kind,
                    explanation,
                    suggestion,
                    build,
                })
            }
        }
    }

    /// What **Continue on GitHub** will do, or `None` while the report is incomplete
    /// or the running edition declares no translation form.
    pub(crate) fn prepare_continue(&self) -> Option<ContinuePlan> {
        let form = crate::identity::links().translation_form?;
        let report = self.report()?;
        let link = report.link(&form);
        let clipboard = link.overflowed().then(|| report.clipboard_text());
        Some(ContinuePlan { link, clipboard })
    }

    /// Continue on GitHub: copy the report when the link could not carry it, then ask
    /// the desktop to open the link.
    pub(crate) fn continue_on_github(&self, ctx: &mut EventContext) {
        let Some(plan) = self.prepare_continue() else {
            return;
        };
        let clipboard = ctx.app_state::<ClipboardHandle>().cloned();
        self.continue_with(&plan, clipboard.as_ref(), |url| {
            crate::shared::external_link::try_open_external_link(url).map_err(|e| match e {
                crate::shared::external_link::OpenLinkError::Refused => String::new(),
                crate::shared::external_link::OpenLinkError::Failed(error) => error,
            })
        });
    }

    /// [`Self::continue_on_github`] with the clipboard and the opener passed in.
    pub(crate) fn continue_with(
        &self,
        plan: &ContinuePlan,
        clipboard: Option<&ClipboardHandle>,
        open: impl FnOnce(&str) -> Result<(), String>,
    ) {
        let copied = match &plan.clipboard {
            Some(text) => clipboard.is_some_and(|c| c.set_text(text).is_ok()),
            None => false,
        };
        *self.inner.last_link.borrow_mut() = Some(plan.link.url.clone());
        self.inner.notice.set(None);
        match open(&plan.link.url) {
            Ok(()) => self.inner.phase.set(Phase::Continued { copied }),
            Err(error) => self.inner.phase.set(Phase::OpenFailed(error)),
        }
        if plan.clipboard.is_some() && !copied {
            self.inner.notice.set(Some(Notice::CopyFailed));
        }
    }

    /// Copy the whole report, for GitHub, Discord or an email.
    pub(crate) fn copy_text(&self, clipboard: Option<&ClipboardHandle>) {
        let Some(report) = self.report() else {
            return;
        };
        let ok = clipboard.is_some_and(|c| c.set_text(&report.clipboard_text()).is_ok());
        self.inner.notice.set(Some(if ok {
            Notice::TextCopied
        } else {
            Notice::CopyFailed
        }));
    }

    /// Copy the link last opened (or the one Continue would open), for a browser that
    /// did not come up.
    pub(crate) fn copy_link(&self, clipboard: Option<&ClipboardHandle>) {
        let url = self
            .inner
            .last_link
            .borrow()
            .clone()
            .or_else(|| self.prepare_continue().map(|p| p.link.url));
        let Some(url) = url else {
            return;
        };
        let ok = clipboard.is_some_and(|c| c.set_text(&url).is_ok());
        self.inner.notice.set(Some(if ok {
            Notice::LinkCopied
        } else {
            Notice::CopyFailed
        }));
    }

    /// Start a new report in the same language.
    pub(crate) fn report_another(&self) {
        self.clear_pick();
        self.inner.query.set(String::new());
        self.inner.scope.set(None);
        self.inner.kind.set(None);
        self.inner.seen_at.set(String::new());
        let _ = self.inner.explanation.set_plain_text("");
        let _ = self.inner.suggestion.set_plain_text("");
        *self.inner.last_link.borrow_mut() = None;
        self.inner.notice.set(None);
        self.inner.phase.set(Phase::Editing);
        self.edited();
    }
}

/// The interface language's tag, as the app names it.
fn interface_tag() -> String {
    crate::shared::ui_locale::current_tag()
        .and_then(|t| supported_locale(&t))
        .unwrap_or(SOURCE_LOCALE)
        .to_string()
}

/// A composed label as the screen shows it: the outer value with its one variable
/// filled by the inner one, or the two joined when the outer has several.
fn composed_text(index: &LocaleIndex, member: Member) -> String {
    let outer = index.entries[member.entry].shown().to_string();
    let Some(c) = member.composed else {
        return outer;
    };
    let inner = index.entries[c].shown();
    fill_one_variable(&outer, inner).unwrap_or_else(|| format!("{outer} + {inner}"))
}

fn composed_english(english: &LocaleIndex, outer: usize, inner: Option<usize>) -> String {
    let outer = english.entries[outer].shown().to_string();
    match inner {
        Some(i) => {
            let inner = english.entries[i].shown();
            fill_one_variable(&outer, inner).unwrap_or_else(|| format!("{outer} + {inner}"))
        }
        None => outer,
    }
}

/// `outer` with its only `{ $name }` replaced by `inner`; `None` unless there is
/// exactly one.
fn fill_one_variable(outer: &str, inner: &str) -> Option<String> {
    if outer.matches("{ $").count() != 1 {
        return None;
    }
    let open = outer.find("{ $")?;
    let close = open + outer[open..].find(" }")? + 2;
    Some(format!("{}{}{}", &outer[..open], inner, &outer[close..]))
}

/// The row for a source: its text, then where it is and what it translates.
fn row_for(index: &LocaleIndex, english: &LocaleIndex, source: &RowSource) -> ResultRow {
    match source {
        RowSource::Group(g) => {
            let first = g.hits.first().copied();
            let title = match first {
                Some(h) if h.composed.is_some() => composed_text(
                    index,
                    Member {
                        entry: h.entry,
                        composed: h.composed,
                    },
                ),
                _ => g.shown.clone(),
            };
            let mut parts: Vec<String> = Vec::new();
            match first {
                Some(h) if h.composed.is_some() => {
                    let outer = index.entries[h.entry].shown().to_string();
                    let inner = h
                        .composed
                        .map(|c| index.entries[c].shown().to_string())
                        .unwrap_or_default();
                    parts.push(
                        tr!(translation_report_composed(first = outer, second = inner))
                            .resolve_now(),
                    );
                }
                Some(h) if g.hits.len() == 1 => {
                    parts.push(location_hint(&index.entries[h.entry]));
                }
                _ => parts.push(
                    tr!(translation_report_places(count = g.hits.len() as i64)).resolve_now(),
                ),
            }
            let breakdown = english_breakdown(index, english, g);
            let only_itself = breakdown.len() == 1 && breakdown[0].0 == g.shown;
            if !breakdown.is_empty() && !only_itself {
                let sources = breakdown
                    .iter()
                    .map(|(s, n)| {
                        if *n > 1 {
                            tr!(translation_report_english_item(
                                source = s.clone(),
                                count = *n as i64
                            ))
                            .resolve_now()
                        } else {
                            s.clone()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" · ");
                parts.push(tr!(translation_report_english(sources = sources)).resolve_now());
            } else if only_itself && index.locale != SOURCE_LOCALE {
                parts.push(tr!(translation_report_same_as_english()).resolve_now());
            }
            ResultRow {
                title,
                subtitle: parts.join(" · "),
            }
        }
        RowSource::English(h) => {
            let en = english.entries[h.english].shown().to_string();
            let language = TranslationReportViewModel::language_label(index.locale);
            let subtitle = match h.translation {
                Some(_) if h.untranslated => {
                    tr!(translation_report_english_untranslated(language = language)).resolve_now()
                }
                Some(t) => tr!(translation_report_english_original(
                    language = language,
                    translation = index.entries[t].shown().to_string()
                ))
                .resolve_now(),
                None => tr!(translation_report_english_missing(language = language)).resolve_now(),
            };
            ResultRow {
                title: en,
                subtitle,
            }
        }
    }
}

/// Where a string appears, from its catalogue and its id's prefix. A hint, never a
/// routing decision.
pub(crate) fn location_hint(entry: &CatalogueEntry) -> String {
    let hint = match entry.catalogue {
        Catalogue::Framework | Catalogue::WidgetOverride => tr!(translation_report_where_control()),
        Catalogue::Export => tr!(translation_report_where_book()),
        Catalogue::Help => {
            let page = crate::help::topic(&entry.id)
                .map(|t| (t.title)().resolve_now())
                .unwrap_or_else(|| entry.id.clone());
            tr!(translation_report_where_help(page = page))
        }
        Catalogue::App => {
            let id = entry.id.as_str();
            if entry.area == Area::Help {
                let key = id.strip_suffix("-more").unwrap_or(id);
                let page = crate::help::topic(key)
                    .map(|t| (t.title)().resolve_now())
                    .unwrap_or_else(|| key.to_string());
                tr!(translation_report_where_help(page = page))
            } else if entry.file == "tooltips.ftl" {
                tr!(translation_report_where_tooltip())
            } else if id.starts_with("menu-") {
                tr!(translation_report_where_menu())
            } else if id.starts_with("settings-") {
                tr!(translation_report_where_settings())
            } else if id.starts_with("shortcut-name-") {
                tr!(translation_report_where_command())
            } else if id.starts_with("status-") {
                tr!(translation_report_where_status())
            } else if id.starts_with("tags-") {
                tr!(translation_report_where_tags())
            } else if id.starts_with("note-template") || entry.file == "templates.ftl" {
                tr!(translation_report_where_template())
            } else if id.starts_with("story-bible") || entry.file == "story_bible.ftl" {
                tr!(translation_report_where_story_bible())
            } else if id.starts_with("welcome-") || id.starts_with("learn-") {
                tr!(translation_report_where_launcher())
            } else if id.starts_with("import-") {
                tr!(translation_report_where_import())
            } else if id.starts_with("export-") {
                tr!(translation_report_where_export())
            } else if id.starts_with("new-work-") {
                tr!(translation_report_where_new_work())
            } else if id.starts_with("help-") {
                tr!(translation_report_where_help_window())
            } else {
                tr!(translation_report_where_interface())
            }
        }
    };
    hint.resolve_now()
}

/// What a fix to this string changes, when that is not the obvious.
fn note_for(entry: &CatalogueEntry, composed: bool) -> Option<String> {
    if composed {
        return Some(tr!(translation_report_note_composed()).resolve_now());
    }
    match entry.area {
        Area::DefaultContent => Some(tr!(translation_report_note_seeded()).resolve_now()),
        Area::Export => Some(
            tr!(translation_report_note_export(
                language = TranslationReportViewModel::language_label(entry.locale)
            ))
            .resolve_now(),
        ),
        Area::Interface | Area::Help => None,
    }
}

#[cfg(test)]
mod tests;
