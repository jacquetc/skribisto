// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleContent` — a reactive **read+write** handle over one `Content` row.
//!
//! Content rows are the only entity the UI edits *directly* (prose in the editor,
//! titles in the heading/folder tabs), so — like [`SingleWork`](crate::singles::SingleWork)
//! and unlike the read-singles [`SingleBinderItem`](crate::singles::SingleBinderItem) /
//! [`SingleBinder`](crate::singles::SingleBinder) — it carries `data`/`dirty` and a
//! `save(stack)` that **creates the row on first save** (a writing item may not have
//! a row for an allowed role yet) and **updates** it thereafter, preserving
//! `created_at`. One handle serves **every `ContentRole`** (SceneText, NoteText,
//! SynopsisText, the four titles) — the editor tabs hold one per field.
//!
//! Two `mod imp` variants share one public surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::{ContentDto, CreateContentDto, UpdateContentDto};

    use crate::singles::LoadingStatus;

    /// The text a handle with no row of its own writes into the row of its role it
    /// found: its own text after the row's, as a merge joins two texts. A blank row
    /// adds nothing, and a row that already holds the same text is not doubled.
    fn joined_onto(stored: &str, own: &str) -> String {
        if stored.trim().is_empty() || stored.trim() == own.trim() {
            own.to_string()
        } else if own.trim().is_empty() {
            stored.to_string()
        } else {
            frontend::common::prose_keep::join_text(stored, own)
        }
    }

    struct Inner {
        /// The owning `BinderItem` — needed to create the row if it doesn't exist.
        item_id: Cell<Option<u64>>,
        role: RefCell<ContentRole>,
        id: Cell<Option<u64>>,
        created_at: Cell<chrono::DateTime<chrono::Utc>>,
        /// The row's durable identity, cached for the same reason `created_at` is:
        /// an update DTO must carry it back verbatim. A nil here would overwrite the
        /// row's uid on every save, orphaning anything that references it.
        uid: Cell<uuid::Uuid>,
        data: Signal<String>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleContent {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleContent {
        /// A handle for the `(item_id, role)` content of an editor field. `existing`
        /// seeds it from the row loaded at tab-open (id + data + `created_at`);
        /// `None` means the row doesn't exist yet and is created on first `save`.
        pub fn for_field(
            ctx: Rc<AppContext>,
            item_id: u64,
            role: ContentRole,
            existing: Option<&ContentDto>,
        ) -> Self {
            let (id, data, created_at, uid) = match existing {
                Some(c) => (Some(c.id), c.data.clone(), c.created_at, c.uid),
                None => (None, String::new(), chrono::Utc::now(), uuid::Uuid::nil()),
            };
            let status = if id.is_some() {
                LoadingStatus::Loaded
            } else {
                LoadingStatus::Unloaded
            };
            Self {
                inner: Rc::new(Inner {
                    item_id: Cell::new(Some(item_id)),
                    role: RefCell::new(role),
                    id: Cell::new(id),
                    created_at: Cell::new(created_at),
                    uid: Cell::new(uid),
                    data: Signal::new(data),
                    loading_status: Signal::new(status),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                    ctx,
                }),
            }
        }

        /// A handle bound to an existing `Content` row by id (read-oriented; `role`
        /// and `item_id` are filled from the loaded row).
        pub fn from_id(ctx: Rc<AppContext>, id: u64) -> Self {
            let s = Self {
                inner: Rc::new(Inner {
                    item_id: Cell::new(None),
                    role: RefCell::new(ContentRole::default()),
                    id: Cell::new(Some(id)),
                    created_at: Cell::new(chrono::Utc::now()),
                    uid: Cell::new(uuid::Uuid::nil()),
                    data: Signal::new(String::new()),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                    ctx,
                }),
            };
            s.refresh();
            s
        }

        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }
        pub fn role(&self) -> ContentRole {
            self.inner.role.borrow().clone()
        }
        pub fn data(&self) -> Signal<String> {
            self.inner.data.clone()
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
        pub fn dirty(&self) -> Signal<bool> {
            self.inner.dirty.clone()
        }

        /// Stage new content (e.g. the editor's `to_djot()` or a title field's
        /// value). Marks dirty only on a genuine change.
        pub fn set_data(&self, v: String) {
            if self.inner.data.get() != v {
                self.inner.dirty.set(true);
                self.inner.data.set(v);
            }
        }

        /// Persist staged content on the undo `stack` — **create** the row if it
        /// has no id yet (remembering the new id + `created_at`), else **update**
        /// it (preserving `created_at`). No-op when clean. Role-aware by
        /// construction: a handle only ever writes its own `ContentRole`.
        ///
        /// One row per role, whatever happened under the handle. A handle with no
        /// row, or whose row a command took away (an undo, a redo joining it onto
        /// another), first looks for the row of its role its item holds now: a
        /// command may have made one while this handle still held unsaved text (a
        /// merge folding the other scene's scratchpad in). It writes into that row,
        /// its text joined after the row's as the merge joins them, rather than
        /// create a second row nothing would ever show, or update a row that is no
        /// longer in the store, which brought it back owned by nothing and lost the
        /// text with the next save.
        pub fn save(&self, stack: Option<u64>) -> anyhow::Result<()> {
            if !self.inner.dirty.get() {
                return Ok(());
            }
            let ctx = &*self.inner.ctx;
            let now = chrono::Utc::now();
            let role = self.inner.role.borrow().clone();
            let mut data = self.inner.data.get();
            let gone = self
                .inner
                .id
                .get()
                .is_some_and(|id| matches!(content_commands::get_content(ctx, &id), Ok(None)));
            if self.inner.id.get().is_none() || gone {
                self.inner.id.set(None);
                if let Some(row) = self.row_of_role(&role) {
                    data = joined_onto(&row.data, &data);
                    self.inner.id.set(Some(row.id));
                    self.inner.created_at.set(row.created_at);
                    self.inner.uid.set(row.uid);
                    self.inner.data.set(data.clone());
                }
            }
            match self.inner.id.get() {
                Some(id) => {
                    content_commands::update_content(
                        ctx,
                        stack,
                        &UpdateContentDto {
                            uid: self.inner.uid.get(),
                            id,
                            created_at: self.inner.created_at.get(),
                            updated_at: now,
                            activated: true,
                            role,
                            data,
                        },
                    )?;
                }
                None => {
                    let Some(item_id) = self.inner.item_id.get() else {
                        anyhow::bail!("cannot create a content row without an owning item");
                    };
                    let created = content_commands::create_content(
                        ctx,
                        stack,
                        &CreateContentDto {
                            uid: Default::default(),
                            created_at: now,
                            updated_at: now,
                            activated: true,
                            role,
                            data,
                        },
                        item_id,
                        0,
                    )?;
                    self.inner.id.set(Some(created.id));
                    self.inner.created_at.set(now);
                    // `with_identity` minted this at the creation boundary; without
                    // caching it the very next save would update the row with a nil uid.
                    self.inner.uid.set(created.uid);
                }
            }
            self.inner.dirty.set(false);
            self.inner.loading_status.set(LoadingStatus::Loaded);
            Ok(())
        }

        /// Persist staged content **without recording it in the undo history**.
        ///
        /// For a write the application reconstructs from a source of truth it
        /// already undoes: prose is mirrored here from the live `TextDocument`
        /// on a timer, and that document owns the writer's typing history at
        /// word granularity. Recording the mirror as well put whole-Djot
        /// `Content::update` entries on the project's stack every few seconds,
        /// interleaved with the structural commands — which is what let a
        /// toast's Undo pop a prose flush instead of the thing it named, and
        /// what made a project-wide Ctrl+Z unsafe to offer at all.
        ///
        /// Per Qleany's own doctrine for a not-undoable command: it goes to a
        /// throwaway stack. [`common::undo_redo::UNTRACKED_STACK_ID`] is that
        /// idea taken to its limit — the command is dropped at the door, so
        /// there is nothing left to clear afterwards, which is the half nobody
        /// remembers to do.
        pub fn save_untracked(&self) -> anyhow::Result<()> {
            self.save(Some(common::undo_redo::UNTRACKED_STACK_ID))
        }

        /// The row of `role` the owning item holds now, read from the store.
        fn row_of_role(&self, role: &ContentRole) -> Option<ContentDto> {
            let item_id = self.inner.item_id.get()?;
            let content_ids = binder_item_commands::get_binder_item_relationship(
                &self.inner.ctx,
                &item_id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap_or_default();
            content_commands::get_content_multi(&self.inner.ctx, &content_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .find(|c| &c.role == role)
        }

        /// Re-fetch this field's row **by `(item_id, role)`**, not by cached row id
        /// — for a field another use case rewrote out from under this handle (a merge
        /// absorbing a neighbour, a split cutting the source in two).
        ///
        /// The id-gated [`refresh`](Self::refresh) is not enough: those use cases may
        /// have *created* the row for the first time, and this handle would still
        /// hold `id == None` and refresh into a no-op.
        pub fn reload(&self) {
            let Some(item_id) = self.inner.item_id.get() else {
                return;
            };
            let role = self.inner.role.borrow().clone();
            let content_ids = binder_item_commands::get_binder_item_relationship(
                &self.inner.ctx,
                &item_id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap_or_default();
            let found = content_commands::get_content_multi(&self.inner.ctx, &content_ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .find(|c| c.role == role);
            match found {
                Some(c) => {
                    self.inner.id.set(Some(c.id));
                    self.inner.created_at.set(c.created_at);
                    self.inner.uid.set(c.uid);
                    self.inner.data.set(c.data);
                }
                None => {
                    self.inner.id.set(None);
                    self.inner.data.set(String::new());
                }
            }
            self.inner.dirty.set(false);
            self.inner.loading_status.set(LoadingStatus::Loaded);
        }

        /// Auto-refresh the persisted `data` when this row changes elsewhere. The
        /// editor owns its live document, so it does NOT wire this for an open tab
        /// (snapshot-at-open); read-only consumers do. Call once from a long-lived
        /// widget's `build`.
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::Content(EntityEvent::Updated)),
                move |event: &Event| {
                    if s.inner
                        .id
                        .get()
                        .map(|id| event.ids.contains(&id))
                        .unwrap_or(false)
                    {
                        s.refresh();
                    }
                },
            );
        }

        fn refresh(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            match content_commands::get_content(&self.inner.ctx, &id) {
                Ok(Some(c)) => {
                    *self.inner.role.borrow_mut() = c.role;
                    self.inner.created_at.set(c.created_at);
                    self.inner.uid.set(c.uid);
                    self.inner.data.set(c.data);
                    self.inner.dirty.set(false);
                    self.inner.error_message.set(String::new());
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Ok(None) => {
                    self.inner.id.set(None);
                    self.inner.loading_status.set(LoadingStatus::Unloaded);
                }
                Err(e) => {
                    self.inner.error_message.set(e.to_string());
                    self.inner.loading_status.set(LoadingStatus::Error);
                }
            }
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use teksilo::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::ContentRole;
    use frontend::direct_access::ContentDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        role: RefCell<ContentRole>,
        id: Cell<Option<u64>>,
        data: Signal<String>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        dirty: Signal<bool>,
    }

    #[derive(Clone)]
    pub struct SingleContent {
        inner: Rc<Inner>,
    }

    /// Fabricated content for the mocks build.
    ///
    /// The mock app runs against a real-but-empty `AppContext` (there is no `#[cfg]`
    /// seam in `OpenDocsStore`), so every field's `existing` row is always `None` and
    /// every editor would render blank. Fabricating here — one level below `OpenDoc`
    /// — gives *every* mock tab plausible content, not just the streams, and keeps
    /// the seam where the house rules put it: in the singles, never in the consumers.
    fn fabricate(item_id: u64, role: &ContentRole) -> String {
        match role {
            ContentRole::SceneText => format!(
                "This is the fabricated body of scene {item_id}. The morning light crept over \
                 the ridgeline and the camp began to stir; she had not slept, and the cold had \
                 settled deep into her hands.\n\nA second paragraph follows, so the manuscript \
                 streams show real flowing prose per row."
            ),
            ContentRole::NoteText => {
                format!("Fabricated note {item_id}: a loose idea worth keeping.")
            }
            ContentRole::SynopsisText => format!(
                "Fabricated synopsis for item {item_id} — one or two lines saying what happens \
                 here, so the Full Synopsis stream reads as a working outline."
            ),
            ContentRole::BookTitle => "The Lighthouse".to_string(),
            ContentRole::BookSubtitle => "a novel".to_string(),
            ContentRole::ChapterTitle => format!("Chapter {item_id}"),
            ContentRole::PartTitle => format!("Part {item_id}"),
            ContentRole::ParatextText => format!(
                "Fabricated paratext {item_id}. Every place in this book is real; every \
                 person in it is not.\n\nA second paragraph, so the page reads as written \
                 matter rather than a stub."
            ),
            // A real epigraph shape: the quotation, a blank line, then the attribution
            // right-aligned — all inside one blockquote, which is what the export relies
            // on to keep the two together.
            ContentRole::EpigraphText => {
                "> The sea is not a place; it is a going.\n>\n> {alignment=right}\n> — Anon., *Tidewater*"
                    .to_string()
            }
            // The writer's own loose notes, in the shape they take: a question, a list.
            ContentRole::ScratchpadText => format!(
                "Fabricated scratchpad for item {item_id}: is this scene in the right place?\n\n\
                 - check the tide tables\n- *slower* opening"
            ),
        }
    }

    /// A deterministic, per-field id for a fabricated `Content` row.
    ///
    /// Offset well clear of the mock binder's own item ids so the two id spaces
    /// cannot be confused while debugging.
    ///
    /// Public because the fabricated *comment* set has to anchor to the same rows
    /// this fabricates. A comment carrying a hand-written content id would point at
    /// a document that does not exist, so the margin would be empty in every mocks
    /// build — which is exactly the build the feature is meant to be demonstrable
    /// in.
    pub fn mock_content_id(item_id: u64, role: &ContentRole) -> u64 {
        900_000 + item_id * MOCK_SLOTS_PER_ITEM + mock_slot(role)
    }

    /// How many content ids each mock item reserves. Room for every role and to spare:
    /// with ten slots per item, a tenth role took slot 10, which is the next item's
    /// slot 0, and two rows of two items shared an id.
    const MOCK_SLOTS_PER_ITEM: u64 = 100;

    /// A role's slot within its item's block, below [`MOCK_SLOTS_PER_ITEM`].
    fn mock_slot(role: &ContentRole) -> u64 {
        match role {
            ContentRole::SceneText => 1,
            ContentRole::NoteText => 2,
            ContentRole::SynopsisText => 3,
            ContentRole::BookTitle => 4,
            ContentRole::BookSubtitle => 5,
            ContentRole::PartTitle => 6,
            ContentRole::ChapterTitle => 7,
            ContentRole::EpigraphText => 8,
            ContentRole::ParatextText => 9,
            ContentRole::ScratchpadText => 10,
        }
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleContent {
        pub fn for_field(
            _ctx: Rc<AppContext>,
            item_id: u64,
            role: ContentRole,
            existing: Option<&ContentDto>,
        ) -> Self {
            let (id, data) = match existing {
                Some(c) => (Some(c.id), c.data.clone()),
                // A fabricated row still gets an id. `None` here would be a
                // *behavioural* difference from the real build, not just a missing
                // number: every feature keyed on "which Content is this" — comments
                // above all — resolves to nothing and silently disappears from the
                // mocks build, which is precisely where such a feature is meant to
                // be demonstrable. Derived from (item, role) so it is stable across
                // refreshes and distinct per field, the same contract
                // `common::uid::fixture_uid` provides for items.
                None => (
                    Some(mock_content_id(item_id, &role)),
                    fabricate(item_id, &role),
                ),
            };
            Self {
                inner: Rc::new(Inner {
                    role: RefCell::new(role),
                    id: Cell::new(id),
                    data: Signal::new(data),
                    loading_status: Signal::new(LoadingStatus::Loaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                }),
            }
        }

        pub fn from_id(_ctx: Rc<AppContext>, id: u64) -> Self {
            Self {
                inner: Rc::new(Inner {
                    role: RefCell::new(ContentRole::default()),
                    id: Cell::new(Some(id)),
                    data: Signal::new("Mock content".to_string()),
                    loading_status: Signal::new(LoadingStatus::Loaded),
                    error_message: Signal::new(String::new()),
                    dirty: Signal::new(false),
                }),
            }
        }

        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }
        pub fn role(&self) -> ContentRole {
            self.inner.role.borrow().clone()
        }
        pub fn data(&self) -> Signal<String> {
            self.inner.data.clone()
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }
        pub fn dirty(&self) -> Signal<bool> {
            self.inner.dirty.clone()
        }

        pub fn set_data(&self, v: String) {
            if self.inner.data.get() != v {
                self.inner.dirty.set(true);
                self.inner.data.set(v);
            }
        }

        pub fn save(&self, _stack: Option<u64>) -> anyhow::Result<()> {
            self.inner.dirty.set(false);
            Ok(())
        }

        /// Mock twin of the real [`save_untracked`]: nothing is recorded there
        /// either, so there is nothing to diverge from.
        pub fn save_untracked(&self) -> anyhow::Result<()> {
            self.save(None)
        }

        /// No backend to re-read: the fabricated data stands.
        pub fn reload(&self) {
            self.inner.dirty.set(false);
        }

        pub fn wire(&self, _ctx: &mut BuildContext) {}
    }
}

pub use imp::SingleContent;
#[cfg(feature = "mocks")]
pub use imp::mock_content_id;

#[cfg(all(test, feature = "mocks"))]
mod mock_id_tests {
    use frontend::common::entities::ContentRole;

    /// Every role of every item gets an id of its own. With ten slots per item the tenth
    /// role (the scratchpad, slot 10) landed on the next item's first id.
    #[test]
    fn no_two_fabricated_rows_share_an_id() {
        let roles = [
            ContentRole::SceneText,
            ContentRole::NoteText,
            ContentRole::SynopsisText,
            ContentRole::BookTitle,
            ContentRole::BookSubtitle,
            ContentRole::PartTitle,
            ContentRole::ChapterTitle,
            ContentRole::EpigraphText,
            ContentRole::ParatextText,
            ContentRole::ScratchpadText,
        ];
        let mut seen = std::collections::HashSet::new();
        for item in 0..500u64 {
            for role in &roles {
                let id = super::mock_content_id(item, role);
                assert!(seen.insert(id), "item {item} {role:?} reuses id {id}");
            }
        }
    }
}

#[cfg(all(test, not(feature = "mocks")))]
mod one_row_per_role_tests {
    use std::rc::Rc;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
    use frontend::direct_access::{ContentDto, CreateBinderItemDto, CreateContentDto};

    use super::SingleContent;

    fn scene(ctx: &AppContext) -> u64 {
        binder_item_commands::create_orphan_binder_item(
            ctx,
            None,
            &CreateBinderItemDto {
                role: BinderItemRole::Item,
                sub_role: BinderItemSubRole::Scene,
                ..Default::default()
            },
        )
        .expect("creating the scene")
        .id
    }

    fn scratchpad_row(ctx: &AppContext, item: u64, text: &str) -> ContentDto {
        content_commands::create_content(
            ctx,
            None,
            &CreateContentDto {
                role: ContentRole::ScratchpadText,
                activated: true,
                data: text.to_string(),
                ..Default::default()
            },
            item,
            -1,
        )
        .expect("creating the scratchpad row")
    }

    fn scratchpads(ctx: &AppContext, item: u64) -> Vec<String> {
        let ids = binder_item_commands::get_binder_item_relationship(
            ctx,
            &item,
            &BinderItemRelationshipField::Contents,
        )
        .expect("contents");
        content_commands::get_content_multi(ctx, &ids)
            .expect("rows")
            .into_iter()
            .flatten()
            .filter(|c| c.role == ContentRole::ScratchpadText)
            .map(|c| c.data)
            .collect()
    }

    /// A handle with no row yet, holding unsaved text, while a command made the row of
    /// its role underneath it (a merge folding the other scene's scratchpad in). Its
    /// save writes into that row, its text after the folded one, instead of creating a
    /// second row the scratchpad would never show.
    #[test]
    fn a_handle_with_no_row_writes_into_the_row_a_command_made() {
        let ctx = Rc::new(AppContext::new());
        let item = scene(&ctx);
        let handle = SingleContent::for_field(ctx.clone(), item, ContentRole::ScratchpadText, None);
        handle.set_data("The tide turns at four.".to_string());
        let folded = scratchpad_row(&ctx, item, "Ask Marthe about the lighthouse.");

        handle.save(None).expect("save");
        assert_eq!(
            scratchpads(&ctx, item),
            vec!["Ask Marthe about the lighthouse.\n\nThe tide turns at four.".to_string()],
            "one row, both notes"
        );
        assert_eq!(
            handle.id(),
            Some(folded.id),
            "the handle now writes that row"
        );

        handle.set_data("Ask Marthe about the lighthouse.\n\nAt four.".to_string());
        handle.save(None).expect("second save");
        assert_eq!(
            scratchpads(&ctx, item),
            vec!["Ask Marthe about the lighthouse.\n\nAt four.".to_string()],
            "the next save updates it as usual"
        );
    }

    /// A handle whose row a command took away (an undo removing a row made since,
    /// a redo joining it onto another). Updating the id brought the row back owned by
    /// nothing, so the text was never saved with the project. The save goes to the row
    /// of its role the item holds now, or to a new one when it holds none.
    #[test]
    fn a_handle_whose_row_was_taken_away_writes_to_the_items_row() {
        let ctx = Rc::new(AppContext::new());
        let item = scene(&ctx);
        let own = scratchpad_row(&ctx, item, "Check the ferry.");
        let handle =
            SingleContent::for_field(ctx.clone(), item, ContentRole::ScratchpadText, Some(&own));
        content_commands::remove_content(&ctx, None, &own.id).expect("removing the row");

        handle.set_data("Check the ferry timetable.".to_string());
        handle.save(None).expect("save");
        assert_eq!(
            scratchpads(&ctx, item),
            vec!["Check the ferry timetable.".to_string()],
            "a new row, owned by the item"
        );
        assert!(
            matches!(content_commands::get_content(&ctx, &own.id), Ok(None)),
            "the removed row was not brought back"
        );

        let now = handle.id().expect("the handle has a row again");
        content_commands::remove_content(&ctx, None, &now).expect("removing it again");
        let joined = scratchpad_row(&ctx, item, "Ask Marthe about the lighthouse.");
        handle.set_data("Check the ferry timetable. Twice.".to_string());
        handle.save(None).expect("save");
        assert_eq!(
            scratchpads(&ctx, item),
            vec![
                "Ask Marthe about the lighthouse.\n\nCheck the ferry timetable. Twice.".to_string()
            ],
        );
        assert_eq!(handle.id(), Some(joined.id));
    }
}
