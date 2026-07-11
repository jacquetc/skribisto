//! Reactive, ordered list of the scenes belonging to one chapter — the data
//! behind the Chapter folder tab's **Full Chapter** view.
//!
//! Per the writing model, a chapter's scenes are *all* scene-bearing items
//! (`Scene` / `ChapterScene`) in the flat, activated item stream after the
//! chapter, up to (but excluding) the next `Chapter` / `Part` / `Book` boundary
//! — even when nested in folders. The boundary/scene predicates come from
//! [`skribisto_model::SubRoleExt`] (single source of truth); folder nesting /
//! `indent` is deliberately ignored.
//!
//! Owns a [`bastyde::data::ListModel<SceneRow>`] for the `Repeater`, updated by a
//! **keyed diff** (remove / insert / move only what changed) so the reconciling
//! `Repeater` reuses each scene's mounted editor across structural edits. Two
//! `#[cfg]`-gated `mod imp` variants share one public surface (see
//! [`crate::models`]).

/// One scene in the Full Chapter view. Deliberately minimal: the title / label
/// shown in the header bind to the per-scene `SingleScene` signals (reactive),
/// so a rename never rebuilds a row — only structural edits touch this model.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SceneRow {
    pub item_id: u64,
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::data::ListModel;
    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_commands, binder_item_commands, work_commands};
    use frontend::common::direct_access::binder::BinderRelationshipField;
    use frontend::common::direct_access::work::WorkRelationshipField;
    use frontend::common::entities::BinderItemSubRole;
    use frontend::common::event::{
        BinderItemManagementEvent, DirectAccessEntity, EntityEvent, Event, Origin,
        TrashManagementEvent, WorkManagementEvent,
    };
    use skribisto_model::SubRoleExt;
    use std::collections::HashSet;

    use super::SceneRow;

    struct Inner {
        model: ListModel<SceneRow>,
        subscribed: Cell<bool>,
        ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        chapter_id: u64,
    }

    #[derive(Clone)]
    pub struct ChapterScenesModel {
        inner: Rc<Inner>,
    }

    impl ChapterScenesModel {
        pub fn new(ctx: Rc<AppContext>, work_id: Signal<Option<u64>>, chapter_id: u64) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::new(),
                    subscribed: Cell::new(false),
                    ctx,
                    work_id,
                    chapter_id,
                }),
            }
        }

        /// The `Repeater`'s data source.
        pub fn list(&self) -> ListModel<SceneRow> {
            self.inner.model.clone()
        }

        /// Current ordered scene ids (for prev/next-sibling maths in the view-model).
        pub fn ids(&self) -> Vec<u64> {
            let m = &self.inner.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.item_id))
                .collect()
        }

        /// Subscribe once to structural events and do an initial fill. Rename
        /// (`Updated`) is intentionally NOT watched — titles are reactive on the
        /// per-scene single, so a rename must not disturb the list.
        pub fn wire(&self, ctx: &mut BuildContext) {
            if !self.inner.subscribed.replace(true) {
                use DirectAccessEntity::BinderItem;
                use EntityEvent::{Created, Removed};
                let origins = [
                    Origin::DirectAccess(BinderItem(Created)),
                    Origin::DirectAccess(BinderItem(Removed)),
                    Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
                    Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
                    Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
                    Origin::BinderItemManagement(BinderItemManagementEvent::SplitScene),
                    Origin::TrashManagement(TrashManagementEvent::TrashBinderItems),
                    Origin::TrashManagement(TrashManagementEvent::RestoreItems),
                    Origin::TrashManagement(TrashManagementEvent::EmptyTrash),
                    Origin::WorkManagement(WorkManagementEvent::LoadWork),
                    Origin::WorkManagement(WorkManagementEvent::NewWork),
                ];
                for origin in origins {
                    let me = self.clone();
                    ctx.subscribe_event(origin, move |_event: &Event| me.refresh());
                }
            }
            self.refresh();
        }

        fn refresh(&self) {
            let next = query(
                &self.inner.ctx,
                self.inner.work_id.get(),
                self.inner.chapter_id,
            );
            reconcile(&self.inner.model, next);
        }
    }

    /// Ordered scenes of the chapter, per the flat-stream boundary rule.
    fn query(ctx: &AppContext, work_id: Option<u64>, chapter_id: u64) -> Vec<SceneRow> {
        let Some(work_id) = work_id else {
            return Vec::new();
        };
        let binder_ids =
            work_commands::get_work_relationship(ctx, &work_id, &WorkRelationshipField::Binders)
                .unwrap_or_default();
        for binder_id in binder_ids {
            let item_ids = binder_commands::get_binder_relationship(
                ctx,
                &binder_id,
                &BinderRelationshipField::BinderItems,
            )
            .unwrap_or_default();
            let flat: Vec<_> = binder_item_commands::get_binder_item_multi(ctx, &item_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .filter(|it| it.activated)
                .collect();
            if let Some(pos) = flat.iter().position(|it| it.id == chapter_id) {
                let sub_roles: Vec<BinderItemSubRole> =
                    flat.iter().map(|it| it.sub_role.clone()).collect();
                return scene_indices(&sub_roles, pos)
                    .into_iter()
                    .map(|i| SceneRow {
                        item_id: flat[i].id,
                    })
                    .collect();
            }
        }
        Vec::new()
    }

    fn is_boundary(sr: &BinderItemSubRole) -> bool {
        sr.opens_chapter() || sr.opens_part() || sr.opens_book() || sr.closes_book()
    }

    /// Indices (into `sub_roles`) of the scenes belonging to the chapter head at
    /// `chapter_pos`: the head itself if it is scene-bearing (a `ChapterScene`
    /// marker — a `Chapter` folder is not), then every scene-bearing item forward
    /// until the next chapter/part/book boundary (or the end of the stream).
    /// `indent`/folder nesting is deliberately not consulted.
    fn scene_indices(sub_roles: &[BinderItemSubRole], chapter_pos: usize) -> Vec<usize> {
        let mut out = Vec::new();
        if sub_roles
            .get(chapter_pos)
            .map(|s| s.carries_scene())
            .unwrap_or(false)
        {
            out.push(chapter_pos);
        }
        for (i, sr) in sub_roles.iter().enumerate().skip(chapter_pos + 1) {
            if is_boundary(sr) {
                break;
            }
            if sr.carries_scene() {
                out.push(i);
            }
        }
        out
    }

    /// Apply `next` onto `model` with the fewest granular ops (remove / insert /
    /// move), so the reconciling `Repeater` keeps surviving rows' editors.
    fn reconcile(model: &ListModel<SceneRow>, next: Vec<SceneRow>) {
        let mut cur: Vec<SceneRow> = (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect();

        // 1. Drop rows no longer present (back-to-front to keep indices stable).
        let keep: HashSet<u64> = next.iter().map(|r| r.item_id).collect();
        let mut i = cur.len();
        while i > 0 {
            i -= 1;
            if !keep.contains(&cur[i].item_id) {
                model.remove(i);
                cur.remove(i);
            }
        }

        // 2. Align remaining rows to `next`'s order, inserting new ids.
        for (pos, want) in next.iter().enumerate() {
            match cur.iter().position(|r| r.item_id == want.item_id) {
                Some(j) if j == pos => {}
                Some(j) => {
                    model.move_item(j, pos);
                    let it = cur.remove(j);
                    cur.insert(pos, it);
                }
                None => {
                    model.insert(pos, want.clone());
                    cur.insert(pos, want.clone());
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::scene_indices;
        use frontend::common::entities::BinderItemSubRole::*;

        #[test]
        fn boundary_rule() {
            // Chapter folder head is not itself a scene; a Note between scenes is
            // skipped (not scene-bearing, not a boundary); the run stops at the
            // next Chapter.
            assert_eq!(
                scene_indices(&[Chapter, Scene, Note, Scene, Chapter, Scene], 0),
                vec![1, 3]
            );
            // A ChapterScene head IS its own first scene; stops at the next Part.
            assert_eq!(
                scene_indices(&[ChapterScene, Scene, Part, Scene], 0),
                vec![0, 1]
            );
            // Text carries no content — neither scene nor boundary.
            assert_eq!(scene_indices(&[Chapter, Text, Scene], 0), vec![2]);
            // BookBegin / BookEnd are boundaries.
            assert_eq!(scene_indices(&[Chapter, Scene, BookEnd, Scene], 0), vec![1]);
            // Runs to the end of the stream when no boundary follows.
            assert_eq!(scene_indices(&[Chapter, Scene, Scene], 0), vec![1, 2]);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::data::ListModel;
    use bastyde::prelude::*;

    use frontend::AppContext;

    use super::SceneRow;

    #[derive(Clone)]
    pub struct ChapterScenesModel {
        model: ListModel<SceneRow>,
    }

    impl ChapterScenesModel {
        pub fn new(_ctx: Rc<AppContext>, _work_id: Signal<Option<u64>>, _chapter_id: u64) -> Self {
            // Three fabricated scenes so the Full Chapter view is non-trivial.
            Self {
                model: ListModel::from_vec(vec![
                    SceneRow { item_id: 201 },
                    SceneRow { item_id: 202 },
                    SceneRow { item_id: 203 },
                ]),
            }
        }

        pub fn list(&self) -> ListModel<SceneRow> {
            self.model.clone()
        }

        pub fn ids(&self) -> Vec<u64> {
            let m = &self.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.item_id))
                .collect()
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}
    }
}

pub use imp::ChapterScenesModel;
