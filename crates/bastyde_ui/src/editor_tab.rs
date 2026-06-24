//! Phase 3 — the editor shown inside a dynamic tab.
//!
//! Dual-pane (Skribisto's signature): a synopsis editor above the main-text
//! editor, each a Bastyde `RichTextEditor` bound to its own
//! `text_document::TextDocument`. The documents live on the tab payload, so
//! edits survive tab rebuilds and can later be read back via `to_markdown()` to
//! save into the item's `Content` rows.

use bastyde::core::Key::E;
use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::tokens::{BorderRole, CornerRadius, SurfaceRole};
use bastyde::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    Divider, Expand, GroupHeader, HStack, MaxSize, Padding, Panel, RectWidget, Spacer, TextWidget, VStack, ZStack,
};

/// How much narrower (px, total across both margins) the synopsis column is than
/// the main writing column, so it reads as the subordinate pane.
const SYNOPSIS_WIDTH_INSET: f32 = 48.0;

/// Per-tab editor state (the dynamic-tab payload). Owns the two live documents.
pub struct EditorTab {
    /// The `BinderItem` this tab edits — used to focus an already-open tab
    /// instead of opening a duplicate.
    pub item_id: u64,
    pub main_doc: TextDocument,
    pub synopsis_doc: TextDocument,
    /// Max width (px) of the centered main-text column — a shared, persisted
    /// settings signal, so the slider in Settings resizes every open editor live.
    pub column_width: Signal<f32>,
}

impl EditorTab {
    /// Build from an item's Markdown content. `main_md` is the scene/note text,
    /// `synopsis_md` the synopsis; blank strings yield empty documents.
    pub fn new(item_id: u64, main_md: &str, synopsis_md: &str, column_width: Signal<f32>) -> Self {
        let main_doc = TextDocument::new();
        let _ = main_doc.set_markdown(main_md).and_then(|op| op.wait());
        let synopsis_doc = TextDocument::new();
        let _ = synopsis_doc
            .set_markdown(synopsis_md)
            .and_then(|op| op.wait());
        Self {
            item_id,
            main_doc,
            synopsis_doc,
            column_width,
        }
    }
}

/// The dual-pane editor widget for one tab. Built by the `TabWidget`'s
/// `dynamic_tab::<EditorTab>` factory; binds to the payload's documents so edits
/// flow straight back into them.
pub fn editor_pane(state: &EditorTab) -> Box<dyn Widget> {
    // Centered, max-width writing column:
    //  - `CenterColumn` proposes the *bounded* available size to the capped
    //    child, so the column tracks the width setting when there's room and
    //    shrinks to the pane when there isn't (instead of overflowing).
    //  - `MaxSize` caps the width (a live, persisted setting).
    //  - the inner `Expand` (fill mode) stretches the editor to the capped box:
    //    `RichTextEditor` sizes to its *content*, not greedily, so without this
    //    it collapses to a few content-sized pixels.
    //
    // `CenterColumn` takes its child as a positional constructor arg (it has no
    // `.child` method), so the capped column is built as a separate `bati!` value
    // and handed in positionally.
    let column = CenterColumn::new(bati!(
        MaxSize::width(state.column_width.get()) {
            bind_max_width: state.column_width.clone()
            Expand {
                RichTextEditor::editor(state.main_doc.clone()) {
                    style: WritingEditorStyle
                    content_padding_symmetric: 8.0, 12.0
                    v_scroll_policy: ScrollPolicy::Auto
                }
            }
        }
    ));

    // The synopsis tracks the *same* live settings width as the main column, but
    // capped a touch narrower so it reads as the subordinate pane. `map` derives a
    // read-only signal that re-fires whenever the slider moves, so both columns
    // resize together.
    let synopsis_width = state
        .column_width
        .map(|w| (w - SYNOPSIS_WIDTH_INSET).max(0.0));

    // A flat, square Content-surface backdrop behind the whole editor: the
    // `TabWidget` doesn't paint a content background, so this Panel supplies one.
    // `corner_radius: 0.0` keeps it edge-to-edge (no rounded card look) and
    // `padding: 0.0` lets the columns own their own insets.
    Box::new(bati!(
        Panel {
            background: SurfaceRole::Content
            corner_radius: 0.0
            padding: 0.0
            VStack {
                spacing: 4.0
                GroupHeader::new(lit!("Synopsis")) {
                    style: TextStyleRole::SmallBold
                    color: TextRole::Secondary
                }
                HStack {
                    Spacer
                    MaxSize::width(synopsis_width.get()) {
                        bind_max_width: synopsis_width.clone()
                        Expand::horizontal {
                            Panel {
                                background: SurfaceRole::Content
                                border_color: BorderRole::Default
                                border_width: 1.0
                                corner_radius: 6.0
                                RichTextEditor::editor(state.synopsis_doc.clone()) {
                                    style: WritingEditorStyle
                                    content_padding_symmetric: 6.0, 30.0
                                    min_lines: 1
                                    max_lines: 6
                                    v_scroll_policy: ScrollPolicy::Auto
                                }
                            }
                        }
                    }
                    Spacer
                }
                GroupHeader::new(lit!("Text")) {
                    style: TextStyleRole::SmallBold
                    color: TextRole::Secondary
                }
                Expand {
                    child: column
                }
            }
        }
    ))
}

/// Editor chrome with a **constant** border instead of the default recipe's
/// focus-aware one. The stock `RichTextEditorStyle` swaps the border to the
/// accent focus ring while focused — and since a writing editor is almost
/// always focused, that reads as a permanent accent frame. This keeps a quiet
/// 1px `Default` border at all times. Mirrors the recipe's frame otherwise
/// (content surface, padding from the widget's `content_padding`, rounded).
#[derive(Debug, Default, Clone, Copy)]
struct WritingEditorStyle;

impl RichTextEditorStyle for WritingEditorStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        if cfg.is_read_only {
            return match cfg.content_padding {
                Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
                None => cfg.viewport,
            };
        }
        let bg = ctx.add(
            RectWidget::new()
                .background(SurfaceRole::Content)
                .border_color(BorderRole::Default)
                .border_width(0.0)
                .corner_radius(CornerRadius::uniform(6.0)),
        );
        let (pt, pr, pb, pl) = cfg.content_padding.unwrap_or((8.0, 12.0, 8.0, 12.0));
        let padded = ctx.add(Padding::new(pt, pr, pb, pl).child_id(cfg.viewport));
        ctx.add(ZStack::new().add_child(bg).add_child(padded))
    }
}

/// Fills the available space and centers its single child **horizontally**,
/// proposing the *bounded* available size to it. Unlike `Center` (which measures
/// the child with an unbounded proposal — so a `MaxSize` cap always reports its
/// maximum), this lets a width-capped child shrink to the available width when
/// the pane is narrower than the cap, so the writing column never overflows.
/// Height fills.
#[derive(Debug)]
struct CenterColumn {
    child_id: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
}

impl CenterColumn {
    fn new(child: impl Widget + 'static) -> Self {
        Self {
            child_id: None,
            pending: Some(Box::new(child)),
        }
    }
}

impl Widget for CenterColumn {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            self.child_id = Some(ctx.add_boxed(w));
        }
        self.child_id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        // Claim all offered space; the child is sized + centered in place_children.
        proposal.resolve(0.0, 0.0).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            // Bounded proposal → a `MaxSize` child reports `min(available, cap)`
            // rather than its full cap, so it shrinks to fit a narrow pane.
            let size = ctx
                .child_size(child.id, SizeProposal::exact(bounds.width, bounds.height))
                .unwrap_or_else(|| bounds.size());
            let dx = ((bounds.width - size.width) / 2.0).max(0.0);
            child.origin = Point::new(bounds.x + dx, bounds.y);
            child.size = size;
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child_id.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    /// A content-sized leaf: reports a small fixed natural size and ignores the
    /// proposal — like `RichTextEditor`, which sizes to its content, not to the
    /// offered space. The inner `Expand` must stretch it to fill the column.
    #[derive(Debug)]
    struct ContentLeaf;
    impl Widget for ContentLeaf {
        fn layout_response(&self, _p: SizeProposal, _c: &LayoutContext) -> LayoutResponse {
            Size::new(40.0, 12.0).into()
        }
    }

    fn deepest(tree: &WidgetTree, mut id: WidgetId) -> WidgetId {
        while let Some(&k) = tree.children(id).first() {
            id = k;
        }
        id
    }

    /// Lay out the real writing-column composition and return the editor leaf's
    /// final bounds.
    fn editor_bounds(cap: f32, avail_w: f32, avail_h: f32) -> Rect {
        let mut tree = WidgetTree::new();
        let root = tree.add(Expand::new().child(CenterColumn::new(
            MaxSize::width(cap).child(Expand::new().child(ContentLeaf)),
        )));
        tree.layout(SizeProposal::exact(avail_w, avail_h));
        tree.bounds(deepest(&tree, root))
    }

    #[test]
    fn caps_width_and_centers_when_pane_is_wide() {
        // avail 1000 > cap 400 → column is 400 wide, centered (x≈300), full height.
        let b = editor_bounds(400.0, 1000.0, 600.0);
        assert!(
            (b.width - 400.0).abs() < 0.5,
            "width capped at 400, got {}",
            b.width
        );
        assert!(
            (b.height - 600.0).abs() < 0.5,
            "height fills 600, got {}",
            b.height
        );
        assert!((b.x - 300.0).abs() < 0.5, "centered at x=300, got {}", b.x);
    }

    #[test]
    fn shrinks_to_pane_when_narrower_than_cap() {
        // avail 250 < cap 400 → column shrinks to 250 wide, x≈0, full height.
        let b = editor_bounds(400.0, 250.0, 600.0);
        assert!(
            (b.width - 250.0).abs() < 0.5,
            "width shrinks to 250, got {}",
            b.width
        );
        assert!(
            (b.height - 600.0).abs() < 0.5,
            "height fills 600, got {}",
            b.height
        );
        assert!(
            b.x.abs() < 0.5,
            "no left margin at full width, got x={}",
            b.x
        );
    }
}
