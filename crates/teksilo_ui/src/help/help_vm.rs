// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `HelpViewModel` — which topic the Help window is showing, and how the reader got
//! there.
//!
//! **Tier 1: one per process, not one per window.** There is a single Help window
//! (`window::HELP_WINDOW_ID`), and its content has nothing to do with any particular
//! `Work` — a topic explains the application, not the manuscript. So this is registered
//! once in `app_state` and reached by clone, and it deliberately holds no `AppIds` and
//! no session.
//!
//! It carries no widgets and no `AppContext`, which is what lets the whole of it be
//! tested headlessly: everything below is plain Rust over three signals.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::*;

use super::{DEFAULT_TOPIC, HelpSection, HelpTopicSpec, all_topics, topic};

/// The Help window's state: what is open, what was typed in the filter, and the trail
/// back.
#[derive(Clone)]
pub struct HelpViewModel {
    /// The key of the topic on screen.
    current: Signal<String>,
    /// The table-of-contents filter.
    query: Signal<String>,
    /// Keys visited before the current one, oldest first.
    ///
    /// A plain stack rather than a browser-style forward/back pair: a help window is
    /// read by following links inward and stepping back out, and a Forward button that
    /// is empty nine times out of ten is a control that teaches nothing.
    history: Rc<RefCell<Vec<String>>>,
    /// `history.len()`, mirrored into a signal so the Back button's enabled state can
    /// bind to it. Kept in step by [`Self::push_history`] and [`Self::back`], the only
    /// two places the stack is touched.
    depth: Signal<usize>,
}

impl Default for HelpViewModel {
    fn default() -> Self {
        Self::new()
    }
}

impl HelpViewModel {
    /// A view-model opened on [`DEFAULT_TOPIC`].
    pub fn new() -> Self {
        Self {
            current: Signal::new(DEFAULT_TOPIC.to_string()),
            query: Signal::new(String::new()),
            history: Rc::new(RefCell::new(Vec::new())),
            depth: Signal::new(0),
        }
    }

    /// The key of the topic on screen.
    pub fn current_key(&self) -> Signal<String> {
        self.current.clone()
    }

    /// The table-of-contents filter text.
    pub fn query(&self) -> Signal<String> {
        self.query.clone()
    }

    /// How many topics are on the back stack. Bind a Back control's `enabled` to this.
    pub fn depth(&self) -> Signal<usize> {
        self.depth.clone()
    }

    /// The spec for the topic on screen, or `None` if the key names nothing.
    ///
    /// `None` is reachable: an extension whose topics were registered when the window
    /// opened can be dropped while it is still open, and the window must render an
    /// honest "not found" rather than the last topic it happened to have.
    pub fn current_topic(&self) -> Option<HelpTopicSpec> {
        topic(&self.current.get())
    }

    /// Show `key`, remembering where the reader was.
    ///
    /// Re-opening the topic already on screen is a no-op rather than a history entry,
    /// so clicking a link to the page you are reading does not make Back a no-op too.
    pub fn open(&self, key: &str) {
        if self.current.get() == key {
            return;
        }
        self.push_history(self.current.get());
        self.current.set(key.to_string());
    }

    /// Show `key` without a history entry: the entry points (F1, the menu, the Learn
    /// pane) start a reading session rather than continue one.
    pub fn open_fresh(&self, key: &str) {
        self.history.borrow_mut().clear();
        self.depth.set(0);
        self.current.set(key.to_string());
    }

    /// Go back one topic. No-op at the start of the trail.
    pub fn back(&self) {
        let previous = self.history.borrow_mut().pop();
        if let Some(previous) = previous {
            self.depth.set(self.history.borrow().len());
            self.current.set(previous);
        }
    }

    fn push_history(&self, key: String) {
        self.history.borrow_mut().push(key);
        self.depth.set(self.history.borrow().len());
    }

    /// Follow a link from a topic body.
    ///
    /// The `:key` form is the same convention the rich-tooltip cascade already uses, so
    /// a concept's explainer reads identically whether it is a tooltip or a page, and
    /// the two cannot drift into different link syntaxes. Anything else is an external
    /// address and goes to the desktop's browser through the app's one allowlisted
    /// opener.
    pub fn follow_link(&self, href: &str, ctx: &mut EventContext) {
        match href.strip_prefix(':') {
            Some(key) => {
                if topic(key).is_some() {
                    self.open(key);
                } else {
                    // A link to a key nothing registers. The drift test makes this
                    // unreachable for built-in topics; it stays possible for a
                    // contributed one, and doing nothing is better than navigating to
                    // a blank page.
                    eprintln!("skribisto: help link to unknown topic key '{key}'");
                }
            }
            None => crate::shared::external_link::open_external_link(href, ctx),
        }
    }

    /// The table of contents: topics grouped by section, in section order, filtered by
    /// [`Self::query`].
    ///
    /// A section with no surviving topic is dropped rather than shown empty, so typing
    /// narrows the list instead of leaving a scaffold of headings behind.
    pub fn contents(&self) -> Vec<(HelpSection, Vec<HelpTopicSpec>)> {
        let needle = self.query.get().trim().to_lowercase();
        let topics = all_topics();
        HelpSection::ALL
            .iter()
            .filter_map(|section| {
                let matching: Vec<HelpTopicSpec> = topics
                    .iter()
                    .filter(|t| t.section == *section)
                    .filter(|t| {
                        needle.is_empty()
                            || (t.title)().resolve_now().to_lowercase().contains(&needle)
                    })
                    .cloned()
                    .collect();
                (!matching.is_empty()).then_some((*section, matching))
            })
            .collect()
    }
}
