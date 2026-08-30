// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ Work ▸ **Templates** — the per-project note-template catalogue.
//!
//! Shaped like the Tags pane beside it: a description, a filter + live count +
//! preset/import/export toolbar, and a bordered list whose rows edit in place.
//!
//! Three things differ from the tag pane, each for a reason:
//!
//! * **No add row.** A template is a body of prose, and there is nowhere sensible to type
//!   one into a settings list. The three ways to make one all live where the prose already
//!   is: apply a preset, import a `.md`/`.djot`, or write a note and use
//!   Document ▸ Save as template. The empty state says exactly that.
//! * **Rows are ordered by hand, not sorted.** The list is the writer's arrangement, so
//!   each row carries move-up/move-down whose enabled state comes from the model rather
//!   than being always-on (a button that silently does nothing is worse than a greyed one).
//! * **Delete asks first.** Losing a tag costs a label; losing a template costs a document
//!   nothing else holds a copy of.
//!
//! Like the dictionary and tag panes it needs generic-closure widgets (`ListView`) that
//! `teksu!` cannot express, so it is a chained-builder module.

use teksilo::core::styles::TextInputVariant;
use teksilo::data::SortFilterListModel;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::tokens::{BorderRole, SurfaceRole, TextRole};
use teksilo::widgets::{
    Button, ButtonVariant, Center, Expand, HStack, IconButton, IconLocation, IconWidget, ListView,
    MaxSize, MenuItem, MenuList, MessageBox, MessageBoxButtons, MessageBoxResult, MinSize, Padding,
    Panel, PopoverButton, SearchField, Spacer, StandardButton, Switcher, TextInput, TextWidget,
    Toast, VStack, ValidationState,
};

use crate::app_ids::HasWorkId;
use crate::models::TemplateRow;
use crate::note_templates::{NoteTemplatesViewModel, Preset};
use crate::toast_scope::ToastWorkExt;

const NAME_COL: &str = "name";
const FILTER_FIELD_MAX_WIDTH: f32 = 260.0;

fn import_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/import.svg")).icon_size(15.0)
}
fn export_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/settings/export.svg")).icon_size(15.0)
}
fn star_glyph(on: bool) -> IconWidget {
    let svg = if on {
        res!("assets/icons/templates/star-filled.svg")
    } else {
        res!("assets/icons/templates/star.svg")
    };
    IconWidget::from_svg_icon(svg).icon_size(15.0)
}
fn move_up_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/templates/move-up.svg")).icon_size(15.0)
}
fn move_down_glyph() -> IconWidget {
    IconWidget::from_svg_icon(res!("assets/icons/templates/move-down.svg")).icon_size(15.0)
}

pub fn work_templates_pane(ctx: &mut BuildContext, vm: &NoteTemplatesViewModel) -> impl Widget {
    let filtered = SortFilterListModel::new(vm.list_model()).with_predicate(NAME_COL, |text| {
        let needle = text.trim().to_lowercase();
        Box::new(move |row: &TemplateRow| {
            needle.is_empty()
                || row.name.to_lowercase().contains(&needle)
                || row.body.to_lowercase().contains(&needle)
        })
    });
    let query = Signal::new(String::new());
    {
        // Pushed imperatively — `filters_signal` would `observe` a derived signal.
        let filter_view = filtered.clone();
        ctx.effect(&query, move |q| filter_view.set_filter(NAME_COL, q));
    }

    let list_vm = vm.clone();
    let list = ListView::from_source(filtered, move |_i, row: &TemplateRow, _selected| {
        Box::new(TemplateRowView {
            vm: list_vm.clone(),
            row: row.clone(),
            root_child: None,
        })
    })
    .auto_item_height(64.0);

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
            // The floor goes on the card, not the list — see the tag pane's own note: a
            // `Switcher` reports its active child's size, and a virtualised `ListView`
            // given unbounded height inside the pane's scroll reports ~nothing.
            MinSize::new(0.0, crate::settings::fields::LIST_MIN_HEIGHT).child(
                Switcher::new(empty_idx)
                    .child(Expand::vertical().child(list))
                    .child(empty_state()),
            ),
        );

    VStack::new()
        .spacing(16.0)
        .child(crate::widgets::tip::RichTip::new(
            crate::tooltip_registry::CONCEPT_NOTE_TEMPLATE,
            TextWidget::new(tr!(settings_templates_description())).color(TextRole::Secondary),
        ))
        .child(toolbar_row(vm, query))
        // `list_box`: the leftover height of the pane goes to the list, so one
        // scroll region replaces two. The floor it carries is the shared one — the
        // 300–320 px constants that used to live here were taller than the pane's
        // whole viewport, so the page scrolled at *minimum* content while the list
        // scrolled inside it.
        .child(crate::settings::fields::list_box(
            Expand::horizontal().child(list_card),
        ))
}

/// Filter + live count on the left; the three creating/exporting controls pushed right.
fn toolbar_row(vm: &NoteTemplatesViewModel, query: Signal<String>) -> impl Widget {
    let count = {
        let vm = vm.clone();
        vm.changed_signal()
            .map(move |_| tr!(settings_templates_count(n = vm.rows().len() as i64)).resolve_now())
    };

    HStack::new()
        .spacing(10.0)
        .child(
            MaxSize::width(FILTER_FIELD_MAX_WIDTH)
                .child(SearchField::new(query).placeholder(tr!(settings_templates_filter()))),
        )
        .child(
            TextWidget::new(lit!(""))
                .text(count)
                .color(TextRole::Secondary),
        )
        .child(Expand::horizontal().child(Spacer::new()))
        .child(preset_button(vm))
        .child(import_button(vm))
        .child(export_button(vm))
}

/// The built-in catalogue, as a popover menu.
fn preset_button(vm: &NoteTemplatesViewModel) -> impl Widget {
    let mut menu = MenuList::new();
    for preset in Preset::ALL {
        let vm = vm.clone();
        menu = menu.item(MenuItem::new(preset.label()).on_activate_fn(move |c| {
            let name = preset.label().resolve_now();
            let summary = vm.apply_preset(preset);
            // Say what happened rather than just closing. Re-applying is safe (a colliding
            // name is suffixed), and silence would read as failure.
            let msg = if summary.renamed.is_empty() {
                tr!(templates_preset_applied(name = name))
            } else {
                tr!(templates_imported_renamed(
                    names = summary.renamed.join(", ")
                ))
            };
            c.show_toast(
                Toast::info(msg)
                    .scoped_id("templates.preset", vm.work_id())
                    .target_work(vm.work_id()),
            );
        }));
    }

    PopoverButton::new(Button::new(tr!(settings_templates_presets())).variant(ButtonVariant::Plain))
        .bare()
        .content(menu)
}

/// Import one **or more** `.md`/`.djot` files — this app's first caller of `pick_files`.
fn import_button(vm: &NoteTemplatesViewModel) -> impl Widget {
    let vm = vm.clone();
    Button::new(tr!(settings_templates_import()))
        .variant(ButtonVariant::Plain)
        .icon(import_glyph(), IconLocation::Leading)
        .on_activate_fn(move |ctx| {
            let vm = vm.clone();
            let req = crate::models::dialog_start_in(
                ctx,
                crate::models::FolderPurpose::DataInterchange,
                FileDialogRequest::pick_files()
                    .title(tr!(settings_templates_import()))
                    .add_filter(
                        tr!(templates_import_filter()).resolve_now(),
                        &["md", "markdown", "djot"],
                    ),
            );
            let _ = ctx.pick_files(req, move |res, c| {
                crate::models::remember_pick(
                    c,
                    crate::models::FolderPurpose::DataInterchange,
                    &res,
                );
                let FileDialogResult::Files(paths) = res else {
                    return;
                };
                if paths.is_empty() {
                    return; // cancelled
                }
                match vm.import_files(&paths) {
                    Ok(summary) => {
                        // One toast per outcome that actually happened, so a partly-failed
                        // batch says both halves rather than only the cheerful one.
                        c.show_toast(
                            Toast::info(tr!(templates_imported(added = summary.added as i64)))
                                .scoped_id("templates.imported", vm.work_id())
                                .target_work(vm.work_id()),
                        );
                        if !summary.renamed.is_empty() {
                            c.show_toast(
                                Toast::info(tr!(templates_imported_renamed(
                                    names = summary.renamed.join(", ")
                                )))
                                .scoped_id("templates.renamed", vm.work_id())
                                .target_work(vm.work_id()),
                            );
                        }
                        if !summary.skipped_files.is_empty() {
                            c.show_toast(
                                Toast::warning(tr!(templates_imported_skipped(
                                    names = summary.skipped_files.join(", ")
                                )))
                                .scoped_id("templates.skipped", vm.work_id())
                                .target_work(vm.work_id()),
                            );
                        }
                    }
                    Err(e) => {
                        c.show_toast(
                            Toast::warning(lit!(format!("{e:#}")))
                                .scoped_id("templates.error", vm.work_id())
                                .target_work(vm.work_id()),
                        );
                    }
                }
            });
        })
}

/// Export one `.djot` per template into a chosen folder — the exact shape import reads
/// back, so export → edit → import round-trips.
fn export_button(vm: &NoteTemplatesViewModel) -> impl Widget {
    let vm = vm.clone();
    Button::new(tr!(settings_templates_export()))
        .variant(ButtonVariant::Plain)
        .icon(export_glyph(), IconLocation::Leading)
        .on_activate_fn(move |ctx| {
            let vm = vm.clone();
            let req = crate::models::dialog_start_in(
                ctx,
                crate::models::FolderPurpose::DataInterchange,
                FileDialogRequest::pick_folder().title(tr!(settings_templates_export())),
            );
            let _ = ctx.pick_folder(req, move |res, c| {
                let FileDialogResult::Folder(Some(dir)) = res else {
                    return;
                };
                crate::models::remember_dialog_dir(
                    c,
                    crate::models::FolderPurpose::DataInterchange,
                    &dir,
                );
                let _ = match vm.export_to_dir(&dir) {
                    Ok(n) => c.show_toast(
                        Toast::info(tr!(templates_exported(n = n as i64)))
                            .scoped_id("templates.exported", vm.work_id())
                            .target_work(vm.work_id()),
                    ),
                    Err(e) => c.show_toast(
                        Toast::warning(lit!(format!("{e:#}")))
                            .scoped_id("templates.error", vm.work_id())
                            .target_work(vm.work_id()),
                    ),
                };
            });
        })
}

/// Shown instead of the list when the project has none. It names all three ways to get
/// one, because none of them is in this pane.
fn empty_state() -> impl Widget {
    Center::new().child(
        Padding::symmetric(24.0, 24.0).child(
            VStack::new()
                .spacing(6.0)
                .child(TextWidget::new(tr!(settings_templates_empty())))
                .child(
                    TextWidget::new(tr!(settings_templates_empty_hint()))
                        .color(TextRole::Secondary),
                ),
        ),
    )
}

/// One template row: star, inline name, a word count, reorder, delete.
///
/// A real `Widget` rather than a builder function for the same reason `TagRowView` is one:
/// `TextInput` writes only to its signal, so persisting an edit needs an effect, and `ctx`
/// is the only place effects can be installed.
struct TemplateRowView {
    vm: NoteTemplatesViewModel,
    row: TemplateRow,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TemplateRowView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TemplateRowView")
            .field("name", &self.row.name)
            .finish()
    }
}

/// Words in a template body — a cheap sense of how much it inserts, shown beside the name.
///
/// Tokens with no alphanumeric character are skipped, so the Djot markup a template is
/// mostly made of (`#`, `##`, the `-` of every bullet) is not counted as words. Without
/// that a six-prompt section would claim twice the words it shows the writer.
fn word_count(body: &str) -> usize {
    body.split_whitespace()
        .filter(|t| t.chars().any(char::is_alphanumeric))
        .count()
}

impl Widget for TemplateRowView {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let id = self.row.id;

        // Star. A plain momentary button rather than a `Toggle`: it writes through the
        // view-model and the model's refresh brings the new state back, so holding a
        // separate signal would just be a second source of truth to keep in step.
        let star = {
            let vm = self.vm.clone();
            let on = self.row.starred;
            IconButton::new(star_glyph(on))
                .embedded()
                .tooltip(if on {
                    tr!(settings_templates_unstar())
                } else {
                    tr!(settings_templates_star())
                })
                .on_activate_fn(move |_c| vm.set_starred(id, !on))
        };

        // Name, with the duplicate warning live as it is typed.
        //
        // A **warning**, not a refusal — unlike the Save-as-template dialog, which refuses.
        // The difference is transient state: mid-rename two rows may legitimately share a
        // name for a keystroke, exactly as the tag pane documents. The dialog has no such
        // in-between.
        let name = Signal::new(self.row.name.clone());
        let validation = Signal::new(ValidationState::None);
        {
            let vm = self.vm.clone();
            let validation = validation.clone();
            ctx.effect(&name, move |typed| {
                validation.set(match vm.duplicate_name(typed, Some(id)) {
                    Some(existing) => ValidationState::Warning(tr!(
                        settings_templates_duplicate_name(name = existing)
                    )),
                    None => ValidationState::None,
                });
            });
        }
        let name_field = {
            let vm = self.vm.clone();
            let sig = name.clone();
            // Commit on Enter **and on blur**, comparing against the model first — an
            // unguarded blur write would push an identical value through the command stack
            // on every pass through the field. Same shape, same reasons, as the tag pane.
            let commit = move || {
                let typed = sig.get();
                let current = vm.rows().into_iter().find(|r| r.id == id).map(|r| r.name);
                if !typed.trim().is_empty() && current.as_deref() != Some(typed.as_str()) {
                    vm.rename(id, &typed);
                }
            };
            let on_blur = commit.clone();
            TextInput::new(name.clone())
                .variant(TextInputVariant::Bare)
                .placeholder(tr!(settings_templates_name_placeholder()))
                .validation(validation)
                .on_submit_fn(move |_c| commit())
                .on_blur_fn(move |_c| on_blur())
        };

        let words = TextWidget::new(tr!(settings_templates_words(
            n = word_count(&self.row.body) as i64
        )))
        .color(TextRole::Secondary);

        let up = {
            let vm = self.vm.clone();
            IconButton::new(move_up_glyph())
                .embedded()
                .tooltip(tr!(settings_templates_move_up()))
                .enabled(self.vm.can_move(id, -1))
                .on_activate_fn(move |_c| vm.move_by(id, -1))
        };
        let down = {
            let vm = self.vm.clone();
            IconButton::new(move_down_glyph())
                .embedded()
                .tooltip(tr!(settings_templates_move_down()))
                .enabled(self.vm.can_move(id, 1))
                .on_activate_fn(move |_c| vm.move_by(id, 1))
        };

        let delete = {
            let vm = self.vm.clone();
            let name = self.row.name.clone();
            IconButton::clear()
                .embedded()
                .tooltip(tr!(settings_templates_delete()))
                .on_activate_fn(move |c| {
                    // Confirm, unlike the tag pane's timed-undo toast. A tag is a label; a
                    // template is a document, and once the row is gone nothing on screen
                    // holds a copy of its text to undo *from*.
                    let vm = vm.clone();
                    MessageBox::question(tr!(templates_delete_title()))
                        .text(tr!(templates_delete_body(name = name.clone())))
                        .buttons(MessageBoxButtons::OkCancel)
                        .on_result(move |r: MessageBoxResult, _c| {
                            if r.button == StandardButton::Ok {
                                vm.delete(&[id]);
                            }
                        })
                        .present(c);
                })
        };

        let root = HStack::new()
            .spacing(8.0)
            .child(star)
            .child(Expand::horizontal().child(name_field))
            .child(words)
            .child(up)
            .child(down)
            .child(delete);

        let child = ctx.add(Padding::symmetric(6.0, 10.0).child(root));
        self.root_child = Some(child);
        vec![child]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_skips_djot_markup() {
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("   \n\n "), 0);
        // `#` and `-` are markup, not words; "Character", "sheet", "Full", "name:" are.
        assert_eq!(word_count("# Character sheet\n\n- Full name:\n"), 4);
    }

    /// A whole preset body counts only its prompts and headings, never the bullets.
    #[test]
    fn word_count_of_a_preset_is_all_prose() {
        let body = crate::note_templates::Preset::BeatSheet.rows()[0]
            .body
            .clone();
        let markup = body
            .split_whitespace()
            .filter(|t| !t.chars().any(char::is_alphanumeric))
            .count();
        assert!(markup > 0, "the body does contain markup tokens");
        assert_eq!(
            word_count(&body),
            body.split_whitespace().count() - markup,
            "every markup token is excluded and every prose token kept"
        );
    }
}
