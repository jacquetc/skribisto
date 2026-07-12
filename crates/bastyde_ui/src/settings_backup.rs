//! The two backup ("Copies de secours") settings panes:
//!
//! - [`general_pane`] — the app-wide default policy (`Sec::BackupSync`).
//! - [`work_backup_pane`] — the open project's optional override (`Sec::Work`),
//!   with a "use the general backup settings" toggle, the "last backup"
//!   indicator / no-backups hint, and an "Open backups list" button.
//!
//! Both build a single [`FormLayout`] with the shared `group` helper and are
//! wrapped by the caller in the standard `pane_frame` (breadcrumb · rule ·
//! scroll), so they look and scroll exactly like every other settings pane.
//!
//! In the per-project pane the override controls are **shown but disabled** while
//! "use general settings" is on (not hidden) — the inherited values stay visible,
//! greyed out. Each control's `.enabled(..)` is bound to a signal that tracks the
//! inherit toggle; flipping it off enables them and materialises an override keyed
//! by the project uid.
//!
//! Because a `BackupPolicy` is a single struct (not per-field signals), each
//! control mirrors one field into a local signal and, on change, does a
//! read-modify-write of the *current* policy through the supplied `set` closure —
//! so concurrent field edits compose.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::tooltip::TooltipContent;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, FormLayout, HStack, IconButton, Segment,
    SegmentedControl, SpinBox, Switcher, TextWidget, Toggle, VStack,
};

use crate::backup::is_destination_available;
use crate::models::{BackupPolicy, RetentionMode, uid_is_usable};
use crate::settings_panel::group;
use crate::view_models::BackupSettingsViewModel;

type Get = Rc<dyn Fn() -> BackupPolicy>;
type Set = Rc<dyn Fn(BackupPolicy)>;

/// The general (app-wide default) backup policy pane — a single `FormLayout`,
/// always editable. The caller wraps it in `pane_frame`.
pub fn general_pane(ctx: &mut BuildContext, vm: &BackupSettingsViewModel) -> impl Widget {
    let get: Get = {
        let vm = vm.clone();
        Rc::new(move || vm.general())
    };
    let set: Set = {
        let vm = vm.clone();
        Rc::new(move |p| vm.set_general(p))
    };
    let always_on = Signal::new(true);
    add_policy_rows(
        FormLayout::new()
            .label(tr!(settings_page_backup()))
            .label_gap(16.0)
            .row_spacing(14.0),
        ctx,
        get,
        set,
        always_on,
    )
}

/// The per-project override pane. A "use general settings" toggle sits above the
/// same control set; when it is on, the controls are **disabled (not hidden)** and
/// display the inherited values. Toggling it off writes an override keyed by uid.
pub fn work_backup_pane(
    ctx: &mut BuildContext,
    vm: &BackupSettingsViewModel,
    uid: String,
    path: String,
    title: String,
) -> impl Widget {
    let usable = uid_is_usable(&uid);
    let has_override = vm.has_override(&uid);
    // `inherit == true` ⇒ no override (use general). Toggling writes/clears it.
    let inherit = Signal::new(!has_override);
    {
        let vm = vm.clone();
        let uid = uid.clone();
        let path = path.clone();
        let title = title.clone();
        ctx.effect(&inherit, move |inherited| {
            // Never key an override by an unusable (empty) uid.
            if !usable {
                return;
            }
            if *inherited {
                vm.clear_override(&uid);
            } else if !vm.has_override(&uid) {
                // Seed the new override from the currently-effective policy.
                let seed = vm.effective_for(&uid);
                vm.set_override(&uid, &path, &title, seed);
            }
        });
    }

    // `enabled == !inherit`: the controls are live only when overriding. A plain
    // mutable signal (not derived) so it drives every control's `.enabled(..)`.
    let enabled = Signal::new(has_override && usable);
    {
        let enabled = enabled.clone();
        ctx.effect(&inherit, move |inherited| {
            enabled.set(!*inherited && usable)
        });
    }

    let get: Get = {
        let vm = vm.clone();
        let uid = uid.clone();
        Rc::new(move || vm.effective_for(&uid))
    };
    let set: Set = {
        let vm = vm.clone();
        let uid = uid.clone();
        let path = path.clone();
        let title = title.clone();
        Rc::new(move |p| {
            if uid_is_usable(&uid) {
                vm.set_override(&uid, &path, &title, p);
            }
        })
    };

    // "Last backup" indicator / no-backups hint (computed when the pane opens).
    let status = {
        let last = vm.last_backup_at(&uid);
        let off = vm.effective_for(&uid).is_effectively_off();
        match (last, off) {
            (_, true) => tr!(settings_backup_none_hint()),
            (Some(dt), _) => tr!(settings_backup_last(date = dt)),
            (None, _) => tr!(settings_backup_last_never()),
        }
    };

    let open_list = {
        let vm = vm.clone();
        let uid = uid.clone();
        let path = path.clone();
        Button::new(tr!(settings_backup_open_list())).on_activate_fn(move |c| {
            let dirs = vm.effective_for(&uid).destinations;
            let (uid, path) = (uid.clone(), path.clone());
            c.present_modal(
                bastyde::core::modal::ModalRequest::deferred(move |t| {
                    t.add(crate::backups_list_panel::BackupsListPanel::new(
                        uid.clone(),
                        path.clone(),
                        dirs.clone(),
                    ))
                })
                .presentation(bastyde::core::modal::ModalPresentation::InTree)
                .title(tr!(backups_title()))
                .size(700, 540)
                .close_behavior(bastyde::core::modal::ModalCloseBehavior::EscapeOrClickOutside),
            );
        })
    };

    // One FormLayout: status + the (always-enabled) inherit toggle, then the same
    // control rows as the general pane — each gated on `enabled` — then the button.
    let form = FormLayout::new()
        .label(tr!(settings_page_work_backup()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(
            TextWidget::new(status)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary),
        )
        .full_width(Toggle::new(inherit).label(tr!(settings_backup_inherit())))
        .full_width(Divider::new());
    add_policy_rows(form, ctx, get, set, enabled)
        .full_width(Divider::new())
        .full_width(open_list)
}

/// Append the shared policy controls (triggers, destinations, retention, dedup) to
/// `form`, each wired to read-modify-write the policy via `set` and gated on
/// `enabled` (always `true` for the general pane). Kept as one FormLayout — no
/// nested forms — so every row shares the pane's label column and full width.
fn add_policy_rows(
    form: FormLayout,
    ctx: &mut BuildContext,
    get: Get,
    set: Set,
    enabled: Signal<bool>,
) -> FormLayout {
    let p0 = get();

    // ── Triggers ──
    let on_close = bool_field(ctx, &get, &set, p0.on_close, |p, v| p.on_close = v);
    let on_open = bool_field(ctx, &get, &set, p0.on_open, |p, v| p.on_open = v);
    let interval_on = bool_field(ctx, &get, &set, p0.interval_enabled, |p, v| {
        p.interval_enabled = v
    });
    let interval_hours = int_field(ctx, &get, &set, p0.interval_hours as i64, |p, v| {
        p.interval_hours = v.clamp(1, 168) as u32
    });

    // ── Retention ──
    let retention_mode = Signal::new(match p0.retention_mode {
        RetentionMode::Tiered => 0usize,
        RetentionMode::KeepLastN => 1usize,
    });
    {
        let get = get.clone();
        let set = set.clone();
        ctx.effect(&retention_mode, move |m| {
            let mut p = get();
            p.retention_mode = if *m == 1 {
                RetentionMode::KeepLastN
            } else {
                RetentionMode::Tiered
            };
            set(p);
        });
    }
    // The tiered-vs-keep-N explanation, shown as a rich tooltip on each segment
    // (short summary + a "more" disclosure) so the choice is self-documenting.
    let tiered_tip = TooltipContent::new(
        "backup-retention-tiered",
        tr!(settings_backup_retention_tiered()),
    )
    .with_more(tr!(settings_backup_retention_tip_more()));
    let keepn_tip = TooltipContent::new(
        "backup-retention-keepn",
        tr!(settings_backup_retention_keep_n()),
    )
    .with_more(tr!(settings_backup_retention_tip_more()));
    let mode_control = SegmentedControl::new(retention_mode.clone())
        .segment(
            Segment::new(tr!(settings_backup_retention_tiered())).rich_tooltip_content(tiered_tip),
        )
        .segment(
            Segment::new(tr!(settings_backup_retention_keep_n())).rich_tooltip_content(keepn_tip),
        )
        .enabled(enabled.clone());

    let keep_n = int_field(ctx, &get, &set, p0.keep_last_n as i64, |p, v| {
        p.keep_last_n = v.max(1) as u32
    });
    let gfs_h = int_field(ctx, &get, &set, p0.gfs_hourly as i64, |p, v| {
        p.gfs_hourly = v.max(0) as u32
    });
    let gfs_d = int_field(ctx, &get, &set, p0.gfs_daily as i64, |p, v| {
        p.gfs_daily = v.max(0) as u32
    });
    let gfs_w = int_field(ctx, &get, &set, p0.gfs_weekly as i64, |p, v| {
        p.gfs_weekly = v.max(0) as u32
    });
    let gfs_m = int_field(ctx, &get, &set, p0.gfs_monthly as i64, |p, v| {
        p.gfs_monthly = v.max(0) as u32
    });
    // T1-1: floor of 1, matching `keep_last_n` — a policy must never be able to
    // express "keep zero backups" through the UI (defence in depth; the actual
    // floor is now enforced unconditionally in `skrib_format::retention`).
    let min_keep = int_field(ctx, &get, &set, p0.min_keep as i64, |p, v| {
        p.min_keep = v.max(1) as u32
    });

    // Keep-N vs GFS params, switched on the selected mode. Fixed-width labels (a
    // `.line()`/Expand label collapses to a sliver inside the hug-width Switcher).
    let gfs = VStack::new()
        .spacing(8.0)
        .child(spin_line(
            tr!(settings_backup_gfs_hourly()),
            gfs_h,
            0,
            168,
            &enabled,
        ))
        .child(spin_line(
            tr!(settings_backup_gfs_daily()),
            gfs_d,
            0,
            60,
            &enabled,
        ))
        .child(spin_line(
            tr!(settings_backup_gfs_weekly()),
            gfs_w,
            0,
            52,
            &enabled,
        ))
        .child(spin_line(
            tr!(settings_backup_gfs_monthly()),
            gfs_m,
            0,
            120,
            &enabled,
        ));
    let keepn = spin_line(tr!(settings_backup_keep_n()), keep_n, 1, 999, &enabled);
    let retention_params = Switcher::new(retention_mode.map(|m| *m))
        .child(gfs)
        .child(keepn);

    // ── Dedup ──
    let dedup = bool_field(ctx, &get, &set, p0.skip_if_unchanged, |p, v| {
        p.skip_if_unchanged = v
    });

    form
        // Triggers
        .full_width(group(tr!(settings_backup_triggers())))
        .full_width(
            Toggle::new(on_close)
                .label(tr!(settings_backup_on_close()))
                .enabled(enabled.clone()),
        )
        .full_width(
            Toggle::new(on_open)
                .label(tr!(settings_backup_on_open()))
                .enabled(enabled.clone()),
        )
        // Interval trigger + its "every N h" spin, on one aligned row.
        .line(
            Toggle::new(interval_on)
                .label(tr!(settings_backup_interval()))
                .enabled(enabled.clone()),
            FixedSize::new().width(120.0).child(
                SpinBox::new(interval_hours, 1i64, 168)
                    .suffix(" h")
                    .enabled(enabled.clone()),
            ),
        )
        // Destinations
        .full_width(group(tr!(settings_backup_destinations())))
        .full_width(DestinationsEditor::new(
            get.clone(),
            set.clone(),
            enabled.clone(),
        ))
        // Retention
        .full_width(group(tr!(settings_backup_retention())))
        .full_width(mode_control)
        .full_width(retention_params)
        .full_width(spin_line(
            tr!(settings_backup_min_keep()),
            min_keep,
            1,
            99,
            &enabled,
        ))
        .full_width(
            Toggle::new(dedup)
                .label(tr!(settings_backup_dedup()))
                .enabled(enabled),
        )
}

/// A retention param row: a fixed-width single-line label + a spin cell. Fixed
/// (not Expand) so the label survives the hug-width proposal a `Switcher` makes.
fn spin_line(
    label: impl Into<LocalizedString>,
    value: Signal<i64>,
    min: i64,
    max: i64,
    enabled: &Signal<bool>,
) -> impl Widget {
    HStack::new()
        .spacing(12.0)
        .child(
            FixedSize::new().width(210.0).child(
                TextWidget::new(label.into())
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
                    .single_line(),
            ),
        )
        .child(
            FixedSize::new()
                .width(120.0)
                .child(SpinBox::new(value, min, max).enabled(enabled.clone())),
        )
}

/// A bool field mirrored into a signal that writes back through `set`.
fn bool_field(
    ctx: &mut BuildContext,
    get: &Get,
    set: &Set,
    initial: bool,
    apply: fn(&mut BackupPolicy, bool),
) -> Signal<bool> {
    let sig = Signal::new(initial);
    let get = get.clone();
    let set = set.clone();
    ctx.effect(&sig, move |v| {
        let mut p = get();
        apply(&mut p, *v);
        set(p);
    });
    sig
}

/// An integer field mirrored into a signal that writes back through `set`.
fn int_field(
    ctx: &mut BuildContext,
    get: &Get,
    set: &Set,
    initial: i64,
    apply: fn(&mut BackupPolicy, i64),
) -> Signal<i64> {
    let sig = Signal::new(initial);
    let get = get.clone();
    let set = set.clone();
    ctx.effect(&sig, move |v| {
        let mut p = get();
        apply(&mut p, *v);
        set(p);
    });
    sig
}

// ── Destinations editor ──────────────────────────────────────────────────────

/// A reactive list of destination folders with per-row availability badges,
/// Remove, an "Add folder…" picker, and a "Refresh" availability re-check. Its
/// buttons honour `enabled` so it dims with the rest in the inherited pane.
///
/// **T2-3.** `is_destination_available` is a blocking `fs::metadata` call, so it
/// must never run inline in `build()` — an unplugged/unmounted or hung network
/// destination would stall the whole settings pane on every rebuild. Instead the
/// badges are driven from `availability` (a cache filled off-thread via the
/// main-thread async executor) and `build()` only ever reads that cache; a
/// change to the destination *set* kicks one background re-check (guarded by
/// `last_checked` so the rebuild the check's own completion triggers doesn't
/// re-spawn another one).
struct DestinationsEditor {
    get: Get,
    set: Set,
    enabled: Signal<bool>,
    /// Bumped on add/remove/refresh/availability-landed so the row list rebuilds.
    epoch: Signal<u64>,
    /// Last-known availability per destination path. `None` (missing key) means
    /// "not checked yet" — rendered as a neutral badge rather than guessing.
    availability: Rc<RefCell<HashMap<String, bool>>>,
    /// The destination set the last background check was kicked off for, so an
    /// unchanged rebuild (e.g. the one the check's own completion causes) does
    /// not spawn a redundant check.
    last_checked: Rc<RefCell<Option<Vec<String>>>>,
    /// The main-thread async executor, fetched once from `app_state`. `None`
    /// only if `install_async()` was somehow not called at startup — falls back
    /// to a synchronous (but still off the hot `build()` common path once
    /// cached) check rather than leaving every badge unknown forever.
    async_rt: Option<AsyncRuntimeHandle>,
    root_child: Option<WidgetId>,
}

impl DestinationsEditor {
    fn new(get: Get, set: Set, enabled: Signal<bool>) -> Self {
        Self {
            get,
            set,
            enabled,
            epoch: Signal::new(0),
            availability: Rc::new(RefCell::new(HashMap::new())),
            last_checked: Rc::new(RefCell::new(None)),
            async_rt: None,
            root_child: None,
        }
    }

    /// Kick a background availability re-check for `dests` unless that exact
    /// set was already the target of the last check.
    fn refresh_availability(&self, dests: Vec<String>) {
        if self.last_checked.borrow().as_ref() == Some(&dests) {
            return;
        }
        *self.last_checked.borrow_mut() = Some(dests.clone());
        let availability = self.availability.clone();
        let epoch = self.epoch.clone();
        match &self.async_rt {
            Some(rt) => {
                rt.spawn_local(async move {
                    let checked = spawn_blocking(move || {
                        dests
                            .into_iter()
                            .map(|d| {
                                let ok = is_destination_available(&d);
                                (d, ok)
                            })
                            .collect::<Vec<_>>()
                    })
                    .await
                    .unwrap_or_default();
                    {
                        let mut map = availability.borrow_mut();
                        map.clear();
                        map.extend(checked);
                    }
                    epoch.set(epoch.get().wrapping_add(1));
                })
                .detach();
            }
            None => {
                let mut map = availability.borrow_mut();
                map.clear();
                for d in &dests {
                    map.insert(d.clone(), is_destination_available(d));
                }
            }
        }
    }
}

impl std::fmt::Debug for DestinationsEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DestinationsEditor").finish()
    }
}

impl Widget for DestinationsEditor {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.epoch.bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            bastyde::core::BindingLevel::Rebuild,
        );
        if self.async_rt.is_none() {
            self.async_rt = ctx.app_state::<AsyncRuntimeHandle>().cloned();
        }

        let dests = (self.get)().destinations;
        // T2-3: kick (or skip, if unchanged since the last check) a background
        // availability re-check rather than calling `is_destination_available`
        // inline below for every row on every rebuild.
        self.refresh_availability(dests.clone());

        let mut col = VStack::new().spacing(6.0);

        if dests.is_empty() {
            col = col.child(
                TextWidget::new(tr!(settings_backup_dest_none()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
        }
        for (i, d) in dests.iter().enumerate() {
            // `None` (not checked yet) renders as a neutral badge rather than
            // guessing available/unavailable.
            let badge = match self.availability.borrow().get(d) {
                Some(true) => TextWidget::new(lit!("✓".to_string())).color(TextRole::Success),
                Some(false) => TextWidget::new(lit!("✗".to_string())).color(TextRole::Error),
                None => TextWidget::new(lit!("…".to_string())).color(TextRole::Secondary),
            };
            let get = self.get.clone();
            let set = self.set.clone();
            let epoch = self.epoch.clone();
            let idx = i;
            let row = HStack::new()
                .spacing(8.0)
                .child(FixedSize::new().width(16.0).child(badge))
                .child(
                    Expand::horizontal().child(
                        TextWidget::new(lit!(d.clone()))
                            .style(TextStyleRole::Small)
                            .single_line(),
                    ),
                )
                .child(
                    IconButton::clear()
                        .tooltip(tr!(settings_backup_dest_remove()))
                        .enabled(self.enabled.clone())
                        .on_activate_fn(move |_c| {
                            let mut p = get();
                            if idx < p.destinations.len() {
                                p.destinations.remove(idx);
                            }
                            set(p);
                            let e = &epoch;
                            e.set(e.get().wrapping_add(1));
                        }),
                );
            col = col.child(row);
        }

        // Add + Refresh buttons.
        let add = {
            let get = self.get.clone();
            let set = self.set.clone();
            let epoch = self.epoch.clone();
            Button::new(tr!(settings_backup_dest_add()))
                .enabled(self.enabled.clone())
                .on_activate_fn(move |c| {
                    let get = get.clone();
                    let set = set.clone();
                    let epoch = epoch.clone();
                    let req =
                        FileDialogRequest::pick_folder().title(tr!(settings_backup_dest_add()));
                    let _ = c.pick_folder(req, move |res, _c| {
                        if let FileDialogResult::Folder(Some(path)) = res {
                            let p_str = path.to_string_lossy().into_owned();
                            let mut p = get();
                            if !p.destinations.contains(&p_str) {
                                p.destinations.push(p_str);
                            }
                            set(p);
                            let e = &epoch;
                            e.set(e.get().wrapping_add(1));
                        }
                    });
                })
        };
        let refresh = {
            let epoch = self.epoch.clone();
            let last_checked = self.last_checked.clone();
            Button::new(tr!(settings_backup_dest_refresh()))
                .variant(ButtonVariant::Plain)
                .enabled(self.enabled.clone())
                .on_activate_fn(move |_c| {
                    // Force a re-check even if the destination set itself
                    // hasn't changed (e.g. a drive was just plugged in).
                    *last_checked.borrow_mut() = None;
                    let e = &epoch;
                    e.set(e.get().wrapping_add(1));
                })
        };
        col = col.child(HStack::new().spacing(8.0).child(add).child(refresh));

        let id = ctx.add(col);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}
