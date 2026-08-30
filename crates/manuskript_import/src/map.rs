// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Map a version-neutral [`Project`] onto a [`skrib_format::WorkBundle`] at the
//! newest `.skrib` format version.
//!
//! Two binders: a **Manuscript** (the outline) and a **Story bible** (the cast,
//! the world, the plots, and the project's own paperwork). The story bible is
//! built first, so a scene's POV and its inline references can resolve to the note
//! items they name. Every constructed item is filtered through
//! [`skribisto_model::content_allowed`], so a row that would violate the writing
//! model cannot be produced.
//!
//! # Structure, and why it is inferred from depth
//!
//! Manuskript has no Book, no Part, no Chapter and no Scene. It has folders and
//! text files, and chapter-ness is a convention the writer holds in their head.
//! Skribisto needs a `(role, sub_role)` for every row, so the shape has to be
//! inferred, and there are only two signals available: the label a row wears, and
//! how deep it sits.
//!
//! **Depth wins.** The default label list does contain "Chapter" and "Scene", but
//! those strings go through `self.tr()` when a project is created, so a French
//! project's list says "Chapitre" and a German one "Kapitel"; and the list is
//! editable, so even in English it may mean nothing of the kind. Matching on it
//! would be right only for an untouched English project and quietly wrong for
//! everyone else. Depth is the same in every language.
//!
//! A **Book is synthesised** to hold it all, because Manuskript has none and
//! Skribisto's pace planning, its analysis and its export scopes are all
//! Book-shaped. Its title is the project's own.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::entities::{ContentRole, StatusCategory};
use skrib_format::{
    BinderFile, BinderItemFile, BinderStatusFile, BinderTagFile, BundledBinder, BundledItem,
    FORMAT_VERSION, InlineContent, ProjectManifest, ProseRef, ShapeTag, WorkBundle, WorkFile,
    binder_dir_name, markdown_to_djot, new_unique_id, prose_file_name, prose_kind, prose_relpath,
};
use skribisto_model::content_allowed;

use crate::model::{Character, OutlineItem, Plot, Project, WorldItem};
use crate::refs::{self, Reference};

/// The result of mapping: the bundle plus what the UI reports.
pub struct Mapped {
    pub bundle: WorkBundle,
    pub imported_items: u64,
    pub imported_revisions: u64,
    pub warnings: Vec<String>,
}

/// Binder ordinals. They also drive the on-disk `binders/NN-slug` directory names,
/// so they have to match the order the binders appear in `bundle.binders`.
const MANUSCRIPT_INDEX: usize = 0;
const STORY_BIBLE_INDEX: usize = 1;

/// Colours for the three story-bible groups, which Manuskript gives none.
///
/// A label brings its own colour and keeps it; only these synthesised tags need
/// one. Chosen away from black and white, because a tag's colour is the same in
/// both themes and either extreme disappears against one of them.
const GROUP_COLORS: [&str; 3] = ["#2980b9", "#16a085", "#8e44ad"];

/// The handful of names the app supplies, resolved in the writer's own language.
///
/// Manuskript stores none of these: it has no binders, no story-bible groups, and
/// its importance scale is three integers whose names live in its UI and are
/// translated at display time. So they are passed down already resolved, exactly
/// as the binder names are, and an imported French project reads in French.
///
/// The **field headings inside a note body are deliberately not here.** Those are
/// Manuskript's own on-disk key names — `Motivation`, `Epiphany`, `Phrase Summary`
/// — which are hard-coded English in every project whatever the writer's language.
/// Reproducing them is what lets a writer recognise their own character sheet;
/// translating them would be this importer inventing a vocabulary the source never
/// had.
#[derive(Debug, Clone)]
pub struct Names {
    pub manuscript_binder: String,
    pub story_bible_binder: String,
    pub characters_group: String,
    pub world_group: String,
    pub plots_group: String,
    pub project_info_note: String,
    pub summary_note: String,
    /// The three rungs of Manuskript's importance scale, lowest first.
    pub importance: [String; 3],
}

impl Names {
    /// The name for a stored importance value, if it is on the scale.
    fn importance_of(&self, value: Option<u8>) -> Option<&str> {
        self.importance.get(value? as usize).map(String::as_str)
    }
}

/// Build the `.skrib` bundle.
///
/// `report(percent, label)` drives the progress toast; `cancel` is polled once per
/// mapped row so a large project can be abandoned mid-walk.
pub fn build_bundle(
    project: &Project,
    names: &Names,
    report: &dyn Fn(f32, &str),
    cancel: &AtomicBool,
) -> Mapped {
    let total = count_rows(&project.outline)
        + project.characters.len() as u64
        + count_world(&project.world)
        + project.plots.len() as u64;
    let mut b = Builder::new(project, names, total, report, cancel);

    let manuscript_bid = b.ids.take();
    let story_bid = b.ids.take();

    // Vocabularies before the walk: every row resolves its own label and rung as
    // it is emitted, so the rows they point at have to exist and be numbered.
    b.prepare_statuses();
    b.prepare_label_tags();

    // The marker table, once, before any prose is scanned against it.
    b.build_reference_targets();

    // The story bible first, so POV and `{C:…}`/`{W:…}`/`{P:…}` can resolve.
    let story_items = b.build_story_bible();
    let manuscript_items = b.build_manuscript();

    let title = first_non_empty([
        project.info.title.as_str(),
        project.source_name.as_str(),
        "Imported Manuskript project",
    ]);
    let work = WorkFile {
        file_id: b.ids.take(),
        created_at: b.now.clone(),
        updated_at: b.now.clone(),
        title,
        author_name: project.info.author.clone(),
        // The spellcheck locale is the only statement in a Manuskript project of
        // what language it is written in. `en_US` is spelled `en-US` here.
        dict_language: project
            .settings
            .dict
            .as_deref()
            .map(|d| vec![d.replace('_', "-")])
            .unwrap_or_default(),
        tag_ids: b.tags.iter().map(|t| t.file_id).collect(),
        dict_word_ids: Vec::new(),
        unique_id: new_unique_id(),
        // Manuskript's chapters are directories holding their scenes, so the
        // folder encoding is the one that matches what the writer already has.
        chapter_flat: false,
        text_replacement_rule_ids: Vec::new(),
        custom_replacement_rules_enabled: false,
        goal_unit: common::entities::GoalUnit::default(),
        // No punctuation house style to carry over. `None`, not an all-false row:
        // the project has simply never been asked, so it should follow the app
        // default exactly as a newly created one does.
        smart_punctuation: None,
        number_chapters: true,
        part_resets_chapter: false,
    };

    let mut binders = vec![make_binder(
        manuscript_bid,
        &names.manuscript_binder,
        &b.now,
        manuscript_items,
    )];
    let mut binder_order = vec![manuscript_bid];
    if !story_items.is_empty() {
        binders.push(make_binder(
            story_bid,
            &names.story_bible_binder,
            &b.now,
            story_items,
        ));
        binder_order.push(story_bid);
    }

    let history = b.build_history();
    let imported_revisions = history.entries.len() as u64;

    let bundle = WorkBundle {
        manifest: ProjectManifest {
            format_version: FORMAT_VERSION,
            // The read floor is computed by the writer at the moment it commits
            // the manifest, never here. Stamping it in a second place is how the
            // two drift apart.
            format_min_read_version: None,
            shape: ShapeTag::Zip,
            work,
            binder_order,
            kind: skrib_format::BundleKind::Regular,
            backup_of: None,
            backup_created_at: None,
        },
        tags: b.tags,
        dict_words: Vec::new(),
        text_replacement_rules: Vec::new(),
        note_templates: Vec::new(),
        statuses: b.statuses,
        // Manuskript stores no images or other binary assets of its own.
        assets: Vec::new(),
        asset_bytes: Default::default(),
        note_template_bodies: Default::default(),
        trash_infos: Vec::new(),
        orphan_footnotes: Vec::new(),
        orphan_comments: Vec::new(),
        paces: Vec::new(),
        progress_snapshots: Vec::new(),
        history,
        binders,
        // A Manuskript project is a foreign format read field by field, not a
        // `.skrib` with unmodelled files in it, so there is nothing to carry:
        // everything understood is above, and everything else is a warning rather
        // than opaque bytes smuggled into the new project.
        carried: Default::default(),
    };

    Mapped {
        bundle,
        imported_items: b.imported_items,
        imported_revisions,
        warnings: b.warnings,
    }
}

fn make_binder(file_id: u64, name: &str, now: &str, items: Vec<BundledItem>) -> BundledBinder {
    BundledBinder {
        binder: BinderFile {
            file_id,
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

/// Sequential file-id allocator. The ids only have to be internally consistent;
/// the load path remaps them to fresh store ids.
struct IdGen(u64);

impl IdGen {
    fn take(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }
}

struct Builder<'a> {
    project: &'a Project,
    names: &'a Names,
    ids: IdGen,
    now: String,
    tags: Vec<BinderTagFile>,
    statuses: Vec<BinderStatusFile>,
    /// Manuskript label index (1-based) → the tag minted for it.
    label_tag: HashMap<usize, u64>,
    /// Manuskript status index (1-based) → the rung minted for it.
    status_id: HashMap<usize, u64>,
    /// Character id → its story-bible note's file id.
    character_item: HashMap<String, u64>,
    /// World id → its note's file id.
    world_item: HashMap<String, u64>,
    /// Plot id → its note's file id.
    plot_item: HashMap<String, u64>,
    /// Outline id → its manuscript row's file id, filled during the walk and used
    /// to resolve `{T:…}` links in a second pass.
    outline_item: HashMap<String, u64>,
    /// Outline id → the row's durable uid, for attaching revisions.
    outline_uid: HashMap<String, uuid::Uuid>,
    /// Rows that cited an outline id, so `{T:…}` can be resolved once every row
    /// exists: a scene may reference one that has not been emitted yet.
    pending_outline_refs: Vec<(u64, String)>,
    /// What every reference marker should be replaced by, built once.
    ///
    /// Once, and not per row, because it is derived from the project alone and
    /// every row's prose is scanned against the same table: rebuilding it per row
    /// would walk the whole outline for every scene in the outline, which is fine
    /// on a fixture and quadratic on a novel.
    reference_targets: HashMap<(char, String), RefName>,
    warnings: Vec<String>,
    imported_items: u64,
    dropped_icons: usize,
    dropped_colors: usize,
    report: &'a dyn Fn(f32, &str),
    cancel: &'a AtomicBool,
    total: u64,
    done: u64,
    cancelled: bool,
}

impl<'a> Builder<'a> {
    fn new(
        project: &'a Project,
        names: &'a Names,
        total: u64,
        report: &'a dyn Fn(f32, &str),
        cancel: &'a AtomicBool,
    ) -> Self {
        Self {
            project,
            names,
            ids: IdGen(0),
            now: chrono::Utc::now().to_rfc3339(),
            tags: Vec::new(),
            statuses: Vec::new(),
            label_tag: HashMap::new(),
            status_id: HashMap::new(),
            character_item: HashMap::new(),
            world_item: HashMap::new(),
            plot_item: HashMap::new(),
            outline_item: HashMap::new(),
            outline_uid: HashMap::new(),
            pending_outline_refs: Vec::new(),
            reference_targets: HashMap::new(),
            warnings: project.notices.clone(),
            imported_items: 0,
            dropped_icons: 0,
            dropped_colors: 0,
            report,
            cancel,
            total,
            done: 0,
            cancelled: false,
        }
    }

    /// Advance the progress bar by one row and poll the cancel token.
    fn tick(&mut self, label: &str) {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
        }
        self.done += 1;
        let fraction = if self.total == 0 {
            1.0
        } else {
            (self.done as f32 / self.total as f32).min(1.0)
        };
        (self.report)(20.0 + fraction * 68.0, label);
    }

    fn make_tag(&mut self, name: &str, color: &str, discoverable: bool) -> u64 {
        let file_id = self.ids.take();
        self.tags.push(BinderTagFile {
            file_id,
            uid: common::uid::new_uid(),
            created_at: self.now.clone(),
            updated_at: self.now.clone(),
            name: name.to_string(),
            color: color.to_string(),
            details: String::new(),
            discoverable,
            creates_in: None,
            note_template: None,
        });
        file_id
    }

    /// Mint one tag per Manuskript label, keeping the colour the writer chose.
    ///
    /// Labels are a filing vocabulary — "Idea", "Research", "Chapter" — which is
    /// what a tag is here. They are **not** discoverable: a discoverable tag marks
    /// the story-bible entries the mention index hunts for in prose, and a label
    /// is not one of those.
    fn prepare_label_tags(&mut self) {
        for (index, label) in self.project.labels.iter().enumerate() {
            let name = label.name.trim();
            if name.is_empty() {
                continue;
            }
            let color = label
                .color
                .clone()
                .unwrap_or_else(|| GROUP_COLORS[index % GROUP_COLORS.len()].to_string());
            let id = self.make_tag(name, &color, false);
            // Stored indices are 1-based, and the vocabulary here is 0-based.
            self.label_tag.insert(index + 1, id);
        }
    }

    /// Mint the workflow ladder from the project's own status names.
    ///
    /// Unlike Plume, whose ladder is an index space with the names living in its
    /// UI, Manuskript stores the names — so a French project keeps its French
    /// rungs and the writer can rename them afterwards like any other.
    ///
    /// The category is assigned by position, the only signal available: the first
    /// rung is where work starts, the last is where it ends, the one before it is
    /// revision, and the rest is drafting. It is a starting point the writer
    /// re-picks per rung.
    ///
    /// A blank row is skipped **without consuming its number**, exactly as
    /// `prepare_label_tags` does. An item's `status` is a 1-based index into this
    /// list as it was stored, so re-numbering around a gap would silently move
    /// every rung below it and land the rows that wore them on the wrong one. A
    /// format-0 `status.xml` is where such a gap comes from: its table carries
    /// Manuskript's own empty rows.
    fn prepare_statuses(&mut self) {
        // (stored index, name) for every rung that has a name. The stored index is
        // what an item cites; the position within this list is what decides the
        // category, because a ladder's ends are its first and last real rungs.
        let rungs: Vec<(usize, String)> = self
            .project
            .statuses
            .iter()
            .enumerate()
            .filter(|(_, name)| !name.trim().is_empty())
            .map(|(index, name)| (index, name.trim().to_string()))
            .collect();
        let count = rungs.len();
        let last = count.saturating_sub(1);

        for (rank, (stored, name)) in rungs.into_iter().enumerate() {
            let category = if count == 1 {
                StatusCategory::Drafting
            } else if rank == 0 {
                StatusCategory::Planned
            } else if rank == last {
                StatusCategory::Final
            } else if rank == last - 1 {
                StatusCategory::Revised
            } else {
                StatusCategory::Drafting
            };
            let file_id = self.ids.take();
            self.statuses.push(BinderStatusFile {
                file_id,
                uid: common::uid::new_uid(),
                created_at: self.now.clone(),
                updated_at: self.now.clone(),
                name,
                category,
                details: String::new(),
            });
            // Keyed by the number the manuscript actually cites, 1-based.
            self.status_id.insert(stored + 1, file_id);
        }
    }

    /// Build one item, splitting model-valid contents into inline title rows and
    /// `.djot` prose refs.
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
    ) -> (u64, BundledItem) {
        let file_id = self.ids.take();
        let item_uid = common::uid::new_uid();
        let dir = binder_dir_name(binder_index, binder_name);

        let mut inline_contents = Vec::new();
        let mut prose_refs = Vec::new();
        let mut prose = std::collections::BTreeMap::new();

        for (content_role, data) in contents {
            if !content_allowed(&role, &sub_role, &content_role) {
                continue;
            }
            let content_id = self.ids.take();
            match prose_kind(&content_role) {
                None => inline_contents.push(InlineContent {
                    file_id: content_id,
                    uid: common::uid::new_uid(),
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    activated: true,
                    role: content_role,
                    text: data,
                }),
                Some(_) => {
                    // `prose_kind` just said this role has a file name, so the
                    // only way this returns `None` is a change to one of the two
                    // and not the other. Skip the content and say so, rather than
                    // taking the whole import down over it.
                    let Some(name) = prose_file_name(item_uid, title, &content_role) else {
                        self.warnings.push(format!(
                            "'{title}' has {content_role:?} content that could not be given a \
                             file name; it was not imported."
                        ));
                        continue;
                    };
                    prose_refs.push(ProseRef {
                        file_id: content_id,
                        uid: common::uid::new_uid(),
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
            file_id,
            BundledItem {
                item: BinderItemFile {
                    file_id,
                    uid: item_uid,
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    title: title.to_string(),
                    sub_title: String::new(),
                    role,
                    sub_role,
                    label: String::new(),
                    activated: true,
                    is_favorite: false,
                    is_exportable,
                    exclude_from_numbering: false,
                    indent,
                    word_count_goal: 0,
                    char_count_goal: 0,
                    dict_language: Vec::new(),
                    aliases: Vec::new(),
                    status_id: None,
                    inline_contents,
                    prose_refs,
                    reference_ids: Vec::new(),
                    point_of_view_ids: Vec::new(),
                    book_ids: Vec::new(),
                    tag_ids: Vec::new(),
                },
                prose,
                comments: Default::default(),
                footnotes: Default::default(),
            },
        )
    }

    /// Convert a Markdown body to Djot, reporting rather than failing.
    fn djot_of(&mut self, markdown: &str, what: &str) -> String {
        if markdown.trim().is_empty() {
            return String::new();
        }
        match markdown_to_djot(markdown) {
            Ok(djot) => djot,
            Err(e) => {
                self.warnings.push(format!(
                    "The text of '{what}' could not be converted ({e}); it was kept exactly as \
                     it was written."
                ));
                markdown.to_string()
            }
        }
    }
}

/// Count every outline row, for the progress denominator.
fn count_rows(items: &[OutlineItem]) -> u64 {
    items
        .iter()
        .map(|i| 1 + count_rows(&i.children))
        .sum::<u64>()
}

fn count_world(items: &[WorldItem]) -> u64 {
    items
        .iter()
        .map(|i| 1 + count_world(&i.children))
        .sum::<u64>()
}

fn first_non_empty<'a>(candidates: impl IntoIterator<Item = &'a str>) -> String {
    candidates
        .into_iter()
        .map(str::trim)
        .find(|s| !s.is_empty())
        .unwrap_or_default()
        .to_string()
}

// ── The story bible ─────────────────────────────────────────────────────────

impl Builder<'_> {
    /// Build the story-bible binder: the cast, the world, the plots, and the two
    /// pieces of project paperwork Skribisto has no field for.
    ///
    /// Group folders are `Folder/Note`, not the bare `Folder/None` a grouping
    /// folder might suggest. That is what earns them the Story bible card grid and
    /// an Overview; a `Folder/None` gets neither, and the imported cast would sit
    /// in a folder that looks like every other folder.
    fn build_story_bible(&mut self) -> Vec<BundledItem> {
        let mut out = Vec::new();
        let binder = self.names.story_bible_binder.clone();

        if !self.project.characters.is_empty() {
            let group = self.names.characters_group.clone();
            let tag = self.make_tag(&group, GROUP_COLORS[0], true);
            let folder_id = self.emit_group(&binder, &group, tag, &mut out);
            let characters = self.project.characters.clone();
            for character in &characters {
                if self.cancelled {
                    break;
                }
                self.tick(&character.name);
                self.emit_character(&binder, character, tag, &mut out);
            }
            self.file_new_notes_in(tag, folder_id);
        }

        if !self.project.world.is_empty() {
            let group = self.names.world_group.clone();
            let tag = self.make_tag(&group, GROUP_COLORS[1], true);
            let folder_id = self.emit_group(&binder, &group, tag, &mut out);
            let world = self.project.world.clone();
            self.emit_world(&binder, &world, 1, tag, &mut out);
            self.file_new_notes_in(tag, folder_id);
        }

        if !self.project.plots.is_empty() {
            let group = self.names.plots_group.clone();
            // Not discoverable: the mention index hunts for the names of things
            // that appear in prose, and a plot's name is a label for a thread, not
            // a word the book contains.
            let tag = self.make_tag(&group, GROUP_COLORS[2], false);
            let folder_id = self.emit_group(&binder, &group, tag, &mut out);
            let plots = self.project.plots.clone();
            for plot in &plots {
                if self.cancelled {
                    break;
                }
                self.tick(&plot.name);
                self.emit_plot(&binder, plot, tag, &mut out);
            }
            self.file_new_notes_in(tag, folder_id);
        }

        self.emit_project_info(&binder, &mut out);
        self.emit_summary_ladder(&binder, &mut out);
        out
    }

    /// A group folder, and the tag that names it.
    fn emit_group(
        &mut self,
        binder: &str,
        name: &str,
        tag: u64,
        out: &mut Vec<BundledItem>,
    ) -> u64 {
        let (id, mut item) = self.make_item(
            STORY_BIBLE_INDEX,
            binder,
            Role::Folder,
            SubRole::Note,
            name,
            0,
            false,
            Vec::new(),
        );
        item.item.tag_ids = vec![tag];
        out.push(item);
        id
    }

    /// Point a group's tag at its own folder, so "Add as note" files a new entry
    /// where its kind already lives.
    ///
    /// Set after the fact rather than at `make_tag`, because the tag has to exist
    /// before the folder it will name can carry it.
    fn file_new_notes_in(&mut self, tag: u64, folder_id: u64) {
        if let Some(t) = self.tags.iter_mut().find(|t| t.file_id == tag) {
            t.creates_in = Some(folder_id);
        }
    }

    fn emit_character(
        &mut self,
        binder: &str,
        character: &Character,
        tag: u64,
        out: &mut Vec<BundledItem>,
    ) {
        if !character.color.trim().is_empty() {
            // A character's swatch has nowhere to live: Skribisto colours tags,
            // not rows, and minting a tag per character would bury the palette.
            self.dropped_colors += 1;
        }
        let synopsis = self.djot_of(&character.summary_sentence, &character.name);
        let body = self.character_body(character);
        let note = self.djot_of(&body, &character.name);
        let (id, mut item) = self.make_item(
            STORY_BIBLE_INDEX,
            binder,
            Role::Item,
            SubRole::Note,
            &character.name,
            1,
            false,
            vec![
                (ContentRole::NoteText, note),
                (ContentRole::SynopsisText, synopsis),
            ],
        );
        item.item.tag_ids = vec![tag];
        if let Some(importance) = self.names.importance_of(character.importance) {
            item.item.label = importance.to_string();
        }
        out.push(item);
        if let Some(key) = character.id.clone() {
            self.character_item.insert(key, id);
        }
    }

    /// A character's remaining fields as a Djot document.
    ///
    /// Headed sections, which is the shape Skribisto's own note templates use, so
    /// an imported sheet and a sheet made from the Character preset read alike.
    /// Empty fields are left out rather than shipped as empty headings.
    fn character_body(&self, character: &Character) -> String {
        let mut sections: Vec<(&str, &str)> = vec![
            ("Paragraph Summary", character.summary_paragraph.as_str()),
            ("Full Summary", character.summary_full.as_str()),
            ("Motivation", character.motivation.as_str()),
            ("Goal", character.goal.as_str()),
            ("Conflict", character.conflict.as_str()),
            ("Epiphany", character.epiphany.as_str()),
            ("Notes", character.notes.as_str()),
        ];
        for (key, value) in &character.infos {
            sections.push((key.as_str(), value.as_str()));
        }
        sections_to_markdown(&sections)
    }

    fn emit_world(
        &mut self,
        binder: &str,
        items: &[WorldItem],
        indent: i64,
        tag: u64,
        out: &mut Vec<BundledItem>,
    ) {
        for entry in items {
            if self.cancelled {
                return;
            }
            self.tick(&entry.name);
            let body = sections_to_markdown(&[
                ("Description", entry.description.as_str()),
                ("Passion", entry.passion.as_str()),
                ("Conflict", entry.conflict.as_str()),
            ]);
            let note = self.djot_of(&body, &entry.name);
            let synopsis = self.djot_of(&entry.description, &entry.name);

            // A world entry with children is a folder, which the matrix allows
            // only a synopsis; one without is a note, which can hold its prose.
            let (id, mut item) = if entry.children.is_empty() {
                self.make_item(
                    STORY_BIBLE_INDEX,
                    binder,
                    Role::Item,
                    SubRole::Note,
                    &entry.name,
                    indent,
                    false,
                    vec![
                        (ContentRole::NoteText, note),
                        (ContentRole::SynopsisText, synopsis),
                    ],
                )
            } else {
                self.make_item(
                    STORY_BIBLE_INDEX,
                    binder,
                    Role::Folder,
                    SubRole::Note,
                    &entry.name,
                    indent,
                    false,
                    vec![(ContentRole::SynopsisText, note)],
                )
            };
            item.item.tag_ids = vec![tag];
            out.push(item);
            if let Some(key) = entry.id.clone() {
                self.world_item.insert(key, id);
            }
            self.emit_world(binder, &entry.children, indent + 1, tag, out);
        }
    }

    fn emit_plot(&mut self, binder: &str, plot: &Plot, tag: u64, out: &mut Vec<BundledItem>) {
        let body = sections_to_markdown(&[
            ("Description", plot.description.as_str()),
            ("Result", plot.result.as_str()),
            ("Summary", plot.summary.as_str()),
        ]);
        let note = self.djot_of(&body, &plot.name);
        let synopsis = self.djot_of(&plot.summary, &plot.name);
        let has_steps = !plot.steps.is_empty();

        // Same rule as a world entry: a plot with beats under it is a folder.
        let (id, mut item) = if has_steps {
            self.make_item(
                STORY_BIBLE_INDEX,
                binder,
                Role::Folder,
                SubRole::Note,
                &plot.name,
                1,
                false,
                vec![(ContentRole::SynopsisText, note)],
            )
        } else {
            self.make_item(
                STORY_BIBLE_INDEX,
                binder,
                Role::Item,
                SubRole::Note,
                &plot.name,
                1,
                false,
                vec![
                    (ContentRole::NoteText, note),
                    (ContentRole::SynopsisText, synopsis),
                ],
            )
        };
        item.item.tag_ids = vec![tag];
        if let Some(importance) = self.names.importance_of(plot.importance) {
            item.item.label = importance.to_string();
        }
        // A plot names the characters it is about. They are already emitted, so
        // this resolves now rather than waiting for the second pass.
        item.item.reference_ids = plot
            .characters
            .iter()
            .filter_map(|c| self.character_item.get(c).copied())
            .collect();
        out.push(item);
        if let Some(key) = plot.id.clone() {
            self.plot_item.insert(key, id);
        }

        for step in &plot.steps {
            let step_body = sections_to_markdown(&[
                ("Meta", step.meta.as_str()),
                ("Summary", step.summary.as_str()),
            ]);
            let step_note = self.djot_of(&step_body, &step.name);
            let step_synopsis = self.djot_of(&step.summary, &step.name);
            let (_, mut child) = self.make_item(
                STORY_BIBLE_INDEX,
                binder,
                Role::Item,
                SubRole::Note,
                &step.name,
                2,
                false,
                vec![
                    (ContentRole::NoteText, step_note),
                    (ContentRole::SynopsisText, step_synopsis),
                ],
            );
            child.item.tag_ids = vec![tag];
            out.push(child);
        }
    }

    /// The project details Skribisto has no field for.
    ///
    /// Title and author are on the `Work` itself and the subtitle is on the Book,
    /// so only what is left over lands here. Nothing is written when nothing is
    /// left over.
    fn emit_project_info(&mut self, binder: &str, out: &mut Vec<BundledItem>) {
        let info = &self.project.info;
        let body = sections_to_markdown(&[
            ("Serie", info.serie.as_str()),
            ("Volume", info.volume.as_str()),
            ("Genre", info.genre.as_str()),
            ("License", info.license.as_str()),
            ("Email", info.email.as_str()),
        ]);
        if body.trim().is_empty() {
            return;
        }
        let note = self.djot_of(&body, &self.names.project_info_note.clone());
        let title = self.names.project_info_note.clone();
        let (_, item) = self.make_item(
            STORY_BIBLE_INDEX,
            binder,
            Role::Item,
            SubRole::Note,
            &title,
            0,
            false,
            vec![(ContentRole::NoteText, note)],
        );
        out.push(item);
    }

    /// The five-rung snowflake ladder, kept whole.
    ///
    /// Skribisto has one synopsis per row, not five nested tellings of the whole
    /// book, so the longest rung becomes the Book's synopsis and the ladder itself
    /// is preserved here rather than four fifths of it being dropped.
    fn emit_summary_ladder(&mut self, binder: &str, out: &mut Vec<BundledItem>) {
        if self.project.summary.is_empty() {
            return;
        }
        let rungs = self.project.summary.rungs();
        let body = sections_to_markdown(&rungs);
        let note = self.djot_of(&body, &self.names.summary_note.clone());
        let title = self.names.summary_note.clone();
        let (_, item) = self.make_item(
            STORY_BIBLE_INDEX,
            binder,
            Role::Item,
            SubRole::Note,
            &title,
            0,
            false,
            vec![(ContentRole::NoteText, note)],
        );
        out.push(item);
    }
}

/// Render `(heading, body)` pairs as a Markdown document, skipping empty ones.
fn sections_to_markdown(sections: &[(&str, &str)]) -> String {
    let mut out = String::new();
    for (heading, body) in sections {
        if body.trim().is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str("## ");
        out.push_str(heading);
        out.push_str("\n\n");
        out.push_str(body.trim());
    }
    out
}

// ── The manuscript ──────────────────────────────────────────────────────────

/// The words a reference marker should be replaced by.
///
/// Only the name: the link itself is resolved from the id maps at the moment the
/// row is decorated, so carrying a file id here would be a second answer to the
/// same question and one of the two would go stale.
type RefName = String;

impl Builder<'_> {
    /// Build the manuscript binder.
    fn build_manuscript(&mut self) -> Vec<BundledItem> {
        let binder = self.names.manuscript_binder.clone();
        let mut out = Vec::new();

        // A Book to hold it all. Manuskript has none, and pace planning, analysis
        // and every export scope are Book-shaped, so an import without one would
        // arrive with three features switched off for no reason the writer chose.
        let title = first_non_empty([
            self.project.info.title.as_str(),
            self.project.source_name.as_str(),
            "Imported Manuskript project",
        ]);
        let subtitle = self.project.info.subtitle.trim().to_string();
        let book_summary = self.book_synopsis();
        let (_, mut book) = self.make_item(
            MANUSCRIPT_INDEX,
            &binder,
            Role::Folder,
            SubRole::Book,
            &title,
            0,
            true,
            vec![
                (ContentRole::BookTitle, title.clone()),
                (ContentRole::BookSubtitle, subtitle.clone()),
                (ContentRole::SynopsisText, book_summary),
            ],
        );
        book.item.sub_title = subtitle;
        out.push(book);

        let depth = max_folder_depth(&self.project.outline);
        // Cloned so the walk can hold `&mut self` while reading the source. One
        // transient copy of the outline, taken once: the alternative is lifting
        // the project out of the builder and threading it through every `emit_*`,
        // which buys a few megabytes on a long novel and costs a parameter on
        // fifteen call sites. Measured against a real 13,000-word project the
        // whole import is well under a second either way.
        let outline = self.project.outline.clone();
        for item in &outline {
            self.emit_row(&binder, item, 0, depth, 1, true, &mut out);
        }

        // The book's closing marker, the same one every other importer emits.
        let (_, book_end) = self.make_item(
            MANUSCRIPT_INDEX,
            &binder,
            Role::Item,
            SubRole::BookEnd,
            "",
            1,
            true,
            Vec::new(),
        );
        out.push(book_end);

        self.resolve_outline_references(&mut out);
        self.report_what_had_no_home();
        out
    }

    /// The Book's own synopsis: the longest rung of the snowflake ladder.
    ///
    /// The longest rather than the last, because a writer who filled in Situation
    /// and stopped should still see it — `Full` being empty does not mean the
    /// ladder is.
    fn book_synopsis(&mut self) -> String {
        let longest = self
            .project
            .summary
            .rungs()
            .into_iter()
            .map(|(_, body)| body.to_string())
            .max_by_key(|body| body.trim().len())
            .unwrap_or_default();
        self.djot_of(&longest, "the book summary")
    }

    /// Emit one outline row and everything under it.
    ///
    /// `depth` is how many folders deep this row sits; `max_depth` the deepest
    /// chain in the project, which is what picks the ladder. `indent` is the
    /// binder indent, one greater than `depth` because the synthesised Book holds
    /// everything.
    #[allow(clippy::too_many_arguments)]
    fn emit_row(
        &mut self,
        binder: &str,
        row: &OutlineItem,
        depth: usize,
        max_depth: usize,
        indent: i64,
        inherited_compile: bool,
        out: &mut Vec<BundledItem>,
    ) {
        if self.cancelled {
            return;
        }
        self.tick(&row.title);
        let compiles = row.compiles(inherited_compile);

        if !row.custom_icon.trim().is_empty() {
            self.dropped_icons += 1;
        }

        let (role, sub_role, title_role) = if row.is_folder() {
            folder_shape(depth, max_depth)
        } else {
            (Role::Item, SubRole::Scene, None)
        };

        let synopsis_md = join_summaries(&row.summary_sentence, &row.summary_full);
        let synopsis = self.djot_of(&synopsis_md, &row.title);
        // Already Djot: `crate::prose` converted it at the reader boundary, and
        // converting again would read Djot as Markdown and quietly demote every
        // bold run to italic.
        let body = self.rewrite_prose(&row.text, &row.title);
        let text = body.text.clone();

        // Offered whatever the row turned out to be; `make_item` drops whichever
        // the combination does not allow. A Part keeps its title and synopsis and
        // loses the scene text; a plain grouping folder keeps only the synopsis.
        //
        // A chapter folder keeps an empty scene text on purpose: in this model a
        // chapter folder holds prose of its own, exactly as the flat encoding
        // does, so giving it the content row means the writer can type on the
        // chapter's own page the moment the project opens.
        let mut contents: Vec<(ContentRole, String)> = Vec::new();
        if let Some(role) = title_role {
            contents.push((role, row.title.clone()));
        }
        contents.push((ContentRole::SceneText, text));
        contents.push((ContentRole::SynopsisText, synopsis));

        let (id, mut item) = self.make_item(
            MANUSCRIPT_INDEX,
            binder,
            role,
            sub_role.clone(),
            &row.title,
            indent,
            compiles,
            contents,
        );
        self.decorate(&mut item, row, &body.references);
        out.push(item);

        if let Some(key) = row.id.clone() {
            self.outline_item.insert(key.clone(), id);
            self.outline_uid.insert(key, self.last_uid(out));
        }

        // A row's notes become a Note of their own: the matrix gives neither a
        // scene nor a chapter folder anywhere else to keep them. A folder's note
        // is its first child; a leaf's is the sibling that follows it, which is
        // where the reader meets it either way.
        let note_indent = if row.is_folder() { indent + 1 } else { indent };
        self.emit_notes(binder, row, note_indent, out);

        for child in &row.children {
            self.emit_row(
                binder,
                child,
                depth + 1,
                max_depth,
                indent + 1,
                compiles,
                out,
            );
        }
    }

    /// The uid of the row just pushed.
    fn last_uid(&self, out: &[BundledItem]) -> uuid::Uuid {
        out.last().map(|i| i.item.uid).unwrap_or_default()
    }

    /// Attach everything that is not content: the vocabulary, the goal, the POV
    /// and the links.
    fn decorate(&mut self, item: &mut BundledItem, row: &OutlineItem, references: &[Reference]) {
        if let Some(index) = row.label {
            match self.label_tag.get(&index) {
                Some(tag) => item.item.tag_ids = vec![*tag],
                None => self.warnings.push(format!(
                    "'{}' wears label {index}, which is not in this project's label list. It \
                     arrived without one.",
                    row.title
                )),
            }
        }
        if let Some(index) = row.status {
            match self.status_id.get(&index) {
                Some(rung) => item.item.status_id = Some(*rung),
                None => self.warnings.push(format!(
                    "'{}' is at status {index}, which is not in this project's status list. It \
                     arrived without one.",
                    row.title
                )),
            }
        }
        if let Some(goal) = row.set_goal {
            item.item.word_count_goal = goal;
        }
        if let Some(pov) = row.pov.as_deref() {
            match self.character_item.get(pov) {
                Some(character) => item.item.point_of_view_ids = vec![*character],
                None => self.warnings.push(format!(
                    "'{}' is told through a character who is not in this project's cast. Its \
                     point of view was not set.",
                    row.title
                )),
            }
        }
        let mut ids: Vec<u64> = Vec::new();
        for reference in references {
            match reference.kind {
                'C' => ids.extend(self.character_item.get(&reference.id).copied()),
                'W' => ids.extend(self.world_item.get(&reference.id).copied()),
                'P' => ids.extend(self.plot_item.get(&reference.id).copied()),
                // An outline row may not exist yet; resolved in a second pass.
                'T' => self
                    .pending_outline_refs
                    .push((item.item.file_id, reference.id.clone())),
                _ => {}
            }
        }
        ids.sort_unstable();
        ids.dedup();
        item.item.reference_ids = ids;
    }

    /// Replace inline reference markers with the words they stand for, recording
    /// what they named.
    fn rewrite_prose(&self, text: &str, what: &str) -> refs::Scanned {
        if text.is_empty() {
            return refs::Scanned::default();
        }
        let _ = what;
        let targets = &self.reference_targets;
        refs::rewrite(text, |reference| {
            targets
                .get(&(reference.kind, reference.id.clone()))
                .cloned()
        })
    }

    /// Everything a `{C:…}`, `{W:…}`, `{P:…}` or `{T:…}` marker can point at.
    fn build_reference_targets(&mut self) {
        let mut out: HashMap<(char, String), RefName> = HashMap::new();
        for character in &self.project.characters {
            if let Some(id) = character.id.clone() {
                out.insert(('C', id), character.name.clone());
            }
        }
        collect_world_targets(&self.project.world, &mut out);
        for plot in &self.project.plots {
            if let Some(id) = plot.id.clone() {
                out.insert(('P', id), plot.name.clone());
            }
        }
        collect_outline_targets(&self.project.outline, &mut out);
        self.reference_targets = out;
    }

    /// Attach the `{T:…}` links now that every row exists.
    fn resolve_outline_references(&mut self, out: &mut [BundledItem]) {
        let pending = std::mem::take(&mut self.pending_outline_refs);
        for (from, target) in pending {
            let Some(to) = self.outline_item.get(&target).copied() else {
                continue;
            };
            if to == from {
                continue; // a row referring to itself is not a link
            }
            if let Some(item) = out.iter_mut().find(|i| i.item.file_id == from)
                && !item.item.reference_ids.contains(&to)
            {
                item.item.reference_ids.push(to);
                item.item.reference_ids.sort_unstable();
            }
        }
    }

    /// A row's notes, as a Note item.
    fn emit_notes(
        &mut self,
        binder: &str,
        row: &OutlineItem,
        indent: i64,
        out: &mut Vec<BundledItem>,
    ) {
        if row.notes.trim().is_empty() {
            return;
        }
        // Notes are plain text in Manuskript, with no `type` of their own, so
        // unlike the body they are converted here rather than at the reader.
        let scanned = self.rewrite_prose(&row.notes, &row.title);
        let note = self.djot_of(&scanned.text, &row.title);
        let title = format!("{} (notes)", row.title);
        let (_, mut item) = self.make_item(
            MANUSCRIPT_INDEX,
            binder,
            Role::Item,
            SubRole::Note,
            &title,
            indent,
            // A note is not part of the book, whichever door made it.
            false,
            vec![(ContentRole::NoteText, note)],
        );
        let mut ids: Vec<u64> = Vec::new();
        for reference in &scanned.references {
            match reference.kind {
                'C' => ids.extend(self.character_item.get(&reference.id).copied()),
                'W' => ids.extend(self.world_item.get(&reference.id).copied()),
                'P' => ids.extend(self.plot_item.get(&reference.id).copied()),
                'T' => self
                    .pending_outline_refs
                    .push((item.item.file_id, reference.id.clone())),
                _ => {}
            }
        }
        ids.sort_unstable();
        ids.dedup();
        item.item.reference_ids = ids;
        out.push(item);
    }

    /// Say once what had nowhere to go, rather than once per row.
    fn report_what_had_no_home(&mut self) {
        if self.dropped_icons > 0 {
            let n = self.dropped_icons;
            self.warnings.push(format!(
                "{n} row(s) carried a custom icon. Skribisto gives a row its icon from what the \
                 row is, so the choice was not imported; nothing else about those rows changed."
            ));
        }
        if self.dropped_colors > 0 {
            let n = self.dropped_colors;
            self.warnings.push(format!(
                "{n} character(s) had a colour beside their name. Skribisto colours tags rather \
                 than rows, so the colours were not imported; the characters themselves all \
                 arrived."
            ));
        }
    }

    /// Turn the recorded revisions into the project's version history.
    ///
    /// Manuskript stores whole-document snapshots with a unix timestamp, which is
    /// exactly what a history entry is here, so an imported project opens with a
    /// Versions dock that already has something in it. Thinned by the same
    /// retention the app applies to its own history: a real novel's
    /// `revisions.xml` has been measured in the tens of megabytes, and importing
    /// all of it would make the new project larger than the old one.
    fn build_history(&mut self) -> skrib_format::history::HistoryLog {
        use skrib_format::history::{DEFAULT_MIN_KEEP, DEFAULT_POLICY, HistoryEntry, HistoryLog};

        let mut log = HistoryLog::default();
        if self.project.revisions.is_empty() {
            return log;
        }
        let mut revisions = self.project.revisions.clone();
        // Oldest first, which is the order the log is kept in.
        revisions.sort_by_key(|r| r.timestamp);

        let mut unplaceable = 0usize;
        for revision in &revisions {
            let Some(uid) = self.outline_uid.get(&revision.item_id).copied() else {
                unplaceable += 1;
                continue;
            };
            let Some(at) = chrono::DateTime::from_timestamp(revision.timestamp, 0) else {
                unplaceable += 1;
                continue;
            };
            // Djot already, for the same reason the row's own body is.
            let djot = revision.text.clone();
            let hash = blake3::hash(djot.as_bytes()).to_hex().to_string();
            log.entries.push(HistoryEntry {
                at: at.to_rfc3339(),
                item_uid: uid,
                role: ContentRole::SceneText,
                hash: hash.clone(),
                bytes: djot.len() as u64,
                thinned_away: 0,
            });
            log.blobs.entry(hash).or_insert(djot);
        }

        let found = log.entries.len();
        skrib_format::history::thin(
            &mut log,
            &DEFAULT_POLICY,
            DEFAULT_MIN_KEEP,
            chrono::Utc::now(),
        );
        let kept = log.entries.len();
        if kept < found {
            self.warnings.push(format!(
                "This project recorded {found} earlier versions; {kept} were kept, thinned the \
                 same way Skribisto thins its own history. They are in the Versions panel."
            ));
        }
        if unplaceable > 0 {
            self.warnings.push(format!(
                "{unplaceable} earlier version(s) named a row that is no longer in the project \
                 and could not be placed."
            ));
        }
        log
    }
}

/// The `(role, sub_role, title content)` a folder takes at `depth`.
///
/// See this module's header for why depth and not the label. `max_depth` is the
/// deepest chain of nested folders in the project, so a flat set of chapter
/// folders becomes chapters and a project with parts above them becomes parts and
/// chapters. Below the chapter rung the ladder runs out: a folder inside a chapter
/// is a plain grouping folder, which claims nothing the model cannot express and
/// leaves the scenes inside it belonging to the chapter above.
fn folder_shape(depth: usize, max_depth: usize) -> (Role, SubRole, Option<ContentRole>) {
    if max_depth <= 1 {
        return (
            Role::Folder,
            SubRole::ChapterScene,
            Some(ContentRole::ChapterTitle),
        );
    }
    match depth {
        0 => (Role::Folder, SubRole::Part, Some(ContentRole::PartTitle)),
        1 => (
            Role::Folder,
            SubRole::ChapterScene,
            Some(ContentRole::ChapterTitle),
        ),
        _ => (Role::Folder, SubRole::None, None),
    }
}

/// The longest chain of nested folders under `items`.
fn max_folder_depth(items: &[OutlineItem]) -> usize {
    items
        .iter()
        .map(|item| {
            if item.is_folder() {
                1 + max_folder_depth(&item.children)
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0)
}

/// Fold Manuskript's two summaries into the one synopsis Skribisto keeps.
///
/// The sentence leads and the fuller telling follows, which is the order they are
/// written in and the order they read in. Either alone is used alone; neither is
/// dropped in favour of the other.
fn join_summaries(sentence: &str, full: &str) -> String {
    match (sentence.trim(), full.trim()) {
        ("", "") => String::new(),
        (s, "") => s.to_string(),
        ("", f) => f.to_string(),
        (s, f) => format!("{s}\n\n{f}"),
    }
}

fn collect_world_targets(items: &[WorldItem], out: &mut HashMap<(char, String), RefName>) {
    for item in items {
        if let Some(id) = item.id.clone() {
            out.insert(('W', id), item.name.clone());
        }
        collect_world_targets(&item.children, out);
    }
}

fn collect_outline_targets(items: &[OutlineItem], out: &mut HashMap<(char, String), RefName>) {
    for item in items {
        if let Some(id) = item.id.clone() {
            out.insert(('T', id), item.title.clone());
        }
        collect_outline_targets(&item.children, out);
    }
}
