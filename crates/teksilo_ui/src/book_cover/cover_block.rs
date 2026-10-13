// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The cover block on a Book's own page: the cover as a thumbnail, and the two buttons
//! that change it.
//!
//! A hand-written `Widget` rather than a `teksu!` tree on its own, for the reason
//! [`crate::tabs::shared::goal_header`] is one: its content follows a signal
//! ([`BookCoverViewModel::revision`]) at the rebuild level, which a static tree cannot. The
//! tree it rebuilds is `teksu!`.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::path::Path;
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, FixedSize, GroupHeader, HStack, ImageFit, ImageWidget, TextWidget,
    VStack,
};

use super::{BookCoverViewModel, CoverState};

/// The longest side the thumbnail is shown at, in logical pixels.
pub const THUMB_EDGE: f32 = 160.0;

/// The longest side a thumbnail is decoded to, in physical pixels: twice what is shown,
/// so it stays sharp on a high-density screen without holding a camera-sized bitmap.
const THUMB_DECODE_EDGE: u32 = 320;

/// How many decoded thumbnails stay cached. A project rarely has more Books than this
/// open at once, and each is a few hundred kilobytes.
const THUMB_CACHE: usize = 8;

/// A decoded, downscaled cover, ready for [`ImageWidget::from_raw`].
#[derive(Debug)]
struct Thumb {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

thread_local! {
    /// Decoded thumbnails by content hash. A Book's tab is rebuilt often (a segment
    /// switch, a theme change), and decoding a full-resolution cover each time would be
    /// felt. Keyed by hash, the picture's identity, so a changed cover is a new entry and
    /// never a stale one.
    static THUMBS: RefCell<VecDeque<(String, Rc<Thumb>)>> = const { RefCell::new(VecDeque::new()) };
}

fn thumbnail(hash: &str, path: &Path) -> Option<Rc<Thumb>> {
    if let Some(hit) = THUMBS.with(|c| {
        c.borrow()
            .iter()
            .find(|(h, _)| h == hash)
            .map(|(_, t)| t.clone())
    }) {
        return Some(hit);
    }
    let bytes = std::fs::read(path).ok()?;
    let decoded = teksilo::canvas::RasterIcon::decode(&bytes).ok()?;
    let small = decoded.downsample_to_max(THUMB_DECODE_EDGE);
    let icon = small.as_ref().unwrap_or(&decoded);
    let thumb = Rc::new(Thumb {
        pixels: icon.pixels().to_vec(),
        width: icon.width(),
        height: icon.height(),
    });
    THUMBS.with(|c| {
        let mut c = c.borrow_mut();
        c.push_front((hash.to_string(), thumb.clone()));
        c.truncate(THUMB_CACHE);
    });
    Some(thumb)
}

/// The size a `w × h` picture is shown at, its longest side [`THUMB_EDGE`].
fn shown_size(w: u32, h: u32) -> (f32, f32) {
    let (w, h) = (w.max(1) as f32, h.max(1) as f32);
    if w >= h {
        (THUMB_EDGE, THUMB_EDGE * h / w)
    } else {
        (THUMB_EDGE * w / h, THUMB_EDGE)
    }
}

/// The cover block for one Book.
pub struct CoverBlock {
    vm: BookCoverViewModel,
    root: Option<WidgetId>,
}

impl CoverBlock {
    pub fn new(vm: BookCoverViewModel) -> Self {
        Self { vm, root: None }
    }
}

impl std::fmt::Debug for CoverBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoverBlock")
            .field("item_id", &self.vm.item_id())
            .finish()
    }
}

impl Widget for CoverBlock {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.revision().bind_to(sid, reg, BindingLevel::Rebuild);

        let state = self.vm.state();
        let has_cover = state != CoverState::None;
        let picture: Box<dyn Widget> = match &state {
            CoverState::Shown { asset, path } => match thumbnail(&asset.content_hash, path) {
                Some(t) => {
                    let (w, h) = shown_size(t.width, t.height);
                    Box::new(teksu!(FixedSize {
                        width: w
                        height: h
                        ImageWidget::from_raw(t.pixels.clone(), t.width, t.height) {
                            fit: ImageFit::Contain
                            alt: self.vm.book_title()
                        }
                    }))
                }
                // On disk but not a picture this build can decode: said, like a missing one.
                None => Box::new(note(tr!(cover_missing()))),
            },
            CoverState::Missing { .. } => Box::new(note(tr!(cover_missing()))),
            CoverState::None => Box::new(note(tr!(cover_none()))),
        };

        let choose_vm = self.vm.clone();
        let choose = Button::new(if has_cover {
            tr!(cover_change())
        } else {
            tr!(cover_pick())
        })
        .variant(ButtonVariant::Plain)
        .on_activate_fn(move |c| choose_vm.choose(c));
        // Remove only when there is something to remove.
        let remove: Option<Box<dyn Widget>> = has_cover.then(|| {
            let remove_vm = self.vm.clone();
            Box::new(
                Button::new(tr!(cover_remove()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |c| remove_vm.remove_and_report(c)),
            ) as Box<dyn Widget>
        });

        let id = teksu!(ctx => VStack {
            spacing: 6.0
            GroupHeader::new(tr!(cover_heading())) {
                style: TextStyleRole::SmallBold
                color: TextRole::Secondary
            }
            HStack {
                spacing: 12.0
                child: picture
                VStack {
                    spacing: 4.0
                    child: choose
                    child_opt: remove
                }
            }
        });
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

fn note(text: LocalizedString) -> TextWidget {
    TextWidget::new(text)
        .style(TextStyleRole::Small)
        .color(TextRole::Secondary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_thumbnail_keeps_the_pictures_proportions_within_the_edge() {
        assert_eq!(
            shown_size(1600, 2400),
            (THUMB_EDGE * 1600.0 / 2400.0, THUMB_EDGE)
        );
        assert_eq!(
            shown_size(2400, 1600),
            (THUMB_EDGE, THUMB_EDGE * 1600.0 / 2400.0)
        );
        assert_eq!(shown_size(500, 500), (THUMB_EDGE, THUMB_EDGE));
        // A degenerate size never divides by zero.
        assert_eq!(shown_size(0, 0), (THUMB_EDGE, THUMB_EDGE));
    }
}
