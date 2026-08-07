// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reactive list model over the open Work's note templates (`NoteTemplate`).
//!
//! The public surface is a `teksilo::data::ListModel<TemplateRow>` a `ListView` binds to
//! (Settings ▸ Work ▸ Templates) plus a `version` signal for consumers that observe rather
//! than bind — the insert menu among them.
//!
//! Two `#[cfg]`-gated `mod imp` variants share one public surface, exactly as
//! [`super::work_tags_list_model`] does: the real one reads this Work's `note_templates`
//! relationship and stays live on `NoteTemplate` events, the feature's own import event and
//! project switches; the mock one holds a fabricated set and mutates it in place.
//!
//! **Rows keep their relationship order, deliberately unlike the tag palette.** Tags sort
//! alphabetically because that is what makes the `status/…` convention cluster; a template
//! list is arranged *by the writer*, so the stored order is the order — reordering is a
//! first-class operation here (see [`WorkNoteTemplatesListModel::move_by`]) and would be
//! meaningless under a sort. Starred-first is a **view** concern belonging to the insert
//! menu ([`starred_first`]), never to the stored order: floating a starred row to the top
//! of the settings list too would make the star silently rewrite the arrangement the
//! writer just made by hand.

/// One template row.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TemplateRow {
    pub id: u64,
    pub name: String,
    /// The Djot body inserted at the caret, verbatim.
    pub body: String,
    /// Floated to the top of the insert menu (not of the settings list).
    pub starred: bool,
}

/// The comparison key for "is this name already taken". Trimmed and lowercased, matching
/// `import_note_templates`' own `name_key`, so the backend and the UI agree on what counts
/// as a collision.
pub fn name_key(name: &str) -> String {
    name.trim().to_lowercase()
}

/// The row whose name collides with `candidate`, ignoring case and surrounding space,
/// excluding `exclude` (a template being renamed never collides with itself).
///
/// A free function so the real and mock `imp` share one definition — the tag model records
/// what two verbatim copies of this cost.
pub fn colliding_name(
    rows: &[TemplateRow],
    candidate: &str,
    exclude: Option<u64>,
) -> Option<String> {
    let key = name_key(candidate);
    if key.is_empty() {
        return None;
    }
    rows.iter()
        .find(|r| Some(r.id) != exclude && name_key(&r.name) == key)
        .map(|r| r.name.clone())
}

/// The insert menu's order: starred rows first, each group keeping the writer's own
/// arrangement.
///
/// A stable partition rather than a sort, so two starred templates stay in the order the
/// writer put them in rather than being re-ordered by an unrelated tiebreak.
pub fn starred_first(rows: &[TemplateRow]) -> Vec<TemplateRow> {
    let mut out: Vec<TemplateRow> = rows.iter().filter(|r| r.starred).cloned().collect();
    out.extend(rows.iter().filter(|r| !r.starred).cloned());
    out
}

/// What a bulk import actually did.
///
/// `created` is the number of rows the **backend** made, not the number handed to it: the
/// use case drops a blank name, so the two differ and reporting the request count would
/// tell the writer a file imported that did not.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ImportOutcome {
    pub created: usize,
    /// Names that collided and were given a numeric suffix.
    pub renamed: Vec<String>,
}

/// The index `from` moves to when nudged by `delta`, or `None` if that would fall off
/// either end (so the caller can disable the button rather than silently no-op).
pub fn moved_index(len: usize, from: usize, delta: isize) -> Option<usize> {
    let to = from as isize + delta;
    if from >= len || to < 0 || to as usize >= len {
        return None;
    }
    Some(to as usize)
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{
        note_template_commands, note_template_management_commands, work_commands,
    };
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::event::{
        DirectAccessEntity, EntityEvent, Event, NoteTemplateManagementEvent, Origin,
        WorkManagementEvent,
    };
    use frontend::direct_access::{CreateNoteTemplateDto, UpdateNoteTemplateDto};
    use frontend::note_template_management::ImportNoteTemplatesDto;

    use crate::app_ids::AppIds;

    use super::TemplateRow;

    struct Inner {
        model: ListModel<TemplateRow>,
        version: Signal<u64>,
        ctx: Rc<AppContext>,
        /// The open Work's own ids — scopes every read to *this* Work's `note_templates`
        /// relationship; a second simultaneously-open Work's templates must never merge
        /// into this one's list.
        ids: AppIds,
    }

    #[derive(Clone)]
    pub struct WorkNoteTemplatesListModel {
        inner: Rc<Inner>,
    }

    impl WorkNoteTemplatesListModel {
        pub fn new(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            let rows = load_rows(&ctx, ids.work_id.get());
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(rows),
                    version: Signal::new(0),
                    ctx,
                    ids,
                }),
            }
        }

        /// A catalogue with **nothing in it**, for tests that need the empty case.
        ///
        /// Not the same as `new` on a fresh context: the mock variant deliberately
        /// fabricates a few presets so the UI has something to render, so a test that
        /// built one and called it empty passed against the real backend and failed
        /// under `--features mocks`. This one is empty in both builds by construction.
        #[cfg_attr(not(test), allow(dead_code))]
        pub fn empty(ctx: Rc<AppContext>, ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(Vec::new()),
                    version: Signal::new(0),
                    ctx,
                    ids,
                }),
            }
        }

        /// Subscribe so the list stays live: any `NoteTemplate` mutation — the settings
        /// pane, Save as template…, a preset, an undo — re-reads it, and a project switch
        /// replaces it wholesale.
        ///
        /// **Re-subscribe on every call**, for the reason `work_tags_list_model::wire`
        /// spells out: `BuildContext::subscribe_event` is scoped to the current build, so a
        /// one-shot guard leaves the model deaf after the first `App` rebuild.
        pub fn wire(&self, ctx: &mut BuildContext) {
            for ev in [
                EntityEvent::Created,
                EntityEvent::Updated,
                EntityEvent::Removed,
            ] {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::NoteTemplate(ev)),
                    move |_event: &Event| me.refresh(),
                );
            }
            // `import_note_templates` creates through its own unit of work, so it publishes
            // one feature event rather than N per-entity `Created` ones.
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::NoteTemplateManagement(
                        NoteTemplateManagementEvent::ImportNoteTemplates,
                    ),
                    move |_event: &Event| me.refresh(),
                );
            }
            // The Work's relationship list itself changes on reorder, which is a `Work`
            // update rather than a `NoteTemplate` one.
            {
                let me = self.clone();
                ctx.subscribe_event(
                    Origin::DirectAccess(DirectAccessEntity::Work(EntityEvent::Updated)),
                    move |_event: &Event| me.refresh(),
                );
            }
            for wev in [WorkManagementEvent::LoadWork, WorkManagementEvent::NewWork] {
                let me = self.clone();
                ctx.subscribe_event(Origin::WorkManagement(wev), move |event: &Event| {
                    if !me.inner.ids.is_bootstrap_or_own(&event.ids) {
                        return;
                    }
                    // Prefer the already-seeded work_id, falling back to the event's own —
                    // this model may wire before the lifecycle seed lands.
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
        pub fn list_model(&self) -> ListModel<TemplateRow> {
            self.inner.model.clone()
        }

        /// Bumped on each refresh; for consumers that observe rather than bind the model.
        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        /// Current rows, in the writer's own order.
        pub fn rows(&self) -> Vec<TemplateRow> {
            snapshot(&self.inner.model)
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn colliding_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
            super::colliding_name(&self.rows(), candidate, exclude)
        }

        /// Create one template at the end of the list. `None` when no project is open or
        /// the write failed. The backend's `Created` event drives the refresh.
        pub fn create(
            &self,
            name: &str,
            body: &str,
            starred: bool,
            owner_id: Option<u64>,
            stack_id: Option<u64>,
        ) -> Option<u64> {
            let owner = owner_id?;
            let now = chrono::Utc::now();
            let dto = CreateNoteTemplateDto {
                // Left nil on purpose: `note_template_controller::with_identity` mints one
                // at the creation boundary, which is the single place that decides what a
                // durable identity is. Minting here too would be a second answer to the
                // same question.
                uid: uuid::Uuid::nil(),
                created_at: now,
                updated_at: now,
                name: name.trim().to_string(),
                body: body.to_string(),
                starred,
            };
            // `-1` appends, matching the tag pane's own create.
            match note_template_commands::create_note_template(
                &self.inner.ctx,
                stack_id,
                &dto,
                owner,
                -1,
            ) {
                Ok(created) => Some(created.id),
                Err(e) => {
                    eprintln!("note templates: create failed: {e}");
                    None
                }
            }
        }

        /// Patch one template. Every scalar is sent, so callers pass the row's current
        /// values for whatever they are not changing.
        ///
        /// Read-modify-write off the live entity rather than off a `TemplateRow`: the row
        /// carries no `created_at`, and inventing one here would reset the template's
        /// creation time on every rename — the silent-wrong-timestamp bug the tag model
        /// documents.
        pub fn update(
            &self,
            id: u64,
            name: &str,
            body: &str,
            starred: bool,
            stack_id: Option<u64>,
        ) {
            let existing = match note_template_commands::get_note_template(&self.inner.ctx, &id) {
                Ok(Some(t)) => t,
                Ok(None) => return,
                Err(e) => {
                    eprintln!("note templates: update failed to read {id}: {e}");
                    return;
                }
            };
            let dto = UpdateNoteTemplateDto {
                id,
                // Carried from the live entity, for the same reason `created_at` is: an
                // update writes every scalar, so a nil here would blank the identity that
                // names this template's file on disk. `with_identity` deliberately does
                // not run on updates, so nothing downstream would put it back.
                uid: existing.uid,
                created_at: existing.created_at,
                updated_at: chrono::Utc::now(),
                name: name.trim().to_string(),
                body: body.to_string(),
                starred,
            };
            if let Err(e) =
                note_template_commands::update_note_template(&self.inner.ctx, stack_id, &dto)
            {
                eprintln!("note templates: update failed: {e}");
            }
        }

        /// Remove the given ids in one undoable step. Missing ids are a safe no-op.
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
            if let Err(e) = note_template_commands::remove_note_template_multi(
                &self.inner.ctx,
                stack_id,
                &present,
            ) {
                eprintln!("note templates: remove failed: {e}");
            }
        }

        /// Nudge one row by `delta` places, rewriting the Work's ordered relationship.
        ///
        /// The whole list is written back rather than a targeted move so the operation is
        /// one undo step and cannot leave the relationship half-permuted.
        pub fn move_by(&self, id: u64, delta: isize, stack_id: Option<u64>) {
            let Some(work_id) = self.inner.ids.work_id.get() else {
                return;
            };
            let mut ids: Vec<u64> = self.rows().into_iter().map(|r| r.id).collect();
            let Some(from) = ids.iter().position(|x| *x == id) else {
                return;
            };
            let Some(to) = super::moved_index(ids.len(), from, delta) else {
                return; // already at the end it is being nudged towards
            };
            let moved = ids.remove(from);
            ids.insert(to, moved);
            if let Err(e) = work_commands::set_work_relationship(
                &self.inner.ctx,
                stack_id,
                &frontend::direct_access::WorkRelationshipDto {
                    id: work_id,
                    field: WorkRelationshipField::NoteTemplates,
                    right_ids: ids,
                },
            ) {
                eprintln!("note templates: reorder failed: {e}");
            }
        }

        /// Bulk-create as ONE undoable step (a file import, or applying a preset).
        ///
        /// Reports what the backend actually created — not what it was asked to — because
        /// the use case drops a blank name, and a summary built from the request count
        /// would claim a file imported that did not.
        pub fn import(
            &self,
            rows: &[TemplateRow],
            work_id: u64,
            stack_id: Option<u64>,
        ) -> super::ImportOutcome {
            if rows.is_empty() {
                return super::ImportOutcome::default();
            }
            let dto = ImportNoteTemplatesDto {
                work_id,
                names: rows.iter().map(|r| r.name.clone()).collect(),
                bodies: rows.iter().map(|r| r.body.clone()).collect(),
                starreds: rows.iter().map(|r| r.starred).collect(),
            };
            match note_template_management_commands::import_note_templates(
                &self.inner.ctx,
                stack_id,
                &dto,
            ) {
                Ok(res) => super::ImportOutcome {
                    created: res.created_ids.len(),
                    renamed: res.renamed_to,
                },
                Err(e) => {
                    eprintln!("note templates: import failed: {e}");
                    super::ImportOutcome::default()
                }
            }
        }

        /// Re-read for the currently seeded Work (`None` → empty).
        pub fn refresh(&self) {
            self.refresh_for(self.inner.ids.work_id.get());
        }

        fn refresh_for(&self, work_id: Option<u64>) {
            let rows = load_rows(&self.inner.ctx, work_id);
            self.inner.model.reconcile_by_key(rows, |r| r.id);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }

    /// Read one Work's `NoteTemplate`s **through `Work.note_templates`** — not
    /// `get_all_note_template`, which returns every template in the whole shared store and
    /// would leak one Work's list into another's pane whenever two are open.
    ///
    /// The relationship's own order is preserved; nothing is sorted here.
    fn load_rows(ctx: &AppContext, work_id: Option<u64>) -> Vec<TemplateRow> {
        let Some(work_id) = work_id else {
            return Vec::new(); // no project open
        };
        let ids = work_commands::get_work_relationship(
            ctx,
            &work_id,
            &WorkRelationshipField::NoteTemplates,
        )
        .unwrap_or_default();
        note_template_commands::get_note_template_multi(ctx, &ids)
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .map(|t| TemplateRow {
                id: t.id,
                name: t.name,
                body: t.body,
                starred: t.starred,
            })
            .collect()
    }

    fn snapshot(model: &ListModel<TemplateRow>) -> Vec<TemplateRow> {
        (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect()
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use teksilo::data::ListModel;
    use teksilo::prelude::*;

    use frontend::AppContext;

    use crate::app_ids::AppIds;

    use super::{TemplateRow, name_key};

    struct Inner {
        model: ListModel<TemplateRow>,
        version: Signal<u64>,
        next_id: Cell<u64>,
    }

    #[derive(Clone)]
    pub struct WorkNoteTemplatesListModel {
        inner: Rc<Inner>,
    }

    fn row(id: u64, name: &str, body: &str, starred: bool) -> TemplateRow {
        TemplateRow {
            id,
            name: name.to_string(),
            body: body.to_string(),
            starred,
        }
    }

    impl WorkNoteTemplatesListModel {
        pub fn new(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            // Shaped like a project that applied a couple of presets: one starred, one not,
            // in a hand-arranged (non-alphabetical) order so the mock exercises the same
            // ordering rules the real model has.
            let rows = vec![
                row(
                    1,
                    "Character sheet",
                    "# Character sheet\n\n## Identity\n\n- Full name:\n- Age:\n",
                    true,
                ),
                row(
                    2,
                    "Location",
                    "# Location\n\n## First impression\n\n- What you notice first:\n",
                    false,
                ),
                row(
                    3,
                    "Beat sheet",
                    "# Beat sheet\n\n- Goal:\n- Conflict:\n- Turn:\n",
                    false,
                ),
            ];
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(rows),
                    version: Signal::new(0),
                    next_id: Cell::new(4),
                }),
            }
        }

        /// A catalogue with **nothing in it**, for tests that need the empty case.
        ///
        /// Not the same as `new` on a fresh context: the mock variant deliberately
        /// fabricates a few presets so the UI has something to render, so a test that
        /// built one and called it empty passed against the real backend and failed
        /// under `--features mocks`. This one is empty in both builds by construction.
        #[cfg_attr(not(test), allow(dead_code))]
        pub fn empty(_ctx: Rc<AppContext>, _ids: AppIds) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::from_vec(Vec::new()),
                    version: Signal::new(0),
                    next_id: Cell::new(1),
                }),
            }
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn refresh(&self) {
            // Mock list is in-memory and always "loaded".
        }

        pub fn list_model(&self) -> ListModel<TemplateRow> {
            self.inner.model.clone()
        }

        pub fn version_signal(&self) -> Signal<u64> {
            self.inner.version.clone()
        }

        pub fn rows(&self) -> Vec<TemplateRow> {
            (0..self.inner.model.len())
                .filter_map(|i| self.inner.model.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn len(&self) -> usize {
            self.inner.model.len()
        }

        pub fn colliding_name(&self, candidate: &str, exclude: Option<u64>) -> Option<String> {
            super::colliding_name(&self.rows(), candidate, exclude)
        }

        pub fn create(
            &self,
            name: &str,
            body: &str,
            starred: bool,
            _owner_id: Option<u64>,
            _stack_id: Option<u64>,
        ) -> Option<u64> {
            let id = self.inner.next_id.get();
            self.inner.next_id.set(id + 1);
            let mut rows = self.rows();
            rows.push(row(id, name.trim(), body, starred));
            self.replace(rows);
            Some(id)
        }

        pub fn update(
            &self,
            id: u64,
            name: &str,
            body: &str,
            starred: bool,
            _stack_id: Option<u64>,
        ) {
            let mut rows = self.rows();
            if let Some(r) = rows.iter_mut().find(|r| r.id == id) {
                r.name = name.trim().to_string();
                r.body = body.to_string();
                r.starred = starred;
            }
            self.replace(rows);
        }

        pub fn remove_all(&self, ids: &[u64], _stack_id: Option<u64>) {
            let keep: Vec<TemplateRow> = self
                .rows()
                .into_iter()
                .filter(|r| !ids.contains(&r.id))
                .collect();
            self.replace(keep);
        }

        pub fn move_by(&self, id: u64, delta: isize, _stack_id: Option<u64>) {
            let mut rows = self.rows();
            let Some(from) = rows.iter().position(|r| r.id == id) else {
                return;
            };
            let Some(to) = super::moved_index(rows.len(), from, delta) else {
                return;
            };
            let moved = rows.remove(from);
            rows.insert(to, moved);
            self.replace(rows);
        }

        /// Mirrors the real path's suffix-don't-skip rule, so the mock build shows the same
        /// summary text the real one does.
        pub fn import(
            &self,
            rows: &[TemplateRow],
            _work_id: u64,
            _stack_id: Option<u64>,
        ) -> super::ImportOutcome {
            let mut current = self.rows();
            let mut renamed = Vec::new();
            let mut created = 0usize;
            for r in rows {
                let raw = r.name.trim();
                if raw.is_empty() {
                    continue;
                }
                let name = if current.iter().any(|e| name_key(&e.name) == name_key(raw)) {
                    let mut n = 2usize;
                    loop {
                        let candidate = format!("{raw} ({n})");
                        if !current
                            .iter()
                            .any(|e| name_key(&e.name) == name_key(&candidate))
                        {
                            renamed.push(candidate.clone());
                            break candidate;
                        }
                        n += 1;
                    }
                } else {
                    raw.to_string()
                };
                let id = self.inner.next_id.get();
                self.inner.next_id.set(id + 1);
                current.push(row(id, &name, &r.body, r.starred));
                created += 1;
            }
            self.replace(current);
            super::ImportOutcome { created, renamed }
        }

        /// No sort — the writer's order is the order, as in the real model.
        fn replace(&self, rows: Vec<TemplateRow>) {
            self.inner.model.replace_all(rows);
            let v = &self.inner.version;
            v.set(v.get().wrapping_add(1));
        }
    }
}

pub use imp::WorkNoteTemplatesListModel;

#[cfg(test)]
mod tests {
    use super::*;

    fn r(id: u64, name: &str, starred: bool) -> TemplateRow {
        TemplateRow {
            id,
            name: name.to_string(),
            starred,
            ..Default::default()
        }
    }

    #[test]
    fn name_key_ignores_case_and_surrounding_space() {
        assert_eq!(name_key("  CHARACTER SHEET  "), "character sheet");
        assert_eq!(name_key("Location"), name_key("location"));
        assert_eq!(name_key("   "), "");
    }

    fn list() -> Vec<TemplateRow> {
        vec![
            r(1, "Character sheet", true),
            r(2, "Location", false),
            r(3, "Beat sheet", false),
        ]
    }

    #[test]
    fn an_exact_name_collides() {
        assert_eq!(
            colliding_name(&list(), "Location", None).as_deref(),
            Some("Location")
        );
    }

    #[test]
    fn a_differently_cased_name_collides_and_reports_the_existing_spelling() {
        assert_eq!(
            colliding_name(&list(), "  lOcAtIoN ", None).as_deref(),
            Some("Location"),
            "the message names the template as it is actually spelled, not as it was typed"
        );
    }

    /// Renaming must not warn that a row collides with itself — that would fire on every
    /// keystroke of a rename that did not change the name.
    #[test]
    fn a_template_never_collides_with_itself() {
        assert_eq!(colliding_name(&list(), "Location", Some(2)), None);
    }

    #[test]
    fn a_blank_candidate_never_collides() {
        assert_eq!(colliding_name(&list(), "", None), None);
        assert_eq!(colliding_name(&list(), "   ", None), None);
    }

    /// The insert menu floats starred rows, and **within each group keeps the writer's own
    /// arrangement** — a sort would have re-ordered "Location" and "Beat sheet".
    #[test]
    fn starred_rows_float_but_the_writers_order_is_kept_within_each_group() {
        let names: Vec<String> = starred_first(&list()).into_iter().map(|r| r.name).collect();
        assert_eq!(names, vec!["Character sheet", "Location", "Beat sheet"]);
    }

    #[test]
    fn starred_first_keeps_several_starred_rows_in_order() {
        let rows = vec![
            r(1, "A", false),
            r(2, "B", true),
            r(3, "C", false),
            r(4, "D", true),
        ];
        let names: Vec<String> = starred_first(&rows).into_iter().map(|r| r.name).collect();
        assert_eq!(names, vec!["B", "D", "A", "C"]);
    }

    #[test]
    fn moving_within_bounds_reports_the_target_index() {
        assert_eq!(moved_index(3, 0, 1), Some(1));
        assert_eq!(moved_index(3, 2, -1), Some(1));
    }

    /// Off either end is `None`, so the pane disables the button instead of offering a
    /// silent no-op.
    #[test]
    fn moving_off_either_end_is_rejected() {
        assert_eq!(moved_index(3, 0, -1), None);
        assert_eq!(moved_index(3, 2, 1), None);
        assert_eq!(moved_index(0, 0, 1), None);
    }
}
