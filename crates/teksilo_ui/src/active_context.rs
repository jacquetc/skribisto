// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! [`ActiveContext`] — "what is the writer looking at, in **this** window, right
//! now."
//!
//! Tier 3 (per window): built once per window in `app/project_shell.rs` from that
//! window's own `EditorsViewModel`/`OutlineViewModel`, never through
//! `ctx.app_state` — a second window on the same Work has its own focus, and the
//! app-state slot is seeded from whichever window happened to be built first.
//!
//! Added to [`DockContext`](crate::docks::DockContext) **only**. A
//! [`ContentTab`](crate::tabs::ContentTab) or an
//! [`AnalysisViewModel`](crate::analysis::AnalysisViewModel) is already scoped
//! to one container by construction (`item_id()` / `scope_item_id()`), so it has
//! no "which container am I" question for this to answer. A dock, which sits
//! outside every tab and outlives all of them, does.
//!
//! ## Only types this module owns cross the seam
//!
//! [`ActivePane`] rather than `editors::Side`, and `HashSet<Uuid>` rather than
//! `models::BinderTreeKey`. Every type on a seam context is a compatibility
//! promise: publishing an internal enum makes renaming it a breaking change for
//! everything installed. Two variants and a uuid set are cheaper to own than that
//! promise is to keep.
//!
//! Everything is published as a [`ReadSignal`] for the reason that module gives —
//! `Signal::set` is `pub` and clones share state, so a raw signal here would let
//! an extension move the writer's focus.

use std::collections::HashSet;
use std::rc::Rc;

use teksilo::core::ObserverHandle;
use teksilo::prelude::Signal;
use uuid::Uuid;

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

use crate::editors::Side;
use crate::models::BinderTreeKey;
use crate::read_signal::ReadSignal;

/// The `BinderItem` the focused editor pane is showing.
///
/// Carries the session-local `id` (what a backend command takes) **and** the
/// durable `uid` (what anything persisted must key on — `EntityId` is re-minted
/// by every `load_work`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActiveItem {
    pub id: u64,
    pub uid: Uuid,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
}

/// Which half of a split editor has focus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivePane {
    Primary,
    Secondary,
}

impl From<Side> for ActivePane {
    fn from(side: Side) -> Self {
        match side {
            Side::Primary => ActivePane::Primary,
            Side::Secondary => ActivePane::Secondary,
        }
    }
}

struct Inner {
    active_item: Signal<Option<ActiveItem>>,
    active_pane: Signal<ActivePane>,
    outline_selection: Signal<HashSet<Uuid>>,
    /// Keeps the projections above in step with the window's own view-models.
    ///
    /// They are bridged by observation rather than derived with `Signal::map`
    /// because a derived signal cannot be observed, and
    /// [`ReadSignal::on_change`] — the whole point of handing these over — is an
    /// observation. Held here so the bridges live exactly as long as the context
    /// an extension is reading through.
    _bridges: Vec<ObserverHandle>,
}

/// The focused item, the focused pane, and the outline selection — read-only,
/// live, and scoped to one window.
#[derive(Clone)]
pub struct ActiveContext {
    inner: Rc<Inner>,
}

impl ActiveContext {
    /// Bridge this window's view-model signals into the seam's own types.
    ///
    /// Crate-internal, and deliberately so: its parameters are the app's internal
    /// types (`Side`, `BinderTreeKey`). Only the accessors below are a promise to
    /// anything installed.
    pub(crate) fn new(
        active_item: &Signal<Option<ActiveItem>>,
        focused_side: &Signal<Side>,
        outline_selection: &Signal<HashSet<BinderTreeKey>>,
    ) -> Self {
        let item = Signal::new(active_item.get());
        let pane = Signal::new(ActivePane::from(focused_side.get()));
        let selection = Signal::new(uids_of(&outline_selection.get()));

        let mut bridges = Vec::with_capacity(3);
        bridges.push({
            let out = item.clone();
            active_item.observe(move |v| out.set(v.clone()))
        });
        bridges.push({
            let out = pane.clone();
            focused_side.observe(move |v| out.set(ActivePane::from(*v)))
        });
        bridges.push({
            let out = selection.clone();
            outline_selection.observe(move |keys| out.set(uids_of(keys)))
        });

        Self {
            inner: Rc::new(Inner {
                active_item: item,
                active_pane: pane,
                outline_selection: selection,
                _bridges: bridges,
            }),
        }
    }

    /// The context for **this window**, read off its own view-models.
    ///
    /// The one call the production site makes, so which signals get bridged is
    /// decided here rather than at the call site. That is the point: the failure
    /// this shape rules out is a refactor passing a *fresh* `Signal` instead of
    /// the live one — which compiles, lays out, and leaves every dock's focus
    /// tracking permanently stuck on its initial value, with nothing to see.
    pub(crate) fn for_window(
        editors: &crate::editors::EditorsViewModel,
        outline: &crate::binder::OutlineViewModel,
    ) -> Self {
        Self::new(
            &editors.active_context(),
            &editors.focused_side_signal(),
            &outline.selection_signal(),
        )
    }

    /// A context wired to nothing — for the standalone/test construction sites
    /// that have no window behind them. Its signals never change.
    pub(crate) fn detached() -> Self {
        Self {
            inner: Rc::new(Inner {
                active_item: Signal::new(None),
                active_pane: Signal::new(ActivePane::Primary),
                outline_selection: Signal::new(HashSet::new()),
                _bridges: Vec::new(),
            }),
        }
    }

    /// The `BinderItem` the focused pane is showing — `None` when no tab is open.
    ///
    /// `role`/`sub_role` are re-resolved whenever the item is retyped in place
    /// (Promote rewrites a `sub_role` with no focus change at all), not only when
    /// focus moves; see `EditorsViewModel::sync_active_item`'s caller in `app.rs`.
    pub fn active_item(&self) -> ReadSignal<Option<ActiveItem>> {
        ReadSignal::new(self.inner.active_item.clone())
    }

    /// Which half of the split editor has focus.
    pub fn active_pane(&self) -> ReadSignal<ActivePane> {
        ReadSignal::new(self.inner.active_pane.clone())
    }

    /// The outline's selected rows, as **durable uids**. Binder rows are excluded
    /// — a `BinderTreeKey` is tagged because a binder and an item could carry the
    /// same uuid without being the same thing, and an extension keying by uid
    /// alone could not tell them apart.
    pub fn outline_selection(&self) -> ReadSignal<HashSet<Uuid>> {
        ReadSignal::new(self.inner.outline_selection.clone())
    }
}

fn uids_of(keys: &HashSet<BinderTreeKey>) -> HashSet<Uuid> {
    keys.iter()
        .filter_map(|k| match k {
            BinderTreeKey::Item(uid) => Some(*uid),
            BinderTreeKey::Binder(_) => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: u64) -> ActiveItem {
        ActiveItem {
            id,
            uid: Uuid::from_u128(id as u128),
            role: BinderItemRole::Item,
            sub_role: BinderItemSubRole::Scene,
        }
    }

    /// The context is a live *view* of the window's own state, not a snapshot
    /// taken when the dock was built — the same failure class as
    /// `WorkInfo.file_name` arriving after the first build.
    #[test]
    fn every_projection_tracks_its_source() {
        let src_item = Signal::new(None);
        let src_side = Signal::new(Side::Primary);
        let src_sel = Signal::new(HashSet::new());
        let cx = ActiveContext::new(&src_item, &src_side, &src_sel);

        assert_eq!(cx.active_item().get(), None);
        assert_eq!(cx.active_pane().get(), ActivePane::Primary);

        src_item.set(Some(item(7)));
        src_side.set(Side::Secondary);
        src_sel.set(HashSet::from([BinderTreeKey::Item(Uuid::from_u128(9))]));

        assert_eq!(cx.active_item().get().map(|i| i.id), Some(7));
        assert_eq!(cx.active_pane().get(), ActivePane::Secondary);
        assert_eq!(
            cx.outline_selection().get(),
            HashSet::from([Uuid::from_u128(9)])
        );
    }

    /// Binder rows carry uuids too, and a tagged key is what tells them from an
    /// item's. Flattening both into one uid set would hand an extension a uid it
    /// would resolve against the wrong entity kind.
    #[test]
    fn only_item_rows_reach_the_selection() {
        let src_sel = Signal::new(HashSet::from([
            BinderTreeKey::Item(Uuid::from_u128(1)),
            BinderTreeKey::Binder(Uuid::from_u128(2)),
        ]));
        let cx = ActiveContext::new(&Signal::new(None), &Signal::new(Side::Primary), &src_sel);
        assert_eq!(
            cx.outline_selection().get(),
            HashSet::from([Uuid::from_u128(1)]),
            "a binder row's uid must not be published as an item's"
        );
    }

    /// Cloning the context must not fork its state — a dock holding a clone and
    /// the window holding the original must agree.
    #[test]
    fn clones_share_one_live_state() {
        let src_item = Signal::new(None);
        let a = ActiveContext::new(
            &src_item,
            &Signal::new(Side::Primary),
            &Signal::new(HashSet::new()),
        );
        let b = a.clone();
        src_item.set(Some(item(3)));
        assert_eq!(b.active_item().get().map(|i| i.id), Some(3));
    }

    /// Nothing handed over may be written back.
    #[test]
    fn an_extension_cannot_move_the_writers_focus() {
        let cx = ActiveContext::detached();
        assert!(cx.active_item().signal().try_set(Some(item(1))).is_err());
        assert!(
            cx.active_pane()
                .signal()
                .try_set(ActivePane::Secondary)
                .is_err()
        );
        assert!(
            cx.outline_selection()
                .signal()
                .try_set(HashSet::new())
                .is_err()
        );
    }
}
