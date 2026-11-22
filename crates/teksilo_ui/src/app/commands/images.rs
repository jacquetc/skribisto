// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet
//
//! `editor.insert_image` — putting a picture into the manuscript.
//!
//! The command is a shell: choose a file, decide what to do about its size, and
//! hand the rest to [`crate::shared::images`], which owns every step that
//! can be tested without a window.
//!
//! ## Why a large image asks first
//!
//! A phone photograph is tens of megapixels. Stored as-is it travels in the
//! project, in every backup, and through every export — which is the writer's
//! right, and sometimes exactly what they want for a print-quality plate. But it
//! is a decision with consequences they cannot see at the moment of dropping a
//! file, so the first large image asks. The prompt's "don't ask again" writes
//! the answer to a setting, which is the LibreOffice posture rather than Word's
//! (compress by default, with a per-document switch users routinely report as
//! not working).

use frontend::AppContext;
use teksilo::prelude::*;
use teksilo::settings::SettingsExt;
use teksilo::widgets::{
    InputDialog, MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton, Toast,
};

use crate::shared::images::{self, ImageError, Landing, PendingImage, SizePolicy};
use crate::toast_scope::ToastWorkExt;

use super::CommandDeps;

/// The remembered answer, or `None` when the writer still wants to be asked.
fn stored_policy(ctx: &EventContext) -> Option<SizePolicy> {
    match policy_signal(ctx).get().as_str() {
        "keep" => Some(SizePolicy::KeepOriginal),
        "downscale" => Some(SizePolicy::Downscale),
        _ => None,
    }
}

fn policy_signal(ctx: &EventContext) -> Signal<String> {
    ctx.settings().signal::<String>(
        crate::IMAGE_SIZE_POLICY_KEY,
        crate::IMAGE_SIZE_POLICY_DEFAULT.to_string(),
    )
}

/// `editor.insert_dropped_images` — files dropped straight onto the prose.
///
/// The same pipeline `Insert image…` runs, minus the file dialog: the writer has
/// already chosen, by dropping. The size prompt still applies — a photograph
/// dragged off a desktop is exactly the case it exists for — and the caret is
/// already where the drop landed, because the editor put it there while the
/// drag was still in the air.
///
/// Anything that is not an image it can decode becomes a **resource** of the item
/// instead, linked at the drop point: a map, a recording or a letter dropped on a scene
/// is research about it, not a picture in it. That goes through the Resources dock's
/// own pipeline (`ResourcesViewModel::link_dropped_files`): the large-file question,
/// the copy into the project, and a toast whose Undo takes both the resource and its
/// link back.
///
/// Nothing is decoded on the interface thread. The drop is sorted by each file's first
/// bytes; the pictures are then read, decoded, stored and recorded away from it, and go
/// in at the drop point once that is done, if their text is still open on the same
/// project. One that turns out not to decode is linked as a resource then.
///
/// Everything goes to the editor the files landed on and that text's item, both
/// parked with the paths, and not to the editor with focus: see
/// [`crate::resources::text_drop`].
fn register_drop(ctx: &mut BuildContext, deps: &CommandDeps) {
    let format = deps.format.clone();
    let docs = deps.session.open_docs.clone();
    let ids = deps.ids.clone();
    let app_ctx = deps.app_ctx.clone();
    let resources = deps.resources.clone();

    ctx.register_action_global(Action::new("editor.insert_dropped_images").on_invoke(
        move |_intent, c: &mut EventContext| {
            let Some(dropped) = format.take_dropped_files() else {
                return;
            };
            let handle = dropped.handle;
            if dropped.paths.is_empty() || !crate::shared::editor_writes::accepts_writes(&handle) {
                return;
            }
            // Sorted by their first bytes and their length, nothing decoded here: a file
            // that opens like a picture within the size a picture may be is one, the rest
            // a resource (see `images::looks_like_picture`). The caret is where the drop
            // landed, so that is where the resources' links go, ahead of any picture
            // inserted there.
            let (pictures, others): (Vec<_>, Vec<_>) = dropped
                .paths
                .iter()
                .cloned()
                .partition(|path| images::looks_like_picture(path));
            if !others.is_empty() {
                resources.link_dropped_files(c, &handle, dropped.item, others);
            }
            if pictures.is_empty() {
                return;
            }
            if docs.media_dir().as_os_str().is_empty() {
                c.show_toast(
                    Toast::warning(tr!(image_no_project()))
                        .scoped_id("images.no_project", ids.work_id.get()),
                );
                return;
            }
            let deps = InsertDeps {
                docs: docs.clone(),
                ids: ids.clone(),
                app_ctx: app_ctx.clone(),
                landing: Landing::new(&handle, Some(handle.cursor_position()), &ids),
            };
            let resources = resources.clone();
            let item = dropped.item;
            examine_all(c, pictures, move |examined, c| {
                let (batch, failed): (Vec<_>, Vec<_>) =
                    examined.into_iter().partition(|(_, result)| result.is_ok());
                let Some(handle) = deps.landing.target(&deps.ids) else {
                    return;
                };
                // A file that opened like a picture and does not decode is a resource
                // after all, as one that never looked like one.
                if !failed.is_empty() {
                    resources.link_dropped_files(
                        c,
                        &handle,
                        item,
                        failed.into_iter().map(|(path, _)| path).collect(),
                    );
                }
                // The whole drop was examined before inserting any of it. The size
                // question is asked once and answered for the batch, and that answer
                // cannot be applied to files that have not been read yet: reading first
                // is what lets one prompt cover ten photographs instead of abandoning the
                // nine behind the first large one.
                let batch: Vec<PendingImage> = batch
                    .into_iter()
                    .filter_map(|(_, result)| result.ok())
                    .collect();
                if !batch.is_empty() {
                    insert_batch(&deps, c, batch);
                }
            });
        },
    ));
}

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    register_editing(ctx, deps);
    register_drop(ctx, deps);
    let format = deps.format.clone();
    let docs = deps.session.open_docs.clone();
    let ids = deps.ids.clone();
    let app_ctx = deps.app_ctx.clone();

    ctx.register_action_global(Action::new("editor.insert_image").on_invoke(
        move |_intent, c: &mut EventContext| {
            // Nothing to insert into: the command is reachable from the menu
            // whatever has focus, so this is the ordinary case, not an error. Nor
            // into a trashed item's text, which takes no writes.
            if format.writable_handle().is_none() {
                return;
            }
            let media_dir = docs.media_dir();
            if media_dir.as_os_str().is_empty() {
                c.show_toast(
                    Toast::warning(tr!(image_no_project()))
                        .scoped_id("images.no_project", ids.work_id.get()),
                );
                return;
            }

            let (format, docs, ids, app_ctx) =
                (format.clone(), docs.clone(), ids.clone(), app_ctx.clone());
            let req = crate::models::dialog_start_in(
                c,
                crate::models::FolderPurpose::InsertImage,
                FileDialogRequest::pick_file()
                    .title(tr!(image_choose_title()))
                    // The filter names what actually decodes, not what the format
                    // family suggests: a `.tif` offered here and refused afterwards
                    // is a worse experience than never offering it.
                    .add_filter(
                        tr!(image_filter_label()).resolve_now(),
                        &["png", "jpg", "jpeg", "webp"],
                    ),
            );
            let _ = c.pick_file(req, move |res, c| {
                let FileDialogResult::File(Some(path)) = res else {
                    return; // cancelled
                };
                crate::models::remember_dialog_file(
                    c,
                    crate::models::FolderPurpose::InsertImage,
                    &path,
                );
                // The text the commands act on now, which the picture goes into once it
                // is read, wherever the caret is in it then.
                let Some(handle) = format.writable_handle() else {
                    return;
                };
                let deps = InsertDeps {
                    docs: docs.clone(),
                    ids: ids.clone(),
                    app_ctx: app_ctx.clone(),
                    landing: Landing::new(&handle, None, &ids),
                };
                examine_all(c, vec![path], move |examined, c| {
                    for (_, result) in examined {
                        match result {
                            Err(e) => images::report(c, &e, deps.landing.work_id),
                            Ok(pending) => {
                                if deps.landing.target(&deps.ids).is_some() {
                                    insert_batch(&deps, c, vec![pending]);
                                }
                            }
                        }
                    }
                });
            });
        },
    ));
}

/// What `finish` needs, bundled so the closures that carry it stay readable.
#[derive(Clone)]
struct InsertDeps {
    docs: crate::models::OpenDocsStore,
    ids: crate::app_ids::AppIds,
    app_ctx: std::rc::Rc<AppContext>,
    /// Where the pictures go: the text a drop landed on, at the drop, or the text Insert
    /// image… acts on, at its caret.
    landing: Landing,
}

/// Examine every file of `paths` away from the interface thread, then hand each with
/// what came of it to `then`, back on it.
fn examine_all(
    ctx: &mut EventContext,
    paths: Vec<std::path::PathBuf>,
    then: impl FnOnce(Vec<(std::path::PathBuf, Result<PendingImage, ImageError>)>, &mut EventContext)
    + 'static,
) {
    // A worker that died before answering read none of them, as far as the writer can
    // be told.
    let unread = paths.clone();
    images::off_the_interface(
        ctx,
        move || {
            paths
                .into_iter()
                .map(|path| {
                    let examined = images::examine(&path);
                    (path, examined)
                })
                .collect::<Vec<_>>()
        },
        move |examined, c| {
            let examined = examined.unwrap_or_else(|| {
                unread
                    .into_iter()
                    .map(|path| {
                        let error = ImageError::Unreadable(path.display().to_string());
                        (path, Err(error))
                    })
                    .collect()
            });
            then(examined, c);
        },
    );
}

/// Insert `batch` under the size the writer keeps pictures at, asking first when that
/// is not settled and one of them is large.
fn insert_batch(deps: &InsertDeps, ctx: &mut EventContext, batch: Vec<PendingImage>) {
    match stored_policy(ctx) {
        Some(policy) => finish(deps, ctx, batch, policy),
        None if batch.iter().any(PendingImage::is_large) => ask_then_finish(deps, ctx, batch),
        None => finish(deps, ctx, batch, SizePolicy::KeepOriginal),
    }
}

/// Ask once about the first large image in a batch, then insert the whole batch
/// under the answer.
///
/// The question covers every image still to be inserted, not just the one that
/// prompted it: a prompt per file in a ten-photograph drop is an interrogation,
/// and asking about one while silently keeping the originals of the nine behind
/// it answers a question the writer did not ask. Downscaling is a no-op on an
/// image already within bounds, so a batch answer cannot shrink anything the
/// writer would have wanted left alone.
fn ask_then_finish(deps: &InsertDeps, ctx: &mut EventContext, batch: Vec<PendingImage>) {
    let Some(prompt_for) = batch.iter().find(|p| p.is_large()).cloned() else {
        // Nothing to ask about: insert as-is rather than presenting an empty
        // question.
        finish(deps, ctx, batch, SizePolicy::KeepOriginal);
        return;
    };
    let megapixels =
        (f64::from(prompt_for.width) * f64::from(prompt_for.height) / 1e6).round() as u64;
    let pending = prompt_for;
    let deps = deps.clone();
    MessageBox::question(tr!(image_large_title()))
        .text(tr!(image_large_text(
            name = pending.file_name.clone(),
            megapixels = megapixels.to_string(),
            width = pending.width.to_string(),
            height = pending.height.to_string()
        )))
        .buttons(MessageBoxButtons::Custom(vec![
            MessageBoxButton::standard(StandardButton::Ok).label(tr!(image_large_keep())),
            MessageBoxButton::standard(StandardButton::No).label(tr!(image_large_downscale())),
            MessageBoxButton::standard(StandardButton::Cancel),
        ]))
        // Keeping the writer's file is the default: it is the only choice that
        // cannot lose anything, and an accidental Enter should not silently
        // re-encode a photograph.
        .default_button(StandardButton::Ok)
        .escape_button(StandardButton::Cancel)
        .on_result(move |r, c| {
            let policy = match r.button {
                StandardButton::Ok => SizePolicy::KeepOriginal,
                StandardButton::No => SizePolicy::Downscale,
                _ => return,
            };
            if r.checkbox_checked {
                policy_signal(c).set(
                    match policy {
                        SizePolicy::KeepOriginal => "keep",
                        SizePolicy::Downscale => "downscale",
                    }
                    .to_string(),
                );
            }
            finish(&deps, c, batch.clone(), policy);
        })
        .show_again_checkbox(tr!(image_large_remember()))
        .present(ctx);
}

/// Store the images, record them, and put them in the prose, one after the other at the
/// landing.
///
/// Order matters: the bytes reach disk before anything references them, so a
/// crash between the two leaves an unreferenced file rather than prose naming a
/// picture that was never written. The stray file is harmless — content-
/// addressed, so re-inserting the same picture reuses it — but nothing reclaims
/// it yet: there is no "remove unused images" command, so an orphan costs its
/// bytes on disk until one exists.
///
/// Downscaling and writing run away from the interface thread; recording and inserting
/// back on it, once they are done ([`images::land`]).
fn finish(deps: &InsertDeps, ctx: &mut EventContext, batch: Vec<PendingImage>, policy: SizePolicy) {
    let work_id = deps.landing.work_id;
    let stored_in = deps.docs.media_dir();
    if stored_in.as_os_str().is_empty() {
        images::report(ctx, &ImageError::NoProject, work_id);
        return;
    }
    let dir = stored_in.clone();
    let deps = deps.clone();
    images::off_the_interface(
        ctx,
        move || {
            batch
                .into_iter()
                .map(|pending| images::prepare(pending, policy, &dir))
                .collect::<Vec<_>>()
        },
        move |prepared, c| {
            let Some(prepared) = prepared else {
                let error = ImageError::NotWritten(stored_in.display().to_string());
                images::report(c, &error, work_id);
                return;
            };
            let mut landing = deps.landing.clone();
            let mut landed = None;
            for image in prepared {
                let outcome = image.and_then(|image| {
                    images::land(
                        &deps.app_ctx,
                        &deps.ids,
                        &deps.docs.media_dir(),
                        &stored_in,
                        &landing,
                        &image,
                        "",
                    )
                });
                match outcome {
                    Ok(Some(handle)) => {
                        // The next one goes after this one, where the insertion left the
                        // caret.
                        landing.at = None;
                        landed = Some(handle);
                    }
                    // Nowhere to go any more, for this one or the rest.
                    Ok(None) => break,
                    Err(e) => images::report(c, &e, work_id),
                }
            }
            // Puts the caret back in the prose, exactly as the template-insert command
            // documents: for a drop, in the editor it landed on, as a text dragged from
            // one editor to another takes focus with it.
            if let Some(handle) = landed {
                handle.focus(c);
            }
        },
    );
}

/// `image.describe` and `image.resize` — editing a picture already in the prose.
///
/// Both act on the image the writer last clicked (see
/// `FormatViewModel::active_image`), because a click on an image deliberately
/// leaves the caret where it was — so the caret cannot say which picture is
/// meant, and a document may hold the same one three times.
///
/// Each rewrites the reference in place: select the image's single character,
/// then insert the new Djot over it. That keeps the edit on the document's own
/// undo stack, at the granularity a writer expects — one Ctrl+Z puts the old
/// description back.
fn register_editing(ctx: &mut BuildContext, deps: &CommandDeps) {
    register_viewing(ctx, deps);
    let describe_format = deps.format.clone();
    ctx.register_action_global(Action::new("image.describe").on_invoke(
        move |_intent, c: &mut EventContext| {
            let Some((handle, image)) = resolve_active(&describe_format) else {
                return;
            };
            let format = describe_format.clone();
            let offset = image_offset(&describe_format).unwrap_or(0);
            InputDialog::new(tr!(image_describe_title()))
                .prompt(tr!(image_describe_explain()))
                .placeholder(tr!(image_describe_placeholder()))
                .default_text(image.alt.clone())
                .on_result(move |result, c| {
                    if let Some(alt) = result {
                        handle.select_range(offset, offset + 1);
                        handle.insert_djot(&image.djot(alt.trim(), image.size));
                    }
                    format.refocus(c);
                })
                .present(c);
        },
    ));

    let resize_format = deps.format.clone();
    ctx.register_action_global(Action::new("image.resize").on_invoke(
        move |_intent, c: &mut EventContext| {
            let Some((handle, image)) = resolve_active(&resize_format) else {
                return;
            };
            let format = resize_format.clone();
            let offset = image_offset(&resize_format).unwrap_or(0);
            // The layout draws a picture no wider than its line, so "the size it is shown
            // at now" is the stored size capped to the editor's width, not the stored size.
            let measure = resize_format.target_width();
            // A percentage of the size it is *now*, not of the file's own
            // dimensions: the writer is looking at the picture on the page, and
            // "half of what I can see" is the thing they can judge. 100 is
            // therefore "leave it", and the reset is a separate, explicit act —
            // see the menu's "Original size".
            InputDialog::new(tr!(image_resize_title()))
                .prompt(tr!(image_resize_explain()))
                .default_text("100".to_string())
                .validate(|text| match text.trim().parse::<u32>() {
                    // Not an error: the writer is still typing.
                    Err(_) if text.trim().is_empty() => Err(None),
                    Ok(p) if (1..=1000).contains(&p) => Ok(()),
                    _ => Err(Some(tr!(image_resize_invalid()))),
                })
                .on_result(move |result, c| {
                    if let Some(text) = result
                        && let Ok(percent) = text.trim().parse::<u32>()
                        && let Some(size) = image.scaled_within(percent, measure)
                    {
                        handle.select_range(offset, offset + 1);
                        handle.insert_djot(&image.djot(&image.alt, Some(size)));
                    }
                    format.refocus(c);
                })
                .present(c);
        },
    ));

    let reset_format = deps.format.clone();
    ctx.register_action_global(Action::new("image.reset_size").on_invoke(
        move |_intent, c: &mut EventContext| {
            let Some((handle, image)) = resolve_active(&reset_format) else {
                return;
            };
            let Some(offset) = image_offset(&reset_format) else {
                return;
            };
            // The file's own pixel dimensions, read back from the resource the
            // editor is already painting — so this restores the size the picture
            // actually is, not a size remembered from when it was inserted.
            let natural = handle.image_resource_size(&image.src).or(image.size);
            handle.select_range(offset, offset + 1);
            handle.insert_djot(&image.djot(&image.alt, natural));
            reset_format.refocus(c);
        },
    ));
}

/// `image.view` and `image.open_externally`: looking at the picture last clicked.
///
/// Neither writes, so both act on a trashed item's text as on any other: a picture in the
/// trash is still the writer's to look at. The viewer gets every picture of that text, in
/// document order, and starts on the clicked one, so the arrow keys walk the text's
/// pictures the way a reader would.
fn register_viewing(ctx: &mut BuildContext, deps: &CommandDeps) {
    let view_format = deps.format.clone();
    let view_docs = deps.session.open_docs.clone();
    let view_ctx = deps.app_ctx.clone();
    ctx.register_action_global(Action::new("image.view").on_invoke(
        move |_intent, c: &mut EventContext| {
            let Some((handle, image)) = resolve_clicked(&view_format) else {
                return;
            };
            let pictures = images::parse_images(&handle.to_djot());
            let entries = crate::picture_viewer::with_file_names(
                &view_ctx,
                crate::picture_viewer::ViewerEntry::from_images(&pictures, None),
            );
            crate::picture_viewer::present(c, entries, image.index, view_docs.media_dir());
        },
    ));

    let open_format = deps.format.clone();
    let open_docs = deps.session.open_docs.clone();
    ctx.register_action_global(Action::new("image.open_externally").on_invoke(
        move |_intent, _c: &mut EventContext| {
            let Some((_, image)) = resolve_clicked(&open_format) else {
                return;
            };
            // Only a project picture's own file, never a path the prose names: nothing
            // typed into a document can make this launch an arbitrary file.
            let Some(path) =
                crate::shared::picture_cache::asset_path(&open_docs.media_dir(), &image.src)
                    .filter(|p| p.exists())
            else {
                return;
            };
            crate::shell::process::open_in_default_app(&path.to_string_lossy());
        },
    ));
}

/// The editor holding the clicked image and the image, for a command that only reads:
/// unlike [`resolve_active`], a trashed item's text qualifies.
fn resolve_clicked(
    format: &crate::format::FormatViewModel,
) -> Option<(teksilo::widgets::rich_text::EditorHandle, images::ImageRef)> {
    let (offset, _src) = format.active_image().get()?;
    let handle = format.handle_for_commands()?;
    let image = images::image_at(&handle.to_plain_text(), offset, &handle.to_djot())?;
    Some((handle, image))
}

/// The clicked image's character offset, if there is one.
fn image_offset(format: &crate::format::FormatViewModel) -> Option<usize> {
    format.active_image().get().map(|(offset, _)| offset)
}

/// The editor holding the clicked image, and the image as the prose stores it.
///
/// `None` when nothing is in hand, when the editor has gone or takes no writes
/// (a trashed item's text), or when the recorded offset no longer holds an image — the prose may have been edited
/// since the click, and rewriting whatever is there now would be worse than
/// doing nothing.
fn resolve_active(
    format: &crate::format::FormatViewModel,
) -> Option<(teksilo::widgets::rich_text::EditorHandle, images::ImageRef)> {
    let (offset, _src) = format.active_image().get()?;
    let handle = format.writable_handle()?;
    let image = images::image_at(&handle.to_plain_text(), offset, &handle.to_djot())?;
    Some((handle, image))
}
