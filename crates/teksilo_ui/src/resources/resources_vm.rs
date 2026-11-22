// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `ResourcesViewModel`: everything the Resources dock does, and the two drops onto the
//! text that add or link a resource.
//!
//! One per window, over the project's [`ResourcesSession`] (Tier 2). It follows the item
//! in front of the writer (`EditorsViewModel::active_item`, as every trailing dock does)
//! and owns no row of its own: it reads the session's model and writes through it.
//!
//! ## Adding files
//!
//! A file over [`sizes::LARGE_FILE_BYTES`] asks first: store it in the project, link to
//! it where it is, or cancel, with a "Don't ask again" that sets
//! [`crate::RESOURCES_LARGE_FILE_KEY`]. The stored ones are copied by a long operation
//! (progress, Cancel) into the project's resources directory, and only then become rows,
//! in one undo step with the linked ones. A Save As that moved the project while the
//! copy ran moves the directory too, so before the rows are made the new uid folders are
//! moved to where the project now keeps them ([`move_stored_folders`]).
//!
//! ## Opening
//!
//! A web link goes through the external-link allow-list, never as a `file:`. A linked
//! file opens in place. A stored file opens in place in a folder project and as a named
//! copy under the app's data in a single-file one ([`opened_files`]); both are remembered
//! so an edit made in the other application comes back ([`ResourcesSession`]). A type
//! not on [`open_safety`]'s list asks first and offers to show the file in its folder.
//! A picture opens in the image viewer, which offers the other application itself.
//!
//! [`opened_files`]: crate::resources::opened_files

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use teksilo::prelude::*;
use teksilo::settings::SettingsExt;
use teksilo::widgets::rich_text::EditorHandle;
use teksilo::widgets::{
    MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton, Toast, ToastAction,
};

use frontend::AppContext;
use frontend::common::entities::WorkShape;
use frontend::common::event::Event;

use crate::app_ids::AppIds;
use crate::models::{ResourceRow, StoreStart, StoredResourceFilesDto, WebLink};
use crate::resources::links::{self, InsertedSpan};
use crate::resources::open_safety::{self, OpenVerdict};
use crate::resources::opened_files::{self, OpenedWhere};
use crate::resources::session::{AttachNotice, ChangeNotice, ResourcesSession};
use crate::resources::{file_kind, sizes};
use crate::shared::long_op::{TrackedOp, event_id};
use crate::shared::progress_toast::ProgressToast;
use crate::toast_scope::ToastWorkExt;

/// How long a toast offering an Undo stays up.
const UNDO_GRACE: Duration = Duration::from_secs(10);

/// The toast id of a store run's progress, scoped to its Work and operation.
const STORE_TOAST_ID: &str = "resources.store";

/// What a large file does, when the writer has said once and for all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LargeFileChoice {
    Store,
    Link,
}

/// The remembered answer, `None` while the writer still wants to be asked.
pub fn large_file_choice(setting: &str) -> Option<LargeFileChoice> {
    match setting {
        "store" => Some(LargeFileChoice::Store),
        "link" => Some(LargeFileChoice::Link),
        _ => None,
    }
}

/// Where new resources go once they exist.
#[derive(Clone)]
pub enum AddTarget {
    /// Added from the dock: listed on the item, nothing else.
    Dock,
    /// Dropped on a text: listed on the item, then linked at `at` in that text.
    Text { handle: EditorHandle, at: usize },
}

/// A store run in flight, and what to do with its result.
struct PendingStore {
    op: TrackedOp,
    dest: PathBuf,
    items: Vec<u64>,
    linked: Vec<PathBuf>,
    target: AddTarget,
}

/// Opens the project's Save as folder dialog. A closure, so this view-model reaches the
/// Save As machinery without importing it.
pub type SaveAsFolderFn = Rc<dyn Fn(&mut EventContext)>;

/// The editor a command writes into: the one the writer last had the caret in.
pub type EditorFn = Rc<dyn Fn() -> Option<EditorHandle>>;

/// The project's pictures folder, where a picture put into the text is stored.
pub type MediaDirFn = Rc<dyn Fn() -> PathBuf>;

struct Inner {
    app_ctx: Rc<AppContext>,
    ids: AppIds,
    session: ResourcesSession,
    focus: Signal<Option<u64>>,
    shape: Signal<Option<WorkShape>>,
    unique_id: Signal<String>,
    editor: RefCell<Option<EditorFn>>,
    media_dir: RefCell<Option<MediaDirFn>>,
    save_as_folder: RefCell<Option<SaveAsFolderFn>>,
    store: RefCell<Option<PendingStore>>,
    progress: ProgressToast,
}

/// The Resources dock's logic for one window. Cheap to clone; clones share it.
#[derive(Clone)]
pub struct ResourcesViewModel {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for ResourcesViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourcesViewModel")
            .field("focus", &self.inner.focus.get())
            .finish()
    }
}

impl ResourcesViewModel {
    pub fn new(
        app_ctx: Rc<AppContext>,
        ids: AppIds,
        session: ResourcesSession,
        focus: Signal<Option<u64>>,
        shape: Signal<Option<WorkShape>>,
        unique_id: Signal<String>,
    ) -> Self {
        Self {
            inner: Rc::new(Inner {
                app_ctx,
                ids,
                session,
                focus,
                shape,
                unique_id,
                editor: RefCell::new(None),
                media_dir: RefCell::new(None),
                save_as_folder: RefCell::new(None),
                store: RefCell::new(None),
                progress: ProgressToast::default(),
            }),
        }
    }

    /// A view-model over a session of its own, following no item: for a tab built with
    /// no window behind it (a test, a standalone tab), which still has to build.
    pub fn detached(app_ctx: Rc<AppContext>, ids: AppIds) -> Self {
        let session = ResourcesSession::new(app_ctx.clone(), ids.clone());
        Self::new(
            app_ctx,
            ids,
            session,
            Signal::new(None),
            Signal::new(None),
            Signal::new(String::new()),
        )
    }

    /// The project's shape, as the Work's Resources page says whether a folder would
    /// save faster.
    pub fn shape(&self) -> Signal<Option<WorkShape>> {
        self.inner.shape.clone()
    }

    /// Hand over what this view-model reaches through the window rather than owns.
    pub fn connect(&self, editor: EditorFn, media_dir: MediaDirFn, save_as_folder: SaveAsFolderFn) {
        *self.inner.editor.borrow_mut() = Some(editor);
        *self.inner.media_dir.borrow_mut() = Some(media_dir);
        *self.inner.save_as_folder.borrow_mut() = Some(save_as_folder);
    }

    pub fn session(&self) -> &ResourcesSession {
        &self.inner.session
    }

    /// The item whose resources the dock shows.
    pub fn focus(&self) -> Signal<Option<u64>> {
        self.inner.focus.clone()
    }

    /// Bumps whenever a row may have changed.
    pub fn version(&self) -> Signal<u64> {
        self.inner.session.model().version_signal()
    }

    /// The item in front of the writer's resources, by title.
    pub fn rows(&self) -> Vec<ResourceRow> {
        match self.inner.focus.get() {
            Some(item) => sorted_by_title(self.inner.session.model().rows_for(item)),
            None => Vec::new(),
        }
    }

    fn items(&self) -> Vec<u64> {
        self.inner.focus.get().into_iter().collect()
    }

    fn editor(&self) -> Option<EditorHandle> {
        self.inner.editor.borrow().as_ref().and_then(|f| f())
    }

    fn media_dir(&self) -> PathBuf {
        self.inner
            .media_dir
            .borrow()
            .as_ref()
            .map(|f| f())
            .unwrap_or_default()
    }

    fn work_id(&self) -> Option<u64> {
        self.inner.ids.work_id.get()
    }

    // ── Adding ───────────────────────────────────────────────────────────────

    /// Ask for files to attach to the item in front of the writer.
    pub fn pick_files(&self, ctx: &mut EventContext) {
        self.pick_files_for(ctx, self.items());
    }

    /// Ask for files to attach to `items`: a folder's Resources page attaches to the
    /// folder, whatever is in front of the writer.
    pub fn pick_files_for(&self, ctx: &mut EventContext, items: Vec<u64>) {
        if items.is_empty() {
            return;
        }
        let req = crate::models::dialog_start_in(
            ctx,
            crate::models::FolderPurpose::AttachResources,
            FileDialogRequest::pick_files().title(tr!(resources_add_files_title())),
        );
        let vm = self.clone();
        let _ = ctx.pick_files(req, move |res, c| {
            crate::models::remember_pick(c, crate::models::FolderPurpose::AttachResources, &res);
            let FileDialogResult::Files(paths) = res else {
                return;
            };
            if !paths.is_empty() {
                vm.add_paths_to(c, items.clone(), paths, AddTarget::Dock);
            }
        });
    }

    /// Attach `paths` to the item in front of the writer: ask about the large ones,
    /// store the rest, link what the writer chose to link. See the module docs.
    pub fn add_paths(&self, ctx: &mut EventContext, paths: Vec<PathBuf>, target: AddTarget) {
        self.add_paths_to(ctx, self.items(), paths, target);
    }

    /// [`Self::add_paths`], attaching to `items` rather than the item in front of the
    /// writer.
    pub fn add_paths_to(
        &self,
        ctx: &mut EventContext,
        items: Vec<u64>,
        paths: Vec<PathBuf>,
        target: AddTarget,
    ) {
        if items.is_empty() || paths.is_empty() {
            return;
        }
        let large: Vec<(PathBuf, u64)> = paths
            .iter()
            .filter_map(|p| {
                let len = std::fs::metadata(p).ok()?.len();
                sizes::is_large(len).then(|| (p.clone(), len))
            })
            .collect();
        let setting = large_file_signal(ctx);
        let (first, len) = match (large.first(), large_file_choice(&setting.get())) {
            (None, _) => {
                self.begin_store(ctx, paths, Vec::new(), items, target);
                return;
            }
            (Some(_), Some(choice)) => {
                let (store, link) = split_large(paths, &large, choice);
                self.begin_store(ctx, store, link, items, target);
                return;
            }
            (Some((first, len)), None) => (first.clone(), *len),
        };
        let vm = self.clone();
        let name = file_label(&first);
        let text = if large.len() > 1 {
            tr!(resources_large_text_many(
                name = name,
                size = crate::shared::units::byte_size(len),
                others = (large.len() - 1) as i64
            ))
        } else {
            tr!(resources_large_text(
                name = name,
                size = crate::shared::units::byte_size(len)
            ))
        };
        MessageBox::question(tr!(resources_large_title()))
            .text(text)
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Ok).label(tr!(resources_large_store())),
                MessageBoxButton::standard(StandardButton::No).label(tr!(resources_large_link())),
                MessageBoxButton::standard(StandardButton::Cancel),
            ]))
            // Storing is the default: it is what a small file does, and the one answer
            // under which the project keeps the file whatever happens to the original.
            .default_button(StandardButton::Ok)
            .escape_button(StandardButton::Cancel)
            .show_again_checkbox(tr!(resources_large_remember()))
            .on_result(move |r, c| {
                let choice = match r.button {
                    StandardButton::Ok => LargeFileChoice::Store,
                    StandardButton::No => LargeFileChoice::Link,
                    _ => return,
                };
                if r.checkbox_checked {
                    large_file_signal(c).set(
                        match choice {
                            LargeFileChoice::Store => "store",
                            LargeFileChoice::Link => "link",
                        }
                        .to_string(),
                    );
                }
                let (store, link) = split_large(paths.clone(), &large, choice);
                vm.begin_store(c, store, link, items.clone(), target.clone());
            })
            .present(ctx);
    }

    /// Copy `store` into the project (a long operation), then add it and `link`.
    fn begin_store(
        &self,
        ctx: &mut EventContext,
        store: Vec<PathBuf>,
        link: Vec<PathBuf>,
        items: Vec<u64>,
        target: AddTarget,
    ) {
        if store.is_empty() {
            self.finish_add(
                ctx,
                &StoredResourceFilesDto::default(),
                &link,
                &items,
                target,
            );
            return;
        }
        if self.inner.store.borrow().is_some() {
            ctx.show_toast(
                Toast::warning(tr!(resources_store_busy()))
                    .scoped_id("resources.busy", self.work_id()),
            );
            return;
        }
        let dest = self.inner.session.model().resources_dir();
        if dest.as_os_str().is_empty() {
            ctx.show_toast(
                Toast::warning(tr!(resources_no_project()))
                    .scoped_id("resources.no_project", self.work_id()),
            );
            return;
        }
        match self.inner.session.model().start_store(&dest, &store) {
            Ok(StoreStart::Done(dto)) => {
                let dto = self.bring_to_current_dir(&dest, *dto);
                self.finish_add(ctx, &dto, &link, &items, target);
            }
            Ok(StoreStart::Running(op_id)) => {
                *self.inner.store.borrow_mut() = Some(PendingStore {
                    op: TrackedOp::start(&self.inner.ids, op_id),
                    dest,
                    items,
                    linked: link,
                    target,
                });
                let toast = self.progress_toast(0.0, "");
                self.inner.progress.show(ctx, toast);
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_store_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
            }
        }
    }

    fn progress_toast(&self, percent: f32, message: &str) -> Toast {
        let vm = self.clone();
        let (work, op) = match self.inner.store.borrow().as_ref() {
            Some(p) => (p.op.work_id(), p.op.op_id().to_string()),
            None => (
                crate::shared::long_op::CapturedWork::now(&self.inner.ids),
                String::new(),
            ),
        };
        Toast::loading(tr!(resources_storing_title()))
            .scoped_op_id(STORE_TOAST_ID, work, &op)
            .body(crate::shared::progress_text::progress_body(
                percent, message,
            ))
            .target_work(work)
            .action(
                ToastAction::destructive(tr!(resources_store_cancel()), move |_c| {
                    vm.cancel_store()
                })
                .closes_toast(false),
            )
    }

    /// Stop the store run in flight.
    pub fn cancel_store(&self) {
        if let Some(p) = self.inner.store.borrow().as_ref() {
            self.inner.session.model().cancel_store(p.op.op_id());
        }
    }

    fn matches(&self, event: &Event) -> bool {
        let Some(id) = event_id(event) else {
            return false;
        };
        self.inner
            .store
            .borrow()
            .as_ref()
            .is_some_and(|p| p.op.matches(&id))
    }

    pub fn on_long_op_progress(&self, ctx: &mut EventContext, event: &Event) {
        if !self.matches(event) {
            return;
        }
        let Some(payload) = crate::shared::long_op::parse_payload(event) else {
            return;
        };
        let percent = payload
            .get("percentage")
            .and_then(|p| p.as_f64())
            .unwrap_or(0.0) as f32;
        let message = payload
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("")
            .to_string();
        let toast = self.progress_toast(percent, &message);
        self.inner.progress.show(ctx, toast);
    }

    pub fn on_long_op_completed(&self, ctx: &mut EventContext, event: &Event) {
        if !self.matches(event) {
            return;
        }
        let Some(pending) = self.inner.store.borrow_mut().take() else {
            return;
        };
        self.inner.progress.dismiss(ctx);
        let Some(dto) = self.inner.session.model().store_result(pending.op.op_id()) else {
            return;
        };
        let dto = self.bring_to_current_dir(&pending.dest, dto);
        self.finish_add(ctx, &dto, &pending.linked, &pending.items, pending.target);
    }

    pub fn on_long_op_cancelled(&self, ctx: &mut EventContext, event: &Event) {
        if !self.matches(event) {
            return;
        }
        let pending = self.inner.store.borrow_mut().take();
        let work = pending.map(|p| p.op.work_id());
        self.inner.progress.end_replacing(
            ctx,
            Toast::info(tr!(resources_store_cancelled()))
                .scoped_id("resources.cancelled", work.and_then(Option::<u64>::from))
                .auto_dismiss_after(Duration::from_secs(4)),
        );
    }

    pub fn on_long_op_failed(&self, ctx: &mut EventContext, event: &Event) {
        if !self.matches(event) {
            return;
        }
        let error = crate::shared::long_op::parse_payload(event)
            .and_then(|p| p.get("error").and_then(|e| e.as_str()).map(str::to_string))
            .unwrap_or_default();
        let pending = self.inner.store.borrow_mut().take();
        let work: Option<u64> = pending.and_then(|p| p.op.work_id().into());
        // A cancel can arrive as a failure carrying the use case's own words for it.
        let toast = if error.contains("operation cancelled") {
            Toast::info(tr!(resources_store_cancelled()))
        } else {
            Toast::error(tr!(resources_store_failed(error = error)))
        };
        self.inner
            .progress
            .end_replacing(ctx, toast.scoped_id("resources.failed", work));
    }

    /// A store run wrote into `dest`; if the project's resources directory is
    /// elsewhere now, move what it wrote there. A file that cannot be moved is reported
    /// as failed rather than turned into a row that would point at nothing.
    fn bring_to_current_dir(
        &self,
        dest: &Path,
        mut dto: StoredResourceFilesDto,
    ) -> StoredResourceFilesDto {
        let now = self.inner.session.model().resources_dir();
        if now.as_os_str().is_empty() || now == dest {
            return dto;
        }
        let failed = move_stored_folders(dest, &now, &dto.uids);
        if failed.is_empty() {
            return dto;
        }
        let mut kept = StoredResourceFilesDto {
            failed_paths: std::mem::take(&mut dto.failed_paths),
            failure_reasons: std::mem::take(&mut dto.failure_reasons),
            ..Default::default()
        };
        for i in 0..dto.uids.len() {
            if failed.contains(&dto.uids[i]) {
                kept.failed_paths.push(dto.source_paths[i].clone());
                kept.failure_reasons
                    .push(tr!(resources_moved_away()).resolve_now());
                continue;
            }
            kept.source_paths.push(dto.source_paths[i].clone());
            kept.uids.push(dto.uids[i].clone());
            kept.file_names.push(dto.file_names[i].clone());
            kept.mime_types.push(dto.mime_types[i].clone());
            kept.byte_sizes.push(dto.byte_sizes[i]);
            kept.content_hashes.push(dto.content_hashes[i].clone());
            kept.widths.push(dto.widths[i]);
            kept.heights.push(dto.heights[i]);
        }
        kept
    }

    /// Make the rows, say what failed, link them where they were dropped, and offer the
    /// folder shape if the project just grew past the notice's size.
    fn finish_add(
        &self,
        ctx: &mut EventContext,
        stored: &StoredResourceFilesDto,
        linked: &[PathBuf],
        items: &[u64],
        target: AddTarget,
    ) {
        let work = self.work_id();
        if !stored.failed_paths.is_empty() {
            let names: Vec<String> = stored
                .failed_paths
                .iter()
                .map(|p| file_label(Path::new(p)))
                .collect();
            ctx.show_toast(
                Toast::warning(tr!(resources_some_failed(
                    count = names.len() as i64,
                    names = names.join(", ")
                )))
                .body(lit!(stored.failure_reasons.join("\n")))
                .scoped_id("resources.some_failed", work),
            );
        }
        if stored.uids.is_empty() && linked.is_empty() {
            return;
        }
        let before = self.stored_bytes();
        let created = match self.inner.session.model().add(items, stored, linked, &[]) {
            Ok(created) => created,
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_add_failed(error = e)))
                        .scoped_id("resources.failed", work),
                );
                return;
            }
        };
        self.inner.session.model().reload();
        ctx.announce(tr!(resources_added(count = created.len() as i64)).resolve_now());
        if let AddTarget::Text { handle, at } = target {
            self.link_added_at(ctx, &handle, at, &created);
        }
        self.offer_folder_project(ctx, before, self.stored_bytes());
    }

    /// The newly added resources, linked at `at` in `handle`'s text, and a toast whose
    /// Undo takes back both the rows and the words.
    fn link_added_at(&self, ctx: &mut EventContext, handle: &EditorHandle, at: usize, ids: &[u64]) {
        if !crate::shared::editor_writes::accepts_writes(handle) {
            return;
        }
        let rows: Vec<ResourceRow> = ids
            .iter()
            .filter_map(|id| self.inner.session.model().by_id(*id))
            .collect();
        let links: Vec<(String, String)> = rows.iter().map(link_of).collect();
        if links.is_empty() {
            return;
        }
        let at = at.min(handle.to_plain_text().chars().count());
        let span = links::insert_links(handle, at, &links);
        let vm = self.clone();
        let handle = handle.clone();
        let ids = ids.to_vec();
        ctx.show_toast(
            Toast::success(tr!(resources_dropped_added(count = ids.len() as i64)))
                .scoped_id("resources.dropped", self.work_id())
                .auto_dismiss_after(UNDO_GRACE)
                .action(ToastAction::primary(tr!(resources_undo()), move |c| {
                    vm.undo_drop(c, &handle, &span, &ids);
                })),
        );
    }

    /// Take back a drop: the link text this window inserted, and the resources.
    fn undo_drop(
        &self,
        ctx: &mut EventContext,
        handle: &EditorHandle,
        span: &InsertedSpan,
        ids: &[u64],
    ) {
        links::remove_span(handle, span);
        if let Err(e) = self.inner.session.model().remove(ids) {
            ctx.show_toast(
                Toast::error(tr!(resources_remove_failed(error = e)))
                    .scoped_id("resources.failed", self.work_id()),
            );
        }
    }

    /// Attach a web address to the item in front of the writer.
    pub fn add_web_link(&self, ctx: &mut EventContext, url: &str, title: &str) -> bool {
        self.add_web_link_to(ctx, &self.items(), url, title)
    }

    /// Attach a web address to `items`.
    pub fn add_web_link_to(
        &self,
        ctx: &mut EventContext,
        items: &[u64],
        url: &str,
        title: &str,
    ) -> bool {
        if items.is_empty() {
            return false;
        }
        let url = crate::format::link_panel::normalize_href(url);
        if !is_web_address(&url) {
            ctx.show_toast(
                Toast::warning(tr!(resources_link_refused(url = url)))
                    .scoped_id("resources.link_refused", self.work_id()),
            );
            return false;
        }
        let link = WebLink {
            url,
            title: title.trim().to_string(),
        };
        match self.inner.session.model().add(
            items,
            &StoredResourceFilesDto::default(),
            &[],
            std::slice::from_ref(&link),
        ) {
            Ok(created) => {
                self.inner.session.model().reload();
                ctx.announce(tr!(resources_added(count = created.len() as i64)).resolve_now());
                true
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_add_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
                false
            }
        }
    }

    /// Text dropped or pasted on the dock: every web address in it becomes a link.
    pub fn add_dropped_text(&self, ctx: &mut EventContext, text: &str) -> bool {
        self.add_dropped_text_to(ctx, &self.items(), text)
    }

    /// Text dropped or pasted on a folder's Resources page: links of `items`.
    pub fn add_dropped_text_to(&self, ctx: &mut EventContext, items: &[u64], text: &str) -> bool {
        let urls = web_addresses_in(text);
        let mut any = false;
        for url in urls {
            any |= self.add_web_link_to(ctx, items, &url, "");
        }
        any
    }

    fn stored_bytes(&self) -> u64 {
        self.inner
            .session
            .model()
            .all()
            .iter()
            .filter(|r| r.is_stored())
            .map(|r| r.byte_size)
            .sum()
    }

    /// The one-time offer to keep a large single-file project as a folder instead.
    fn offer_folder_project(&self, ctx: &mut EventContext, before: u64, after: u64) {
        if self.inner.shape.get() != Some(WorkShape::Zip)
            || !sizes::crosses_zip_notice(before, after)
        {
            return;
        }
        let uid = self.inner.unique_id.get();
        let declined = folder_notice_signal(ctx);
        if declined_for(&declined.get(), &uid) {
            return;
        }
        let save_as = self.inner.save_as_folder.borrow().clone();
        MessageBox::information(tr!(resources_folder_title()))
            .text(tr!(resources_folder_text(
                size = crate::shared::units::byte_size(after)
            )))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Ok)
                    .label(tr!(resources_folder_switch())),
                MessageBoxButton::standard(StandardButton::Cancel)
                    .label(tr!(resources_folder_not_now())),
            ]))
            .default_button(StandardButton::Ok)
            .escape_button(StandardButton::Cancel)
            .show_again_checkbox(tr!(resources_folder_never()))
            .on_result(move |r, c| {
                if r.checkbox_checked {
                    let signal = folder_notice_signal(c);
                    signal.set(with_declined(&signal.get(), &uid));
                }
                if r.button == StandardButton::Ok
                    && let Some(save_as) = save_as.as_ref()
                {
                    save_as(c);
                }
            })
            .present(ctx);
    }

    // ── Opening ──────────────────────────────────────────────────────────────

    /// Open the resource whose uid a link in the text names.
    pub fn open_uid(&self, ctx: &mut EventContext, uid: uuid::Uuid) {
        match self.inner.session.model().by_uid(uid) {
            Some(row) => self.open(ctx, &row),
            None => {
                ctx.show_toast(
                    Toast::warning(tr!(resources_link_gone()))
                        .scoped_id("resources.gone", self.work_id()),
                );
            }
        }
    }

    /// Open `row`: see the module docs.
    pub fn open(&self, ctx: &mut EventContext, row: &ResourceRow) {
        if row.is_web_link() {
            crate::shared::external_link::open_external_link(&row.location, ctx);
            return;
        }
        if row.missing {
            self.say_missing(ctx, row);
            return;
        }
        // Judged by the file opened, like the open below: the viewer's "Open in another
        // app" hands this path to the system.
        if file_kind::is_viewable_picture(&name_to_judge(row))
            && let Some(path) = row.path.clone()
        {
            self.view_picture(ctx, row, path);
            return;
        }
        match open_safety::verdict(&name_to_judge(row)) {
            OpenVerdict::Open => self.open_file(ctx, row),
            OpenVerdict::Ask => self.ask_before_opening(ctx, row),
        }
    }

    fn say_missing(&self, ctx: &mut EventContext, row: &ResourceRow) {
        let mut toast =
            Toast::warning(tr!(resources_not_found_toast(name = row.file_name.clone())))
                .scoped_id("resources.missing", self.work_id());
        if row.is_linked() {
            let vm = self.clone();
            let row = row.clone();
            toast = toast.action(ToastAction::primary(tr!(resources_locate()), move |c| {
                vm.locate(c, &row);
            }));
        }
        ctx.show_toast(toast);
    }

    fn ask_before_opening(&self, ctx: &mut EventContext, row: &ResourceRow) {
        let vm = self.clone();
        let row = row.clone();
        MessageBox::warning(tr!(resources_unsafe_title(name = row.file_name.clone())))
            .text(tr!(resources_unsafe_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Ok)
                    .label(tr!(resources_show_in_folder())),
                MessageBoxButton::standard(StandardButton::Discard)
                    .label(tr!(resources_open_anyway())),
                MessageBoxButton::standard(StandardButton::Cancel),
            ]))
            // Showing it is the default: it is what lets the writer see what the file is.
            .default_button(StandardButton::Ok)
            .escape_button(StandardButton::Cancel)
            .on_result(move |r, c| match r.button {
                StandardButton::Ok => vm.reveal(c, &row),
                StandardButton::Discard => vm.open_file(c, &row),
                _ => {}
            })
            .present(ctx);
    }

    /// Where `row`'s file opens: the file itself, or a fresh copy for a single-file
    /// project's stored file. `None` when there is nothing to open.
    fn file_to_hand_over(
        &self,
        row: &ResourceRow,
    ) -> Result<Option<(PathBuf, OpenedWhere)>, String> {
        let Some(path) = row.path.clone() else {
            return Ok(None);
        };
        if row.is_linked() {
            return Ok(Some((path, OpenedWhere::InPlace)));
        }
        if self.inner.shape.get() == Some(WorkShape::Folder) {
            return Ok(Some((path, OpenedWhere::InPlace)));
        }
        let data = crate::identity::app_paths()
            .map(|p| p.data_dir().to_path_buf())
            .ok_or_else(|| tr!(resources_no_data_folder()).resolve_now())?;
        let copy = opened_files::write_open_copy(&path, &data, row.uid, &row.file_name)
            .map_err(|e| e.to_string())?;
        Ok(Some((copy, OpenedWhere::Copy)))
    }

    /// Hand `row`'s file to the application the desktop opens it with.
    pub fn open_file(&self, ctx: &mut EventContext, row: &ResourceRow) {
        match self.file_to_hand_over(row) {
            Ok(Some((path, place))) => {
                if row.is_stored() {
                    self.remember(row, &path, place);
                }
                crate::shell::process::open_in_default_app(&path.to_string_lossy());
            }
            Ok(None) => {}
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_open_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
            }
        }
    }

    /// Open the folder holding `row`'s file, with the file in view.
    pub fn reveal(&self, ctx: &mut EventContext, row: &ResourceRow) {
        if row.is_web_link() {
            return;
        }
        if row.missing {
            self.say_missing(ctx, row);
            return;
        }
        match self.file_to_hand_over(row) {
            Ok(Some((path, place))) => {
                if row.is_stored() {
                    self.remember(row, &path, place);
                }
                crate::shell::process::reveal_in_file_manager(&path.to_string_lossy());
            }
            Ok(None) => {}
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_open_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
            }
        }
    }

    fn remember(&self, row: &ResourceRow, path: &Path, place: OpenedWhere) {
        let hash = if row.content_hash.is_empty() {
            opened_files::hash_file(path)
                .map(|(_, h)| h)
                .unwrap_or_default()
        } else {
            row.content_hash.clone()
        };
        self.inner
            .session
            .opened()
            .remember(row.uid, path, place, &hash);
    }

    /// Show a picture resource in the image viewer, whose Open in another app goes
    /// through [`Self::open_file`], so its edits come back like any other.
    fn view_picture(&self, ctx: &mut EventContext, row: &ResourceRow, path: PathBuf) {
        let hash = if row.content_hash.is_empty() {
            opened_files::hash_file(&path)
                .map(|(_, h)| h)
                .unwrap_or_default()
        } else {
            row.content_hash.clone()
        };
        let entry = crate::picture_viewer::ViewerEntry {
            src: links::resource_href(row.uid),
            alt: if row.description.trim().is_empty() {
                row.title.clone()
            } else {
                row.description.clone()
            },
            item: None,
            file_name: Some(row.file_name.clone()),
            file: Some(crate::picture_viewer::ViewerFile { path, hash }),
        };
        let vm = self.clone();
        let row = row.clone();
        let opener: crate::picture_viewer::OpenElsewhereFn = Rc::new(move |c, _entry| {
            vm.open_file(c, &row);
            true
        });
        crate::picture_viewer::present_with_opener(ctx, vec![entry], 0, self.media_dir(), opener);
    }

    /// The image viewer's Open in another app for a list that mixes pictures of the text
    /// with picture resources (the Gallery): a resource goes through
    /// [`Self::open_file`], so a single-file project opens a copy and gets the edit
    /// back; anything else is left to the viewer.
    pub fn viewer_opener(&self) -> crate::picture_viewer::OpenElsewhereFn {
        let vm = self.clone();
        Rc::new(move |c, entry| {
            let Some(row) = links::resource_uid(&entry.src)
                .and_then(|uid| vm.inner.session.model().by_uid(uid))
            else {
                return false;
            };
            // Through the same check as every other open: a resource reaches the
            // viewer by its name, and this hands its path to the system.
            match open_safety::verdict(&name_to_judge(&row)) {
                OpenVerdict::Open => vm.open_file(c, &row),
                OpenVerdict::Ask => vm.ask_before_opening(c, &row),
            }
            true
        })
    }

    // ── Editing ──────────────────────────────────────────────────────────────

    /// A new title and description for `row`.
    pub fn edit_text(&self, ctx: &mut EventContext, id: u64, title: &str, description: &str) {
        if let Err(e) = self
            .inner
            .session
            .model()
            .edit_text(id, title.trim(), description.trim())
        {
            ctx.show_toast(
                Toast::error(tr!(resources_edit_failed(error = e)))
                    .scoped_id("resources.failed", self.work_id()),
            );
        }
    }

    /// Take `row` off the item in front of the writer; it stays in the project.
    pub fn detach(&self, ctx: &mut EventContext, row: &ResourceRow) {
        let Some(item) = self.inner.focus.get() else {
            return;
        };
        self.detach_from(ctx, item, row);
    }

    /// Take `row` off `item`; it stays in the project.
    pub fn detach_from(&self, ctx: &mut EventContext, item: u64, row: &ResourceRow) {
        match self.inner.session.model().detach(item, row.id) {
            Ok(()) => self.undo_toast(
                ctx,
                tr!(resources_detached_toast(title = row.title.clone())),
            ),
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_edit_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
            }
        }
    }

    /// Say what was just done, with an Undo that takes back exactly that step and
    /// nothing a later edit pushed on top of it (`shared::undo_toast`). Read straight
    /// after the call that pushed, as the stamp has to be.
    fn undo_toast(&self, ctx: &mut EventContext, message: LocalizedString) {
        let app_ctx = self.inner.app_ctx.clone();
        let seq = crate::shared::undo_toast::stamp(&app_ctx);
        if seq.is_none() {
            return;
        }
        let stack = self.inner.ids.stack_id.get();
        let session = self.inner.session.clone();
        ctx.show_toast(
            Toast::info(message)
                // One toast for the feature: a burst of removals replaces its own
                // snackbar rather than stacking a tower of them.
                .scoped_id("resources.undo", self.work_id())
                .action(crate::shared::undo_toast::undo_action(
                    app_ctx,
                    stack,
                    seq,
                    tr!(resources_undo()),
                    move |_c| session.model().reload(),
                )),
        );
    }

    /// Ask which items to attach `rows` to, in the binder tree, then attach them as one
    /// undo step: the row menu's Attach to items…, wherever a resource is listed, and
    /// the Work's Resources page.
    pub fn pick_items_and_attach(&self, ctx: &mut EventContext, rows: Vec<ResourceRow>) {
        if rows.is_empty() {
            return;
        }
        let picker = crate::widgets::DestinationPicker::multiple(
            self.inner.app_ctx.clone(),
            self.inner.ids.work_id.clone(),
        );
        let vm = self.clone();
        let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
        let on_confirm: crate::resources::unattached::ChoseItems = Rc::new(move |c, items| {
            vm.attach(c, &items, &ids);
        });
        crate::resources::unattached::present_attach(ctx, picker, on_confirm);
    }

    /// Attach the resources `ids` to every item of `items`, as one undo step. Says how
    /// many it attached, or why it could not.
    pub fn attach(&self, ctx: &mut EventContext, items: &[u64], ids: &[u64]) -> bool {
        if items.is_empty() || ids.is_empty() {
            return false;
        }
        match self.inner.session.model().attach(items, ids) {
            Ok(()) => {
                self.inner.session.model().reload();
                ctx.announce(
                    tr!(resources_attached(
                        count = ids.len() as i64,
                        items = items.len() as i64
                    ))
                    .resolve_now(),
                );
                true
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_edit_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
                false
            }
        }
    }

    /// Attach the resources `ids` to `item`, dropped on its text or its binder row, and
    /// say so with an Undo. Those it already carries are left as they are; nothing at
    /// all is said when it carried every one.
    pub fn attach_dropped(&self, ctx: &mut EventContext, item: u64, ids: &[u64]) {
        match self.attach_quietly(item, ids) {
            Ok(Some(notice)) => self.raise_attach_notice(ctx, notice),
            Ok(None) => {}
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_edit_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
            }
        }
    }

    /// [`Self::attach_dropped`] for a drop with no event context (a binder row's, which
    /// lands in the tree's data source): the attach is made now and its toast waits
    /// for the window's next event (`raise_attach_notices`).
    pub fn attach_dropped_later(&self, item: u64, ids: &[u64]) {
        if let Ok(Some(notice)) = self.attach_quietly(item, ids) {
            self.inner.session.push_attach_notice(notice);
        }
    }

    /// Raise the attach notices binder drops queued.
    pub fn raise_attach_notices(&self, ctx: &mut EventContext) {
        for notice in self.inner.session.take_attach_notices() {
            self.raise_attach_notice(ctx, notice);
        }
    }

    /// Attach what `item` does not carry yet, as one undo step. `None` when it carried
    /// every one already.
    fn attach_quietly(&self, item: u64, ids: &[u64]) -> Result<Option<AttachNotice>, String> {
        let model = self.inner.session.model();
        let carried = model.attachments(&[item]).remove(&item).unwrap_or_default();
        let mut new: Vec<u64> = Vec::new();
        for id in ids {
            if !carried.contains(id) && !new.contains(id) {
                new.push(*id);
            }
        }
        if new.is_empty() {
            return Ok(None);
        }
        model.attach(&[item], &new)?;
        let seq = crate::shared::undo_toast::stamp(&self.inner.app_ctx);
        model.reload();
        let title = match new.as_slice() {
            [one] => model.by_id(*one).map(|r| r.title).unwrap_or_default(),
            _ => String::new(),
        };
        Ok(Some(AttachNotice {
            seq,
            title,
            count: new.len(),
            item_title: model.item_title(item),
        }))
    }

    fn raise_attach_notice(&self, ctx: &mut EventContext, notice: AttachNotice) {
        let message = if notice.count == 1 {
            tr!(resources_attached_one_toast(
                title = notice.title.clone(),
                item = notice.item_title.clone()
            ))
        } else {
            tr!(resources_attached_many_toast(
                count = notice.count as i64,
                item = notice.item_title.clone()
            ))
        };
        let session = self.inner.session.clone();
        let mut toast = Toast::info(message).scoped_id("resources.undo", self.work_id());
        if notice.seq.is_some() {
            toast = toast.action(crate::shared::undo_toast::undo_action(
                self.inner.app_ctx.clone(),
                self.inner.ids.stack_id.get(),
                notice.seq,
                tr!(resources_undo()),
                move |_c| session.model().reload(),
            ));
        }
        ctx.show_toast(toast);
    }

    /// Remove `rows` from the project after asking, as one undo step: the Work's
    /// Resources page, for one row or a selection.
    pub fn remove_many(&self, ctx: &mut EventContext, rows: &[ResourceRow]) {
        match rows {
            [] => {}
            [row] => self.remove(ctx, row),
            _ => {
                let vm = self.clone();
                let rows = rows.to_vec();
                MessageBox::warning(tr!(resources_remove_many_title(count = rows.len() as i64)))
                    .text(tr!(resources_remove_text()))
                    .buttons(MessageBoxButtons::Custom(vec![
                        MessageBoxButton::standard(StandardButton::Discard)
                            .label(tr!(resources_remove_confirm())),
                        MessageBoxButton::standard(StandardButton::Cancel),
                    ]))
                    .default_button(StandardButton::Cancel)
                    .escape_button(StandardButton::Cancel)
                    .on_result(move |r, c| {
                        if r.button == StandardButton::Discard {
                            vm.remove_now(c, &rows);
                        }
                    })
                    .present(ctx);
            }
        }
    }

    /// Remove `rows` with no question, as one undo step.
    fn remove_now(&self, ctx: &mut EventContext, rows: &[ResourceRow]) {
        let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
        match self.inner.session.model().remove(&ids) {
            Ok(()) => {
                let message = match rows {
                    [row] => tr!(resources_removed_toast(title = row.title.clone())),
                    _ => tr!(resources_removed_many_toast(count = rows.len() as i64)),
                };
                self.undo_toast(ctx, message);
                for row in rows {
                    self.inner.session.opened().forget(row.uid);
                }
            }
            Err(e) => {
                ctx.show_toast(
                    Toast::error(tr!(resources_remove_failed(error = e)))
                        .scoped_id("resources.failed", self.work_id()),
                );
            }
        }
    }

    /// Remove `row` from the project, after asking.
    pub fn remove(&self, ctx: &mut EventContext, row: &ResourceRow) {
        let vm = self.clone();
        let row = row.clone();
        MessageBox::warning(tr!(resources_remove_title(title = row.title.clone())))
            .text(tr!(resources_remove_text()))
            .buttons(MessageBoxButtons::Custom(vec![
                MessageBoxButton::standard(StandardButton::Discard)
                    .label(tr!(resources_remove_confirm())),
                MessageBoxButton::standard(StandardButton::Cancel),
            ]))
            .default_button(StandardButton::Cancel)
            .escape_button(StandardButton::Cancel)
            .on_result(move |r, c| {
                if r.button == StandardButton::Discard {
                    vm.remove_now(c, std::slice::from_ref(&row));
                }
            })
            .present(ctx);
    }

    /// Ask where a missing linked file went, relink it, and relink the project's other
    /// missing files from the same folder that are in the new one under the same name.
    pub fn locate(&self, ctx: &mut EventContext, row: &ResourceRow) {
        if !row.is_linked() {
            return;
        }
        let mut req = FileDialogRequest::pick_file()
            .title(tr!(resources_locate_title(name = row.file_name.clone())));
        if let Some(dir) = Path::new(&row.location).parent()
            && dir.is_dir()
        {
            req = req.starting_dir(dir.to_path_buf());
        }
        let vm = self.clone();
        let row = row.clone();
        let _ = ctx.pick_file(req, move |res, c| {
            let FileDialogResult::File(Some(path)) = res else {
                return;
            };
            vm.relink(c, &row, &path);
        });
    }

    fn relink(&self, ctx: &mut EventContext, row: &ResourceRow, path: &Path) {
        let mut moves = vec![(row.id, path.to_path_buf())];
        moves.extend(siblings_to_relink(
            &self.inner.session.model().all(),
            row,
            path,
        ));
        let mut done = 0usize;
        for (id, to) in &moves {
            if self.inner.session.model().relink(*id, to).is_ok() {
                done += 1;
            }
        }
        if done > 1 {
            ctx.show_toast(
                Toast::success(tr!(resources_relinked_more(count = (done - 1) as i64)))
                    .scoped_id("resources.relinked", self.work_id()),
            );
        }
    }

    // ── Into the text ────────────────────────────────────────────────────────

    /// The editor the dock's Insert into the text writes into, at its caret.
    pub fn insertion_point(&self) -> Option<(EditorHandle, usize)> {
        let handle = self.editor()?;
        if !crate::shared::editor_writes::accepts_writes(&handle) {
            return None;
        }
        let (a, b) = handle.selection();
        Some((handle, a.max(b)))
    }

    /// Say there is no text to insert into.
    pub fn say_no_text(&self, ctx: &mut EventContext) {
        ctx.show_toast(
            Toast::info(tr!(resources_no_text())).scoped_id("resources.no_text", self.work_id()),
        );
    }

    /// Whether inserting `row` asks between the picture and a link to it.
    pub fn offers_picture(row: &ResourceRow) -> bool {
        !row.is_web_link() && !row.missing && file_kind::is_viewable_picture(&row.file_name)
    }

    /// Link `row` at `at` in `handle`'s text.
    pub fn insert_link(&self, handle: &EditorHandle, at: usize, row: &ResourceRow) {
        if !crate::shared::editor_writes::accepts_writes(handle) {
            return;
        }
        links::insert_links(handle, at, &[link_of(row)]);
    }

    /// Put a picture resource into `handle`'s text at `at` as a picture of the text: a
    /// copy of its bytes, stored as an `Asset` (reusing one with the same bytes), from
    /// then on independent of the resource.
    pub fn insert_picture(
        &self,
        ctx: &mut EventContext,
        handle: &EditorHandle,
        at: usize,
        row: &ResourceRow,
    ) {
        if !crate::shared::editor_writes::accepts_writes(handle) {
            return;
        }
        let Some(path) = row.path.as_deref() else {
            return;
        };
        let alt = if row.description.trim().is_empty() {
            row.title.clone()
        } else {
            row.description.clone()
        };
        let media_dir = self.media_dir();
        let result = crate::shared::images::insert_picture_file(
            &self.inner.app_ctx,
            &self.inner.ids,
            &media_dir,
            handle,
            at,
            path,
            &alt,
        );
        if let Err(e) = result {
            ctx.show_toast(
                Toast::error(lit!(e.to_string())).scoped_id("images.failed", self.work_id()),
            );
        }
    }

    // ── Changes made in other applications ───────────────────────────────────

    /// Raise the notices the project's pickup queued: one per changed file.
    pub fn raise_change_notices(&self, ctx: &mut EventContext) {
        for notice in self.inner.session.take_notices() {
            self.raise_notice(ctx, notice);
        }
    }

    fn raise_notice(&self, ctx: &mut EventContext, notice: ChangeNotice) {
        let mut toast = Toast::info(tr!(resources_changed_elsewhere(
            name = notice.file_name.clone()
        )))
        .scoped_id(
            &format!("resources.changed.{}", notice.row_id),
            self.work_id(),
        )
        .target_work(self.work_id());
        if let Some(undo) = notice.undo.clone() {
            let session = self.inner.session.clone();
            let work = self.work_id();
            let row_id = notice.row_id;
            toast = toast
                .auto_dismiss_after(UNDO_GRACE)
                .action(ToastAction::primary(tr!(resources_undo()), move |c| {
                    if let Err(e) = session.undo_change(row_id, &undo) {
                        c.show_toast(
                            Toast::error(tr!(resources_edit_failed(error = e)))
                                .scoped_id("resources.failed", work),
                        );
                    }
                }));
        }
        ctx.show_toast(toast);
    }
}

/// The large files the writer chose to link, set apart from everything stored.
fn split_large(
    paths: Vec<PathBuf>,
    large: &[(PathBuf, u64)],
    choice: LargeFileChoice,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    match choice {
        LargeFileChoice::Store => (paths, Vec::new()),
        LargeFileChoice::Link => paths
            .into_iter()
            .partition(|p| !large.iter().any(|(l, _)| l == p)),
    }
}

/// The link a resource puts into the text: its title, to its address.
fn link_of(row: &ResourceRow) -> (String, String) {
    let words = if row.title.trim().is_empty() {
        row.display_name().to_string()
    } else {
        row.title.clone()
    };
    let href = if row.is_web_link() {
        row.location.clone()
    } else {
        links::resource_href(row.uid)
    };
    (words, href)
}

/// `rows` by title, as a reader looks them up: case and accents aside.
pub fn sorted_by_title(mut rows: Vec<ResourceRow>) -> Vec<ResourceRow> {
    rows.sort_by_cached_key(|r| (title_key(&r.title), r.id));
    rows
}

pub(crate) fn title_key(title: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    title
        .nfd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect::<String>()
        .to_lowercase()
}

/// A file's name, for a sentence.
fn file_label(path: &Path) -> String {
    path.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned())
}

/// Whether `url` is an address a web link may hold: `http` or `https`, a host, and no
/// user name or password (the backend refuses those too; refusing here says why).
pub fn is_web_address(url: &str) -> bool {
    let Some((scheme, rest)) = url.trim().split_once("://") else {
        return false;
    };
    if !scheme.eq_ignore_ascii_case("http") && !scheme.eq_ignore_ascii_case("https") {
        return false;
    }
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let authority = &rest[..end];
    !authority.is_empty() && !authority.contains('@')
}

/// The web addresses in a dropped or pasted text: one per line or per word that reads as
/// one (a browser drops a page as its address, or as `text/uri-list` lines).
pub fn web_addresses_in(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        if word.starts_with('#') {
            continue;
        }
        let word = word.trim_matches(|c: char| matches!(c, '<' | '>' | '"' | '\''));
        if is_web_address(word) && !out.iter().any(|u| u == word) {
            out.push(word.to_string());
        }
    }
    out
}

/// The other missing linked files that were in `located`'s old folder and are in the
/// folder of `new_path` under the same name: the moves a Locate brings along.
pub fn siblings_to_relink(
    rows: &[ResourceRow],
    located: &ResourceRow,
    new_path: &Path,
) -> Vec<(u64, PathBuf)> {
    let (Some(old_dir), Some(new_dir)) = (Path::new(&located.location).parent(), new_path.parent())
    else {
        return Vec::new();
    };
    rows.iter()
        .filter(|r| r.id != located.id && r.is_linked() && r.missing)
        .filter_map(|r| {
            let old = Path::new(&r.location);
            if old.parent() != Some(old_dir) {
                return None;
            }
            let candidate = new_dir.join(old.file_name()?);
            candidate.is_file().then_some((r.id, candidate))
        })
        .collect()
}

/// Move the uid folders a store run wrote into `from` over to `to`. Returns the uids that
/// could not be moved.
pub fn move_stored_folders(from: &Path, to: &Path, uids: &[String]) -> Vec<String> {
    let mut failed = Vec::new();
    for uid in uids {
        let source = from.join(uid);
        let target = to.join(uid);
        let moved = std::fs::create_dir_all(to)
            .and_then(|()| std::fs::rename(&source, &target))
            .or_else(|_| {
                copy_dir(&source, &target).and_then(|()| std::fs::remove_dir_all(&source))
            });
        if moved.is_err() {
            failed.push(uid.clone());
        }
    }
    failed
}

/// Copy a uid folder (one file, by construction) across file systems.
fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            opened_files::replace_file(&entry.path(), &to.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn large_file_signal(ctx: &EventContext) -> Signal<String> {
    ctx.settings().signal::<String>(
        crate::RESOURCES_LARGE_FILE_KEY,
        crate::RESOURCES_LARGE_FILE_DEFAULT.to_string(),
    )
}

fn folder_notice_signal(ctx: &EventContext) -> Signal<String> {
    ctx.settings()
        .signal::<String>(crate::RESOURCES_FOLDER_NOTICE_DECLINED_KEY, String::new())
}

/// Whether the folder-project notice was declined for good for the project `uid`. The
/// setting is a list of unique ids separated by spaces; a project with no id yet is
/// never "declined" (an unsaved project has no identity to remember it by).
pub fn declined_for(setting: &str, uid: &str) -> bool {
    !uid.trim().is_empty() && setting.split_whitespace().any(|u| u == uid)
}

/// The setting with `uid` added to the declined list.
pub fn with_declined(setting: &str, uid: &str) -> String {
    if uid.trim().is_empty() || declined_for(setting, uid) {
        return setting.to_string();
    }
    let mut out: Vec<&str> = setting.split_whitespace().collect();
    out.push(uid.trim());
    out.join(" ")
}

/// The name the open-safety rule judges: the name of the file actually handed to the
/// system, never only the row's `file_name`. A linked file opens its `location`, and a
/// shared project can pair a harmless name ("map.pdf") with a path to a `.desktop`
/// launcher or a script; judged by the name it was given, it opened without asking.
/// A stored file's path is built from its `file_name`, so the two agree there anyway.
pub(crate) fn name_to_judge(row: &ResourceRow) -> String {
    row.path
        .as_deref()
        .and_then(Path::file_name)
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| row.file_name.clone())
}

#[cfg(test)]
mod tests;
