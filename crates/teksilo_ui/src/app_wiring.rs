// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! App-lifetime wiring: how an extension gets a real [`BuildContext`] without
//! owning a widget.
//!
//! ## The gap this closes
//!
//! [`crate::commands_ext`] exists because an extension has no always-mounted
//! widget to hang a *verb* on. The same is true of its *state*, and until this
//! module there was nowhere to put it: an extension could
//! [register a settings key](crate::settings_ext) that its own backend hooks
//! could not read.
//!
//! That is not a hypothetical shape. Both of the other backend doors —
//! `bundle_contributors` and `lifecycle` — hand their work to `Send + Sync` code
//! on a worker thread, and `SettingsStore` is `Rc`-backed and lives on the UI
//! thread. So an extension whose save hook should obey a preference had exactly
//! two options, and both were wrong: read `general.toml` itself, reimplementing
//! the store and missing every `--config` pin; or hang the read off a panel's
//! `build`, which means the preference silently stops being honoured whenever
//! that panel is closed.
//!
//! ## What it is
//!
//! One callback, run from `App::build`, given `App`'s own `BuildContext`. What
//! an extension does with it is its own business — the usual shape is to read a
//! signal and mirror it into something `Send`:
//!
//! ```text
//! let enabled = Arc::new(AtomicBool::new(true));
//! let _handle = teksilo_ui::app_wiring::register_wiring("acme", {
//!     let enabled = enabled.clone();
//!     Rc::new(move |ctx: &mut BuildContext| {
//!         let signal = ctx.settings().signal("acme.enabled", true);
//!         enabled.store(signal.get(), Ordering::Relaxed);
//!         let enabled = enabled.clone();
//!         ctx.effect(&signal, move |v| enabled.store(*v, Ordering::Relaxed));
//!     })
//! });
//! ```
//!
//! **The extension chooses the type**, which is the whole reason this is a
//! callback rather than a "mirror these keys for me" convenience. A
//! `SettingsStore` key is pinned to the Rust type it was first read as, and
//! `signal::<T>` *panics* if a second reader asks for another one. A helper that
//! guessed `T` from the registered default's TOML variant would guess `f64` for
//! a key its owner reads as `f32`, and turn a working extension into a startup
//! crash. Only the extension knows.
//!
//! ## Lifetime, and why re-running is correct
//!
//! `App::build` runs once per window and again on every rebuild. This runs with
//! it, and that is right rather than merely tolerable: everything a callback
//! registers through `ctx` — effects, observers, actions — is owned by that build
//! cycle and dropped when it ends, so re-running restores exactly one of each.
//! A callback that keeps state outside `ctx` must therefore be **idempotent**: it
//! will run again, and more than one window may be open.
//!
//! ⚠ Like every registry in the seam this is a **snapshot**, taken as each window
//! builds. Register at startup, on the main thread, before `run()`.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::BuildContext;

/// What an extension wants done with `App`'s `BuildContext`, once per build.
pub type Wiring = Rc<dyn Fn(&mut BuildContext)>;

struct Registered {
    namespace: String,
    wiring: Wiring,
}

// `thread_local!`, like every UI-side registry here: `Wiring` is `Rc`-backed and
// is only ever run on the thread that builds windows.
thread_local! {
    static WIRING: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// Ask for `App`'s `BuildContext`, once per window build.
///
/// `namespace` identifies the extension, not what the callback does: registering
/// the same namespace twice **replaces** the earlier entry rather than stacking a
/// second copy, so a re-registration cannot quietly double-wire.
///
/// The returned [`WiringHandle`] unregisters on drop.
pub fn register_wiring(namespace: impl Into<String>, wiring: Wiring) -> WiringHandle {
    let namespace = namespace.into();
    WIRING.with(|reg| {
        let mut reg = reg.borrow_mut();
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            wiring,
        });
    });
    WiringHandle { namespace }
}

/// Unregisters its wiring when dropped.
#[derive(Debug)]
pub struct WiringHandle {
    namespace: String,
}

impl Drop for WiringHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = WIRING.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Whether anything is registered. For a caller that wants to skip building the
/// context it would pass.
pub fn has_wiring() -> bool {
    WIRING.with(|reg| !reg.borrow().is_empty())
}

/// Run every registered callback against `App`'s own `BuildContext`.
///
/// Call from `App::build`. A callback that panics is **caught and reported**,
/// and the remaining ones still run: an extension that cannot wire itself up is
/// a degraded application, and taking the window down over it would put a
/// writer's manuscript behind an extension's bug.
pub fn run_all_wiring(ctx: &mut BuildContext) {
    let all: Vec<(String, Wiring)> = WIRING.with(|reg| {
        reg.borrow()
            .iter()
            .map(|r| (r.namespace.clone(), r.wiring.clone()))
            .collect()
    });
    for (namespace, wiring) in all {
        // `AssertUnwindSafe`: `ctx` is borrowed for the call and not observed
        // afterwards by this function, and the alternative to catching is a
        // window that does not open.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| wiring(ctx)));
        if result.is_err() {
            eprintln!(
                "skribisto: extension '{namespace}' panicked while wiring itself up, skipping it"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A `BuildContext` cannot be made outside a real widget tree, so these
    /// exercise the registry itself — the part with the ordering, replacement and
    /// drop rules in it. That the callback receives `App`'s own context is
    /// `App::build`'s single call site, and is covered where the seam is
    /// exercised end to end.
    fn counter() -> (Arc<AtomicUsize>, Wiring) {
        let n = Arc::new(AtomicUsize::new(0));
        let wiring: Wiring = {
            let n = n.clone();
            Rc::new(move |_ctx: &mut BuildContext| {
                n.fetch_add(1, Ordering::Relaxed);
            })
        };
        (n, wiring)
    }

    #[test]
    fn nothing_is_registered_to_begin_with() {
        assert!(!has_wiring());
    }

    #[test]
    fn re_registering_a_namespace_replaces_rather_than_stacks() {
        let (first, w1) = counter();
        let (second, w2) = counter();
        let _a = register_wiring("test.wiring.dup", w1);
        let _b = register_wiring("test.wiring.dup", w2);
        let registered = WIRING.with(|reg| reg.borrow().len());
        assert_eq!(registered, 1, "the later registration must win outright");
        assert_eq!(first.load(Ordering::Relaxed), 0);
        assert_eq!(second.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn dropping_the_handle_unregisters() {
        {
            let (_n, w) = counter();
            let _h = register_wiring("test.wiring.scoped", w);
            assert!(has_wiring());
        }
        assert!(
            !has_wiring(),
            "a dropped handle must leave no wiring behind"
        );
    }

    #[test]
    fn registrations_keep_their_order() {
        let (_a, wa) = counter();
        let (_b, wb) = counter();
        let _ha = register_wiring("test.wiring.a", wa);
        let _hb = register_wiring("test.wiring.b", wb);
        let order: Vec<String> =
            WIRING.with(|reg| reg.borrow().iter().map(|r| r.namespace.clone()).collect());
        assert_eq!(order, vec!["test.wiring.a", "test.wiring.b"]);
    }
}
