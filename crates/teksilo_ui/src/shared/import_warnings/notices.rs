// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One project-import notice, worded in the interface language.
//!
//! The importers report what they had to drop or guess at as
//! [`ImportNotice`] values, which `import_management` carries across its DTO
//! as JSON (see `skrib_format::import_notice`). This is the one place they
//! become sentences. Titles, file names and the error text an operating system
//! or a parser produced are data and pass through as they are.
//!
//! A line that does not decode is shown as it is. That is what an older build
//! wrote, and dropping it would hide something the writer was told about.

use skrib_format::import_notice::{ImportNotice, ProjectContainer, TextKind};
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// The sentence for one line of an import's `warnings`.
pub(crate) fn sentence(raw: &str) -> LocalizedString {
    match ImportNotice::decode(raw) {
        Some(notice) => worded(notice),
        None => lit!(raw.to_string()),
    }
}

fn container(c: ProjectContainer) -> &'static str {
    match c {
        ProjectContainer::SingleFile => "single-file",
        ProjectContainer::Folder => "folder",
    }
}

fn text_kind(k: TextKind) -> &'static str {
    match k {
        TextKind::Scene => "scene",
        TextKind::Note => "note",
        TextKind::Synopsis => "synopsis",
        TextKind::Title => "title",
    }
}

/// Counts reach Fluent as numbers: a string argument never matches a plural
/// category and always falls through to `*[other]`.
fn n(count: u64) -> i64 {
    i64::try_from(count).unwrap_or(i64::MAX)
}

fn worded(notice: ImportNotice) -> LocalizedString {
    use ImportNotice::*;
    match notice {
        FlattenedRichTexts { count } => {
            tr!(import_notice_flattened_rich_texts(count = n(count)))
        }
        FreeTextBadges { count } => tr!(import_notice_free_text_badges(count = n(count))),
        SeparatorTextDropped { name } => tr!(import_notice_separator_text_dropped(name = name)),
        SeparatorWithoutScene { name } => {
            tr!(import_notice_separator_without_scene(name = name))
        }
        ChapterWithoutScenes { name } => tr!(import_notice_chapter_without_scenes(name = name)),
        RichTextUnconverted { error } => tr!(import_notice_rich_text_unconverted(error = error)),
        UnresolvedCrossLink { number } => {
            tr!(import_notice_unresolved_cross_link(number = number))
        }
        ReadCopy {
            container: c,
            day: Some(day),
        } => tr!(import_notice_read_copy_dated(
            container = container(c),
            day = day
        )),
        ReadCopy {
            container: c,
            day: None,
        } => tr!(import_notice_read_copy(container = container(c))),
        OriginalFormat => tr!(import_notice_original_format()),
        NewerSingleFileBeside {
            sibling,
            sibling_day,
            folder_day,
        } => tr!(import_notice_newer_single_file_beside(
            file = sibling,
            file_day = sibling_day,
            folder_day = folder_day
        )),
        PickleSkipped { member } => tr!(import_notice_pickle_skipped(file = member)),
        FileUnreadable { member, error } => {
            tr!(import_notice_file_unreadable(file = member, error = error))
        }
        NotUtf8 { member } => tr!(import_notice_not_utf8(file = member)),
        TableUnreadable { member, error } => {
            tr!(import_notice_table_unreadable(file = member, error = error))
        }
        OutlineMissing { member } => tr!(import_notice_outline_missing(file = member)),
        OutlineUnreadable { member, error } => {
            tr!(import_notice_outline_unreadable(
                file = member,
                error = error
            ))
        }
        NamedNoneRow { member, name } => {
            tr!(import_notice_named_none_row(file = member, name = name))
        }
        UnknownField { member, field } => {
            tr!(import_notice_unknown_field(file = member, field = field))
        }
        ColourlessLabels { member, count } => tr!(import_notice_colorless_labels(
            file = member,
            count = n(count)
        )),
        SettingsUnreadable { member, error } => {
            tr!(import_notice_settings_unreadable(
                file = member,
                error = error
            ))
        }
        RevisionsUnreadable { member, error } => {
            tr!(import_notice_revisions_unreadable(
                file = member,
                error = error
            ))
        }
        ForeignFiles { count, shown } => {
            let more = count.saturating_sub(shown.len() as u64);
            let files = shown.join(", ");
            if more > 0 {
                tr!(import_notice_foreign_files_more(
                    count = n(count),
                    files = files,
                    more = n(more)
                ))
            } else {
                tr!(import_notice_foreign_files(count = n(count), files = files))
            }
        }
        NotACharacterFile { member } => tr!(import_notice_not_a_character_file(file = member)),
        NotATextItem { member } => tr!(import_notice_not_a_text_item(file = member)),
        MissingBlankLine { member } => tr!(import_notice_missing_blank_line(file = member)),
        FolderWithoutMetadata { member, metadata } => {
            tr!(import_notice_folder_without_metadata(
                folder = member,
                file = metadata
            ))
        }
        RowsGivenIds { count } => tr!(import_notice_rows_given_ids(count = n(count))),
        PlotsUnreadable { member, error } => {
            tr!(import_notice_plots_unreadable(file = member, error = error))
        }
        WorldUnreadable { member, error } => {
            tr!(import_notice_world_unreadable(file = member, error = error))
        }
        WorldNotOpml { member } => tr!(import_notice_world_not_opml(file = member)),
        TextFlattened { title } => tr!(import_notice_text_flattened(title = title)),
        TextUnconverted { title, error } => {
            tr!(import_notice_text_unconverted(title = title, error = error))
        }
        TextWithoutFileName { title, text } => tr!(import_notice_text_without_file_name(
            title = title,
            kind = text_kind(text)
        )),
        UnknownLabel { title, index } => {
            tr!(import_notice_unknown_label(title = title, index = index))
        }
        UnknownStatus { title, index } => {
            tr!(import_notice_unknown_status(title = title, index = index))
        }
        UnknownPointOfView { title } => tr!(import_notice_unknown_point_of_view(title = title)),
        UnresolvedReferences { title, error } => {
            tr!(import_notice_unresolved_references(
                title = title,
                error = error
            ))
        }
        CustomIcons { count } => tr!(import_notice_custom_icons(count = n(count))),
        CharacterColours { count } => tr!(import_notice_character_colors(count = n(count))),
        VersionsKept { found, kept } => tr!(import_notice_versions_kept(
            found = n(found),
            kept = n(kept)
        )),
        VersionsUnplaced { count } => tr!(import_notice_versions_unplaced(count = n(count))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_encoded_notice_is_worded_and_its_data_kept() {
        let raw = ImportNotice::UnknownLabel {
            title: "The Mill".into(),
            index: 7,
        }
        .encode();
        let text = sentence(&raw).resolve_now();
        assert!(text.contains("The Mill") && text.contains('7'), "{text}");
        assert!(!text.contains("UnknownLabel"), "{text}");
        assert!(!text.starts_with('{'), "{text}");
    }

    #[test]
    fn a_count_selects_its_plural() {
        let one = sentence(&ImportNotice::CustomIcons { count: 1 }.encode()).resolve_now();
        let many = sentence(&ImportNotice::CustomIcons { count: 4 }.encode()).resolve_now();
        assert_ne!(
            one.replace('1', ""),
            many.replace('4', ""),
            "one and other read the same: {one} / {many}"
        );
    }

    #[test]
    fn a_line_from_an_older_build_is_shown_as_it_is() {
        assert_eq!(
            sentence("could not convert some rich text").resolve_now(),
            "could not convert some rich text"
        );
    }

    #[test]
    fn the_file_list_names_what_it_left_out() {
        let text = sentence(
            &ImportNotice::ForeignFiles {
                count: 7,
                shown: vec!["a.txt".into(), "b.png".into()],
            }
            .encode(),
        )
        .resolve_now();
        assert!(
            text.contains("a.txt, b.png") && text.contains('5'),
            "{text}"
        );
    }
}
