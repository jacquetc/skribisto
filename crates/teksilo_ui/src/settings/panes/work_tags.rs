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

use teksilo::core::BindingLevel;
use teksilo::core::styles::{ComboBoxVariant, TextInputVariant};
use teksilo::data::SortFilterListModel;
use teksilo::prelude::*;
use teksilo::res;
use teksilo::tokens::{BorderRole, CornerRadius, SurfaceRole};
use teksilo::widgets::{
    BuiltInIcons, Button, ButtonVariant, Center, ColorEdit, ComboBox, Expand, FixedSize, HStack,
    IconButton, IconLocation, IconWidget, ListView, MaxSize, MenuItem, MenuList, MinSize, Padding,
    Panel, PopoverButton, RectWidget, SearchField, Shrinkable, Spacer, Switcher, TextInput,
    TextWidget, Toast, Toggle, VStack, ValidationState,
};

use crate::app_ids::HasWorkId;
use crate::models::TagRow;
use crate::note_templates::NoteTemplatesViewModel;
use crate::tags::{Preset, TagsViewModel, contrast};
use crate::toast_scope::ToastWorkExt;
use frontend::common::entities::BinderItemRole;

const NAME_COL: &str = "name";
const FILTER_FIELD_MAX_WIDTH: f32 = 260.0;
/// `ComboBox`'s own minimum width, mirrored here as the floor the row's two dropdowns
/// compress to. Below it the combo clamps anyway, so shrinking further would only make the
/// row lie about how much room it needs.
const COMBO_MIN_WIDTH: f32 = 120.0;
/// How far the row's two lower lines hang under the swatch + colour well above them.
const PICKERS_INDENT: f32 = 26.0;

/// The tag row's third line: *Creates in* ⟨folder⟩ · *Starting template* ⟨template⟩.
///
/// **Both dropdowns take an equal share of the leftover width**, rather than each
/// asking for its own content and then being compressed. Both are sized by words
/// the writer chose — a folder called "Personnages secondaires et lieux" makes the
/// first combo as wide as that title — and a `ComboBox` is rigid by default, so
/// the row used to overflow and push the trailing picker out of reach. `Shrinkable`
/// fixed the overflow but not the outcome: at the width this row is *actually*
/// given (about 561 px, see the tests below) two natural widths of a few hundred
/// pixels each leave a deficit large enough that both combos land on
/// `COMBO_MIN_WIDTH` — clamped at 120 px, truncating exactly the folder titles the
/// compression was introduced to protect, and worse in French.
///
/// An `Expand` with the default zero basis takes the labels out of the competition
/// and splits what is left in half, so each picker gets ~193 px instead of 120 and
/// the two agree on a width whatever the titles say. The `Shrinkable` stays inside
/// it as the floor for a pane narrower than this one.
fn pickers_line(
    creates_in: impl Widget + 'static,
    template: impl Widget + 'static,
) -> impl Widget + 'static {
    let label = |text: LocalizedString| {
        TextWidget::new(text)
            .style(TextStyleRole::Tiny)
            .color(TextRole::Secondary)
    };
    HStack::new()
        .spacing(8.0)
        .child(label(tr!(settings_tags_creates_in())))
        .child(
            Expand::horizontal().child(
                Shrinkable::new()
                    .min_width(COMBO_MIN_WIDTH)
                    .child(creates_in),
            ),
        )
        .child(label(tr!(settings_tags_template())))
        .child(
            Expand::horizontal()
                .child(Shrinkable::new().min_width(COMBO_MIN_WIDTH).child(template)),
        )
}
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

    // The list is a widget of its own so the template refresh has an id of its own to
    // rebuild: see [`TagList`].
    let list_card = TagList {
        vm: vm.clone(),
        templates: templates.clone(),
        filtered,
        root_child: None,
    };

    // Description lives on the add field's tooltip — no body-copy paragraph above the list.
    VStack::new()
        .spacing(16.0)
        .child(add_row(ctx, vm))
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

/// The bordered list of tags, as a widget of its own.
///
/// **Why a widget and not a builder function.** Each row's "Starting template" dropdown is
/// a `ComboBox::from_items`, which copies the list it is handed into a private model
/// nothing writes to again, so a template created on the sibling Templates page is
/// invisible here until the options are derived afresh. Deriving them afresh means a
/// `BindingLevel::Rebuild`, and a Rebuild needs a widget id.
///
/// `BuildContext::self_id()` inside [`work_tags_pane`] is **not** this pane: the pane is a
/// plain function handed the caller's context, and the caller is the `SettingsPanel`
/// itself. Binding there rebuilt the entire Settings window on every fire, which threw
/// away the pane's own filter query signal and its `SortFilterListModel` with it: the
/// filter box cleared and every filtered-out tag reappeared each time a tag was added,
/// deleted, a preset applied or a CSV imported (all of which push a `Work` update, which
/// is what the templates model bumps its version on). Rebuilding *this* id re-derives the
/// options and the rows, and leaves the query, the toolbar and the add field alone.
struct TagList {
    vm: TagsViewModel,
    templates: NoteTemplatesViewModel,
    filtered: SortFilterListModel<TagRow>,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for TagList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TagList").finish()
    }
}

impl Widget for TagList {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.templates.changed_signal().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Both resolved once per list build, not once per row: a project with forty tags
        // would otherwise walk the whole binder forty times to paint one dropdown each.
        let folders = folder_options(&self.vm.app_ctx(), &self.vm.ids());
        let template_rows = template_options(&self.templates);
        let list_vm = self.vm.clone();
        let list =
            ListView::from_source(self.filtered.clone(), move |_i, row: &TagRow, _selected| {
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
            let vm = self.vm.clone();
            vm.changed_signal().map(move |_| usize::from(vm.is_empty()))
        };
        let card = Panel::new()
            .background(SurfaceRole::Content)
            .border_color(BorderRole::Default)
            .border_width(1.0)
            .corner_radius(8.0)
            .padding(0.0)
            .child(
                // The floor goes on the card, not the list: a `Switcher` reports its active
                // child's size, and a virtualised `ListView` given unbounded height inside the
                // pane's own scroll reports ~nothing.
                MinSize::new(0.0, crate::settings::fields::LIST_MIN_HEIGHT).child(
                    Switcher::new(empty_idx)
                        .child(Expand::vertical().child(list))
                        .child(empty_state(&self.vm)),
                ),
            );
        let root = ctx.add(card);
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
            // A tag can be filed into a folder the writer has since trashed, and that
            // tag is still set: trashing removes nothing, it only flips `activated`
            // over the subtree, so `creates_in` still names the row. [`folder_options`]
            // cannot offer it, because nobody may *newly* pick a destination inside the
            // trash, so without the extra entry below the lookup misses and the row
            // falls through to "Ask me the first time" while the store says otherwise.
            //
            // Surfaced rather than cleared, deliberately. Clearing is the other honest
            // answer and it is worse: it throws away a choice the writer never revoked,
            // it happens behind their back the moment they open this pane, and
            // restoring the folder from the trash would not bring the filing back.
            // Naming it says what is true, and any live folder in the list replaces it
            // in one click.
            let (folders, shown) =
                row_destinations(&self.vm.app_ctx(), &self.folders, self.row.creates_in);
            let selected = Signal::new(shown);
            let vm = self.vm.clone();
            ComboBox::from_items(folders, selected, |o: &PickOption| lit!(o.label.clone()))
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
                    Padding::new(0.0, 0.0, 0.0, PICKERS_INDENT)
                        .child(pickers_line(creates_in, template)),
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
/// organising, not misusing.
///
/// Trashed rows are excluded by `ordered_flat_items`, and must be: a destination inside
/// the trash is one the capture flow refuses to file into. That is about what may be
/// *picked*, not about what is already stored, so a tag whose folder has since been
/// trashed gets its own extra entry from [`trashed_destination`] rather than being
/// misreported here as unset.
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

/// What one row's "New notes go to" dropdown offers, and the entry it starts on.
///
/// `offered` is the shared, live-folders-only list from [`folder_options`]; the pair this
/// returns is that list plus, when the row needs it, one extra entry naming its own
/// trashed destination. The selection is looked up in the widened list, so a tag whose
/// folder is in the trash starts on *that* row and not on "Ask me the first time".
///
/// Split out of the widget so the "trashed" case can be asserted against a real store:
/// the difference is one entry in a `ComboBox`'s item list, which a laid-out tree cannot
/// be asked about.
fn row_destinations(
    app_ctx: &frontend::AppContext,
    offered: &[PickOption],
    creates_in: Option<u64>,
) -> (Vec<PickOption>, Option<PickOption>) {
    let mut folders = offered.to_vec();
    if let Some(trashed) = trashed_destination(app_ctx, creates_in, &folders) {
        folders.push(trashed);
    }
    let shown = folders
        .iter()
        .find(|o| o.id == creates_in)
        .cloned()
        // A destination that no longer resolves *at all* — the folder was deleted for
        // good rather than trashed — has no title left to show and nothing to restore,
        // so the unset row is the truth here and not a fallback.
        .or_else(|| folders.first().cloned());
    (folders, shown)
}

/// This tag's own filing destination, when it is a folder [`folder_options`] cannot
/// offer because the writer has trashed it.
///
/// `None` in the three cases where the pane already tells the truth: the tag files
/// nowhere, its folder is live and therefore already in `offered`, or the id names no
/// row at all — a destination emptied out of the trash is genuinely gone, and there is
/// no title left to put in front of the writer.
///
/// One store read, and only for a row that is actually in this state.
fn trashed_destination(
    app_ctx: &frontend::AppContext,
    creates_in: Option<u64>,
    offered: &[PickOption],
) -> Option<PickOption> {
    let destination = creates_in?;
    if offered.iter().any(|o| o.id == Some(destination)) {
        return None;
    }
    let item = frontend::commands::binder_item_commands::get_binder_item(app_ctx, &destination)
        .ok()
        .flatten()?;
    if item.activated {
        return None;
    }
    let name = if item.title.trim().is_empty() {
        tr!(settings_tags_creates_in_untitled()).resolve_now()
    } else {
        item.title.clone()
    };
    Some(PickOption {
        id: Some(destination),
        label: tr!(settings_tags_creates_in_trashed(name = name)).resolve_now(),
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::app_ids::AppIds;
    use crate::models::WorkNoteTemplatesListModel;

    fn view_models() -> (
        Rc<frontend::AppContext>,
        TagsViewModel,
        NoteTemplatesViewModel,
    ) {
        let app_ctx = Rc::new(frontend::AppContext::new());
        let ids = AppIds::new();
        let tags = TagsViewModel::detached(app_ctx.clone(), ids.clone());
        let templates = NoteTemplatesViewModel::new(
            WorkNoteTemplatesListModel::new(app_ctx.clone(), ids.clone()),
            ids,
        );
        (app_ctx, tags, templates)
    }

    fn option(id: Option<u64>, label: &str) -> PickOption {
        PickOption {
            id,
            label: label.to_string(),
        }
    }

    /// Stands in for the Settings panel: it hosts the pane exactly as
    /// `settings::content::build` does, by calling it with its own `BuildContext`, and
    /// counts how many times it is itself built.
    struct Host {
        vm: TagsViewModel,
        templates: NoteTemplatesViewModel,
        builds: Rc<Cell<u32>>,
        child: Option<WidgetId>,
    }

    impl std::fmt::Debug for Host {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Host").finish()
        }
    }

    impl Widget for Host {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            self.builds.set(self.builds.get() + 1);
            let pane = work_tags_pane(ctx, &self.vm, &self.templates);
            let id = ctx.add(pane);
            self.child = Some(id);
            vec![id]
        }

        fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
            self.child
                .and_then(|id| ctx.child_size(id, proposal))
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
        }

        fn children(&self) -> Vec<WidgetId> {
            self.child.into_iter().collect()
        }
    }

    /// The template refresh must rebuild the list and nothing above it.
    ///
    /// The pane is a plain function handed its caller's `BuildContext`, so a
    /// `BindingLevel::Rebuild` on `ctx.self_id()` names the **caller**, the Settings
    /// panel itself. Firing it tore down the whole window: the pane's filter query signal and its
    /// `SortFilterListModel` were rebuilt from scratch, so the writer's filter text
    /// vanished and every filtered-out tag came back. The templates model bumps its
    /// version on any `Work` update, which is what adding or deleting a tag pushes, so
    /// this happened on ordinary palette edits and not only on a template edit.
    #[test]
    fn a_template_refresh_does_not_rebuild_the_settings_window() {
        let (app_ctx, vm, templates) = view_models();
        let builds = Rc::new(Cell::new(0));
        let mut tree = crate::test_support::tree_with_settings(&app_ctx);
        tree.add_boxed(Box::new(Host {
            vm,
            templates: templates.clone(),
            builds: builds.clone(),
            child: None,
        }));
        tree.layout(SizeProposal::exact(560.0, 480.0));
        assert_eq!(builds.get(), 1, "the host builds once to begin with");

        // Bumped the way `WorkNoteTemplatesListModel::refresh_for` bumps it, rather than
        // through `refresh()`: the mock model's `refresh` is a no-op, so driving it that
        // way would leave this test unable to fail under `--features mocks`. A tag added,
        // a tag deleted, a preset applied and a CSV imported all reach this signal, since
        // each pushes a `Work` update and the model bumps its version on every one.
        let version = templates.changed_signal();
        version.set(version.get().wrapping_add(1));
        tree.layout(SizeProposal::exact(560.0, 480.0));
        assert_eq!(
            builds.get(),
            1,
            "the Settings panel itself was rebuilt: the pane's filter query and its \
             filtered model went with it"
        );
    }

    /// The pane must say what the tag is actually set to.
    ///
    /// Trashing removes nothing: the folder keeps its uid, its title and its place in the
    /// binder, and the tag still names it. But it drops out of `folder_options`, because
    /// nobody may newly pick a destination inside the trash, so the row's lookup missed
    /// and fell through to the first entry, "Ask me the first time" — a pane telling the
    /// writer their tag was unset while the store said otherwise, and one that would flip
    /// back on its own the moment the folder was restored.
    #[test]
    fn a_tag_filed_into_a_trashed_folder_is_not_reported_as_unset() {
        use frontend::commands::{binder_commands, binder_item_commands, work_commands};
        use frontend::direct_access::{CreateBinderDto, CreateBinderItemDto, CreateWorkDto};
        use frontend::trash_management::TrashSelectionDto;

        let app_ctx = Rc::new(frontend::AppContext::new());
        let work = work_commands::create_orphan_work(&app_ctx, None, &CreateWorkDto::default())
            .expect("create work");
        let binder = binder_commands::create_binder(
            &app_ctx,
            None,
            &CreateBinderDto {
                name: "Research".into(),
                activated: true,
                ..Default::default()
            },
            work.id,
            -1,
        )
        .expect("create binder");
        let folder = binder_item_commands::create_binder_item(
            &app_ctx,
            None,
            &CreateBinderItemDto {
                status: None,
                title: "People".into(),
                role: BinderItemRole::Folder,
                activated: true,
                ..Default::default()
            },
            binder.id,
            -1,
        )
        .expect("create folder");

        frontend::commands::trash_management_commands::trash_selection(
            &app_ctx,
            None,
            &TrashSelectionDto {
                work_id: work.id,
                binder_ids: Vec::new(),
                binder_item_ids: vec![folder.id],
            },
        )
        .expect("trash the folder");

        // The premise, asserted rather than assumed: the row is still there and only
        // `activated` changed, which is why the tag still points at it.
        let stored = binder_item_commands::get_binder_item(&app_ctx, &folder.id)
            .expect("read the folder")
            .expect("trashing does not remove the row");
        assert!(
            !stored.activated,
            "the folder must actually be in the trash"
        );

        // What `folder_options` would hand this row: live folders only, so not this one.
        let offered = vec![option(None, "Ask me the first time")];
        let (folders, shown) = row_destinations(&app_ctx, &offered, Some(folder.id));

        let shown = shown.expect("the dropdown must start on something");
        assert_eq!(
            shown.id,
            Some(folder.id),
            "the row must start on the folder the tag is really set to, not on the unset entry"
        );
        assert!(
            shown.label.contains("People"),
            "the trashed destination must be named, so the writer can see what it is: {:?}",
            shown.label
        );
        assert_ne!(
            shown.label, offered[0].label,
            "and it must not read as the unset entry"
        );
        assert_eq!(
            folders.len(),
            2,
            "exactly one extra entry is added, and only for a row that needs it"
        );

        // A tag that files nowhere, and a tag pointing at a live folder, are untouched.
        let (live_only, unset) = row_destinations(&app_ctx, &offered, None);
        assert_eq!(live_only.len(), 1, "an unset tag adds no entry");
        assert_eq!(unset.map(|o| o.id), Some(None), "and still reads as unset");
    }

    /// The width one tag row is **actually** offered by the shipping window.
    ///
    /// Derived, never typed: the pane column is
    /// [`crate::settings::fields::PANE_W`], and the list card spends 1 px of it on
    /// each side for its border. The constant this replaces was `760.0`, which is
    /// 151 px more than this window has ever had — so the assertion below passed
    /// against headroom that does not exist, while in the real pane both of the
    /// row's dropdowns sat clamped at [`COMBO_MIN_WIDTH`].
    const ROW_W: f32 = crate::settings::fields::PANE_W - 2.0;

    /// A row with the longest titles the pane has to survive.
    fn crowded_row(vm: TagsViewModel) -> TagRowView {
        TagRowView {
            vm,
            row: TagRow {
                id: 1,
                name: "personnage".to_string(),
                color: DEFAULT_NEW_COLOR.to_string(),
                creates_in: Some(2),
                note_template: Some(3),
                ..TagRow::default()
            },
            folders: vec![
                option(None, "Not set"),
                option(
                    Some(2),
                    "Personnages secondaires, lieux et themes recurrents du deuxieme cycle",
                ),
            ],
            templates: vec![option(None, "None"), option(Some(3), "Fiche personnage")],
            root_child: None,
        }
    }

    /// A folder title is whatever the writer typed, and the combo showing it is rigid by
    /// default, so the row's natural width follows that title without limit. Rigid, the
    /// row overflows and the trailing "Starting template" control is pushed out of reach;
    /// the writer cannot get at the control that chooses the tag's note template at all.
    #[test]
    fn a_long_folder_title_does_not_push_the_template_picker_out_of_the_row() {
        let (app_ctx, vm, _templates) = view_models();
        let mut tree = crate::test_support::tree_with_settings(&app_ctx);
        tree.add_boxed(Box::new(crowded_row(vm)));
        tree.layout(SizeProposal::with_width(ROW_W));
        let wanted = tree
            .measure_root_intrinsic(SizeProposal::with_width(ROW_W))
            .expect("the row is the tree's only root");
        assert!(
            wanted.width <= ROW_W + 0.5,
            "the row wants {} px inside {ROW_W}: the trailing picker is pushed out of it",
            wanted.width
        );
    }

    /// Fitting is not the same as being usable.
    ///
    /// With both pickers rigid-then-compressed, the row *fitted* at 609 px and both
    /// dropdowns sat on their 120 px floor — truncating exactly the folder titles
    /// the compression exists to protect. The share the two now take is what this
    /// asserts, at the width the window really offers.
    #[test]
    fn both_pickers_get_a_real_share_of_the_row_not_their_floor() {
        let (app_ctx, vm, _templates) = view_models();
        let mut tree = crate::test_support::tree_with_settings(&app_ctx);
        let row = tree.add_boxed(Box::new(crowded_row(vm)));
        tree.layout(SizeProposal::with_width(ROW_W));

        // Descend the row: TagRowView → Padding → VStack → the third line's
        // Padding → the pickers `HStack`. Spelled out rather than searched for,
        // so a change to the row's shape fails here loudly instead of silently
        // measuring the wrong widget.
        let only = |tree: &teksilo::core::widget_tree::WidgetTree, id: WidgetId| {
            let kids = tree.children(id);
            assert_eq!(kids.len(), 1, "expected exactly one child of {id:?}");
            kids[0]
        };
        let body = only(&tree, only(&tree, row));
        let third_line = tree.children(body)[2];
        let pickers = tree.children(only(&tree, third_line));
        assert_eq!(pickers.len(), 4, "label, picker, label, picker");

        for slot in [pickers[1], pickers[3]] {
            let w = tree.bounds(slot).width;
            assert!(
                w > COMBO_MIN_WIDTH + 1.0,
                "a picker got {w} px inside {ROW_W}, i.e. its {COMBO_MIN_WIDTH} px floor: \
                 the folder title it shows is truncated in the shipping window"
            );
        }
        let (a, b) = (tree.bounds(pickers[1]).width, tree.bounds(pickers[3]).width);
        assert!(
            (a - b).abs() < 1.0,
            "the two pickers should share the leftover evenly, got {a} and {b}"
        );
    }
}
