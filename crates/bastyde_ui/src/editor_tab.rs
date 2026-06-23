//! Phase 3 — the editor shown inside a dynamic tab.
//!
//! Dual-pane (Skribisto's signature): a synopsis editor above the main-text
//! editor, each a Bastyde `RichTextEditor` bound to its own
//! `text_document::TextDocument`. The documents live on the tab payload, so
//! edits survive tab rebuilds and can later be read back via `to_markdown()` to
//! save into the item's `Content` rows.

use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use bastyde::widgets::{Divider, Expand, HStack, MaxSize, Spacer, TextWidget, VStack};

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
        let _ = synopsis_doc.set_markdown(synopsis_md).and_then(|op| op.wait());
        Self { item_id, main_doc, synopsis_doc, column_width }
    }
}

/// The dual-pane editor widget for one tab. Built by the `TabWidget`'s
/// `dynamic_tab::<EditorTab>` factory; binds to the payload's documents so edits
/// flow straight back into them.
pub fn editor_pane(state: &EditorTab) -> Box<dyn Widget> {
    // Multi-arg builders + no `BuildContext` here, so this reads cleaner as plain
    // builder calls than `bati!`.
    let synopsis = RichTextEditor::editor(state.synopsis_doc.clone())
        .content_padding_symmetric(6.0, 10.0)
        .min_lines(2)
        .max_lines(6)
        .v_scroll_policy(ScrollPolicy::Auto);
    let main = RichTextEditor::editor(state.main_doc.clone())
        .content_padding_symmetric(8.0, 12.0)
        .v_scroll_policy(ScrollPolicy::Auto);

    // Centered, max-width writing column: spacers push a width-capped column to
    // the middle; the cap is a live, persisted setting. On windows narrower than
    // the cap the spacers collapse and the column uses the full width.
    let column = HStack::new()
        .child(Spacer::new())
        .child(
            MaxSize::width(state.column_width.get())
                .bind_max_width(state.column_width.clone())
                // `Expand` makes the editor fill the column's width AND height;
                // without it the greedy editor falls back to ~100px tall.
                .child(Expand::new().child(main)),
        )
        .child(Spacer::new());

    Box::new(
        VStack::new()
            .spacing(4.0)
            .child(
                TextWidget::new(lit!("Synopsis"))
                    .style(TextStyleRole::SmallBold)
                    .color(TextRole::Secondary),
            )
            .child(synopsis)
            .child(Divider::new())
            .child(
                TextWidget::new(lit!("Text"))
                    .style(TextStyleRole::SmallBold)
                    .color(TextRole::Secondary),
            )
            .child(Expand::new().child(column)),
    )
}
