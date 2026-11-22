// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `UnattachedResourcesViewModel`: the logic of Settings ▸ Work ▸ **Resources**, the
//! project's resources that no item carries.
//!
//! A resource ends up attached to nothing when the writer detaches it from its last item,
//! or when the item it was on is deleted for good. It is still in the project, and still
//! travels with it (a stored file is in every copy and every backup), which is why this
//! page exists: to find those, and to do one of three things with them, for one or a
//! selection:
//!
//! - **attach them to items**, picked in the binder tree ([`DestinationPicker::multiple`]),
//!   as one undo step;
//! - **delete them**, after asking, as one undo step;
//! - **export copies** to a folder the writer picks ([`super::export`]).
//!
//! A resource on a trashed item is attached: emptying the trash detaches it, restoring
//! the item brings it back with it.
//!
//! Above the list, the size the project's stored files take, and for a single-file
//! project past [`sizes::ZIP_NOTICE_BYTES`] the advice that a folder project saves
//! faster ([`suggests_folder`]), with the button that runs `work.save_as_folder`.
//!
//! [`DestinationPicker::multiple`]: crate::widgets::DestinationPicker::multiple

use std::rc::Rc;

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::core::styles::PanelVariant;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Divider, Expand, FixedSize, HStack, IconButton, Padding, Panel, Spacer,
    TextWidget, Toast, VStack,
};

use frontend::common::entities::WorkShape;

use crate::models::ResourceRow;
use crate::resources::export::{ExportReport, export_copies};
use crate::resources::{ResourcesViewModel, sizes, sorted_by_title};
use crate::toast_scope::ToastWorkExt;
use crate::widgets::DestinationPicker;

/// The named command the size line's button fires.
pub const SAVE_AS_FOLDER: &str = "work.save_as_folder";

/// Whether the size line advises keeping the project as a folder: a single-file project
/// whose stored files take [`sizes::ZIP_NOTICE_BYTES`] or more, the size at which the
/// dock offers the switch.
pub fn suggests_folder(stored_bytes: u64, shape: Option<WorkShape>) -> bool {
    shape == Some(WorkShape::Zip) && stored_bytes >= sizes::ZIP_NOTICE_BYTES
}

struct Inner {
    work_id: Signal<Option<u64>>,
    resources: ResourcesViewModel,
}

/// The Work's Resources page. Cheap to clone; clones share it.
#[derive(Clone)]
pub struct UnattachedResourcesViewModel {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for UnattachedResourcesViewModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnattachedResourcesViewModel")
            .finish_non_exhaustive()
    }
}

impl UnattachedResourcesViewModel {
    pub fn new(work_id: Signal<Option<u64>>, resources: ResourcesViewModel) -> Self {
        Self {
            inner: Rc::new(Inner { work_id, resources }),
        }
    }

    pub fn resources(&self) -> &ResourcesViewModel {
        &self.inner.resources
    }

    /// Bumps whenever a resource or an attachment may have changed.
    pub fn version(&self) -> Signal<u64> {
        self.inner.resources.version()
    }

    fn work(&self) -> Option<u64> {
        self.inner.work_id.get()
    }

    /// The resources no item carries, by title. None when the attachments cannot be
    /// read: listing every resource as loose would offer each one to Remove.
    pub fn rows(&self) -> Vec<ResourceRow> {
        match self.inner.resources.session().model().unattached() {
            Ok(rows) => sorted_by_title(rows),
            Err(e) => {
                eprintln!("resources: could not read what the items carry: {e}");
                Vec::new()
            }
        }
    }

    /// The bytes the project's stored files take, every resource counted, attached or
    /// not.
    pub fn stored_bytes(&self) -> u64 {
        self.inner
            .resources
            .session()
            .model()
            .all()
            .iter()
            .filter(|r| r.is_stored())
            .map(|r| r.byte_size)
            .sum()
    }

    /// Whether the size line advises a folder project now.
    pub fn suggests_folder(&self) -> bool {
        suggests_folder(self.stored_bytes(), self.inner.resources.shape().get())
    }

    /// The project's shape, which decides whether the advice can apply at all.
    pub fn shape(&self) -> Signal<Option<WorkShape>> {
        self.inner.resources.shape()
    }

    /// Attach `rows` to `items`, as one undo step.
    pub fn attach(&self, ctx: &mut EventContext, items: &[u64], rows: &[ResourceRow]) -> bool {
        let ids: Vec<u64> = rows.iter().map(|r| r.id).collect();
        self.inner.resources.attach(ctx, items, &ids)
    }

    /// Ask which items to attach `rows` to, in the binder tree, then attach them.
    pub fn pick_items_and_attach(&self, ctx: &mut EventContext, rows: Vec<ResourceRow>) {
        // The same flow the row menus' Attach to items… runs.
        self.inner.resources.pick_items_and_attach(ctx, rows);
    }

    /// Delete `rows` from the project, after asking, as one undo step.
    pub fn delete(&self, ctx: &mut EventContext, rows: &[ResourceRow]) {
        self.inner.resources.remove_many(ctx, rows);
    }

    /// Ask for a folder and write copies of `rows` there.
    pub fn export(&self, ctx: &mut EventContext, rows: Vec<ResourceRow>) {
        if rows.is_empty() {
            return;
        }
        let req = crate::models::dialog_start_in(
            ctx,
            crate::models::FolderPurpose::Export,
            FileDialogRequest::pick_folder().title(tr!(resources_export_title())),
        );
        let vm = self.clone();
        let _ = ctx.pick_folder(req, move |res, c| {
            let FileDialogResult::Folder(Some(dir)) = res else {
                return;
            };
            crate::models::remember_dialog_dir(c, crate::models::FolderPurpose::Export, &dir);
            let report = export_copies(&rows, &dir);
            vm.say_exported(c, &report);
        });
    }

    /// One toast for what went, one for what did not.
    fn say_exported(&self, ctx: &mut EventContext, report: &ExportReport) {
        let work = self.work();
        ctx.show_toast(
            Toast::success(tr!(resources_exported(count = report.written() as i64)))
                .scoped_id("resources.exported", work)
                .target_work(work),
        );
        if !report.missing.is_empty() {
            ctx.show_toast(
                Toast::warning(tr!(resources_export_missing(
                    count = report.missing.len() as i64,
                    names = report.missing.join(", ")
                )))
                .scoped_id("resources.export_missing", work)
                .target_work(work),
            );
        }
        if !report.failed.is_empty() {
            let names: Vec<&str> = report.failed.iter().map(|(n, _)| n.as_str()).collect();
            let reasons: Vec<&str> = report.failed.iter().map(|(_, r)| r.as_str()).collect();
            ctx.show_toast(
                Toast::error(tr!(resources_export_failed(
                    count = names.len() as i64,
                    names = names.join(", ")
                )))
                .body(lit!(reasons.join("\n")))
                .scoped_id("resources.export_failed", work)
                .target_work(work),
            );
        }
    }
}

/// What the attach picker's confirm hands back: the items chosen.
pub type ChoseItems = Rc<dyn Fn(&mut EventContext, Vec<u64>)>;

const CARD_W: f32 = 520.0;
const CARD_H: f32 = 520.0;

/// Present the binder tree, several items at once, and hand the chosen ones to
/// `on_confirm`.
pub fn present_attach(ctx: &mut EventContext, picker: DestinationPicker, on_confirm: ChoseItems) {
    // The tree fills from backend events, and this is raised long after those landed.
    picker.reload();
    ctx.present_modal(
        ModalRequest::deferred(move |t| {
            t.add(AttachPanel {
                picker: picker.clone(),
                on_confirm: on_confirm.clone(),
                root: None,
            })
        })
        .presentation(ModalPresentation::InTree)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
        .title(tr!(resources_attach_title()))
        .size(CARD_W as u32, CARD_H as u32),
    );
}

/// The picker's card: a heading, the tree, Cancel and Attach.
struct AttachPanel {
    picker: DestinationPicker,
    on_confirm: ChoseItems,
    root: Option<WidgetId>,
}

impl std::fmt::Debug for AttachPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AttachPanel").finish_non_exhaustive()
    }
}

impl Widget for AttachPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let body = self.picker.view(tr!(resources_attach_empty()));
        let enabled = self.picker.has_item_selection();
        let confirm = {
            let picker = self.picker.clone();
            let on_confirm = self.on_confirm.clone();
            move |c: &mut EventContext| {
                let items = picker.selected_item_ids();
                if items.is_empty() {
                    return;
                }
                c.dismiss_modal();
                on_confirm(c, items);
            }
        };
        let root = teksu!(ctx => FixedSize {
            width: CARD_W
            height: CARD_H
            Panel {
                variant: PanelVariant::Raised
                corner_radius: 10.0
                padding: 0.0
                VStack {
                    spacing: 0.0
                    Expand::horizontal {
                        Padding::symmetric(10.0, 14.0) {
                            HStack {
                                spacing: 8.0
                                Expand::horizontal {
                                    TextWidget::new(tr!(resources_attach_hint())) {
                                        style: TextStyleRole::Small
                                        color: TextRole::Secondary
                                    }
                                }
                                IconButton::clear() {
                                    tooltip: tr!(resources_dialog_cancel())
                                    on_activate_fn: |c| c.dismiss_modal()
                                }
                            }
                        }
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::vertical {
                        child: body
                    }
                    Expand::horizontal {
                        Divider
                    }
                    Expand::horizontal {
                        FixedSize {
                            height: 52.0
                            Padding::symmetric(10.0, 22.0) {
                                HStack {
                                    spacing: 8.0
                                    Spacer
                                    Button::new(tr!(resources_dialog_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: |c| c.dismiss_modal()
                                    }
                                    Button::new(tr!(resources_attach_confirm())) {
                                        variant: ButtonVariant::Filled
                                        enabled: enabled
                                        on_activate_fn: confirm
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
        self.root = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root.into_iter().collect()
    }
}

#[cfg(test)]
mod tests;
