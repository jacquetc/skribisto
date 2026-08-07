// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Editor ▸ Distraction-free themes — the theme library.
//!
//! The managed-collection shape the app's other library panes already share
//! (`export_styles`, `work_tags`, `user_dictionary`): a read-only **Built-in**
//! section offering only *Duplicate*, a **My themes** section with Edit / Export…
//! / Delete, and an Import… action above it. Built-ins are Rust values, so a
//! theme the app shipped can be copied but never edited away.
//!
//! Picking a theme writes an id into an ordinary setting; the surface resolves
//! it — see `DistractionFreeThemesViewModel`.

use bastyde::core::binding::BindingLevel;
use bastyde::data::ListModel;
use bastyde::prelude::*;
use bastyde::tokens::Color;
use bastyde::widgets::{
    Badge, Button, ButtonVariant, ColorEdit, HStack, ListView, MaxSize, Padding, Spacer,
    StandardListItem, TextInput, Toast, VStack,
};

use crate::distraction_free::theme::DistractionFreeTheme;
use crate::view_models::DistractionFreeThemesViewModel;

#[allow(unused_imports)]
use super::super::*;

/// The whole pane body. The caller wraps it in `pane_frame`.
pub(in crate::settings) fn distraction_free_themes_pane(
    ctx: &mut BuildContext,
    vm: &DistractionFreeThemesViewModel,
    current: Signal<String>,
) -> impl Widget {
    // The user theme loaded in the editor below (by id).
    let selected: Signal<Option<String>> = Signal::new(None);

    // ── Built-in themes (static) ──
    let builtin_model = ListModel::from_vec(vm.builtin_themes());
    let bvm = vm.clone();
    let bcur = current.clone();
    let builtin_list = ListView::new(builtin_model, move |_i, t: &DistractionFreeTheme, _sel| {
        Box::new(builtin_row(&bvm, t, bcur.clone()))
    })
    .auto_item_height(52.0);

    // ── User themes (live: repopulated on every library bump) ──
    let user_model = ListModel::from_vec(vm.user_themes());
    {
        let model = user_model.clone();
        let vm = vm.clone();
        let selected = selected.clone();
        ctx.effect(&vm.changed_signal(), move |_| {
            let rows = vm.user_themes();
            // If the theme being edited was deleted, drop the editor selection.
            if let Some(cur) = selected.get()
                && !rows.iter().any(|t| t.id == cur)
            {
                selected.set(None);
            }
            model.replace_all(rows);
        });
    }
    let uvm = vm.clone();
    let usel = selected.clone();
    let ucur = current.clone();
    let user_list = ListView::new(user_model, move |_i, t: &DistractionFreeTheme, _sel| {
        Box::new(user_row(&uvm, t, usel.clone(), ucur.clone()))
    })
    .auto_item_height(52.0);

    let import_vm = vm.clone();
    let import_sel = selected.clone();
    let toolbar = HStack::new().spacing(8.0).child(Spacer::new()).child(
        Button::new(tr!(settings_themes_import()))
            .variant(ButtonVariant::Tinted)
            .on_activate_fn(move |c| present_import(c, import_vm.clone(), import_sel.clone())),
    );

    VStack::new()
        .spacing(6.0)
        .child(group(tr!(settings_themes_builtin())))
        // `MaxSize::height`, not `MinSize`: as a minimum the list grows to fit
        // and swallows the pane — the same trap `export_styles` documents.
        .child(Padding::symmetric(4.0, 0.0).child(MaxSize::height(180.0).child(builtin_list)))
        .child(Padding::new(14.0, 0.0, 0.0, 0.0).child(group(tr!(settings_themes_user()))))
        .child(Padding::symmetric(6.0, 4.0).child(toolbar))
        .child(Padding::symmetric(4.0, 0.0).child(MaxSize::height(150.0).child(user_list)))
        .child(Padding::new(14.0, 0.0, 0.0, 0.0).child(group(tr!(settings_themes_editor_group()))))
        .child(ThemeEditor::new(vm.clone(), selected))
}

/// "Use" writes the id into the setting the surface reads.
fn use_button(id: &str, current: Signal<String>) -> impl Widget {
    let id_for_click = id.to_string();
    let id_for_state = id.to_string();
    Button::new(tr!(settings_themes_use()))
        .variant(ButtonVariant::Plain)
        // Disabled on the theme already in force, so the button doubles as the
        // answer to "which one am I using?" without a second affordance.
        .enabled(current.map(move |c| c.as_str() != id_for_state.as_str()))
        .on_activate_fn(move |_c| current.set(id_for_click.clone()))
}

fn builtin_row(
    vm: &DistractionFreeThemesViewModel,
    theme: &DistractionFreeTheme,
    current: Signal<String>,
) -> impl Widget {
    let vm = vm.clone();
    let base_id = theme.id.clone();
    let duplicate = Button::new(tr!(settings_themes_duplicate()))
        .variant(ButtonVariant::Plain)
        .on_activate_fn(move |_c| {
            if let Some(base) = vm.builtin_themes().into_iter().find(|t| t.id == base_id) {
                vm.duplicate(&base, &tr!(settings_themes_copy_suffix()).resolve_now());
            }
        });
    StandardListItem::new(lit!(theme.name.clone()))
        .subtitle(lit!(swatch_line(theme)))
        .trailing_slot(
            HStack::new()
                .spacing(8.0)
                .child(Badge::new(tr!(settings_themes_builtin_badge())))
                .child(use_button(&theme.id, current))
                .child(duplicate),
        )
}

fn user_row(
    vm: &DistractionFreeThemesViewModel,
    theme: &DistractionFreeTheme,
    selected: Signal<Option<String>>,
    current: Signal<String>,
) -> impl Widget {
    let id = theme.id.clone();
    let edit = {
        let selected = selected.clone();
        let id = id.clone();
        Button::new(tr!(settings_themes_edit()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |_c| selected.set(Some(id.clone())))
    };
    let export = {
        let vm = vm.clone();
        let id = id.clone();
        let name = theme.name.clone();
        Button::new(tr!(settings_themes_export()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |c| present_export(c, vm.clone(), &id, &name))
    };
    let delete = {
        let vm = vm.clone();
        let id = id.clone();
        Button::new(tr!(settings_themes_delete()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |_c| vm.remove(&id))
    };
    let id_hl = id.clone();
    StandardListItem::new(lit!(theme.name.clone()))
        .subtitle(lit!(swatch_line(theme)))
        .selected(selected.map(move |s| s.as_deref() == Some(id_hl.as_str())))
        .trailing_slot(
            HStack::new()
                .spacing(6.0)
                .child(use_button(&theme.id, current))
                .child(edit)
                .child(export)
                .child(delete),
        )
}

/// A row's subtitle: the colours, and a legibility warning when a pairing falls
/// below WCAG AA.
///
/// A **warning, never a refusal** — the same call the tags pane makes about a
/// duplicate name. A writer who wants pale grey on cream for an hour is making a
/// choice, not a mistake; the app's job is to be honest about it, not to veto it.
///
/// It names *which* pairing fell short, because the subtitle shows only the page
/// and the ink: a bare "low contrast" on a theme whose page and ink are visibly
/// fine — because it is the caret band that swallows the prose — is a warning
/// about two colours the writer can see are legible, and reads as a bug.
fn swatch_line(t: &DistractionFreeTheme) -> String {
    let base = format!("{} · {}", t.editor_background, t.editor_text);
    let warning = match contrast_warning(t) {
        ContrastWarning::None => return base,
        ContrastWarning::Surfaces => tr!(settings_themes_low_contrast()),
        ContrastWarning::Band => tr!(settings_themes_low_contrast_band()),
    };
    format!("{base} — {}", warning.resolve_now())
}

/// Which legibility warning a theme has earned, if any. Split out from
/// [`swatch_line`] so the choice is testable without a locale bundle — the
/// wording is presentation, the choice is the behaviour.
#[derive(Debug, PartialEq, Eq)]
enum ContrastWarning {
    None,
    /// The page against the prose, or the control strip against the margin.
    Surfaces,
    /// Both of those are fine and the caret band is what hides the prose.
    Band,
}

fn contrast_warning(t: &DistractionFreeTheme) -> ContrastWarning {
    use crate::distraction_free::theme::WCAG_AA_BODY;
    // Surfaces first: they are the pairing a writer stares at for hours, and a
    // theme failing both should be told about the bigger fault.
    if t.prose_contrast() < WCAG_AA_BODY || t.widget_contrast() < WCAG_AA_BODY {
        ContrastWarning::Surfaces
    } else if t.caret_band_contrast() < WCAG_AA_BODY {
        ContrastWarning::Band
    } else {
        ContrastWarning::None
    }
}

// ── JSON import / export (file dialogs) ──

fn present_import(
    ctx: &mut EventContext,
    vm: DistractionFreeThemesViewModel,
    selected: Signal<Option<String>>,
) {
    let req = crate::models::dialog_start_in(
        ctx,
        crate::models::FolderPurpose::DataInterchange,
        FileDialogRequest::pick_file()
            .title(tr!(settings_themes_import()))
            .add_filter(tr!(settings_themes_json_filter()).resolve_now(), &["json"]),
    );
    let _ = ctx.pick_file(req, move |res, ectx| {
        if let FileDialogResult::File(Some(path)) = res {
            crate::models::remember_dialog_file(
                ectx,
                crate::models::FolderPurpose::DataInterchange,
                &path,
            );
            match vm.import_from(&path) {
                Ok(t) => {
                    selected.set(Some(t.id.clone()));
                    // Broadcast: the theme library is app-wide, so every open
                    // window's picker gained an entry.
                    ectx.show_toast(
                        Toast::info(tr!(settings_themes_imported()))
                            .body(lit!(t.name.clone()))
                            .auto_dismiss_after(std::time::Duration::from_secs(4))
                            .broadcast(),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_themes_import_failed()))
                            .body(lit!(format!("{e:#}")))
                            .auto_dismiss_after(std::time::Duration::from_secs(6))
                            .broadcast(),
                    );
                }
            }
        }
    });
}

fn present_export(
    ctx: &mut EventContext,
    vm: DistractionFreeThemesViewModel,
    id: &str,
    name: &str,
) {
    let id = id.to_string();
    let req = crate::models::dialog_start_in(
        ctx,
        crate::models::FolderPurpose::DataInterchange,
        FileDialogRequest::save_file()
            .title(tr!(settings_themes_export()))
            .default_file_name(format!("{}.json", slugify(name)))
            .add_filter(tr!(settings_themes_json_filter()).resolve_now(), &["json"]),
    );
    let _ = ctx.save_file(req, move |res, ectx| {
        if let FileDialogResult::Saved(Some(path)) = res {
            crate::models::remember_dialog_file(
                ectx,
                crate::models::FolderPurpose::DataInterchange,
                &path,
            );
            let mut target = path.clone();
            if target.extension().and_then(|e| e.to_str()) != Some("json") {
                target.set_extension("json");
            }
            // Local, not broadcast: writing a file out changes nothing another
            // window could care about.
            match vm.export_to(&target, &id) {
                Ok(()) => {
                    ectx.show_toast(
                        Toast::info(tr!(settings_themes_exported()))
                            .body(lit!(target.to_string_lossy().into_owned()))
                            .auto_dismiss_after(std::time::Duration::from_secs(4)),
                    );
                }
                Err(e) => {
                    ectx.show_toast(
                        Toast::error(tr!(settings_themes_export_failed()))
                            .body(lit!(format!("{e:#}")))
                            .auto_dismiss_after(std::time::Duration::from_secs(6)),
                    );
                }
            }
        }
    });
}

fn slugify(name: &str) -> String {
    let s: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let trimmed = s.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "theme".to_string()
    } else {
        trimmed
    }
}

// ── The theme editor (reactive on the selection) ──

/// The name + five colour fields for the selected user theme.
///
/// A real `impl Widget` rebinding only on the selection, following `TagRowView`:
/// `ColorEdit` and `TextInput` have no change callback, so each field is written
/// back through a `ctx.effect` — and rebuilding on every keystroke would fight
/// the writer's caret.
struct ThemeEditor {
    vm: DistractionFreeThemesViewModel,
    selected: Signal<Option<String>>,
    child: Option<WidgetId>,
}

impl ThemeEditor {
    fn new(vm: DistractionFreeThemesViewModel, selected: Signal<Option<String>>) -> Self {
        Self {
            vm,
            selected,
            child: None,
        }
    }
}

impl std::fmt::Debug for ThemeEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ThemeEditor").finish()
    }
}

impl Widget for ThemeEditor {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let self_id = ctx.self_id();
        self.selected
            .bind_to(self_id, ctx.binding_registry(), BindingLevel::Rebuild);

        let Some(theme) = self
            .selected
            .get()
            .and_then(|id| self.vm.user_themes().into_iter().find(|t| t.id == id))
        else {
            // Nothing selected: a hint rather than an empty hole, so the section
            // heading above it does not read as a broken control.
            let hint =
                TextWidget::new(tr!(settings_themes_editor_empty())).color(TextRole::Secondary);
            self.child = Some(ctx.add(Padding::symmetric(6.0, 4.0).child(hint)));
            return self.child.into_iter().collect();
        };

        let name = Signal::new(theme.name.clone());

        // One write-back per field: read the **stored** theme fresh, apply this
        // one field, persist. Mutating a captured clone instead would let two
        // fields edited in quick succession clobber each other.
        let field = |vm: &DistractionFreeThemesViewModel,
                     id: &str,
                     apply: fn(&mut DistractionFreeTheme, String)|
         -> Box<dyn Fn(String)> {
            let vm = vm.clone();
            let id = id.to_string();
            Box::new(move |v: String| {
                if let Some(mut stored) = vm.user_themes().into_iter().find(|t| t.id == id) {
                    apply(&mut stored, v);
                    vm.update(&stored);
                }
            })
        };

        // `ColorEdit` is the trigger+popover shape the tags pane uses, over a
        // `Signal<Color>`; the stored value is a hex string, so each one bridges
        // on close — which is also the natural commit point (one write per
        // choice, not one per drag of the picker).
        //
        // `alpha` is per-field, not a constant: the caret band is the one colour
        // here that sits *behind* prose on a page whose colour this same theme
        // fixes, so a writer may reasonably want the paper to show through it.
        // The other four are surfaces and text — translucency there would just
        // let the app palette leak into a theme that is meant to replace it.
        let color_row = |ctx: &mut BuildContext,
                         color: Color,
                         alpha: bool,
                         write: Box<dyn Fn(String)>|
         -> WidgetId {
            let sig = Signal::new(color);
            let s2 = sig.clone();
            ctx.add(
                ColorEdit::new(sig)
                    .alpha_enabled(alpha)
                    .swatches(bastyde::widgets::color_picker::DEFAULT_SWATCHES.to_vec())
                    .on_close(move || write(s2.get().to_hex_lower(alpha))),
            )
        };
        let paper_id = color_row(
            ctx,
            Color::from_hex(&theme.editor_background),
            false,
            field(&self.vm, &theme.id, |t, v| t.editor_background = v),
        );
        let ink_id = color_row(
            ctx,
            Color::from_hex(&theme.editor_text),
            false,
            field(&self.vm, &theme.id, |t, v| t.editor_text = v),
        );
        let general_id = color_row(
            ctx,
            Color::from_hex(&theme.general_background),
            false,
            field(&self.vm, &theme.id, |t, v| t.general_background = v),
        );
        let widget_id = color_row(
            ctx,
            Color::from_hex(&theme.widget_text),
            false,
            field(&self.vm, &theme.id, |t, v| t.widget_text = v),
        );
        // Seeded from `caret_band_color()`, not the raw field: a theme duplicated
        // before this axis existed stores nothing here, and showing the writer
        // the black that a bare `from_hex("")` yields would offer them a swatch
        // the mode does not actually paint. They see the derived tint, and the
        // first edit writes it down.
        let band_id = color_row(
            ctx,
            theme.caret_band_color(),
            true,
            field(&self.vm, &theme.id, |t, v| t.caret_band = v),
        );
        {
            let write = field(&self.vm, &theme.id, |t, v| t.name = v);
            ctx.effect(&name, move |v: &String| write(v.clone()));
        }

        // `line_ids` takes both sides as ids, so the labels are added here too.
        let paper_label = ctx.add(field_label(tr!(settings_themes_field_paper())));
        let ink_label = ctx.add(field_label(tr!(settings_themes_field_ink())));
        let general_label = ctx.add(field_label(tr!(settings_themes_field_general())));
        let widget_label = ctx.add(field_label(tr!(settings_themes_field_widget_text())));
        let band_label = ctx.add(field_label(tr!(settings_themes_field_caret_band())));

        let form = FormLayout::new()
            .label_gap(16.0)
            .row_spacing(12.0)
            .line(
                field_label(tr!(settings_themes_field_name())),
                FixedSize::new().width(240.0).child(TextInput::new(name)),
            )
            .line_ids(paper_label, paper_id)
            .line_ids(ink_label, ink_id)
            .line_ids(general_label, general_id)
            .line_ids(widget_label, widget_id)
            .line_ids(band_label, band_id);
        self.child = Some(ctx.add(form));
        self.child.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::distraction_free::theme::builtin_themes;

    #[test]
    fn a_shipped_theme_earns_no_warning() {
        for t in builtin_themes() {
            assert_eq!(contrast_warning(&t), ContrastWarning::None, "`{}`", t.id);
        }
    }

    /// The row's subtitle shows the page and the ink. When those two are the
    /// problem, "low contrast" beside them is self-explanatory.
    #[test]
    fn an_unreadable_page_earns_the_surfaces_warning() {
        let mut t = builtin_themes()[0].clone();
        t.editor_text = t.editor_background.clone();
        assert_eq!(contrast_warning(&t), ContrastWarning::Surfaces);
    }

    /// …and when they are *not*, it is not. A band pushed to the colour of the
    /// prose leaves the two swatches in the subtitle demonstrably legible, so a
    /// bare "low contrast" there would be a warning about the wrong colours.
    #[test]
    fn a_band_that_hides_the_prose_earns_its_own_warning_not_the_surfaces_one() {
        let mut t = builtin_themes()[0].clone();
        t.caret_band = t.editor_text.clone();
        assert_eq!(contrast_warning(&t), ContrastWarning::Band);
        assert!(
            swatch_line(&t).starts_with(&format!("{} · {}", t.editor_background, t.editor_text)),
            "the swatches must still be shown — they are what the writer is \
             being told is fine"
        );
    }

    /// A theme that fails both is told about the surfaces: the band is the
    /// smaller fault, and fixing the page may well fix it too.
    #[test]
    fn failing_both_reports_the_surfaces() {
        let mut t = builtin_themes()[0].clone();
        t.editor_text = t.editor_background.clone();
        t.caret_band = t.editor_text.clone();
        assert_eq!(contrast_warning(&t), ContrastWarning::Surfaces);
    }
}
