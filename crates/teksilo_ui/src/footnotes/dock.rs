// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The footnotes dock — every note in the manuscript, in the order a reader will
//! meet them.
//!
//! Its own surface on the trailing rail, and deliberately **not** the comment
//! margin. The two look alike and are not alike at all: a comment is a remark
//! about the book, a footnote is part of it. Putting them together would teach
//! writers that a footnote is commentary — and would put a card that ships to
//! readers beside one that never leaves the project.
//!
//! # A row is a number, a note and where it lives
//!
//! The marker chip carries the number the book prints, so the dock and the prose
//! agree at a glance. Under it, the note's own words, editable in place. Beside
//! it, the scene the reference sits in — a footnote with no home is the one thing
//! this dock has to make obvious, because nothing in the prose can.
//!
//! # Navigation both ways
//!
//! Clicking a row opens its scene and puts the caret on the marker (through a
//! parked seek, since the editor may not exist yet). Moving the caret onto a
//! marker in the prose lights up its row here. Neither direction goes through the
//! other: the dock never reaches into an editor, and an editor never reaches into
//! the dock — both talk to [`FootnotesViewModel`].
//!
//! The list is a [`ListView`] rather than a table: a row is a variable-height
//! composite (a chip, a wrapping body editor, a breadcrumb), which is the free-form
//! shape a delegate exists for. Columns cannot hold a body that wraps.

use teksilo::core::overlay::OverlayPlacement;
use teksilo::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use teksilo::core::widget::WidgetPlacement;
use teksilo::data::ListModel;
use teksilo::prelude::*;
use teksilo::tokens::CornerRadius;
use teksilo::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use teksilo::widgets::{
    ActivateOn, Button, ButtonVariant, Divider, DockOpenLocation, DockSide, DockWidget,
    DockWidgetId, Expand, FocusScope, HStack, IconButton, IconWidget, ListView, MenuItem, MenuList,
    Padding, Panel, PopoverIconButton, ScrollBarMode, Spacer, Switcher, TextWidget, ToolbarItem,
    TraversalScopePolicy, VStack, Wrap,
};

use crate::binder::dock::OpenItemFn;
use crate::footnotes::{FootnoteFilter, FootnotesViewModel};
use crate::models::FootnoteRow;

/// How many lines of a note's body the editor shows before it grows.
const BODY_MIN_LINES: u32 = 2;

/// Package the footnotes panel as a trailing `DockWidget`.
///
/// `focus` is the active editor tab's item id — the same signal the Inspector and
/// the per-document comments dock take, so "this document" means the same thing on
/// every trailing tab.
pub fn footnotes_dock(
    vm: FootnotesViewModel,
    dock_id: DockWidgetId,
    focus: Signal<Option<u64>>,
    on_open: OpenItemFn,
) -> DockWidget {
    DockWidget::new(dock_id, tr!(footnotes_title()), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(footnotes_panel(
            vm.clone(),
            focus.clone(),
            on_open.clone(),
        ))
    })
    .icon(crate::icons::activity::footnotes_icon)
    .show_header(true)
    // The way in, pinned where a writer looks for it.
    //
    // A footnote is created *in the prose*, so the command lives in the Document
    // menu with the other insert-at-the-caret verbs — and that is exactly where
    // nobody found it. The dock is where someone goes when they are thinking
    // about footnotes, so the dock is where the button belongs. It fires the same
    // action the menu and `Ctrl+Alt+F` do, so the three cannot drift; it is not
    // gated on there being a caret, because the command already says so out loud
    // and a permanently-greyed button teaches nothing.
    .header_actions(|_id| {
        vec![ToolbarItem::custom(
            IconButton::add()
                .toolbar()
                .tooltip(tr!(footnotes_insert_tooltip()))
                .on_activate_fn(|ctx| ctx.send_intent(Intent::new("editor.insert_footnote"))),
        )]
    })
    .default_location(DockOpenLocation::side(DockSide::Trailing))
}

fn footnotes_panel(
    vm: FootnotesViewModel,
    focus: Signal<Option<u64>>,
    on_open: OpenItemFn,
) -> impl Widget {
    VStack::new()
        .spacing(0.0)
        .child(filter_bar(vm.clone()))
        .child(Divider::new())
        .child(Expand::new().child(list_body(vm, focus, on_open)))
}

/// The filter chips, with the orphan count beside the chip that finds them.
///
/// `Button` is rigid by framework contract — it keeps its natural width in an
/// over-constrained row rather than eliding — so the chips go in a `Wrap`, not an
/// `HStack`. That is not a preference: three chips plus a count in a fixed row ran
/// off the end of a 260 dp dock, in every locale, the last time this shape was
/// built next door.
fn filter_bar(vm: FootnotesViewModel) -> impl Widget {
    let current = vm.filter();
    let version = vm.model().version_signal();

    let chip = |label: LocalizedString, which: FootnoteFilter, vm: FootnotesViewModel| {
        let current = vm.filter();
        Button::new(label)
            .variant(ButtonVariant::Plain)
            .enabled(current.map(move |c| *c != which))
            .on_activate_fn(move |_ctx| vm.set_filter(which))
    };

    // The label stays a real translated string and only the number is reactive, so
    // the chip is present even at zero — an orphan count that appeared and
    // disappeared would be a control that moves under the pointer.
    let orphan_count = {
        let vm = vm.clone();
        version.map(move |_| vm.orphan_count().to_string())
    };
    let orphan_btn = {
        let vm = vm.clone();
        HStack::new()
            .spacing(2.0)
            .child(
                Button::new(tr!(footnotes_filter_orphaned()))
                    .variant(ButtonVariant::Plain)
                    .enabled(current.map(|c| *c != FootnoteFilter::Orphaned))
                    .on_activate_fn(move |_ctx| vm.set_filter(FootnoteFilter::Orphaned)),
            )
            .child(
                TextWidget::new(lit!(""))
                    .text(orphan_count)
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            )
    };

    Padding::symmetric(8.0, 6.0).child(
        Wrap::new()
            .spacing(4.0)
            .line_spacing(4.0)
            .child(chip(
                tr!(footnotes_filter_all()),
                FootnoteFilter::All,
                vm.clone(),
            ))
            .child(chip(
                tr!(footnotes_filter_document()),
                FootnoteFilter::ThisDocument,
                vm.clone(),
            ))
            .child(orphan_btn),
    )
}

fn list_body(
    vm: FootnotesViewModel,
    focus: Signal<Option<u64>>,
    on_open: OpenItemFn,
) -> impl Widget {
    FootnotesList {
        model: ListModel::from_vec(vm.visible_rows(focus.get())),
        vm,
        focus,
        on_open,
        child: None,
    }
}

/// The list, wrapped in a widget so the re-derivation effects have a context.
///
/// A plain builder cannot subscribe: `ctx.effect` needs a `BuildContext`, and the
/// rows here depend on three separate signals (the model's version, the dock's
/// filter, and which document is in front of the writer). Without the
/// subscription the dock would show whatever was true when it was first built —
/// which is nothing, because a project has no notes at the instant its window
/// opens.
struct FootnotesList {
    vm: FootnotesViewModel,
    focus: Signal<Option<u64>>,
    model: ListModel<FootnoteRow>,
    on_open: OpenItemFn,
    child: Option<WidgetId>,
}

impl std::fmt::Debug for FootnotesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FootnotesList").finish_non_exhaustive()
    }
}

impl Widget for FootnotesList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let empty = Padding::symmetric(16.0, 24.0).child(
            TextWidget::new(tr!(footnotes_empty()))
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        );

        let list = {
            let vm = self.vm.clone();
            let model = self.model.clone();
            let on_open = self.on_open.clone();
            let caret = self.vm.caret_label();
            ListView::new(self.model.clone(), move |_i, row: &FootnoteRow, _sel| {
                Box::new(note_row(row, vm.clone(), caret.clone())) as Box<dyn Widget>
            })
            .auto_item_height(72.0)
            .scroll_bar_style(ScrollBarMode::Overlay)
            // Single click navigates: a note is a place in the book, not a
            // selection.
            .activate_on(ActivateOn::SingleClick)
            .on_activate({
                let model = model.clone();
                let vm = self.vm.clone();
                move |idx, _ctx| {
                    let Some(row) = model.with_item(idx, |r| r.clone()) else {
                        return;
                    };
                    let Some(item_id) = row.item_id else {
                        // An orphan has nowhere to go, and its badge already says
                        // why. Opening *something* would be a guess.
                        return;
                    };
                    // Park the seek before opening: a fresh open builds the editor
                    // during the following frame, and the editor consumes the
                    // parked request as it attaches.
                    if let Some(content_id) = row.content_id {
                        vm.request_seek(content_id, &row.label);
                    }
                    on_open(item_id, row.item_title.clone());
                }
            })
        };

        let has_rows = Signal::new(!self.model.is_empty());
        let switcher = Switcher::new(has_rows.clone().map(|n| usize::from(*n)))
            .child(empty)
            .child(list);
        let id = ctx.add(switcher);
        self.child = Some(id);

        // Re-derive whenever anything the list depends on moves. Separate effects
        // (a combined `zip` is derived, and observing a derived signal panics).
        let rederive = {
            let vm = self.vm.clone();
            let focus = self.focus.clone();
            let model = self.model.clone();
            let has_rows = has_rows.clone();
            move || {
                let rows = vm.visible_rows(focus.get());
                has_rows.set(!rows.is_empty());
                model.reconcile_by_key(rows, |r| r.id);
            }
        };
        rederive();
        {
            // **Structure**, not version. A row's body editor commits on every
            // keystroke, which refreshes the model; rebuilding the list on that
            // re-mints the editor being typed into and the writer loses the
            // caret after one character. See `structure_key`.
            let f = rederive.clone();
            ctx.effect(&self.vm.model().structure_signal(), move |_| f());
        }
        {
            let f = rederive.clone();
            ctx.effect(&self.vm.filter(), move |_| f());
        }
        {
            let f = rederive.clone();
            ctx.effect(&self.focus, move |_| f());
        }
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x, bounds.y);
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child.into_iter().collect()
    }
}

/// One note: its marker, where it lives, its own words, and its menu.
///
/// A composite rather than a `StandardListItem`, because the body is *editable*:
/// a list item's label is a string, and the one thing this row has to offer is a
/// place to write. The header carries the parts that are not writable — the number,
/// the breadcrumb, and the menu — so the editor below it is the only thing in the
/// row that takes a caret.
fn note_row(
    row: &FootnoteRow,
    vm: FootnotesViewModel,
    caret: Signal<Option<String>>,
) -> impl Widget {
    let id = row.id;
    let label = row.label.clone();
    let here = caret.map(move |c| c.as_deref() == Some(label.as_str()));

    let home: LocalizedString = if row.orphaned {
        tr!(footnotes_orphaned())
    } else if row.item_title.trim().is_empty() {
        tr!(footnotes_untitled_home())
    } else {
        lit!(row.item_title.clone())
    };
    // An orphan's breadcrumb is not a place, it is a warning — and it is the only
    // thing on the row that says so, since nothing in the prose can.
    let home_color = if row.orphaned {
        TextRole::Warning
    } else {
        TextRole::Secondary
    };

    let menu_vm = vm.clone();
    let actions = tr!(footnotes_actions());
    let more = PopoverIconButton::new(
        IconButton::new(IconWidget::chevron_down(12.0)).tooltip(actions.clone()),
    )
    .bare()
    .show_disclosure_caret(false)
    .content(row_menu(id, menu_vm))
    .placement(OverlayPlacement::BelowPreferred)
    .access_label(actions);

    let header = HStack::new()
        .spacing(6.0)
        .child(marker_chip(row, here))
        .child(
            TextWidget::new(home)
                .style(TextStyleRole::Tiny)
                .color(home_color)
                .single_line()
                .overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing)),
        )
        .child(Spacer::new())
        .child(more);

    Padding::symmetric(8.0, 6.0).child(
        VStack::new()
            .spacing(3.0)
            .child(header)
            .child(body_editor(row, vm)),
    )
}

/// The number the book prints, in a chip that lights up when the caret is standing
/// on this note's marker in the prose.
fn marker_chip(row: &FootnoteRow, here: Signal<bool>) -> impl Widget {
    let background = here.map(|on| {
        if *on {
            SurfaceRole::Accent
        } else {
            SurfaceRole::Raised
        }
    });
    let ink = here.map(|on| {
        if *on {
            TextRole::OnAccent
        } else {
            TextRole::Secondary
        }
    });
    Panel::new()
        .background(background)
        .corner_radius(3.0)
        .padding(2.0)
        .child(
            Padding::symmetric(4.0, 0.0).child(
                TextWidget::new(lit!(row.marker()))
                    .style(TextStyleRole::Tiny)
                    .color(ink),
            ),
        )
}

/// The note's own words, edited in place and committed on every change.
///
/// A `RichTextEditor` rather than a `TextInput`, and Djot rather than plain text:
/// a footnote carries the citations and the emphasis a `TextInput` has no room for
/// — and the body reaches the exporter as markup, so anything less would silently
/// flatten it on the way in.
fn body_editor(row: &FootnoteRow, vm: FootnotesViewModel) -> impl Widget {
    let id = row.id;
    let doc = vm.body_doc(id, &row.body);
    let commit = {
        let vm = vm.clone();
        let doc = doc.clone();
        move || {
            let text = doc.to_djot().unwrap_or_default();
            vm.set_body(id, &text);
        }
    };
    RichTextEditor::editor(doc)
        // A footnote body is where a citation's URL naturally lands, whether
        // typed, pasted or imported. Nothing here can *make* a link, but one
        // that arrives has to be followable rather than dead.
        .on_link_activated(|href, ctx| {
            crate::shared::external_link::open_external_link(href, ctx);
        })
        .min_lines(BODY_MIN_LINES)
        .v_scroll_policy(ScrollPolicy::AlwaysOff)
        .style(NoteBodyStyle { id, vm })
        .on_change(commit)
}

/// A body that reads as prose, not as a form field — and the one place that
/// reports this row's *real* keyboard focus back to the view-model.
///
/// The default editor recipe frames every body in a filled, bordered box; three of
/// those stacked in a 300 dp dock read as a settings pane rather than as three
/// notes. So the fill goes, and the affordance is the focus ring alone.
///
/// `cfg.is_focused` is also the only place in the dock that knows, moment to
/// moment, whether a caret is genuinely sitting in *this* row's body — so this
/// forwards it to [`FootnotesViewModel::set_editing`], which
/// [`body_doc`](crate::footnotes::FootnotesViewModel::body_doc) then
/// consults before ever re-syncing an already-cached document from a body
/// change that landed elsewhere. Without this, that gate would have nothing
/// truthful to read and could only guess.
struct NoteBodyStyle {
    id: u64,
    vm: FootnotesViewModel,
}

impl RichTextEditorStyle for NoteBodyStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        {
            let vm = self.vm.clone();
            let id = self.id;
            ctx.effect(&cfg.is_focused, move |focused| {
                if *focused {
                    vm.set_editing(Some(id));
                } else if vm.editing().get() == Some(id) {
                    vm.set_editing(None);
                }
            });
        }
        if cfg.is_read_only {
            return match cfg.content_padding {
                Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
                None => cfg.viewport,
            };
        }
        let ring = ctx.theme_signal().get().shape.focus_ring_width;
        let border = cfg.is_focused.map(move |f| if *f { ring } else { 0.0 });
        let bg = ctx.add(
            teksilo::widgets::RectWidget::new()
                .background(SurfaceRole::Transparent)
                .border_color(BorderRole::Focused)
                .border_width(border)
                .corner_radius(CornerRadius::uniform(3.0)),
        );
        let (pt, pr, pb, pl) = cfg.content_padding.unwrap_or((2.0, 4.0, 2.0, 4.0));
        let padded = ctx.add(Padding::new(pt, pr, pb, pl).child_id(cfg.viewport));
        ctx.add(
            teksilo::widgets::ZStack::new()
                .add_child(bg)
                .add_child(padded),
        )
    }
}

/// The row's one menu.
///
/// Delete is the only entry, and it is always present: everything else a writer
/// does to a note — its words, where it points — is done in the prose or in the
/// box above. Deleting takes the reference with it, because a `[^label]` with
/// nothing behind it still renders and still reaches the exporter with nothing to
/// print (see the model's own note).
///
/// No confirmation dialog first — see `FootnotesViewModel::delete`'s own doc
/// comment for why the click fires immediately and a grace-window "Undo" toast
/// is the safety net instead.
fn row_menu(id: u64, vm: FootnotesViewModel) -> MenuList {
    MenuList::new()
        .item(MenuItem::new(tr!(footnotes_delete())).on_activate_fn(move |c| vm.delete(c, id)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::{FootnotesListModel, OpenDocsStore};

    fn vm(ctx: &std::rc::Rc<frontend::AppContext>) -> FootnotesViewModel {
        let docs = OpenDocsStore::new(ctx.clone());
        FootnotesViewModel::new(
            FootnotesListModel::new(ctx.clone(), AppIds::new(), docs.clone()),
            docs,
            Signal::new(None),
        )
    }

    /// The dock must lay out non-degenerately. It binds a model that subscribes to
    /// backend events, so a bare `WidgetTree` would *panic* inside
    /// `subscribe_event` — hence `tree_with_events` (see `crate::test_support`).
    #[test]
    fn the_dock_builds_and_lays_out() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let on_open: OpenItemFn = std::rc::Rc::new(|_id, _title| {});
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let id = tree.add_boxed(Box::new(footnotes_panel(
            vm(&ctx),
            Signal::new(None),
            on_open,
        )));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 700.0));
        let b = tree.bounds(id);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "laid out to nothing ({b:?})"
        );
    }

    /// The widest child right edge that sticks out past its parent's, anywhere in
    /// the subtree — 0.0 when everything fits.
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

    /// The chips have to **wrap**, not run off the end.
    ///
    /// `Button` is rigid by framework contract — it keeps its natural width in an
    /// over-constrained row rather than eliding — so three chips plus a count in an
    /// `HStack` would simply leave the dock, taking the orphan count with them. The
    /// header is measured rather than the whole panel because it needs no text
    /// backend to be measured honestly: whether it fits is decided by the layout,
    /// not by the font.
    #[test]
    fn the_filter_bar_fits_the_trailing_dock() {
        for width in [260.0_f32, 300.0] {
            let ctx = std::rc::Rc::new(frontend::AppContext::new());
            let mut tree = crate::test_support::tree_with_events(&ctx);
            let id = tree.add_boxed(Box::new(filter_bar(vm(&ctx))));
            tree.layout(teksilo::prelude::SizeProposal::exact(width, 200.0));
            let (over, who) = worst_overflow(&tree, id);
            assert!(
                over <= 1.0,
                "the filter bar overflows a {width} dp dock by {over:.0} dp ({who}) — \
                 it has to wrap, not run off the end"
            );
        }
    }

    /// A row lays out with its marker, its breadcrumb and its body editor, at the
    /// width the trailing rail actually gives it.
    #[test]
    fn a_row_builds_at_dock_width() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let row = FootnoteRow {
            id: 1,
            label: "fn1".into(),
            body: "The parish register gives the name as Aleyn.".into(),
            content_id: Some(11),
            item_id: Some(10),
            item_title: "The Ridgeline".into(),
            number: Some(3),
            ordinal: 3,
            orphaned: false,
        };
        let id = tree.add_boxed(Box::new(note_row(&row, vm(&ctx), Signal::new(None))));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 200.0));
        let b = tree.bounds(id);
        assert!(
            b.width > 0.0 && b.height > 0.0,
            "row laid out to nothing ({b:?})"
        );
    }

    /// An orphan's row still builds — and it is the row that matters most, because
    /// nothing in the prose can say a note has been left behind.
    #[test]
    fn an_orphan_row_builds_with_no_home_to_show() {
        let ctx = std::rc::Rc::new(frontend::AppContext::new());
        let mut tree = crate::test_support::tree_with_events(&ctx);
        let row = FootnoteRow {
            id: 2,
            label: "fn2".into(),
            body: "A note whose sentence was cut.".into(),
            ordinal: usize::MAX,
            orphaned: true,
            ..Default::default()
        };
        assert_eq!(row.marker(), crate::models::UNNUMBERED_MARKER);
        let id = tree.add_boxed(Box::new(note_row(&row, vm(&ctx), Signal::new(None))));
        tree.layout(teksilo::prelude::SizeProposal::exact(300.0, 200.0));
        assert!(tree.bounds(id).height > 0.0);
    }
}
