// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A press on the empty page around a writing editor puts the caret in it.
//!
//! A short scene leaves most of its tab empty, and nothing on screen tells the
//! writer where the editor stops: it draws no frame. A press in the margins
//! beside the text or in the space under it used to land on a layout container
//! and do nothing. Now a press beside the text puts the caret on the line at that
//! height, at the end of the line on the side that was pressed, and a press under
//! the page puts it at the end of the text.
//!
//! **Only presses nothing else took.** The handler runs on a widget the page
//! already owns ([`CenterColumnFlowing`](super::CenterColumnFlowing), `ColumnWithMargin`, [`PageTail`]) and
//! acts only when the framework's own hit test chose that widget, which is
//! [`EventContext::dispatch_target`]. A title field, a tag chip, a comment card
//! or a scroll bar is a node of its own and wins the press over any background,
//! so none of them is overridden. Deciding from a rectangle instead would have to
//! list every one of them, and would go wrong the day one more is added.
//!
//! **A mouse on press, a finger on release.** That is how the editor's own text
//! behaves, and how the framework gives focus: a mouse focuses on press, and a
//! finger only once it lifts, because a finger that lands and slides is
//! scrolling the page, not choosing a place. So a contact only counts if it
//! lifts within its tap slop, which leaves a pan started on the margin to the
//! page.

use std::cell::Cell;
use std::rc::Rc;

use teksilo::canvas::{Point, Rect, Size};
use teksilo::core::widget::{LayoutContext, LayoutResponse, Widget, WidgetPlacement};
use teksilo::core::widget_builder::HandlerSet;
use teksilo::core::widget_id::WidgetId;
use teksilo::prelude::*;
use teksilo::text_document::TextDocument;
use teksilo::tokens::{GestureProfile, PointerKind};
use teksilo::widgets::rich_text::{EditorHandle, WeakEditorHandle};

/// The editor a press on its page puts the caret in, and the document it shows.
///
/// The document is here because the handle cannot say where its text ends: that
/// is the document's last cursor position, which counts a separator between each
/// two blocks and so is not `character_count()`.
///
/// The handle is a weak one. This is held by widgets around the editor, which a
/// rebuild may keep after the editor itself is gone, and a strong handle there
/// would keep the old editor's state alive with them.
#[derive(Clone)]
pub struct PageCaret {
    editor: WeakEditorHandle,
    doc: TextDocument,
}

impl std::fmt::Debug for PageCaret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageCaret").finish_non_exhaustive()
    }
}

/// Where in the text a press on the background goes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Aim {
    /// The line at the press's height, hit-tested at this x, in the local space
    /// of the widget that took the press.
    Line { x: f32 },
    /// The end of the text.
    End,
}

impl PageCaret {
    pub fn new(editor: &EditorHandle, doc: &TextDocument) -> Self {
        Self {
            editor: editor.downgrade(),
            doc: doc.clone(),
        }
    }

    /// Put the caret at `aim` and give the editor the keyboard.
    ///
    /// `window` is where the press landed (window space) and `local` the same
    /// point in the space of the widget that took it.
    fn place(&self, aim: Aim, window: Point, local: Point, ctx: &mut EventContext) {
        // Gone with its editor: a rebuild replaced it, and the page around the
        // new one carries the new one's caret.
        let Some(handle) = self.editor.upgrade() else {
            return;
        };
        let end = crate::shared::binder_ops::end_of(&self.doc);
        let mut at = match aim {
            Aim::Line { x } => Point::new(window.x - local.x + x, window.y),
            Aim::End => Point::new(window.x, f32::INFINITY),
        };
        // Beside the editor's top padding is beside its first line. The margin
        // spans the editor's whole height, not only its text's.
        if let Some(first) = handle.offset_rect(0)
            && at.y < first.y
        {
            at.y = first.y + first.height / 2.0;
        }
        let below_the_text = handle
            .offset_rect(end)
            .is_none_or(|last| at.y >= last.bottom());
        if below_the_text {
            handle.select_range(end, end);
        } else {
            place_as_a_click(&handle, at);
        }
        handle.focus(ctx);
    }
}

/// Put the caret where a click at `at` (window space, at the edge of the text)
/// would put it, as the editor's own pointer code does.
///
/// **Not `offset_at_point` and `select_range`.** Where a paragraph wraps, each
/// line keeps its trailing space, so the end of one line and the start of the
/// next are the same position in the text, and only the caret's affinity says
/// which of the two lines it is drawn on. The hit test answers that affinity
/// along with the position (the start of the lower line, for a point left of
/// it), and `offset_at_point` keeps only the position; `select_range` then
/// sets the default affinity, which draws the caret at the end of the line
/// above. A press left of a paragraph's second line put the caret at the end of
/// its first.
///
/// `reposition_caret_for_context_menu` keeps both. It leaves a selection alone
/// when the point is inside it, which a click does not, so the selection is
/// collapsed first.
fn place_as_a_click(handle: &EditorHandle, at: Point) {
    let caret = handle.cursor_position();
    handle.select_range(caret, caret);
    handle.reposition_caret_for_context_menu(at);
}

/// A contact that has landed on the background and not lifted yet.
#[derive(Clone, Copy)]
struct Contact {
    pointer: teksilo::core::pointer::PointerId,
    kind: PointerKind,
    window: Point,
    local: Point,
}

/// How far a contact may travel and still be a tap: the framework's own slop for
/// its kind, so this agrees with every button on the page.
fn tap_slop(kind: PointerKind) -> f32 {
    match kind {
        PointerKind::Touch => GestureProfile::TOUCH.tap_slop,
        PointerKind::Pen(_) => GestureProfile::PEN.tap_slop,
        // The mouse's, as the framework maps an unclassified device.
        _ => GestureProfile::MOUSE.tap_slop,
    }
}

/// The pointer handler that puts the caret in `caret()`'s editor when a primary
/// press lands on a background `owns` accepts.
///
/// Installed on the background's own node, so it also runs, in the preview pass,
/// for every press on the widgets inside it. `owns` is what tells the two apart:
/// it is asked about the framework's hit target, and answers for the node itself
/// (and for a child that is background too, such as an empty comment margin).
///
/// `aim` turns the press's local position into the place in the text. It always
/// answers `Ignored`: the press goes on to whatever else wants it, which for a
/// finger includes the page's own pan.
pub(crate) fn caret_on_press(
    caret: impl Fn() -> Option<PageCaret> + 'static,
    owns: impl Fn(WidgetId) -> bool + 'static,
    aim: impl Fn(Point) -> Aim + 'static,
) -> impl FnMut(&WidgetEvent, &mut EventContext) -> EventResponse + 'static {
    let mut contact: Option<Contact> = None;
    move |event, ctx| {
        match event {
            WidgetEvent::PointerDown {
                position, button, ..
            } => {
                contact = None;
                if *button != PointerButton::Primary || !ctx.dispatch_target().is_some_and(&owns) {
                    return EventResponse::Ignored;
                }
                let Some(window) = ctx.pointer_position() else {
                    return EventResponse::Ignored;
                };
                let kind = ctx.pointer_kind();
                if kind.is_direct() {
                    contact = Some(Contact {
                        pointer: ctx.pointer().id,
                        kind,
                        window,
                        local: *position,
                    });
                } else if let Some(caret) = caret() {
                    caret.place(aim(*position), window, *position, ctx);
                }
            }
            WidgetEvent::PointerUp { .. } => {
                let Some(down) = contact.take() else {
                    return EventResponse::Ignored;
                };
                let lifted = ctx.pointer_position();
                let tapped = ctx.pointer().id == down.pointer
                    && lifted.is_some_and(|at| {
                        let (dx, dy) = (at.x - down.window.x, at.y - down.window.y);
                        (dx * dx + dy * dy).sqrt() <= tap_slop(down.kind)
                    });
                if tapped && let Some(caret) = caret() {
                    // Where the contact landed, not where it lifted: the slop is
                    // a tolerance, and the writer aimed with the landing.
                    caret.place(aim(down.local), down.window, down.local, ctx);
                }
            }
            WidgetEvent::PointerCancel { .. } => contact = None,
            _ => {}
        }
        EventResponse::Ignored
    }
}

/// The aim for a press beside a column placed at `left`, `width` wide, in the
/// local space of the widget around it: just inside the column's edge on the side
/// that was pressed, so the hit test lands on that end of the line.
pub(crate) fn beside(left: f32, width: f32, local: Point) -> Aim {
    // One pixel in from the edge, inside the editor's content padding: the hit
    // test clamps a point beside a line to that line's end.
    let x = if local.x < left {
        left + 1.0
    } else if local.x >= left + width {
        left + width - 1.0
    } else {
        local.x
    };
    Aim::Line { x }
}

/// The page under a writing surface's last editor: a press there puts the caret
/// at the end of the text.
///
/// Wraps the content of the page's `ScrollArea`, which is laid out with
/// `widget_resizable` so this is placed at least as tall as the viewport. It
/// places its child at the child's own height, at the top, so everything under
/// the content is this widget's own area and a press there lands on it.
///
/// The editor is read from `caret` at the press, not at build: the page's editor
/// publishes itself into the slot as it is constructed, and a rebuild replaces it.
pub struct PageTail {
    child: Option<WidgetId>,
    pending: Option<Box<dyn Widget>>,
    caret: Rc<std::cell::RefCell<Option<PageCaret>>>,
    bottom_space: f32,
}

impl std::fmt::Debug for PageTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PageTail").finish_non_exhaustive()
    }
}

impl PageTail {
    pub fn new(
        caret: Rc<std::cell::RefCell<Option<PageCaret>>>,
        child: impl Widget + 'static,
    ) -> Self {
        Self {
            child: None,
            pending: Some(Box::new(child)),
            caret,
            bottom_space: 0.0,
        }
    }

    /// Keep `height` of empty page under the content, which scrolls with it and
    /// takes presses like the rest of the space under it.
    pub fn bottom_space(mut self, height: f32) -> Self {
        self.bottom_space = height;
        self
    }
}

impl Widget for PageTail {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        if let Some(w) = self.pending.take() {
            self.child = Some(ctx.add_boxed(w));
        }
        let me = ctx.self_id();
        let caret = self.caret.clone();
        ctx.apply_self_handlers(HandlerSet::new().on_pointer_event(caret_on_press(
            move || caret.borrow().clone(),
            move |target| target == me,
            |_| Aim::End,
        )));
        self.children()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        let child = self
            .child
            .and_then(|id| ctx.child_size(id, proposal))
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0));
        Size::new(child.width, child.height + self.bottom_space).into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        for child in children.iter_mut() {
            let height = ctx
                .child_size(child.id, proposal)
                .map_or(bounds.height, |s| s.height);
            child.origin = bounds.origin();
            child.size = Size::new(bounds.width, height.min(bounds.height));
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.child.into_iter().collect()
    }
}

/// The column's place inside the widget centring it, for [`beside`]: its left
/// edge and its width, in that widget's local space. Written by its layout and
/// read by its press handler.
pub(crate) type ColumnSpan = Rc<Cell<(f32, f32)>>;
