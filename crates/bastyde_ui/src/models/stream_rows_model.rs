//! Reactive, ordered list of the rows belonging to one **container** — the data
//! behind the Full Chapter / Full Part / Full Book streams and their Full Synopsis
//! twins.
//!
//! Per the writing model, a container's extent is a run of the flat, ordered,
//! *activated* item stream: everything after the container head, up to the first
//! item that closes it. What closes it depends on the container's **level** — a
//! chapter ends at the next chapter/part/book boundary, a part ends only at the
//! next part/book (chapters live *inside* it), a book ends only at the next book or
//! at a `BookEnd`. Inside that extent, a **row** is any item that carries scene
//! prose or opens a chapter or a part; notes, plain folders, `Item/Text` and
//! `Item/BookEnd` are not rows. All the predicates come from
//! [`skribisto_model::SubRoleExt`] (single source of truth); folder nesting /
//! `indent` is deliberately ignored — containment is UI-only.
//!
//! The container **head is never a row**: it is the pane header, and its own
//! content (a chapter folder's prose, any container's synopsis) is rendered as the
//! pane's own section. Only folder containers host a stream ([`StreamLevel::for_container`]),
//! so the head is always a `Folder` — which matters, because a chapter folder now
//! `carries_scene()` just like the flat `Item/ChapterScene` it promotes to.
//!
//! Owns a [`bastyde::data::ListModel<StreamRow>`] for the `Repeater`, updated by a
//! **keyed diff** (remove / insert / move / in-place update) so the reconciling
//! `Repeater` reuses each row's mounted editor across structural edits. Two
//! `#[cfg]`-gated `mod imp` variants share one public surface (see
//! [`crate::models`]).

use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
use skribisto_model::SubRoleExt;

/// One row in a stream. Carries the full `(role, sub_role)` — the view needs it to
/// pick the row's chrome (part heading / chapter heading / scene header) and to ask
/// the constraint matrix which editors the row gets; the view-model needs it to gate
/// merge and split.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StreamRow {
    pub item_id: u64,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
}

/// Which kind of container a stream is showing — it selects the boundary rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamLevel {
    Chapter,
    Part,
    Book,
}

impl StreamLevel {
    /// The level for a container head, or `None` if this `(role, sub_role)` has no
    /// stream. **The single place that decision is made** — `ContentTab::new` and
    /// `StreamViewModel::new` both gate on this rather than each matching on
    /// `(role, sub_role)` themselves, so there is no second partial function to keep
    /// in sync by hand.
    ///
    /// Only *folder* containers get a stream. A flat `Item/ChapterScene` keeps its
    /// dual-pane writing editor — that one flowing page is the point of it — and its
    /// scenes still stream inside the enclosing Full Part / Full Book, where it
    /// appears as a chapter-heading row carrying its own prose.
    pub fn for_container(role: &BinderItemRole, sub_role: &BinderItemSubRole) -> Option<Self> {
        if *role != BinderItemRole::Folder {
            return None;
        }
        match sub_role {
            BinderItemSubRole::ChapterScene => Some(Self::Chapter),
            BinderItemSubRole::Part => Some(Self::Part),
            BinderItemSubRole::Book => Some(Self::Book),
            _ => None,
        }
    }
}

/// Does `sr` close a container at `level`?
// The mock row model fabricates its rows, so under `--features mocks` these pure
// functions are exercised only by the tests below.
#[cfg_attr(feature = "mocks", allow(dead_code))]
fn is_boundary(level: StreamLevel, sr: &BinderItemSubRole) -> bool {
    match level {
        // A chapter ends at the next chapter, part or book.
        StreamLevel::Chapter => {
            sr.opens_chapter() || sr.opens_part() || sr.opens_book() || sr.closes_book()
        }
        // A part ends at the next part or book — chapters live *inside* it.
        StreamLevel::Part => sr.opens_part() || sr.opens_book() || sr.closes_book(),
        // A book ends only at the next book, or at its explicit end marker.
        StreamLevel::Book => sr.opens_book() || sr.closes_book(),
    }
}

/// Is `sr` worth a row? Scene-bearing items are the prose; chapter and part heads
/// are the structure headings that make a Full Book read as a manuscript.
#[cfg_attr(feature = "mocks", allow(dead_code))]
fn is_row(sr: &BinderItemSubRole) -> bool {
    sr.carries_scene() || sr.opens_chapter() || sr.opens_part()
}

/// Indices (into `sub_roles`) of the rows belonging to the container head at `head`:
/// every row-worthy item forward from `head + 1` until `level`'s boundary (or the end
/// of the stream).
///
/// The head itself is never a row — it is the pane header, and its own prose/synopsis
/// is the pane's own section. (This matters now that a chapter folder `carries_scene()`
/// like any scene: without the `skip`, a `Folder/ChapterScene` head would list itself.)
/// `indent` / folder nesting is deliberately not consulted.
#[cfg_attr(feature = "mocks", allow(dead_code))]
pub fn row_indices(sub_roles: &[BinderItemSubRole], head: usize, level: StreamLevel) -> Vec<usize> {
    let mut out = Vec::new();
    for (i, sr) in sub_roles.iter().enumerate().skip(head + 1) {
        if is_boundary(level, sr) {
            break;
        }
        if is_row(sr) {
            out.push(i);
        }
    }
    out
}

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::RefCell;
    use std::collections::HashSet;
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

    use super::{StreamLevel, StreamRow, row_indices};

    /// Called with the ids of rows that left the stream, so the owner can release
    /// their shared documents. Stored (not one-shot) — it fires from every refresh.
    type OnRemoved = Box<dyn Fn(&[u64])>;

    struct Inner {
        model: ListModel<StreamRow>,
        ctx: Rc<AppContext>,
        work_id: Signal<Option<u64>>,
        head_id: u64,
        level: StreamLevel,
        on_removed: RefCell<Option<OnRemoved>>,
    }

    #[derive(Clone)]
    pub struct StreamRowsModel {
        inner: Rc<Inner>,
    }

    impl StreamRowsModel {
        /// `level` is resolved once by the caller (via [`StreamLevel::for_container`]),
        /// so this model stays a total function over a container its caller already
        /// validated.
        pub fn new(
            ctx: Rc<AppContext>,
            work_id: Signal<Option<u64>>,
            head_id: u64,
            level: StreamLevel,
        ) -> Self {
            Self {
                inner: Rc::new(Inner {
                    model: ListModel::new(),
                    ctx,
                    work_id,
                    head_id,
                    level,
                    on_removed: RefCell::new(None),
                }),
            }
        }

        /// The `Repeater`'s data source.
        pub fn list(&self) -> ListModel<StreamRow> {
            self.inner.model.clone()
        }

        /// Current ordered rows — for the view-model's synchronous prev/next gating
        /// (merge/split), which needs `(role, sub_role)` and not just ids.
        pub fn rows(&self) -> Vec<StreamRow> {
            let m = &self.inner.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.clone()))
                .collect()
        }

        /// Current ordered row ids.
        pub fn ids(&self) -> Vec<u64> {
            self.rows().into_iter().map(|r| r.item_id).collect()
        }

        /// Subscribe once to structural events and do an initial fill. `on_removed`
        /// is invoked (from this and every later refresh) with the ids that left the
        /// stream, so the owner can release their shared documents.
        ///
        /// **`on_removed` must not capture its owner strongly.** It is stored for the
        /// model's lifetime, and the model is itself owned by that owner — an `Rc`
        /// capture would close a cycle, the owner's `Drop` would never run, and every
        /// document the stream ever opened would leak. `StreamViewModel::wire` passes
        /// a `Weak`-capturing closure.
        ///
        /// Rename (`BinderItem::Updated`) is intentionally NOT watched — titles are
        /// reactive on the per-row single, so a rename must not disturb the list.
        /// `Promote` *is* watched: it rewrites a row's `(role, sub_role)` in place,
        /// which `reconcile` turns into an in-place `set`.
        pub fn wire(&self, ctx: &mut BuildContext, on_removed: impl Fn(&[u64]) + 'static) {
            *self.inner.on_removed.borrow_mut() = Some(Box::new(on_removed));
            // Subscribe on **every** build. `BuildContext::subscribe_event` scopes a
            // subscription to the widget's current build and drops it on the next one, so
            // a "subscribe once" guard would make this model go deaf the first time its
            // host widget rebuilt. Re-subscribing cannot duplicate: the old callbacks are
            // already gone.
            {
                use DirectAccessEntity::BinderItem;
                use EntityEvent::{Created, Removed};
                let origins = [
                    Origin::DirectAccess(BinderItem(Created)),
                    Origin::DirectAccess(BinderItem(Removed)),
                    Origin::BinderItemManagement(BinderItemManagementEvent::Duplicate),
                    Origin::BinderItemManagement(BinderItemManagementEvent::MoveItems),
                    Origin::BinderItemManagement(BinderItemManagementEvent::MergeTwoScenes),
                    Origin::BinderItemManagement(BinderItemManagementEvent::SplitScene),
                    Origin::BinderItemManagement(BinderItemManagementEvent::Promote),
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
                self.inner.head_id,
                self.inner.level,
            );
            let removed = reconcile(&self.inner.model, next);
            if !removed.is_empty() {
                // Take the callback out of the RefCell before invoking it: it calls
                // back into the owner, which may re-enter this model.
                let cb = self.inner.on_removed.borrow_mut().take();
                if let Some(cb) = cb {
                    cb(&removed);
                    *self.inner.on_removed.borrow_mut() = Some(cb);
                }
            }
        }
    }

    /// The container's ordered rows, per the extent + row rules.
    fn query(
        ctx: &AppContext,
        work_id: Option<u64>,
        head_id: u64,
        level: StreamLevel,
    ) -> Vec<StreamRow> {
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
            if let Some(pos) = flat.iter().position(|it| it.id == head_id) {
                let sub_roles: Vec<BinderItemSubRole> =
                    flat.iter().map(|it| it.sub_role.clone()).collect();
                return row_indices(&sub_roles, pos, level)
                    .into_iter()
                    .map(|i| StreamRow {
                        item_id: flat[i].id,
                        role: flat[i].role.clone(),
                        sub_role: flat[i].sub_role.clone(),
                    })
                    .collect();
            }
        }
        Vec::new()
    }

    /// Apply `next` onto `model` with the fewest granular ops (remove / insert /
    /// move / in-place set), so the reconciling `Repeater` keeps surviving rows'
    /// editors. Returns the ids that left the stream.
    fn reconcile(model: &ListModel<StreamRow>, next: Vec<StreamRow>) -> Vec<u64> {
        let mut cur: Vec<StreamRow> = (0..model.len())
            .filter_map(|i| model.with_item(i, |r| r.clone()))
            .collect();

        // 1. Drop rows no longer present (back-to-front to keep indices stable).
        let keep: HashSet<u64> = next.iter().map(|r| r.item_id).collect();
        let mut removed = Vec::new();
        let mut i = cur.len();
        while i > 0 {
            i -= 1;
            if !keep.contains(&cur[i].item_id) {
                removed.push(cur[i].item_id);
                model.remove(i);
                cur.remove(i);
            }
        }

        // 2. Align the survivors to `next`'s order, insert new ids, and update in
        //    place any row whose *type* changed under it — a Promote rewrites
        //    `(role, sub_role)` without moving the item, and the row's chrome and
        //    its allowed editors both depend on it.
        for (pos, want) in next.iter().enumerate() {
            match cur.iter().position(|r| r.item_id == want.item_id) {
                Some(j) => {
                    if j != pos {
                        model.move_item(j, pos);
                        let it = cur.remove(j);
                        cur.insert(pos, it);
                    }
                    if cur[pos] != *want {
                        model.set(pos, want.clone());
                        cur[pos] = want.clone();
                    }
                }
                None => {
                    model.insert(pos, want.clone());
                    cur.insert(pos, want.clone());
                }
            }
        }
        removed
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::rc::Rc;

    use bastyde::data::ListModel;
    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};

    use super::{StreamLevel, StreamRow};

    #[derive(Clone)]
    pub struct StreamRowsModel {
        model: ListModel<StreamRow>,
    }

    /// Fabricated rows matching the mock binder fixture in
    /// `singles::SingleBinderItem`: the mock Book (101) holds a Part (301) with two
    /// chapters — a chapter folder (104) holding scenes 201-203, and a flat
    /// `Item/ChapterScene` (302) followed by scene 303.
    fn mock_rows(level: StreamLevel) -> Vec<StreamRow> {
        use BinderItemRole::{Folder, Item};
        use BinderItemSubRole::{ChapterScene, Part, Scene};
        let row = |item_id, role, sub_role| StreamRow {
            item_id,
            role,
            sub_role,
        };
        match level {
            // A chapter streams only its own scenes.
            StreamLevel::Chapter => vec![
                row(201, Item, Scene),
                row(202, Item, Scene),
                row(203, Item, Scene),
            ],
            // A part streams its chapters' heads *and* their scenes.
            StreamLevel::Part => vec![
                row(104, Folder, ChapterScene),
                row(201, Item, Scene),
                row(202, Item, Scene),
                row(203, Item, Scene),
                row(302, Item, ChapterScene),
                row(303, Item, Scene),
            ],
            // A book adds the part heads on top.
            StreamLevel::Book => vec![
                row(301, Folder, Part),
                row(104, Folder, ChapterScene),
                row(201, Item, Scene),
                row(202, Item, Scene),
                row(203, Item, Scene),
                row(302, Item, ChapterScene),
                row(303, Item, Scene),
            ],
        }
    }

    impl StreamRowsModel {
        pub fn new(
            _ctx: Rc<AppContext>,
            _work_id: Signal<Option<u64>>,
            _head_id: u64,
            level: StreamLevel,
        ) -> Self {
            Self {
                model: ListModel::from_vec(mock_rows(level)),
            }
        }

        pub fn list(&self) -> ListModel<StreamRow> {
            self.model.clone()
        }

        pub fn rows(&self) -> Vec<StreamRow> {
            let m = &self.model;
            (0..m.len())
                .filter_map(|i| m.with_item(i, |r| r.clone()))
                .collect()
        }

        pub fn ids(&self) -> Vec<u64> {
            self.rows().into_iter().map(|r| r.item_id).collect()
        }

        pub fn wire(&self, _ctx: &mut BuildContext, _on_removed: impl Fn(&[u64]) + 'static) {}
    }
}

pub use imp::StreamRowsModel;

#[cfg(test)]
mod tests {
    use super::{StreamLevel, row_indices};
    use frontend::common::entities::BinderItemRole;
    use frontend::common::entities::BinderItemSubRole::*;

    /// The original Full Chapter rule. The head is the pane header, never a row —
    /// which is now load-bearing: a chapter folder `carries_scene()` like any scene,
    /// so without the head-skip a `Folder/ChapterScene` head would list itself.
    #[test]
    fn boundary_rule() {
        let chapter = |s: &[_], h| row_indices(s, h, StreamLevel::Chapter);
        // A Note between scenes is skipped (not scene-bearing, not a boundary); the
        // run stops at the next chapter.
        assert_eq!(
            chapter(&[ChapterScene, Scene, Note, Scene, ChapterScene, Scene], 0),
            vec![1, 3]
        );
        // Text carries no content — neither scene nor boundary.
        assert_eq!(chapter(&[ChapterScene, Text, Scene], 0), vec![2]);
        // BookBegin / BookEnd are boundaries.
        assert_eq!(chapter(&[ChapterScene, Scene, BookEnd, Scene], 0), vec![1]);
        // Runs to the end of the stream when no boundary follows.
        assert_eq!(chapter(&[ChapterScene, Scene, Scene], 0), vec![1, 2]);
        // The head is never a row, even though it carries scene prose.
        assert_eq!(
            chapter(&[ChapterScene, Part, Scene], 0),
            Vec::<usize>::new()
        );
    }

    /// A part does not end at a chapter — it contains them, heads and scenes alike.
    #[test]
    fn part_stream_contains_its_chapters_heads_and_their_scenes() {
        assert_eq!(
            row_indices(
                &[
                    Part,
                    ChapterScene,
                    Scene,
                    Scene,
                    ChapterScene,
                    Scene,
                    Part,
                    Scene
                ],
                0,
                StreamLevel::Part
            ),
            vec![1, 2, 3, 4, 5],
            "stops at the next Part, keeps both chapter heads"
        );
    }

    /// A book contains parts, chapters and scenes.
    #[test]
    fn book_stream_contains_parts_chapters_and_scenes() {
        assert_eq!(
            row_indices(
                &[
                    Book,
                    Part,
                    ChapterScene,
                    Scene,
                    Scene,
                    Part,
                    ChapterScene,
                    Scene
                ],
                0,
                StreamLevel::Book
            ),
            vec![1, 2, 3, 4, 5, 6, 7],
            "a part no longer ends a book stream"
        );
    }

    #[test]
    fn book_end_terminates_a_book_stream() {
        assert_eq!(
            row_indices(
                &[Book, ChapterScene, Scene, BookEnd, Scene],
                0,
                StreamLevel::Book
            ),
            vec![1, 2]
        );
    }

    /// Both encodings of "a new book starts here" close the previous one.
    #[test]
    fn a_second_book_terminates_the_first() {
        assert_eq!(
            row_indices(
                &[Book, ChapterScene, Scene, Book, Scene],
                0,
                StreamLevel::Book
            ),
            vec![1, 2]
        );
        assert_eq!(
            row_indices(
                &[Book, ChapterScene, Scene, BookBegin, Scene],
                0,
                StreamLevel::Book
            ),
            vec![1, 2]
        );
    }

    /// Notes and plain folders are organisational, not manuscript — skipped, but they
    /// do not close the run.
    #[test]
    fn notes_and_folders_are_skipped_not_boundaries() {
        assert_eq!(
            row_indices(
                &[Part, Scene, Note, None, Scene, Part],
                0,
                StreamLevel::Part
            ),
            vec![1, 4]
        );
    }

    #[test]
    fn degenerate_extents() {
        assert_eq!(
            row_indices(&[Part, Scene, Scene], 0, StreamLevel::Part),
            vec![1, 2]
        );
        assert_eq!(
            row_indices(&[Part, Part, Scene], 0, StreamLevel::Part),
            Vec::<usize>::new(),
            "an empty part yields no rows"
        );
        assert_eq!(
            row_indices(&[ChapterScene], 0, StreamLevel::Chapter),
            Vec::<usize>::new()
        );
    }

    /// Only the folder containers host a stream — the flat chapter/book markers keep
    /// their own tabs.
    #[test]
    fn only_folder_containers_have_a_level() {
        use BinderItemRole::{Folder, Item};
        assert_eq!(
            StreamLevel::for_container(&Folder, &ChapterScene),
            Some(StreamLevel::Chapter)
        );
        assert_eq!(
            StreamLevel::for_container(&Folder, &Part),
            Some(StreamLevel::Part)
        );
        assert_eq!(
            StreamLevel::for_container(&Folder, &Book),
            Some(StreamLevel::Book)
        );
        // The flat chapter is *not* a container: it keeps its dual-pane editor.
        assert_eq!(
            StreamLevel::for_container(&Item, &ChapterScene),
            Option::None
        );
        assert_eq!(StreamLevel::for_container(&Item, &BookBegin), Option::None);
        assert_eq!(StreamLevel::for_container(&Item, &Scene), Option::None);
        assert_eq!(StreamLevel::for_container(&Folder, &Note), Option::None);
        assert_eq!(StreamLevel::for_container(&Folder, &None), Option::None);
    }
}
