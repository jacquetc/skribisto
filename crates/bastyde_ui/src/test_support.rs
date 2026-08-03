// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Test-only scaffolding for headless widget tests.
//!
//! **Why this exists.** Several panes call `ctx.subscribe_event(..)` in their wiring
//! child so they re-source on backend changes — the Overview table, the Corkboard grid,
//! the manuscript stream. A bare [`WidgetTree`](bastyde::core::widget_tree::WidgetTree)
//! has no event source, and `subscribe_event` *panics* when there is none, so those panes
//! simply could not be laid out in a test at all. They were therefore only ever tested at
//! segment 0, which is precisely the segment none of them occupy.
//!
//! [`tree_with_events`] closes that hole: it registers the same
//! [`EventSource`](bastyde::core::event_source::EventSource) adapter the real app does,
//! over a throwaway `AppContext`, plus a no-op poster. The subscriptions are real — they
//! are simply never fired, because nothing in a headless test mutates the store on a
//! background thread.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::WidgetEvent;
use bastyde::core::accessibility::widget_id_to_node_id;
use bastyde::core::event_source::{
    AppEventPoster, EventSourceAdapter, SubscriptionId, TreeAppContext,
};
use bastyde::core::widget_id::WidgetId;
use bastyde::core::widget_tree::WidgetTree;
use bastyde::settings::SettingsStore;
use bastyde::widgets::ToastRegistry;

use frontend::{AppContext, EventHubClient};

/// A poster that drops everything.
///
/// The real one hands an event to the winit event loop; there is no loop here, and a
/// headless test asserts on **layout**, not on delivery. Dropping is honest — the
/// alternative (buffering events nobody drains) would only look like it worked.
struct NullPoster;

impl AppEventPoster for NullPoster {
    fn post_subscription_event(&self, _sub_id: SubscriptionId, _event: Box<dyn Any + Send>) {}
}

/// A `WidgetTree` that can host widgets which subscribe to backend events.
///
/// Pass the same `AppContext` the widgets under test were built against, so their
/// subscriptions land on the store they read.
pub(crate) fn tree_with_events(app_ctx: &Rc<AppContext>) -> WidgetTree {
    tree_with_events_and_state(app_ctx, HashMap::new())
}

/// As [`tree_with_events`], plus a throwaway [`SettingsStore`] in `app_state`.
///
/// `ctx.settings()` **panics** when no store is registered, so any widget that reads
/// a preference while building — the search preview reads its column width — cannot
/// be laid out by `tree_with_events` alone. The store is backed by a per-test temp
/// file (never the user's real settings) and left behind on disk, exactly as the
/// view-model tests' own temp stores are.
pub(crate) fn tree_with_settings(app_ctx: &Rc<AppContext>) -> WidgetTree {
    use std::sync::atomic::{AtomicU32, Ordering};
    static N: AtomicU32 = AtomicU32::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "skribisto_test_support_settings_{}_{n}.toml",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let store = SettingsStore::open(path).expect("open temp settings store");

    let mut state: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
    state.insert(TypeId::of::<SettingsStore>(), Box::new(store));
    tree_with_events_and_state(app_ctx, state)
}

/// A `WidgetTree` with a real [`ToastRegistry`] installed as `app_state`, so
/// `ctx.show_toast(...)` inside a wired handler reaches the SAME registry the
/// caller can then inspect via [`ToastRegistry::live_count`].
///
/// **Why this exists.** A test that only calls `work_scoped_toast_id(...)`
/// twice and compares the strings never touches the real call site — reverting
/// it to a bare static id would still pass. Driving the ACTUAL view-model
/// method through a real `EventContext` (wire a `Button` to it, then
/// [`click`] it) and asserting on `registry.live_count()` catches that: a bare
/// id collapses two Works' toasts into one live entry via
/// `ToastRegistry::enqueue`'s update-in-place merge.
pub(crate) fn tree_with_toast_registry(
    app_ctx: &Rc<AppContext>,
    registry: &ToastRegistry,
) -> WidgetTree {
    let mut state: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
    state.insert(TypeId::of::<ToastRegistry>(), Box::new(registry.clone()));
    tree_with_events_and_state(app_ctx, state)
}

/// Click `id` in `tree` via an AccessKit `Click` action — the same
/// synthetic-activation path `outline.rs`'s rename tests use, so a wired
/// `Button::on_activate_fn` (or any `on_activate_fn`) handler runs with a
/// real `EventContext`, not merely a direct fn call bypassing dispatch.
pub(crate) fn click(tree: &mut WidgetTree, id: WidgetId) {
    tree.dispatch_event(WidgetEvent::AccessAction {
        action: bastyde::core::accesskit::Action::Click,
        target: Some(id),
        target_node: widget_id_to_node_id(id),
        data: None,
    });
}

fn tree_with_events_and_state(
    app_ctx: &Rc<AppContext>,
    state: HashMap<TypeId, Box<dyn Any>>,
) -> WidgetTree {
    let mut tree = WidgetTree::new();
    let client = EventHubClient::new(&app_ctx.event_hub);
    let adapter = EventSourceAdapter::new(crate::EventHubSource { client });
    tree.set_app_context(Rc::new(
        TreeAppContext::with_source_and_poster(adapter, Arc::new(NullPoster)).with_app_state(state),
    ));
    tree
}
