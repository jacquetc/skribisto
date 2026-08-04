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

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::{ContentDto, CreateContentDto, UpdateContentDto};

    use crate::singles::LoadingStatus;

    struct Inner {
        /// The owning `BinderItem` — needed to create the row if it doesn't exist.
        item_id: Cell<Option<u64>>,
        role: RefCell<ContentRole>,
        id: Cell<Option<u64>>,
        created_at: Cell<chrono::DateTime<chrono::Utc>>,
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
            let (id, data, created_at) = match existing {
                Some(c) => (Some(c.id), c.data.clone(), c.created_at),
                None => (None, String::new(), chrono::Utc::now()),
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
        pub fn save(&self, stack: Option<u64>) -> anyhow::Result<()> {
            if !self.inner.dirty.get() {
                return Ok(());
            }
            let ctx = &*self.inner.ctx;
            let now = chrono::Utc::now();
            let role = self.inner.role.borrow().clone();
            let data = self.inner.data.get();
            match self.inner.id.get() {
                Some(id) => {
                    content_commands::update_content(
                        ctx,
                        stack,
                        &UpdateContentDto {
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
                }
            }
            self.inner.dirty.set(false);
            self.inner.loading_status.set(LoadingStatus::Loaded);
            Ok(())
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

    use bastyde::prelude::*;

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
        let slot = match role {
            ContentRole::SceneText => 1,
            ContentRole::NoteText => 2,
            ContentRole::SynopsisText => 3,
            ContentRole::BookTitle => 4,
            ContentRole::BookSubtitle => 5,
            ContentRole::PartTitle => 6,
            ContentRole::ChapterTitle => 7,
            ContentRole::EpigraphText => 8,
            ContentRole::ParatextText => 9,
        };
        900_000 + item_id * 10 + slot
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
