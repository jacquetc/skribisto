// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Where the book stands — shown once when a project with an active writing plan opens,
//! and available from Work ▸ Writing plan… whenever it is asked for.
//!
//! A writer who has set a deadline has asked the app to hold them to it. Making them go
//! looking for the answer — open the Book, find the Pace segment — is the app declining the
//! job it was given. So it says so on the way in, once, and gets out of the way: Escape or a
//! click outside closes it, and a checkbox stops it coming back.
//!
//! That checkbox is why the menu row exists. Turning the greeting off used to make the
//! summary unreachable: there was no other door to it, so "not every morning" and "never
//! again" were the same answer. [`present`] is now called from both, unchanged — the same
//! card, the same figures, the same forward action — and the checkbox governs only whether
//! it opens by itself.
//!
//! **Fired after a fresh count, not before.** The numbers Pace draws come from the
//! `ProgressSnapshot` history, which is written on save — so at opening the newest one can
//! be days old, and a summary quoting it would be quietly wrong on exactly the morning it
//! matters. The wiring runs `count_words` first and presents this on its completion, which
//! also closes a real gap: a project opened and never saved used to leave a hole in the
//! streak and the chart. The menu row needs none of that: it fires while the app is running,
//! so the count it reads is the count the status bar is already showing.
//!
//! ## Chrome
//!
//! `ctx.present_modal` does **not** wrap a hand-drawn panel in a container, and its
//! `.title(..)` is only honoured by the native-window backend — so an in-tree modal that
//! draws neither has no surface and no name: this card used to render its text straight onto
//! the dimmed project behind it, unreadable. It now carries the same shell every other modal
//! in this app does (`Panel`, raised, 10 dp corners; a header strip with the title and a
//! close ✕; a footer bar) — see `backup::list_panel`, which is the fullest example of it.
//!
//! Inside that shell the blocks are [`crate::pace::panel_section`] cards: literally the same
//! call the Book's Pace segment draws its Schedule / Progress / Advancement sections with,
//! so the card that greets the writer and the planner it sends them to are one design.

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel,
    ScrollArea, Spacer, TextWidget, Toggle, VStack,
};

use frontend::AppContext;
use frontend::commands::{binder_item_commands, pace_commands, work_commands};
use frontend::common::direct_access::work::WorkRelationshipField;

use crate::app_ids::AppIds;
use crate::goals::format_goal;
use crate::pace::CARD_PADDING;

/// "Open this Book's plan" — supplied by the caller, which is the only place that knows how
/// this window opens a tab.
type OpenPace = Rc<dyn Fn(u64, &mut EventContext)>;

const CARD_W: f32 = 560.0;
const CARD_H: f32 = 420.0;
/// The title strip, and the bar the Close button sits on. Same measures as the backups
/// browser's, so two modals opened a minute apart do not have different chrome.
const HEADER_H: f32 = 44.0;
const FOOTER_H: f32 = 52.0;

/// One active plan's headline facts, resolved once when the panel is built.
struct PlanRow {
    book_item_id: u64,
    title: String,
    goal: i64,
    written: i64,
}

/// Every Book in this project carrying an **active** plan — `(item id, title, goal)`, and
/// no measurement.
///
/// Split from [`active_plans`] because measuring is by far the expensive half: it walks a
/// Book's whole subtree counting words, and [`has_active_plan`] is re-asked on every
/// `BinderItem` event to keep the menu row in step. Asking "is there a plan" must not cost
/// a word count of the manuscript.
fn plan_books(app_ctx: &AppContext, ids: &AppIds) -> Vec<(u64, String, i64)> {
    let Some(work_id) = ids.work_id.get() else {
        return Vec::new();
    };
    let pace_ids =
        work_commands::get_work_relationship(app_ctx, &work_id, &WorkRelationshipField::Paces)
            .unwrap_or_default();
    let mut out = Vec::new();
    for pace in pace_commands::get_pace_multi(app_ctx, &pace_ids)
        .unwrap_or_default()
        .into_iter()
        .flatten()
    {
        if !pace.active {
            continue;
        }
        let Some(book_item_id) = pace.book_item else {
            continue;
        };
        let Ok(Some(book)) = binder_item_commands::get_binder_item(app_ctx, &book_item_id) else {
            continue;
        };
        // The Book's target is its own `BinderItem` field — the same number the Pace tab's
        // goal box edits and the Inspector shows, never a copy.
        let goal = book.word_count_goal;
        if goal <= 0 {
            continue;
        }
        out.push((book_item_id, book.title, goal));
    }
    out
}

/// Every Book in this project with an **active** plan, measured.
///
/// Empty means there is nothing to say, and the caller shows nothing at all rather than a
/// modal explaining that it has no news.
fn active_plans(app_ctx: &AppContext, ids: &AppIds) -> Vec<PlanRow> {
    let Some(work_id) = ids.work_id.get() else {
        return Vec::new();
    };
    plan_books(app_ctx, ids)
        .into_iter()
        .map(|(book_item_id, title, goal)| PlanRow {
            book_item_id,
            title,
            goal,
            // Live, from the one measurement service, so this panel cannot disagree with
            // the Book's own page about how far along it is.
            written: crate::goals::measure::measure(
                app_ctx,
                work_id,
                book_item_id,
                skribisto_model::counting::CountingMethodSetting::default(),
                &frontend::common::entities::GoalUnit::Words,
            )
            .map_or(0, |m| m.subtree.words as i64),
        })
        .collect()
}

/// Whether this project has anything worth opening the panel for.
///
/// Two callers, and they must not drift: the load wiring asks before greeting the writer,
/// and the Work menu asks to decide whether its row is enabled. A row that opened an empty
/// card would be worse than a greyed one, which at least says the feature exists and what
/// turns it on.
pub fn has_active_plan(app_ctx: &AppContext, ids: &AppIds) -> bool {
    !plan_books(app_ctx, ids).is_empty()
}

/// Present the summary. The caller has already decided it should appear.
pub fn present(
    ctx: &mut EventContext,
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    show_on_open: Signal<bool>,
    statuses: crate::statuses::StatusesViewModel,
    open_pace: OpenPace,
) {
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(PaceSummaryPanel {
                rows: active_plans(&app_ctx, &ids),
                // Measured here, with the plans, so both halves of the card describe the
                // same moment — and measured *once*, not per build.
                completion: crate::statuses::completion::measure(&app_ctx, &ids, &statuses),
                show_on_open: show_on_open.clone(),
                open_pace: open_pace.clone(),
                root_child: None,
                close_button: None,
            })
        })
        .presentation(ModalPresentation::InTree)
        .title(tr!(pace_summary_title()))
        .size(CARD_W as u32, CARD_H as u32)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct PaceSummaryPanel {
    rows: Vec<PlanRow>,
    /// Where the manuscript stands, by rung. The *content* is
    /// [`crate::statuses::completion`]'s and is shared verbatim with the panel the menu
    /// bar opens — two renderings of one number that could drift apart is exactly what
    /// that split prevents.
    completion: crate::statuses::completion::Completion,
    show_on_open: Signal<bool>,
    /// Open the Book's Pace planner — the panel's one forward action.
    open_pace: OpenPace,
    root_child: Option<WidgetId>,
    /// The footer's Close, captured for [`Widget::initial_focus_hint`].
    close_button: Option<WidgetId>,
}

impl std::fmt::Debug for PaceSummaryPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PaceSummaryPanel").finish()
    }
}

impl Widget for PaceSummaryPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let mut list = VStack::new().spacing(12.0);
        // The completion readout leads: it answers "where is the book" for every project,
        // whereas the plans below answer "will I hit the date" only where one is set.
        // Skipped entirely on a manuscript with no prose rows — the readout's own empty
        // line has nothing to add to a card that is already about a deadline.
        //
        // Untitled: the readout's first line is its own heading ("7 of 22 scenes
        // finished"), so a card heading above it would say the same thing twice.
        if !self.completion.is_empty() {
            list = list.child(crate::pace::panel_card(crate::tabs::Boxed::new(
                crate::statuses::completion::readout(&self.completion),
            )));
        }
        for row in &self.rows {
            let open = self.open_pace.clone();
            let book = row.book_item_id;
            // Titled with the book's own name — a project can carry several Books with
            // several plans, and "88,415 of 120,000" means nothing without saying of what.
            list = list.child(crate::pace::panel_section(
                lit!(row.title.clone()),
                VStack::new()
                    .spacing(8.0)
                    .child(crate::goals::readout::line(
                        row.written.max(0) as usize,
                        row.goal,
                        &frontend::common::entities::GoalUnit::Words,
                    ))
                    .child(
                        TextWidget::new(tr!(pace_summary_remaining(
                            words = format_goal((row.goal - row.written).max(0))
                        )))
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .single_line(),
                    )
                    .child(
                        HStack::new().child(
                            Button::new(tr!(pace_summary_open()))
                                .variant(ButtonVariant::Plain)
                                .rich_tooltip(crate::tooltip_registry::PACE_PLAN)
                                .on_activate_fn(move |c| {
                                    open(book, c);
                                    c.dismiss_modal();
                                }),
                        ),
                    ),
            ));
        }

        // The setting says "show it"; the checkbox asks not to. A local signal carries the
        // inverted sense, with one effect writing through — a derived signal is read-only,
        // and a `Toggle` needs somewhere to write.
        let hide = Signal::new(!self.show_on_open.get());
        {
            let show = self.show_on_open.clone();
            ctx.effect(&hide, move |off| {
                let want = !*off;
                if show.get() != want {
                    show.set(want);
                }
            });
        }

        let body =
            ctx.add(ScrollArea::new().child(Padding::symmetric(CARD_PADDING, 16.0).child(list)));

        // Built by hand rather than inside the `teksu!` shell so its id can be captured
        // for `initial_focus_hint` — see that method for why the ✕ must not have it.
        let close_button = ctx.add(
            Button::new(tr!(pace_summary_close()))
                .variant(ButtonVariant::Filled)
                .on_activate_fn(|c| c.dismiss_modal()),
        );
        self.close_button = Some(close_button);
        let footer = ctx.add(
            Padding::symmetric(10.0, 16.0).child(
                HStack::new()
                    .spacing(8.0)
                    .child(Toggle::new(hide).label(tr!(pace_summary_dont_show())))
                    .child(Spacer::new())
                    .add_child(close_button),
            ),
        );
        // `Expand::horizontal` around each bar, not a bare `FixedSize`: a height-only
        // `FixedSize` reports its child's *intrinsic* width and is then placed at it, which
        // starves the header title and leaves the footer's `Spacer` nothing to push
        // against — the trap `backup::list_panel` documents at length.
        let root = teksu!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    Expand::horizontal {
                        FixedSize {
                            height: HEADER_H
                            Padding::symmetric(8.0, 16.0) {
                                HStack {
                                    spacing: 8.0
                                    Expand::horizontal {
                                        TextWidget::new(tr!(pace_summary_title())) {
                                            style: TextStyleRole::Small
                                            color: TextRole::Secondary
                                            single_line
                                        }
                                    }
                                    IconButton::clear() {
                                        tooltip: tr!(pace_summary_close())
                                        on_activate_fn: |c| c.dismiss_modal()
                                    }
                                }
                            }
                        }
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::vertical {
                        child_id: body
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::horizontal {
                        FixedSize {
                            height: FOOTER_H
                            child_id: footer
                        }
                    }
                }
            }
        });
        self.root_child = Some(root);
        vec![root]
    }

    /// Open with the footer's **Close** focused, not the header's ✕.
    ///
    /// The modal pipeline's own fallback is `first_focusable_descendant`, and in tree
    /// order that is the ✕ in the title strip — so the card opened with a 2 dp focus ring
    /// drawn around its dismiss glyph, pointing the eye at the way out of a card the
    /// writer had only just been shown. Both buttons do the same thing; this is the one
    /// that reads as the dialog's default, and it sits where the eye finishes reading.
    ///
    /// Deliberately not "Open the plan": that button opens a tab, which is a larger step
    /// than Enter should take on a card that appeared by itself. Escape and this Close
    /// then agree.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.close_button
    }

    /// Announce the card as a named dialog.
    ///
    /// `ctx.present_modal` does not wrap a hand-drawn panel in a `ModalContainer`, so
    /// nothing else here would emit a `Role::Dialog` node — the same gap the New Work and
    /// Import documents wizards close this way.
    fn accessibility(&self, builder: &mut teksilo::core::accessibility::AccessNodeBuilder) {
        builder.set_role(teksilo::core::accesskit::Role::Dialog);
        builder.set_name(tr!(pace_summary_title()).resolve_now());
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::statuses::completion::{Completion, Tally};
    use teksilo::core::widget_tree::WidgetTree;

    fn panel(rows: Vec<PlanRow>, completion: Completion) -> (WidgetTree, WidgetId) {
        let mut tree = WidgetTree::new()
            .with_theme(teksilo::presets::intui::light())
            .with_text_backend(Rc::new(std::cell::RefCell::new(
                teksilo::canvas::MockTextBackend::new(),
            )));
        let id = tree.add(PaceSummaryPanel {
            rows,
            completion,
            show_on_open: Signal::new(true),
            open_pace: Rc::new(|_, _| {}),
            root_child: None,
            close_button: None,
        });
        // `unspecified`, so the card reports what it wants rather than being placed at
        // the proposal — which is also the assertion that it does not swell to fill the
        // window (the modal-centering trap `new_work::panel` documents).
        tree.layout(SizeProposal::unspecified());
        (tree, id)
    }

    fn one_plan() -> Vec<PlanRow> {
        vec![PlanRow {
            book_item_id: 1,
            title: "Faux-semblants".into(),
            goal: 120_000,
            written: 88_415,
        }]
    }

    fn descendants(tree: &WidgetTree, id: WidgetId, out: &mut Vec<WidgetId>) {
        for child in tree.children(id) {
            out.push(child);
            descendants(tree, child, out);
        }
    }

    /// **The bug this card shipped with.** `present_modal` does not wrap a hand-drawn
    /// panel in a container, and `ModalRequest::title` is only honoured by the
    /// native-window backend — so a card that draws neither has no surface at all: the
    /// text rendered straight onto the dimmed project behind it, one line of the summary
    /// crossing another line of the pane underneath.
    ///
    /// The guard is not "a `Panel` exists somewhere" but "a `Panel` covers the whole
    /// card": a background that stopped short of the edges would still leave text on the
    /// scrim, which is exactly the complaint.
    #[test]
    fn the_card_is_backed_by_a_panel_covering_all_of_it() {
        let (tree, id) = panel(
            one_plan(),
            Completion {
                tallies: vec![],
                total: 0,
            },
        );
        let card = tree.bounds(id).size();
        assert_eq!((card.width, card.height), (CARD_W, CARD_H));

        let mut kids = Vec::new();
        descendants(&tree, id, &mut kids);
        let covered = kids.iter().any(|&k| {
            tree.widget_type_name(k)
                .is_some_and(|n| n.ends_with("::Panel"))
                && tree.bounds(k).size().width >= CARD_W
                && tree.bounds(k).size().height >= CARD_H
        });
        assert!(
            covered,
            "the summary must sit on a surface of its own, not on the dimmed project"
        );
    }

    /// The header and footer bars must span the card, not be placed at their content
    /// width — the `FixedSize`-reports-its-child's-intrinsic-width trap
    /// `backup::list_panel` documents. A starved header leaves the title jammed against
    /// the ✕ and the footer's `Spacer` with nothing to push Close against.
    #[test]
    fn the_header_and_footer_bars_span_the_card() {
        let (tree, id) = panel(
            one_plan(),
            Completion {
                tallies: vec![],
                total: 0,
            },
        );
        let mut kids = Vec::new();
        descendants(&tree, id, &mut kids);
        for (name, h) in [("header", HEADER_H), ("footer", FOOTER_H)] {
            let spans = kids.iter().any(|&k| {
                let s = tree.bounds(k).size();
                (s.height - h).abs() < 0.5 && s.width >= CARD_W - 0.5
            });
            assert!(spans, "the {h}px {name} bar must span the full card width");
        }
    }

    /// Focus opens on the footer's Close, not the header's ✕ (the modal pipeline's own
    /// `first_focusable_descendant` fallback). See [`Widget::initial_focus_hint`]'s doc.
    #[test]
    fn focus_opens_on_the_footer_close_not_the_header_glyph() {
        let (tree, id) = panel(
            one_plan(),
            Completion {
                tallies: vec![],
                total: 0,
            },
        );
        let hinted = tree
            .widget_initial_focus_hint(id)
            .expect("the card pins its own initial focus");
        let first = tree
            .first_focusable_descendant(id)
            .expect("the card has focus stops");
        assert_ne!(
            hinted, first,
            "the hint must not be the tree-order first focusable — that is the ✕"
        );
        assert!(
            tree.bounds(hinted).origin().y > tree.bounds(first).origin().y,
            "…it is the footer's Close, which sits below the header"
        );
    }

    /// A manuscript with no prose rows gets no completion card at all — the readout's
    /// own empty line has nothing to add to a card that is already about a deadline —
    /// while one with rows gets both blocks.
    #[test]
    fn the_completion_block_appears_only_when_there_is_a_manuscript_to_report_on() {
        fn panels(rows: Vec<PlanRow>, c: Completion) -> usize {
            let (tree, id) = panel(rows, c);
            let mut kids = Vec::new();
            descendants(&tree, id, &mut kids);
            kids.iter()
                .filter(|&&k| {
                    tree.widget_type_name(k)
                        .is_some_and(|n| n.ends_with("::Panel"))
                })
                .count()
        }
        let empty = panels(
            one_plan(),
            Completion {
                tallies: vec![],
                total: 0,
            },
        );
        let full = panels(
            one_plan(),
            Completion {
                tallies: vec![Tally {
                    rung: None,
                    count: 3,
                }],
                total: 3,
            },
        );
        assert_eq!(
            full,
            empty + 1,
            "a measured manuscript adds exactly one card, the completion readout"
        );
    }
}
