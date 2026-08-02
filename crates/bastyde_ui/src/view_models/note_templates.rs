// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `NoteTemplatesViewModel` — the template feature's business logic, shared by the
//! Settings ▸ Work ▸ Templates pane, the Document menu's insert submenu, and the
//! Save-as-template dialog.
//!
//! The entity is still called `NoteTemplate` — templates began as a Note-only feature and
//! the name is on disk, in `templates.ron`, in every project already saved. Renaming it
//! would be a format migration for a word. They apply to any editor now.
//!
//! It owns no state of its own beyond the Layer-A handles it composes: the reactive
//! [`WorkNoteTemplatesListModel`](crate::models::WorkNoteTemplatesListModel) and
//! [`AppIds`](crate::app_ids::AppIds) (the owner `Work` and the undo stack every mutation
//! needs).
//!
//! Plain Rust, no `#[cfg]`: the real/mock seam lives in the model below it, so this is
//! unit-testable headless and identical in both builds.

use std::path::Path;
use std::rc::Rc;

use anyhow::{Context, Result, bail};
use bastyde::data::ListModel;
use bastyde::prelude::*;

use crate::app_ids::{AppIds, HasWorkId};
use crate::models::{TemplateRow, WorkNoteTemplatesListModel, starred_first};

/// Refuse a single template file larger than this.
///
/// A template is a page or two of scaffolding. A file this size is far likelier the wrong
/// file than a template, and accepting it would be costly for the whole life of the
/// project rather than just at import: the body lives in the `WorkBundle`, which
/// `content_fingerprint` clones and RON-serialises **in full on every save, save-as and
/// backup**. `import_tags` caps its own row count for the same class of reason.
pub const MAX_TEMPLATE_BYTES: usize = 512 * 1024;

/// Refuse an import of more files than this in one go, matching the tag importer's posture.
pub const MAX_IMPORT_FILES: usize = 100;

/// What an import did, for the confirmation toast.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct TemplateImportSummary {
    /// Templates actually created.
    pub added: usize,
    /// Names that collided and were given a numeric suffix. Never dropped — see
    /// `import_note_templates_uc`'s own note on why this differs from tag import.
    pub renamed: Vec<String>,
    /// Files that could not be read, were empty, or were too big.
    pub skipped_files: Vec<String>,
}

#[derive(Clone)]
pub struct NoteTemplatesViewModel {
    list: WorkNoteTemplatesListModel,
    ids: AppIds,
}

impl NoteTemplatesViewModel {
    pub fn new(list: WorkNoteTemplatesListModel, ids: AppIds) -> Self {
        Self { list, ids }
    }

    pub fn wire(&self, ctx: &mut BuildContext) {
        self.list.wire(ctx);
    }

    pub fn refresh(&self) {
        self.list.refresh();
    }

    /// The reactive model the settings pane's `ListView` binds to.
    pub fn list_model(&self) -> ListModel<TemplateRow> {
        self.list.list_model()
    }

    /// Bumped on every refresh — for the insert menu, which observes rather than binds.
    pub fn changed_signal(&self) -> Signal<u64> {
        self.list.version_signal()
    }

    /// Rows in the writer's own order (the settings pane's order).
    pub fn rows(&self) -> Vec<TemplateRow> {
        self.list.rows()
    }

    /// Rows in the **insert menu's** order: starred first, each group keeping the writer's
    /// arrangement.
    pub fn menu_rows(&self) -> Vec<TemplateRow> {
        starred_first(&self.rows())
    }

    pub fn is_empty(&self) -> bool {
        self.list.len() == 0
    }

    /// One template by id, for the insert command.
    pub fn body_of(&self, id: u64) -> Option<String> {
        self.rows().into_iter().find(|r| r.id == id).map(|r| r.body)
    }

    /// The name a candidate collides with, ignoring case and surrounding space, or `None`.
    ///
    /// Unlike the tag palette — where a duplicate is a *warning* because two tags may
    /// legitimately share a name for a moment mid-rename — a template name is what the
    /// insert menu is picked by, so two identical entries are genuinely ambiguous. The
    /// Save-as-template dialog therefore **refuses** on a collision; inline rename in the
    /// pane still only warns, for the same transient-state reason tags give.
    pub fn duplicate_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
        self.list.colliding_name(candidate, exclude)
    }

    fn stack(&self) -> Option<u64> {
        self.ids.stack_id.get()
    }

    /// Create one template. `None` when no project is open or the name is blank.
    pub fn create(&self, name: &str, body: &str, starred: bool) -> Option<u64> {
        if name.trim().is_empty() {
            return None;
        }
        self.list
            .create(name, body, starred, self.ids.work_id.get(), self.stack())
    }

    /// Capture the given prose as a new template. The caller has already validated the
    /// name against [`Self::duplicate_name`]; this re-checks so a race between the dialog's
    /// last keystroke and its OK cannot create a duplicate.
    pub fn save_as_template(&self, name: &str, body: &str) -> Result<u64> {
        let name = name.trim();
        if name.is_empty() {
            bail!("a template needs a name");
        }
        if let Some(clash) = self.duplicate_name(name, None) {
            bail!("a template called '{clash}' already exists");
        }
        if body.len() > MAX_TEMPLATE_BYTES {
            bail!("this note is too large to save as a template");
        }
        self.create(name, body, false)
            .ok_or_else(|| anyhow::anyhow!("no project is open"))
    }

    /// Each field is a separate undo step, which is what a writer editing a settings row
    /// expects.
    pub fn rename(&self, id: u64, name: &str) {
        self.patch(id, |r| r.name = name.trim().to_string());
    }

    pub fn set_body(&self, id: u64, body: &str) {
        self.patch(id, |r| r.body = body.to_string());
    }

    pub fn set_starred(&self, id: u64, on: bool) {
        self.patch(id, |r| r.starred = on);
    }

    fn patch(&self, id: u64, f: impl FnOnce(&mut TemplateRow)) {
        let Some(mut row) = self.rows().into_iter().find(|r| r.id == id) else {
            return;
        };
        f(&mut row);
        self.list
            .update(id, &row.name, &row.body, row.starred, self.stack());
    }

    pub fn delete(&self, ids: &[u64]) {
        self.list.remove_all(ids, self.stack());
    }

    /// Nudge a row one place up (`-1`) or down (`+1`) in the writer's arrangement.
    pub fn move_by(&self, id: u64, delta: isize) {
        self.list.move_by(id, delta, self.stack());
    }

    /// Whether a row can move in the given direction — drives the button's enabled state,
    /// so the writer never gets a button that silently does nothing.
    pub fn can_move(&self, id: u64, delta: isize) -> bool {
        let rows = self.rows();
        rows.iter()
            .position(|r| r.id == id)
            .and_then(|from| crate::models::moved_index(rows.len(), from, delta))
            .is_some()
    }

    fn import_rows(&self, rows: Vec<TemplateRow>) -> TemplateImportSummary {
        // No open project ⇒ nowhere for the templates to live — the same silent no-op
        // `create` gives by returning `None`.
        let (Some(work_id), false) = (self.ids.work_id.get(), rows.is_empty()) else {
            return TemplateImportSummary::default();
        };
        // `added` is what the backend *created*, never what it was handed: the use case
        // drops a blank name (a file stem of only punctuation tidies to one), so reporting
        // the request count would tell the writer a file imported that did not.
        let outcome = self.list.import(&rows, work_id, self.stack());
        TemplateImportSummary {
            added: outcome.created,
            renamed: outcome.renamed,
            skipped_files: Vec::new(),
        }
    }

    // --- files -------------------------------------------------------------

    /// Import one or more `.md` / `.djot` files as templates, in one undo step.
    ///
    /// The template **name** comes from the file stem, tidied — a writer picking
    /// `character-sheet.md` means "Character sheet", not "character-sheet". A file that
    /// cannot be read, is empty, or is over [`MAX_TEMPLATE_BYTES`] is reported in the
    /// summary rather than failing the whole batch: one bad file among five should not
    /// cost the other four.
    pub fn import_files(&self, paths: &[std::path::PathBuf]) -> Result<TemplateImportSummary> {
        if paths.len() > MAX_IMPORT_FILES {
            bail!(
                "that is {} files — more than the {MAX_IMPORT_FILES} this can import at once",
                paths.len()
            );
        }
        let mut rows = Vec::new();
        let mut skipped_files = Vec::new();
        for path in paths {
            match read_template_file(path) {
                Ok(row) => rows.push(row),
                Err(_) => skipped_files.push(file_label(path)),
            }
        }
        let mut summary = self.import_rows(rows);
        summary.skipped_files = skipped_files;
        Ok(summary)
    }

    /// Export every template as one `.djot` file per row into `dir`.
    ///
    /// Per-file rather than a single blob because templates *are* documents — this is the
    /// exact shape `import_files` reads back, so export→import round-trips. File names go
    /// through `slugify`, so a name containing a slash or a reserved stem still lands on
    /// one safe segment.
    ///
    /// **Every name is made unique before it is written.** `slugify` is many-to-one —
    /// "Character Sheet" and "character-sheet!" both reduce to `character-sheet` — and two
    /// templates can legitimately reach here with names that collide, because the pane's
    /// inline rename only *warns* on a duplicate and still commits it. Writing both to one
    /// path would silently drop the first while the toast reported them all exported. The
    /// bundle writer avoids this by prefixing the row's `file_id`; that would be an ugly
    /// name for a file the writer is about to look at, so a `-2` suffix is appended only
    /// where one is actually needed.
    pub fn export_to_dir(&self, dir: &Path) -> Result<usize> {
        let rows = self.rows();
        let mut used: std::collections::HashSet<String> = std::collections::HashSet::new();
        for r in &rows {
            let stem = unique_stem(&skrib_format::slug::slugify(&r.name), &mut used);
            let path = dir.join(format!("{stem}.djot"));
            std::fs::write(&path, &r.body)
                .with_context(|| format!("writing {}", path.display()))?;
        }
        Ok(rows.len())
    }

    /// Apply a built-in preset, as ONE undo step.
    pub fn apply_preset(&self, preset: crate::note_templates::Preset) -> TemplateImportSummary {
        self.import_rows(preset.rows())
    }
}

/// The open Work these templates belong to — the pane's toast call sites use this to route
/// feedback to the Work it is actually about.
impl HasWorkId for NoteTemplatesViewModel {
    fn app_ids(&self) -> &AppIds {
        &self.ids
    }
}

/// The first free `<base>`, `<base>-2`, `<base>-3`… for a stem, recording what it took.
///
/// `slugify` can return an empty string (a name of only punctuation), which would write to
/// a bare `.djot` — a hidden file on Unix. `template` stands in for that.
fn unique_stem(base: &str, used: &mut std::collections::HashSet<String>) -> String {
    let base = if base.is_empty() { "template" } else { base };
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    let mut n = 2usize;
    loop {
        let candidate = format!("{base}-{n}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        n += 1;
    }
}

/// A file stem turned into a human template name: separators become spaces, and the first
/// letter is capitalised. `character-sheet.md` → `Character sheet`.
///
/// Deliberately gentle — it does **not** title-case every word, which would turn
/// `notes on the antagonist` into a headline nobody typed.
pub fn name_from_stem(stem: &str) -> String {
    let spaced: String = stem
        .chars()
        .map(|c| if c == '-' || c == '_' { ' ' } else { c })
        .collect();
    let trimmed = spaced.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut chars = trimmed.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// What to show for a file that could not be imported.
fn file_label(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// Read one template file into a row.
///
/// `.md` is converted to Djot up front rather than stored as Markdown: `Content.data` is
/// Djot end to end, so converting at import means the stored body is exactly what
/// `insert_djot` will paste, with no per-insert conversion and no chance of the two
/// dialects diverging later. Anything that is not `.md` is taken as Djot verbatim.
fn read_template_file(path: &Path) -> Result<TemplateRow> {
    let meta = std::fs::metadata(path).with_context(|| format!("reading {}", path.display()))?;
    if meta.len() as usize > MAX_TEMPLATE_BYTES {
        bail!("{} is too large for a template", path.display());
    }
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    if text.trim().is_empty() {
        bail!("{} is empty", path.display());
    }
    let is_markdown = path
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("md") || e.eq_ignore_ascii_case("markdown"));
    let body = if is_markdown {
        markdown_to_djot(&text)?
    } else {
        text
    };
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Template");
    Ok(TemplateRow {
        id: 0,
        name: name_from_stem(stem),
        body,
        starred: false,
    })
}

/// Markdown → Djot in one hop, through `text-document`'s own document model.
///
/// Not the two-hop `html_to_djot(markdown_to_html(..))`: that round-trips through Qt-shaped
/// rich-text HTML and loses structure the direct path keeps.
fn markdown_to_djot(markdown: &str) -> Result<String> {
    let doc = bastyde::text_document::TextDocument::new();
    doc.set_markdown(markdown)?.wait()?;
    Ok(doc.to_djot()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `slugify` is many-to-one, and two templates can legitimately reach export with
    /// names that collide — the pane's inline rename only warns. Writing both to one path
    /// would drop the first silently while the toast claimed both were exported.
    #[test]
    fn export_stems_are_disambiguated() {
        let mut used = std::collections::HashSet::new();
        assert_eq!(unique_stem("character-sheet", &mut used), "character-sheet");
        assert_eq!(
            unique_stem("character-sheet", &mut used),
            "character-sheet-2"
        );
        assert_eq!(
            unique_stem("character-sheet", &mut used),
            "character-sheet-3"
        );
        assert_eq!(unique_stem("location", &mut used), "location");
    }

    /// A name of only punctuation slugifies to nothing, which would write a bare `.djot`
    /// — a hidden file on Unix, and invisible to the importer that reads the folder back.
    #[test]
    fn an_empty_slug_gets_a_real_name() {
        let mut used = std::collections::HashSet::new();
        assert_eq!(unique_stem("", &mut used), "template");
        assert_eq!(unique_stem("", &mut used), "template-2");
    }

    /// Export writes one file per row even when every name collides — the count the toast
    /// reports must match what is actually on disk.
    #[test]
    fn exporting_colliding_names_writes_one_file_each() {
        let dir = tempfile::tempdir().unwrap();
        let mut used = std::collections::HashSet::new();
        // Three names that all reduce to the same slug.
        for name in ["Character Sheet", "character-sheet", "CHARACTER SHEET!"] {
            let stem = unique_stem(&skrib_format::slug::slugify(name), &mut used);
            std::fs::write(dir.path().join(format!("{stem}.djot")), name).unwrap();
        }
        let written: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            written.len(),
            3,
            "three templates must produce three files, got {written:?}"
        );
    }

    #[test]
    fn a_file_stem_becomes_a_readable_name() {
        assert_eq!(name_from_stem("character-sheet"), "Character sheet");
        assert_eq!(name_from_stem("character_sheet"), "Character sheet");
        assert_eq!(name_from_stem("Location"), "Location");
        assert_eq!(
            name_from_stem("notes  on   the antagonist"),
            "Notes on the antagonist"
        );
    }

    /// Only the first letter is raised — a stem is not a headline.
    #[test]
    fn a_stem_is_not_title_cased() {
        assert_eq!(
            name_from_stem("notes-on-the-antagonist"),
            "Notes on the antagonist"
        );
    }

    #[test]
    fn an_empty_stem_yields_an_empty_name() {
        assert_eq!(name_from_stem(""), "");
        assert_eq!(name_from_stem("   "), "");
    }

    #[test]
    fn markdown_becomes_djot_with_its_structure_intact() {
        let djot =
            markdown_to_djot("# Title\n\nSome **bold** text.\n\n- one\n- two\n").expect("convert");
        assert!(djot.contains("# Title"), "heading survives: {djot}");
        assert!(
            djot.contains("one") && djot.contains("two"),
            "list survives: {djot}"
        );
    }

    /// `.djot` input is stored verbatim — no conversion, no normalisation.
    #[test]
    fn a_djot_file_is_read_verbatim() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("beat-sheet.djot");
        let body = "# Beat sheet\n\n- Goal:\n- Conflict:\n";
        std::fs::write(&path, body).unwrap();

        let row = read_template_file(&path).expect("read");
        assert_eq!(row.body, body);
        assert_eq!(row.name, "Beat sheet");
        assert!(!row.starred);
    }

    #[test]
    fn an_empty_file_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("blank.djot");
        std::fs::write(&path, "   \n\n").unwrap();
        assert!(read_template_file(&path).is_err());
    }

    #[test]
    fn an_oversized_file_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("huge.djot");
        std::fs::write(&path, "x".repeat(MAX_TEMPLATE_BYTES + 1)).unwrap();
        assert!(read_template_file(&path).is_err());
    }

    #[test]
    fn a_markdown_file_is_converted_on_the_way_in() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("character-sheet.md");
        std::fs::write(&path, "# Character sheet\n\n- Name:\n").unwrap();

        let row = read_template_file(&path).expect("read");
        assert_eq!(row.name, "Character sheet");
        assert!(row.body.contains("Character sheet"));
    }
}
