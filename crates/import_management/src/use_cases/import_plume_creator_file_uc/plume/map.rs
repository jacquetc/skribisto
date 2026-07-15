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
    BinderFile, BinderItemFile, BundledBinder, BundledItem, DictWordFile, FORMAT_VERSION,
    InlineContent, ProjectManifest, ProseRef, ShapeTag, WorkBundle, WorkFile, binder_dir_name,
    html_to_djot, new_unique_id, prose_file_name, prose_kind, prose_relpath,
};
use skribisto_model::content_allowed;

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
        author_name: String::new(),
        dict_language: String::new(),
        tag_ids: Vec::new(),
        dict_word_ids: dict.iter().map(|d| d.file_id).collect(),
        unique_id: new_unique_id(),
        // Plume Creator organises chapters as folders of sheets → folder mode.
        chapter_flat: false,
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
            shape: ShapeTag::Zip,
            work,
            binder_order,
            // An imported project is a regular project, not a backup.
            kind: skrib_format::BundleKind::Regular,
            backup_of: None,
            backup_created_at: None,
        },
        tags: Vec::new(),
        dict_words: dict,
        trash_infos: Vec::new(),
        binders,
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

struct Builder<'a> {
    ids: IdGen,
    source: &'a PlumeSource,
    now: String,
    /// Plume attendance obj `number` → the story-bible note item's file id.
    attend_id_map: HashMap<u32, u64>,
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
                let (oid, obi) = self.make_item(
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
            PlumeKind::Separator => self.emit_separator(node, indent, bindex, bname, out),
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
        let (_id, bi) = self.make_item(
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
        let (_id, bi) = self.make_item(
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
        let (_id, bi) = self.make_item(
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
        out.push(bi);
        self.emit_sibling_note(node, indent, bindex, bname, out);
    }

    fn emit_separator(
        &mut self,
        node: &PlumeNode,
        indent: i64,
        bindex: usize,
        bname: &str,
        out: &mut Vec<BundledItem>,
    ) {
        // A separator has no valid content target (Item/Text carries none). Keep it
        // as a titled marker; warn if it unexpectedly held prose.
        if !self.text_djot(node.number).is_empty()
            || !self.synopsis_djot(node.number).is_empty()
            || !self.note_djot(node.number).is_empty()
        {
            self.warnings.push(format!(
                "separator '{}' carried text that has no place in the writing model and was dropped",
                node.name
            ));
        }
        let (_id, bi) = self.make_item(
            bindex,
            bname,
            Role::Item,
            SubRole::Text,
            &node.name,
            indent,
            true,
            Vec::new(),
            Vec::new(),
            "",
        );
        out.push(bi);
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
                    file_id: content_id,
                    created_at: self.now.clone(),
                    updated_at: self.now.clone(),
                    activated: true,
                    role: content_role,
                    text: data,
                }),
                Some(_) => {
                    let name = prose_file_name(content_id, title, &content_role)
                        .expect("prose_kind matched");
                    prose_refs.push(ProseRef {
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
                    indent,
                    word_count_goal: 0,
                    char_count_goal: 0,
                    dict_language: String::new(),
                    inline_contents,
                    prose_refs,
                    reference_ids,
                    tag_ids: Vec::new(),
                },
                prose,
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

/// Compose an attendance obj's synopsis (plain Djot text, *not* HTML): its
/// quick-details paragraph, then a metadata line of aliases · box labels ·
/// `<spinbox_label> <value>`.
fn build_obj_synopsis(obj: &PlumeObj, spinbox_label: &str) -> String {
    let mut meta: Vec<String> = Vec::new();
    if !obj.aliases.is_empty() {
        meta.push(obj.aliases.clone());
    }
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
