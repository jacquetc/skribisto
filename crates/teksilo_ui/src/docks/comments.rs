// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two comment docks.
//!
//! * **Leading rail, 4th tab** — project-wide: every thread in the Work, grouped
//!   by the item it lives in, for the "walk my notes" review pass.
//! * **Trailing rail, 3rd tab** — this document: the focused item's threads only.
//!   It joins Inspector and Format on the rail that is already about *whatever is
//!   in front of me right now*, and reuses the very signal Inspector uses
//!   (`EditorsViewModel::active_item()`), so it rebuilds exactly when Inspector
//!   does with no new plumbing for "what is focused".
//!
//! Both are `ListView`s over the **same**
//! [`CommentsListModel`](crate::models::CommentsListModel): a comment card is
//! a variable-height composite (breadcrumb, quoted snippet, body, footer chips),
//! which is the free-form row shape `ListView`'s delegate signature exists for.
//! `TableView` would force column-shaped cells a wrapping body cannot live in, and
//! `TreeView` would imply a hierarchy comments do not have among themselves —
//! replies are a flat ordered thread, mirroring OOXML, whose `w15:paraIdParent`
//! always points at the thread root rather than at a peer reply.
//!
//! Neither dock imports `EditorsViewModel`. Navigation goes through the shared
//! [`OpenItemFn`], exactly as the outline and trash docks do.

use teksilo::prelude::*;
use teksilo::widgets::{
    ActivateOn, Button, ButtonVariant, Divider, DockOpenLocation, DockSide, DockWidget,
    DockWidgetId, Expand, FocusScope, HStack, IconButton, IconWidget, ListView, MaxSize, MenuItem,
    MenuList, Padding, ScrollBarMode, StandardListItem, Switcher, TextWidget, TraversalScopePolicy,
    VStack, Wrap,
};

/// How much of a row the project dock's breadcrumb may take before it elides.
///
/// A cap rather than a share: the item title is user data of any length, and the
/// snippet beside it is the thing the row exists to show.
const BREADCRUMB_WIDTH: f32 = 88.0;

use crate::docks::outline::OpenItemFn;
use crate::models::CommentRow;
use crate::view_models::{CommentFilter, CommentSort, CommentsViewModel};

/// Which dock this is. The only differences are scope, the breadcrumb line, and
/// which rail it defaults to — everything else is shared, so the two can never
/// drift apart in how they render a thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommentScope {
    /// Every thread in the Work.
    Project,
    /// Only the focused item's threads.
    Document,
}

/// The project-wide comments dock (leading rail).
pub fn comments_project_dock(
    vm: CommentsViewModel,
    dock_id: DockWidgetId,
    on_open: OpenItemFn,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(comments_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(comments_panel(
            vm.clone(),
            CommentScope::Project,
            Signal::new(None),
            on_open.clone(),
        ))
    })
    .icon(crate::icons::activity::comments_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Leading))
}

/// The current-document comments dock (trailing rail).
///
/// `focus` is the active editor tab's item id — the same signal the Inspector
/// dock takes, so the two stay in step by construction.
pub fn comments_document_dock(
    vm: CommentsViewModel,
    dock_id: DockWidgetId,
    focus: Signal<Option<u64>>,
    on_open: OpenItemFn,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(comments_document_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(comments_panel(
            vm.clone(),
            CommentScope::Document,
            focus.clone(),
            on_open.clone(),
        ))
    })
    .icon(crate::icons::activity::comments_icon)
    .show_header(true)
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

/// Filter chips over the thread list, with an empty state beneath.
fn comments_panel(
    vm: CommentsViewModel,
    scope: CommentScope,
    focus: Signal<Option<u64>>,
    on_open: OpenItemFn,
) -> impl Widget {
    let header = filter_bar(vm.clone());
    let body = thread_list(vm.clone(), scope, focus.clone(), on_open);

    // Recomputed whenever the model or the filter changes, so the empty state and
    // the list can never both be wrong at once.
    let version = vm.model().version_signal();
    let filter = vm.filter_signal();
    let has_rows = {
        let vm = vm.clone();
        let focus = focus.clone();
        version.zip(&filter).zip(&focus).map(move |_| {
            let scope_item = match scope {
                CommentScope::Project => None,
                CommentScope::Document => focus.get(),
            };
            !vm.visible_rows(scope_item).is_empty()
        })
    };

    let empty_text = match scope {
        CommentScope::Project => tr!(comments_empty_project()),
        CommentScope::Document => tr!(comments_empty_document()),
    };

    let switcher = Switcher::new(has_rows.map(|b| usize::from(*b)))
        .child(
            Padding::symmetric(16.0, 24.0).child(
                TextWidget::new(empty_text)
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
        .child(body);

    VStack::new()
        .spacing(0.0)
        .child(header)
        .child(Divider::new())
        .child(Expand::new().child(switcher))
}

/// All / Open / Resolved / Orphaned.
///
/// **The orphan chip is always visible, even at zero.** A filter that only
/// appeared once something was wrong would let a lone orphan sit unnoticed — the
/// exact shape of Google Docs' and Confluence's long-standing complaints.
fn filter_bar(vm: CommentsViewModel) -> impl Widget {
    let current = vm.filter_signal();
    let version = vm.model().version_signal();

    let chip = |label: LocalizedString, which: CommentFilter, vm: CommentsViewModel| {
        let current = vm.filter_signal();
        Button::new(label)
            .variant(ButtonVariant::Plain)
            .enabled(current.map(move |c| *c != which))
            .on_activate_fn(move |_ctx| vm.set_filter(which))
    };

    // The label stays a real translated string; only the number is reactive, so
    // the chip is always present (including at zero) without smuggling a count
    // into the ftl value.
    let orphan_count = {
        let vm = vm.clone();
        version.map(move |_| vm.orphan_count().to_string())
    };
    let orphan_btn = {
        let vm = vm.clone();
        HStack::new()
            .spacing(2.0)
            .child(
                Button::new(tr!(comments_filter_orphaned()))
                    .variant(ButtonVariant::Plain)
                    .enabled(current.map(|c| *c != CommentFilter::Orphaned))
                    .on_activate_fn(move |_ctx| vm.set_filter(CommentFilter::Orphaned)),
            )
            .child(
                TextWidget::new(lit!(""))
                    .text(orphan_count)
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
    };

    // A `Wrap`, not an `HStack` — and that is the whole fix for a dock this narrow.
    //
    // `Button` is **rigid** by framework contract: `Button::layout_response` drops
    // its shrink weight and is unit-tested to keep its natural width in an
    // over-constrained row rather than truncate its label. So four chips plus a sort
    // control in an `HStack` do not compress, do not elide and do not wrap — they
    // simply run off the end of a 260 dp dock, taking the last chip and the sort
    // with them. Wrapping is what the search dock does with its own option row, for
    // the same reason ("this narrow dock always overflows six chips").
    //
    // The chips keep their words rather than becoming icons: they are a
    // single-choice facet whose current value must be readable without hovering,
    // and four invented glyphs would be four things to learn where four short words
    // already work. Only the sort — two states, and by far the widest label —
    // becomes an icon.
    Padding::symmetric(8.0, 6.0).child(
        Wrap::new()
            .spacing(4.0)
            .line_spacing(4.0)
            .child(chip(
                tr!(comments_filter_all()),
                CommentFilter::All,
                vm.clone(),
            ))
            .child(chip(
                tr!(comments_filter_open()),
                CommentFilter::Open,
                vm.clone(),
            ))
            .child(chip(
                tr!(comments_filter_resolved()),
                CommentFilter::Resolved,
                vm.clone(),
            ))
            .child(orphan_btn)
            .child(sort_toggle(vm.clone())),
    )
}

/// Document order ↔ newest first.
///
/// Document order is the default because that is the order a writer scrolls past
/// their own notes; "newest first" is the separate review-pass question ("what did
/// I just leave myself"), so it is an explicit toggle rather than a second sort
/// nobody would discover.
fn sort_toggle(vm: CommentsViewModel) -> impl Widget {
    // An icon, not a label. "In document order" / "Dans l'ordre du document" was the
    // widest thing in the dock by a wide margin, sitting where it had the least room
    // to fail gracefully — and a sort has two states, which is exactly what an icon
    // toggle is for. The glyph shown is the *current* order and the tooltip names
    // it, the same contract as the title bar's spell-check toggle.
    //
    // Still a `Switcher` over two statically-labelled buttons rather than one button
    // with a reactive tooltip: a translated string pushed through a plain `String`
    // setter would lose its `LocalizedString` identity (which is exactly why that
    // type is not `Display`).
    let button = |icon: IconWidget, tip: LocalizedString, next, vm: CommentsViewModel| {
        IconButton::new(icon)
            .toolbar()
            .tooltip(tip)
            .on_activate_fn(move |_ctx| vm.set_sort(next))
    };
    Switcher::new(vm.sort_signal().map(|s| match s {
        CommentSort::DocumentOrder => 0usize,
        CommentSort::NewestFirst => 1usize,
    }))
    .child(button(
        crate::icons::comments::sort_document_icon(),
        tr!(comments_sort_document()),
        CommentSort::NewestFirst,
        vm.clone(),
    ))
    .child(button(
        crate::icons::comments::sort_newest_icon(),
        tr!(comments_sort_newest()),
        CommentSort::DocumentOrder,
        vm.clone(),
    ))
}

/// The thread list itself.
fn thread_list(
    vm: CommentsViewModel,
    scope: CommentScope,
    focus: Signal<Option<u64>>,
    on_open: OpenItemFn,
) -> impl Widget {
    let model = vm.model().list_model();
    let row_vm = vm.clone();
    let row_focus = focus.clone();

    ListView::new(model.clone(), move |_i, row: &CommentRow, _sel| {
        Box::new(comment_card(row, scope, row_vm.clone(), row_focus.clone())) as Box<dyn Widget>
    })
    .auto_item_height(64.0)
    .scroll_bar_style(ScrollBarMode::Overlay)
    // Single click opens: a comment is a navigation target, not a selection.
    .activate_on(ActivateOn::SingleClick)
    .on_activate({
        let model = model.clone();
        let vm = vm.clone();
        move |idx, _ctx| {
            if let Some(row) = model.with_item(idx, |r| r.clone())
                && let Some(item_id) = row.item_id
            {
                // Park the seek *before* opening: a fresh open builds the editor
                // during the following frame, and the editor consumes the parked
                // seek as it attaches. An item that was already open rebuilds on
                // focus and consumes it the same way.
                //
                // An orphan is deliberately excluded: it has no live range, and
                // landing the caret at a stale offset would be a lie. Opening its
                // item is still useful, and the row's own badge says why there is
                // nowhere to jump to.
                if row.is_anchored()
                    && let Some(content_id) = row.content_id
                {
                    vm.request_seek(
                        content_id,
                        row.range_start as usize,
                        (row.range_start + row.range_length) as usize,
                    );
                }
                on_open(item_id, row.item_title.clone());
            }
        }
    })
}

/// One thread, rendered as a card.
fn comment_card(
    row: &CommentRow,
    scope: CommentScope,
    vm: CommentsViewModel,
    focus: Signal<Option<u64>>,
) -> impl Widget {
    let id = row.id;
    let resolved = row.resolved;
    let orphaned = row.orphaned;

    // The quoted snippet: what the comment is *about*. A paragraph comment gets a
    // pilcrow instead of quotation marks, so "about this phrase" and "about this
    // whole paragraph" are distinguishable without a second column.
    // Shown, not stored: the quote keeps the sentinel so it still matches the
    // prose it was captured from, but a `U+FFFC` in this list would draw as an
    // unrenderable box. `🖼` reads as "there is a picture here", which is what a
    // comment spanning one is about.
    let quoted = crate::comments::anchor::for_display(&row.quote_exact);
    let snippet = if orphaned {
        tr!(comments_orphan_snippet())
    } else if row.kind == frontend::common::entities::CommentAnchorKind::Paragraph {
        // A pilcrow instead of quotation marks, so "about this paragraph" and
        // "about this phrase" are distinguishable without a second column.
        lit!(format!("¶ {quoted}"))
    } else {
        lit!(format!("“{quoted}”"))
    };

    // The latest turn carries **its own** author, not the thread's: a summary that
    // read "Jane: keep it" when Marc wrote it attributes the wrong opinion.
    let body_line = if let Some(last) = row.latest_reply().filter(|r| !r.body.is_empty()) {
        format!("{}: {}", last.author_name, last.body)
    } else {
        format!("{}: {}", row.author_name, row.body)
    };

    let status = if orphaned {
        tr!(comments_status_orphaned())
    } else if resolved {
        tr!(comments_status_resolved())
    } else {
        tr!(comments_status_open())
    };
    let mut footer =
        HStack::new()
            .spacing(6.0)
            .child(
                TextWidget::new(status)
                    .style(TextStyleRole::Tiny)
                    .color(if orphaned {
                        TextRole::Warning
                    } else {
                        TextRole::Secondary
                    }),
            );
    if row.reply_count() > 0 {
        footer = footer.child(
            TextWidget::new(tr!(comments_reply_count(count = row.reply_count() as i64)))
                .style(TextStyleRole::Tiny)
                .color(TextRole::Secondary),
        );
    }

    let mut item = StandardListItem::new(snippet)
        // The quoted snippet is arbitrary prose and elides like the subtitle
        // already does. Left to wrap (the default) it reports its full single-line
        // width when measured unconstrained, and pushes the status/reply-count
        // slot off the row's trailing edge.
        .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
        .subtitle(lit!(body_line))
        .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
        .trailing_slot(footer);

    // The breadcrumb is the project dock's alone: in the per-document dock it
    // would repeat the dock's own scope on every row.
    if scope == CommentScope::Project && !row.item_title.is_empty() {
        // Capped, and elided within the cap: an item title is user data of any
        // length, and an uncapped one takes as much of the row as it likes — in a
        // 260 dp dock that is the whole row, with the snippet it is supposed to be
        // annotating pushed out past the edge.
        item = item.leading_slot(
            MaxSize::width(BREADCRUMB_WIDTH).child(
                TextWidget::new(lit!(row.item_title.clone()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary)
                    .single_line()
                    .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing)),
            ),
        );
    }

    let menu_vm = vm.clone();
    item.context_menu(move |_pos, _ctx| {
        Some(Box::new(card_menu(
            menu_vm.clone(),
            id,
            resolved,
            orphaned,
            focus.clone(),
        )) as Box<dyn Widget>)
    })
}

/// Reply / Resolve / Delete.
///
/// **Delete is always present**, orphan or not. The Confluence failure this design
/// exists to avoid is precisely a comment the UI will neither reopen nor remove.
fn card_menu(
    vm: CommentsViewModel,
    id: u64,
    resolved: bool,
    orphaned: bool,
    _focus: Signal<Option<u64>>,
) -> impl Widget {
    let mut menu = MenuList::new();

    if resolved {
        let vm = vm.clone();
        menu = menu.item(
            MenuItem::new(tr!(comments_menu_reopen()))
                .on_activate_fn(move |_ctx| vm.reopen(id, None)),
        );
    } else if !orphaned {
        let vm = vm.clone();
        menu = menu.item(
            MenuItem::new(tr!(comments_menu_resolve()))
                .on_activate_fn(move |_ctx| vm.resolve(id, None)),
        );
    }

    let del_vm = vm.clone();
    menu.item(
        MenuItem::new(tr!(comments_menu_delete()))
            .on_activate_fn(move |_ctx| del_vm.delete(id, None)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::CommentsListModel;

    fn vm() -> CommentsViewModel {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        CommentsViewModel::new(
            CommentsListModel::new(ctx.clone(), AppIds::new()),
            ctx,
            Signal::new(None),
        )
    }

    /// Both docks must lay out non-degenerately. They bind a model that subscribes
    /// to backend events, so a bare `WidgetTree` would *panic* in
    /// `subscribe_event` — hence `tree_with_events` (see `crate::test_support`).
    #[test]
    fn both_docks_build_and_lay_out() {
        for scope in [CommentScope::Project, CommentScope::Document] {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let vm = CommentsViewModel::new(
                CommentsListModel::new(ctx.clone(), AppIds::new()),
                ctx.clone(),
                Signal::new(None),
            );
            let on_open: OpenItemFn = std::rc::Rc::new(|_id, _title| {});
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(Box::new(comments_panel(
                vm,
                scope,
                Signal::new(None),
                on_open,
            )));
            tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 700.0));
            let b = tree.bounds(id);
            assert!(
                b.width > 0.0 && b.height > 0.0,
                "{:?} dock laid out to zero size ({b:?})",
                match scope {
                    CommentScope::Project => "project",
                    CommentScope::Document => "document",
                }
            );
        }
    }

    /// The orphan chip is part of the always-on contract: it must be built whether
    /// or not anything is actually orphaned, so a lone orphan can never hide behind
    /// a filter that only appears once something is already wrong.
    ///
    /// Both sides of that condition are covered between the two builds: the real
    /// model starts from an empty store, while the mock fixture ships an orphan.
    /// Pinning the count here would therefore make the test true in one build and
    /// false in the other — which is exactly how it went unnoticed that the whole
    /// mock test binary had stopped compiling.
    #[test]
    fn the_filter_bar_builds_whether_or_not_anything_is_orphaned() {
        let vm = vm();
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(Box::new(filter_bar(vm)));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 40.0));
        assert!(tree.bounds(id).width > 0.0);
    }

    /// The widest child right edge that sticks out past its parent's, anywhere in
    /// the subtree — 0.0 when everything fits.
    ///
    /// This is the measurement the two tests above could not make. Both only ever
    /// asserted `width > 0.0`, which a row overflowing its dock by 200 px passes
    /// just as happily as one that fits.
    fn worst_overflow(
        tree: &teksilo::core::widget_tree::WidgetTree,
        root: WidgetId,
    ) -> (f32, String) {
        let mut worst: f32 = 0.0;
        let mut who = String::new();
        let mut stack = vec![root];
        let short = |t: Option<&'static str>| {
            t.unwrap_or("?")
                .rsplit("::")
                .next()
                .unwrap_or("?")
                .to_string()
        };
        while let Some(id) = stack.pop() {
            let p = tree.bounds(id);
            for kid in tree.children(id) {
                let b = tree.bounds(kid);
                if b.width > 0.0 && p.width > 0.0 {
                    let over = (b.x + b.width) - (p.x + p.width);
                    if over > worst {
                        worst = over;
                        who = format!(
                            "{} inside {}",
                            short(tree.widget_type_name(kid)),
                            short(tree.widget_type_name(id))
                        );
                    }
                }
                stack.push(kid);
            }
        }
        (worst, who)
    }

    /// The filter bar fits the dock it lives in — at **both** sides' widths.
    ///
    /// The regression: four rigid `Button` chips plus a long-labelled sort control
    /// in an `HStack`. `Button` drops its shrink weight by framework contract, so
    /// the row neither compressed nor elided — the last chip and the whole sort
    /// control simply ran off the end of the dock, and did so in every locale.
    ///
    /// 260 dp is the framework's default side size (the leading side, where the
    /// project dock lives); 300 dp is the trailing side this app sets explicitly.
    ///
    /// Scoped to the **header**, not the whole panel. A row's snippet and subtitle
    /// elide, and elision is exactly what the headless text backend does not
    /// reproduce — measured through `tree_with_events` a row reports the width its
    /// untruncated text would take, so a whole-dock assertion fails on rows that
    /// demonstrably render elided in the app. The header needs no text backend to
    /// be measured honestly: `Button` is rigid by contract, so whether the row fits
    /// is decided by the layout, not by the font.
    #[test]
    fn the_filter_bar_fits_both_dock_widths() {
        for width in [260.0_f32, 300.0] {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(Box::new(filter_bar(vm())));
            tree.layout(teksilo::prelude::SizeProposal::exact(width, 200.0));
            let (over, who) = worst_overflow(&tree, id);
            assert!(
                over <= 1.0,
                "the filter bar overflows a {width} dp dock by {over:.0} dp ({who}) — \
                 it has to wrap, not run off the end"
            );
        }
    }
}
