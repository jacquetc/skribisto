// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How big the picture is drawn and where: the viewer's zoom and pan, as plain numbers.
//!
//! Kept apart from the widget so every rule here is tested without a window: what "fit"
//! means, where the picture goes when it is smaller than the view, which point stays still
//! under the pointer while zooming, and how far a pan may go.

/// The smallest scale a zoom out reaches, relative to the picture's own pixels.
pub const MIN_SCALE: f32 = 0.05;

/// The largest scale a zoom in reaches. Past this a screenshot is a grid of squares, and the
/// pixels drawn are already the picture's own, so going further shows nothing new.
pub const MAX_SCALE: f32 = 16.0;

/// One step of the zoom buttons and keys.
pub const STEP: f32 = 1.25;

/// How the picture is scaled.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Zoom {
    /// As large as fits the view, never larger than its own pixels: a small picture is
    /// shown at its size rather than blown up into a blur.
    Fit,
    /// This many view pixels for each of the picture's.
    Scale(f32),
}

/// The zoom, and how far the picture's centre sits from the view's, in view pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewState {
    pub zoom: Zoom,
    pub pan: (f32, f32),
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            zoom: Zoom::Fit,
            pan: (0.0, 0.0),
        }
    }
}

/// The scale at which `picture` fits `view`, at most 1.
pub fn fit_scale(picture: (f32, f32), view: (f32, f32)) -> f32 {
    let (pw, ph) = (picture.0.max(1.0), picture.1.max(1.0));
    let (vw, vh) = (view.0.max(1.0), view.1.max(1.0));
    (vw / pw).min(vh / ph).min(1.0)
}

impl ViewState {
    /// The scale drawn now.
    pub fn scale(&self, picture: (f32, f32), view: (f32, f32)) -> f32 {
        match self.zoom {
            Zoom::Fit => fit_scale(picture, view),
            Zoom::Scale(s) => s,
        }
    }

    /// Where the picture is drawn, as `(x, y, width, height)` relative to the view's
    /// top-left corner.
    pub fn picture_rect(&self, picture: (f32, f32), view: (f32, f32)) -> (f32, f32, f32, f32) {
        let s = self.scale(picture, view);
        let (w, h) = (picture.0 * s, picture.1 * s);
        let (px, py) = self.clamped_pan(picture, view, self.pan);
        (
            view.0 / 2.0 + px - w / 2.0,
            view.1 / 2.0 + py - h / 2.0,
            w,
            h,
        )
    }

    /// Back to fit, centred.
    pub fn fit(&mut self) {
        *self = Self::default();
    }

    /// The picture's own size, with the view's centre on the same point of the picture.
    pub fn actual_size(&mut self, picture: (f32, f32), view: (f32, f32)) {
        self.zoom_to(1.0, (view.0 / 2.0, view.1 / 2.0), picture, view);
    }

    /// Multiply the scale by `factor`, keeping the point of the picture under `anchor` (view
    /// coordinates) where it is.
    pub fn zoom_by(
        &mut self,
        factor: f32,
        anchor: (f32, f32),
        picture: (f32, f32),
        view: (f32, f32),
    ) {
        if !factor.is_finite() || factor <= 0.0 {
            return;
        }
        let target = self.scale(picture, view) * factor;
        self.zoom_to(target, anchor, picture, view);
    }

    /// Set the scale to `target`, keeping the point under `anchor` still.
    pub fn zoom_to(
        &mut self,
        target: f32,
        anchor: (f32, f32),
        picture: (f32, f32),
        view: (f32, f32),
    ) {
        let from = self.scale(picture, view);
        let to = clamp_scale(target, picture, view);
        if !to.is_finite() || from <= 0.0 {
            return;
        }
        // The anchor relative to the view's centre, and the picture's centre relative to
        // the same. The picture point under the anchor is `(anchor - centre) / from`;
        // keeping it there at `to` moves the centre by the same ratio.
        let pan = self.clamped_pan(picture, view, self.pan);
        let ax = anchor.0 - view.0 / 2.0;
        let ay = anchor.1 - view.1 / 2.0;
        let ratio = to / from;
        let next = (ax - (ax - pan.0) * ratio, ay - (ay - pan.1) * ratio);
        self.zoom = Zoom::Scale(to);
        self.pan = self.clamped_pan(picture, view, next);
    }

    /// Move the picture by `delta` view pixels, as a drag does.
    pub fn pan_by(&mut self, delta: (f32, f32), picture: (f32, f32), view: (f32, f32)) {
        let pan = self.clamped_pan(picture, view, self.pan);
        self.pan = self.clamped_pan(picture, view, (pan.0 + delta.0, pan.1 + delta.1));
    }

    /// `pan`, limited so the picture never leaves an edge of the view uncovered while it is
    /// larger than the view on that axis, and centred on an axis where it is smaller.
    fn clamped_pan(&self, picture: (f32, f32), view: (f32, f32), pan: (f32, f32)) -> (f32, f32) {
        let s = self.scale(picture, view);
        let limit = |size: f32, room: f32, at: f32| {
            let slack = ((size * s) - room) / 2.0;
            if slack <= 0.0 || !at.is_finite() {
                0.0
            } else {
                at.clamp(-slack, slack)
            }
        };
        (
            limit(picture.0, view.0, pan.0),
            limit(picture.1, view.1, pan.1),
        )
    }
}

/// `scale` within [`MIN_SCALE`]..[`MAX_SCALE`], and never below what fits when fitting is
/// already smaller than the minimum (a huge picture in a small view can still be seen whole).
fn clamp_scale(scale: f32, picture: (f32, f32), view: (f32, f32)) -> f32 {
    let floor = MIN_SCALE.min(fit_scale(picture, view));
    if !scale.is_finite() {
        return floor;
    }
    scale.clamp(floor, MAX_SCALE)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEW: (f32, f32) = (800.0, 600.0);

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 0.01
    }

    #[test]
    fn fit_shrinks_a_large_picture_and_never_enlarges_a_small_one() {
        assert!(close(fit_scale((1600.0, 600.0), VIEW), 0.5));
        assert!(close(fit_scale((400.0, 1200.0), VIEW), 0.5));
        assert!(close(fit_scale((200.0, 100.0), VIEW), 1.0));
        // A degenerate size never divides by zero.
        assert!(fit_scale((0.0, 0.0), (0.0, 0.0)).is_finite());
    }

    #[test]
    fn a_fitted_picture_is_centred() {
        let state = ViewState::default();
        let (x, y, w, h) = state.picture_rect((1600.0, 600.0), VIEW);
        assert!(close(w, 800.0) && close(h, 300.0));
        assert!(close(x, 0.0) && close(y, 150.0));
    }

    #[test]
    fn zooming_keeps_the_point_under_the_pointer_still() {
        let picture = (1600.0, 1200.0);
        let mut state = ViewState::default();
        state.actual_size(picture, VIEW);
        let anchor = (600.0, 200.0);
        let before = state.picture_rect(picture, VIEW);
        let point = (
            (anchor.0 - before.0) / before.2,
            (anchor.1 - before.1) / before.3,
        );
        state.zoom_by(2.0, anchor, picture, VIEW);
        let after = state.picture_rect(picture, VIEW);
        let moved = (
            (anchor.0 - after.0) / after.2,
            (anchor.1 - after.1) / after.3,
        );
        assert!(
            close(point.0, moved.0) && close(point.1, moved.1),
            "{point:?} vs {moved:?}"
        );
        assert_eq!(state.zoom, Zoom::Scale(2.0));
    }

    #[test]
    fn zoom_is_clamped_both_ways() {
        let picture = (1000.0, 1000.0);
        let mut state = ViewState::default();
        for _ in 0..100 {
            state.zoom_by(STEP, (400.0, 300.0), picture, VIEW);
        }
        assert_eq!(state.zoom, Zoom::Scale(MAX_SCALE));
        for _ in 0..200 {
            state.zoom_by(1.0 / STEP, (400.0, 300.0), picture, VIEW);
        }
        assert_eq!(state.zoom, Zoom::Scale(MIN_SCALE));
        // A huge picture can still be zoomed out to fit, below the minimum.
        let huge = (100_000.0, 100.0);
        let mut state = ViewState::default();
        state.zoom_by(0.001, (0.0, 0.0), huge, VIEW);
        assert!(close(state.scale(huge, VIEW), fit_scale(huge, VIEW)));
        // Nonsense factors are ignored.
        let before = state;
        state.zoom_by(f32::NAN, (0.0, 0.0), huge, VIEW);
        state.zoom_by(-1.0, (0.0, 0.0), huge, VIEW);
        assert_eq!(state, before);
    }

    #[test]
    fn a_pan_stops_at_the_edges_and_does_nothing_on_a_smaller_axis() {
        let picture = (1600.0, 400.0);
        let mut state = ViewState::default();
        state.actual_size(picture, VIEW);
        state.pan_by((10_000.0, 10_000.0), picture, VIEW);
        let (x, y, w, _) = state.picture_rect(picture, VIEW);
        // Dragged as far right as it goes: the picture's left edge sits on the view's.
        assert!(close(x, 0.0), "x {x}");
        assert!(close(x + w, 1600.0));
        // 400 high in a 600 view: centred vertically whatever the drag.
        assert!(close(y, 100.0), "y {y}");
        state.pan_by((-100_000.0, 0.0), picture, VIEW);
        let (x, _, w, _) = state.picture_rect(picture, VIEW);
        assert!(close(x + w, VIEW.0), "right edge on the view's");
    }

    #[test]
    fn fit_resets_the_pan() {
        let picture = (1600.0, 1200.0);
        let mut state = ViewState::default();
        state.actual_size(picture, VIEW);
        state.pan_by((100.0, 100.0), picture, VIEW);
        state.fit();
        assert_eq!(state, ViewState::default());
    }
}
