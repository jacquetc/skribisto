// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Test-only scaffolding for headless widget tests.
//!
//! **Why this exists.** Several panes call `ctx.subscribe_event(..)` in their wiring
//! child so they re-source on backend changes — the Overview table, the Corkboard grid,
//! the manuscript stream. A bare [`WidgetTree`](teksilo::core::widget_tree::WidgetTree)
//! has no event source, and `subscribe_event` *panics* when there is none, so those panes
//! simply could not be laid out in a test at all. They were therefore only ever tested on
//! the container's own page, which is precisely the segment none of them occupy.
//!
//! [`tree_with_events`] closes that hole: it registers the same
//! [`EventSource`](teksilo::core::event_source::EventSource) adapter the real app does,
//! over a throwaway `AppContext`, plus a no-op poster. The subscriptions are real — they
//! are simply never fired, because nothing in a headless test mutates the store on a
//! background thread.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use teksilo::core::WidgetEvent;
use teksilo::core::accessibility::widget_id_to_node_id;
use teksilo::core::event_source::{
    AppEventPoster, EventSourceAdapter, SubscriptionId, TreeAppContext,
};
use teksilo::core::widget_id::WidgetId;
use teksilo::core::widget_tree::WidgetTree;
use teksilo::i18n::lit;
use teksilo::prelude::{EventContext, SizeProposal};
use teksilo::settings::SettingsStore;
use teksilo::widgets::{Button, ToastRegistry};

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
        action: teksilo::core::accesskit::Action::Click,
        target: Some(id),
        target_node: widget_id_to_node_id(id),
        data: None,
    });
}

/// Run `f` with a real `EventContext`, the way a button press would: a `Button`
/// wired to it is added to `tree` as a root of its own and [`click`]ed, with a
/// layout pass on either side so whatever `f` raised (a toast, a modal request)
/// has been built by the time this returns. For view-model methods that need
/// the context but do not call for a widget of their own to test.
pub(crate) fn press(tree: &mut WidgetTree, f: impl Fn(&mut EventContext) + 'static) {
    let button = tree.add(Button::new(lit!("press")).on_activate_fn(f));
    tree.layout(SizeProposal::exact(900.0, 600.0));
    click(tree, button);
    tree.layout(SizeProposal::exact(900.0, 600.0));
}

/// The titles of the dialogs presented since the last drain, taking them off the
/// queue. A headless tree has no window manager to present them, so they wait
/// there, and the title is what tells one dialog from another.
pub(crate) fn drain_dialog_titles(tree: &mut WidgetTree) -> Vec<String> {
    tree.drain_pending_modal_requests()
        .into_iter()
        .filter_map(|queued| queued.request.title)
        .collect()
}

/// Points the open registry at a directory of the test's own for as long as
/// this lives, so code that consults it (the import dialogs' refusal to write
/// over an open project) neither reads nor reaps the machine's real lock files.
/// The override is per thread, like the test itself.
pub(crate) struct IsolatedOpenRegistry {
    _dir: tempfile::TempDir,
}

impl IsolatedOpenRegistry {
    pub(crate) fn new() -> Self {
        let dir = tempfile::tempdir().expect("a temporary open-registry directory");
        crate::shell::open_registry::set_dir_override(Some(dir.path().to_path_buf()));
        Self { _dir: dir }
    }
}

impl Drop for IsolatedOpenRegistry {
    fn drop(&mut self) {
        crate::shell::open_registry::set_dir_override(None);
    }
}

/// As [`tree_with_events`], plus whatever `state` the caller supplies: the general
/// form [`tree_with_settings`]/[`tree_with_toast_registry`] each specialise for one
/// type. Reach for this directly when a pane needs more than one `app_state` type at
/// once (e.g. both `TagsViewModel` and `MentionIndex`), rather than layering two
/// single-purpose helpers that would each build (and discard) their own `WidgetTree`.
pub(crate) fn tree_with_app_state(
    app_ctx: &Rc<AppContext>,
    state: HashMap<TypeId, Box<dyn Any>>,
) -> WidgetTree {
    tree_with_events_and_state(app_ctx, state)
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

/// First node at/under `root` whose fully-qualified type name ends with `suffix`
/// (DFS pre-order); type names come from `std::any::type_name`, so match the leaf.
///
/// Shared rather than re-declared per test module: this was written twice, once
/// in `tabs::tests` and once in `docks::inspector::tests`, and the two copies had
/// already drifted (`ends_with` against `contains`) — which is the difference
/// between "the Wrap" and "the WrapPanel that happens to contain it".
pub(crate) fn first_of_type(tree: &WidgetTree, root: WidgetId, suffix: &str) -> Option<WidgetId> {
    if tree
        .widget_type_name(root)
        .is_some_and(|n| n.ends_with(suffix))
    {
        return Some(root);
    }
    tree.children(root)
        .into_iter()
        .find_map(|c| first_of_type(tree, c, suffix))
}

/// Run `f` with the **real shipped** `en-US` messages installed.
///
/// Not `I18nConfig::test_only` with a hand-copied list of patterns, which is
/// the other precedent in this crate (`project::open_failure`): that proves a
/// copy agrees with itself, and the point here is to assert the text a writer
/// actually reads, from the message that actually shipped.
///
/// It also has to exist at all: with no manager installed, a message carrying
/// a `{ $count -> … }` plural selector resolves to its own id. A test comparing
/// two such strings then compares two ids, and passes whatever the arguments.
pub(crate) fn with_real_messages(f: impl FnOnce()) {
    with_shipped_messages("en-US", f);
}

/// Run `f` with the shipped `.ftl` files of `locale` installed as the only locale,
/// so `tr!(…).resolve_now()` returns exactly what a writer running in it reads.
///
/// For the locales `tr!` cannot check at compile time: keys are validated against
/// `en-US` only, so a French message with a misspelt argument or a missing key
/// compiles, and only resolving it says so.
pub(crate) fn with_shipped_messages(locale: &str, f: impl FnOnce()) {
    use teksilo::i18n::config::I18nConfig;
    use teksilo::i18n::manager::I18nManager;
    use teksilo::i18n::thread_local::{clear, install};

    // The app's own catalogue, so a `.ftl` file added to it reaches these tests
    // too: `startup`'s drift test holds that list in step with the files on disk.
    let Some(&(tag, files)) = crate::startup::app_locales()
        .iter()
        .find(|(tag, _)| *tag == locale)
    else {
        panic!("{locale} is not a locale the app ships");
    };
    let id: teksilo::i18n::LanguageIdentifier = match tag.parse() {
        Ok(id) => id,
        Err(e) => panic!("{tag} is not a locale: {e:?}"),
    };
    clear();
    let cfg = I18nConfig::new()
        .source_locale(id.clone())
        .supported_locales([id.clone()])
        .compile_in(&[(tag, files)])
        .auto_detect_os_locale(false)
        .fallback_locale(id);
    install(I18nManager::from_config(&cfg));
    f();
    clear();
}
