//! The two backup ("Copies de secours") settings panes:
//!
//! - [`general_pane`] — the app-wide default policy (`Sec::BackupSync`).
//! - [`work_backup_pane`] — the open project's optional override (`Sec::Work`),
//!   with an "Inherit general settings" toggle, the "last backup" indicator, a
//!   no-backups hint, and an "Open backups list" button.
//!
//! Both are driven by [`BackupSettingsViewModel`]. Because a `BackupPolicy` is a
//! single struct (not per-field signals), each control mirrors one field into a
//! local signal and, on change, does a read-modify-write of the *current* policy
//! through the supplied `set` closure — so concurrent field edits compose.

use std::rc::Rc;

use bastyde::prelude::*;
use bastyde::widgets::tooltip::TooltipContent;
use bastyde::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, SegmentedControl,
    Segment, SpinBox, Switcher, TextWidget, Toggle, VStack,
};

use crate::backup::is_destination_available;
use crate::models::{BackupPolicy, RetentionMode};
use crate::view_models::BackupSettingsViewModel;

type Get = Rc<dyn Fn() -> BackupPolicy>;
type Set = Rc<dyn Fn(BackupPolicy)>;

/// The general (app-wide default) backup policy pane.
pub fn general_pane(ctx: &mut BuildContext, vm: &BackupSettingsViewModel) -> impl Widget {
    let get: Get = {
        let vm = vm.clone();
        Rc::new(move || vm.general())
    };
    let set: Set = {
        let vm = vm.clone();
        Rc::new(move |p| vm.set_general(p))
    };
    VStack::new()
        .spacing(16.0)
        .child(section_header(tr!(settings_backup_general_title())))
        .child(policy_controls(ctx, get, set))
}

/// The per-project override pane. Shows an "Inherit general settings" toggle;
/// when off, the same controls edit an override keyed by the project's `uid`.
pub fn work_backup_pane(
    ctx: &mut BuildContext,
    vm: &BackupSettingsViewModel,
    uid: String,
    path: String,
    title: String,
) -> impl Widget {
    let has_override = vm.has_override(&uid);
    // `inherit == true` ⇒ no override (use general). Toggling writes/clears it.
    let inherit = Signal::new(!has_override);
    {
        let vm = vm.clone();
        let uid = uid.clone();
        let path = path.clone();
        let title = title.clone();
        ctx.effect(&inherit, move |inh| {
            if *inh {
                vm.clear_override(&uid);
            } else if !vm.has_override(&uid) {
                // Seed the new override from the currently-effective policy.
                let seed = vm.effective_for(&uid);
                vm.set_override(&uid, &path, &title, seed);
            }
        });
    }

    // The override editor is shown only when not inheriting (Switcher on inherit).
    let editor_index = inherit.map(|inh| if *inh { 0usize } else { 1usize });
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
        Rc::new(move |p| vm.set_override(&uid, &path, &title, p))
    };

    let editor = Switcher::new(editor_index)
        .child(
            Padding::symmetric(0.0, 6.0).child(
                TextWidget::new(tr!(settings_backup_inheriting()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            ),
        )
        .child(policy_controls(ctx, get, set));

    // "Last backup" indicator + no-backups hint (computed when the pane opens).
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

    VStack::new()
        .spacing(16.0)
        .child(section_header(tr!(settings_backup_work_title())))
        .child(TextWidget::new(status).style(TextStyleRole::Small).color(TextRole::Secondary))
        .child(Toggle::new(inherit).label(tr!(settings_backup_inherit())))
        .child(editor)
        .child(open_list)
}

fn section_header(label: impl Into<LocalizedString>) -> impl Widget {
    TextWidget::new(label.into()).style(TextStyleRole::BodyBold)
}

/// The shared set of policy controls (triggers, destinations, retention, dedup),
/// each wired to read-modify-write the policy via `set`.
fn policy_controls(ctx: &mut BuildContext, get: Get, set: Set) -> impl Widget {
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
    let tiered_tip = TooltipContent::new("backup-retention-tiered", tr!(settings_backup_retention_tiered()))
        .with_more(tr!(settings_backup_retention_tip_more()));
    let keepn_tip = TooltipContent::new("backup-retention-keepn", tr!(settings_backup_retention_keep_n()))
        .with_more(tr!(settings_backup_retention_tip_more()));
    let mode_control = SegmentedControl::new(retention_mode.clone())
        .segment(Segment::new(tr!(settings_backup_retention_tiered())).rich_tooltip_content(tiered_tip))
        .segment(Segment::new(tr!(settings_backup_retention_keep_n())).rich_tooltip_content(keepn_tip));

    let keep_n = int_field(ctx, &get, &set, p0.keep_last_n as i64, |p, v| {
        p.keep_last_n = v.max(1) as u32
    });
    let gfs_h = int_field(ctx, &get, &set, p0.gfs_hourly as i64, |p, v| p.gfs_hourly = v.max(0) as u32);
    let gfs_d = int_field(ctx, &get, &set, p0.gfs_daily as i64, |p, v| p.gfs_daily = v.max(0) as u32);
    let gfs_w = int_field(ctx, &get, &set, p0.gfs_weekly as i64, |p, v| p.gfs_weekly = v.max(0) as u32);
    let gfs_m = int_field(ctx, &get, &set, p0.gfs_monthly as i64, |p, v| p.gfs_monthly = v.max(0) as u32);
    let min_keep = int_field(ctx, &get, &set, p0.min_keep as i64, |p, v| p.min_keep = v.max(0) as u32);

    // Show keep-N vs GFS params reactively on the selected mode.
    let retention_params = Switcher::new(retention_mode.map(|m| *m))
        .child(
            // Tiered / GFS
            VStack::new()
                .spacing(6.0)
                .child(spin_row(tr!(settings_backup_gfs_hourly()), gfs_h, 0, 168))
                .child(spin_row(tr!(settings_backup_gfs_daily()), gfs_d, 0, 60))
                .child(spin_row(tr!(settings_backup_gfs_weekly()), gfs_w, 0, 52))
                .child(spin_row(tr!(settings_backup_gfs_monthly()), gfs_m, 0, 120)),
        )
        .child(spin_row(tr!(settings_backup_keep_n()), keep_n, 1, 999));

    // ── Dedup ──
    let dedup = bool_field(ctx, &get, &set, p0.skip_if_unchanged, |p, v| {
        p.skip_if_unchanged = v
    });

    VStack::new()
        .spacing(10.0)
        .child(group_label(tr!(settings_backup_triggers())))
        .child(Toggle::new(on_close).label(tr!(settings_backup_on_close())))
        .child(Toggle::new(on_open).label(tr!(settings_backup_on_open())))
        .child(HStack::new().spacing(10.0)
            .child(Toggle::new(interval_on).label(tr!(settings_backup_interval())))
            .child(FixedSize::new().width(120.0).child(
                SpinBox::new(interval_hours, 1i64, 168).suffix(" h"),
            )))
        .child(Divider::new())
        .child(group_label(tr!(settings_backup_destinations())))
        .child(DestinationsEditor::new(get.clone(), set.clone()))
        .child(Divider::new())
        .child(group_label(tr!(settings_backup_retention())))
        .child(mode_control)
        .child(retention_params)
        .child(spin_row(tr!(settings_backup_min_keep()), min_keep, 0, 99))
        .child(Divider::new())
        .child(Toggle::new(dedup).label(tr!(settings_backup_dedup())))
}

fn group_label(label: impl Into<LocalizedString>) -> impl Widget {
    TextWidget::new(label.into())
        .style(TextStyleRole::SmallBold)
        .color(TextRole::Secondary)
}

fn spin_row(label: impl Into<LocalizedString>, value: Signal<i64>, min: i64, max: i64) -> impl Widget {
    HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(TextWidget::new(label.into()).style(TextStyleRole::Small)))
        .child(FixedSize::new().width(120.0).child(SpinBox::new(value, min, max)))
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
/// Remove, an "Add folder…" picker, and a "Refresh" availability re-check.
struct DestinationsEditor {
    get: Get,
    set: Set,
    /// Bumped on add/remove/refresh so the row list rebuilds.
    epoch: Signal<u64>,
    root_child: Option<WidgetId>,
}

impl DestinationsEditor {
    fn new(get: Get, set: Set) -> Self {
        Self {
            get,
            set,
            epoch: Signal::new(0),
            root_child: None,
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
        self.epoch
            .bind_to(ctx.self_id(), ctx.binding_registry(), bastyde::core::BindingLevel::Rebuild);

        let dests = (self.get)().destinations;
        let mut col = VStack::new().spacing(6.0);

        if dests.is_empty() {
            col = col.child(
                TextWidget::new(tr!(settings_backup_dest_none()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            );
        }
        for (i, d) in dests.iter().enumerate() {
            let available = is_destination_available(d);
            let badge = if available {
                TextWidget::new(lit!("✓".to_string())).color(TextRole::Success)
            } else {
                TextWidget::new(lit!("✗".to_string())).color(TextRole::Error)
            };
            let get = self.get.clone();
            let set = self.set.clone();
            let epoch = self.epoch.clone();
            let idx = i;
            let row = HStack::new()
                .spacing(8.0)
                .child(FixedSize::new().width(16.0).child(badge))
                .child(Expand::horizontal().child(
                    TextWidget::new(lit!(d.clone())).style(TextStyleRole::Small).single_line(),
                ))
                .child(
                    IconButton::clear()
                        .tooltip(tr!(settings_backup_dest_remove()))
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
            Button::new(tr!(settings_backup_dest_add())).on_activate_fn(move |c| {
                let get = get.clone();
                let set = set.clone();
                let epoch = epoch.clone();
                let req = FileDialogRequest::pick_folder().title(tr!(settings_backup_dest_add()));
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
            Button::new(tr!(settings_backup_dest_refresh()))
                .variant(ButtonVariant::Plain)
                .on_activate_fn(move |_c| {
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
