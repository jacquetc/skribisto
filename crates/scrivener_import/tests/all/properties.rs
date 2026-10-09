// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Statements about every input, not about one fixture: a binder of any shape
//! reads back row for row, and no archive, however it is made, takes the
//! analysis down.

use proptest::prelude::*;
use scrivener_fixtures::hostile::{RawEntry, zip as raw_zip};
use scrivener_fixtures::{Generation, Item, Kind, Platform, Spec};
use scrivener_import::Analysis;

use crate::support::{run, temp, write, write_file};

/// A binder of any shape: depths that each step down at most one level.
fn binder() -> impl Strategy<Value = Vec<(u16, u8, bool)>> {
    prop::collection::vec((0u16..4, 0u8..3, any::<bool>()), 1..40).prop_map(|raw| {
        let mut previous: Option<u16> = None;
        raw.into_iter()
            .map(|(step, kind, text)| {
                let allowed = previous.map_or(0, |p| p + 1);
                let depth = step.min(allowed);
                previous = Some(depth);
                (depth, kind, text)
            })
            .collect()
    })
}

fn generation() -> impl Strategy<Value = (Generation, Platform)> {
    prop_oneof![
        Just((Generation::Format20, Platform::Mac)),
        Just((Generation::Format20, Platform::Windows)),
        Just((Generation::Format15, Platform::Windows)),
        Just((Generation::Format10, Platform::Mac)),
        Just((Generation::IosStub, Platform::Ios)),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 24, ..ProptestConfig::default() })]

    /// Any binder the generator writes, in any generation, reads back with the
    /// same depths, kinds, titles and texts, in order.
    #[test]
    fn any_binder_reads_back_row_for_row(rows in binder(), (g, p) in generation()) {
        let mut spec = Spec::novel(g, p);
        spec.template_folder = None;
        spec.items = vec![Item::new(0, Kind::Draft, "Manuscript")];
        for (i, (depth, kind, text)) in rows.iter().enumerate() {
            let kind = match kind {
                0 => Kind::Text,
                1 => Kind::Folder,
                _ => Kind::Text,
            };
            let mut item = Item::new(depth + 1, kind, &format!("Item {i}"));
            if *text {
                item = item.with_text(i, 1);
            }
            spec.items.push(item);
        }
        let dir = temp();
        let folder = write(&spec, dir.path());
        let analysis = run(&folder);
        let Analysis::Read(read) = analysis else {
            return Err(TestCaseError::fail(format!("not read: {}", crate::support::summary(&analysis))));
        };
        prop_assert!(read.project.check_shape().is_ok());
        prop_assert_eq!(read.project.binder.len(), spec.items.len());
        for (row, item) in read.project.binder.iter().zip(&spec.items) {
            prop_assert_eq!(row.depth, item.depth);
            prop_assert_eq!(&row.title, &item.title);
            prop_assert_eq!(row.payload.text.is_some(), !item.text.is_empty());
        }
    }

    /// An archive of any names and bytes (climbing names, links, duplicates,
    /// nothing like a project) ends in an answer, never a panic.
    #[test]
    fn no_archive_takes_the_analysis_down(
        members in prop::collection::vec(
            (
                prop::sample::select(vec![
                    "P.scriv/P.scrivx", "P.scriv/Files/version.txt", "P.scriv/Files/Data/x",
                    "P.scriv/Files/Docs/1.rtf", "../x", "/abs", "P.scriv/Files/search.indexes",
                    "P.scriv/Files/binder.autosave", "P.scriv/Mobile/binder.mob", "P.scriv/Files",
                    "Q.scrivx", "P.scriv/Files/Data/0BADF00D-0000-4000-8000-000000000001/content.rtf",
                ]),
                prop::collection::vec(any::<u8>(), 0..64),
                any::<bool>(),
            ),
            0..12,
        )
    ) {
        let entries: Vec<RawEntry> = members
            .into_iter()
            .map(|(name, data, link)| {
                let mut entry = RawEntry::file(name, &data);
                if link {
                    entry.unix_mode = Some(0o120_777);
                }
                entry
            })
            .collect();
        let dir = temp();
        let path = write_file(&raw_zip(&entries), dir.path(), "any.zip");
        let analysis = run(&path);
        if let Analysis::Read(read) = &analysis {
            prop_assert!(read.project.check_shape().is_ok());
        }
    }
}
