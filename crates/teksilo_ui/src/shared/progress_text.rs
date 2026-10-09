// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `NN % · phase` body of a long operation's progress toast.
//!
//! A long operation reports progress as `(percent, message)`, and the backend
//! cannot translate: it runs below the UI, with no Fluent bundle. So the
//! message is one of two things, told apart here and nowhere else:
//!
//! - a **phase code** the backend emits on purpose (`import.outline`,
//!   `export.writing`, …), mapped to a translated label by [`phase_label`];
//! - **data** — the title of the entry being converted, a source path — shown
//!   as it is, because it is the writer's own text.
//!
//! `completed` is the long-operation manager's end-of-run marker, not a phase
//! the writer needs named, so it maps to nothing. The backup scheduler has its
//! own codes (`backup.*`) and its own mapper for them, since one of them
//! carries arguments.

use teksilo::prelude::*;

/// The phase codes the importers and the exporter report. Kept as constants on
/// the UI side only to document the vocabulary; the backend crates spell the
/// same strings, and the `every_backend_phase_code_is_mapped` test ties the two.
pub(crate) const IMPORT_OPENING: &str = "import.opening";
pub(crate) const IMPORT_OUTLINE: &str = "import.outline";
pub(crate) const IMPORT_CONVERTING: &str = "import.converting";
pub(crate) const IMPORT_WRITING: &str = "import.writing";
pub(crate) const IMPORT_DONE: &str = "import.done";
pub(crate) const IMPORT_STRUCTURE: &str = "import.structure";
pub(crate) const EXPORT_START: &str = "export.start";
/// `skrib_format::tree_read::gather`, which export (and save and backup, whose
/// progress is not worded) reads the project through.
pub(crate) const PROJECT_READING: &str = "project.reading";
pub(crate) const EXPORT_WRITING: &str = "export.writing";
const COMPLETED: &str = "completed";

/// The translated label for a phase code, `Some(None)` for a code that is
/// deliberately silent, and `None` when `message` is not a code at all.
fn phase_label(message: &str) -> Option<Option<LocalizedString>> {
    Some(Some(match message {
        IMPORT_OPENING => tr!(progress_import_opening()),
        IMPORT_OUTLINE => tr!(progress_import_outline()),
        IMPORT_CONVERTING => tr!(progress_import_converting()),
        IMPORT_WRITING => tr!(progress_import_writing()),
        IMPORT_DONE => tr!(progress_import_done()),
        IMPORT_STRUCTURE => tr!(progress_import_structure()),
        EXPORT_START => tr!(progress_export_start()),
        PROJECT_READING => tr!(progress_project_reading()),
        EXPORT_WRITING => tr!(progress_export_writing()),
        COMPLETED | "" => return Some(None),
        _ => return None,
    }))
}

/// The translated name of the phase `message` reports, or `None` when it is data
/// (the title of the entry being converted) or a code that is deliberately
/// silent: what a screen reader hears once per phase, never the data that ticks
/// under it.
pub(crate) fn phase_name(message: &str) -> Option<LocalizedString> {
    phase_label(message).flatten()
}

/// The text a progress row or toast shows for `message`, without the
/// percentage: a phase code translated, data as it is, nothing for a silent
/// code.
pub(crate) fn progress_label(message: &str) -> Option<LocalizedString> {
    match phase_label(message) {
        Some(label) => label,
        None => Some(lit!(message.to_string())),
    }
}

/// The toast body: the rounded percentage, then the phase when there is one.
/// The percent sign and the separator come from the catalogue, since where
/// they go (and whether a space precedes `%`) differs by language.
pub(crate) fn progress_body(percent: f32, message: &str) -> LocalizedString {
    percent_body(percent, progress_label(message))
}

/// [`progress_body`] for a caller that has already resolved its own label
/// (the backup scheduler, whose `backup.*` codes carry arguments).
pub(crate) fn percent_body(percent: f32, label: Option<LocalizedString>) -> LocalizedString {
    let percent = percent.clamp(0.0, 100.0).round() as i64;
    match label {
        Some(label) => tr!(progress_percent_with_label(
            percent = percent,
            label = label.resolve_now()
        )),
        None => tr!(progress_percent(percent = percent)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_passes_through_and_completed_is_silent() {
        assert_eq!(
            progress_label("Chapter One").map(|l| l.resolve_now()),
            Some("Chapter One".to_string())
        );
        assert!(progress_label(COMPLETED).is_none());
        assert!(progress_label("").is_none());
    }

    #[test]
    fn every_phase_code_is_translated_not_echoed() {
        for code in [
            IMPORT_OPENING,
            IMPORT_OUTLINE,
            IMPORT_CONVERTING,
            IMPORT_WRITING,
            IMPORT_DONE,
            IMPORT_STRUCTURE,
            EXPORT_START,
            EXPORT_WRITING,
            PROJECT_READING,
        ] {
            let label = progress_label(code).expect("a phase has a label");
            assert_ne!(label.resolve_now(), code, "{code} reached the screen raw");
        }
    }

    /// The backend crates spell these codes as plain literals (they cannot
    /// depend on the UI); a code renamed on one side alone would silently
    /// show the raw code in the toast. Grep the sources for every literal the
    /// mapper knows.
    #[test]
    fn every_backend_phase_code_is_mapped() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let sources = [
            (IMPORT_OPENING, "plume_import/src/lib.rs"),
            (IMPORT_OUTLINE, "plume_import/src/lib.rs"),
            (IMPORT_CONVERTING, "plume_import/src/lib.rs"),
            (IMPORT_WRITING, "plume_import/src/lib.rs"),
            (IMPORT_DONE, "plume_import/src/lib.rs"),
            (IMPORT_OPENING, "manuskript_import/src/lib.rs"),
            (IMPORT_OUTLINE, "manuskript_import/src/lib.rs"),
            (IMPORT_CONVERTING, "manuskript_import/src/lib.rs"),
            (IMPORT_WRITING, "manuskript_import/src/lib.rs"),
            (IMPORT_DONE, "manuskript_import/src/lib.rs"),
            (IMPORT_OPENING, "scrivener_import/src/convert.rs"),
            (IMPORT_OUTLINE, "scrivener_import/src/convert.rs"),
            (IMPORT_CONVERTING, "scrivener_import/src/convert.rs"),
            (IMPORT_WRITING, "scrivener_import/src/convert.rs"),
            (IMPORT_DONE, "scrivener_import/src/convert.rs"),
            (
                IMPORT_STRUCTURE,
                "import_management/src/use_cases/analyze_document_import_uc.rs",
            ),
            (
                EXPORT_START,
                "export_management/src/use_cases/export_work_uc.rs",
            ),
            (
                EXPORT_WRITING,
                "export_management/src/use_cases/export_work_uc.rs",
            ),
            (PROJECT_READING, "skrib_format/src/tree_read.rs"),
        ];
        for (code, file) in sources {
            let text =
                std::fs::read_to_string(root.join(file)).unwrap_or_else(|e| panic!("{file}: {e}"));
            assert!(
                text.contains(&format!("\"{code}\"")),
                "{file} no longer reports `{code}`"
            );
        }
    }

    #[test]
    fn the_body_carries_the_rounded_percentage() {
        let body = progress_body(41.6, COMPLETED).resolve_now();
        assert!(body.contains("42"), "{body}");
        let body = progress_body(10.0, "Chapter One").resolve_now();
        assert!(
            body.contains("10") && body.contains("Chapter One"),
            "{body}"
        );
    }
}
