// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Form-field checks that touch the disk, worked out when the field changes and
//! read for free afterwards.
//!
//! # Why the verdict is cached rather than derived
//!
//! A derived signal (`Signal::map`, `zip`, `and`) recomputes on **every** read,
//! and a bound field reads its validation each time it paints. A check that
//! touches the disk inside one is disk I/O per frame. The import dialogs'
//! destination check did exactly that: it created and deleted a probe file in
//! the chosen folder on every read, for as long as the dialog stayed open, which
//! a synced folder sees as an endless stream of files appearing and vanishing.
//!
//! [`CachedValidation`] holds the verdict in a plain signal instead and
//! recomputes it from an observer on the inputs, so the check runs once per
//! edit, inside the edit's own event, and a read is a clone. The price is that a
//! verdict can go stale while nothing is typed, in either direction:
//!
//! * **A pass gone stale** (a folder deleted behind the dialog's back). A caller
//!   about to act on the verdict calls [`CachedValidation::recheck`] first,
//!   which is what every button that writes somewhere does.
//! * **A refusal gone stale** (a folder created, a drive mounted or a permission
//!   fixed after the field was filled in). The button is off, so its recheck is
//!   out of reach; [`retry_refusals`] looks at the disk again, once a second,
//!   for the fields a form on screen refuses.

use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};

use teksilo::core::signal::ObserverHandle;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::ValidationState;

/// A field's [`ValidationState`], recomputed when one of its inputs is set and
/// cached in between.
///
/// Cheap to clone: clones share the verdict, the check and the observers. The
/// observers are released with the last clone, so a view-model holding one does
/// not keep its own inputs alive in a cycle.
#[derive(Clone)]
pub(crate) struct CachedValidation {
    verdict: Signal<ValidationState>,
    recheck: Rc<dyn Fn()>,
    /// Keeps the input observers registered for as long as a clone exists.
    _watch: Rc<Vec<ObserverHandle>>,
}

impl CachedValidation {
    /// Run `check` now, and again every time one of `inputs` is set.
    ///
    /// `check` reads whatever it needs itself (typically clones of the same
    /// input signals); it is handed nothing, because a check over two fields
    /// needs both whichever one changed.
    pub(crate) fn new<T: Clone + 'static>(
        inputs: &[&Signal<T>],
        check: impl Fn() -> ValidationState + 'static,
    ) -> Self {
        let verdict = Signal::new(check());
        // Weak: the observers live in the inputs, and an input must not keep the
        // verdict of a view-model that has gone.
        let target = verdict.downgrade();
        let recheck: Rc<dyn Fn()> = Rc::new(move || {
            if let Some(verdict) = target.as_ref().and_then(|w| w.upgrade()) {
                verdict.set(check());
            }
        });
        let watch = inputs
            .iter()
            .map(|input| {
                let recheck = recheck.clone();
                input.observe(move |_| recheck())
            })
            .collect();
        Self {
            verdict,
            recheck,
            _watch: Rc::new(watch),
        }
    }

    /// The cached verdict, for a field's `.validation(..)`. Reading it never
    /// runs the check.
    pub(crate) fn signal(&self) -> Signal<ValidationState> {
        self.verdict.clone()
    }

    /// Whether the cached verdict lets the form proceed: anything but an error.
    /// A warning (a file that will be replaced, say) is advice, not a refusal.
    pub(crate) fn passes(&self) -> Signal<bool> {
        self.verdict
            .map(|v| !matches!(v, ValidationState::Error(_)))
    }

    /// Run the check again now and report whether it passes. For the moment
    /// just before acting on the verdict, when the disk may have changed since
    /// the field last did.
    pub(crate) fn recheck(&self) -> bool {
        (self.recheck)();
        !self.refuses()
    }

    /// Whether the cached verdict refuses (an error). Reads the cache, never
    /// the disk.
    pub(crate) fn refuses(&self) -> bool {
        matches!(self.verdict.get(), ValidationState::Error(_))
    }

    /// The cached refusal's own words, or `None` when the verdict does not
    /// refuse. For saying why somewhere the field itself is not on screen.
    pub(crate) fn refusal(&self) -> Option<LocalizedString> {
        match self.verdict.get() {
            ValidationState::Error(reason) => Some(reason),
            _ => None,
        }
    }
}

/// How long a form on screen waits before it looks at the disk again for a
/// field it refuses.
pub(crate) const RETRY_REFUSED_AFTER: Duration = Duration::from_secs(1);

/// A form some of whose fields are checked against the disk, as
/// [`retry_refusals`] sees it.
pub(crate) trait DiskChecked: Clone + 'static {
    /// The cached verdicts of the fields checked against the disk. A refusal
    /// appearing in one of them starts the countdown to the next look.
    fn disk_verdicts(&self) -> Vec<Signal<ValidationState>>;

    /// Whether a field names something the disk refused: a path that is not
    /// there, not of the right kind, or not writable. Reads the fields and the
    /// cached verdicts only, never the disk. A blank field does not count:
    /// nothing on disk can fill it in.
    fn refused_on_disk(&self) -> bool;

    /// Run the checks of the fields [`Self::refused_on_disk`] counts again.
    fn retry_refused(&self);
}

/// While the widget being built is on screen, look at the disk again, once per
/// `every` ([`RETRY_REFUSED_AFTER`] in the app), for the fields `form` refuses
/// on the disk's say-so.
///
/// Without it, a refusal outlives its cause: the field keeps saying the folder
/// is missing after the writer has created it, and the button it gates stays
/// off until the field is edited for no reason the writer can see.
///
/// Cheap by construction. Nothing is scheduled while nothing is refused, so an
/// idle form never wakes the event loop. A refused path is looked at with a
/// metadata read, and the folder check's writability probe can only create its
/// file on the one look that clears the refusal, after which the looking
/// stops. The countdown runs off the window's frame tick, woken through its
/// shared deadline, and ends with the widget.
pub(crate) fn retry_refusals(ctx: &mut BuildContext, form: impl DiskChecked, every: Duration) {
    let wake = ctx.wake_at_handle();
    let due: Rc<Cell<Option<Instant>>> = Rc::default();
    // Ask for a look at `at` (or keep an earlier one already asked for), and
    // for the event loop to wake then. The wake deadline is shared by the whole
    // window, so the earlier of two deadlines wins, never the later.
    let schedule = {
        let (due, wake) = (due.clone(), wake.clone());
        Rc::new(move |at: Instant| {
            let at = due.get().map_or(at, |pending| pending.min(at));
            due.set(Some(at));
            wake.set(Some(wake.get().map_or(at, |other| other.min(at))));
        })
    };

    if form.refused_on_disk() {
        schedule(Instant::now() + every);
    }
    for verdict in form.disk_verdicts() {
        let (form, schedule) = (form.clone(), schedule.clone());
        ctx.effect(&verdict, move |_| {
            if form.refused_on_disk() {
                schedule(Instant::now() + every);
            }
        });
    }

    let tick = ctx.frame_tick();
    ctx.effect(&tick, move |_| {
        let Some(at) = due.get() else {
            return;
        };
        let now = Instant::now();
        if now < at {
            // A frame pumped for some other reason may have consumed the wake
            // asked for; ask again, or the look would wait for the next input.
            schedule(at);
            return;
        }
        due.set(None);
        if form.refused_on_disk() {
            form.retry_refused();
        }
        if form.refused_on_disk() {
            schedule(now + every);
        }
    });
}

/// The four things a destination-folder field can say, in the words of the form
/// that owns it.
pub(crate) struct FolderMessages {
    pub(crate) required: fn() -> LocalizedString,
    pub(crate) missing: fn() -> LocalizedString,
    pub(crate) not_folder: fn() -> LocalizedString,
    pub(crate) readonly: fn() -> LocalizedString,
}

/// Validate a destination folder: it must be named, exist, be a directory, and
/// be writable by this user.
///
/// Touches the disk, so it belongs in a [`CachedValidation`] and never in a
/// derived signal or a `build()`.
pub(crate) fn folder_state(dir: &str, messages: &FolderMessages) -> ValidationState {
    let trimmed = dir.trim();
    if trimmed.is_empty() {
        return ValidationState::Error((messages.required)());
    }
    let path = Path::new(trimmed);
    if !path.exists() {
        return ValidationState::Error((messages.missing)());
    }
    if !path.is_dir() {
        return ValidationState::Error((messages.not_folder)());
    }
    if !dir_writable(path) {
        return ValidationState::Error((messages.readonly)());
    }
    ValidationState::None
}

/// Probe writability with a uniquely named temporary file (owner mode bits alone
/// do not say whether the current user may write), then remove it.
fn dir_writable(dir: &Path) -> bool {
    let probe = dir.join(format!(".skribisto-writetest-{}", std::process::id()));
    match std::fs::File::create(&probe) {
        Ok(_) => {
            let _ = std::fs::remove_file(&probe);
            true
        }
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use teksilo::i18n::lit;

    static MESSAGES: FolderMessages = FolderMessages {
        required: || lit!("required"),
        missing: || lit!("missing"),
        not_folder: || lit!("not a folder"),
        readonly: || lit!("read-only"),
    };

    fn is_error(v: &ValidationState) -> bool {
        matches!(v, ValidationState::Error(_))
    }

    /// The whole point of the type: the check runs when an input is set and at
    /// no other time, however often the verdict is read.
    #[test]
    fn the_check_runs_per_edit_not_per_read() {
        let runs = Rc::new(Cell::new(0));
        let input = Signal::new(String::new());
        let counted = runs.clone();
        let cached = CachedValidation::new(&[&input], move || {
            counted.set(counted.get() + 1);
            ValidationState::None
        });
        assert_eq!(runs.get(), 1, "the first verdict is worked out up front");

        let verdict = cached.signal();
        let passes = cached.passes();
        for _ in 0..100 {
            let _ = verdict.get();
            let _ = passes.get();
        }
        assert_eq!(
            runs.get(),
            1,
            "reading the verdict must never rerun the check"
        );

        input.set("a".into());
        input.set("ab".into());
        assert_eq!(runs.get(), 3, "each edit reruns it once");

        assert!(cached.recheck());
        assert_eq!(runs.get(), 4, "and so does an explicit recheck");
    }

    /// A folder deleted after it was chosen: the cached verdict still says what
    /// it said, because nothing was typed, and a recheck sees the truth.
    #[test]
    fn a_folder_state_is_cached_until_rechecked() {
        let base = tempfile::tempdir().unwrap();
        let folder = base.path().join("books");
        std::fs::create_dir(&folder).unwrap();
        let location = Signal::new(String::new());
        let read = location.clone();
        let cached =
            CachedValidation::new(&[&location], move || folder_state(&read.get(), &MESSAGES));
        assert!(
            is_error(&cached.signal().get()),
            "a blank folder is refused"
        );

        location.set(folder.to_string_lossy().into_owned());
        assert!(matches!(cached.signal().get(), ValidationState::None));

        std::fs::remove_dir(&folder).unwrap();
        assert!(
            matches!(cached.signal().get(), ValidationState::None),
            "a read must not look at the disk again"
        );
        assert!(
            !cached.recheck(),
            "the recheck before acting must see it gone"
        );
        assert!(is_error(&cached.signal().get()));
    }

    /// The probe cleans up after itself: validating a folder leaves it as it was.
    #[test]
    fn folder_state_leaves_no_probe_behind() {
        let dir = tempfile::tempdir().unwrap();
        let state = folder_state(&dir.path().to_string_lossy(), &MESSAGES);
        assert!(matches!(state, ValidationState::None));
        assert_eq!(std::fs::read_dir(dir.path()).unwrap().count(), 0);
    }

    /// A stand-in form: one disk-checked field whose refusal the test controls,
    /// counting how often it is looked at again.
    #[derive(Clone)]
    struct Form {
        verdict: Signal<ValidationState>,
        /// What the disk would say if looked at now.
        on_disk: Rc<Cell<bool>>,
        retries: Rc<Cell<u32>>,
    }

    impl Form {
        fn refused() -> Self {
            Self {
                verdict: Signal::new(ValidationState::Error(lit!("missing"))),
                on_disk: Rc::new(Cell::new(false)),
                retries: Rc::default(),
            }
        }
        fn passing() -> Self {
            let form = Self::refused();
            form.verdict.set(ValidationState::None);
            form.on_disk.set(true);
            form
        }
    }

    impl DiskChecked for Form {
        fn disk_verdicts(&self) -> Vec<Signal<ValidationState>> {
            vec![self.verdict.clone()]
        }
        fn refused_on_disk(&self) -> bool {
            is_error(&self.verdict.get())
        }
        fn retry_refused(&self) {
            self.retries.set(self.retries.get() + 1);
            if self.on_disk.get() {
                self.verdict.set(ValidationState::None);
            }
        }
    }

    /// The widget a form's panel is: it installs the retry when built.
    #[derive(Debug)]
    struct Panel {
        form: Form,
        every: Duration,
    }

    impl std::fmt::Debug for Form {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Form").finish()
        }
    }

    impl Widget for Panel {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            retry_refusals(ctx, self.form.clone(), self.every);
            Vec::new()
        }
        fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
            proposal.resolve(10.0, 10.0).into()
        }
    }

    fn mounted(form: &Form, every: Duration) -> teksilo::core::widget_tree::WidgetTree {
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.add(Panel {
            form: form.clone(),
            every,
        });
        tree.layout(SizeProposal::exact(100.0, 100.0));
        tree
    }

    /// A form with nothing refused asks for nothing: no wake, and frames pumped
    /// for other reasons look at nothing.
    #[test]
    fn nothing_refused_nothing_scheduled() {
        let form = Form::passing();
        let tree = mounted(&form, Duration::ZERO);
        assert_eq!(
            tree.wake_at_handle().get(),
            None,
            "an idle form never wakes the loop"
        );
        for _ in 0..20 {
            tree.frame_tick().set(0.016);
        }
        assert_eq!(form.retries.get(), 0);
    }

    /// A refusal is looked at again once its time comes, not on every frame:
    /// frames pumped before then (typing elsewhere, a caret blinking) do no I/O.
    #[test]
    fn a_refusal_is_looked_at_when_due_and_not_per_frame() {
        let form = Form::refused();
        let tree = mounted(&form, Duration::from_secs(3600));
        let wake = tree.wake_at_handle().get().expect("a look is scheduled");
        assert!(wake > Instant::now() + Duration::from_secs(3000));
        for _ in 0..20 {
            tree.frame_tick().set(0.016);
        }
        assert_eq!(form.retries.get(), 0, "not due yet");
        // A frame that consumed the window's wake does not lose the look.
        tree.wake_at_handle().set(None);
        tree.frame_tick().set(0.016);
        assert_eq!(tree.wake_at_handle().get(), Some(wake));
    }

    /// Once due, the refusal is looked at; while the disk still refuses, the
    /// next look is scheduled, and once it passes, the looking stops.
    #[test]
    fn a_refusal_is_retried_until_the_disk_agrees() {
        let form = Form::refused();
        let tree = mounted(&form, Duration::ZERO);
        // Whether the mounting layout pass already woke for the first look is
        // the tree's business; count from here.
        let from = form.retries.get();
        tree.frame_tick().set(0.016);
        assert_eq!(form.retries.get(), from + 1);
        assert!(
            tree.wake_at_handle().get().is_some(),
            "still refused: look again"
        );

        form.on_disk.set(true);
        tree.frame_tick().set(0.016);
        assert_eq!(form.retries.get(), from + 2);
        assert!(!is_error(&form.verdict.get()), "the refusal cleared");
        for _ in 0..20 {
            tree.frame_tick().set(0.016);
        }
        assert_eq!(
            form.retries.get(),
            from + 2,
            "and nothing more is looked at"
        );
    }

    /// A refusal appearing after the form was built (the writer typed a path
    /// that is not there) starts the countdown by itself.
    #[test]
    fn a_new_refusal_starts_the_countdown() {
        let form = Form::passing();
        let tree = mounted(&form, Duration::ZERO);
        assert_eq!(tree.wake_at_handle().get(), None);
        form.verdict.set(ValidationState::Error(lit!("missing")));
        form.on_disk.set(true);
        assert!(
            tree.wake_at_handle().get().is_some(),
            "the refusal asked for a look"
        );
        tree.frame_tick().set(0.016);
        assert_eq!(form.retries.get(), 1);
        assert!(!is_error(&form.verdict.get()));
    }

    #[test]
    fn folder_state_refuses_a_file_and_a_missing_path() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("novel.skrib");
        std::fs::write(&file, b"x").unwrap();
        assert!(is_error(&folder_state(&file.to_string_lossy(), &MESSAGES)));
        assert!(is_error(&folder_state(
            &dir.path().join("nowhere").to_string_lossy(),
            &MESSAGES
        )));
        assert!(is_error(&folder_state("   ", &MESSAGES)));
    }
}
