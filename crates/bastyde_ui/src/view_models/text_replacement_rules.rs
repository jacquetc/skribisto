// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `TextReplacementRulesViewModel` — the per-project custom lexicon feature's
//! business logic ("btw → by the way"), shared by the Settings ▸ Text
//! replacements pane and (from the trigger-replacement engine) the editor
//! session that watches for a fired trigger.
//!
//! It owns no state of its own beyond the Layer-A handles it composes: the
//! reactive [`TextReplacementRuleListModel`](crate::models::TextReplacementRuleListModel)
//! (the lexicon + its collection writes), the shared
//! [`SingleWork`](crate::singles::SingleWork) handle (the per-project master
//! switch, `Work.custom_replacement_rules_enabled`), and
//! [`AppIds`](crate::app_ids::AppIds) (the owner `Work` + undo stack every
//! mutation needs).
//!
//! Plain Rust, no `#[cfg]`: the real/mock seam lives in the model below it, so
//! this is unit-testable headless and identical in both builds.

use std::path::Path;

use anyhow::{Context, Result};
use bastyde::data::ListModel;
use bastyde::prelude::*;

use crate::app_ids::AppIds;
use crate::models::{TextReplacementRuleListModel, TextReplacementRuleRow, trigger_key};
use crate::singles::SingleWork;

/// Refuse an import larger than this. A hand-curated lexicon of shorthand
/// expansions is realistically a few hundred entries; a file this big is far
/// likelier a wrong file than a lexicon, and importing it would bury the
/// lexicon and balloon the undo step.
const MAX_IMPORT_ROWS: usize = 2_000;

/// The CSV header, and the field order `parse_csv`/`format_csv` agree on.
const CSV_HEADER: [&str; 3] = ["trigger", "replacement", "enabled"];

/// What an import did, for the confirmation toast.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TextReplacementImportSummary {
    /// Rules actually created.
    pub added: usize,
    /// Rows skipped because the trigger was already in the lexicon (or repeated in the file).
    pub duplicates: usize,
    /// Rows skipped because they were blank or unparseable.
    pub malformed: usize,
}

#[derive(Clone)]
pub struct TextReplacementRulesViewModel {
    list: TextReplacementRuleListModel,
    work: SingleWork,
    ids: AppIds,
}

impl TextReplacementRulesViewModel {
    pub fn new(list: TextReplacementRuleListModel, work: SingleWork, ids: AppIds) -> Self {
        Self { list, work, ids }
    }

    /// Wire the held Layer-A handle's event subscriptions (once, from `App::build`).
    pub fn wire(&self, ctx: &mut BuildContext) {
        self.list.wire(ctx);
    }

    /// The open Work this lexicon belongs to — the pane's toast call sites
    /// use this to route feedback ("rule added", "imported", …) to the Work
    /// it is actually about (see `crate::toast_scope::ToastWorkExt`).
    pub fn work_id(&self) -> Option<u64> {
        self.ids.work_id.get()
    }

    /// The reactive lexicon to bind (the pane wraps it in a `SortFilterListModel`).
    pub fn list_model(&self) -> ListModel<TextReplacementRuleRow> {
        self.list.list_model()
    }

    /// Bumped on every change — bind this where the model itself is not bound.
    pub fn changed_signal(&self) -> Signal<u64> {
        self.list.version_signal()
    }

    /// The lexicon, sorted.
    pub fn rows(&self) -> Vec<TextReplacementRuleRow> {
        self.list.rows()
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    // ── The per-project master switch ───────────────────────────────────────

    /// Whether the custom lexicon is active for the open project. Off by
    /// default for every new project — a writer opts a specific project into
    /// custom replacements explicitly, never inherits it.
    pub fn enabled_signal(&self) -> Signal<bool> {
        self.work.custom_replacement_rules_enabled()
    }

    /// Flip the master switch, and persist. A no-op when already in that
    /// state — the pane drives this from an effect on a mirrored `Signal<bool>`,
    /// which fires on every rebuild, and an unconditional write would queue a
    /// pointless undo entry and a disk save each time Settings is opened.
    pub fn set_enabled(&self, on: bool) {
        if self.work.custom_replacement_rules_enabled().get() == on {
            return;
        }
        self.work.set_custom_replacement_rules_enabled(on);
        self.work.save(self.ids.stack_id.get());
    }

    // ── Rules ────────────────────────────────────────────────────────────────

    /// The trigger a candidate collides with, ignoring case and surrounding
    /// space, excluding `exclude`.
    ///
    /// Duplicates are **refused** at creation (unlike tag names): two rules for
    /// the same trigger could never both fire, so a collision here is always a
    /// mistake, not a legitimate transient state.
    pub fn duplicate_trigger(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
        self.list.colliding_trigger(candidate, exclude)
    }

    /// Whether `trigger` is a valid, non-duplicate addition (drives the Add
    /// row's enabled state / validation).
    pub fn can_add(&self, trigger: &str) -> bool {
        let t = trigger.trim();
        !t.is_empty() && self.duplicate_trigger(t, None).is_none()
    }

    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }

    /// Create one rule. `None` when no project is open, the trigger is blank,
    /// or it collides with an existing one.
    pub fn create(&self, trigger: &str, replacement: &str, enabled: bool) -> Option<u64> {
        if !self.can_add(trigger) {
            return None;
        }
        self.list.create(
            trigger,
            replacement,
            enabled,
            self.ids.work_id.get(),
            self.stack(),
        )
    }

    /// Patch one field, leaving the rest of the row as it is. Each is a
    /// separate undo step, which is what a writer editing a settings row
    /// expects.
    ///
    /// Retyping a trigger onto one that already exists is **refused**, and the
    /// caller is told so, unlike the tag palette this row is modelled on. Two
    /// tags may share a name — it is only ever cosmetic. Two rules sharing a
    /// trigger is incoherent: the engine drops all but one when it compiles the
    /// lexicon, so the writer would be left with a rule visibly in the list that
    /// silently never fires. A blank trigger is refused for the same reason.
    pub fn set_trigger(&self, id: u64, trigger: &str) -> bool {
        let t = trigger.trim();
        if t.is_empty() || self.duplicate_trigger(t, Some(id)).is_some() {
            return false;
        }
        self.patch(id, |r| r.trigger = t.to_string());
        true
    }

    pub fn set_replacement(&self, id: u64, replacement: &str) {
        self.patch(id, |r| r.replacement = replacement.to_string());
    }

    pub fn set_rule_enabled(&self, id: u64, on: bool) {
        self.patch(id, |r| r.enabled = on);
    }

    fn patch(&self, id: u64, f: impl FnOnce(&mut TextReplacementRuleRow)) {
        let Some(mut row) = self.rows().into_iter().find(|r| r.id == id) else {
            return;
        };
        f(&mut row);
        self.list.update(
            id,
            &row.trigger,
            &row.replacement,
            row.enabled,
            self.stack(),
        );
    }

    /// Delete rules in one undoable step. Missing ids are a safe no-op (a
    /// stale toast Undo after a manual delete).
    pub fn delete(&self, ids: &[u64]) {
        self.list.remove_all(ids, self.stack());
    }

    // ── CSV import/export ───────────────────────────────────────────────────

    /// Import a lexicon from a `.csv`. One undo step for the whole file.
    pub fn import_from(&self, path: &Path) -> Result<TextReplacementImportSummary> {
        let text =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let (rows, malformed) = parse_csv(&text)?;
        if rows.is_empty() {
            return Ok(TextReplacementImportSummary {
                malformed,
                ..Default::default()
            });
        }
        let requested = rows.len();
        let skipped = self
            .list
            .import(&rows, self.ids.work_id.get(), self.stack());
        Ok(TextReplacementImportSummary {
            added: requested.saturating_sub(skipped.len()),
            duplicates: skipped.len(),
            malformed,
        })
    }

    /// Export the lexicon to a `.csv`. Returns how many rows were written.
    pub fn export_to(&self, path: &Path) -> Result<usize> {
        let rows = self.rows();
        let text = format_csv(&rows)?;
        std::fs::write(path, text).with_context(|| format!("writing {}", path.display()))?;
        Ok(rows.len())
    }
}

/// Serialize a lexicon to RFC-4180 CSV with a header row.
pub fn format_csv(rows: &[TextReplacementRuleRow]) -> Result<String> {
    let mut w = csv::Writer::from_writer(Vec::new());
    w.write_record(CSV_HEADER).context("writing CSV header")?;
    for r in rows {
        w.write_record([
            r.trigger.as_str(),
            r.replacement.as_str(),
            if r.enabled { "true" } else { "false" },
        ])
        .context("writing CSV row")?;
    }
    let bytes = w.into_inner().context("finishing CSV")?;
    String::from_utf8(bytes).context("CSV is not UTF-8")
}

/// Parse a lexicon CSV, returning the usable rows and a count of the
/// malformed ones.
///
/// Deliberately lenient about everything except size: a writer may well edit
/// this file in a spreadsheet, and one bad row should cost that row, not the
/// import.
///
/// A **missing or empty** `enabled` column reads as `true` — the opposite
/// default from tags' `discoverable`, so that a hand-written two-column
/// `trigger,replacement` file imports as a working lexicon rather than as rules
/// that all need switching on. A **present but unrecognised** value reads as
/// `false`, exactly as tags' does: the writer wrote something there, and
/// guessing "yes" from text we could not parse is how a rule the writer meant
/// to disable ends up firing.
pub fn parse_csv(text: &str) -> Result<(Vec<TextReplacementRuleRow>, usize)> {
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
                "this file has more than {MAX_IMPORT_ROWS} rows — that is not a replacement lexicon"
            );
        }
        let Ok(record) = record else {
            malformed += 1;
            continue;
        };
        let trigger = record.get(0).unwrap_or_default().trim();
        if trigger.is_empty() {
            malformed += 1;
            continue;
        }
        // Dedup within the file so a repeated row is reported once as a duplicate
        // rather than being handed to the backend to skip silently.
        if !seen.insert(trigger_key(trigger)) {
            malformed += 1;
            continue;
        }
        let replacement = record.get(1).unwrap_or_default();
        let enabled = match record.get(2).map(|s| s.trim().to_lowercase()) {
            None => true,
            Some(s) if s.is_empty() => true,
            Some(s) => matches!(s.as_str(), "true" | "yes" | "1"),
        };
        rows.push(TextReplacementRuleRow {
            id: 0,
            trigger: trigger.to_string(),
            replacement: replacement.to_string(),
            enabled,
        });
    }
    Ok((rows, malformed))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(trigger: &str, replacement: &str, enabled: bool) -> TextReplacementRuleRow {
        TextReplacementRuleRow {
            id: 0,
            trigger: trigger.into(),
            replacement: replacement.into(),
            enabled,
        }
    }

    #[test]
    fn csv_round_trips() {
        let rows = vec![row("--", "—", true), row("btw", "by the way", false)];
        let text = format_csv(&rows).unwrap();
        let (back, malformed) = parse_csv(&text).unwrap();
        assert_eq!(malformed, 0);
        assert_eq!(back, rows);
    }

    #[test]
    fn a_blank_trigger_is_malformed_not_imported() {
        let (rows, malformed) =
            parse_csv("trigger,replacement,enabled\n  ,x,true\nbtw,by the way,true\n").unwrap();
        assert_eq!(malformed, 1);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].trigger, "btw");
    }

    #[test]
    fn a_repeated_trigger_within_one_file_is_counted_once() {
        let (rows, malformed) =
            parse_csv("trigger,replacement\nbtw,by the way\nBTW,by the way2\n").unwrap();
        assert_eq!(
            rows.len(),
            1,
            "the second is a duplicate, case-insensitively"
        );
        assert_eq!(malformed, 1);
    }

    /// Missing `enabled` must not fail the file — and unlike tags' `discoverable`, it
    /// defaults to `true`: a rule the writer bothered to list is one they mean to use.
    #[test]
    fn missing_enabled_column_defaults_to_true() {
        let (rows, malformed) = parse_csv("trigger,replacement\nbtw,by the way\n").unwrap();
        assert_eq!(malformed, 0);
        assert!(rows[0].enabled);
    }

    #[test]
    fn enabled_accepts_the_obvious_spellings() {
        let (rows, _) = parse_csv(
            "trigger,replacement,enabled\na,x,true\nb,x,YES\nc,x,1\nd,x,nope\ne,x,false\n",
        )
        .unwrap();
        assert!(rows[0].enabled && rows[1].enabled && rows[2].enabled);
        assert!(!rows[3].enabled && !rows[4].enabled);
    }

    #[test]
    fn an_absurdly_large_file_is_refused() {
        let mut text = String::from("trigger,replacement\n");
        for i in 0..(MAX_IMPORT_ROWS + 10) {
            text.push_str(&format!("t{i},r{i}\n"));
        }
        assert!(parse_csv(&text).is_err());
    }

    /// A replacement may contain the delimiter (e.g. a comma in an expanded phrase);
    /// RFC-4180 quoting is why CSV was chosen over a hand-rolled delimited format.
    #[test]
    fn a_replacement_containing_a_comma_survives() {
        let rows = vec![row("addr", "123 Main St, Springfield", true)];
        let text = format_csv(&rows).unwrap();
        let (back, _) = parse_csv(&text).unwrap();
        assert_eq!(back, rows);
    }

    #[test]
    fn the_header_matches_the_documented_columns() {
        assert_eq!(CSV_HEADER, ["trigger", "replacement", "enabled"]);
    }
}

/// The collision rules, exercised through a real view-model over the fabricated
/// list model — the mock `imp` needs no backend, so the actual `create` /
/// `set_trigger` paths run here rather than a re-stated pure helper.
#[cfg(all(test, feature = "mocks"))]
mod collision_tests {
    use std::rc::Rc;

    use frontend::AppContext;

    use super::*;
    use crate::app_ids::AppIds;
    use crate::models::TextReplacementRuleListModel;
    use crate::singles::SingleWork;

    /// A view-model over the mock lexicon, which ships `--` , `btw` and `teh`.
    fn vm() -> TextReplacementRulesViewModel {
        let ctx = Rc::new(AppContext::new());
        let ids = AppIds::new();
        TextReplacementRulesViewModel::new(
            TextReplacementRuleListModel::new(ctx.clone(), ids.clone()),
            SingleWork::new(ctx),
            ids,
        )
    }

    fn trigger_of(vm: &TextReplacementRulesViewModel, id: u64) -> String {
        vm.rows()
            .into_iter()
            .find(|r| r.id == id)
            .map(|r| r.trigger)
            .expect("the row must still exist")
    }

    fn id_of(vm: &TextReplacementRulesViewModel, trigger: &str) -> u64 {
        vm.rows()
            .into_iter()
            .find(|r| r.trigger == trigger)
            .map(|r| r.id)
            .expect("the fixture must carry this rule")
    }

    /// The bug this guards: renaming one rule's trigger onto another's used to
    /// succeed, leaving two rules the engine can never both honour — it drops all
    /// but one at compile time, so the loser sits in the list looking active and
    /// silently never fires.
    #[test]
    fn renaming_a_trigger_onto_an_existing_one_is_refused() {
        let vm = vm();
        let teh = id_of(&vm, "teh");
        assert!(!vm.set_trigger(teh, "btw"), "the write must be refused");
        assert_eq!(
            trigger_of(&vm, teh),
            "teh",
            "the row must keep the trigger it had"
        );
    }

    /// Case-insensitively, matching how the engine keys its lexicon.
    #[test]
    fn renaming_onto_a_differently_cased_trigger_is_refused_too() {
        let vm = vm();
        let teh = id_of(&vm, "teh");
        assert!(!vm.set_trigger(teh, "BTW"));
        assert_eq!(trigger_of(&vm, teh), "teh");
    }

    #[test]
    fn renaming_to_a_free_trigger_succeeds() {
        let vm = vm();
        let teh = id_of(&vm, "teh");
        assert!(vm.set_trigger(teh, "hte"));
        assert_eq!(trigger_of(&vm, teh), "hte");
    }

    /// A rule never collides with itself, so re-committing an unchanged field
    /// (which a blur does on every pass through the row) must not be refused.
    #[test]
    fn a_rule_does_not_collide_with_itself() {
        let vm = vm();
        let btw = id_of(&vm, "btw");
        assert!(vm.set_trigger(btw, "btw"));
        assert_eq!(trigger_of(&vm, btw), "btw");
    }

    /// A blank trigger could never fire, so it is refused rather than stored.
    #[test]
    fn blanking_a_trigger_is_refused() {
        let vm = vm();
        let btw = id_of(&vm, "btw");
        assert!(!vm.set_trigger(btw, "   "));
        assert_eq!(trigger_of(&vm, btw), "btw");
    }

    /// The creation half of the same invariant.
    #[test]
    fn creating_a_duplicate_trigger_is_refused() {
        let vm = vm();
        let before = vm.rows().len();
        assert_eq!(vm.create("BTW", "Something else", true), None);
        assert!(!vm.can_add("btw"));
        assert_eq!(vm.rows().len(), before, "nothing may have been created");
    }

    #[test]
    fn creating_a_novel_trigger_succeeds() {
        let vm = vm();
        let before = vm.rows().len();
        assert!(vm.create("phd", "PhD", true).is_some());
        assert_eq!(vm.rows().len(), before + 1);
    }
}
