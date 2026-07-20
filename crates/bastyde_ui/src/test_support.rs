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

use std::any::Any;
use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::event_source::{
    AppEventPoster, EventSourceAdapter, SubscriptionId, TreeAppContext,
};
use bastyde::core::widget_tree::WidgetTree;

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
    let mut tree = WidgetTree::new();
    let client = EventHubClient::new(&app_ctx.event_hub);
    let adapter = EventSourceAdapter::new(crate::EventHubSource { client });
    tree.set_app_context(Rc::new(TreeAppContext::with_source_and_poster(
        adapter,
        Arc::new(NullPoster),
    )));
    tree
}
