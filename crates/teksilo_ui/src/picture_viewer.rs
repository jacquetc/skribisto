// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The image viewer: one picture of a list at a time, as large as the window allows, with
//! zoom, pan, and the previous and next pictures a key away.
//!
//! Opened from a picture in the prose (a double click, or View the image in its menu),
//! where the list is that text's pictures, and from the Gallery, where the list is the
//! tiles the Gallery shows, in its order and under its filters. Either way the viewer only
//! *reads*: it never edits the prose, so it opens on a trashed item's text as on any other.
//!
//! The picture is decoded once, away from the interface thread
//! ([`crate::shared::picture_cache`]), no larger than [`VIEWER_EDGE`] on its longest side,
//! which is past any screen and within what a graphics card takes as one texture. Zoom
//! then changes only where those pixels are drawn ([`stage`](mod@stage)), and the rules for how
//! far it goes are plain numbers in [`zoom`].
//!
//! What the viewer cannot do: put the picture on the clipboard. Teksilo's clipboard carries
//! text and rich text, not pixels, so "Copy image" waits on the framework.

mod stage;
pub mod zoom;

use std::path::{Path, PathBuf};
use std::rc::Rc;

use teksilo::core::BindingLevel;
use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{
    Center, Divider, Expand, HStack, IconButton, ModalContainer, Padding, Shrinkable, Spacer,
    TextWidget, Toast, VStack,
};

use crate::intents::AppIntent;
use crate::shared::images::ImageRef;
use crate::shared::picture_cache::{self, PictureCache, PictureKey, PictureState};
use crate::toast_scope::ToastWorkExt;
use stage::{StageState, stage};

/// The longest side a picture is decoded to for the viewer.
pub const VIEWER_EDGE: u32 = 8192;

/// The note an image belongs to, so the viewer can name it and open it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerItem {
    pub item_id: u64,
    pub title: String,
}

/// One picture the viewer can show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerEntry {
    /// The reference as the prose stores it, `assets/<hash>.<ext>` for a project picture.
    pub src: String,
    /// The writer's description of it, possibly empty.
    pub alt: String,
    pub item: Option<ViewerItem>,
    /// The name the picture had when it came into the project, for Save a copy. `None` when
    /// the project does not record one.
    pub file_name: Option<String>,
    /// A picture that is not one of the project's pictures of the text: a resource's file,
    /// named by its path and its content hash rather than by `src`. `None` for every
    /// picture of a text, which `src` names.
    pub file: Option<ViewerFile>,
}

/// The file a picture entry shows when it is not a picture of the text (see
/// [`ViewerEntry::file`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewerFile {
    pub path: PathBuf,
    /// The blake3 of its bytes, hex, as the project records it: what the thumbnail cache
    /// names it by. Empty for a file the project never hashed (a linked file), which the
    /// cache keys by its place instead.
    pub hash: String,
}

/// Opens the current picture in another application, when the opener is not simply
/// "hand the file to the desktop": a resource's stored file goes through the Resources
/// feature, which opens a single-file project's as a copy and brings an edit back.
///
/// Handed the current entry, and answers whether it opened it: a list that mixes
/// pictures of the text with resources (the Gallery) opens only the resources itself,
/// and leaves the rest to the desktop hand-over.
pub type OpenElsewhereFn = Rc<dyn Fn(&mut EventContext, &ViewerEntry) -> bool>;

impl ViewerEntry {
    /// The entries for every picture of one text, in document order.
    pub fn from_images(images: &[ImageRef], item: Option<ViewerItem>) -> Vec<Self> {
        images
            .iter()
            .map(|image| Self {
                src: image.src.clone(),
                alt: image.alt.clone(),
                item: item.clone(),
                file_name: None,
                file: None,
            })
            .collect()
    }

    /// What the viewer's header calls this picture: its description, else its note's
    /// title, else nothing at all (the header then reads only the position).
    pub fn caption(&self) -> String {
        let alt = self.alt.trim();
        if !alt.is_empty() {
            return alt.to_string();
        }
        self.item
            .as_ref()
            .map(|i| i.title.trim().to_string())
            .unwrap_or_default()
    }

    /// The name Save a copy proposes: the name the picture came in with, else the stored
    /// file's own.
    pub fn suggested_file_name(&self) -> String {
        self.file_name
            .clone()
            .filter(|n| !n.trim().is_empty())
            .or_else(|| {
                Path::new(&self.src)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "image.png".to_string())
    }
}

/// `entries`, each given the name its picture came into the project with, read from the
/// project's `Asset` rows by content hash. An entry whose picture has no row keeps `None`.
pub fn with_file_names(
    app_ctx: &frontend::AppContext,
    mut entries: Vec<ViewerEntry>,
) -> Vec<ViewerEntry> {
    let names: std::collections::HashMap<String, String> =
        frontend::commands::asset_commands::get_all_asset(app_ctx)
            .unwrap_or_default()
            .into_iter()
            .map(|asset| (asset.content_hash, asset.file_name))
            .collect();
    for entry in &mut entries {
        if let Some(hash) = picture_cache::asset_hash(&entry.src) {
            entry.file_name = names.get(hash).cloned();
        }
    }
    entries
}

/// The viewer's state: which picture, of which list, from where.
#[derive(Clone)]
pub struct PictureViewerViewModel {
    entries: Rc<Vec<ViewerEntry>>,
    index: Signal<usize>,
    media_dir: PathBuf,
    stage: Rc<StageState>,
    /// Replaces the desktop hand-over of Open in another app; see [`OpenElsewhereFn`].
    open_elsewhere: Option<OpenElsewhereFn>,
}

impl std::fmt::Debug for PictureViewerViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PictureViewerViewModel")
            .field("count", &self.entries.len())
            .field("index", &self.index.get())
            .finish()
    }
}

impl PictureViewerViewModel {
    /// The viewer over `entries`, on the one at `start` (clamped into the list).
    pub fn new(entries: Vec<ViewerEntry>, start: usize, media_dir: PathBuf) -> Self {
        let start = start.min(entries.len().saturating_sub(1));
        Self {
            entries: Rc::new(entries),
            index: Signal::new(start),
            media_dir,
            stage: StageState::new(),
            open_elsewhere: None,
        }
    }

    /// The same viewer, with Open in another app going through `opener`.
    pub fn with_opener(mut self, opener: OpenElsewhereFn) -> Self {
        self.open_elsewhere = Some(opener);
        self
    }

    pub fn count(&self) -> usize {
        self.entries.len()
    }

    pub fn index(&self) -> usize {
        self.index.get()
    }

    pub fn current(&self) -> Option<&ViewerEntry> {
        self.entries.get(self.index.get())
    }

    /// The previous picture, staying on the first.
    pub fn previous(&self) {
        let i = self.index.get();
        if i > 0 {
            self.index.set(i - 1);
        }
    }

    /// The next picture, staying on the last.
    pub fn next(&self) {
        let i = self.index.get();
        if i + 1 < self.entries.len() {
            self.index.set(i + 1);
        }
    }

    pub fn first(&self) {
        self.index.set_if_changed(0);
    }

    pub fn last(&self) {
        self.index
            .set_if_changed(self.entries.len().saturating_sub(1));
    }

    /// The file the current picture is stored in, `None` for a reference outside the
    /// project's pictures (a hand-typed path, an address on the web).
    pub fn current_path(&self) -> Option<PathBuf> {
        let entry = self.current()?;
        match &entry.file {
            Some(file) => Some(file.path.clone()),
            None => picture_cache::asset_path(&self.media_dir, &entry.src),
        }
    }

    /// The cache key of the current picture at `edge`.
    pub(crate) fn current_key(&self, edge: u32) -> Option<PictureKey> {
        let entry = self.current()?;
        match &entry.file {
            // Never the recorded hash as it is: a linked file has none, and a crafted
            // project's need not be one (see `picture_cache::file_key_hash`).
            Some(file) => picture_cache::file_key_hash(&file.path, &file.hash)
                .map(|hash| PictureKey { hash, edge }),
            None => PictureKey::for_src(&entry.src, edge),
        }
    }

    /// Open the current picture's file in the application the desktop opens such files
    /// with. Only ever a project picture: a reference the writer typed is never handed to
    /// the desktop, so nothing in a document can make the viewer launch an arbitrary path.
    pub fn open_externally(&self, ctx: &mut EventContext) {
        if let Some(open) = &self.open_elsewhere
            && let Some(entry) = self.current().cloned()
            && open(ctx, &entry)
        {
            return;
        }
        if let Some(path) = self.current_path().filter(|p| p.exists()) {
            crate::shell::process::open_in_default_app(&path.to_string_lossy());
        }
    }

    /// Ask where to save a copy of the current picture's file, and write it there.
    pub fn save_copy(&self, ctx: &mut EventContext) {
        let (Some(source), Some(entry)) = (self.current_path(), self.current().cloned()) else {
            return;
        };
        let req = crate::models::dialog_start_in(
            ctx,
            crate::models::FolderPurpose::Export,
            FileDialogRequest::save_file()
                .title(tr!(picture_viewer_save_copy_title()))
                .default_file_name(entry.suggested_file_name()),
        );
        let _ = ctx.save_file(req, move |res, ectx| {
            let FileDialogResult::Saved(Some(target)) = res else {
                return;
            };
            crate::models::remember_dialog_file(
                ectx,
                crate::models::FolderPurpose::Export,
                &target,
            );
            let toast = match std::fs::copy(&source, &target) {
                Ok(_) => Toast::success(tr!(picture_viewer_saved())),
                Err(e) => Toast::error(tr!(picture_viewer_save_failed(error = e.to_string()))),
            };
            ectx.show_toast(toast.scoped_id("picture_viewer.save", None));
        });
    }

    /// Close the viewer and open the note the current picture belongs to.
    pub fn open_note(&self, ctx: &mut EventContext) {
        let Some(item) = self.current().and_then(|e| e.item.clone()) else {
            return;
        };
        ctx.dismiss_modal();
        ctx.send_intent(AppIntent::OpenItem {
            item_id: item.item_id,
            title: item.title,
        });
    }

    /// The keys the viewer answers wherever focus is inside it.
    fn key(&self, event: &WidgetEvent) -> EventResponse {
        let WidgetEvent::KeyDown { key, modifiers, .. } = event else {
            return EventResponse::Ignored;
        };
        if modifiers.alt() {
            return EventResponse::Ignored;
        }
        let command = modifiers.command();
        match key {
            Key::ArrowLeft | Key::PageUp if !command => self.previous(),
            Key::ArrowRight | Key::PageDown if !command => self.next(),
            Key::Home if !command => self.first(),
            Key::End if !command => self.last(),
            Key::Character('+') | Key::Character('=') => self.stage.zoom_in(),
            Key::Character('-') => self.stage.zoom_out(),
            Key::Character('0') => self.stage.fit(),
            Key::Character('1') => self.stage.actual_size(),
            _ => return EventResponse::Ignored,
        }
        EventResponse::Handled
    }
}

/// Show the viewer over `entries`, on `start`.
pub fn present(
    ctx: &mut EventContext,
    entries: Vec<ViewerEntry>,
    start: usize,
    media_dir: PathBuf,
) {
    if entries.is_empty() {
        return;
    }
    present_model(ctx, PictureViewerViewModel::new(entries, start, media_dir));
}

/// [`present`], with Open in another app going through `opener` (see [`OpenElsewhereFn`]).
pub fn present_with_opener(
    ctx: &mut EventContext,
    entries: Vec<ViewerEntry>,
    start: usize,
    media_dir: PathBuf,
    opener: OpenElsewhereFn,
) {
    if entries.is_empty() {
        return;
    }
    present_model(
        ctx,
        PictureViewerViewModel::new(entries, start, media_dir).with_opener(opener),
    );
}

fn present_model(ctx: &mut EventContext, vm: PictureViewerViewModel) {
    ctx.present_modal(
        // The app's dialog frame (`ModalContainer`): its surface, its `Role::Dialog` node
        // and the name a screen reader announces. No padding: the picture uses the whole
        // surface, and the toolbar pads itself.
        ModalRequest::deferred(move |t| {
            t.add(
                ModalContainer::new(ViewerPanel {
                    vm: vm.clone(),
                    root: None,
                })
                .title(tr!(picture_viewer_title()))
                .padding(0.0),
            )
        })
        .presentation(ModalPresentation::InTree)
        .title(tr!(picture_viewer_title()).resolve_now())
        // Escape only: a drag that pans past the picture's edge ends outside it, and an
        // outside click closing the viewer would turn every such drag into a dismissal.
        .close_behavior(ModalCloseBehavior::EscapeKey)
        .size(1200, 860),
    );
}

/// The viewer's content: the header, the stage, rebuilt for each picture.
struct ViewerPanel {
    vm: PictureViewerViewModel,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for ViewerPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ViewerPanel").field("vm", &self.vm).finish()
    }
}

impl Widget for ViewerPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = ctx.binding_registry();
        self.vm.index.bind_to(sid, reg, BindingLevel::Rebuild);

        let vm = self.vm.clone();
        let entry = vm.current().cloned();
        let position = tr!(picture_viewer_position(
            index = (vm.index() + 1) as i64,
            count = vm.count() as i64
        ));
        let caption = entry.as_ref().map(ViewerEntry::caption).unwrap_or_default();
        let has_note = entry.as_ref().is_some_and(|e| e.item.is_some());
        let has_file = vm.current_path().is_some_and(|p| p.exists());
        let at_first = vm.index() == 0;
        let at_last = vm.index() + 1 >= vm.count();

        // The picture itself, or why there is none.
        let body: Box<dyn Widget> = match (entry.as_ref(), vm.current_path()) {
            (Some(entry), Some(path)) => {
                let rt = ctx.app_state::<AsyncRuntimeHandle>().cloned();
                let state = vm
                    .current_key(VIEWER_EDGE)
                    .map(|key| PictureCache::shared().request(rt.as_ref(), path, key));
                match state {
                    Some(state) => {
                        state.bind_to(sid, reg, BindingLevel::Rebuild);
                        match state.get() {
                            PictureState::Ready(decoded) => {
                                self.vm.stage.show(
                                    &format!("{}:{}", vm.index(), entry.src),
                                    decoded.width,
                                    decoded.height,
                                );
                                Box::new(stage(self.vm.stage.clone(), decoded, entry.caption()))
                            }
                            PictureState::Loading | PictureState::Idle => {
                                Box::new(message(tr!(picture_viewer_loading())))
                            }
                            PictureState::Failed => Box::new(message(tr!(picture_viewer_failed()))),
                        }
                    }
                    None => Box::new(message(tr!(picture_viewer_failed()))),
                }
            }
            _ => Box::new(message(tr!(picture_viewer_failed()))),
        };

        let prev_vm = vm.clone();
        let next_vm = vm.clone();
        let out_vm = vm.clone();
        let fit_vm = vm.clone();
        let actual_vm = vm.clone();
        let in_vm = vm.clone();
        let note_vm = vm.clone();
        let open_vm = vm.clone();
        let save_vm = vm.clone();
        let key_vm = vm.clone();
        let preview_vm = vm.clone();

        // Icon buttons, as in the app's other toolbars; each tooltip is also the button's
        // name for a screen reader.
        let icon = |glyph: teksilo::widgets::IconWidget, tip: teksilo::i18n::LocalizedString| {
            IconButton::new(glyph).toolbar().tooltip(tip)
        };
        let mut toolbar = HStack::new()
            .spacing(2.0)
            .child(
                icon(
                    crate::icons::viewer::previous(),
                    tr!(picture_viewer_previous()),
                )
                .enabled(!at_first)
                .on_activate_fn(move |_| prev_vm.previous()),
            )
            .child(
                icon(crate::icons::viewer::next(), tr!(picture_viewer_next()))
                    .enabled(!at_last)
                    .on_activate_fn(move |_| next_vm.next()),
            )
            .child(
                Padding::symmetric(0.0, 6.0).child(
                    TextWidget::new(position)
                        .style(TextStyleRole::Small)
                        .color(TextRole::Secondary)
                        .single_line(),
                ),
            )
            // Its own height, so the row centres it beside the position; it gives way
            // before the buttons do in a narrow window.
            .child(
                Shrinkable::new().min_width(40.0).child(
                    TextWidget::new(lit!(caption))
                        .style(TextStyleRole::BodyBold)
                        .single_line(),
                ),
            )
            .child(Spacer::new())
            .child(
                icon(
                    crate::icons::viewer::zoom_out(),
                    tr!(picture_viewer_zoom_out()),
                )
                .on_activate_fn(move |_| out_vm.stage.zoom_out()),
            )
            .child(
                icon(crate::icons::viewer::fit(), tr!(picture_viewer_fit()))
                    .on_activate_fn(move |_| fit_vm.stage.fit()),
            )
            .child(
                icon(
                    crate::icons::viewer::actual_size(),
                    tr!(picture_viewer_actual_size()),
                )
                .on_activate_fn(move |_| actual_vm.stage.actual_size()),
            )
            .child(
                icon(
                    crate::icons::viewer::zoom_in(),
                    tr!(picture_viewer_zoom_in()),
                )
                .on_activate_fn(move |_| in_vm.stage.zoom_in()),
            );
        if has_note || has_file {
            toolbar = toolbar.child(Padding::symmetric(4.0, 6.0).child(Divider::vertical()));
        }
        if has_note {
            toolbar = toolbar.child(
                icon(
                    crate::icons::viewer::open_note(),
                    tr!(picture_viewer_open_note()),
                )
                .on_activate_fn(move |c| note_vm.open_note(c)),
            );
        }
        if has_file {
            toolbar = toolbar
                .child(
                    icon(
                        crate::icons::viewer::open_external(),
                        tr!(picture_viewer_open_externally()),
                    )
                    .on_activate_fn(move |c| open_vm.open_externally(c)),
                )
                .child(
                    icon(
                        crate::icons::viewer::save_copy(),
                        tr!(picture_viewer_save_copy()),
                    )
                    .on_activate_fn(move |c| save_vm.save_copy(c)),
                );
        }

        // Last, at the right end, where a dialog's close button is everywhere else. Escape
        // closes too; a click outside does not (see `present`).
        let toolbar = toolbar
            .child(Padding::symmetric(4.0, 6.0).child(Divider::vertical()))
            .child(
                icon(crate::icons::viewer::close(), tr!(picture_viewer_close()))
                    .on_activate_fn(|c| c.dismiss_modal()),
            );

        // The overlay takes its content's size, so this panel says how big it is (see
        // `layout_response`); the frame around it is the `ModalContainer` `present` wraps
        // it in.
        let id = teksu!(ctx => VStack {
            spacing: 0.0
            focusable: true
            on_key: move |e, _c| key_vm.key(e)
            on_key_preview: move |e, _c| preview_vm.key(e)
            Padding::symmetric(6.0, 8.0) {
                child: toolbar
            }
            Divider::horizontal()
            Expand::new() {
                child: body
            }
        });
        self.root = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        Size::new(
            viewer_extent(proposal.width, 1100.0, 1600.0),
            viewer_extent(proposal.height, 780.0, 1200.0),
        )
        .into()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [teksilo::core::widget::WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // Fill the box `layout_response` claimed: the stage takes whatever the header
        // leaves, and a stage measured at its own size would have none.
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

/// The viewer's size on one axis: most of the window when the overlay says how large the
/// window is, leaving a margin to see that a dialog is open, else `fallback`.
fn viewer_extent(offered: Option<f32>, fallback: f32, max: f32) -> f32 {
    match offered {
        Some(room) if room.is_finite() && room > 0.0 => {
            (room - 48.0).clamp(240.0_f32.min(room), max)
        }
        _ => fallback,
    }
}

fn message(text: teksilo::i18n::LocalizedString) -> Center {
    Center::new().child(
        TextWidget::new(text)
            .style(TextStyleRole::Body)
            .color(TextRole::Secondary),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(src: &str, alt: &str, item: Option<(u64, &str)>) -> ViewerEntry {
        ViewerEntry {
            src: src.to_string(),
            alt: alt.to_string(),
            item: item.map(|(item_id, title)| ViewerItem {
                item_id,
                title: title.to_string(),
            }),
            file_name: None,
            file: None,
        }
    }

    #[test]
    fn the_viewer_takes_most_of_the_window_and_never_more() {
        assert_eq!(viewer_extent(Some(1200.0), 1100.0, 1600.0), 1152.0);
        assert_eq!(viewer_extent(Some(4000.0), 1100.0, 1600.0), 1600.0);
        assert_eq!(viewer_extent(Some(200.0), 1100.0, 1600.0), 200.0);
        assert_eq!(viewer_extent(None, 1100.0, 1600.0), 1100.0);
        assert_eq!(viewer_extent(Some(f32::INFINITY), 780.0, 1200.0), 780.0);
    }

    #[test]
    fn navigation_stays_within_the_list() {
        let vm = PictureViewerViewModel::new(
            vec![
                entry("a", "", None),
                entry("b", "", None),
                entry("c", "", None),
            ],
            9,
            PathBuf::new(),
        );
        assert_eq!(vm.index(), 2, "a start past the end lands on the last");
        vm.next();
        assert_eq!(vm.index(), 2);
        vm.previous();
        vm.previous();
        vm.previous();
        assert_eq!(vm.index(), 0);
        vm.last();
        assert_eq!(vm.index(), 2);
        vm.first();
        assert_eq!(vm.index(), 0);
    }

    #[test]
    fn the_keys_browse_and_zoom() {
        let vm = PictureViewerViewModel::new(
            vec![entry("a", "", None), entry("b", "", None)],
            0,
            PathBuf::new(),
        );
        let press = |key| WidgetEvent::KeyDown {
            key,
            modifiers: Modifiers::NONE,
            text: None,
        };
        assert_eq!(vm.key(&press(Key::ArrowRight)), EventResponse::Handled);
        assert_eq!(vm.index(), 1);
        assert_eq!(vm.key(&press(Key::ArrowLeft)), EventResponse::Handled);
        assert_eq!(vm.index(), 0);
        vm.stage.show("a", 4000, 3000);
        assert_eq!(vm.key(&press(Key::Character('1'))), EventResponse::Handled);
        assert_eq!(vm.stage.view().zoom, zoom::Zoom::Scale(1.0));
        assert_eq!(vm.key(&press(Key::Character('0'))), EventResponse::Handled);
        assert_eq!(vm.stage.view().zoom, zoom::Zoom::Fit);
        assert_eq!(vm.key(&press(Key::A)), EventResponse::Ignored);
    }

    #[test]
    fn a_caption_and_a_file_name_fall_back_sensibly() {
        let hash = "0".repeat(64);
        let described = entry(
            &format!("assets/{hash}.png"),
            " a gull ",
            Some((1, "Notes")),
        );
        assert_eq!(described.caption(), "a gull");
        let undescribed = entry(&format!("assets/{hash}.png"), "", Some((1, "Shore notes")));
        assert_eq!(undescribed.caption(), "Shore notes");
        assert_eq!(undescribed.suggested_file_name(), format!("{hash}.png"));
        let named = ViewerEntry {
            file_name: Some("gull.jpg".into()),
            ..undescribed
        };
        assert_eq!(named.suggested_file_name(), "gull.jpg");
    }

    /// A resource's file opens in the viewer whether or not the project recorded its
    /// hash, and a recorded hash that is not a content hash never becomes the key.
    #[test]
    fn a_resource_file_is_keyed_with_or_without_a_recorded_hash() {
        let tmp = tempfile::tempdir().unwrap_or_else(|e| panic!("tempdir: {e}"));
        let path = tmp.path().join("linked.png");
        std::fs::write(&path, b"bytes").unwrap_or_else(|e| panic!("write: {e}"));
        let place = picture_cache::file_key_hash(&path, "");
        let recorded = "b".repeat(64);
        let file = |hash: &str| ViewerEntry {
            file: Some(ViewerFile {
                path: path.clone(),
                hash: hash.to_string(),
            }),
            ..entry("resource:x", "", None)
        };
        let vm = PictureViewerViewModel::new(
            vec![file(&recorded), file(""), file("../../escape")],
            0,
            PathBuf::new(),
        );
        let hash_now = |vm: &PictureViewerViewModel| vm.current_key(64).map(|k| k.hash);
        assert_eq!(hash_now(&vm), Some(recorded));
        vm.next();
        assert!(place.is_some());
        assert_eq!(hash_now(&vm), place, "a linked file is keyed by its place");
        vm.next();
        assert_eq!(hash_now(&vm), place);
    }

    #[test]
    fn only_a_project_picture_has_a_file_to_open() {
        let hash = "a".repeat(64);
        let vm = PictureViewerViewModel::new(
            vec![
                entry(&format!("assets/{hash}.png"), "", None),
                entry("/etc/passwd", "", None),
                entry("https://example.org/x.png", "", None),
            ],
            0,
            PathBuf::from("/media"),
        );
        assert_eq!(
            vm.current_path(),
            Some(PathBuf::from(format!("/media/{hash}.png")))
        );
        vm.next();
        assert_eq!(vm.current_path(), None);
        vm.next();
        assert_eq!(vm.current_path(), None);
    }
}
