// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `SingleBinderItem` — a reactive handle over one `BinderItem`.
//!
//! Caches the full `BinderItemDto` reactively (refreshed on the entity's `Updated`
//! events) and exposes it via [`dto`](imp::SingleBinderItem::dto) + mapped field
//! signals. Structural edits (reindent, move, trash) are tree mutations and stay in
//! `OutlineViewModel`'s undoable commands; this single is the read half for those.
//!
//! It does own **one** write, because that write must never be done by halves:
//! [`set_title`](imp::SingleBinderItem::set_title) and
//! [`set_sub_title`](imp::SingleBinderItem::set_sub_title).
//!
//! An item's name lives in two places — `BinderItem.title`, which the outline tree and
//! the tab show, and a title `Content` row (`BookTitle` / `PartTitle` / `ChapterTitle`),
//! which is what gets compiled into the manuscript. They are **one title with two
//! homes**. Writing only one of them is what made renaming a chapter in its editor leave
//! the tree showing the old name, and renaming it in the tree leave the manuscript
//! showing the old one. So every rename goes through here, and here writes both. Same
//! for the subtitle (`BinderItem.sub_title` + `BookSubtitle`).
//!
//! Two `mod imp` variants share one public surface. See [`crate::singles`].

#[cfg(not(feature = "mocks"))]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::commands::{binder_item_commands, content_commands};
    use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
    use frontend::common::entities::ContentRole;
    use frontend::common::event::{DirectAccessEntity, EntityEvent, Event, Origin};
    use frontend::direct_access::{BinderItemDto, ContentDto, UpdateBinderItemDto};

    use crate::singles::{LoadingStatus, SingleContent};

    /// Which of the item's two names is being written.
    #[derive(Clone, Copy)]
    enum TitlePart {
        Title,
        SubTitle,
    }

    /// A scalar-only update DTO from a fetched item (relationships untouched).
    fn update_dto(it: &BinderItemDto) -> UpdateBinderItemDto {
        UpdateBinderItemDto {
            id: it.id,
            created_at: it.created_at,
            updated_at: it.updated_at,
            // Carried through unchanged: `uid` is the item's durable identity,
            // never re-minted by an edit.
            uid: it.uid.clone(),
            title: it.title.clone(),
            sub_title: it.sub_title.clone(),
            role: it.role.clone(),
            sub_role: it.sub_role.clone(),
            label: it.label.clone(),
            activated: it.activated,
            is_favorite: it.is_favorite,
            is_exportable: it.is_exportable,
            indent: it.indent,
            word_count_goal: it.word_count_goal,
            char_count_goal: it.char_count_goal,
            dict_language: it.dict_language.clone(),
            aliases: it.aliases.clone(),
        }
    }

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<BinderItemDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
        ctx: Rc<AppContext>,
    }

    #[derive(Clone)]
    pub struct SingleBinderItem {
        inner: Rc<Inner>,
    }

    #[allow(dead_code)] // public reactive surface; wired to consumers incrementally
    impl SingleBinderItem {
        pub fn new(ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    dto: Signal::new(None),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                    ctx,
                }),
            }
        }

        /// Point the handle at an id (or clear it) and load the entity. Reads are
        /// synchronous, so the cached `dto` is current as soon as this returns —
        /// usable both as a persistent bound handle and as a one-shot probe.
        pub fn set_id(&self, id: Option<u64>) {
            self.inner.id.set(id);
            match id {
                Some(_) => self.refresh(),
                None => self.clear(),
            }
        }

        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }

        /// Auto-refresh when this `BinderItem` changes elsewhere.
        ///
        /// Call it from `build` — **every** build, not once. `BuildContext::subscribe_event`
        /// scopes a subscription to the widget's current build and drops it on the next
        /// one, so a "wire once" guard silently makes the handle deaf the first time its
        /// widget rebuilds. Re-subscribing cannot duplicate: the old callback is gone.
        /// (One-shot probes need not wire at all — each `set_id` reloads synchronously.)
        pub fn wire(&self, ctx: &mut BuildContext) {
            let s = self.clone();
            ctx.subscribe_event(
                Origin::DirectAccess(DirectAccessEntity::BinderItem(EntityEvent::Updated)),
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

        /// The cached full DTO (clone), if loaded.
        pub fn dto(&self) -> Option<BinderItemDto> {
            self.inner.dto.get()
        }
        /// The reactive whole-entity signal — bind UI to derived views of it.
        pub fn dto_signal(&self) -> Signal<Option<BinderItemDto>> {
            self.inner.dto.clone()
        }
        /// The item's title as a reactive signal (empty when unloaded).
        pub fn title(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.title.clone()).unwrap_or_default())
        }
        /// The item's subtitle as a reactive signal (empty when unloaded).
        pub fn sub_title(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.sub_title.clone()).unwrap_or_default())
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }

        /// Rename the item: writes `BinderItem.title` **and** the title `Content` row the
        /// constraint matrix gives this `(role, sub_role)`, if it has one.
        ///
        /// Both, always. The tree and the tab read the entity field; the manuscript reads
        /// the content row. Writing one without the other is how they drift apart.
        /// Undoable on `stack`.
        pub fn set_title(&self, title: &str, stack: Option<u64>) -> anyhow::Result<()> {
            self.write_name(title, TitlePart::Title, stack)
        }

        /// Set the item's subtitle: `BinderItem.sub_title` **and** its `BookSubtitle`
        /// row, when the matrix allows one (only a Book does).
        pub fn set_sub_title(&self, sub_title: &str, stack: Option<u64>) -> anyhow::Result<()> {
            self.write_name(sub_title, TitlePart::SubTitle, stack)
        }

        /// Set the item's `dict_language` (the space-separated language list). A scalar field
        /// with no mirrored `Content` row, so a plain read-modify-write — but the **full** DTO
        /// (`update_binder_item` replaces every scalar, so a partial one would blank the
        /// title/role/sub_role/indent). Undoable on `stack`.
        pub fn set_dict_language(&self, tags: &str, stack: Option<u64>) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleBinderItem: no id");
            };
            let Some(it) = self.dto() else {
                anyhow::bail!("SingleBinderItem: item {id} not loaded");
            };
            let mut dto = update_dto(&it);
            dto.dict_language = tags.to_string();
            dto.updated_at = chrono::Utc::now();
            binder_item_commands::update_binder_item(&self.inner.ctx, stack, &dto)?;
            self.refresh();
            Ok(())
        }

        /// Set the item's `is_exportable` flag (the per-item "include in exports" toggle).
        /// A scalar field with no mirrored `Content` row — a plain read-modify-write of the
        /// **full** DTO (like [`set_dict_language`](Self::set_dict_language)). Undoable on
        /// `stack`; a subtree "apply to children" wraps a run of these in one composite step.
        pub fn set_exportable(&self, on: bool, stack: Option<u64>) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleBinderItem: no id");
            };
            let Some(it) = self.dto() else {
                anyhow::bail!("SingleBinderItem: item {id} not loaded");
            };
            let mut dto = update_dto(&it);
            dto.is_exportable = on;
            dto.updated_at = chrono::Utc::now();
            binder_item_commands::update_binder_item(&self.inner.ctx, stack, &dto)?;
            self.refresh();
            Ok(())
        }

        /// Set the item's aliases — the other names it answers to in prose, which the
        /// mention index matches alongside its title.
        ///
        /// A list of primitives, not a relationship, so it rides the same scalar
        /// read-modify-write as [`set_dict_language`](Self::set_dict_language).
        pub fn set_aliases(&self, aliases: &[String], stack: Option<u64>) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleBinderItem: no id");
            };
            let Some(it) = self.dto() else {
                anyhow::bail!("SingleBinderItem: item {id} not loaded");
            };
            let mut dto = update_dto(&it);
            dto.aliases = aliases.to_vec();
            dto.updated_at = chrono::Utc::now();
            binder_item_commands::update_binder_item(&self.inner.ctx, stack, &dto)?;
            self.refresh();
            Ok(())
        }

        /// Set which palette tags this item carries.
        ///
        /// Unlike every other writer here this is a **relationship**, so it must NOT go
        /// through `update_dto`: `UpdateBinderItemDto` deliberately carries no relationship
        /// vectors, precisely so a scalar patch cannot clobber them (see
        /// `view_models::binder_ops::update_item_dto`). Writing the junction directly is
        /// also already undoable — `set_binder_item_relationship` is backed by
        /// `UndoableSetRelationshipUseCase`, which stores the before-list itself.
        /// Persist the item's confirmed references — the story-bible entries a writer has
        /// pinned. Same relationship write as `set_tags`, and undoable on the same stack: a
        /// pin is an edit, and mis-pinning must be one Ctrl+Z.
        pub fn set_references(&self, item_ids: &[u64], stack: Option<u64>) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleBinderItem: no id");
            };
            binder_item_commands::set_binder_item_relationship(
                &self.inner.ctx,
                stack,
                &frontend::direct_access::BinderItemRelationshipDto {
                    id,
                    field: frontend::common::direct_access::binder_item::BinderItemRelationshipField::References,
                    right_ids: item_ids.to_vec(),
                },
            )?;
            self.refresh();
            Ok(())
        }

        pub fn set_tags(&self, tag_ids: &[u64], stack: Option<u64>) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleBinderItem: no id");
            };
            binder_item_commands::set_binder_item_relationship(
                &self.inner.ctx,
                stack,
                &frontend::direct_access::BinderItemRelationshipDto {
                    id,
                    field: frontend::common::direct_access::binder_item::BinderItemRelationshipField::Tags,
                    right_ids: tag_ids.to_vec(),
                },
            )?;
            self.refresh();
            Ok(())
        }

        fn write_name(
            &self,
            text: &str,
            part: TitlePart,
            stack: Option<u64>,
        ) -> anyhow::Result<()> {
            let Some(id) = self.inner.id.get() else {
                anyhow::bail!("SingleBinderItem: no id");
            };
            let Some(it) = self.dto() else {
                anyhow::bail!("SingleBinderItem: item {id} not loaded");
            };

            // 1. The entity field — what the outline tree and the tab show.
            let mut dto = update_dto(&it);
            match part {
                TitlePart::Title => dto.title = text.to_string(),
                TitlePart::SubTitle => dto.sub_title = text.to_string(),
            }
            dto.updated_at = chrono::Utc::now();
            binder_item_commands::update_binder_item(&self.inner.ctx, stack, &dto)?;

            // 2. The matching Content row — what the manuscript compiles.
            let role = match part {
                TitlePart::Title => skribisto_model::title_role_of(&it.role, &it.sub_role),
                TitlePart::SubTitle => skribisto_model::content_allowed(
                    &it.role,
                    &it.sub_role,
                    &ContentRole::BookSubtitle,
                )
                .then_some(ContentRole::BookSubtitle),
            };
            if let Some(role) = role {
                let existing = self.content_row(id, &role);
                let field =
                    SingleContent::for_field(self.inner.ctx.clone(), id, role, existing.as_ref());
                field.set_data(text.to_string());
                field.save(stack)?;
            }

            self.refresh();
            Ok(())
        }

        /// The item's `Content` row for `role`, if it exists.
        fn content_row(&self, id: u64, role: &ContentRole) -> Option<ContentDto> {
            let ids = binder_item_commands::get_binder_item_relationship(
                &self.inner.ctx,
                &id,
                &BinderItemRelationshipField::Contents,
            )
            .unwrap_or_default();
            content_commands::get_content_multi(&self.inner.ctx, &ids)
                .unwrap_or_default()
                .into_iter()
                .flatten()
                .find(|c| &c.role == role)
        }

        fn refresh(&self) {
            let Some(id) = self.inner.id.get() else {
                return;
            };
            self.inner.loading_status.set(LoadingStatus::Loading);
            match binder_item_commands::get_binder_item(&self.inner.ctx, &id) {
                Ok(Some(it)) => {
                    self.inner.dto.set(Some(it));
                    self.inner.error_message.set(String::new());
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                Ok(None) => self.clear(),
                Err(e) => self.fail(&e.to_string()),
            }
        }

        fn clear(&self) {
            self.inner.dto.set(None);
            self.inner.error_message.set(String::new());
            self.inner.loading_status.set(LoadingStatus::Unloaded);
        }

        fn fail(&self, msg: &str) {
            self.inner.error_message.set(msg.to_string());
            self.inner.loading_status.set(LoadingStatus::Error);
        }
    }
}

#[cfg(feature = "mocks")]
mod imp {
    use std::cell::Cell;
    use std::rc::Rc;

    use bastyde::prelude::*;

    use frontend::AppContext;
    use frontend::common::entities::{BinderItemRole, BinderItemSubRole};
    use frontend::direct_access::BinderItemDto;

    use crate::singles::LoadingStatus;

    struct Inner {
        id: Cell<Option<u64>>,
        dto: Signal<Option<BinderItemDto>>,
        loading_status: Signal<LoadingStatus>,
        error_message: Signal<String>,
    }

    #[derive(Clone)]
    pub struct SingleBinderItem {
        inner: Rc<Inner>,
    }

    /// Fabricated item matching the mock binder-tree fixture, so every container
    /// opens onto a real stream: the Book (101) holds a Part (301) holding two
    /// chapters — a chapter folder (104) with scenes 201-203, and a flat
    /// `Item/ChapterScene` (302) followed by scene 303. Anything else falls back to
    /// a plain Scene. Kept in step with `models::StreamRowsModel`'s mock rows.
    fn mock_dto(id: u64) -> BinderItemDto {
        use BinderItemRole::*;
        use BinderItemSubRole::*;
        let (role, sub_role, title) = match id {
            101 => (Folder, Book, "Book One"),
            102 => (Item, BookBegin, "Opening"),
            103 => (Item, Scene, "Scene at dawn"),
            104 => (Folder, ChapterScene, "Chapter Two"),
            105 => (Item, ChapterScene, "Confrontation"),
            106 => (Item, Note, "Character sketch"),
            107 => (Item, Text, "Random idea"),
            201 => (Item, Scene, "Scene 1"),
            202 => (Item, Scene, "Scene 2"),
            203 => (Item, Scene, "Scene 3"),
            301 => (Folder, Part, "Part One — Arrival"),
            302 => (Item, ChapterScene, "Into the Dark"),
            303 => (Item, Scene, "The light returns"),
            _ => (Item, Scene, "Mock Item"),
        };
        BinderItemDto {
            id,
            // DETERMINISTIC, not `new_uid()` — see `single_binder::mock_dto`.
            uid: common::uid::fixture_uid(id),
            title: title.to_string(),
            role,
            sub_role,
            is_exportable: true,
            ..Default::default()
        }
    }

    #[allow(dead_code)] // identical surface to the real variant; some unused under mocks
    impl SingleBinderItem {
        pub fn new(_ctx: Rc<AppContext>) -> Self {
            Self {
                inner: Rc::new(Inner {
                    id: Cell::new(None),
                    dto: Signal::new(None),
                    loading_status: Signal::new(LoadingStatus::Unloaded),
                    error_message: Signal::new(String::new()),
                }),
            }
        }

        pub fn set_id(&self, id: Option<u64>) {
            self.inner.id.set(id);
            match id {
                Some(i) => {
                    self.inner.dto.set(Some(mock_dto(i)));
                    self.inner.loading_status.set(LoadingStatus::Loaded);
                }
                None => {
                    self.inner.dto.set(None);
                    self.inner.loading_status.set(LoadingStatus::Unloaded);
                }
            }
        }
        pub fn id(&self) -> Option<u64> {
            self.inner.id.get()
        }
        pub fn wire(&self, _ctx: &mut BuildContext) {}

        pub fn dto(&self) -> Option<BinderItemDto> {
            self.inner.dto.get()
        }
        pub fn dto_signal(&self) -> Signal<Option<BinderItemDto>> {
            self.inner.dto.clone()
        }
        pub fn title(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.title.clone()).unwrap_or_default())
        }
        pub fn sub_title(&self) -> Signal<String> {
            self.inner
                .dto
                .map(|d| d.as_ref().map(|x| x.sub_title.clone()).unwrap_or_default())
        }
        pub fn loading_status(&self) -> Signal<LoadingStatus> {
            self.inner.loading_status.clone()
        }
        pub fn error_message(&self) -> Signal<String> {
            self.inner.error_message.clone()
        }

        /// Rename the fabricated item, so the mock outline and tab track the edit just
        /// like the real ones.
        pub fn set_title(&self, title: &str, _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.title = title.to_string();
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }

        pub fn set_sub_title(&self, sub_title: &str, _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.sub_title = sub_title.to_string();
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }

        pub fn set_references(&self, _item_ids: &[u64], _stack: Option<u64>) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn set_dict_language(&self, tags: &str, _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.dict_language = tags.to_string();
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }

        pub fn set_exportable(&self, on: bool, _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.is_exportable = on;
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }

        pub fn set_aliases(&self, aliases: &[String], _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.aliases = aliases.to_vec();
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }

        /// Mirrors the real writer's *effect* (the DTO's `tags` vector changes) even though
        /// the real one writes a junction rather than a DTO field — consumers must not be
        /// able to tell the two halves apart.
        pub fn set_tags(&self, tag_ids: &[u64], _stack: Option<u64>) -> anyhow::Result<()> {
            if let Some(mut d) = self.inner.dto.get() {
                d.tags = tag_ids.to_vec();
                self.inner.dto.set(Some(d));
            }
            Ok(())
        }
    }
}

pub use imp::SingleBinderItem;
