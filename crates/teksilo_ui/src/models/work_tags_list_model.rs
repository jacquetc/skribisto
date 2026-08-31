// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the open Work's tag palette (`BinderTag`).
//!
//! The public surface is a `teksilo::data::ListModel<TagRow>` a `ListView` binds to (the
//! Settings ▸ Tags pane), a `version` signal for consumers that observe rather than bind,
//! and a **lookup** signal every chip renderer reads — see [`lookup_signal`](WorkTagsListModel::lookup_signal).
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface: the real one reads via
//! `Work.tags` (scoped to this window's own open Work — a process can host several) and
//! stays live on `BinderTag` events, tag-management events and project switches, with
//! writes going through the generated `binder_tag_commands` plus
//! `tag_management_commands::import_tags` for the bulk path; the mock one holds a
//! fabricated palette and mutates it in place, since `--features mocks` has no real
//! `Work` to own a created `BinderTag`.
//!
//! Rows are sorted case-insensitively by name. That ordering is load-bearing rather than
//! cosmetic: it is what makes the `status/…` naming convention cluster in every list,
//! which is why no explicit ordering was added to the entity.
//!
//! No text colour here — it is derived from `color` at paint time (see
//! `crate::tags::contrast`), never stored.

use std::collections::HashMap;

/// One palette row.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TagRow {
    pub id: u64,
    /// The durable identity, for anything that outlives a session. A store id is
    /// re-minted by every `load_work`, so the capture recents key on this instead.
    pub uid: uuid::Uuid,
    pub name: String,
    /// Background colour as the writer chose it, `#rrggbb`.
    pub color: String,
    /// A line or two on what the tag means; shown in its hover tooltip.
    pub details: String,
    /// Items carrying this tag are story-bible material for the mention index.
    pub discoverable: bool,
    /// Where a note created under this tag lands, and what shape it starts in. Both
    /// are the writer's own filing, set on the Tags page or answered once the first
    /// time they file under the tag, and both are what let "Add as note" ask a single
    /// question: picking the tag settles the destination and the template with it.
    ///
    /// Read-only here. They are relationships, so they are written through
    /// [`crate::tags::TagsViewModel::set_creates_in`] rather than the scalar `update` path the
    /// other four fields share.
    pub creates_in: Option<u64>,
    pub note_template: Option<u64>,
}

/// Sort key: case-insensitive, then exact, so equal-fold names keep a deterministic order.
///
/// `pub` so chip renderers restoring palette order after an id-keyed lookup use
/// *this* comparator rather than restating it — the ordering is load-bearing (it is what
/// makes `status/…` cluster), so two spellings of it would be two orderings.
pub fn sort_rows(rows: &mut [TagRow]) {
    rows.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.name.cmp(&b.name))
    });
}

/// The comparison key for "is this name already taken". Trimmed and lowercased — the
/// backend permits duplicates, so this only drives the UI's warning and import's skip.
pub use crate::shared::list_naming::name_key;

/// The row whose name collides with `candidate`, ignoring case and surrounding space,
/// excluding `exclude` (a tag being renamed never collides with itself).
///
/// A free function beside [`name_key`] and [`sort_rows`], not a method, because the real
/// and mock `imp` both need it and both had their own verbatim copy — two chances for the
/// duplicate-name warning to behave differently in the app than in every test that covers
/// it, which is the one place the difference would never be noticed.
pub fn colliding_name(rows: &[TagRow], candidate: &str, exclude: Option<u64>) -> Option<String> {
    crate::shared::list_naming::colliding_name(rows, candidate, exclude)
}

impl crate::shared::list_naming::NamedRow for TagRow {
    fn row_id(&self) -> u64 {
        self.id
    }
    fn row_name(&self) -> &str {
        &self.name
    }
}

fn build_lookup(rows: &[TagRow]) -> HashMap<u64, TagRow> {
    rows.iter().map(|r| (r.id, r.clone())).collect()
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::collections::HashMap;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_tag_commands, tag_management_commands, work_commands};
    use frontend::common::direct_access::binder_tag::BinderTagRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, Origin, TagManagementEvent, WorkManagementEvent,
    };
    use frontend::direct_access::BinderTagRelationshipDto;
    use frontend::direct_access::{CreateBinderTagDto, UpdateBinderTagDto};
    use frontend::tag_management::ImportTagsDto;

    use crate::app_ids::AppIds;

    use super::{TagRow, build_lookup, sort_rows};

    struct Inner {
        model: ListModel<TagRow>,
        version: Signal<u64>,
        lookup: Signal<Rc<HashMap<u64, TagRow>>>,
        ctx: Rc<AppContext>,
        /// The open Work's own ids — scopes every read to *this* Work's `tags`
        /// relationship (see [`load_rows`]); a second simultaneously-open Work's
        /// tag palette must never merge into this one's list.
        ids: AppIds,
    }

    #[derive(Clone)]
    pub struct WorkTagsListModel {
        inner: Rc<Inner>,
    }

    impl WorkTagsListModel {
        pub fn new(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            let rows = load_rows(&ctx, ids.work_id.get());
            let lookup = Signal::new(Rc::new(build_lookup(&rows)));
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(rows),
                    version: Signal::new(0),
                    lookup,
                    ctx,
                    ids,
                }),
            }
        }

        /// Subscribe so the palette stays live: any `BinderTag` mutation — from the
        /// settings pane, the inspector's "New tag…", a preset, or an undo/redo — re-reads
        /// it, and a project switch replaces it wholesale.
        ///
        /// **Re-subscribe on every call.** `BuildContext::subscribe_event` is scoped to the
        /// current build and dropped on the next one (`App::build` re-wires every rebuild).
        /// A one-shot "subscribed" guard left the palette deaf after the first App rebuild:
        /// `LoadWork` never refreshed, so Settings ▸ Tags stayed empty until a local create
        /// happened to fire `BinderTag::Created` through a *new* subscription path.
        ///
        /// Always `refresh()` at the end so a Work opened before this model was wired (or
        /// after `work_id` was re-seeded) lands its tags without waiting for an event.
        ///
        /// `BinderTag`/`ImportTags` carry no `work_id`, but `refresh` always re-derives
        /// from this model's own `ids.work_id` (see [`load_rows`]), so a sibling Work's
        /// tag mutation only ever costs a harmless, still-correct re-read. `LoadWork`/
        /// `NewWork`/`CloseWork` DO carry `work_id` and are guarded accordingly.
        pub fn wire(&self, ctx: &mut BuildContext) {
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::BinderTag(ev)),
                    move |_event: &Event| me.refresh(),
                );
            }
            // `import_tags` creates through its own unit of work, so it publishes a feature
            // event rather than N per-entity `Created` ones.
            let me = self.clone();
            ctx.subscribe_event(
                Origin::TagManagement(TagManagementEvent::ImportTags),
                move |_event: &Event| me.refresh(),
            );
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    if !me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    // Prefer the already-seeded work_id. Fall back to the event's id:
                    // `tags.wire` historically registered *before* ProjectLifecycle's
                    // LoadWork seed, so a bare `ids.work_id` refresh ran with `None`
                    // and left the palette empty until the next BinderTag create.
                    let work_id = me
                        .inner
                        .ids
                        .work_id
                        .get()
                        .or_else(|| event.ids.first().copied());
                    me.refresh_for(work_id);
                });
            }
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::WorkManagement(WorkManagementEvent::CloseWork),
                    move |event: &Event| {
                        if me.inner.ids.is_event_for_my_work(&event.ids) {
                            me.refresh_for(None)
                        }
                    },
                );
            }
            // Catch up when this window is already seeded (rebuild / late wire).
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// The reactive model to bind a `ListView` to (through the pane's
        /// `SortFilterListModel` search projection).
        pub fn list_model(&self) -> ListModel<TagRow> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh; for consumers that observe rather than bind the model.
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// id → row, rebuilt once per refresh.
        ///
        /// Every chip on every stream row and corkboard card resolves its tag through this.
        /// A linear scan of the model per chip would be O(rows × tags) per frame; one map
        /// rebuilt per *mutation* is O(tags) per mutation and O(1) per chip.
        pub fn lookup_signal(&self) -> Signal<Rc<HashMap<u64, TagRow>>> {
            self.inner.lookup.clone()
        }

        /// Current rows, sorted — for export, dedup and the preset diff.
        pub fn rows(&self) -> Vec<TagRow> {
            snapshot(&self.inner.model)
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// The row whose name collides with `candidate`, ignoring case and surrounding
        /// space, excluding `exclude` (the tag being renamed never collides with itself).
        pub fn colliding_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
            super::colliding_name(&self.rows(), candidate, exclude)
        }

        /// Create one tag under `owner_id`. `None` when no project is open or the write
        /// failed. The backend's `Created` event drives the refresh.
        pub fn create(
            &self,
            name: &str,
            color: &str,
            details: &str,
            discoverable: bool,
            owner_id: Option<u64>,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let owner = owner_id?;
            let now = chrono::Utc::now();
            let dto = CreateBinderTagDto {
                uid: Default::default(),
                created_at: now,
                updated_at: now,
                name: name.trim().to_string(),
                color: color.to_string(),
                details: details.to_string(),
                discoverable,
                // A tag is born unfiled and untemplated: the writer sets both on the
                // Tags page, or answers once the first time they file under it.
                creates_in: None,
                note_template: None,
            };
            match binder_tag_commands::create_binder_tag(&self.inner.ctx, stack_id, &dto, owner, -1)
            {
                Ok(created) => Some(created.id),
                Err(e) => {
                    eprintln!("tags: create failed: {e}");
                    None
                }
            }
        }

        /// Patch one tag. Every field is sent, so callers pass the row's current values
        /// for whatever they are not changing (`update_binder_tag` replaces all scalars).
        ///
        /// Read-modify-write off the live entity rather than off a `TagRow`: the row does
        /// not carry `created_at` (no renderer wants it), and inventing one here would
        /// reset the tag's creation time on every rename — the same silent-wrong-timestamp
        /// bug that had every progress snapshot stamped 1970.
        pub fn update(
            &self,
            id: u64,
            name: &str,
            color: &str,
            details: &str,
            discoverable: bool,
            stack_id: Option<u64>,
        ) {
            let existing = match binder_tag_commands::get_binder_tag(&self.inner.ctx, &id) {
                Ok(Some(t)) => t,
                Ok(None) => return,
                Err(e) => {
                    eprintln!("tags: update failed to read {id}: {e}");
                    return;
                }
            };
            let dto = UpdateBinderTagDto {
                // Carried through unchanged: a nil here would write over the row's durable
                // identity on every edit, orphaning anything that references it.
                uid: existing.uid,
                id,
                created_at: existing.created_at,
                updated_at: chrono::Utc::now(),
                name: name.trim().to_string(),
                color: color.to_string(),
                details: details.to_string(),
                discoverable,
            };
            if let Err(e) = binder_tag_commands::update_binder_tag(&self.inner.ctx, stack_id, &dto)
            {
                eprintln!("tags: update failed: {e}");
            }
        }

        /// Point a tag at the folder its notes are created in, or clear it.
        ///
        /// A **relationship**, so it cannot ride the scalar `update` above: that one
        /// rewrites the row's own columns, and a reference lives in a junction table
        /// where the generated back-reference sweep can reach it. Clearing is an empty
        /// slice rather than a separate call, which is what the generated
        /// `SetRelationship` takes.
        ///
        /// Undoable on the shared stack like every other tag edit, so a writer who
        /// answers the first-use question and immediately regrets it can walk it back.
        /// The context this palette reads through. Threaded to surfaces that must walk
        /// the binder beside the palette (the Tags page's destination picker), so they
        /// need not reach for `app_state` and get another Work's.
        pub fn app_ctx(&self) -> Rc<AppContext> {
            self.inner.ctx.clone()
        }

        pub fn set_relationship(
            &self,
            id: u64,
            field: BinderTagRelationshipField,
            target: Option<u64>,
            stack_id: Option<u64>,
        ) {
            let dto = BinderTagRelationshipDto {
                id,
                field,
                right_ids: target.into_iter().collect(),
            };
            if let Err(e) =
                binder_tag_commands::set_binder_tag_relationship(&self.inner.ctx, stack_id, &dto)
            {
                eprintln!("tags: set relationship failed: {e}");
            }
        }

        /// Remove the given ids in one undoable step. The generated `remove_multi` scrubs
        /// the item junction too, so no manual detach is needed. Missing ids are a safe
        /// no-op (a stale Undo after a manual delete).
        pub fn remove_all(&self, ids: &[u64], stack_id: Option<u64>) {
            let existing: std::collections::HashSet<u64> =
                self.rows().into_iter().map(|r| r.id).collect();
            let present: Vec<u64> = ids
                .iter()
                .copied()
                .filter(|id| existing.contains(id))
                .collect();
            if present.is_empty() {
                return;
            }
            if let Err(e) =
                binder_tag_commands::remove_binder_tag_multi(&self.inner.ctx, stack_id, &present)
            {
                eprintln!("tags: remove failed: {e}");
            }
        }

        /// Bulk-create as ONE undoable step (a CSV import, or applying a preset), skipping
        /// names already present. Returns the names that were skipped, for the summary.
        pub fn import(&self, rows: &[TagRow], work_id: u64, stack_id: Option<u64>) -> Vec<String> {
            if rows.is_empty() {
                return Vec::new();
            }
            let dto = ImportTagsDto {
                work_id,
                names: rows.iter().map(|r| r.name.clone()).collect(),
                colors: rows.iter().map(|r| r.color.clone()).collect(),
                details: rows.iter().map(|r| r.details.clone()).collect(),
                discoverables: rows.iter().map(|r| r.discoverable).collect(),
            };
            match tag_management_commands::import_tags(&self.inner.ctx, stack_id, &dto) {
                Ok(res) => res.skipped_names,
                Err(e) => {
                    eprintln!("tags: import failed: {e}");
                    rows.iter().map(|r| r.name.clone()).collect()
                }
            }
        }

        /// Re-read the palette for the currently seeded Work (`None` → empty).
        pub fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        /// Re-read the palette for an explicit Work id (e.g. a LoadWork event that
        /// has not yet been written into `AppIds` by the lifecycle seed).
        fn refresh_for(&self, work_id: Option<u64>) {
            let rows = load_rows(&self.inner.ctx, work_id);
            self.inner.lookup.set(Rc::new(build_lookup(&rows)));
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// Read one Work's `BinderTag`s (via `Work.tags`), into sorted rows — **not**
    /// `get_all_binder_tag`, which returns every tag in the whole shared store:
    /// with a second Work simultaneously open, that would leak one Work's tag
    /// palette into the other's Inspector and Settings ▸ Tags pane.
    fn load_rows(ctx: &AppContext, work_id: Option<u64>) -> Vec<TagRow> {
        let Some(work_id) = work_id else {
            return Vec::new(); // no project open
        };
        let tag_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Tags)
                .unwrap_or_default();
        let mut rows: Vec<TagRow> = binder_tag_commands::get_binder_tag_multi(ctx, &tag_ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|t| TagRow {
                id: t.id,
                uid: t.uid,
                name: t.name,
                color: t.color,
                details: t.details,
                discoverable: t.discoverable,
                creates_in: t.creates_in,
                note_template: t.note_template,
            })
            .collect();
        sort_rows(&mut rows);
        rows
    }

    fn snapshot(model: &ListModel<TagRow>) -> Vec<TagRow> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::collections::HashMap;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::common::direct_access::binder_tag::BinderTagRelationshipField;

    use crate::app_ids::AppIds;

    use super::{TagRow, build_lookup, name_key, sort_rows};

    struct Inner {
        model: ListModel<TagRow>,
        version: Signal<u64>,
        lookup: Signal<Rc<HashMap<u64, TagRow>>>,
        next_id: Cell<u64>,
        /// Kept rather than dropped so `app_ctx` answers in both builds. A mock palette
        /// still sits in a real `AppContext`; what it lacks is a seeded Work, not a
        /// context.
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct WorkTagsListModel {
        inner: Rc<Inner>,
    }

    fn row(id: u64, name: &str, color: &str, details: &str, discoverable: bool) -> TagRow {
        TagRow {
            id,
            // Stable per row rather than nil, so a mock build's capture recents (which
            // key on uid) can tell two mock tags apart.
            uid: uuid::Uuid::from_u128(id as u128),
            name: name.to_string(),
            color: color.to_string(),
            details: details.to_string(),
            discoverable,
            creates_in: None,
            note_template: None,
        }
    }

    impl WorkTagsListModel {
        pub fn new(ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            // A palette shaped like the Basic preset: the flags, and the discoverable
            // taxonomy. No `status/…` rungs — a workflow stage is single-valued and
            // ordered, so it has its own axis now (`crate::statuses`), and a mock palette
            // that still carried one would show a status-shaped *tag* beside a real status
            // glyph in the very build meant for judging how the two read together.
            let mut rows = vec![
                row(
                    1,
                    "continuity check",
                    "#c0392b",
                    "Verify against what came before",
                    false,
                ),
                row(2, "plot point", "#c2185b", "", false),
                row(3, "cut candidate", "#95a5a6", "", false),
                row(
                    4,
                    "needs research",
                    "#f39c12",
                    "Check this before publishing",
                    false,
                ),
                row(5, "character", "#2980b9", "A person in the story", true),
                row(6, "place", "#8e44ad", "", true),
            ];
            sort_rows(&mut rows);
            let lookup = Signal::new(Rc::new(build_lookup(&rows)));
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(rows),
                    version: Signal::new(0),
                    lookup,
                    next_id: Cell::new(7),
                    ctx,
                }),
            }
        }

        pub fn app_ctx(&self) -> Rc<AppContext> {
            self.inner.ctx.clone()
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn refresh(&self) {
            // Mock palette is in-memory and always "loaded".
        }

        pub fn list_model(&self) -> ListModel<TagRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn lookup_signal(&self) -> Signal<Rc<HashMap<u64, TagRow>>> {
            self.inner.lookup.clone()
        }

        pub fn rows(&self) -> Vec<TagRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        pub fn colliding_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
            super::colliding_name(&self.rows(), candidate, exclude)
        }

        pub fn create(
            &self,
            name: &str,
            color: &str,
            details: &str,
            discoverable: bool,
            _owner_id: Option<u64>,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = self.inner.next_id.get();
            self.inner.next_id.set(id + 1);
            let mut rows = self.rows();
            rows.push(row(id, name.trim(), color, details, discoverable));
            self.replace(rows);
            Some(id)
        }

        pub fn update(
            &self,
            id: u64,
            name: &str,
            color: &str,
            details: &str,
            discoverable: bool,
            _stack_id: Option<u64>,
        ) {
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == id) {
                r.name = name.trim().to_string();
                r.color = color.to_string();
                r.details = details.to_string();
                r.discoverable = discoverable;
            }
            self.replace(rows);
        }

        pub fn set_relationship(
            &self,
            id: u64,
            field: BinderTagRelationshipField,
            target: Option<u64>,
            _stack_id: Option<u64>,
        ) {
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == id) {
                match field {
                    BinderTagRelationshipField::CreatesIn => r.creates_in = target,
                    BinderTagRelationshipField::NoteTemplate => r.note_template = target,
                }
            }
            self.replace(rows);
        }

        pub fn remove_all(&self, ids: &[u64], _stack_id: Option<u64>) {
            let keep: Vec<TagRow> = self
                .rows()
                .into_iter()
                .filter(|r| !ids.contains(&r.id))
                .collect();
            self.replace(keep);
        }

        pub fn import(
            &self,
            rows: &[TagRow],
            _work_id: u64,
            _stack_id: Option<u64>,
        ) -> Vec<String> {
            let mut current = self.rows();
            let mut skipped = Vec::new();
            for r in rows {
                if r.name.trim().is_empty()
                    || current
                        .iter()
                        .any(|e| name_key(&e.name) == name_key(&r.name))
                {
                    skipped.push(r.name.clone());
                    continue;
                }
                let id = self.inner.next_id.get();
                self.inner.next_id.set(id + 1);
                current.push(row(id, r.name.trim(), &r.color, &r.details, r.discoverable));
            }
            self.replace(current);
            skipped
        }

        fn replace(&self, mut rows: Vec<TagRow>) {
            sort_rows(&mut rows);
            self.inner.lookup.set(Rc::new(build_lookup(&rows)));
            self.inner.model.replace_all(rows);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }
}

pub use imp::WorkTagsListModel;

#[cfg(test)]
mod tests {
    use super::*;

    fn r(id: u64, name: &str) -> TagRow {
        TagRow {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// A palette has no stored order, so it is shown alphabetically and that is the whole
    /// contract. (It used to also demonstrate a `status/…` prefix clustering; the ladder
    /// that convention stood in for is a real, ordered axis now — see `crate::statuses` —
    /// and prefix-sorting a tag name is no longer a thing anything relies on.)
    #[test]
    fn sorting_is_alphabetical_and_case_insensitive() {
        let mut rows = vec![
            r(1, "place"),
            r(2, "Artifact"),
            r(3, "character"),
            r(4, "needs research"),
        ];
        sort_rows(&mut rows);
        let names: Vec<&str> = rows.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["Artifact", "character", "needs research", "place"]
        );
    }

    #[test]
    fn sorting_is_case_insensitive_but_deterministic() {
        let mut rows = vec![r(1, "beta"), r(2, "Alpha"), r(3, "alpha")];
        sort_rows(&mut rows);
        let names: Vec<&str> = rows.iter().map(|r| r.name.as_str()).collect();
        // Equal-fold names keep a stable relative order rather than swapping per run.
        assert_eq!(names, vec!["Alpha", "alpha", "beta"]);
    }

    #[test]
    fn name_key_ignores_case_and_surrounding_space() {
        assert_eq!(name_key("  CHARACTER  "), "character");
        assert_eq!(name_key("character"), name_key("Character"));
        assert_eq!(name_key("   "), "");
    }

    /// The palette behind the duplicate-name warning tests, shaped like the fixture the
    /// live probe uses: a one-letter tag whose case is what makes the check interesting.
    fn palette() -> Vec<TagRow> {
        vec![r(1, "A"), r(2, "B"), r(3, "very looooooooooong tag")]
    }

    /// Typing a name that already exists must warn — this is the whole point of the
    /// feature, and it is what the settings pane's add field and every inline rename bind
    /// to. Duplicates stay *legal*; this only decides whether the writer is told.
    #[test]
    fn an_exact_name_collides() {
        assert_eq!(colliding_name(&palette(), "B", None).as_deref(), Some("B"));
    }

    /// The case-insensitive half, stated separately because it is the one a naive
    /// implementation gets wrong and the one the live probe exercises: tag "A" exists, the
    /// writer types "a".
    #[test]
    fn a_differently_cased_name_collides_and_reports_the_existing_spelling() {
        assert_eq!(
            colliding_name(&palette(), "a", None).as_deref(),
            Some("A"),
            "the warning names the tag as it is actually spelled, not as it was typed"
        );
    }

    #[test]
    fn surrounding_space_does_not_hide_a_collision() {
        assert_eq!(
            colliding_name(&palette(), "  b  ", None).as_deref(),
            Some("B")
        );
    }

    /// Renaming a tag must not warn that it collides with itself — that would fire on
    /// every keystroke of every rename that did not change the name.
    #[test]
    fn a_tag_never_collides_with_itself() {
        assert_eq!(colliding_name(&palette(), "A", Some(1)), None);
        // …but it still collides with a *different* row of the same name.
        let mut two = palette();
        two.push(r(4, "a"));
        assert_eq!(colliding_name(&two, "A", Some(1)).as_deref(), Some("a"));
    }

    #[test]
    fn a_novel_name_does_not_collide() {
        assert_eq!(colliding_name(&palette(), "character", None), None);
    }

    /// Blank is not a collision. Without this the add field would warn the instant it was
    /// cleared, and every empty inline rename would sit permanently warned.
    #[test]
    fn a_blank_candidate_never_collides() {
        assert_eq!(colliding_name(&palette(), "", None), None);
        assert_eq!(colliding_name(&palette(), "   ", None), None);
    }

    #[test]
    fn an_empty_palette_has_nothing_to_collide_with() {
        assert_eq!(colliding_name(&[], "anything", None), None);
    }
}
