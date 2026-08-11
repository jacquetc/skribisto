// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet
//
//! `editor.insert_image` — putting a picture into the manuscript.
//!
//! The command is a shell: choose a file, decide what to do about its size, and
//! hand the rest to [`crate::view_models::images`], which owns every step that
//! can be tested without a window.
//!
//! ## Why a large image is worth a question
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

use crate::toast_scope::ToastWorkExt;
use crate::view_models::images::{self, PendingImage, SizePolicy};

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
    ctx.settings()
        .signal::<String>(crate::IMAGE_SIZE_POLICY_KEY, "ask".to_string())
}

/// `work.set_cover` and `work.clear_cover` — the book's front.
///
/// A cover is not inline content: nothing in the prose refers to it, so it is
/// chosen from the book rather than typed into a scene. It rides the same
/// storage path as an inserted picture (content-addressed file in the media
/// directory, `Asset` row) and differs only in carrying `is_cover`.
fn register_cover(ctx: &mut BuildContext, deps: &CommandDeps) {
    let docs = deps.session.open_docs.clone();
    let ids = deps.ids.clone();
    let app_ctx = deps.app_ctx.clone();

    let (d, i, a) = (docs.clone(), ids.clone(), app_ctx.clone());
    ctx.register_action_global(Action::new("work.set_cover").on_invoke(
        move |_intent, c: &mut EventContext| {
            let media_dir = d.media_dir();
            if media_dir.as_os_str().is_empty() {
                c.show_toast(
                    Toast::warning(tr!(image_no_project()))
                        .scoped_id("images.no_project", i.work_id.get()),
                );
                return;
            }
            let (d, i, a) = (d.clone(), i.clone(), a.clone());
            let req = crate::models::dialog_start_in(
                c,
                crate::models::FolderPurpose::InsertImage,
                FileDialogRequest::pick_file()
                    .title(tr!(cover_choose_title()))
                    .add_filter(
                        tr!(image_filter_label()).resolve_now(),
                        &["png", "jpg", "jpeg", "webp"],
                    ),
            );
            let _ = c.pick_file(req, move |res, c| {
                let FileDialogResult::File(Some(path)) = res else {
                    return;
                };
                crate::models::remember_dialog_file(
                    c,
                    crate::models::FolderPurpose::InsertImage,
                    &path,
                );
                // A cover is stored as the writer's file, never downscaled: it is
                // the one picture in the project whose resolution is the whole
                // point, and an ereader renders it full-screen.
                let image = match images::examine(&path) {
                    Ok(p) => p,
                    Err(e) => {
                        c.show_toast(
                            Toast::error(lit!(e.to_string()))
                                .scoped_id("images.failed", i.work_id.get()),
                        );
                        return;
                    }
                };
                if let Err(e) = images::store(&image, &d.media_dir()) {
                    c.show_toast(
                        Toast::error(lit!(e.to_string()))
                            .scoped_id("images.failed", i.work_id.get()),
                    );
                    return;
                }
                match images::set_cover(&a, i.stack_id.get(), i.work_id.get(), Some(&image)) {
                    Ok(()) => c.show_toast(
                        Toast::success(tr!(cover_set())).scoped_id("images.cover", i.work_id.get()),
                    ),
                    Err(_) => c.show_toast(
                        Toast::error(tr!(image_not_recorded()))
                            .scoped_id("images.failed", i.work_id.get()),
                    ),
                };
            });
        },
    ));

    let (i, a) = (ids.clone(), app_ctx.clone());
    ctx.register_action_global(Action::new("work.clear_cover").on_invoke(
        move |_intent, c: &mut EventContext| {
            // The bytes stay. Clearing the cover un-marks a picture; it does not
            // delete one, for the same reason undoing an insert does not — see
            // `store`'s doc on why reclamation is always explicit.
            match images::set_cover(&a, i.stack_id.get(), i.work_id.get(), None) {
                Ok(()) => c.show_toast(
                    Toast::success(tr!(cover_cleared())).scoped_id("images.cover", i.work_id.get()),
                ),
                Err(_) => c.show_toast(
                    Toast::error(tr!(image_not_recorded()))
                        .scoped_id("images.failed", i.work_id.get()),
                ),
            };
        },
    ));
}

/// `editor.insert_dropped_images` — files dropped straight onto the prose.
///
/// The same pipeline `Insert image…` runs, minus the file dialog: the writer has
/// already chosen, by dropping. The size prompt still applies — a photograph
/// dragged off a desktop is exactly the case it exists for — and the caret is
/// already where the drop landed, because the editor put it there while the
/// drag was still in the air.
///
/// Anything that is not an image it can decode is skipped in silence. A drop is
/// often several files at once, and a toast per unsupported one would bury the
/// pictures that did arrive.
fn register_drop(ctx: &mut BuildContext, deps: &CommandDeps) {
    let format = deps.format.clone();
    let docs = deps.session.open_docs.clone();
    let ids = deps.ids.clone();
    let app_ctx = deps.app_ctx.clone();

    ctx.register_action_global(Action::new("editor.insert_dropped_images").on_invoke(
        move |_intent, c: &mut EventContext| {
            let paths = format.dropped_files().get();
            format.dropped_files().set(Vec::new());
            if paths.is_empty() || format.handle_for_commands().is_none() {
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
                format: format.clone(),
                docs: docs.clone(),
                ids: ids.clone(),
                app_ctx: app_ctx.clone(),
            };
            let stored = stored_policy(c);
            // Examine the whole drop before inserting any of it. The size
            // question is asked once and answered for the batch, and that
            // answer cannot be applied to files that have not been read yet —
            // reading first is what lets one prompt cover ten photographs
            // instead of abandoning the nine behind the first large one.
            let batch: Vec<PendingImage> = paths
                .iter()
                .filter_map(|path| images::examine(path).ok())
                .collect();
            if batch.is_empty() {
                return;
            }
            match stored {
                Some(policy) => {
                    for pending in batch {
                        finish(&deps, c, pending, policy);
                    }
                }
                None if batch.iter().any(PendingImage::is_large) => {
                    ask_then_finish(&deps, c, batch);
                }
                None => {
                    for pending in batch {
                        finish(&deps, c, pending, SizePolicy::KeepOriginal);
                    }
                }
            }
        },
    ));
}

pub(super) fn register(ctx: &mut BuildContext, deps: &CommandDeps) {
    register_cover(ctx, deps);
    register_editing(ctx, deps);
    register_drop(ctx, deps);
    let format = deps.format.clone();
    let docs = deps.session.open_docs.clone();
    let ids = deps.ids.clone();
    let app_ctx = deps.app_ctx.clone();

    ctx.register_action_global(Action::new("editor.insert_image").on_invoke(
        move |_intent, c: &mut EventContext| {
            // Nothing to insert into: the command is reachable from the menu
            // whatever has focus, so this is the ordinary case, not an error.
            if format.handle_for_commands().is_none() {
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
                let pending = match images::examine(&path) {
                    Ok(p) => p,
                    Err(e) => {
                        c.show_toast(
                            Toast::error(lit!(e.to_string()))
                                .scoped_id("images.failed", ids.work_id.get()),
                        );
                        return;
                    }
                };
                let deps = InsertDeps {
                    format: format.clone(),
                    docs: docs.clone(),
                    ids: ids.clone(),
                    app_ctx: app_ctx.clone(),
                };
                match stored_policy(c) {
                    Some(policy) => finish(&deps, c, pending, policy),
                    None if pending.is_large() => ask_then_finish(&deps, c, vec![pending]),
                    None => finish(&deps, c, pending, SizePolicy::KeepOriginal),
                }
            });
        },
    ));
}

/// What `finish` needs, bundled so the closures that carry it stay readable.
#[derive(Clone)]
struct InsertDeps {
    format: crate::view_models::FormatViewModel,
    docs: crate::models::OpenDocsStore,
    ids: crate::app_ids::AppIds,
    app_ctx: std::rc::Rc<AppContext>,
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
        for pending in batch {
            finish(deps, ctx, pending, SizePolicy::KeepOriginal);
        }
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
            for pending in batch.clone() {
                finish(&deps, c, pending, policy);
            }
        })
        .show_again_checkbox(tr!(image_large_remember()))
        .present(ctx);
}

/// Store the image, record it, and put it in the prose.
///
/// Order matters: the bytes reach disk before anything references them, so a
/// crash between the two leaves an unreferenced file rather than prose naming a
/// picture that was never written. The stray file is harmless — content-
/// addressed, so re-inserting the same picture reuses it — but nothing reclaims
/// it yet: there is no "remove unused images" command, so an orphan costs its
/// bytes on disk until one exists.
fn finish(deps: &InsertDeps, ctx: &mut EventContext, pending: PendingImage, policy: SizePolicy) {
    let image = images::apply_policy(pending, policy);
    let media_dir = deps.docs.media_dir();
    if let Err(e) = images::store(&image, &media_dir) {
        ctx.show_toast(
            Toast::error(lit!(e.to_string())).scoped_id("images.failed", deps.ids.work_id.get()),
        );
        return;
    }

    // The `Asset` row is what a save writes into the bundle; without it the file
    // sits in the media directory and never travels with the project.
    let work_id = deps.ids.work_id.get().unwrap_or_default();
    let created = frontend::commands::asset_commands::create_asset(
        &deps.app_ctx,
        deps.ids.stack_id.get(),
        &frontend::asset::dtos::CreateAssetDto {
            created_at: Default::default(),
            updated_at: Default::default(),
            content_hash: image.content_hash.clone(),
            file_name: image.file_name.clone(),
            mime_type: image.mime_type.clone(),
            width: u64::from(image.width),
            height: u64::from(image.height),
            byte_size: image.bytes.len() as u64,
            alt: String::new(),
            // Inserting a picture into the prose never makes it the cover; that
            // is a separate, deliberate choice made from the image itself.
            is_cover: false,
        },
        work_id,
        -1,
    );
    if created.is_err() {
        ctx.show_toast(
            Toast::error(tr!(image_not_recorded()))
                .scoped_id("images.failed", deps.ids.work_id.get()),
        );
        return;
    }

    let Some(handle) = deps.format.handle_for_commands() else {
        return;
    };
    // Register before inserting: the insertion triggers a relayout, and a paint
    // that cannot resolve the name draws nothing where the image belongs.
    handle.add_image_resource(&image.relative_path(), &image.mime_type, &image.bytes);
    handle.insert_djot(&image.djot("", image.width, image.height));

    // The menu overlay took focus when it opened; without this the writer is
    // left with no caret, exactly as the template-insert command documents.
    deps.format.refocus(ctx);
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
                        && let Some(size) = image.scaled(percent)
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

/// The clicked image's character offset, if there is one.
fn image_offset(format: &crate::view_models::FormatViewModel) -> Option<usize> {
    format.active_image().get().map(|(offset, _)| offset)
}

/// The editor holding the clicked image, and the image as the prose stores it.
///
/// `None` when nothing is in hand, when the editor has gone, or when the
/// recorded offset no longer holds an image — the prose may have been edited
/// since the click, and rewriting whatever is there now would be worse than
/// doing nothing.
fn resolve_active(
    format: &crate::view_models::FormatViewModel,
) -> Option<(teksilo::widgets::rich_text::EditorHandle, images::ImageRef)> {
    let (offset, _src) = format.active_image().get()?;
    let handle = format.handle_for_commands()?;
    let image = images::image_at(&handle.to_plain_text(), offset, &handle.to_djot())?;
    Some((handle, image))
}
