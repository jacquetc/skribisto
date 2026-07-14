//! **Phase 0.2 de-risking stub** — the search preview dock (bottom side).
//!
//! This exists to answer one question before the real search feature is built:
//! *can an editable `RichTextEditor` live inside a dock at all?* No editable
//! widget exists inside any `DockWidget` in this codebase or in bastyde's own
//! examples — every dock so far is a tree, a list, or read-only text. A side's
//! content is `visible_when`-parked while the side is collapsed
//! (`bastyde-widgets/src/docking.rs`), and that path has only ever been
//! exercised by non-focusable content.
//!
//! So: mount a real editor here, collapse and reveal the bottom band a few
//! times, and confirm the caret, the keyboard focus and the editor's own local
//! undo survive the cycle. If they don't, the search feature's editable preview
//! needs a different shape and it is far cheaper to learn that now.
//!
//! Replaced by the real preview (bound to the selected result's `OpenDoc`) once
//! this is answered.

use bastyde::core::styles::{RichTextEditorStyle, RichTextEditorStyleConfig};
use bastyde::prelude::*;
use bastyde::text_document::TextDocument;
use bastyde::widgets::rich_text::{RichTextEditor, ScrollPolicy};
use bastyde::widgets::{
    DockOpenLocation, DockSide, DockWidget, DockWidgetId, Expand, FocusScope, Padding,
    TraversalScopePolicy,
};

/// An editor that paints **no background of its own**, so it sits flush on whatever
/// surface the dock is painted with.
///
/// The default `RichTextEditor` style draws a rounded `SurfaceRole::Content` rect —
/// which reads as a floating white card dropped onto the dock, not as part of it.
/// Rather than hard-code the dock's role here (and have the two drift apart the next
/// time the theme moves), draw nothing: the dock's own surface shows through, so the
/// two can never disagree.
struct SeamlessEditorStyle;

impl RichTextEditorStyle for SeamlessEditorStyle {
    fn make_body(&self, cfg: &RichTextEditorStyleConfig, ctx: &mut BuildContext) -> WidgetId {
        match cfg.content_padding {
            Some((t, r, b, l)) => ctx.add(Padding::new(t, r, b, l).child_id(cfg.viewport)),
            None => cfg.viewport,
        }
    }
}

/// A scratch document with enough prose to show a caret moving around.
fn scratch_doc() -> TextDocument {
    let doc = TextDocument::new();
    let _ = doc
        .set_djot(
            "Le vent s'était levé d'un coup. Elle appela « Aurélien ! Aurélien, réponds-moi » — \
             sa voix se perdit dans les arbres. Elle se souvint alors de la promesse d'Aurélien, \
             celle qu'il n'avait jamais eu l'intention de tenir.",
        )
        .and_then(|op| op.wait());
    doc
}

/// The bottom-side preview dock. `FocusScope` with `Continue` (not `Cycle`):
/// groups the dock's tab order without trapping the keyboard inside it — the
/// same policy the outline dock uses for its tree.
pub fn search_preview_dock(dock_id: DockWidgetId) -> DockWidget {
    DockWidget::new(dock_id, lit!("Preview (stub)"), move |_id| {
        FocusScope::new(TraversalScopePolicy::Continue).child(
            Padding::symmetric(12.0, 8.0).child(
                Expand::new().child(
                    RichTextEditor::editor(scratch_doc())
                        .style(SeamlessEditorStyle)
                        .content_padding_symmetric(8.0, 8.0)
                        .v_scroll_policy(ScrollPolicy::Auto),
                ),
            ),
        )
    })
    // The bottom side is a RAIL (an activity bar), not a tab strip: the dock is
    // identified by its glyph there, so it needs no title tab and no header bar —
    // both would spend the band's scarce height on chrome instead of prose.
    .icon(crate::activity_icons::search_preview_icon)
    .show_header(false)
    .default_location(DockOpenLocation::side(DockSide::Bottom))
}
