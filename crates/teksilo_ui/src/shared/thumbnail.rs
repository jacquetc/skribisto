// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A picture's thumbnail in a waterfall grid: the Gallery's tiles and a folder's
//! Resources grid.
//!
//! The thumbnails are decoded off the interface thread and cached
//! ([`crate::shared::picture_cache`]); a tile shows a placeholder of its final size until
//! its picture arrives, laid out from the size the project stores, so the columns do not
//! move when it does.

use std::path::PathBuf;

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::{Center, ImageFit, ImageWidget, Panel, TextWidget};

use crate::shared::images::ImageRef;
use crate::shared::picture_cache::{self, PictureCache, PictureKey, PictureState};

/// The thumbnail edge, in physical pixels, for tiles `size` logical pixels wide: twice the
/// width so a picture stays sharp on a high-density screen, in steps so dragging the slider
/// asks for a few sizes rather than one per pixel.
pub fn thumb_edge(size: f32) -> u32 {
    let want = (size.max(1.0) * 2.0).ceil() as u32;
    [256, 512, picture_cache::MAX_THUMB_EDGE]
        .into_iter()
        .find(|&edge| edge >= want)
        .unwrap_or(picture_cache::MAX_THUMB_EDGE)
}

/// A picture's width over its height, from its stored size, kept within 1:4 and 4:1 so a
/// sliver of a screenshot is neither an endless column nor a line of pixels. With no
/// size known, 4:3.
pub fn tile_ratio(size: Option<(u32, u32)>) -> f32 {
    let (w, h) = size.unwrap_or((4, 3));
    let ratio = w.max(1) as f32 / h.max(1) as f32;
    if ratio.is_finite() {
        ratio.clamp(0.25, 4.0)
    } else {
        4.0 / 3.0
    }
}

/// A thumbnail: its picture once decoded, a quiet placeholder of the same size until then,
/// and the same placeholder with a word when the picture cannot be shown.
pub struct Thumb {
    /// `None` for a reference outside the project's pictures, which is never read.
    path: Option<PathBuf>,
    /// The cache key's hash, `None` when there is nothing to key on.
    hash: Option<String>,
    alt: String,
    rt: Option<AsyncRuntimeHandle>,
    edge: u32,
    /// Draw the picture on the placeholder's background: a folder preview fits a picture
    /// of any shape into a square cell, and the cell should still read as one.
    backdrop: bool,
    root: Option<WidgetId>,
}

impl Thumb {
    /// A picture of a text, which its reference names under `media_dir`.
    pub fn of_picture(
        media_dir: &std::path::Path,
        image: &ImageRef,
        rt: Option<AsyncRuntimeHandle>,
        edge: u32,
    ) -> Self {
        Self {
            path: picture_cache::asset_path(media_dir, &image.src),
            hash: picture_cache::asset_hash(&image.src).map(str::to_string),
            alt: image.alt.clone(),
            rt,
            edge,
            backdrop: false,
            root: None,
        }
    }

    /// A file named by its path and the content hash the project records for it: a
    /// picture resource. A file with no such hash (a linked file) is keyed by its place
    /// instead, and a recorded hash of any other shape is never used as a file name (see
    /// [`picture_cache::file_key_hash`]).
    pub fn of_file(
        path: Option<PathBuf>,
        hash: &str,
        alt: &str,
        rt: Option<AsyncRuntimeHandle>,
        edge: u32,
    ) -> Self {
        Self {
            hash: path
                .as_deref()
                .and_then(|p| picture_cache::file_key_hash(p, hash)),
            path,
            alt: alt.to_string(),
            rt,
            edge,
            backdrop: false,
            root: None,
        }
    }

    /// Draw on the placeholder's background (see the field's docs).
    pub fn backdrop(mut self, on: bool) -> Self {
        self.backdrop = on;
        self
    }
}

impl std::fmt::Debug for Thumb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Thumb").field("path", &self.path).finish()
    }
}

impl Widget for Thumb {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let state = match (self.path.clone(), self.hash.clone()) {
            (Some(path), Some(hash)) => {
                let key = PictureKey {
                    hash,
                    edge: self.edge,
                };
                let slot = PictureCache::shared().request(self.rt.as_ref(), path, key);
                slot.bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
                slot.get()
            }
            _ => PictureState::Failed,
        };
        let id = match state {
            PictureState::Ready(decoded) => {
                let picture =
                    ImageWidget::from_raw(decoded.pixels.clone(), decoded.width, decoded.height)
                        .fit(ImageFit::Contain)
                        .alt(self.alt.clone());
                if self.backdrop {
                    ctx.add(Panel::new().background(SurfaceRole::Sunken).child(picture))
                } else {
                    ctx.add(picture)
                }
            }
            PictureState::Failed => ctx.add(
                Panel::new().background(SurfaceRole::Sunken).child(
                    Center::new().child(
                        TextWidget::new(tr!(gallery_unavailable()))
                            .style(TextStyleRole::Tiny)
                            .color(TextRole::Secondary),
                    ),
                ),
            ),
            PictureState::Loading | PictureState::Idle => {
                ctx.add(Panel::new().background(SurfaceRole::Sunken))
            }
        };
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // The cell is the aspect box around it: the picture, or its placeholder, fills it.
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_thumbnail_is_twice_the_tile_in_a_few_steps() {
        assert_eq!(thumb_edge(100.0), 256);
        assert_eq!(thumb_edge(128.0), 256);
        assert_eq!(thumb_edge(200.0), 512);
        assert_eq!(thumb_edge(400.0), 1024);
        assert_eq!(thumb_edge(5000.0), picture_cache::MAX_THUMB_EDGE);
        assert_eq!(thumb_edge(f32::NAN), 256);
    }

    /// A picture resource is keyed safely whatever its recorded hash: its own content
    /// hash, a key from its place when it has none, never a crafted one.
    #[test]
    fn a_file_thumbnail_is_keyed_by_a_content_hash_or_by_its_place() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let path = tmp.path().join("linked.png");
        std::fs::write(&path, b"bytes").unwrap_or_else(|e| panic!("write: {e}"));
        let hash = "f".repeat(64);
        let recorded = Thumb::of_file(Some(path.clone()), &hash, "", None, 256);
        assert_eq!(recorded.hash.as_deref(), Some(hash.as_str()));
        let linked = Thumb::of_file(Some(path.clone()), "", "", None, 256);
        let crafted = Thumb::of_file(Some(path.clone()), "../../escape", "", None, 256);
        let place = picture_cache::file_key_hash(&path, "");
        assert!(place.is_some());
        assert_eq!(linked.hash, place);
        assert_eq!(crafted.hash, place);
        assert_eq!(Thumb::of_file(None, &hash, "", None, 256).hash, None);
    }

    #[test]
    fn a_tile_keeps_its_picture_proportions_within_bounds() {
        assert!((tile_ratio(Some((1600, 900))) - 16.0 / 9.0).abs() < 1e-4);
        assert_eq!(tile_ratio(Some((10, 10_000))), 0.25);
        assert_eq!(tile_ratio(Some((10_000, 10))), 4.0);
        assert_eq!(tile_ratio(Some((0, 0))), 1.0);
        assert!((tile_ratio(None) - 4.0 / 3.0).abs() < 1e-4);
    }
}
