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

use teksilo::core::binding::BindingLevel;
use teksilo::data::ListModel;
use teksilo::tokens::Color;
use teksilo::widgets::{
    Badge, Button, ButtonVariant, ColorEdit, HStack, ListView, Padding, Spacer, StandardListItem,
    TextInput, Toast, VStack,
};

use crate::distraction_free::DistractionFreeThemesViewModel;
use crate::distraction_free::theme::DistractionFreeTheme;

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

    // ── One list, not two ──
    //
    // Verbatim the merge `export_styles` makes next door, for the same reason:
    // a built-in list capped at 180 px over a user list capped at 150, each
    // scrolling on its own above an editor, is three scroll regions where the
    // rows already say which kind they are. Every built-in row carries the
    // `Built-in` badge and offers only Duplicate; a user row offers Edit /
    // Export… / Delete. Built-ins are Rust values, so they are captured once and
    // always lead.
    let builtin_themes = vm.builtin_themes();
    let builtin_ids: std::collections::HashSet<String> =
        builtin_themes.iter().map(|t| t.id.clone()).collect();

    let all_themes = |vm: &DistractionFreeThemesViewModel,
                      builtin: &[DistractionFreeTheme]|
     -> Vec<DistractionFreeTheme> {
        let mut all = builtin.to_vec();
        all.extend(vm.user_themes());
        all
    };

    let model = ListModel::from_vec(all_themes(vm, &builtin_themes));
    {
        let model = model.clone();
        let vm = vm.clone();
        let selected = selected.clone();
        let builtin_themes = builtin_themes.clone();
        let builtin_ids = builtin_ids.clone();
        ctx.effect(&vm.changed_signal(), move |_| {
            let all = all_themes(&vm, &builtin_themes);
            // If the theme being edited was deleted, drop the editor selection.
            if let Some(cur) = selected.get()
                && !all
                    .iter()
                    .any(|t| t.id == cur && !builtin_ids.contains(&t.id))
            {
                selected.set(None);
            }
            model.replace_all(all);
        });
    }
    let lvm = vm.clone();
    let lsel = selected.clone();
    let lcur = current.clone();
    let list = ListView::new(model, move |_i, t: &DistractionFreeTheme, _sel| {
        if builtin_ids.contains(&t.id) {
            Box::new(builtin_row(&lvm, t, lcur.clone())) as Box<dyn Widget>
        } else {
            Box::new(user_row(&lvm, t, lsel.clone(), lcur.clone())) as Box<dyn Widget>
        }
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
        .child(group(tr!(settings_page_distraction_free_themes())))
        .child(Padding::symmetric(6.0, 4.0).child(toolbar))
        // `list_box`, not a hand-tuned `MaxSize::height`: the caps this replaces
        // had no relation to the height the pane actually offers, so the page
        // scrolled while the lists scrolled inside it.
        .child(Padding::symmetric(4.0, 0.0).child(crate::settings::fields::list_box(list)))
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
        // User-named, so unbounded — and a settings pane is measured under
        // `width: None`, where one over-wide child inflates the whole page.
        // See `binder::dock`.
        .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
        .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
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
        // See the note in `builtin_row` above.
        .label_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
        .subtitle_overflow(TextOverflow::Ellipsis(EllipsisMode::Trailing))
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
            .default_file_name(format!("{}.json", slugify(name, "theme")))
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

use crate::shared::slug::slugify;

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
                    .swatches(teksilo::widgets::color_picker::DEFAULT_SWATCHES.to_vec())
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
            // Named, so the form emits the `Role::Form` landmark assistive
            // technology navigates by — every sibling pane's form is named, and
            // an unnamed one demotes to a presentational group with no way in.
            .label(tr!(settings_themes_editor_group()))
            .label_gap(16.0)
            // 14, like the twenty-two other panes in this window. 12 here was the
            // only row rhythm in the settings tree that did not match its
            // neighbours.
            .row_spacing(14.0)
            .line(
                field_label(tr!(settings_themes_field_name())),
                // No `FixedSize::width`: a `FormLayout` places its field slot at
                // `field_col_width` regardless — see `fields::slider_field`.
                TextInput::new(name),
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

    use teksilo::core::widget_tree::WidgetTree;
    use teksilo::core::{LayoutContext, LayoutResponse, WidgetId};

    /// A private theme library per call, so the tests do not race each other's
    /// writes.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

    fn vm_with_a_user_theme() -> (DistractionFreeThemesViewModel, std::path::PathBuf) {
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("skrib-dfthemes-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        let svc = crate::models::DistractionFreeThemesService::open_at(dir.join("themes.toml"))
            .expect("a fresh library");
        let vm = DistractionFreeThemesViewModel::new(svc);
        let base = vm.builtin_themes().into_iter().next().expect("a built-in");
        vm.duplicate(&base, "copy").expect("duplicate");
        (vm, dir)
    }

    struct PaneHost {
        vm: Option<DistractionFreeThemesViewModel>,
        root: Option<WidgetId>,
    }
    impl std::fmt::Debug for PaneHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PaneHost").finish()
        }
    }
    impl Widget for PaneHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let vm = self.vm.take().expect("built once");
            let body = distraction_free_themes_pane(
                ctx,
                &vm,
                Signal::new(vm.builtin_themes()[0].id.clone()),
            );
            let id = ctx.add(body);
            self.root = Some(id);
            vec![id]
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

    /// One list carries both kinds, shipped ones first.
    ///
    /// The pane used to stack a built-in list capped at 180 px over a user list
    /// capped at 150, each scrolling inside a page that also scrolled. The merge
    /// is only sound while the two kinds stay tellable apart *by id*, which is
    /// what the row builder switches on.
    #[test]
    fn one_list_carries_the_shipped_themes_and_then_mine() {
        let (vm, dir) = vm_with_a_user_theme();
        let builtin = vm.builtin_themes();
        let all = vm.all_themes();
        assert!(
            all.len() > builtin.len(),
            "the merged list must hold the writer's copies too"
        );
        let ids: std::collections::HashSet<String> = builtin.iter().map(|t| t.id.clone()).collect();
        assert!(
            vm.user_themes().iter().all(|t| !ids.contains(&t.id)),
            "a duplicate must not answer to a built-in's id, or the merged list \
             would render it with Duplicate instead of Edit / Export… / Delete"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The pane fits in something like a pane.
    #[test]
    fn the_pane_fits_without_stacking_three_scroll_regions() {
        let (vm, dir) = vm_with_a_user_theme();
        let mut tree = WidgetTree::new();
        tree.add(PaneHost {
            vm: Some(vm),
            root: None,
        });
        let wanted = tree
            .measure_root_intrinsic(SizeProposal::with_width(crate::settings::fields::PANE_W))
            .expect("the pane is the tree's only root");
        assert!(
            wanted.height < 700.0,
            "the pane wants {} px against a ~431 px viewport",
            wanted.height
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

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
