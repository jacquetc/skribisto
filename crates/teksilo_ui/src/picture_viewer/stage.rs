// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The zoomable, pannable picture: one decoded picture drawn at the [`ViewState`]'s scale
//! and offset, clipped to the space it is given.
//!
//! The picture is decoded once and never again while the viewer shows it: zooming changes
//! only the rectangle its pixels are drawn into, so it is a relayout, not a rebuild, and a
//! pinch or a held key costs no decode and no upload.
//!
//! Its input, all of it local to this widget:
//! - a drag pans;
//! - the wheel pans, and Ctrl (⌘ on a Mac) with the wheel zooms around the pointer, which
//!   this widget takes for itself so the editor's own Ctrl+wheel text size, behind the
//!   viewer, never sees it;
//! - a pinch, from a trackpad or a touch screen, zooms around its centre;
//! - a double click switches between fit and the picture's own size at that point.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use teksilo::canvas::Point;
use teksilo::core::BindingLevel;
use teksilo::core::event::ScrollDelta;
use teksilo::core::gesture::{DragPhase, PinchPhase};
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::widgets::{ImageFit, ImageWidget};

use super::zoom::{STEP, ViewState, Zoom};
use crate::shared::picture_cache::Decoded;

/// Pixels a wheel line pans by.
const WHEEL_LINE: f32 = 48.0;

/// What the stage and the panel around it share: the view, the space it was last laid out
/// in, and the signal that asks for a relayout when either changes the view.
#[derive(Debug)]
pub struct StageState {
    view: Cell<ViewState>,
    viewport: Cell<(f32, f32)>,
    picture: Cell<(f32, f32)>,
    /// Which picture the view belongs to, so a rebuild showing the same one (a theme
    /// change, the picture arriving from the decoder) keeps its zoom.
    showing: RefCell<String>,
    revision: Signal<u64>,
}

impl StageState {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            view: Cell::new(ViewState::default()),
            viewport: Cell::new((1.0, 1.0)),
            picture: Cell::new((1.0, 1.0)),
            showing: RefCell::new(String::new()),
            revision: Signal::new(0),
        })
    }

    /// The view now.
    pub fn view(&self) -> ViewState {
        self.view.get()
    }

    /// Show the picture `id`, `width` × `height`: fit and centred when it is a different
    /// picture from the one shown, as it was when it is the same.
    pub fn show(&self, id: &str, width: u32, height: u32) {
        let size = (width.max(1) as f32, height.max(1) as f32);
        let same = *self.showing.borrow() == id && self.picture.get() == size;
        if same {
            return;
        }
        *self.showing.borrow_mut() = id.to_string();
        self.picture.set(size);
        self.view.set(ViewState::default());
        self.bump();
    }

    pub fn fit(&self) {
        self.edit(|v, _, _| v.fit());
    }

    pub fn actual_size(&self) {
        self.edit(|v, p, w| v.actual_size(p, w));
    }

    pub fn zoom_in(&self) {
        self.edit(|v, p, w| v.zoom_by(STEP, (w.0 / 2.0, w.1 / 2.0), p, w));
    }

    pub fn zoom_out(&self) {
        self.edit(|v, p, w| v.zoom_by(1.0 / STEP, (w.0 / 2.0, w.1 / 2.0), p, w));
    }

    fn zoom_at(&self, factor: f32, anchor: (f32, f32)) {
        self.edit(|v, p, w| v.zoom_by(factor, anchor, p, w));
    }

    fn pan(&self, delta: (f32, f32)) {
        self.edit(|v, p, w| v.pan_by(delta, p, w));
    }

    /// Between fit and the picture's own size, keeping `at` still.
    fn toggle_at(&self, at: (f32, f32)) {
        self.edit(|v, p, w| {
            if v.zoom == Zoom::Fit {
                v.zoom_to(1.0, at, p, w);
            } else {
                v.fit();
            }
        });
    }

    fn edit(&self, f: impl FnOnce(&mut ViewState, (f32, f32), (f32, f32))) {
        let mut view = self.view.get();
        f(&mut view, self.picture.get(), self.viewport.get());
        if view != self.view.get() {
            self.view.set(view);
            self.bump();
        }
    }

    fn bump(&self) {
        self.revision.set(self.revision.get().wrapping_add(1));
    }
}

/// The widget. Build it with [`stage`], which attaches the input handlers.
pub struct Stage {
    state: Rc<StageState>,
    picture: Rc<Decoded>,
    alt: String,
    image: Option<WidgetId>,
}

impl std::fmt::Debug for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stage")
            .field("picture", &(self.picture.width, self.picture.height))
            .field("view", &self.state.view())
            .finish()
    }
}

/// The stage for `picture`, described by `alt`, with its pointer input wired to `state`.
pub fn stage(state: Rc<StageState>, picture: Rc<Decoded>, alt: String) -> impl Widget {
    let drag = state.clone();
    let wheel = state.clone();
    let pinch = state.clone();
    let double = state.clone();
    Stage {
        state,
        picture,
        alt,
        image: None,
    }
    .cursor(CursorIcon::Grab)
    .on_drag(move |phase, _ctx| {
        if let DragPhase::Moved { delta, .. } = phase {
            drag.pan((delta.x, delta.y));
        }
    })
    .on_scroll(move |event, ctx| {
        let WidgetEvent::Scroll {
            delta,
            modifiers,
            window_position,
            ..
        } = event
        else {
            return EventResponse::Ignored;
        };
        let (dx, dy) = match *delta {
            ScrollDelta::Lines { x, y } => (x * WHEEL_LINE, y * WHEEL_LINE),
            ScrollDelta::Pixels { x, y } => (x, y),
        };
        if modifiers.command() {
            let view = wheel.viewport.get();
            let anchor = window_position
                .and_then(|p| ctx.to_local(p))
                .map(|p| (p.x, p.y))
                .unwrap_or((view.0 / 2.0, view.1 / 2.0));
            // One wheel notch up is one step in; a trackpad's small pixel deltas give a
            // proportionally small zoom, so the gesture feels continuous.
            let notches = if matches!(delta, ScrollDelta::Lines { .. }) {
                dy
            } else {
                dy / WHEEL_LINE
            };
            wheel.zoom_at(STEP.powf(notches), anchor);
        } else {
            wheel.pan((dx, dy));
        }
        EventResponse::Handled
    })
    .on_pinch(move |phase, _ctx| {
        if let PinchPhase::Changed { center, scale, .. } = phase {
            pinch.zoom_at(scale, (center.x, center.y));
        }
    })
    .on_double_tap(move |tap, _ctx| {
        double.toggle_at((tap.position.x, tap.position.y));
    })
}

impl Widget for Stage {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.state.revision.bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Relayout,
        );
        let id = ctx.add(
            ImageWidget::from_raw(
                self.picture.pixels.clone(),
                self.picture.width,
                self.picture.height,
            )
            .fit(ImageFit::Fill)
            .alt(self.alt.clone()),
        );
        self.image = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        LayoutResponse::flexible(proposal.resolve(400.0, 300.0), 1.0)
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        let view = (bounds.width.max(1.0), bounds.height.max(1.0));
        self.state.viewport.set(view);
        let (x, y, w, h) = self
            .state
            .view()
            .picture_rect(self.state.picture.get(), view);
        for child in children.iter_mut() {
            child.origin = Point::new(bounds.x + x, bounds.y + y);
            child.size = Size::new(w.max(1.0), h.max(1.0));
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.image.into_iter().collect()
    }

    fn clips_children(&self) -> bool {
        true
    }
}
