// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;

use frontend::import_management::{
    ScrivenerArg, ScrivenerFinding, ScrivenerFindingFamily, ScrivenerPlace, ScrivenerReportGroup,
    ScrivenerTotals,
};
use teksilo::platform::clipboard::MemoryClipboard;

use crate::import_scrivener::ScrivenerImportResult;

/// The row the import made for the one finding below.
const ROW: Uuid = Uuid::from_u128(42);

fn report() -> ImportReport {
    ImportReport::from_result(&ScrivenerImportResult {
        output_path: "/books/The Lighthouse.skrib".into(),
        research_folder: String::new(),
        totals: ScrivenerTotals::Empty,
        findings: vec![ScrivenerFinding::Reported {
            key: "scrivener-prose-heading-repeats-title".into(),
            family: ScrivenerFindingFamily::Prose,
            group: ScrivenerReportGroup::CheckThese,
            place: ScrivenerPlace::Text,
            item: 3,
            binder_path: vec!["Draft".into(), "Arrival".into()],
            item_uid: Some(ROW),
            args: Vec::<ScrivenerArg>::new(),
        }],
        record: None,
    })
}

/// **Copy report** puts the whole report on the clipboard and says so; with no clipboard,
/// it says it could not.
#[test]
fn copy_puts_the_report_on_the_clipboard_and_says_so() {
    crate::test_support::with_real_messages(|| {
        let vm = ImportReportViewModel::new(report(), None);
        assert_eq!(vm.copy_status().get(), "");
        let clipboard = ClipboardHandle::new(MemoryClipboard::new());
        assert!(vm.copy_to(Some(&clipboard)));
        assert_eq!(clipboard.get_text().ok(), Some(vm.report().text()));
        assert_eq!(vm.copy_status().get(), "Report copied");
        assert!(!vm.copy_to(None));
        assert_eq!(vm.copy_status().get(), "The report could not be copied.");
    });
}

/// An item opens its row only in a window that shows the new project, and only a row
/// that project holds.
#[test]
fn an_item_opens_only_where_its_project_is_shown() {
    let unlinked = ImportReportViewModel::new(report(), None);
    assert!(!unlinked.links_items());
    assert!(!unlinked.can_open(ROW));
    assert!(
        unlinked.says_where_items_open(),
        "the dialog says where its items open"
    );

    let finder: ItemFinder = Rc::new(|uid| (uid == ROW).then(|| (7, "Arrival".to_string())));
    let linked = ImportReportViewModel::new(report(), Some(finder));
    assert!(linked.links_items());
    assert!(linked.can_open(ROW));
    assert!(!linked.can_open(Uuid::from_u128(43)));
    assert!(!linked.says_where_items_open());
}

/// The lines the dialog shows above the groups.
#[test]
fn the_dialog_says_where_the_import_wrote() {
    crate::test_support::with_real_messages(|| {
        let vm = ImportReportViewModel::new(report(), None);
        assert_eq!(
            vm.written_to_line().resolve_now(),
            "Written to /books/The Lighthouse.skrib"
        );
        assert!(
            vm.read_from_line().is_none(),
            "no wizard of this session started it"
        );
        assert!(!vm.has_choices());
        assert_eq!(vm.groups().len(), 1);
    });
}
