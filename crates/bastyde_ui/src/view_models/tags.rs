// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TagsViewModel` — the tag feature's business logic, shared by the Settings ▸ Tags pane,
//! the Inspector's tag section, and (later) the chip popover.
//!
//! It owns no state of its own beyond the Layer-A handles it composes: the reactive
//! [`WorkTagsListModel`] (the palette plus its writes)
//! and [`AppIds`] (the owner `Work` and undo stack every mutation
//! needs).
//!
//! Plain Rust, no `#[cfg]`: the real/mock seam lives in the model below it, so this is
//! unit-testable headless and identical in both builds.

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context, Result};
use bastyde::data::ListModel;
use bastyde::prelude::*;

use crate::app_ids::{AppIds, HasWorkId};
use crate::models::{TagRow, WorkTagsListModel, name_key};
use crate::tags::Preset;

/// Refuse an import larger than this. A tag palette is a hand-curated set of at most a few
/// dozen; a CSV this big is far likelier a wrong file than a palette, and importing it
/// would bury the palette and balloon the undo step.
const MAX_IMPORT_ROWS: usize = 500;

/// The CSV header, and the field order `parse_csv`/`format_csv` agree on.
///
/// No `text_color` column: it is not stored (it is derived from `color` at paint time), and
/// a column the writer may fill but which is silently discarded is worse than no column.
const CSV_HEADER: [&str; 4] = ["name", "color", "details", "discoverable"];

/// Fallback colour for an imported row with a blank or unparseable one, so a malformed CSV
/// still yields usable tags rather than invisible ones.
const FALLBACK_COLOR: &str = "#607d8b";

/// What an import did, for the confirmation toast.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TagImportSummary {
    /// Tags actually created.
    pub added: usize,
    /// Rows skipped because the name was already in the palette (or repeated in the file).
    pub duplicates: usize,
    /// Rows skipped because they were blank or unparseable.
    pub malformed: usize,
}

#[derive(Clone)]
pub struct TagsViewModel {
    list: WorkTagsListModel,
    ids: AppIds,
}

impl TagsViewModel {
    pub fn new(list: WorkTagsListModel, ids: AppIds) -> Self {
        Self { list, ids }
    }

    /// Wire the held Layer-A handle's event subscriptions (from `App::build`, after
    /// the LoadWork seed is registered — see `WorkTagsListModel::wire`).
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.list.wire(ctx);
    }

    /// Re-read the open Work's palette. Used when a project becomes live outside a
    /// `BinderTag` mutation (e.g. attach of an already-open Work).
    pub fn refresh(&self) {
        self.list.refresh();
    }

    /// The reactive palette to bind (the pane wraps it in a `SortFilterListModel`).
    pub fn list_model(&self) -> ListModel<TagRow> {
        self.list.list_model()
    }

    /// Bumped on every change — bind this where the model itself is not bound.
    pub fn changed_signal(&self) -> Signal<u64> {
        self.list.version_signal()
    }

    /// id → row, for chip renderers. See `WorkTagsListModel::lookup_signal`.
    pub fn lookup_signal(&self) -> Signal<Rc<HashMap<u64, TagRow>>> {
        self.list.lookup_signal()
    }

    /// The palette, sorted.
    pub fn rows(&self) -> Vec<TagRow> {
        self.list.rows()
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    /// The name a candidate collides with, ignoring case and surrounding space, or `None`.
    ///
    /// Duplicate names are **allowed** — the backend does not care and must not be made to.
    /// This drives a warning, never a refusal: two tags may legitimately share a name for a
    /// moment while the writer is renaming one of them. `exclude` is the tag being renamed,
    /// which never collides with itself.
    pub fn duplicate_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
        self.list.colliding_name(candidate, exclude)
    }

    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }

    /// Create one tag. `None` when no project is open or the name is blank.
    pub fn create(
        &self,
        name: &str,
        color: &str,
        details: &str,
        discoverable: bool,
    ) -> Option<u64> {
        if name.trim().is_empty() {
            return None;
        }
        self.list.create(
            name,
            color,
            details,
            discoverable,
            self.ids.work_id.get(),
            self.stack(),
        )
    }

    /// Patch one field, leaving the rest of the row as it is. Each is a separate undo step,
    /// which is what a writer editing a settings row expects.
    pub fn rename(&self, id: u64, name: &str) {
        self.patch(id, |r| r.name = name.trim().to_string());
    }

    pub fn recolor(&self, id: u64, color: &str) {
        self.patch(id, |r| r.color = color.to_string());
    }

    pub fn set_details(&self, id: u64, details: &str) {
        self.patch(id, |r| r.details = details.to_string());
    }

    pub fn set_discoverable(&self, id: u64, on: bool) {
        self.patch(id, |r| r.discoverable = on);
    }

    fn patch(&self, id: u64, f: impl FnOnce(&mut TagRow)) {
        let Some(mut row) = self.rows().into_iter().find(|r| r.id == id) else {
            return;
        };
        f(&mut row);
        self.list.update(
            id,
            &row.name,
            &row.color,
            &row.details,
            row.discoverable,
            self.stack(),
        );
    }

    /// Delete tags. The generated `remove_multi` scrubs the item junction too, so items
    /// carrying them simply lose them; undo restores both the rows and the assignments.
    pub fn delete(&self, ids: &[u64]) {
        self.list.remove_all(ids, self.stack());
    }

    /// Apply a named preset as ONE undo step, adding only the names not already present.
    pub fn apply_preset(&self, preset: Preset) -> TagImportSummary {
        self.import_rows(preset.rows())
    }

    /// Every preset, for the "Apply a preset…" menu.
    pub fn presets(&self) -> [Preset; 5] {
        Preset::ALL
    }

    fn import_rows(&self, rows: Vec<TagRow>) -> TagImportSummary {
        // No open project ⇒ nowhere for the palette to live — same silent no-op
        // as `create` returning `None` above.
        let (Some(work_id), false) = (self.ids.work_id.get(), rows.is_empty()) else {
            return TagImportSummary::default();
        };
        let requested = rows.len();
        let skipped = self.list.import(&rows, work_id, self.stack());
        TagImportSummary {
            added: requested.saturating_sub(skipped.len()),
            duplicates: skipped.len(),
            malformed: 0,
        }
    }

    // --- CSV ---------------------------------------------------------------

    /// Import a palette from a `.csv`. One undo step for the whole file.
    pub fn import_from(&self, path: &Path) -> Result<TagImportSummary> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let (rows, malformed) = parse_csv(&text)?;
        let mut summary = self.import_rows(rows);
        summary.malformed = malformed;
        Ok(summary)
    }

    /// Export the palette to a `.csv`. Returns how many rows were written.
    pub fn export_to(&self, path: &Path) -> Result<usize> {
        let rows = self.rows();
        let text = format_csv(&rows)?;
        std::fs::write(path, text).with_context(|| format!("writing {}", path.display()))?;
        Ok(rows.len())
    }
}

/// The open Work this palette belongs to — the pane's toast call sites use
/// this (via [`HasWorkId::work_id`]) to route feedback ("tag added", "preset
/// applied", …) to the Work it is actually about (see
/// `crate::toast_scope::ToastWorkExt`).
impl HasWorkId for TagsViewModel {
    fn app_ids(&self) -> &AppIds {
        &self.ids
    }
}

/// Serialize a palette to RFC-4180 CSV with a header row.
pub fn format_csv(rows: &[TagRow]) -> Result<String> {
    let mut w = csv::Writer::from_writer(Vec::new());
    w.write_record(CSV_HEADER).context("writing CSV header")?;
    for r in rows {
        w.write_record([
            r.name.as_str(),
            r.color.as_str(),
            r.details.as_str(),
            if r.discoverable { "true" } else { "false" },
        ])
        .context("writing CSV row")?;
    }
    let bytes = w.into_inner().context("finishing CSV")?;
    String::from_utf8(bytes).context("CSV is not UTF-8")
}

/// Parse a palette CSV, returning the usable rows and a count of the malformed ones.
///
/// Deliberately lenient about everything except size: a writer may well edit this file in a
/// spreadsheet, and one bad row should cost that row, not the import. A missing `details`
/// or `discoverable` column is fine (they default), and an unrecognised `discoverable`
/// value reads as `false` rather than failing the file.
pub fn parse_csv(text: &str) -> Result<(Vec<TagRow>, usize)> {
    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(true)
        .from_reader(text.as_bytes());

    let mut rows = Vec::new();
    let mut malformed = 0usize;
    let mut seen = std::collections::HashSet::new();

    for record in reader.records() {
        if rows.len() >= MAX_IMPORT_ROWS {
            anyhow::bail!(
                "this file has more than {MAX_IMPORT_ROWS} rows — that is not a tag palette"
            );
        }
        let Ok(record) = record else {
            malformed += 1;
            continue;
        };
        let name = record.get(0).unwrap_or_default().trim();
        if name.is_empty() {
            malformed += 1;
            continue;
        }
        // Dedup within the file so a repeated row is reported once as a duplicate rather
        // than being handed to the backend to skip silently.
        if !seen.insert(name_key(name)) {
            malformed += 1;
            continue;
        }
        let color = record.get(1).unwrap_or_default().trim();
        rows.push(TagRow {
            id: 0,
            name: name.to_string(),
            color: if is_hex_color(color) {
                color.to_string()
            } else {
                FALLBACK_COLOR.to_string()
            },
            details: record.get(2).unwrap_or_default().trim().to_string(),
            discoverable: matches!(
                record
                    .get(3)
                    .unwrap_or_default()
                    .trim()
                    .to_lowercase()
                    .as_str(),
                "true" | "yes" | "1"
            ),
        });
    }
    Ok((rows, malformed))
}

fn is_hex_color(s: &str) -> bool {
    s.len() == 7 && s.starts_with('#') && s[1..].chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str, color: &str, details: &str, discoverable: bool) -> TagRow {
        TagRow {
            id: 0,
            name: name.into(),
            color: color.into(),
            details: details.into(),
            discoverable,
        }
    }

    #[test]
    fn csv_round_trips() {
        let rows = vec![
            row("status/draft", "#607d8b", "Not yet revised", false),
            row("character", "#2980b9", "", true),
        ];
        let text = format_csv(&rows).unwrap();
        let (back, malformed) = parse_csv(&text).unwrap();
        assert_eq!(malformed, 0);
        assert_eq!(back, rows);
    }

    /// The round trip **through a real file**, not just through a String.
    ///
    /// `csv_round_trips` above covers `format_csv` against `parse_csv` in memory, which is
    /// where the interesting parsing lives. What it cannot see is `export_to` itself: the
    /// path handling, the write, and the fact that what lands on disk is what
    /// `import_from`'s reader will later be handed. That glue had no coverage at all, and it
    /// is the half a writer actually exercises — the UI path around it goes through a native
    /// file dialog, which no automation probe can drive, so this is the furthest out the
    /// export can be checked at all.
    ///
    /// Written against `format_csv` + a real `std::fs` read rather than `TagsViewModel`,
    /// because constructing the view-model needs an `AppContext` and the thing under test is
    /// the file, not the palette plumbing.
    #[test]
    fn an_exported_file_is_readable_back_from_disk() {
        let rows = vec![
            row("status/draft", "#607d8b", "Not yet revised", false),
            row("Character, minor", "#2980b9", "with, commas", true),
            row("très soigné", "#000000", "accents survive UTF-8", true),
        ];

        let dir = std::env::temp_dir().join(format!("skrib-csv-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("palette.csv");
        std::fs::write(&path, format_csv(&rows).unwrap()).unwrap();

        let text = std::fs::read_to_string(&path).expect("the exported file must be readable");
        assert!(
            text.starts_with("name,color,details,discoverable"),
            "an externally-edited file is the point, so the header has to be the documented \
             one; got {:?}",
            text.lines().next()
        );
        let (back, malformed) = parse_csv(&text).unwrap();
        assert_eq!(
            malformed, 0,
            "a file we just wrote must not parse as malformed"
        );
        assert_eq!(back, rows, "what lands on disk is what comes back");

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A tag name may contain the delimiter; RFC-4180 quoting is why CSV was chosen over a
    /// hand-rolled delimited format.
    #[test]
    fn a_name_containing_a_comma_survives() {
        let rows = vec![row("Character, minor", "#2980b9", "with, commas", true)];
        let text = format_csv(&rows).unwrap();
        let (back, _) = parse_csv(&text).unwrap();
        assert_eq!(back, rows);
    }

    #[test]
    fn a_blank_name_is_malformed_not_imported() {
        let (rows, malformed) =
            parse_csv("name,color,details,discoverable\n  ,#f00,,true\ngood,#0f0,,false\n")
                .unwrap();
        assert_eq!(malformed, 1);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "good");
    }

    #[test]
    fn a_repeated_name_within_one_file_is_counted_once() {
        let (rows, malformed) = parse_csv("name,color\nplace,#00a\nPLACE,#0a0\n").unwrap();
        assert_eq!(
            rows.len(),
            1,
            "the second is a duplicate, case-insensitively"
        );
        assert_eq!(malformed, 1);
    }

    /// Missing optional columns must not fail the file — a spreadsheet export may well
    /// carry only what the writer cared about.
    #[test]
    fn missing_optional_columns_default() {
        let (rows, malformed) = parse_csv("name,color\nplace,#00aabb\n").unwrap();
        assert_eq!(malformed, 0);
        assert_eq!(rows[0].details, "");
        assert!(!rows[0].discoverable);
        assert_eq!(rows[0].color, "#00aabb");
    }

    #[test]
    fn a_bad_colour_falls_back_rather_than_failing_the_row() {
        let (rows, _) = parse_csv("name,color\nplace,octarine\n").unwrap();
        assert_eq!(
            rows[0].color, FALLBACK_COLOR,
            "an unusable colour must still yield a visible tag"
        );
    }

    #[test]
    fn discoverable_accepts_the_obvious_spellings() {
        let (rows, _) = parse_csv(
            "name,color,details,discoverable\na,#000000,,true\nb,#000000,,YES\nc,#000000,,1\nd,#000000,,nope\n",
        )
        .unwrap();
        assert!(rows[0].discoverable && rows[1].discoverable && rows[2].discoverable);
        assert!(!rows[3].discoverable, "anything else reads as false");
    }

    #[test]
    fn an_absurdly_large_file_is_refused() {
        let mut text = String::from("name,color\n");
        for i in 0..(MAX_IMPORT_ROWS + 10) {
            text.push_str(&format!("tag{i},#000000\n"));
        }
        assert!(parse_csv(&text).is_err());
    }

    /// The header names the four stored fields — and deliberately not a fifth for text
    /// colour, which is derived rather than persisted.
    #[test]
    fn the_header_has_no_text_colour_column() {
        assert_eq!(CSV_HEADER, ["name", "color", "details", "discoverable"]);
        let text = format_csv(&[row("a", "#000000", "", false)]).unwrap();
        assert!(!text.contains("text_color"));
    }
}
