// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The notice a project import raises about what it could not carry across, and
//! the way back to it once the notice is gone.
//!
//! Both project importers (Plume Creator, Manuskript) record everything they had
//! to drop or guess at. Read by nobody, that is an import that quietly lost data,
//! so the notice has three properties the first version lacked:
//!
//! * **It stays until the writer closes it** (`persistent`). A toast left to its
//!   default vanishes after ten seconds, which is shorter than the time it takes
//!   to notice one while a project is opening.
//! * **It is always admitted** (`ToastPriority::High`). A normal toast arriving
//!   while the corner already holds five is dropped outright, archive included.
//! * **It can be reopened** from the notification log (the status-bar bell, and
//!   Settings ▸ Notifications). The archive keeps no closures, only an action's
//!   name, so the **Details** action is named ([`WarningsNotice::action`]) and
//!   the list itself travels in the toast body, which the archive does keep.
//!   [`replay_archived_action`] is the hook both logs call to turn that name back
//!   into the dialog.
//!
//! **To a screen reader it is an alert, so what it says names the result too.**
//! Teksilo gives a warning toast at `High` priority the `Alert` role, which is
//! announced assertively, and the notice is raised just after the import's own
//! result toast, which is a polite status. An assertive announcement can cut
//! short a polite one being spoken, so the result could be lost to the very
//! writer who most needs to hear it. The notice's spoken name
//! ([`Toast::announcement`]) therefore carries the import's result as well as
//! the count, and the visible title stays the count alone. The name is what a
//! live region announces; the list is the body, there to read and in the log
//! afterwards.
//!
//! Each import's notice carries an id of its own. The archive merges a toast into
//! any earlier row with the same id, across sessions, so a shared id would fold
//! every import's warnings into one row, and reopening it would show the last
//! import's list under the first import's date.
//!
//! **Only the latest notice of an importer stays on screen** ([`LiveNotice`]).
//! A notice that waits for the writer and is always admitted would otherwise
//! pile up import after import, and the toast corner holds five: once full, the
//! next import's own progress, result and failure toasts had no room left and
//! were dropped, unseen and unlogged. The earlier notice goes from the screen
//! only; its row, and so its list, stays in the log.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::i18n::{LocalizedString, lit, localized};
use teksilo::prelude::*;
use teksilo::widgets::{
    ArchivedAction, MessageBox, MessageBoxButtons, NotificationEntry, Toast, ToastAction,
    ToastHandle, ToastPriority,
};

/// One importer's warnings notice: its words, and the name its **Details**
/// action is archived under.
pub(crate) struct WarningsNotice {
    /// The archived name of the **Details** action, and the prefix of every
    /// notice's own id. Stable: it is persisted in the notification archive, and
    /// a renamed action would leave every archived notice unable to reopen.
    pub(crate) action: &'static str,
    /// The headline, given the number of warnings.
    pub(crate) title: fn(i64) -> LocalizedString,
    /// The label of the action that opens the list.
    pub(crate) details: fn() -> LocalizedString,
    /// The title of the dialog that shows the list.
    pub(crate) dialog_title: fn() -> LocalizedString,
}

/// The Plume Creator importer's notice.
pub(crate) static PLUME: WarningsNotice = WarningsNotice {
    action: "import.plume.warnings",
    title: |count| tr!(import_plume_warnings(count = count)),
    details: || tr!(import_plume_details()),
    dialog_title: || tr!(import_plume_warnings_title()),
};

/// The Manuskript importer's notice.
pub(crate) static MANUSKRIPT: WarningsNotice = WarningsNotice {
    action: "import.manuskript.warnings",
    title: |count| tr!(import_manuskript_warnings(count = count)),
    details: || tr!(import_manuskript_details()),
    dialog_title: || tr!(import_manuskript_warnings_title()),
};

/// Every notice an archived action can name.
static ALL: [&WarningsNotice; 2] = [&PLUME, &MANUSKRIPT];

impl WarningsNotice {
    /// The notice for one import's `warnings`, or `None` when there are none.
    /// `result` is the line the import's own result toast shows, which the
    /// notice's announcement repeats (see the module doc).
    ///
    /// Broadcast, like the import's own progress toast: the import belongs to no
    /// open Work, and one shared view-model is wired from every window.
    pub(crate) fn toast(
        &'static self,
        warnings: &[String],
        result: &LocalizedString,
    ) -> Option<Toast> {
        if warnings.is_empty() {
            return None;
        }
        let count = warnings.len() as i64;
        let text = warnings.join("\n");
        let details = text.clone();
        Some(
            Toast::warning((self.title)(count))
                .announcement(self.announcement(count, result.clone()))
                .id(format!("{}:{}", self.action, uuid::Uuid::new_v4()))
                .body(lit!(text))
                .persistent()
                .priority(ToastPriority::High)
                .broadcast()
                .action(
                    ToastAction::primary((self.details)(), move |c| {
                        self.present_details(c, &details)
                    })
                    // Reading the list is not dismissing it; the close button is.
                    .closes_toast(false)
                    .shortcut_id(self.action),
                ),
        )
    }

    /// What a screen reader announces when the notice appears: the import's
    /// `result`, then the count. Resolved when it is read, so it follows a change
    /// of language like the title does.
    fn announcement(&self, count: i64, result: LocalizedString) -> LocalizedString {
        let title = self.title;
        localized(move || {
            tr!(import_warnings_announcement(
                result = result.resolve_now().trim().to_string(),
                notice = title(count).resolve_now()
            ))
            .resolve_now()
        })
    }

    /// Raise the notice for `warnings` in place of the importer's previous one,
    /// which leaves the screen but keeps its row in the log. Nothing happens
    /// when there are no warnings: the previous notice is about another import,
    /// and stays until the writer closes it. `result` is the line of the
    /// import's own result toast.
    pub(crate) fn show(
        &'static self,
        ctx: &mut EventContext,
        warnings: &[String],
        result: &LocalizedString,
        live: &LiveNotice,
    ) {
        if let Some(toast) = self.toast(warnings, result) {
            live.replace(ctx, toast);
        }
    }

    /// The dialog holding the whole list.
    fn present_details(&self, ctx: &mut EventContext, text: &str) {
        MessageBox::warning((self.dialog_title)())
            .text(lit!(text.to_string()))
            .buttons(MessageBoxButtons::Ok)
            .present(ctx);
    }
}

/// The notice of one importer that is on screen right now, if any.
///
/// Held by the importer's view-model, one per importer, so that importing a
/// Manuskript project never takes a Plume import's notice down. Cheap to clone;
/// clones share the slot.
#[derive(Clone, Default)]
pub(crate) struct LiveNotice(Rc<RefCell<Option<ToastHandle>>>);

impl LiveNotice {
    /// Take the previous notice off the screen, then raise `toast` and remember
    /// it. Dismissing first frees its slot in the corner before the new notice
    /// asks for one.
    fn replace(&self, ctx: &mut EventContext, toast: Toast) {
        // Taken out of the cell before dismissing: a dismissal runs callbacks,
        // and none of them may find this cell still borrowed.
        let previous = self.0.borrow_mut().take();
        if let Some(previous) = previous {
            previous.dismiss(ctx);
        }
        let handle = ctx.show_toast(toast);
        *self.0.borrow_mut() = Some(handle);
    }
}

/// The notice an archived action belongs to, if it is one of these.
fn notice_for(action: &ArchivedAction) -> Option<&'static WarningsNotice> {
    let name = action.intent_name.as_deref()?;
    ALL.iter().copied().find(|notice| notice.action == name)
}

/// Replay an action clicked in the notification log: the `on_action_invoked`
/// hook of the status-bar bell and of Settings ▸ Notifications.
///
/// An archived entry has lost its closures; what it kept is the action's name
/// and the toast's body. For an import's **Details** that is everything the
/// dialog needs, so the list reopens as it was first shown, in any window and
/// after a restart. Any other name is ignored: nothing else in the application
/// archives a replayable action.
pub(crate) fn replay_archived_action(
    entry: &NotificationEntry,
    action: &ArchivedAction,
    ctx: &mut EventContext,
) {
    if let Some(notice) = notice_for(action) {
        notice.present_details(ctx, entry.body.as_deref().unwrap_or_default());
    }
}

/// A notification archive holding one import's notice for `warnings`, filed
/// the way the app files it: raised through a real toast registry. For the
/// tests of the surfaces that list the archive.
#[cfg(test)]
pub(crate) fn archive_holding(
    notice: &'static WarningsNotice,
    warnings: &[String],
) -> Rc<teksilo::widgets::NotificationArchiveModel> {
    use teksilo::widgets::{NotificationArchiveModel, ToastInstallOptions, ToastRegistry};

    let archive = Rc::new(NotificationArchiveModel::in_memory());
    let registry = ToastRegistry::with_archive(
        ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        },
        archive.clone(),
    );
    let mut tree = crate::test_support::tree_with_toast_registry(
        &Rc::new(frontend::AppContext::new()),
        &registry,
    );
    let warnings = warnings.to_vec();
    crate::test_support::press(&mut tree, move |c| {
        if let Some(toast) = notice.toast(&warnings, &lit!("Imported 12 items.")) {
            c.show_toast(toast);
        }
    });
    archive
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;
    use teksilo::core::styles::BannerSeverity;
    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::presets::intui;
    use teksilo::widgets::{
        ArchivedActionStyle, Expand, FixedSize, NotificationArchiveModel, NotificationLog, Spacer,
        ToastHandle, ToastHost, ToastInstallOptions, ToastRegistry, ToastRoute, VStack, ZStack,
    };

    use frontend::AppContext;

    use crate::test_support::{click, drain_dialog_titles, press};

    fn warnings() -> Vec<String> {
        vec![
            "Chapter 3: a separator carried prose; it became a scene.".to_string(),
            "Link to \"Marie\" could not be resolved.".to_string(),
        ]
    }

    /// The line of the import's own result toast, as a view-model passes it.
    fn result() -> LocalizedString {
        tr!(import_plume_done(imported = 12, skipped = 0))
    }

    /// A window with a toast corner and an in-memory archive, as the app has.
    fn window() -> (WidgetTree, ToastRegistry, Rc<NotificationArchiveModel>) {
        let options = ToastInstallOptions {
            archive: None,
            ..ToastInstallOptions::default()
        };
        let archive = Rc::new(NotificationArchiveModel::in_memory());
        let registry = ToastRegistry::with_archive(options.clone(), archive.clone());
        let mut tree =
            crate::test_support::tree_with_toast_registry(&Rc::new(AppContext::new()), &registry);
        tree.set_theme(intui::light());
        let root = tree.add(
            VStack::new().child(
                FixedSize::new()
                    .width(200.0)
                    .height(120.0)
                    .child(Spacer::new()),
            ),
        );
        let host = tree.add(ToastHost::new(registry.clone(), options));
        let filled = tree.add(Expand::new().respect_intrinsic().child(root));
        tree.add(ZStack::new().child(filled).child(host));
        (tree, registry, archive)
    }

    /// The notice stays: no auto-dismiss timer is armed for it, so nothing but
    /// the writer can take it down.
    #[test]
    fn the_warnings_notice_waits_for_the_writer() {
        let (mut tree, registry, _archive) = window();
        let wake = tree.wake_at_handle();
        let live = LiveNotice::default();
        press(&mut tree, move |c| {
            PLUME.show(c, &warnings(), &result(), &live)
        });

        assert_eq!(registry.live_count(), 1, "the notice is up");
        assert!(
            wake.get().is_none(),
            "a timed toast arms a wake deadline to dismiss itself; the \
             warnings notice must not have one"
        );
    }

    /// To a screen reader the notice is an alert, announced over the polite
    /// result toast raised just before it. So what it announces names the
    /// import's result as well as the count, while the title on screen and in
    /// the log stays the count. The list itself is not part of the announcement.
    #[test]
    fn the_notice_is_an_alert_that_names_the_result() {
        use teksilo::core::accesskit::Role;

        let (mut tree, _registry, archive) = window();
        let live = LiveNotice::default();
        press(&mut tree, move |c| {
            PLUME.show(c, &warnings(), &result(), &live)
        });

        let alert = tree
            .find_by_role(Role::Alert)
            .expect("a warning admitted at High priority is an alert");
        let spoken = tree
            .accessibility_node(alert)
            .name()
            .unwrap_or_default()
            .to_string();
        let result = result().resolve_now();
        let count = tr!(import_plume_warnings(count = 2)).resolve_now();
        assert!(spoken.contains(result.trim()), "the result: {spoken}");
        assert!(spoken.contains(&count), "the count: {spoken}");
        assert!(
            !spoken.contains("Marie"),
            "the list is there to read, not announced: {spoken}"
        );
        let title = archive
            .entries()
            .with_item(0, |e| e.title.clone())
            .expect("the notice is archived");
        assert_eq!(title, count, "the visible title is the count alone");
    }

    /// Nothing to report, nothing shown.
    #[test]
    fn no_warnings_no_notice() {
        let (mut tree, registry, archive) = window();
        let live = LiveNotice::default();
        press(&mut tree, move |c| {
            MANUSKRIPT.show(c, &[], &result(), &live)
        });
        assert_eq!(registry.live_count(), 0);
        assert_eq!(archive.entries().len(), 0);
    }

    /// What the archive keeps is enough to reopen the list: the Details action
    /// under its name, and the whole list in the body. Replaying it from the log
    /// opens the dialog again, long after the toast itself is gone.
    #[test]
    fn the_archived_notice_reopens_the_whole_list() {
        let (mut tree, registry, archive) = window();
        let handle: Rc<RefCell<Option<ToastHandle>>> = Rc::default();
        let shown = handle.clone();
        press(&mut tree, move |c| {
            if let Some(toast) = MANUSKRIPT.toast(&warnings(), &result()) {
                *shown.borrow_mut() = Some(c.show_toast(toast));
            }
        });
        assert_eq!(registry.live_count(), 1);
        assert_eq!(archive.entries().len(), 1);

        let entry = archive
            .entries()
            .with_item(0, |e| e.clone())
            .expect("the notice is archived");
        assert_eq!(entry.body.as_deref(), Some(warnings().join("\n").as_str()));
        let details = entry
            .actions
            .iter()
            .find(|a| a.intent_name.as_deref() == Some(MANUSKRIPT.action))
            .cloned()
            .expect("the Details action is archived under its name");

        // The writer closes the toast.
        press(&mut tree, move |c| {
            if let Some(h) = handle.borrow().as_ref() {
                h.dismiss(c);
            }
        });
        assert_eq!(registry.live_count(), 0, "the toast is gone");
        assert_eq!(archive.entries().len(), 1, "its archive row is not");

        let replayed_entry = entry.clone();
        press(&mut tree, move |c| {
            replay_archived_action(&replayed_entry, &details, c)
        });
        assert_eq!(
            drain_dialog_titles(&mut tree),
            vec![tr!(import_manuskript_warnings_title()).resolve_now()],
            "the list opens again"
        );
    }

    /// The whole way back, as the writer takes it: the notification log (what
    /// Settings ▸ Notifications shows, and the bell's popover) offers the
    /// archived **Details** as a live button, and pressing it reopens the list
    /// through the hook both logs install.
    #[test]
    fn the_notification_log_offers_details_and_reopens_the_list() {
        let (mut tree, _registry, archive) = window();
        let live = LiveNotice::default();
        press(&mut tree, move |c| {
            PLUME.show(c, &warnings(), &result(), &live)
        });

        let mut log = WidgetTree::new().with_theme(intui::light());
        log.add(NotificationLog::new(archive).on_action_invoked(replay_archived_action));
        log.layout(SizeProposal::exact(480.0, 360.0));
        let details = log
            .find_by_label(&tr!(import_plume_details()).resolve_now())
            .expect("the archived Details is a button, not an inert tag");
        click(&mut log, details);
        assert_eq!(
            drain_dialog_titles(&mut log),
            vec![tr!(import_plume_warnings_title()).resolve_now()]
        );
    }

    /// Two imports are two rows: the archive merges by id across sessions, so a
    /// shared id would fold one import's list into another's. On screen, the
    /// second notice takes the first one's place, so notices never pile up in
    /// the corner, and the first list stays reachable from its row.
    #[test]
    fn each_import_is_its_own_row_and_the_latest_is_on_screen() {
        let (mut tree, registry, archive) = window();
        let live = LiveNotice::default();
        let first = live.clone();
        press(&mut tree, move |c| {
            PLUME.show(c, &warnings(), &result(), &first)
        });
        let second = live.clone();
        press(&mut tree, move |c| {
            PLUME.show(c, &["Only one thing.".to_string()], &result(), &second)
        });
        assert_eq!(
            registry.live_count(),
            1,
            "the latest notice replaces the first"
        );
        assert_eq!(archive.entries().len(), 2, "and both are archived");
        let bodies: Vec<Option<String>> = (0..2)
            .filter_map(|i| archive.entries().with_item(i, |e| e.body.clone()))
            .collect();
        assert!(
            bodies.contains(&Some(warnings().join("\n"))),
            "the first list is still in the log: {bodies:?}"
        );

        // An import with nothing to report leaves the last notice alone: it is
        // about another import.
        let quiet = live.clone();
        press(&mut tree, move |c| PLUME.show(c, &[], &result(), &quiet));
        assert_eq!(registry.live_count(), 1);
    }

    /// Each importer keeps its own notice: a Manuskript import never takes a
    /// Plume import's list off the screen.
    #[test]
    fn one_importer_never_replaces_the_other_s_notice() {
        let (mut tree, registry, _archive) = window();
        let (plume, manuskript) = (LiveNotice::default(), LiveNotice::default());
        press(&mut tree, move |c| {
            PLUME.show(c, &warnings(), &result(), &plume)
        });
        press(&mut tree, move |c| {
            MANUSKRIPT.show(c, &warnings(), &result(), &manuskript)
        });
        assert_eq!(registry.live_count(), 2);
    }

    /// Only the two import notices replay; any other archived name is inert.
    #[test]
    fn an_unknown_archived_action_does_nothing() {
        let (mut tree, _registry, _archive) = window();
        let entry = NotificationEntry {
            id: 1,
            severity: BannerSeverity::Warning,
            priority: ToastPriority::Normal,
            title: "x".into(),
            body: Some("y".into()),
            actions: Vec::new(),
            timestamp: jiff::Timestamp::UNIX_EPOCH,
            group: None,
            source: None,
            read: false,
            dedup_id: None,
            updates: Vec::new(),
            route: ToastRoute::Broadcast,
        };
        let action = ArchivedAction {
            label: "Retry".into(),
            intent_name: Some("app.build.retry".into()),
            style: ArchivedActionStyle::Link,
            closes_on_invoke: true,
        };
        press(&mut tree, move |c| {
            replay_archived_action(&entry, &action, c)
        });
        assert!(drain_dialog_titles(&mut tree).is_empty());
    }
}
