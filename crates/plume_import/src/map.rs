// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Map the parsed, version-neutral Plume model into a [`skrib_format::WorkBundle`]
//! at the newest `.skrib` format version.
//!
//! Two binders: a **Manuscript** (the book/act/chapter/scene tree) and a **Story
//! Bible** (the Attendance characters/places/items). The story bible is built
//! first so cross-links (`attend`/`pov`) from manuscript nodes can resolve to the
//! note items. Every constructed item is filtered through
//! `skribisto_model::content_allowed`, so an item that violates the writing-model
//! constraint matrix can never be produced.

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};

use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::entities::ContentRole;
use skrib_format::{
    BinderFile, BinderItemFile, BinderTagFile, BundledBinder, BundledItem, DictWordFile,
    FORMAT_VERSION, InlineContent, ProjectManifest, ProseRef, ShapeTag, WorkBundle, WorkFile,
    binder_dir_name, html_to_djot, new_unique_id, prose_file_name, prose_kind, prose_relpath,
};
use skribisto_model::SubRoleExt;
use skribisto_model::content_allowed;
use skribisto_model::scene_break::{self, SceneBreakTier};

use super::model::{PlumeAttendance, PlumeInfo, PlumeKind, PlumeNode, PlumeObj, PlumeTree};
use super::source::PlumeSource;

/// The result of mapping: the bundle plus the counts/warnings the UI reports.
pub struct Mapped {
    pub bundle: WorkBundle,
    pub imported_items: u64,
    pub skipped_trashed: u64,
    pub warnings: Vec<String>,
}

/// Binder ordinals (also drive the on-disk `binders/NN-slug` directory names, so
/// they must match the order the binders appear in `bundle.binders`).
const MANUSCRIPT_INDEX: usize = 0;
const STORY_BIBLE_INDEX: usize = 1;

/// Build the `.skrib` bundle from the parsed Plume model.
///
/// `report(percent, label)` is called as each node/attendance object is mapped
/// (the walk occupies the 20‑88 % band; the phases before/after own the rest),
/// and `cancel` is polled per node so a large project can be aborted mid-walk —
/// once set, the recursion unwinds fast and `plume::import` writes nothing.
#[allow(clippy::too_many_arguments)]
pub fn build_bundle(
    tree: &PlumeTree,
    attendance: &PlumeAttendance,
    info: &PlumeInfo,
    dict_words: &[String],
    source: &PlumeSource,
    manuscript_name: &str,
    story_bible_name: &str,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Mapped {
    // Total units of mapping work = every tree node + every attendance group/obj.
    // Trashed subtrees are visited at their root only, so `done` may finish a
    // little short of `total`; the "Writing…" phase (90 %) picks up from there.
    let total = count_tree_nodes(&tree.roots)
        + attendance
            .groups
            .iter()
            .map(|g| 1 + g.objs.len() as u64)
            .sum::<u64>();
    let mut b = Builder::new(source, total, report, cancel);

    let manuscript_bid = b.ids.take();
    let story_bid = b.ids.take();

    // Status tags BEFORE the walk: `emit_*` attaches one per node as it goes, so the
    // decision of whether badges are a vocabulary at all has to be already made.
    b.prepare_badge_tags(tree);

    // Story bible FIRST — populates `attend_id_map` before the manuscript walk.
    let story_items = b.build_story_bible(attendance, story_bible_name);

    // Manuscript.
    let mut manuscript_items = Vec::new();
    for root in &tree.roots {
        b.emit_node(
            root,
            0,
            MANUSCRIPT_INDEX,
            manuscript_name,
            &mut manuscript_items,
        );
    }

    // Dictionary.
    let dict: Vec<DictWordFile> = dict_words
        .iter()
        .map(|w| DictWordFile {
            file_id: b.ids.take(),
            created_at: b.now.clone(),
            updated_at: b.now.clone(),
            word: w.clone(),
        })
        .collect();

    let title = first_non_empty([
        info.title.as_str(),
        tree.project_name.as_str(),
        "Imported Plume project",
    ]);
    let work = WorkFile {
        file_id: b.ids.take(),
        created_at: info.created_at.clone().unwrap_or_else(|| b.now.clone()),
        updated_at: info.updated_at.clone().unwrap_or_else(|| b.now.clone()),
        title,
        // Empty because there is nothing to carry over: the Plume Creator format has
        // no author field anywhere — not in `info` (whose `plume-information` element
        // holds only the project name, paths, dates and text styles), not in `tree`,
        // not in `attendance`. Verified against real v0.3 projects. The writer sets
        // their name afterwards in Settings ▸ Work ▸ Author.
        //
        // This is *not* the same as the legacy SQLite `.skrib` path, which does have a
        // `t_author` column and does carry it over (`load_work_uc`).
        author_name: String::new(),
        dict_language: Vec::new(),
        tag_ids: b.tags.iter().map(|t| t.file_id).collect(),
        dict_word_ids: dict.iter().map(|d| d.file_id).collect(),
        unique_id: new_unique_id(),
        // Plume Creator had a word-goal widget, but it was a *session* sprint counter held
        // in memory and never written to the project file, so an imported project has no
        // targets at all and no unit to infer one from. Words is the default a new project
        // gets; the writer picks otherwise in Settings ▸ Work ▸ Structure.
        goal_unit: common::entities::GoalUnit::default(),
        // Plume Creator organises chapters as folders of sheets → folder mode.
        chapter_flat: false,
        // Plume has no custom-replacement concept to import.
        text_replacement_rule_ids: Vec::new(),
        custom_replacement_rules_enabled: false,
        // Nor a punctuation house style. `None`, not an all-false row: the
        // imported project has simply never been asked, so it should follow the
        // app default exactly as a newly created one does.
        smart_punctuation: None,
        // Plume numbered nothing itself — its chapter names are literal text, and this
        // importer carries them over verbatim (see `node.name` below). Numbering on is
        // still the right import default: it matches every freshly created project, and
        // an imported chapter genuinely titled "Chapitre 3" is exactly the case the
        // heading's redundancy guard exists to collapse.
        number_chapters: true,
        part_resets_chapter: false,
    };

    // Binders, in on-disk order: Manuscript, then Story Bible (only if non-empty).
    let mut binders = vec![make_binder(
        manuscript_bid,
        manuscript_name,
        &b.now,
        manuscript_items,
    )];
    let mut binder_order = vec![manuscript_bid];
    if !story_items.is_empty() {
        binders.push(make_binder(
            story_bid,
            story_bible_name,
            &b.now,
            story_items,
        ));
        binder_order.push(story_bid);
    }

    let bundle = WorkBundle {
        manifest: ProjectManifest {
            format_version: FORMAT_VERSION,
            // The read floor is stamped by the writer (`folder_io::write_folder`), not
            // here — see the same note in `skrib_format::mapping::from_entities`. This
            // hand-written literal is exactly the kind of second construction site that
            // would otherwise have been forgotten.
            format_min_read_version: None,
            shape: ShapeTag::Zip,
            work,
            binder_order,
            // An imported project is a regular project, not a backup.
            kind: skrib_format::BundleKind::Regular,
            backup_of: None,
            backup_created_at: None,
        },
        tags: b.tags,
        dict_words: dict,
        text_replacement_rules: Vec::new(),
        // Plume has no template concept, so an imported project starts with none —
        // the writer applies a preset from Settings ▸ Templates if they want any.
        note_templates: Vec::new(),
        // A Plume project has no binary assets to carry over.
        assets: Vec::new(),
        asset_bytes: Default::default(),
        note_template_bodies: Default::default(),
        trash_infos: Vec::new(),
        // Plume has no comment/annotation concept either, so there is nothing to
        // import and nothing that could already be orphaned.
        orphan_footnotes: Vec::new(),
        orphan_comments: Vec::new(),
        // Plume has no writing-plan or progress-history concept to import.
        paces: Vec::new(),
        progress_snapshots: Vec::new(),
        // A freshly imported project has no past to show: its history starts at
        // the moment of import, and the first save records that state.
        history: Default::default(),
        binders,
        // A `.plume` archive is a foreign format read field by field, not a
        // `.skrib` bundle with unmodelled files in it, so there is nothing to
        // carry: everything the importer understood is above, and everything it
        // did not is reported as a warning rather than smuggled into the new
        // project as opaque bytes.
        carried: Default::default(),
    };

    Mapped {
        bundle,
        imported_items: b.imported_items,
        skipped_trashed: b.skipped_trashed,
        warnings: b.warnings,
    }
}

fn make_binder(file_id: u64, name: &str, now: &str, items: Vec<BundledItem>) -> BundledBinder {
    BundledBinder {
        binder: BinderFile {
            file_id,
            // The importer WRITES a v3 bundle, so it mints identities rather
            // than leaning on the loader's heal-on-empty path.
            uid: common::uid::new_uid(),
            created_at: now.to_string(),
            updated_at: now.to_string(),
            name: name.to_string(),
            activated: true,
            item_order: items.iter().map(|i| i.item.file_id).collect(),
        },
        items,
    }
}

// ---------------------------------------------------------------------------

/// Colours handed to synthesised tags, cycled by creation order.
///
/// Deliberately excludes near-black and near-white: a tag's colour is theme-constant, so
/// either extreme disappears against one of the two surfaces. Ten distinct hues is more
/// than any real Plume project needs (three story-bible groups is typical).
const TAG_PALETTE: [&str; 10] = [
    "#c0392b", // red
    "#d35400", // orange
    "#f39c12", // amber
    "#27ae60", // green
    "#16a085", // teal
    "#2980b9", // blue
    "#8e44ad", // purple
    "#c2185b", // pink
    "#795548", // brown
    "#607d8b", // slate
];

struct Builder<'a> {
    ids: IdGen,
    source: &'a PlumeSource,
    now: String,
    /// Plume attendance obj `number` → the story-bible note item's file id.
    attend_id_map: HashMap<u32, u64>,
    /// Synthesised palette rows, in creation order.
    tags: Vec<BinderTagFile>,
    /// Story-bible group name → its (discoverable) tag file id.
    group_tag: HashMap<String, u64>,
    /// Node badge value → its (non-discoverable) status tag file id. Empty when the
    /// badges did not look like a reused vocabulary — see `badges_look_like_a_vocabulary`.
    badge_tag: HashMap<String, u64>,
    warnings: Vec<String>,
    imported_items: u64,
    skipped_trashed: u64,
    /// Progress reporter (percent in the 20‑88 % band, current item label).
    report: &'a dyn Fn(f32, &str),
    /// Cancel token, polled once per mapped node.
    cancel: &'a AtomicBool,
    /// Total + processed mapping units, for the percentage.
    total: u64,
    done: u64,
    /// Latches once `cancel` is observed set, so the recursive walk unwinds fast.
    cancelled: bool,
}

impl<'a> Builder<'a> {
    fn new(
        source: &'a PlumeSource,
        total: u64,
        report: &'a dyn Fn(f32, &str),
        cancel: &'a AtomicBool,
    ) -> Self {
        Self {
            ids: IdGen::new(),
            source,
            now: chrono::Utc::now().to_rfc3339(),
            attend_id_map: HashMap::new(),
            tags: Vec::new(),
            group_tag: HashMap::new(),
            badge_tag: HashMap::new(),
            warnings: Vec::new(),
            imported_items: 0,
            skipped_trashed: 0,
            report,
            cancel,
            total,
            done: 0,
            cancelled: false,
        }
    }

    /// Mint a palette tag and return its file id. Colours cycle through `TAG_PALETTE`.
    fn make_tag(&mut self, name: &str, details: &str, discoverable: bool) -> u64 {
        let file_id = self.ids.take();
        let color = TAG_PALETTE[self.tags.len() % TAG_PALETTE.len()].to_string();
        self.tags.push(BinderTagFile {
            // A Plume project carries no durable identity, so every imported row mints one.
            uid: common::uid::new_uid(),
            file_id,
            created_at: self.now.clone(),
            updated_at: self.now.clone(),
            name: name.to_string(),
            color,
            details: details.to_string(),
            discoverable,
            // A Plume project has no notion of either, so an imported tag is unfiled
            // and untemplated: the writer is asked once, the first time they use it.
            creates_in: None,
            note_template: None,
        });
        file_id
    }

    /// Mint the status tags, if the project's badges look like a reused vocabulary.
    ///
    /// Must run before the manuscript walk, because `emit_*` attaches the tag as each node
    /// is emitted. When the heuristic declines, `badge_tag` stays empty and every badge is
    /// left where it already was, in `BinderItem.label`.
    fn prepare_badge_tags(&mut self, tree: &PlumeTree) {
        let counts = count_badges(&tree.roots);
        if !badges_look_like_a_vocabulary(&counts) {
            if !counts.is_empty() {
                self.warnings.push(format!(
                    "{} distinct scene badges look like free text rather than a status \
                     vocabulary — left as item labels instead of tags",
                    counts.len()
                ));
            }
            return;
        }
        // Sorted so the assigned colours are stable across runs of the same project
        // rather than following HashMap iteration order.
        let mut names: Vec<&String> = counts.keys().collect();
        names.sort();
        for name in names {
            let id = self.make_tag(name, "", false);
            self.badge_tag.insert(name.clone(), id);
        }
    }

    /// Move a node's badge from its label onto a status tag, when the badges were imported
    /// as tags. A no-op when the heuristic declined, leaving the badge in `label` exactly
    /// as previous imports produced it.
    ///
    /// The label is cleared on success: `make_item` has already put the badge there, and
    /// showing the same string as both a subtitle and a chip is duplication, not emphasis.
    fn apply_badge(&self, badge: &str, bi: &mut BundledItem) {
        if let Some(id) = self.badge_tag.get(badge.trim()) {
            bi.item.tag_ids = vec![*id];
            bi.item.label.clear();
        }
    }

    /// Advance the progress bar by one mapping unit and poll the cancel token.
    /// Reports into the 20‑88 % band, leaving room for the surrounding phases.
    fn tick(&mut self, label: &str) {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
        }
        self.done += 1;
        let frac = if self.total == 0 {
            1.0
        } else {
            (self.done as f32 / self.total as f32).min(1.0)
        };
        (self.report)(20.0 + frac * 68.0, label);
    }

    // --- Story bible ------------------------------------------------------

    fn build_story_bible(
        &mut self,
        attendance: &PlumeAttendance,
        binder_name: &str,
    ) -> Vec<BundledItem> {
        let mut out = Vec::new();
        for group in &attendance.groups {
            if self.cancelled {
                break;
            }
            self.tick(&group.name);
            let group_doc = self.attend_doc_djot(group.number);
            // Skip a group with no members and no own description — an empty
            // "Characters" folder is pure clutter.
            if group.objs.is_empty() && group_doc.is_empty() {
                continue;
            }

            // One discoverable tag per story-bible group, named by the writer's own group
            // ("Personnages", "Lieux", or whatever they called it) rather than by three
            // invented English names. Discoverable because these ARE the story-bible
            // entities the mention index scans prose for.
            let group_tag_id = match self.group_tag.get(&group.name) {
                Some(id) => *id,
                None => {
                    let id = self.make_tag(&group.name, "", true);
                    self.group_tag.insert(group.name.clone(), id);
                    id
                }
            };

            let mut folder_contents = Vec::new();
            if !group_doc.is_empty() {
                folder_contents.push((ContentRole::SynopsisText, group_doc));
            }
            let (_gid, gbi) = self.make_item(
                STORY_BIBLE_INDEX,
                binder_name,
                Role::Folder,
                SubRole::None,
                &group.name,
                0,
                false,
                folder_contents,
                Vec::new(),
                "",
            );
            out.push(gbi);

            for obj in &group.objs {
                if self.cancelled {
                    break;
                }
                self.tick(&obj.name);
                let note_text = self.attend_doc_djot(obj.number);
                let synopsis = build_obj_synopsis(obj, &attendance.spinbox_label);
                let (oid, mut obi) = self.make_item(
                    STORY_BIBLE_INDEX,
                    binder_name,
                    Role::Item,
                    SubRole::Note,
                    &obj.name,
                    1,
                    true,
                    vec![
                        (ContentRole::NoteText, note_text),
                        (ContentRole::SynopsisText, synopsis),
                    ],
                    Vec::new(),
                    "",
                );
                // Set after construction rather than as two more `make_item` parameters:
                // it already takes ten, and only story-bible objects have either. As a
                // real field (not synopsis prose) aliases make the mention index work
                // on an imported project with no author effort.
                obi.item.aliases = obj.aliases.clone();
                obi.item.tag_ids = vec![group_tag_id];
                out.push(obi);
                if let Some(num) = obj.number {
                    self.attend_id_map.insert(num, oid);
                }
            }
        }
        out
    }

    // --- Manuscript -------------------------------------------------------

    fn emit_node(
        &mut self,
        node: &PlumeNode,
        indent: i64,
        bindex: usize,
        bname: &str,
        out: &mut Vec<BundledItem>,
    ) {
        // Once cancelled, every recursive call returns here — the walk unwinds
        // without doing further HTML→Djot work. `plume::import` then bails
        // before writing any output.
        if self.cancelled {
            return;
        }
        self.tick(&node.name);
        if node.is_trashed {
            self.skipped_trashed += count_meaningful(node);
            return;
        }
        match node.kind {
            PlumeKind::Book => self.emit_container(
                node,
                indent,
                SubRole::Book,
                ContentRole::BookTitle,
                true,
                bindex,
                bname,
                out,
            ),
            PlumeKind::Act => self.emit_container(
                node,
                indent,
                SubRole::Part,
                ContentRole::PartTitle,
                false,
                bindex,
                bname,
                out,
            ),
            PlumeKind::Chapter => {
                if node.children.iter().any(|c| c.kind == PlumeKind::Scene) {
                    self.emit_container(
                        node,
                        indent,
                        SubRole::ChapterScene,
                        ContentRole::ChapterTitle,
                        false,
                        bindex,
                        bname,
                        out,
                    )
                } else {
                    self.emit_chapter_scene(node, indent, bindex, bname, out);
                }
            }
            PlumeKind::Scene => self.emit_scene(node, indent, bindex, bname, out),
            PlumeKind::Separator => self.emit_separator(node, out),
        }
    }

    /// A container node (Book/Act/Chapter-with-scenes). Emits the folder, then its
    /// overflow prose/notes as synthetic first children, then its real children,
    /// then a `BookEnd` marker (books only).
    #[allow(clippy::too_many_arguments)]
    fn emit_container(
        &mut self,
        node: &PlumeNode,
        indent: i64,
        sub_role: SubRole,
        title_role: ContentRole,
        with_book_end: bool,
        bindex: usize,
        bname: &str,
        out: &mut Vec<BundledItem>,
    ) {
        let synopsis = self.synopsis_djot(node.number);
        let mut contents = vec![(title_role, node.name.clone())];
        if !synopsis.is_empty() {
            contents.push((ContentRole::SynopsisText, synopsis));
        }
        let refs = self.resolve_attend(&node.attend);
        let (_id, mut bi) = self.make_item(
            bindex,
            bname,
            Role::Folder,
            sub_role,
            &node.name,
            indent,
            true,
            contents,
            refs,
            &node.badge,
        );
        self.apply_badge(&node.badge, &mut bi);
        out.push(bi);

        // Overflow: the container's own T prose → a child Scene; own N → a child Note.
        let own_text = self.text_djot(node.number);
        if !own_text.is_empty() {
            let (_i, cbi) = self.make_item(
                bindex,
                bname,
                Role::Item,
                SubRole::Scene,
                &content_title(&node.name),
                indent + 1,
                true,
                vec![
                    (ContentRole::SceneText, own_text),
                    (ContentRole::SynopsisText, String::new()),
                ],
                Vec::new(),
                "",
            );
            out.push(cbi);
        }
        let own_note = self.note_djot(node.number);
        if !own_note.is_empty() {
            let (_i, nbi) = self.make_item(
                bindex,
                bname,
                Role::Item,
                SubRole::Note,
                &notes_title(&node.name),
                indent + 1,
                true,
                vec![
                    (ContentRole::NoteText, own_note),
                    (ContentRole::SynopsisText, String::new()),
                ],
                Vec::new(),
                "",
            );
            out.push(nbi);
        }

        for child in &node.children {
            self.emit_node(child, indent + 1, bindex, bname, out);
        }

        if with_book_end {
            let (_i, ebi) = self.make_item(
                bindex,
                bname,
                Role::Item,
                SubRole::BookEnd,
                "",
                indent + 1,
                true,
                Vec::new(),
                Vec::new(),
                "",
            );
            out.push(ebi);
        }
    }

    /// A leaf chapter (no scene children) → a `ChapterScene` the user writes
    /// straight into. Its own note becomes a following-sibling `Note`.
    fn emit_chapter_scene(
        &mut self,
        node: &PlumeNode,
        indent: i64,
        bindex: usize,
        bname: &str,
        out: &mut Vec<BundledItem>,
    ) {
        let scene = self.text_djot(node.number);
        let synopsis = self.synopsis_djot(node.number);
        let refs = self.resolve_attend(&node.attend);
        let (_id, mut bi) = self.make_item(
            bindex,
            bname,
            Role::Item,
            SubRole::ChapterScene,
            &node.name,
            indent,
            true,
            vec![
                (ContentRole::ChapterTitle, node.name.clone()),
                (ContentRole::SceneText, scene),
                (ContentRole::SynopsisText, synopsis),
            ],
            refs,
            &node.badge,
        );
        self.apply_badge(&node.badge, &mut bi);
        out.push(bi);
        self.emit_sibling_note(node, indent, bindex, bname, out);
        self.warn_dropped_separators(node);
    }

    fn emit_scene(
        &mut self,
        node: &PlumeNode,
        indent: i64,
        bindex: usize,
        bname: &str,
        out: &mut Vec<BundledItem>,
    ) {
        let scene = self.text_djot(node.number);
        let synopsis = self.synopsis_djot(node.number);
        let refs = self.resolve_attend(&node.attend);
        let (_id, mut bi) = self.make_item(
            bindex,
            bname,
            Role::Item,
            SubRole::Scene,
            &node.name,
            indent,
            true,
            vec![
                (ContentRole::SceneText, scene),
                (ContentRole::SynopsisText, synopsis),
            ],
            refs,
            &node.badge,
        );
        self.apply_badge(&node.badge, &mut bi);
        out.push(bi);
        self.emit_sibling_note(node, indent, bindex, bname, out);
    }

    /// A Plume separator becomes a scene-break marker in the *preceding* scene's
    /// prose, so it needs neither a binder slot nor a name of its own — hence the
    /// narrower signature than its `emit_*` siblings.
    fn emit_separator(&mut self, node: &PlumeNode, out: &mut [BundledItem]) {
        if !self.text_djot(node.number).is_empty()
            || !self.synopsis_djot(node.number).is_empty()
            || !self.note_djot(node.number).is_empty()
        {
            self.warnings.push(format!(
                "separator '{}' carried text that has no place in the writing model and was dropped",
                node.name
            ));
        }
        // Plume's separator is exactly Skribisto's scene break, so it becomes a
        // marker paragraph in the preceding scene's prose rather than a
        // contentless item in the tree. Its `name` holds the glyph the author
        // chose (typically "* * *"), which decides the tier.
        let tier = scene_break::tier_of_plain_line(&node.name).unwrap_or(SceneBreakTier::Minor);
        if !append_marker_to_previous_scene(out, tier) {
            self.warnings.push(format!(
                "separator '{}' had no preceding scene in its chapter to attach to and was dropped",
                node.name
            ));
        }
    }

    /// Emit a scene/chapter-scene's own note as a following sibling `Note` (those
    /// leaves cannot themselves hold `NoteText`).
    fn emit_sibling_note(
        &mut self,
        node: &PlumeNode,
        indent: i64,
        bindex: usize,
        bname: &str,
        out: &mut Vec<BundledItem>,
    ) {
        let note = self.note_djot(node.number);
        if note.is_empty() {
            return;
        }
        let (_i, nbi) = self.make_item(
            bindex,
            bname,
            Role::Item,
            SubRole::Note,
            &notes_title(&node.name),
            indent,
            true,
            vec![
                (ContentRole::NoteText, note),
                (ContentRole::SynopsisText, String::new()),
            ],
            Vec::new(),
            "",
        );
        out.push(nbi);
    }

    fn warn_dropped_separators(&mut self, node: &PlumeNode) {
        if node.children.iter().any(|c| c.kind == PlumeKind::Separator) {
            self.warnings.push(format!(
                "chapter '{}' has no scenes, so its separators were dropped",
                node.name
            ));
        }
    }

    // --- Item construction -------------------------------------------------

    /// Build one item, splitting model-valid contents into inline title rows and
    /// `.djot` prose refs (exactly as `skrib_format::from_entities` does from store
    /// entities). Returns the item's file id and the bundled item.
    #[allow(clippy::too_many_arguments)]
    fn make_item(
        &mut self,
        binder_index: usize,
        binder_name: &str,
        role: Role,
        sub_role: SubRole,
        title: &str,
        indent: i64,
        is_exportable: bool,
        contents: Vec<(ContentRole, String)>,
        reference_ids: Vec<u64>,
        label: &str,
    ) -> (u64, BundledItem) {
        let item_id = self.ids.take();
        // Minted here rather than at construction below because the prose file
        // names are derived from it: `skrib_format::prose_file_name` keys on the
        // item's durable uid so a name survives a reopen unchanged.
        let item_uid = common::uid::new_uid();
        let dir = binder_dir_name(binder_index, binder_name);

        let mut inline_contents = Vec::new();
        let mut prose_refs = Vec::new();
        let mut prose = BTreeMap::new();

        for (content_role, data) in contents {
            if !content_allowed(&role, &sub_role, &content_role) {
                continue; // never serialise an invalid (role, sub_role, content) triple
            }
            let content_id = self.ids.take();
            match prose_kind(&content_role) {
                None => inline_contents.push(InlineContent {
                    uid: common::uid::new_uid(),
                    file_id: content_id,
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    activated: true,
                    role: content_role,
                    text: data,
                }),
                Some(_) => {
                    let name = prose_file_name(item_uid, title, &content_role)
                        .expect("prose_kind matched");
                    prose_refs.push(ProseRef {
                        uid: common::uid::new_uid(),
                        file_id: content_id,
                        created_at: self.now.clone(),
                        updated_at: self.now.clone(),
                        activated: true,
                        role: content_role,
                        path: prose_relpath(&dir, &name),
                    });
                    prose.insert(content_id, data);
                }
            }
        }

        self.imported_items += 1;
        (
            item_id,
            BundledItem {
                item: BinderItemFile {
                    file_id: item_id,
                    uid: item_uid,
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    title: title.to_string(),
                    sub_title: String::new(),
                    role,
                    sub_role,
                    label: label.to_string(),
                    activated: true,
                    is_favorite: false,
                    is_exportable,
                    // Plume has no unnumbered-chapter concept, so nothing here can be
                    // imported as one. The writer marks their prologue afterwards.
                    exclude_from_numbering: false,
                    indent,
                    word_count_goal: 0,
                    char_count_goal: 0,
                    dict_language: Vec::new(),
                    // Empty here; story-bible objects (the only nodes with aliases) get
                    // theirs set post-construction in `build_story_bible` — tree items
                    // never have any.
                    aliases: Vec::new(),
                    inline_contents,
                    prose_refs,
                    reference_ids,
                    // Plume Creator has no point-of-view concept to carry over.
                    point_of_view_ids: Vec::new(),
                    // Nor a Book-filing concept: every imported item starts unfiled, the
                    // same ordinary state as any pre-existing project's own items.
                    book_ids: Vec::new(),
                    tag_ids: Vec::new(),
                },
                prose,
                // Plume carries no annotations, so an imported item starts unannotated.
                comments: std::collections::BTreeMap::new(),
                footnotes: Default::default(),
            },
        )
    }

    // --- HTML→Djot helpers (copy out of `source` before touching `self`) ---

    fn text_djot(&mut self, number: Option<u32>) -> String {
        let raw = number.and_then(|n| self.source.text(n)).map(str::to_string);
        self.convert(raw.as_deref())
    }
    fn synopsis_djot(&mut self, number: Option<u32>) -> String {
        let raw = number
            .and_then(|n| self.source.synopsis(n))
            .map(str::to_string);
        self.convert(raw.as_deref())
    }
    fn note_djot(&mut self, number: Option<u32>) -> String {
        let raw = number.and_then(|n| self.source.note(n)).map(str::to_string);
        self.convert(raw.as_deref())
    }
    fn attend_doc_djot(&mut self, number: Option<u32>) -> String {
        let raw = number
            .and_then(|n| self.source.attend_doc(n))
            .map(str::to_string);
        self.convert(raw.as_deref())
    }

    fn convert(&mut self, html: Option<&str>) -> String {
        match html {
            None => String::new(),
            Some(h) => match html_to_djot(h) {
                // Qt writes a full HTML boilerplate even for an *empty* document
                // (a single empty paragraph), which converts to whitespace, not
                // "". Collapse a whitespace-only result to empty so the emptiness
                // checks (skip empty groups / synopses / overflow children) fire;
                // real prose is returned verbatim.
                Ok(d) if d.trim().is_empty() => String::new(),
                Ok(d) => d,
                Err(e) => {
                    self.warnings
                        .push(format!("could not convert some rich text: {e}"));
                    String::new()
                }
            },
        }
    }

    fn resolve_attend(&mut self, attend: &[u32]) -> Vec<u64> {
        let mut refs = Vec::new();
        for num in attend {
            match self.attend_id_map.get(num) {
                Some(id) => refs.push(*id),
                None => self.warnings.push(format!(
                    "cross-link to attendance #{num} could not be resolved (missing or trashed)"
                )),
            }
        }
        refs
    }
}

/// Sequential file-id allocator (ids are internally consistent; the load path
/// remaps them to fresh store ids).
struct IdGen {
    next: u64,
}
impl IdGen {
    fn new() -> Self {
        Self { next: 1 }
    }
    fn take(&mut self) -> u64 {
        let id = self.next;
        self.next += 1;
        id
    }
}

/// The synopsis shown on an imported story-bible note: the quick-details paragraph, then a
/// metadata line of box labels · spin-box value.
///
/// `obj.aliases` is deliberately absent from this text — they live on `BinderItem.aliases`
/// instead, where the mention index can actually use them.
fn build_obj_synopsis(obj: &PlumeObj, spinbox_label: &str) -> String {
    let mut meta: Vec<String> = Vec::new();
    for label in &obj.box_labels {
        if !label.is_empty() {
            meta.push(label.clone());
        }
    }
    if !obj.spinbox.is_empty() {
        meta.push(
            format!("{} {}", spinbox_label, obj.spinbox)
                .trim()
                .to_string(),
        );
    }

    let mut parts: Vec<String> = Vec::new();
    if !obj.quick_details.trim().is_empty() {
        parts.push(obj.quick_details.trim().to_string());
    }
    if !meta.is_empty() {
        parts.push(meta.join(" · "));
    }
    parts.join("\n\n")
}

/// Count each distinct non-empty badge across the (non-trashed) tree.
fn count_badges(nodes: &[PlumeNode]) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    fn walk(nodes: &[PlumeNode], out: &mut HashMap<String, usize>) {
        for n in nodes {
            // Trashed subtrees are not emitted, so they must not sway the heuristic.
            if n.is_trashed {
                continue;
            }
            let badge = n.badge.trim();
            if !badge.is_empty() {
                *out.entry(badge.to_string()).or_insert(0) += 1;
            }
            walk(&n.children, out);
        }
    }
    walk(nodes, &mut out);
    out
}

/// Whether a project's badges look like a **reused vocabulary** ("draft", "to revise") as
/// opposed to per-scene free text.
///
/// Plume's `badge` is an unvalidated free-text attribute — confirmed against the original
/// C++ importer (`v1.9.43:.../skrplumecreatorimporter.cpp`), which read it and passed it
/// straight through as a tree-item label with no enum, no validation and no vocabulary. So
/// a project may carry hundreds of distinct badges, and turning each into a tag would bury
/// the palette in junk.
///
/// Two conditions, both necessary: few enough distinct values to be a vocabulary at all,
/// and each value reused on average across at least two items. Twelve distinct badges over
/// four hundred scenes is a vocabulary; twelve over thirteen scenes is prose.
fn badges_look_like_a_vocabulary(counts: &HashMap<String, usize>) -> bool {
    const MAX_DISTINCT: usize = 12;
    if counts.is_empty() || counts.len() > MAX_DISTINCT {
        return false;
    }
    let total: usize = counts.values().sum();
    counts.len() * 2 <= total
}

fn content_title(name: &str) -> String {
    if name.trim().is_empty() {
        "(content)".to_string()
    } else {
        format!("{name} (content)")
    }
}

fn notes_title(name: &str) -> String {
    if name.trim().is_empty() {
        "(notes)".to_string()
    } else {
        format!("{name} (notes)")
    }
}

/// Count the meaningful (non-separator) nodes in a subtree — used for the
/// `skipped_trashed` tally.
/// Append a scene-break marker paragraph to the most recent scene-bearing item's
/// `SceneText`. Returns `false` when there is none to attach to.
///
/// The walk goes backwards because `emit_scene` pushes a sibling `Note` right
/// after the scene it belongs to, so the previous item is not reliably the
/// scene. It stops at a structural opener (a Book/Part/Chapter container) so a
/// separator that leads a chapter cannot reach back and mark the *previous*
/// chapter's last scene — `carries_scene` is tested first, since a `ChapterScene`
/// both opens a chapter and holds prose of its own.
fn append_marker_to_previous_scene(out: &mut [BundledItem], tier: SceneBreakTier) -> bool {
    for bi in out.iter_mut().rev() {
        if bi.item.sub_role.carries_scene() {
            // Keep walking past a scene that carries no `SceneText` row (a Plume
            // scene with only a synopsis). Giving up on the first one would drop
            // the mark while reporting that no preceding scene existed — which
            // would be untrue, and the wrong thing to tell the writer.
            let Some(file_id) = bi
                .item
                .prose_refs
                .iter()
                .find(|p| p.role == ContentRole::SceneText)
                .map(|p| p.file_id)
            else {
                continue;
            };
            let text = bi.prose.entry(file_id).or_default();
            if !text.trim().is_empty() {
                text.push_str("\n\n");
            }
            text.push_str(scene_break::canonical_djot(tier));
            return true;
        }
        if bi.item.sub_role.opens_book()
            || bi.item.sub_role.opens_part()
            || bi.item.sub_role.opens_chapter()
        {
            return false;
        }
    }
    false
}

fn count_meaningful(node: &PlumeNode) -> u64 {
    let self_count = if node.kind == PlumeKind::Separator {
        0
    } else {
        1
    };
    self_count + node.children.iter().map(count_meaningful).sum::<u64>()
}

/// Count every node in a forest (all kinds, trashed or not) — the denominator
/// for the manuscript-walk progress bar.
fn count_tree_nodes(nodes: &[PlumeNode]) -> u64 {
    nodes
        .iter()
        .map(|n| 1 + count_tree_nodes(&n.children))
        .sum()
}

fn first_non_empty<'a>(candidates: impl IntoIterator<Item = &'a str>) -> String {
    candidates
        .into_iter()
        .map(str::trim)
        .find(|s| !s.is_empty())
        .unwrap_or("")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::PlumeGroup;

    fn node(kind: PlumeKind, number: u32, name: &str, badge: &str) -> PlumeNode {
        PlumeNode {
            kind,
            number: Some(number),
            name: name.to_string(),
            is_trashed: false,
            badge: badge.to_string(),
            attend: Vec::new(),
            children: Vec::new(),
        }
    }

    fn obj(number: u32, name: &str, aliases: &[&str]) -> PlumeObj {
        PlumeObj {
            number: Some(number),
            name: name.to_string(),
            aliases: aliases.iter().map(|a| a.to_string()).collect(),
            quick_details: String::new(),
            box_labels: [String::new(), String::new(), String::new()],
            spinbox: String::new(),
        }
    }

    fn map(tree: PlumeTree, attendance: PlumeAttendance) -> Mapped {
        let source = PlumeSource::for_tests();
        let info = PlumeInfo {
            title: "T".into(),
            created_at: None,
            updated_at: None,
        };
        build_bundle(
            &tree,
            &attendance,
            &info,
            &[],
            &source,
            "Manuscript",
            "Story Bible",
            &|_, _| {},
            &AtomicBool::new(false),
        )
    }

    fn tag<'a>(m: &'a Mapped, name: &str) -> &'a BinderTagFile {
        m.bundle
            .tags
            .iter()
            .find(|t| t.name == name)
            .unwrap_or_else(|| panic!("no tag named {name:?} in {:?}", names(m)))
    }

    fn names(m: &Mapped) -> Vec<&str> {
        m.bundle.tags.iter().map(|t| t.name.as_str()).collect()
    }

    /// Every item across every binder, for assertions that don't care where it lives.
    fn items(m: &Mapped) -> Vec<&BinderItemFile> {
        m.bundle
            .binders
            .iter()
            .flat_map(|b| b.items.iter().map(|i| &i.item))
            .collect()
    }

    fn item<'a>(m: &'a Mapped, title: &str) -> &'a BinderItemFile {
        items(m)
            .into_iter()
            .find(|i| i.title == title)
            .unwrap_or_else(|| panic!("no item titled {title:?}"))
    }

    // --- the vocabulary heuristic ------------------------------------------

    #[test]
    fn no_badges_is_not_a_vocabulary() {
        assert!(!badges_look_like_a_vocabulary(&HashMap::new()));
    }

    #[test]
    fn a_few_values_reused_across_many_scenes_is_a_vocabulary() {
        let counts = HashMap::from([
            ("draft".to_string(), 40),
            ("to revise".to_string(), 12),
            ("done".to_string(), 8),
        ]);
        assert!(badges_look_like_a_vocabulary(&counts));
    }

    #[test]
    fn one_distinct_badge_per_scene_is_free_text() {
        // 5 distinct over 5 items: nothing is reused, so these are per-scene notes.
        let counts: HashMap<String, usize> = (0..5).map(|i| (format!("note {i}"), 1)).collect();
        assert!(!badges_look_like_a_vocabulary(&counts));
    }

    #[test]
    fn too_many_distinct_values_is_free_text_however_often_reused() {
        // 13 distinct, each used 10 times: reuse is high, but no author keeps a
        // thirteen-state workflow — this is a project using badges as a notes field.
        let counts: HashMap<String, usize> = (0..13).map(|i| (format!("v{i}"), 10)).collect();
        assert!(!badges_look_like_a_vocabulary(&counts));
    }

    #[test]
    fn trashed_nodes_do_not_sway_the_heuristic() {
        let mut trashed = node(PlumeKind::Scene, 9, "Cut", "one-off badge");
        trashed.is_trashed = true;
        let roots = vec![
            node(PlumeKind::Scene, 1, "A", "draft"),
            node(PlumeKind::Scene, 2, "B", "draft"),
            trashed,
        ];
        let counts = count_badges(&roots);
        assert_eq!(counts.len(), 1, "the trashed node's badge is not counted");
        assert_eq!(counts["draft"], 2);
    }

    // --- story-bible groups → discoverable tags ----------------------------

    #[test]
    fn story_bible_groups_become_discoverable_tags_carrying_their_objects() {
        let attendance = PlumeAttendance {
            spinbox_label: String::new(),
            groups: vec![
                PlumeGroup {
                    number: Some(1),
                    name: "Personnages".into(),
                    objs: vec![obj(1, "Elise", &["Kiri"]), obj(2, "Marc", &[])],
                },
                PlumeGroup {
                    number: Some(2),
                    name: "Lieux".into(),
                    objs: vec![obj(3, "Le phare", &[])],
                },
            ],
        };
        let m = map(
            PlumeTree {
                project_name: "P".into(),
                roots: vec![],
            },
            attendance,
        );

        // Named by the writer's own groups, not by three invented English names.
        assert_eq!(names(&m), vec!["Personnages", "Lieux"]);
        assert!(tag(&m, "Personnages").discoverable);
        assert!(tag(&m, "Lieux").discoverable);
        assert_ne!(
            tag(&m, "Personnages").color,
            tag(&m, "Lieux").color,
            "distinct hues so two groups are distinguishable at a glance"
        );

        let chars = tag(&m, "Personnages").file_id;
        assert_eq!(item(&m, "Elise").tag_ids, vec![chars]);
        assert_eq!(item(&m, "Marc").tag_ids, vec![chars]);
        assert_eq!(item(&m, "Le phare").tag_ids, vec![tag(&m, "Lieux").file_id]);

        // The palette is reachable from the Work, or the settings pane shows nothing.
        let mut work_ids = m.bundle.manifest.work.tag_ids.clone();
        work_ids.sort();
        let mut all: Vec<u64> = m.bundle.tags.iter().map(|t| t.file_id).collect();
        all.sort();
        assert_eq!(work_ids, all);
    }

    // --- aliases ------------------------------------------------------------

    #[test]
    fn object_aliases_land_on_the_item_and_leave_the_synopsis() {
        let attendance = PlumeAttendance {
            spinbox_label: String::new(),
            groups: vec![PlumeGroup {
                number: Some(1),
                name: "Characters".into(),
                objs: vec![obj(1, "Elizabeth Bennet", &["Lizzy", "Miss Bennet"])],
            }],
        };
        let m = map(
            PlumeTree {
                project_name: "P".into(),
                roots: vec![],
            },
            attendance,
        );

        let it = item(&m, "Elizabeth Bennet");
        assert_eq!(
            it.aliases,
            vec!["Lizzy".to_string(), "Miss Bennet".to_string()],
            "multi-word aliases survive as separate entries"
        );
        // They used to lead the synopsis metadata line; as structured data they must not
        // also appear as prose.
        let prose: String = m
            .bundle
            .binders
            .iter()
            .flat_map(|b| b.items.iter())
            .flat_map(|i| i.item.inline_contents.iter().map(|c| c.text.clone()))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(
            !prose.contains("Lizzy"),
            "aliases must not be duplicated into the synopsis: {prose:?}"
        );
    }

    // --- badges → status tags ----------------------------------------------

    #[test]
    fn reused_badges_become_status_tags_and_vacate_the_label() {
        let roots = vec![
            node(PlumeKind::Scene, 1, "A", "draft"),
            node(PlumeKind::Scene, 2, "B", "draft"),
            node(PlumeKind::Scene, 3, "C", "done"),
            node(PlumeKind::Scene, 4, "D", "done"),
        ];
        let m = map(
            PlumeTree {
                project_name: "P".into(),
                roots,
            },
            PlumeAttendance {
                spinbox_label: String::new(),
                groups: vec![],
            },
        );

        let draft = tag(&m, "draft");
        assert!(
            !draft.discoverable,
            "a workflow state is not story-bible material"
        );
        assert_eq!(item(&m, "A").tag_ids, vec![draft.file_id]);
        assert_eq!(
            item(&m, "A").label,
            "",
            "the badge moved to a tag, so showing it as a label too would duplicate it"
        );
        assert_eq!(item(&m, "C").tag_ids, vec![tag(&m, "done").file_id]);
    }

    #[test]
    fn free_text_badges_stay_as_labels_and_are_reported() {
        // Four scenes, four distinct badges: not a vocabulary.
        let roots = vec![
            node(PlumeKind::Scene, 1, "A", "check the ferry timetable"),
            node(PlumeKind::Scene, 2, "B", "rewrite the storm"),
            node(PlumeKind::Scene, 3, "C", "too long?"),
            node(PlumeKind::Scene, 4, "D", "cut this"),
        ];
        let m = map(
            PlumeTree {
                project_name: "P".into(),
                roots,
            },
            PlumeAttendance {
                spinbox_label: String::new(),
                groups: vec![],
            },
        );

        assert!(m.bundle.tags.is_empty(), "no junk tags: {:?}", names(&m));
        assert_eq!(
            item(&m, "A").label,
            "check the ferry timetable",
            "left exactly where earlier imports put it"
        );
        assert!(item(&m, "A").tag_ids.is_empty());
        assert!(
            m.warnings.iter().any(|w| w.contains("free text")),
            "the writer should be told why: {:?}",
            m.warnings
        );
    }
}
