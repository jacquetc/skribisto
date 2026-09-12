// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `UpdateViewModel` — the one answer to "is this copy behind?", shared by every
//! surface that shows it.
//!
//! ## Tier 1, and why it is a process global rather than `app_state`
//!
//! Which release is current is a fact about the *installation*: one per process,
//! identical in every window, unrelated to any `Work`. That makes it Tier 1, so
//! the `app_state` trap `AppIds` documents does not apply.
//!
//! It is reached through [`view_model`] rather than `ctx.app_state` for a reason
//! `app_state` cannot solve: the Launcher is not an `App`. `shell/launcher_window.rs`
//! builds `WelcomePanel` under its own `WindowConfig` and never constructs `App`,
//! so nothing registered by `App::build` exists there. A `thread_local` reached
//! by a free function is available to both, and the whole UI runs on one thread.
//!
//! ## Why the state is a `Signal` over a file rather than a notification
//!
//! The obvious home is the status-bar bell, and it is the wrong one. Three
//! verified behaviours of teksilo's archive make an update notice a poor fit for
//! it, all of them correct for the events it *was* built for:
//!
//! - `NotificationArchiveModel::push` merges by `dedup_id` and then sets
//!   `read = false` and bumps the unread count regardless ("an in-place update IS
//!   new information for the user"). A daily re-check would therefore relight the
//!   badge every day, which is the nag this feature exists to avoid.
//! - The bell marks entries read when its popover *closes*, scoped by
//!   `route_visible`, and a `Broadcast` entry is visible at every scope. Opening
//!   the bell to look at an export notice and pressing Escape would consume the
//!   update row.
//! - An archived action is inert in this application: teksilo replays
//!   `intent_name` only through an `on_action_invoked` hook, and nothing here
//!   wires one (`settings/panes/notifications.rs` says so outright).
//!
//! All three come from the same mismatch. The archive is a log of *events*, and
//! "a newer version exists" is not an event, it is a standing fact. A fact
//! rendered from persisted state is always true, needs no read/unread state, and
//! clears itself when it stops being true, which is what happens the moment the
//! reader updates.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::*;

use crate::models::updates_file::UpdatesService;
use crate::updates::channel::Channel;
use crate::updates::compare::{self, Verdict};
use crate::updates::feed;

/// A newer release, as the surfaces need it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Available {
    /// The version, rendered (`3.0.2`).
    pub version: String,
    /// Publication date, `YYYY-MM-DD`. Empty when the feed omitted it.
    pub date: String,
    /// Release-notes page in the interface language, if the feed published one.
    pub notes_url: Option<String>,
    /// Download page in the interface language, if the feed published one.
    pub download_url: Option<String>,
}

/// How a hand-driven check ended, for the one toast it raises.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckOutcome {
    /// A newer release exists.
    Behind(Available),
    /// This build is the newest release, or newer than it.
    Current,
    /// The check could not be completed. Carries the reason, for the toast.
    Failed(String),
}

struct Inner {
    /// The standing fact, or `None` when there is nothing to say. Bound by every
    /// surface; one `Signal` reaches every window.
    available: Signal<Option<Available>>,
    /// Guards the once-per-process automatic kick. Both `App::build` and the
    /// Launcher's panel call [`UpdateViewModel::start_if_due`], and a session
    /// that opens three windows must still make one request.
    kicked: RefCell<bool>,
    store: UpdatesService,
    /// The version this build reports, read once at construction.
    ///
    /// Held rather than re-read because it is a constant of the process, and
    /// because holding it is what lets a test pin it rather than inherit
    /// whatever `git describe` said about the checkout (`for_tests`).
    running: String,
}

/// Handle on the update state. Cloneable; every clone shares one state.
#[derive(Clone)]
pub struct UpdateViewModel {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for UpdateViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateViewModel").finish()
    }
}

thread_local! {
    static VIEW_MODEL: RefCell<Option<UpdateViewModel>> = const { RefCell::new(None) };
}

/// The process's update state, created on first use.
///
/// Safe to call from anywhere on the UI thread, including before any window
/// exists.
pub fn view_model() -> UpdateViewModel {
    VIEW_MODEL.with(|slot| {
        if let Some(vm) = slot.borrow().as_ref() {
            return vm.clone();
        }
        let vm = UpdateViewModel::new(open_store());
        *slot.borrow_mut() = Some(vm.clone());
        vm
    })
}

/// The build a test pretends to be running.
///
/// Any parseable version does; this one is real enough to read in a failure
/// message. The feed versions the tests record are `999.0.0` on one side of it
/// and `0.0.1` on the other, so neither is close enough to it for a bump here to
/// change a verdict.
#[cfg(test)]
pub(crate) const RUNNING_VERSION_IN_TESTS: &str = "3.0.0";

/// Seed the process's update state. Tests only.
///
/// Each test thread has its own `thread_local`, so a seeded state is visible to
/// the widgets that test builds and to nothing else.
#[cfg(test)]
pub(crate) fn set_view_model_for_test(vm: UpdateViewModel) {
    VIEW_MODEL.with(|slot| *slot.borrow_mut() = Some(vm));
}

/// `updates.toml` under this edition's config directory, or an in-memory
/// stand-in when there is no usable one.
///
/// Under `cfg(test)` it is **always** in-memory. A unit test must not read the
/// developer's own `updates.toml`: a machine that happens to have a discovery
/// recorded would render an update line inside `AboutPanel`'s layout test and
/// pass or fail for a reason that has nothing to do with the code.
#[cfg(test)]
fn open_store() -> UpdatesService {
    UpdatesService::in_memory_default()
}

#[cfg(not(test))]
fn open_store() -> UpdatesService {
    match crate::identity::app_paths() {
        Some(paths) => UpdatesService::open(&paths).unwrap_or_else(|e| {
            eprintln!(
                "skribisto: could not open updates.toml ({e}); not remembering update checks"
            );
            UpdatesService::in_memory_default()
        }),
        None => UpdatesService::in_memory_default(),
    }
}

impl UpdateViewModel {
    pub fn new(store: UpdatesService) -> Self {
        Self::with_running_version(store, crate::version::app_version())
    }

    /// The same, with the running version supplied rather than taken from the
    /// build that is executing.
    fn with_running_version(store: UpdatesService, running: String) -> Self {
        let vm = Self {
            inner: Rc::new(Inner {
                available: Signal::new(None),
                kicked: RefCell::new(false),
                store,
                running,
            }),
        };
        vm.reload_from_store();
        vm
    }

    /// A view-model on a pinned, parseable build. **Every test must use this**,
    /// never [`Self::new`].
    ///
    /// [`crate::version::app_version`] is `git describe` of the checkout, so its
    /// value is a property of the clone rather than of the code. A checkout with
    /// no tags reachable — which is exactly what `actions/checkout` produces,
    /// since it is shallow and fetches none — reports a bare commit hash.
    /// [`compare::verdict`] answers [`Verdict::Unknown`] for that, deliberately
    /// and correctly, and `available` is then `None` whatever the feed said.
    ///
    /// Under `new`, therefore, every assertion that a newer release is announced
    /// fails in CI and passes on a maintainer's tagged clone, and — worse — every
    /// assertion that nothing is announced passes in CI for the wrong reason,
    /// testing an unreadable running version instead of the rule it names.
    #[cfg(test)]
    pub(crate) fn for_tests(store: UpdatesService) -> Self {
        Self::with_running_version(store, RUNNING_VERSION_IN_TESTS.to_string())
    }

    /// The standing fact, for a surface to bind.
    pub fn available(&self) -> Signal<Option<Available>> {
        self.inner.available.clone()
    }

    /// Whether any update surface should be drawn at all on this channel.
    pub fn shows_update_state(&self) -> bool {
        super::shows_update_state()
    }

    /// Recompute [`Self::available`] from what is on disk plus the running
    /// version.
    ///
    /// Called at construction and after every check. Doing the comparison here
    /// rather than storing its result means an application that has just been
    /// updated goes quiet on its very first launch, with no network and no
    /// bookkeeping: the stored `3.0.2` is no longer newer than the running
    /// `3.0.2`, so the surfaces simply have nothing to draw.
    pub fn reload_from_store(&self) {
        let stored = self.inner.store.get();
        let next = match compare::verdict(&self.inner.running, &stored.latest_version) {
            Verdict::Behind(newer) => Some(Available {
                version: newer.to_string(),
                date: stored.latest_date.clone(),
                notes_url: pick(&stored.notes).map(str::to_string),
                download_url: pick(&stored.download).map(str::to_string),
            }),
            Verdict::Current | Verdict::Unknown => None,
        };
        self.inner.available.set(next);
    }

    /// Start the once-a-day check, if this channel checks at all, the reader has
    /// not turned it off, and one has not already run today.
    ///
    /// Idempotent within a process: the second and third window of a session
    /// find it already kicked and do nothing.
    ///
    /// Deferred to [`BuildContext::run_after_mount`] for two reasons.
    /// `spawn_local_with` **panics** outside a window context, and `build()` has
    /// no window yet. And nothing about a version check belongs on the path that
    /// puts the first window on screen: the writer is waiting for that, and this
    /// can happen whenever.
    pub fn start_if_due(&self, ctx: &mut BuildContext, enabled: bool) {
        // Before the once-per-process guard, and before anything else. Settings ▸
        // Notifications clears a discovery when the reader turns the check off,
        // but that path only exists while the Settings window is open: a value
        // turned off through `--config`, or edited in `general.toml` by hand,
        // would otherwise leave the last discovery on screen forever. This is the
        // launch-time half of the same rule, and it is cheap because it does
        // nothing at all unless there is something to clear.
        self.forget_if_disabled(enabled);

        if self.inner.kicked.replace(true) {
            return;
        }
        if !enabled || !Channel::current().checks_automatically() {
            return;
        }
        let Some(today) = today() else { return };
        if self.inner.store.checked_today(&today) {
            return;
        }
        let me = self.clone();
        ctx.run_after_mount(move |c| me.spawn(c, today, None));
    }

    /// Run a check because the reader asked for one, reporting the outcome
    /// through `on_done`.
    ///
    /// Ignores both the cadence and the setting: asking is an answer the reader
    /// is entitled to whenever they ask for it.
    pub fn check_now(
        &self,
        ctx: &mut EventContext,
        on_done: impl Fn(CheckOutcome, &mut EventContext) + 'static,
    ) {
        let today = today().unwrap_or_default();
        self.spawn(ctx, today, Some(Rc::new(on_done)));
    }

    #[allow(clippy::type_complexity)]
    fn spawn(
        &self,
        ctx: &mut EventContext,
        today: String,
        on_done: Option<Rc<dyn Fn(CheckOutcome, &mut EventContext)>>,
    ) {
        let Some(url) = crate::identity::update_feed() else {
            // An edition that declared no feed. Silence, never somebody else's
            // download page; see `AppIdentity::update_feed`.
            if let Some(done) = on_done {
                done(
                    CheckOutcome::Failed("this build declares no update source".to_string()),
                    ctx,
                );
            }
            return;
        };

        // Before the request, not after: this is a "do not ask again today"
        // mark, so a machine that is offline all week does not retry on every
        // launch. See `models::updates_file`.
        if !today.is_empty() {
            self.inner.store.mark_attempt(&today);
        }

        let me = self.clone();
        ctx.spawn_local_with(
            async move { spawn_blocking(move || feed::fetch(&url)).await },
            move |joined, ctx2| {
                let outcome = match joined {
                    Ok(Ok(feed)) => me.absorb(feed),
                    Ok(Err(e)) => CheckOutcome::Failed(e),
                    Err(_) => CheckOutcome::Failed("the check stopped unexpectedly".to_string()),
                };
                if let Some(done) = on_done {
                    done(outcome, ctx2);
                }
            },
        )
        .detach();
    }

    /// Store what a successful fetch found and recompute the standing fact.
    fn absorb(&self, feed: feed::Feed) -> CheckOutcome {
        self.inner.store.record(
            &feed.version,
            &feed.date,
            feed.notes.clone(),
            feed.download.clone(),
        );
        self.reload_from_store();
        match self.inner.available.get() {
            Some(a) => CheckOutcome::Behind(a),
            None => CheckOutcome::Current,
        }
    }

    /// React to the reader turning the check off.
    ///
    /// The surfaces have to go dark at once. Without this the last discovery
    /// would keep being drawn from the file, so a reader who turned the check off
    /// because they did not want to be told would go on being told.
    pub fn forget_if_disabled(&self, enabled: bool) {
        if enabled {
            return;
        }
        if self.inner.available.get().is_some() {
            self.inner.store.forget_discovery();
            self.reload_from_store();
        }
    }
}

/// Today in UTC as `YYYY-MM-DD`, or `None` if the clock is unusable.
///
/// UTC rather than local time for the same reason `date_convert::today_utc`
/// gives: this is a cadence gate, and a gate that moves with the timezone would
/// let a traveller check twice in one day.
fn today() -> Option<String> {
    crate::date_convert::today_utc().map(|d| d.to_string())
}

/// The link for the interface language, with the fallbacks
/// [`feed::Feed::download_for`] documents.
///
/// Resolved at read time rather than stored, so switching the interface language
/// switches the page the button opens without another check.
fn pick(map: &std::collections::BTreeMap<String, String>) -> Option<&str> {
    let language = teksilo::i18n::current_locale()
        .map(|sig| sig.get().to_string())
        .unwrap_or_default();
    let base = language.split(['-', '_']).next().unwrap_or(&language);
    map.get(&language)
        .or_else(|| map.get(base))
        .or_else(|| map.get("en"))
        .or_else(|| map.values().next())
        .map(String::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::updates_file::UpdatesService;
    use std::collections::BTreeMap;

    fn links(kind: &str) -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "en".to_string(),
                format!("https://www.skribisto.eu/{kind}/"),
            ),
            (
                "fr".to_string(),
                format!("https://www.skribisto.eu/fr/{kind}/"),
            ),
        ])
    }

    #[test]
    fn nothing_is_shown_when_no_check_has_ever_run() {
        let vm = UpdateViewModel::for_tests(UpdatesService::in_memory_default());
        assert_eq!(vm.available().get(), None);
    }

    #[test]
    fn a_stored_newer_release_becomes_the_standing_fact() {
        let store = UpdatesService::in_memory_default();
        store.record("999.0.0", "2026-12-01", links("news"), links("download"));
        let vm = UpdateViewModel::for_tests(store);
        let a = vm
            .available()
            .get()
            .expect("999.0.0 is newer than any build");
        assert_eq!(a.version, "999.0.0");
        assert_eq!(a.date, "2026-12-01");
        assert!(a.download_url.is_some());
        assert!(a.notes_url.is_some());
    }

    #[test]
    fn a_stored_older_release_says_nothing() {
        let store = UpdatesService::in_memory_default();
        store.record("0.0.1", "2020-01-01", links("news"), links("download"));
        let vm = UpdateViewModel::for_tests(store);
        assert_eq!(vm.available().get(), None);
    }

    /// The self-clearing property, and the reason the comparison is redone on
    /// load rather than its result stored. After updating to the version the
    /// file names, the surfaces must go dark with no network and no bookkeeping.
    #[test]
    fn the_notice_clears_itself_once_the_reader_has_updated() {
        let store = UpdatesService::in_memory_default();
        // Storing exactly the running version means "you are up to date".
        store.record(
            RUNNING_VERSION_IN_TESTS,
            "2026-09-04",
            links("news"),
            links("download"),
        );
        let vm = UpdateViewModel::for_tests(store);
        assert_eq!(
            vm.available().get(),
            None,
            "a stored version equal to the running one is not a newer one"
        );
    }

    #[test]
    fn an_unreadable_stored_version_says_nothing() {
        let store = UpdatesService::in_memory_default();
        store.record("not-a-version", "", links("news"), links("download"));
        let vm = UpdateViewModel::for_tests(store);
        assert_eq!(vm.available().get(), None);
    }

    #[test]
    fn turning_the_check_off_clears_the_surfaces_and_the_file() {
        let store = UpdatesService::in_memory_default();
        store.record("999.0.0", "2026-12-01", links("news"), links("download"));
        let vm = UpdateViewModel::for_tests(store.clone());
        assert!(vm.available().get().is_some());

        vm.forget_if_disabled(false);
        assert_eq!(vm.available().get(), None, "the surfaces go dark at once");
        assert_eq!(
            store.get().latest_version,
            "",
            "and the discovery is gone, so turning it back on cannot resurrect it"
        );
    }

    #[test]
    fn leaving_the_check_on_changes_nothing() {
        let store = UpdatesService::in_memory_default();
        store.record("999.0.0", "2026-12-01", links("news"), links("download"));
        let vm = UpdateViewModel::for_tests(store);
        vm.forget_if_disabled(true);
        assert!(vm.available().get().is_some());
    }

    #[test]
    fn one_signal_is_shared_by_every_clone() {
        // Two windows hold two clones and must never disagree about the fact.
        let store = UpdatesService::in_memory_default();
        let vm = UpdateViewModel::for_tests(store.clone());
        let other = vm.clone();
        store.record("999.0.0", "2026-12-01", links("news"), links("download"));
        vm.reload_from_store();
        assert_eq!(other.available().get(), vm.available().get());
        assert!(other.available().get().is_some());
    }
}
