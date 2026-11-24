// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::*;
use crate::test_support::with_shipped_messages;
use teksilo::platform::clipboard::MemoryClipboard;

/// The running edition's form address, which a report link starts with.
fn form() -> String {
    crate::identity::links()
        .translation_form
        .expect("the community edition takes translation reports")
}

fn row_titles(vm: &TranslationReportViewModel) -> Vec<String> {
    (0..vm.rows().len())
        .filter_map(|i| vm.rows().with_item(i, |r| r.title.clone()))
        .collect()
}

/// The row reading exactly `title`.
fn row(vm: &TranslationReportViewModel, title: &str) -> usize {
    row_titles(vm)
        .iter()
        .position(|t| t == title)
        .unwrap_or_else(|| panic!("no row {title:?} in {:?}", row_titles(vm)))
}

fn suggestion(vm: &TranslationReportViewModel) -> String {
    vm.suggestion().to_plain_text().unwrap_or_default()
}

#[test]
fn the_language_follows_the_interface_until_the_writer_picks_one() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        assert_eq!(vm.language(), "fr-FR");
        vm.choose_language("de-DE");
        assert_eq!(vm.language(), "de-DE");
        vm.choose_language("xx-YY");
        assert_eq!(vm.language(), "de-DE", "an unshipped tag is ignored");
    });
}

#[test]
fn a_language_is_offered_by_its_own_name() {
    let label = TranslationReportViewModel::language_label("fr-FR");
    assert!(label.contains("fr-FR"), "{label}");
    assert!(label.to_lowercase().contains("français"), "{label}");
    assert_eq!(
        TranslationReportViewModel::languages().len(),
        SUPPORTED_LOCALES.len()
    );
}

/// fr-FR uses "Annuler" for several messages: one row, which says how many places
/// and which English words they translate.
#[test]
fn identical_values_are_one_row_that_counts_its_places() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("annuler"));
        let i = row(&vm, "Annuler");
        let subtitle = vm
            .rows()
            .with_item(i, |r| r.subtitle.clone())
            .unwrap_or_default();
        assert!(
            subtitle.contains("Cancel") || subtitle.contains("Undo"),
            "{subtitle}"
        );
    });
}

#[test]
fn nothing_is_listed_before_anything_is_typed() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        assert_eq!(vm.rows().len(), 0);
        vm.query().set(String::from("   "));
        assert_eq!(vm.rows().len(), 0);
    });
}

/// Picking prefills the suggestion with the value as written: the mnemonic and the
/// placeholders stay, so the writer edits rather than retypes.
#[test]
fn picking_a_text_prefills_the_suggestion_as_written() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        let i = row(&vm, "Signaler un problème…");
        vm.pick_row(i);
        assert!(vm.has_target());
        let card = vm.card().expect("a card for the pick");
        assert_eq!(card.text, "Signaler un problème…");
        assert!(card.english.is_some(), "{card:?}");
        assert!(suggestion(&vm).contains('&'), "{:?}", suggestion(&vm));
        assert!(
            !vm.suggestion_hints().is_empty(),
            "a value with a mnemonic gets the hint about it"
        );
    });
}

#[test]
fn the_report_waits_for_a_kind_and_a_suggestion() {
    with_shipped_messages("fr-FR", || {
        let _serial = crate::identity::lock_for_test();
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        vm.pick_row(row(&vm, "Signaler un problème…"));
        assert!(!vm.can_continue().get(), "no kind yet");
        vm.choose_kind(ProblemKind::Unnatural);
        assert!(vm.can_continue().get());
        let _ = vm.suggestion().set_plain_text("  ");
        vm.edited();
        assert!(
            !vm.can_continue().get(),
            "an empty suggestion is not a report"
        );
        let _ = vm.suggestion().set_plain_text("&Signaler un souci…");
        vm.edited();
        let report = vm.report().expect("complete");
        let Target::Message { id, catalogue, .. } = &report.target else {
            panic!("{report:?}");
        };
        assert_eq!(id, "menu-help-report");
        assert_eq!(*catalogue, "app");
        assert_eq!(report.locale, "fr-FR");
        assert_eq!(report.suggestion, "&Signaler un souci…");
    });
}

/// Reporting every place of a group names the others in the block's `also:`.
#[test]
fn reporting_every_place_names_them_all() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("annuler"));
        vm.pick_row(row(&vm, "Annuler"));
        let card = vm.card().expect("a card");
        assert!(card.places.len() > 2, "{:?}", card.places);
        assert_eq!(card.places[0].0, PlaceChoice::All);
        vm.choose_place(PlaceChoice::All);
        vm.choose_kind(ProblemKind::Inconsistent);
        let report = vm.report().expect("complete");
        let Target::Message { also, .. } = &report.target else {
            panic!("{report:?}");
        };
        assert_eq!(also.len(), card.places.len() - 2);
        vm.choose_place(PlaceChoice::One(1));
        let Some(Target::Message { also, .. }) = vm.report().map(|r| r.target) else {
            panic!("still complete");
        };
        assert!(also.is_empty());
    });
}

#[test]
fn a_composed_label_is_reported_with_both_messages() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        let fr = LocaleIndex::build("fr-FR").expect("shipped");
        let trash = fr
            .find(Catalogue::App, "undo-target-trash")
            .map(|i| fr.entries[i].shown().to_string())
            .expect("indexed");
        let sentence = format!("Annuler {trash}");
        vm.query().set(String::from(&sentence));
        vm.pick_row(row(&vm, &sentence));
        let card = vm.card().expect("a card");
        assert_eq!(card.text, sentence);
        assert!(
            card.note.is_some(),
            "a composed text says the fix may go in either"
        );
        assert!(suggestion(&vm).contains("menu-edit-undo-target ="));
        assert!(suggestion(&vm).contains("undo-target-trash ="));
        vm.choose_kind(ProblemKind::Spelling);
        let report = vm.report().expect("complete");
        assert_eq!(report.current, sentence);
        let Target::Message { composed, .. } = &report.target else {
            panic!("{report:?}");
        };
        assert_eq!(composed.as_deref(), Some("undo-target-trash"));
    });
}

/// A writer may type the English they know; the row then says what the translation
/// reads, and picking it reports the translation.
#[test]
fn an_english_query_reports_the_translation() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Report a Problem…"));
        let i = row(&vm, "Report a Problem…");
        vm.pick_row(i);
        let card = vm.card().expect("a card");
        assert_eq!(card.id, "menu-help-report");
        assert_eq!(card.text, "Signaler un problème…");
    });
}

#[test]
fn a_seeded_name_says_what_a_fix_changes() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        let fr = LocaleIndex::build("fr-FR").expect("shipped");
        let draft = fr
            .find(Catalogue::App, "status-draft")
            .map(|i| fr.entries[i].shown().to_string())
            .expect("indexed");
        vm.query().set(String::from(&draft));
        vm.pick_row(row(&vm, &draft));
        let card = vm.card().expect("a card");
        assert!(card.note.is_some(), "{card:?}");
    });
}

#[test]
fn a_text_not_found_is_reported_by_where_it_was_seen() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.cannot_find();
        assert!(vm.is_not_found());
        assert!(vm.card().is_none());
        vm.choose_kind(ProblemKind::NotTranslated);
        let _ = vm.suggestion().set_plain_text("Couverture");
        vm.edited();
        assert!(!vm.can_continue().get(), "where it was seen is needed");
        vm.seen_at().set("La couverture d’un EPUB".to_string());
        assert!(vm.can_continue().get());
        let report = vm.report().expect("complete");
        assert!(matches!(report.target, Target::NotFound { .. }));
    });
}

/// The window opened from a help page lists that page before anything is typed.
#[test]
fn a_help_page_scope_lists_its_paragraphs() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.open_for(Some(Scope::HelpPage("help-getting-started".to_string())));
        assert!(vm.rows().len() > 2, "{:?}", row_titles(&vm));
        vm.pick_row(0);
        let report = {
            vm.choose_kind(ProblemKind::Spelling);
            vm.report().expect("complete")
        };
        let Target::Message {
            catalogue, file, ..
        } = &report.target
        else {
            panic!("{report:?}");
        };
        assert_eq!(*catalogue, "help");
        assert_eq!(file, "getting-started.djot");
        vm.open_for(None);
        assert_eq!(vm.rows().len(), 0);
    });
}

#[test]
fn a_long_report_goes_through_the_clipboard() {
    with_shipped_messages("fr-FR", || {
        let _serial = crate::identity::lock_for_test();
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        vm.pick_row(row(&vm, "Signaler un problème…"));
        vm.choose_kind(ProblemKind::WrongMeaning);
        let _ = vm.suggestion().set_plain_text(&"語".repeat(1_000));
        vm.edited();
        let plan = vm.prepare_continue().expect("complete");
        assert!(plan.link.url.starts_with(&form()));
        assert!(plan.clipboard.is_some());

        let clipboard = ClipboardHandle::new(MemoryClipboard::new());
        vm.continue_with(&plan, Some(&clipboard), |_| Ok(()));
        assert_eq!(vm.phase().get(), Phase::Continued { copied: true });
        assert!(
            clipboard
                .get_text()
                .unwrap_or_default()
                .contains(&"語".repeat(1_000))
        );

        vm.continue_with(&plan, Some(&clipboard), |_| Err("no browser".to_string()));
        assert_eq!(
            vm.phase().get(),
            Phase::OpenFailed("no browser".to_string())
        );
        vm.copy_link(Some(&clipboard));
        assert_eq!(clipboard.get_text().ok(), Some(plan.link.url.clone()));
        assert_eq!(vm.notice().get(), Some(Notice::LinkCopied));
    });
}

#[test]
fn a_short_report_leaves_the_clipboard_alone() {
    with_shipped_messages("fr-FR", || {
        let _serial = crate::identity::lock_for_test();
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        vm.pick_row(row(&vm, "Signaler un problème…"));
        vm.choose_kind(ProblemKind::WrongMeaning);
        let plan = vm.prepare_continue().expect("complete");
        assert!(plan.clipboard.is_none());
        let clipboard = ClipboardHandle::new(MemoryClipboard::new());
        let _ = clipboard.set_text("the writer's own clipboard");
        vm.continue_with(&plan, Some(&clipboard), |_| Ok(()));
        assert_eq!(vm.phase().get(), Phase::Continued { copied: false });
        assert_eq!(
            clipboard.get_text().ok().as_deref(),
            Some("the writer's own clipboard")
        );

        vm.copy_text(Some(&clipboard));
        assert_eq!(vm.notice().get(), Some(Notice::TextCopied));
        assert!(
            clipboard
                .get_text()
                .unwrap_or_default()
                .contains("menu-help-report")
        );
        vm.copy_text(None);
        assert_eq!(vm.notice().get(), Some(Notice::CopyFailed));
    });
}

/// Continue on Discord puts the whole report on the clipboard, short or long, since
/// Discord takes no prefilled message, and opens the project's translation channel.
#[test]
fn discord_gets_the_whole_report_through_the_clipboard() {
    with_shipped_messages("fr-FR", || {
        let _serial = crate::identity::lock_for_test();
        let channel = TranslationReportViewModel::discord_channel()
            .expect("the community edition has a translation channel");
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        vm.pick_row(row(&vm, "Signaler un problème…"));
        vm.choose_kind(ProblemKind::WrongMeaning);
        let clipboard = ClipboardHandle::new(MemoryClipboard::new());
        let _ = clipboard.set_text("the writer's own clipboard");

        let mut opened = None;
        vm.discord_with(Some(&clipboard), |url| {
            opened = Some(url.to_string());
            Ok(())
        });
        assert_eq!(opened.as_deref(), Some(channel.as_str()));
        assert_eq!(vm.phase().get(), Phase::OnDiscord { copied: true });
        assert_eq!(vm.notice().get(), None);
        let pasted = clipboard.get_text().unwrap_or_default();
        assert!(pasted.contains("menu-help-report"), "{pasted}");

        // Copy link gives the channel, not the GitHub form.
        vm.copy_link(Some(&clipboard));
        assert_eq!(clipboard.get_text().ok(), Some(channel.clone()));

        // A clipboard that refuses is said, beside the channel that still opened.
        vm.discord_with(None, |_| Ok(()));
        assert_eq!(vm.phase().get(), Phase::OnDiscord { copied: false });
        assert_eq!(vm.notice().get(), Some(Notice::CopyFailed));

        vm.discord_with(Some(&clipboard), |_| Err("no browser".to_string()));
        assert_eq!(
            vm.phase().get(),
            Phase::OpenFailed("no browser".to_string())
        );
    });
}

/// An incomplete report, or an edition with no channel, opens nothing and leaves the
/// clipboard alone.
#[test]
fn discord_waits_for_a_report_and_a_channel() {
    with_shipped_messages("fr-FR", || {
        let _serial = crate::identity::lock_for_test();
        let clipboard = ClipboardHandle::new(MemoryClipboard::new());
        let _ = clipboard.set_text("the writer's own clipboard");
        let untouched = |clipboard: &ClipboardHandle| {
            clipboard.get_text().ok().as_deref() == Some("the writer's own clipboard")
        };

        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        vm.pick_row(row(&vm, "Signaler un problème…"));
        vm.discord_with(Some(&clipboard), |_| panic!("no kind yet: nothing to open"));
        assert_eq!(vm.phase().get(), Phase::Editing);
        assert!(untouched(&clipboard));

        let _h = crate::identity::register(crate::identity::AppIdentity::new(
            "eu",
            "acme-writer",
            "Acme Writer",
        ));
        assert!(TranslationReportViewModel::discord_channel().is_none());
        vm.choose_kind(ProblemKind::WrongMeaning);
        assert!(vm.report().is_some());
        vm.discord_with(Some(&clipboard), |_| panic!("no channel: nothing to open"));
        assert_eq!(vm.phase().get(), Phase::Editing);
        assert!(untouched(&clipboard));
    });
}

/// An edition that declares no translation form gets no link to open.
#[test]
fn an_edition_without_a_form_cannot_continue() {
    with_shipped_messages("fr-FR", || {
        let _serial = crate::identity::lock_for_test();
        let _h = crate::identity::register(crate::identity::AppIdentity::new(
            "eu",
            "acme-writer",
            "Acme Writer",
        ));
        let vm = TranslationReportViewModel::new();
        vm.query().set(String::from("Signaler un problème…"));
        vm.pick_row(row(&vm, "Signaler un problème…"));
        vm.choose_kind(ProblemKind::WrongMeaning);
        assert!(vm.report().is_some());
        assert!(vm.prepare_continue().is_none());
    });
}

#[test]
fn reporting_another_keeps_only_the_language() {
    with_shipped_messages("fr-FR", || {
        let vm = TranslationReportViewModel::new();
        vm.choose_language("it-IT");
        vm.query().set(String::from("annulla"));
        vm.pick_row(0);
        vm.choose_kind(ProblemKind::Other);
        vm.report_another();
        assert_eq!(vm.language(), "it-IT");
        assert_eq!(vm.query().get(), "");
        assert!(!vm.has_target());
        assert_eq!(vm.kind().get(), None);
        assert_eq!(suggestion(&vm), "");
        assert_eq!(vm.phase().get(), Phase::Editing);
    });
}

#[test]
fn every_location_hint_resolves() {
    with_shipped_messages("en-US", || {
        let index = LocaleIndex::build("en-US").expect("shipped");
        for entry in &index.entries {
            let hint = location_hint(entry);
            assert!(
                !hint.is_empty() && !hint.starts_with("translation-report-"),
                "{hint}"
            );
        }
    });
}
