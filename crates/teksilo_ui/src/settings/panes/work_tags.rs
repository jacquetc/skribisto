// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Tags** — the per-project tag palette manager.
//!
//! Shaped like the personal-dictionary pane next to it: a description, a prominent add row, a
//! filter + live count + preset/import/export toolbar, and a bordered list. Each row edits one
//! tag in place — rename, recolour, describe, mark story-bible, delete.
//!
//! Two things here that the dictionary pane has no equivalent of:
//!
//! * **A duplicate name is a warning, never a refusal.** The backend permits duplicates and
//!   must not be made to care; two tags legitimately share a name for a moment while one is
//!   being renamed. It is surfaced as `ValidationState::Warning` — "suspicious but accepted"
//!   — which is reachable only by binding an external signal, since the
//!   `ValidationOutcome → ValidationState` bridge has no path to `Warning` and `.validator()`
//!   fires on commit rather than as you type.
//! * **Deleting a tag in use detaches it from every item**, so the confirmation says how many.
//!
//! Like the dictionary pane it needs generic-closure widgets (`ListView`) the `teksu!` DSL
//! cannot express, so it is a chained-builder module.

use teksilo::core::styles::{ComboBoxVariant, TextInputVariant};
use teksilo::data::SortFilterListModel;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::tokens::{BorderRole, CornerRadius, SurfaceRole};
use teksilo::widgets::{
    BuiltInIcons, Button, ButtonVariant, Center, ColorEdit, ComboBox, Expand, FixedSize, HStack,
    IconButton, IconLocation, IconWidget, ListView, MaxSize, MenuItem, MenuList, MinSize, Padding,
    Panel, PopoverButton, RectWidget, SearchField, Spacer, Switcher, TextInput, TextWidget, Toast,
    Toggle, VStack, ValidationState,
};

use crate::app_ids::HasWorkId;
use crate::models::TagRow;
use crate::note_templates::NoteTemplatesViewModel;
use crate::tags::{Preset, TagsViewModel, contrast};
use crate::toast_scope::ToastWorkExt;
use frontend::common::entities::BinderItemRole;

const NAME_COL: &str = "name";
const FILTER_FIELD_MAX_WIDTH: f32 = 260.0;
const LIST_MIN_HEIGHT: f32 = 320.0;
const SWATCH_SIZE: f32 = 12.0;

/// Colour offered for a tag created here before the writer picks one. Mid-slate: legible in
/// both themes and visibly "unset".
const DEFAULT_NEW_COLOR: &str = "#607d8b";

fn add_glyph() -> IconWidget {
    (BuiltInIcons::defaults().add)().icon_size(15.0)
}
fn import_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/import.svg")).icon_size(15.0)
}
fn export_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/export.svg")).icon_size(15.0)
}

/// `app_ctx`, `ids` and `templates` are threaded from `WorkSession` rather than read off
/// `app_state`, the same discipline `settings::content` already applies to `vm` and for
/// the same reason recorded there: an `app_state` lookup resolves to whichever Work's
/// session registered first, not this window's.
pub fn work_tags_pane(
    ctx: &mut BuildContext,
    vm: &TagsViewModel,
    templates: &NoteTemplatesViewModel,
) -> impl Widget {
    let filtered = SortFilterListModel::new(vm.list_model()).with_predicate(NAME_COL, |text| {
        let needle = text.trim().to_lowercase();
        Box::new(move |row: &TagRow| {
            needle.is_empty()
                || row.name.to_lowercase().contains(&needle)
                || row.details.to_lowercase().contains(&needle)
        })
    });
    let query = Signal::new(String::new());
    {
        // Pushed imperatively — `filters_signal` would `observe` a derived signal.
        let filter_view = filtered.clone();
        ctx.effect(&query, move |q| filter_view.set_filter(NAME_COL, q));
    }

    let list_vm = vm.clone();
    // Resolved once per pane build, not once per row: a project with forty tags would
    // otherwise walk the whole binder forty times to paint one dropdown each.
    let folders = folder_options(&vm.app_ctx(), &vm.ids());
    let template_rows = template_options(templates);
    let list = ListView::from_source(filtered, move |_i, row: &TagRow, _selected| {
        Box::new(TagRowView {
            vm: list_vm.clone(),
            row: row.clone(),
            folders: folders.clone(),
            templates: template_rows.clone(),
            root_child: None,
        })
    })
    .auto_item_height(82.0);

    let empty_idx = {
        let vm = vm.clone();
        vm.changed_signal().map(move |_| usize::from(vm.is_empty()))
    };
    let list_card = Panel::new()
        .background(SurfaceRole::Content)
        .border_color(BorderRole::Default)
        .border_width(1.0)
        .corner_radius(8.0)
        .padding(0.0)
        .child(
            // The floor goes on the card, not the list: a `Switcher` reports its active
            // child's size, and a virtualised `ListView` given unbounded height inside the
            // pane's own scroll reports ~nothing.
            MinSize::new(0.0, LIST_MIN_HEIGHT).child(
                Switcher::new(empty_idx)
                    .child(Expand::vertical().child(list))
                    .child(empty_state(vm)),
            ),
        );

    // Description lives on the add field's tooltip — no body-copy paragraph above the list.
    VStack::new()
        .spacing(16.0)
        .child(add_row(ctx, vm))
        .child(toolbar_row(vm, query))
        .child(Expand::horizontal().child(list_card))
}

/// The prominent add row: name field with a leading `+`, and a filled button.
fn add_row(ctx: &mut BuildContext, vm: &TagsViewModel) -> impl Widget {
    let text = Signal::new(String::new());
    // Owned rather than derived: `.validation()` treats a bound signal as a shared *write*
    // target, and a `.map()` signal is lazy and read-only — binding one here would be wrong.
    let validation = Signal::new(ValidationState::None);

    {
        // Live as the writer types, not on commit. `TextInput` has no change hook, so the
        // warning is pushed from an effect on the text signal — the same imperative-push
        // shape the filter above uses, and for the same reason.
        //
        // On commit would be useless anyway: the warning would arrive at the same moment as
        // the tag it was meant to inform.
        let vm = vm.clone();
        let validation = validation.clone();
        ctx.effect(&text, move |typed| {
            validation.set(match vm.duplicate_name(typed, None) {
                Some(existing) => {
                    ValidationState::Warning(tr!(settings_tags_duplicate(name = existing)))
                }
                None => ValidationState::None,
            });
        });
    }

    let commit = {
        let vm = vm.clone();
        let text = text.clone();
        let validation = validation.clone();
        move |ctx: &mut EventContext| {
            let name = text.get().trim().to_string();
            if name.is_empty() {
                return;
            }
            // Backend permits same-name tags, but creating another is almost always a
            // mistake — refuse here and toast so the writer sees why Add did nothing
            // (the inline validation warning is easy to miss when clicking the button).
            if let Some(existing) = vm.duplicate_name(&name, None) {
                ctx.show_toast(
                    Toast::warning(tr!(settings_tags_duplicate(name = existing)))
                        .scoped_id("tags.duplicate", vm.work_id())
                        .target_work(vm.work_id()),
                );
                return;
            }
            if vm.create(&name, DEFAULT_NEW_COLOR, "", false).is_some() {
                ctx.show_toast(
                    Toast::info(tr!(settings_tags_added(name = name.clone())))
                        .scoped_id("tags.added", vm.work_id())
                        .target_work(vm.work_id()),
                );
            }
            text.set(String::new());
            validation.set(ValidationState::None);
        }
    };

    let field = {
        let commit = commit.clone();
        TextInput::new(text.clone())
            .leading_slot(add_glyph().color(TextRole::Secondary))
            .placeholder(tr!(settings_tags_add_placeholder()))
            .validation(validation.clone())
            .rich_tooltip_content(teksilo::widgets::tooltip::TooltipContent::new(
                "settings.tags",
                tr!(settings_tags_desc()),
            ))
            .on_submit_fn(move |ctx| commit(ctx))
    };

    let can_add = text.map(|t| !t.trim().is_empty());
    let add_btn = Button::new(tr!(settings_tags_add()))
        .variant(ButtonVariant::Filled)
        .enabled(can_add)
        .on_activate_fn(move |ctx| commit(ctx));

    // "Apply a preset…" lives here, beside "Add tag", rather than in the toolbar below: both
    // controls create tags, and the toolbar is for acting on the tags that already exist.
    //
    // It also has to be here. The toolbar carries a filter field whose width is capped, so its
    // buttons get whatever is left; adding a third one overflowed the pane and clipped
    // "Export…" off the right edge. Shrinking the field would have papered over it in English
    // only — "Appliquer un préréglage…" is half again as wide as "Apply a preset…", so French
    // would still have clipped. The add row's field is `Expand`, so it absorbs whatever the
    // buttons need in any locale.
    HStack::new()
        .spacing(10.0)
        .child(Expand::horizontal().child(field))
        .child(preset_button(vm))
        .child(add_btn)
}

/// The preset catalogue, as a popover menu.
fn preset_button(vm: &TagsViewModel) -> impl Widget {
    let mut menu = MenuList::new();
    for preset in Preset::ALL {
        let vm = vm.clone();
        menu = menu.item(MenuItem::new(preset.label()).on_activate_fn(move |c| {
            let summary = vm.apply_preset(preset);
            // Say what happened rather than just closing: re-applying a preset is a no-op by
            // design (names already present are skipped), and silence would read as failure.
            c.show_toast(
                Toast::info(tr!(settings_tags_preset_applied(
                    added = summary.added as i64,
                    skipped = summary.duplicates as i64
                )))
                .scoped_id("tags.preset", vm.work_id())
                .target_work(vm.work_id()),
            );
        }));
    }

    PopoverButton::new(Button::new(tr!(settings_tags_apply_preset())).variant(ButtonVariant::Plain))
        .bare()
        .content(menu)
}

/// Filter + live count on the left, Import…/Export… pushed to the right — the same shape as
/// the personal-dictionary pane's toolbar, and deliberately no wider.
fn toolbar_row(vm: &TagsViewModel, query: Signal<String>) -> impl Widget {
    let count = {
        let vm = vm.clone();
        vm.changed_signal()
            .map(move |_| tr!(settings_tags_count(n = vm.rows().len() as i64)).resolve_now())
    };

    HStack::new()
        .spacing(10.0)
        .child(
            MaxSize::width(FILTER_FIELD_MAX_WIDTH)
                .child(SearchField::new(query).placeholder(tr!(settings_tags_filter()))),
        )
        .child(
            TextWidget::new(lit!(""))
                .text(count)
                .color(TextRole::Secondary),
        )
        .child(Expand::horizontal().child(Spacer::new()))
        .child(import_button(vm))
        .child(export_button(vm))
}

fn import_button(vm: &TagsViewModel) -> impl Widget {
    let vm = vm.clone();
    Button::new(tr!(settings_tags_import()))
        .variant(ButtonVariant::Plain)
        .icon(import_glyph(), IconLocation::Leading)
        .on_activate_fn(move |ctx| {
            let vm = vm.clone();
            let req = crate::models::dialog_start_in(
                ctx,
                crate::models::FolderPurpose::DataInterchange,
                FileDialogRequest::pick_file()
                    .title(tr!(settings_tags_import()))
                    .add_filter(tr!(settings_tags_csv_filter()).resolve_now(), &["csv"]),
            );
            let _ = ctx.pick_file(req, move |res, c| {
                if let FileDialogResult::File(Some(path)) = res {
                    crate::models::remember_dialog_file(
                        c,
                        crate::models::FolderPurpose::DataInterchange,
                        &path,
                    );
                    match vm.import_from(&path) {
                        Ok(s) => {
                            c.show_toast(
                                Toast::info(tr!(settings_tags_imported(
                                    added = s.added as i64,
                                    skipped = (s.duplicates + s.malformed) as i64
                                )))
                                .scoped_id("tags.imported", vm.work_id())
                                .target_work(vm.work_id()),
                            );
                        }
                        Err(e) => {
                            c.show_toast(
                                Toast::info(lit!(format!("{e:#}")))
                                    .scoped_id("tags.error", vm.work_id())
                                    .target_work(vm.work_id()),
                            );
                        }
                    }
                }
            });
        })
}

fn export_button(vm: &TagsViewModel) -> impl Widget {
    let vm = vm.clone();
    Button::new(tr!(settings_tags_export()))
        .variant(ButtonVariant::Plain)
        .icon(export_glyph(), IconLocation::Leading)
        .on_activate_fn(move |ctx| {
            let vm = vm.clone();
            let req = crate::models::dialog_start_in(
                ctx,
                crate::models::FolderPurpose::DataInterchange,
                FileDialogRequest::save_file()
                    .title(tr!(settings_tags_export()))
                    .default_file_name("tags.csv".to_string())
                    .add_filter(tr!(settings_tags_csv_filter()).resolve_now(), &["csv"]),
            );
            let _ = ctx.save_file(req, move |res, c| {
                if let FileDialogResult::Saved(Some(mut path)) = res {
                    crate::models::remember_dialog_file(
                        c,
                        crate::models::FolderPurpose::DataInterchange,
                        &path,
                    );
                    if path.extension().and_then(|e| e.to_str()) != Some("csv") {
                        path.set_extension("csv");
                    }
                    match vm.export_to(&path) {
                        Ok(n) => {
                            c.show_toast(
                                Toast::info(tr!(settings_tags_exported(n = n as i64)))
                                    .scoped_id("tags.exported", vm.work_id())
                                    .target_work(vm.work_id()),
                            );
                        }
                        Err(e) => {
                            c.show_toast(
                                Toast::info(lit!(format!("{e:#}")))
                                    .scoped_id("tags.error", vm.work_id())
                                    .target_work(vm.work_id()),
                            );
                        }
                    }
                }
            });
        })
}

/// One palette row: swatch + colour picker, inline name, details, story-bible toggle, delete.
///
/// A real `Widget` rather than a plain builder function, because a row needs a
/// `BuildContext`: `Toggle` and `TextInput` both write only to their signals (neither has a
/// change callback), so persisting an edit means observing those signals from an effect —
/// and `ctx` is the only place effects can be installed. A `ListView` delegate returning
/// `Box<dyn Widget>` gives every row its own `build`, which is exactly the hook needed.
struct TagRowView {
    vm: TagsViewModel,
    row: TagRow,
    /// Both resolved once per pane build and shared by every row: see the note at the
    /// call site.
    folders: Vec<PickOption>,
    templates: Vec<PickOption>,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TagRowView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagRowView")
            .field("name", &self.row.name)
            .finish()
    }
}

impl Widget for TagRowView {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = self.row.id;
        let fill = contrast::parse(&self.row.color);

        // Colour. `ColorEdit` is the DateEdit-shaped trigger+popover; seeding its grid with
        // the framework palette keeps a writer picking coherent hues rather than typing hex.
        let color_signal = Signal::new(fill);
        let color_edit = {
            let vm = self.vm.clone();
            let color_signal = color_signal.clone();
            ColorEdit::new(color_signal.clone())
                .swatches(teksilo::widgets::color_picker::DEFAULT_SWATCHES.to_vec())
                .on_close(move || vm.recolor(id, &color_signal.get().to_hex_lower(false)))
        };

        // Name, with the duplicate warning live as it is typed.
        let name = Signal::new(self.row.name.clone());
        let validation = Signal::new(ValidationState::None);
        {
            let vm = self.vm.clone();
            let validation = validation.clone();
            ctx.effect(&name, move |typed| {
                // `Some(id)` excludes this row — a tag never collides with itself.
                validation.set(match vm.duplicate_name(typed, Some(id)) {
                    Some(existing) => {
                        ValidationState::Warning(tr!(settings_tags_duplicate(name = existing)))
                    }
                    None => ValidationState::None,
                });
            });
        }
        // Both fields commit on Enter **and on blur**. Enter alone is not enough: nothing
        // about a bare inline field says it must be confirmed, so typing a description and
        // clicking away — or just closing Settings — silently threw the text away.
        //
        // Each commit compares against the model first. Blur fires whenever focus moves,
        // including when nothing was typed, and an unguarded write would push an identical
        // value through the command stack (an undo step that changes nothing) on every pass
        // through the field.
        let name_field = {
            let vm = self.vm.clone();
            let sig = name.clone();
            let commit = move || {
                let typed = sig.get();
                let current = vm.rows().into_iter().find(|r| r.id == id).map(|r| r.name);
                // A blank name would leave a row that cannot be identified; keep the old one.
                if !typed.trim().is_empty() && current.as_deref() != Some(typed.as_str()) {
                    vm.rename(id, &typed);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(name.clone())
                .variant(TextInputVariant::Bare)
                .validation(validation)
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        let details = Signal::new(self.row.details.clone());
        let details_field = {
            let vm = self.vm.clone();
            let sig = details.clone();
            let commit = move || {
                let typed = sig.get();
                let current = vm
                    .rows()
                    .into_iter()
                    .find(|r| r.id == id)
                    .map(|r| r.details);
                // Blank IS meaningful here — it clears the description.
                if current.as_deref() != Some(typed.as_str()) {
                    vm.set_details(id, &typed);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(details.clone())
                .variant(TextInputVariant::Bare)
                .placeholder(tr!(settings_tags_details_placeholder()))
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        // The toggle writes only to its signal, so the write-back rides an effect. It
        // compares against the model first: without that guard the refresh that follows the
        // write would echo straight back in as a second write.
        let discoverable = Signal::new(self.row.discoverable);
        {
            let vm = self.vm.clone();
            ctx.effect(&discoverable, move |on| {
                let current = vm
                    .rows()
                    .into_iter()
                    .find(|r| r.id == id)
                    .map(|r| r.discoverable);
                if current != Some(*on) {
                    vm.set_discoverable(id, *on);
                }
            });
        }
        // The row's one control whose label cannot explain itself: "Find in prose" says
        // what switching it on makes Skribisto do, not what set the tag joins. Bound by
        // registry key so it reads identically here and in the pill field's "New tag…"
        // form.
        let toggle = Toggle::new(discoverable)
            .label(tr!(settings_tags_discoverable()))
            .rich_tooltip(crate::tooltip_registry::WM_FIND_IN_PROSE);

        // ── Where its notes go, and what shape they start in ────────────────
        //
        // Both write a relationship rather than a column, so neither rides the row's
        // `update` path. Both are also genuinely optional: "Ask me the first time" is
        // the first entry, so clearing a destination is as easy as setting one. A tag
        // whose folder the writer later reorganises away must be un-filable without
        // deleting the tag itself.
        let creates_in = {
            let selected = Signal::new(
                self.folders
                    .iter()
                    .find(|o| o.id == self.row.creates_in)
                    .cloned()
                    // A destination that no longer resolves (the folder was deleted
                    // between this build and the last) falls back to the unset row
                    // rather than rendering a blank control.
                    .or_else(|| self.folders.first().cloned()),
            );
            let vm = self.vm.clone();
            ComboBox::from_items(self.folders.clone(), selected, |o: &PickOption| {
                lit!(o.label.clone())
            })
            .variant(ComboBoxVariant::Plain)
            .on_select(move |o: &PickOption, _c| vm.set_creates_in(id, o.id))
        };

        let template = {
            let selected = Signal::new(
                self.templates
                    .iter()
                    .find(|o| o.id == self.row.note_template)
                    .cloned()
                    .or_else(|| self.templates.first().cloned()),
            );
            let vm = self.vm.clone();
            ComboBox::from_items(self.templates.clone(), selected, |o: &PickOption| {
                lit!(o.label.clone())
            })
            .variant(ComboBoxVariant::Plain)
            .on_select(move |o: &PickOption, _c| vm.set_note_template(id, o.id))
        };

        let delete = {
            let vm = self.vm.clone();
            let name = self.row.name.clone();
            IconButton::clear()
                .embedded()
                .tooltip(tr!(settings_tags_delete(name = name.clone())))
                .on_activate_fn(move |c| {
                    // Deleting detaches the tag from every item carrying it, and undo restores
                    // both. A timed toast is the right weight for a reversible action whose
                    // result is visible on screen — a modal would be nagging.
                    vm.delete(&[id]);
                    c.show_toast(
                        Toast::info(tr!(settings_tags_deleted(name = name.clone())))
                            .scoped_id("tags.deleted", vm.work_id())
                            .target_work(vm.work_id()),
                    );
                })
        };

        let body = Padding::symmetric(6.0, 10.0).child(
            VStack::new()
                .spacing(2.0)
                .child(
                    HStack::new()
                        .spacing(8.0)
                        .child(swatch(fill))
                        .child(color_edit)
                        .child(Expand::horizontal().child(name_field))
                        .child(toggle)
                        .child(delete),
                )
                .child(Padding::new(0.0, 0.0, 0.0, 26.0).child(details_field))
                .child(
                    Padding::new(0.0, 0.0, 0.0, 26.0).child(
                        HStack::new()
                            .spacing(8.0)
                            .child(
                                TextWidget::new(tr!(settings_tags_creates_in()))
                                    .style(TextStyleRole::Tiny)
                                    .color(TextRole::Secondary),
                            )
                            .child(creates_in)
                            .child(
                                TextWidget::new(tr!(settings_tags_template()))
                                    .style(TextStyleRole::Tiny)
                                    .color(TextRole::Secondary),
                            )
                            .child(template),
                    ),
                ),
        );
        let root = ctx.add(body);
        self.root_child = Some(root);
        vec![root]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}

/// One row of a picker: a store id and what to call it.
///
/// `Option<u64>` rather than `u64` so "not set" is a real, selectable choice rather
/// than a state the writer can only reach by never touching the control. Clearing a
/// destination has to be as easy as setting one: a tag filed into a folder the writer
/// later reorganises away should be un-filable without deleting the tag.
#[derive(Clone, PartialEq, Eq)]
pub(crate) struct PickOption {
    pub id: Option<u64>,
    pub label: String,
}

/// Every folder in the Work, as destinations a tag can file into, in binder order.
///
/// Folders only: a note is created *inside* something, and a scene has no inside. Not
/// narrowed to notes folders, deliberately, because "where do my research notes go" is
/// the writer's question to answer and a project that keeps them under a Book folder is
/// organising, not misusing. Trashed rows are excluded by `ordered_flat_items`.
fn folder_options(app_ctx: &frontend::AppContext, ids: &crate::app_ids::AppIds) -> Vec<PickOption> {
    let mut out = vec![PickOption {
        id: Option::None,
        label: tr!(settings_tags_creates_in_unset()).resolve_now(),
    }];
    let Some(work_id) = ids.work_id.get() else {
        return out;
    };
    for (_, it) in crate::models::binder_stream::ordered_flat_items(app_ctx, work_id) {
        if it.role == BinderItemRole::Folder {
            out.push(PickOption {
                id: Some(it.id),
                // An untitled folder is ordinary, not an edge case: a picker row
                // reading nothing at all is unpickable.
                label: if it.title.trim().is_empty() {
                    tr!(settings_tags_creates_in_untitled()).resolve_now()
                } else {
                    it.title.clone()
                },
            });
        }
    }
    out
}

/// Every note template, as starting shapes, plus a blank.
fn template_options(templates: &NoteTemplatesViewModel) -> Vec<PickOption> {
    let mut out = vec![PickOption {
        id: Option::None,
        label: tr!(settings_tags_template_unset()).resolve_now(),
    }];
    out.extend(templates.rows().into_iter().map(|t| PickOption {
        id: Some(t.id),
        label: t.name,
    }));
    out
}

/// The row's leading colour dot.
///
/// `FixedSize`, not `MinSize`: a minimum only floors the size, so inside the row's `HStack`
/// the dot stretched to the full row height and rendered as a tall bar rather than a dot.
///
/// The hairline is what keeps a near-black or near-white tag visible against the matching
/// surface — a tag colour is theme-constant, so either extreme would otherwise vanish in one
/// theme. It is derived from the fill rather than taken from a border token because no token
/// is strong enough: see [`contrast::outline_on`].
fn swatch(color: teksilo::tokens::Color) -> impl Widget {
    Center::new().child(
        FixedSize::new()
            .width(SWATCH_SIZE)
            .height(SWATCH_SIZE)
            .child(
                RectWidget::new()
                    .background(color)
                    .corner_radius(CornerRadius::uniform(9999.0))
                    .border_color(contrast::outline_on(color))
                    .border_width(1.0),
            ),
    )
}

/// Shown instead of an empty bordered box, which reads as broken. Offers the preset menu
/// again, since an empty palette is exactly when a writer wants one.
fn empty_state(vm: &TagsViewModel) -> impl Widget {
    let mut menu = MenuList::new();
    for preset in Preset::ALL {
        let vm = vm.clone();
        menu = menu.item(MenuItem::new(preset.label()).on_activate_fn(move |c| {
            let summary = vm.apply_preset(preset);
            c.show_toast(
                Toast::info(tr!(settings_tags_preset_applied(
                    added = summary.added as i64,
                    skipped = summary.duplicates as i64
                )))
                .scoped_id("tags.preset", vm.work_id())
                .target_work(vm.work_id()),
            );
        }));
    }
    Center::new().child(
        VStack::new()
            .spacing(10.0)
            .child(TextWidget::new(tr!(settings_tags_empty())).color(TextRole::Secondary))
            .child(
                PopoverButton::new(
                    Button::new(tr!(settings_tags_apply_preset())).variant(ButtonVariant::Filled),
                )
                .bare()
                .content(menu),
            ),
    )
}
