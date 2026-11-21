// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Deliver a global search occurrence once its destination field has mounted.

use frontend::common::entities::MatchField;
use std::rc::Rc;
use teksilo::core::BindingLevel;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::widgets::rich_text::EditorHandle;

#[derive(Clone, Debug, PartialEq)]
pub struct EditorHit {
    pub field: MatchField,
    pub start: usize,
    pub matched: String,
}

pub type PendingHit = Signal<Option<EditorHit>>;

type Reveal = Rc<dyn Fn(&EditorHit, WidgetId, &mut EventContext) -> bool>;

/// Keep the editor mounted while a new request is delivered. Both synopsis
/// layouts can exist at once; only the active field may consume the request.
pub(crate) struct Destination {
    pending: PendingHit,
    field: MatchField,
    reveal: Reveal,
    after_paint: Signal<bool>,
    child: Option<Box<dyn Widget>>,
    id: Option<WidgetId>,
}

impl Destination {
    pub fn new(
        child: impl Widget + 'static,
        pending: PendingHit,
        field: MatchField,
        reveal: impl Fn(&EditorHit, WidgetId, &mut EventContext) -> bool + 'static,
    ) -> Self {
        Self {
            child: Some(Box::new(child)),
            id: None,
            pending,
            field,
            reveal: Rc::new(reveal),
            after_paint: Signal::new(false),
        }
    }

    pub fn prose(
        child: impl Widget + 'static,
        pending: PendingHit,
        field: MatchField,
        doc: &TextDocument,
        handle: Option<EditorHandle>,
    ) -> Self {
        let doc = doc.clone();
        Self::new(child, pending, field, move |hit, _, ctx| {
            let len = hit.matched.chars().count();
            let Some(end) = hit.start.checked_add(len) else {
                return true;
            };
            // Global search records document character positions. Its single-line
            // snippets display a soft line break as a space.
            if doc.text_at(hit.start, len).is_ok_and(|text| {
                text.replace(teksilo::text_document::LINE_SEPARATOR, " ") == hit.matched
            }) && let Some(handle) = &handle
            {
                handle.focus(ctx);
                handle.select_range(hit.start, end);
                if !handle.reveal_range(ctx, hit.start, end) {
                    // An editor below the fold may have mounted without ever
                    // painting. Bring it into view, then locate the range.
                    handle.reveal_widget(ctx);
                    return false;
                }
            }
            true
        })
    }
}

impl std::fmt::Debug for Destination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Destination")
            .field("field", &self.field)
            .finish_non_exhaustive()
    }
}

impl Widget for Destination {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(child) = self.child.take() {
            self.id = Some(ctx.add_boxed(child));
        }
        self.pending
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.after_paint
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        if let Some(hit) = self.pending.get()
            && hit.field == self.field
            && let Some(id) = self.id
        {
            let active = ctx.activation_signal(ctx.self_id());
            let pending = self.pending.clone();
            let reveal = self.reveal.clone();
            let after_paint = self.after_paint.clone();
            ctx.run_after_mount(move |ctx| {
                if active.get() && pending.get().as_ref() == Some(&hit) {
                    if reveal(&hit, id, ctx) || after_paint.get() {
                        pending.set(None);
                        after_paint.set(false);
                    } else {
                        after_paint.set(true);
                        ctx.request_frame();
                    }
                }
            });
        }
        self.children()
    }

    fn preserves_children_on_rebuild(&self) -> bool {
        true
    }

    fn children(&self) -> Vec<WidgetId> {
        self.id.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.id
            .and_then(|id| ctx.child_layout_response(id, proposal))
            .unwrap_or_else(|| Size::ZERO.into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _: SizeProposal,
        children: &mut [WidgetPlacement],
        _: &LayoutContext,
    ) {
        for child in children {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::{NoopWindowOps, widget_tree::WidgetTree};
    use teksilo::widgets::rich_text::RichTextEditor;

    #[test]
    fn reveals_exact_character_range_before_and_after_mount() {
        let doc = TextDocument::new();
        doc.set_plain_text("Été\nchat et chat").unwrap();
        let hit = |start| EditorHit {
            field: MatchField::Body,
            start,
            matched: "chat".into(),
        };
        let pending = Signal::new(Some(hit(12)));
        let editor = RichTextEditor::editor(doc.clone());
        let handle = editor.handle();
        let mut tree = WidgetTree::new();
        tree.add_boxed(Box::new(Destination::prose(
            editor,
            pending.clone(),
            MatchField::Body,
            &doc,
            Some(handle.clone()),
        )));
        let mut settle = || {
            for _ in 0..3 {
                tree.layout(SizeProposal::exact(400.0, 200.0));
                tree.run_mount_actions(&mut NoopWindowOps);
            }
        };
        settle();
        assert_eq!(handle.selected_text(), "chat");
        assert_eq!(handle.cursor_position(), 16);
        pending.set(Some(hit(4)));
        settle();
        assert_eq!(handle.selected_text(), "chat");
        assert_eq!(handle.cursor_position(), 8);
        pending.set(Some(hit(6)));
        settle();
        assert_eq!(handle.cursor_position(), 8, "stale hit moved the selection");
    }
}
