// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The **Corkboard** pane — a container's contents as a grid of synopsis cards.
//!
//! One segment of a Book / Part / Chapter-folder tab. A header (breadcrumb ·
//! count · nested/flat · ＋New, over filter · order · card-size) sits above a
//! virtualized [`GridView`] of cards. The grid binds the raw backend-driven model
//! in natural order (drag-reorder + drag-out enabled) and swaps to the model's
//! filter projection while a search **or sort** is active (reorder inert — you
//! don't drag a projected view). Card size is a live slider bound to `GridView`'s
//! reactive `.sizing`.
//!
//! A **container** card additionally wraps its tile in a [`DropTarget`], so cards
//! dropped onto it are re-parented *into* it — `GridView`'s own drop targeting
//! only ever yields `Before`/`After` (`Into` is trees-only), and a payload the
//! target rejects bubbles straight back to the grid's ordinary reorder.
//!
//! Each card's cheap fields (title/type/label) come from the model; the synopsis
//! excerpt and word count are resolved lazily per **visible** tile by
//! [`SingleCorkboardCard`], so a long board stays as cheap as one viewport.

use std::rc::Rc;

use bastyde::canvas::{EdgeInsets, Rect};
use bastyde::core::BindingLevel;
use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::core::styles::PanelVariant;
use bastyde::core::widget::WidgetPlacement;
use bastyde::data::{ListDataSource, SortDirection};
use bastyde::i18n::LocalizedString;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::{
    Badge, Breadcrumb, BreadcrumbItem, ButtonVariant, Center, DeadZone, Divider, DragTransferMode,
    DropTarget, Expand, FixedSize, GridSizing, GridView, HStack, IconButton, IconWidget, MenuItem,
    MenuList, Padding, Panel, PopoverIconButton, RowDragData, SearchField, Segment,
    SegmentedControl, Slider, Spacer, SplitButton, Switcher, TextInput, TextWidget, TileContext,
    Toast, VStack,
};

// `WidgetEvent`, `Key`, `PointerButton`, `EventResponse` and the `WidgetBuilder`
// gesture/key hooks all come from `bastyde::prelude::*` above.

use frontend::AppContext;
use frontend::common::entities::BinderItemSubRole;

use crate::binder::create_labels::{
    recommendation_label, recommendation_placement, recommendation_tooltip_key,
};
use crate::models::{CorkboardCard, OpenDoc};
use crate::singles::SingleCorkboardCard;
use crate::toast_scope::ToastWorkExt;
use crate::view_models::{CorkboardViewModel, EditorTypography};

mod card;
mod chrome;
mod grid;
/// The fixed "＋" glyph for the create button's main region.
mod header;
mod move_target;
mod synopsis;

use card::*;
use chrome::*;
use grid::*;
#[allow(unused_imports)]
use header::*;
use synopsis::*;

pub(super) fn add_icon() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/add.svg")).icon_size(14.0)
}

/// Total vertical padding a card's `Padding::uniform(12)` removes from the tile
/// height before [`CardColumn`] lays out the header / synopsis / footer.
pub(super) const CARD_PADDING: f32 = 24.0;

/// The pixel height of a card at slider width `w` — index-card proportion (wider
/// than tall), floored so a small card still fits its header + a line or two +
/// footer. Shared by the grid sizing and the per-card synopsis height.
pub(super) fn card_tile_height(w: f32) -> f32 {
    (w * 0.72).max(146.0)
}

/// Map the card-size slider (a minimum tile width) to an adaptive grid sizing —
/// as many ≥ `w`-wide columns as fit, stretched, with a card-shaped height.
pub(super) fn sizing_for(w: f32) -> GridSizing {
    GridSizing::Adaptive {
        min_width: w,
        max_width: Some(w * 1.6),
        height: card_tile_height(w),
    }
}

/// The pane: a wiring child, the header, and the grid filling the rest.
pub fn corkboard_pane(tab: &super::ContentTab) -> Box<dyn Widget> {
    let Some(vm) = tab.corkboard().cloned() else {
        // Non-container tabs never reach here (the segment only exists for them),
        // but keep the switcher total.
        return Box::new(VStack::new());
    };
    Box::new(
        VStack::new()
            .spacing(0.0)
            .child(WireCorkboard { vm: vm.clone() })
            .child(corkboard_header(&vm))
            .child(Expand::new().child(CorkboardGrid { vm, root: None })),
    )
}

/// Zero-size child that wires the view-model (subscribes model + probe + the
/// search/sort plumbing) on build. `wire` is idempotent per build.
pub(crate) struct WireCorkboard {
    pub(crate) vm: CorkboardViewModel,
}
impl std::fmt::Debug for WireCorkboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WireCorkboard").finish()
    }
}
impl Drop for WireCorkboard {
    fn drop(&mut self) {
        // The pane is being torn down (segment switch / tab close): flush + release
        // every synopsis doc the board held open, so no card edit is stranded.
        // `enter()` already covers drill in/out; this covers leaving the board.
        self.vm.release_all_synopses();
    }
}
impl Widget for WireCorkboard {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.vm.wire(ctx);
        Vec::new()
    }
    fn layout_response(&self, _p: SizeProposal, _c: &LayoutContext) -> LayoutResponse {
        Size::new(0.0, 0.0).into()
    }
}
