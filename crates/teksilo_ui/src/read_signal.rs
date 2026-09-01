// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! [`ReadSignal`] — a reactive value an extension may read and bind to, but never
//! write.
//!
//! ## Why the seam cannot hand out a bare `Signal`
//!
//! [`Signal::set`](teksilo::prelude::Signal::set) is `pub`, and every clone of a
//! signal shares one live state. So a seam context carrying a raw
//! `Signal<Option<ActiveItem>>` would not be telling an extension *what the writer
//! is looking at* — it would be handing over the app's own focus to drive. Same
//! for `dirty_seq`: an extension that set it backwards would make a Work with
//! unsaved edits report itself clean, and the close guard would let them go.
//!
//! ## Why not simply `signal.map(|v| v.clone())`
//!
//! It once could not be: [`observe`](teksilo::prelude::Signal::observe) *panicked*
//! on a derived signal, and `BuildContext::effect` is nothing but `observe` — so
//! an extension that reacted to focus imperatively, the obvious thing to do with
//! this, took a runtime panic. That is fixed in teksilo: a derived signal carries
//! the mutable roots it was built from and observing it registers on each of them.
//! (The same fix stopped the macOS native menu bar aborting the process on
//! `enabled(unsaved.and(&backup_mode.not()))`.)
//!
//! What survives is the *typing*. A bare `Signal<T>` handed across the seam is
//! writable by anyone holding it, and a projection is only unwritable as long as
//! nobody upstream hands out the mutable one by mistake — a property of a call
//! site, which rots. `ReadSignal` makes it a property of the type: it holds the
//! real mutable signal privately and publishes exactly two doors,
//! [`ReadSignal::signal`] for widget binding (a derived projection, so a `set` on
//! it cannot reach the original) and [`ReadSignal::on_change`] for effects.
//! Writing is not discouraged; it is unreachable.

use teksilo::prelude::{BuildContext, Signal};

/// A read-only view of a reactive value. Cheap to clone; every clone reads the
/// same live state.
#[derive(Clone)]
pub struct ReadSignal<T: Clone + 'static> {
    inner: Signal<T>,
}

impl<T: Clone + 'static> ReadSignal<T> {
    /// Wrap a live signal. Crate-internal: only the app may decide which of its
    /// own state becomes readable through the seam.
    pub(crate) fn new(inner: Signal<T>) -> Self {
        Self { inner }
    }

    /// The value right now.
    pub fn get(&self) -> T {
        self.inner.get()
    }

    /// A **derived** signal for widget binding — anything taking `impl
    /// Into<Prop<T>>`. Derived signals carry their sources, so a widget bound to
    /// this re-renders when the underlying value changes, and `set` on it is a
    /// panic rather than a write that reaches the app.
    pub fn signal(&self) -> Signal<T> {
        self.inner.map(|v| v.clone())
    }

    /// Run `f` whenever the value changes, for the lifetime of the build that
    /// registered it (torn down on rebuild or destroy, exactly like any
    /// `ctx.effect`).
    pub fn on_change(&self, ctx: &mut BuildContext, f: impl Fn(&T) + 'static) {
        ctx.effect(&self.inner, f);
    }
}

impl<T: Clone + std::fmt::Debug + 'static> std::fmt::Debug for ReadSignal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ReadSignal")
            .field(&self.inner.get())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole point: what comes out for binding cannot write back.
    #[test]
    fn the_bindable_projection_is_read_only() {
        let src = Signal::new(1u32);
        let ro = ReadSignal::new(src.clone());
        assert!(
            ro.signal().try_set(9).is_err(),
            "a bound projection must not be writable, or the seam hands over the app's own state"
        );
        assert_eq!(src.get(), 1, "…and the original must be untouched");
    }

    /// It is a *view*, not a copy: the app's own writes are visible through it.
    #[test]
    fn reads_track_the_live_value() {
        let src = Signal::new(1u32);
        let ro = ReadSignal::new(src.clone());
        assert_eq!(ro.get(), 1);
        src.set(7);
        assert_eq!(ro.get(), 7);
        assert_eq!(ro.signal().get(), 7, "the derived projection tracks it too");
    }

    /// Both doors are observable, so `ctx.effect` cannot panic on either.
    ///
    /// Asserted on the two signals rather than through a `BuildContext` (the
    /// framework exposes no way to build one outside a real build pass): what
    /// `on_change` hands `ctx.effect` must be observable, and so must the
    /// projection `signal()` hands a widget — an extension that reaches for the
    /// bindable one and observes it directly is doing nothing wrong.
    #[test]
    fn both_doors_are_observable() {
        use std::cell::Cell;
        use std::rc::Rc;

        let src = Signal::new(0u32);
        let ro = ReadSignal::new(src.clone());

        let seen = Rc::new(Cell::new(0u32));
        let s = seen.clone();
        let _handle = ro
            .inner
            .try_observe(move |v| s.set(*v))
            .expect("on_change's target must be observable, or ctx.effect panics on it");

        let derived_seen = Rc::new(Cell::new(0u32));
        let d = derived_seen.clone();
        let _derived = ro.signal().try_observe(move |v| d.set(*v)).expect(
            "a derived signal registers on the mutable roots it was built from; \
             this used to be the panic that made ReadSignal wrap the mutable one",
        );

        src.set(4);
        assert_eq!(seen.get(), 4);
        assert_eq!(derived_seen.get(), 4);
    }

    /// …and the projection is still unwritable, which is now the whole of this
    /// type's job. Observability no longer distinguishes it from a plain
    /// `map()`; being impossible to `set` through does.
    #[test]
    fn the_observable_projection_is_still_not_writable() {
        let src = Signal::new(1u32);
        let ro = ReadSignal::new(src.clone());
        let bound = ro.signal();

        assert!(bound.try_observe(|_| {}).is_ok(), "observable…");
        assert!(bound.try_set(9).is_err(), "…but never writable");
        assert_eq!(src.get(), 1);
    }
}
