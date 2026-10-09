// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ImportReportViewModel`: the Scrivener import report dialog's business logic.
//!
//! One per dialog, made by the window that shows it from a finished import's
//! [`ImportReport`]. It holds what the dialog lists as data models, and does what its
//! buttons and links ask: **Copy report** (the whole report as plain text, in the language
//! Skribisto shows itself in, then *Report copied* for a screen reader), **Open now** (the
//! new project in a window of its own, as the result notice's button does), and a link from
//! an item to the row of the new project that holds it.
//!
//! **A link needs the project open in this window.** The report names each item by where
//! the writer knew it in Scrivener, and the import gave each one a row of the new project;
//! only a window showing that project can open the row. The window says whether it does by
//! handing over an [`ItemFinder`]: with none, the items are plain text and the dialog says
//! where they become links.

use std::rc::Rc;

use teksilo::data::ListModel;
use teksilo::i18n::LocalizedString;
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use uuid::Uuid;

use super::report::{ImportReport, ReportGroup};
use crate::intents::AppIntent;

/// Finds the row of the window's open project that has this uid: its store id and its
/// title. Given by a window showing the project the report is about, and only then.
pub(crate) type ItemFinder = Rc<dyn Fn(Uuid) -> Option<(u64, String)>>;

#[derive(Clone)]
pub struct ImportReportViewModel {
    inner: Rc<Inner>,
}

struct Inner {
    report: ImportReport,
    groups: ListModel<ReportGroup>,
    choices: ListModel<LocalizedString>,
    /// What **Copy report** last did, said once for a screen reader; empty before.
    copy_status: Signal<String>,
    find_item: Option<ItemFinder>,
}

impl ImportReportViewModel {
    pub(crate) fn new(report: ImportReport, find_item: Option<ItemFinder>) -> Self {
        let groups = ListModel::from_vec(report.groups.clone());
        let choices = ListModel::from_vec(report.choices.clone());
        Self {
            inner: Rc::new(Inner {
                report,
                groups,
                choices,
                copy_status: Signal::new(String::new()),
                find_item,
            }),
        }
    }

    pub(crate) fn report(&self) -> &ImportReport {
        &self.inner.report
    }

    /// The groups that have a line, in their fixed order.
    pub(crate) fn groups(&self) -> ListModel<ReportGroup> {
        self.inner.groups.clone()
    }

    /// What the writer chose, one sentence each.
    pub(crate) fn choices(&self) -> ListModel<LocalizedString> {
        self.inner.choices.clone()
    }

    /// Whether the writer's choices are known: the import was started from a wizard of
    /// this session.
    pub(crate) fn has_choices(&self) -> bool {
        !self.inner.choices.is_empty()
    }

    /// The project the import read, as a line, when it is known.
    pub(crate) fn read_from_line(&self) -> Option<LocalizedString> {
        self.inner
            .report
            .source
            .clone()
            .map(|path| tr!(import_scrivener_report_from(path = path)))
    }

    /// The project the import wrote, as a line.
    pub(crate) fn written_to_line(&self) -> LocalizedString {
        tr!(import_scrivener_report_to(
            path = self.inner.report.output_path.clone()
        ))
    }

    /// Whether an item with a row in the new project opens it from here: this window shows
    /// the project.
    pub(crate) fn links_items(&self) -> bool {
        self.inner.find_item.is_some()
    }

    /// Whether the dialog says where its items open: a line names an item that has a row
    /// in the new project, and this window does not show it.
    pub(crate) fn says_where_items_open(&self) -> bool {
        !self.links_items()
            && self
                .inner
                .report
                .groups
                .iter()
                .flat_map(|g| g.lines.iter())
                .flat_map(|l| l.items.iter())
                .any(|i| i.uid.is_some())
    }

    /// Whether `uid` names a row of the project this window shows.
    pub(crate) fn can_open(&self, uid: Uuid) -> bool {
        self.inner
            .find_item
            .as_ref()
            .is_some_and(|find| find(uid).is_some())
    }

    /// Open the row `uid` names in this window's editor, closing the report so it shows.
    pub(crate) fn open_item(&self, ctx: &mut EventContext, uid: Uuid) {
        let Some((item_id, title)) = self.inner.find_item.as_ref().and_then(|find| find(uid))
        else {
            return;
        };
        ctx.send_intent(AppIntent::OpenItem { item_id, title });
        ctx.dismiss_modal();
    }

    /// **Open now**: the new project in a window of its own, as the result notice's button
    /// opens it, and the report closed.
    pub(crate) fn open_now(&self, ctx: &mut EventContext) {
        ctx.send_intent(AppIntent::OpenWorkPathInNewWindow {
            path: self.inner.report.output_path.clone(),
        });
        ctx.dismiss_modal();
    }

    /// **Copy report**: the whole report on the clipboard, in the language Skribisto shows
    /// itself in.
    pub(crate) fn copy(&self, ctx: &mut EventContext) {
        let clipboard = ctx.app_state::<ClipboardHandle>().cloned();
        self.copy_to(clipboard.as_ref());
    }

    /// Put the report on `clipboard`, and say whether it went: `false`, and said so, with
    /// no clipboard or one that refused it.
    pub(crate) fn copy_to(&self, clipboard: Option<&ClipboardHandle>) -> bool {
        let copied = clipboard.is_some_and(|c| c.set_text(&self.inner.report.text()).is_ok());
        let said = if copied {
            tr!(import_scrivener_report_copied())
        } else {
            tr!(import_scrivener_report_copy_failed())
        };
        self.inner.copy_status.set(said.resolve_now());
        copied
    }

    /// What **Copy report** last did, for the line a screen reader hears.
    pub(crate) fn copy_status(&self) -> Signal<String> {
        self.inner.copy_status.clone()
    }
}

#[cfg(test)]
mod tests;
