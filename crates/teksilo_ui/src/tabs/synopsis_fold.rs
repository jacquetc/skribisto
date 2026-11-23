// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A tab's two synopsis **folds**, and which of their states the writer chose.
//!
//! A writing page shows its synopsis in one of two places, and each folds its own
//! way: above the text it is a disclosure ([`ContentTab::synopsis_expanded`]), beside
//! the text it is the Side splitter's first pane, collapsed down to its divider
//! ([`ContentTab::side_splitter`]). Both are per-item memory: an item folded is still
//! folded the next time it opens, after a restart too, through the item roster in
//! `workspace.toml` ([`crate::shared::ItemViewStates`]).
//!
//! **Only the writer's own gestures are remembered.** The app moves both folds too:
//! a search hit in the synopsis unfolds it so the match can be shown, a tab opens
//! folded because Settings says so, a remembered fold is put back. None of those is a
//! choice about this item, and recording them would be wrong in two ways. A search
//! would quietly undo a fold the writer made on purpose, and an item merely opened
//! under "start folded" would stop following that setting when it is switched off.
//! So every write the app makes goes through [`SynopsisFoldMemory::quietly`], and the
//! watchers record only the writes that do not.
//!
//! [`ContentTab::synopsis_expanded`]: super::ContentTab::synopsis_expanded
//! [`ContentTab::side_splitter`]: super::ContentTab::side_splitter

use std::cell::Cell;
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::prelude::Signal;
use teksilo::widgets::SplitterModel;

use crate::shared::SynopsisFolds;
use crate::tabs::shared::editor::SYNOPSIS_PANE;

/// Watches one tab's two synopsis folds and keeps what the writer chose for them.
pub(crate) struct SynopsisFoldMemory {
    chosen: Rc<Cell<SynopsisFolds>>,
    /// Set while the app itself moves a fold, so the watchers leave the choice alone.
    quiet: Rc<Cell<bool>>,
    expanded: Signal<bool>,
    splitter: SplitterModel,
    /// Settings ▸ "Start with the synopsis folded".
    default_folded: Signal<bool>,
    /// The Side pane's collapsed state as the watcher last saw it. The splitter's
    /// `version` moves for every change to it (a drag, a show, a hide), so a fold is
    /// told apart by comparing against this rather than by the bump alone.
    side_seen: Rc<Cell<bool>>,
    _watchers: [ObserverHandle; 3],
}

impl SynopsisFoldMemory {
    /// Start watching. `expanded` must already hold the state this tab opens at,
    /// which the caller derives from `default_folded`.
    pub(crate) fn new(
        expanded: Signal<bool>,
        splitter: SplitterModel,
        default_folded: Signal<bool>,
    ) -> Self {
        let chosen = Rc::new(Cell::new(SynopsisFolds::default()));
        let quiet = Rc::new(Cell::new(false));
        let side_seen = Rc::new(Cell::new(splitter.is_collapsed(SYNOPSIS_PANE)));

        // The disclosure above the text. `Signal::set` notifies even when the value
        // does not change, which is harmless here: every write the app makes is
        // quiet, so whatever reaches this unquieted is the writer's click.
        let top = {
            let chosen = chosen.clone();
            let quiet = quiet.clone();
            expanded.observe(move |open| {
                if !quiet.get() {
                    let mut folds = chosen.get();
                    folds.top = Some(!*open);
                    chosen.set(folds);
                }
            })
        };

        // The column beside the text. The model sets the collapsed flag *before* it
        // bumps `version`, so the state read here is the new one. Reading it is all
        // this does: mutating the model from its own version observer re-enters it.
        let side = {
            let chosen = chosen.clone();
            let quiet = quiet.clone();
            let seen = side_seen.clone();
            // A strong capture of the model it observes. Dropping the handle below
            // with the tab unregisters this closure, which is what breaks the
            // model → signal → closure → model cycle.
            let model = splitter.clone();
            splitter.version().observe(move |_| {
                let collapsed = model.is_collapsed(SYNOPSIS_PANE);
                if collapsed == seen.get() {
                    return;
                }
                seen.set(collapsed);
                if !quiet.get() {
                    let mut folds = chosen.get();
                    folds.side = Some(collapsed);
                    chosen.set(folds);
                }
            })
        };

        // Settings ▸ "Start with the synopsis folded", followed live by every tab
        // whose fold the writer has not set, so the switch shows its effect at once.
        let default = {
            let chosen = chosen.clone();
            let quiet = quiet.clone();
            let expanded = expanded.clone();
            default_folded.observe(move |folded| {
                if chosen.get().top.is_none() && expanded.get() == *folded {
                    quiet.set(true);
                    expanded.set(!*folded);
                    quiet.set(false);
                }
            })
        };

        Self {
            chosen,
            quiet,
            expanded,
            splitter,
            default_folded,
            side_seen,
            _watchers: [top, side, default],
        }
    }

    /// What the writer chose for this tab's folds, for the item roster and the
    /// workspace record. `None` on a fold they never touched.
    pub(crate) fn chosen(&self) -> SynopsisFolds {
        self.chosen.get()
    }

    /// Put back folds remembered for this item, before its pane is built.
    ///
    /// A fold with no remembered choice goes back to its default: the setting above
    /// the text, open beside it.
    pub(crate) fn seed(&self, folds: SynopsisFolds) {
        self.chosen.set(folds);
        let top_folded = folds.top.unwrap_or_else(|| self.default_folded.get());
        let side_folded = folds.side.unwrap_or(false);
        self.quietly(|| {
            self.expanded.set(!top_folded);
            // Instantly: the pane has not been built, and a remembered fold animating
            // shut on opening would show the writer a column they did not ask for.
            self.splitter
                .set_collapsed_immediate(SYNOPSIS_PANE, side_folded);
        });
        self.side_seen.set(side_folded);
    }

    /// Unfold the synopsis wherever it is, for a search hit that lands in it, without
    /// recording that as the writer's choice.
    pub(crate) fn reveal(&self) {
        self.quietly(|| {
            self.expanded.set(true);
            self.splitter.set_collapsed(SYNOPSIS_PANE, false);
        });
    }

    /// Run `f`, which moves a fold, as the app rather than the writer.
    fn quietly(&self, f: impl FnOnce()) {
        let was = self.quiet.replace(true);
        f();
        self.quiet.set(was);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use teksilo::widgets::{Orientation, PaneDescriptor};

    fn splitter() -> SplitterModel {
        SplitterModel::from_panes(
            vec![
                PaneDescriptor::new().size(280.0).collapsible(true),
                PaneDescriptor::new().stretch(1.0),
            ],
            Orientation::Horizontal,
        )
    }

    fn memory(default_folded: bool) -> (SynopsisFoldMemory, Signal<bool>, SplitterModel) {
        let expanded = Signal::new(!default_folded);
        let model = splitter();
        let default = Signal::new(default_folded);
        let memory = SynopsisFoldMemory::new(expanded.clone(), model.clone(), default);
        (memory, expanded, model)
    }

    #[test]
    fn nothing_is_chosen_until_the_writer_folds_something() {
        let (memory, _, _) = memory(false);
        assert_eq!(memory.chosen(), SynopsisFolds::default());
    }

    #[test]
    fn folding_either_synopsis_is_recorded_as_the_writers_choice() {
        let (memory, expanded, model) = memory(false);
        expanded.set(false);
        model.set_collapsed(SYNOPSIS_PANE, true);
        assert_eq!(
            memory.chosen(),
            SynopsisFolds {
                top: Some(true),
                side: Some(true),
            }
        );
        // And unfolding is a choice too: it is what keeps an item open under a
        // "start folded" setting.
        expanded.set(true);
        model.set_collapsed(SYNOPSIS_PANE, false);
        assert_eq!(
            memory.chosen(),
            SynopsisFolds {
                top: Some(false),
                side: Some(false),
            }
        );
    }

    /// The regression this type exists for: a search hit unfolding the synopsis must
    /// not overwrite a fold the writer made on purpose.
    #[test]
    fn a_search_reveal_unfolds_both_without_recording_it() {
        let (memory, expanded, model) = memory(false);
        expanded.set(false);
        model.set_collapsed(SYNOPSIS_PANE, true);
        memory.reveal();
        assert!(expanded.get(), "the match has to be visible");
        assert!(!model.is_collapsed(SYNOPSIS_PANE));
        assert_eq!(
            memory.chosen(),
            SynopsisFolds {
                top: Some(true),
                side: Some(true),
            },
            "still folded as far as the item's memory goes"
        );
    }

    #[test]
    fn a_seed_puts_both_folds_back_and_is_not_a_new_choice() {
        let (memory, expanded, model) = memory(false);
        let remembered = SynopsisFolds {
            top: Some(true),
            side: Some(true),
        };
        memory.seed(remembered);
        assert!(!expanded.get());
        assert!(model.is_collapsed(SYNOPSIS_PANE));
        assert_eq!(memory.chosen(), remembered);
    }

    #[test]
    fn an_empty_seed_falls_back_to_the_setting_above_and_open_beside() {
        let (memory, expanded, model) = memory(true);
        memory.seed(SynopsisFolds::default());
        assert!(!expanded.get(), "the setting folds it");
        assert!(
            !model.is_collapsed(SYNOPSIS_PANE),
            "no setting folds the column"
        );
        assert_eq!(memory.chosen(), SynopsisFolds::default());
    }

    #[test]
    fn the_setting_moves_an_untouched_fold_live_and_leaves_a_chosen_one() {
        let expanded = Signal::new(true);
        let default = Signal::new(false);
        let memory = SynopsisFoldMemory::new(expanded.clone(), splitter(), default.clone());

        default.set(true);
        assert!(!expanded.get(), "an untouched fold follows the setting");
        assert_eq!(
            memory.chosen().top,
            None,
            "and following it is not a choice"
        );

        expanded.set(true);
        default.set(false);
        default.set(true);
        assert!(expanded.get(), "an item the writer unfolded stays unfolded");
    }
}
