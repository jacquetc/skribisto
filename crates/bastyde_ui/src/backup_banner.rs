// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `BackupBanner` — the permanent warning strip shown while a backup file is open.
//!
//! A tiny reactive widget: zero children (and zero height) when no backup is open,
//! else a non-dismissable `Banner` warning that changes can't be saved to the
//! backup, with **Restore** and **Save As** actions. Mounted above the content in
//! `App`'s root column. Rebuilds only on the rare `backup_context` transitions.

use bastyde::core::BindingLevel;
use bastyde::prelude::*;
use bastyde::widgets::{Banner, Button, ButtonVariant, HStack};

use crate::backup::BackupContext;
use crate::singles::SingleWork;
use crate::view_models::{RestoreViewModel, SaveAsViewModel};

pub struct BackupBanner {
    backup_context: Signal<Option<BackupContext>>,
    restore: RestoreViewModel,
    save_as: SaveAsViewModel,
    single_work: SingleWork,
    root_child: Option<WidgetId>,
}

impl BackupBanner {
    pub fn new(
        backup_context: Signal<Option<BackupContext>>,
        restore: RestoreViewModel,
        save_as: SaveAsViewModel,
        single_work: SingleWork,
    ) -> Self {
        Self {
            backup_context,
            restore,
            save_as,
            single_work,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for BackupBanner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackupBanner").finish()
    }
}

impl Widget for BackupBanner {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.backup_context
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let Some(_bc) = self.backup_context.get() else {
            // Not a backup window: no banner, no height.
            self.root_child = None;
            return vec![];
        };

        let restore = self.restore.clone();
        let save_as_vm = self.save_as.clone();
        let single_work = self.single_work.clone();

        let actions = HStack::new()
            .spacing(8.0)
            .child(
                Button::new(tr!(backup_banner_restore()))
                    .variant(ButtonVariant::Filled)
                    .on_activate_fn(move |c| restore.begin(c)),
            )
            .child(
                Button::new(tr!(backup_banner_save_as()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |c| save_as_from_banner(c, &save_as_vm, &single_work)),
            );

        // No `.on_dismiss` ⇒ no close button ⇒ a permanent reminder.
        let banner = Banner::warning(tr!(backup_banner_title()))
            .description(tr!(backup_banner_description()))
            .action(actions);

        let id = ctx.add(banner);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Hug the banner's height; zero when there's no backup open.
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

/// "Save As" from the banner: pick a `.skrib` target and save the current store
/// (the backup's content + any edits) there, mirroring the File ▸ Save-as-file
/// menu — the escape hatch for keeping edits made in backup mode.
fn save_as_from_banner(
    ctx: &mut EventContext,
    save_as: &SaveAsViewModel,
    single_work: &SingleWork,
) {
    let base = {
        let title = single_work.title().get();
        if title.trim().is_empty() {
            "restored".to_string()
        } else {
            title
        }
    };
    let req = FileDialogRequest::save_file()
        .title(tr!(backup_banner_save_as()))
        .default_file_name(format!("{base}.skrib"))
        .add_filter("Skribisto work", &["skrib"]);
    let save_as = save_as.clone();
    let _ = ctx.save_file(req, move |res, ectx| {
        if let FileDialogResult::Saved(Some(path)) = res {
            let mut target = path.to_string_lossy().into_owned();
            if !target.ends_with(".skrib") {
                target.push_str(".skrib");
            }
            // `begin` flushes the editors into the store before the background op
            // reads it. This is the *only* way edits made in backup mode can be
            // kept (Save is off there), so writing the pre-edit prose here would
            // lose them outright.
            save_as.begin(ectx, target, false);
        }
    });
}
