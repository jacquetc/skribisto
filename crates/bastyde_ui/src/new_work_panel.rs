// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The New Work modal — create a work (name · format · location · language ·
//! template).
//!
//! Presented as an in-tree modal (see the `work.new` action in `app.rs`). A
//! single scrollable form column; the header/footer chrome mirrors the Welcome
//! panel (title strip + close button, full-width rule, bottom action bar).
//!
//! The body is a two-column [`FormLayout`] (auto-sized label column, `Role::Form`
//! a11y). All business logic lives on [`NewWorkViewModel`]; this view is thin —
//! it binds the VM's signals and forwards the footer buttons to its methods.
//!
//! Format and Template are, *for now*, rendered as `SegmentedControl`s (the
//! design's rich tile / radio-row variants are kept as comments for a later
//! pass). Location uses a [`FilePickerField`] with its embedded browse
//! affordance — no separate Browse button. The only runtime-computed string is
//! the "Will create `…/<slug>.skrib`" path preview (`text` on the VM's derived
//! signal); every other string is `tr!`-localized.

use std::collections::HashMap;
use std::rc::Rc;

use bastyde::core::styles::PanelVariant;
use bastyde::i18n::{LocalizedString, localized};
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::tooltip::TooltipContent;
use bastyde::widgets::{
    Button, ButtonVariant, ComboBox, Divider, Expand, FilePickerField, FilePickerKind, FixedSize,
    FormLayout, HStack, IconButton, IconWidget, Padding, Panel, RadioTile, RadioTileGroup,
    ScrollArea, Spacer, TextInput, TextWidget, TileLayout, Toggle, VStack,
};

use frontend::AppContext;

use crate::view_models::NewWorkViewModel;

/// The card size (matches the design's 600×680 dialog).
const CARD_W: f32 = 600.0;
const CARD_H: f32 = 680.0;

pub struct NewWorkPanel {
    /// Owns the form's signals for this modal session (created once in `new`).
    vm: NewWorkViewModel,
    root_child: Option<WidgetId>,
    /// The Work-name field, captured during `build` and handed to the modal
    /// pipeline by [`Widget::initial_focus_hint`] — see the note there.
    name_field: std::cell::Cell<Option<WidgetId>>,
}

impl NewWorkPanel {
    /// Presented over an already-open project (File ▸ New Work / Ctrl+N):
    /// creates the work in place, replacing this window's project.
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            vm: NewWorkViewModel::new(app_ctx),
            root_child: None,
            name_field: std::cell::Cell::new(None),
        }
    }

    /// Presented from the Launcher (`WelcomeViewModel::new_work`): creation is
    /// deferred to a freshly-opened project window, which then closes the
    /// Launcher — see [`crate::view_models::NewWorkViewModel::new_for_launcher`].
    pub fn new_for_launcher(
        app_ctx: Rc<AppContext>,
        factory: crate::windows::ProjectWindowFactory,
    ) -> Self {
        Self {
            vm: NewWorkViewModel::new_for_launcher(app_ctx, factory),
            root_child: None,
            name_field: std::cell::Cell::new(None),
        }
    }

    /// A left-column field label (dimmed, small).
    fn field_label(text: LocalizedString) -> TextWidget {
        TextWidget::new(text)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
    }

    /// A sub-field hint line (dimmed, small).
    fn hint(text: LocalizedString) -> TextWidget {
        TextWidget::new(text)
            .style(TextStyleRole::Small)
            .color(TextRole::Secondary)
    }

    /// The project language dropdown, populated from the **dictionary registry** (the languages
    /// a writer can spell-check in) — NOT the app's UI-translation locales, which are a
    /// different, unrelated list (a French-UI user may well write an English novel). Bound to
    /// the VM's language tag; a plain `ComboBox`, not `LanguageSwitcher` (which would change the
    /// app's own UI locale). Display names are data → `localized(..)`, never `tr!`.
    fn language_combo(&self) -> ComboBox<String> {
        let entries = crate::dictionary_registry::entries();
        let tags: Vec<String> = entries.iter().map(|e| e.id.clone()).collect();
        let labels: HashMap<String, String> = entries
            .iter()
            .map(|e| (e.id.clone(), format!("{} ({})", e.display_name, e.id)))
            .collect();
        ComboBox::from_items(tags, self.vm.language(), move |tag: &String| {
            let display = labels.get(tag).cloned().unwrap_or_else(|| tag.clone());
            localized(move || display.clone())
        })
        .placeholder(tr!(new_work_language()))
    }

    /// The reactive "Will create …" preview — the one runtime-computed string.
    fn path_preview(&self) -> impl Widget + 'static {
        HStack::new()
            .spacing(7.0)
            .child(
                TextWidget::new(tr!(new_work_will_create()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary),
            )
            .child(
                // The target path lives on the VM as a derived `Signal<String>`;
                // this is the sole `lit!`/`text` runtime string.
                TextWidget::new(lit!(""))
                    .text(self.vm.target_path())
                    .style(TextStyleRole::Small)
                    .color(TextRole::Accent),
            )
    }

    /// The two-column form body.
    fn form(&self) -> impl Widget + 'static {
        let vm = &self.vm;
        FormLayout::new()
            .label(tr!(new_work_title()))
            .label_gap(16.0)
            .row_spacing(18.0)
            // ── Work name (required — inline error while blank) ────────────
            .line(
                Self::field_label(tr!(new_work_name())),
                TextInput::new(vm.name())
                    .placeholder(tr!(new_work_name_placeholder()))
                    .validation(vm.name_validation()),
            )
            // ── Format: two selectable cards + the "convert later" note ────
            .line(
                Self::field_label(tr!(new_work_format())),
                VStack::new()
                    .spacing(8.0)
                    .child(
                        RadioTileGroup::new(vm.format_idx())
                            .layout(TileLayout::Row)
                            .tile(
                                RadioTile::new()
                                    .icon(Self::tile_icon(res!(
                                        "assets/icons/new_work/single-file.svg"
                                    )))
                                    .title(tr!(new_work_single_file()))
                                    .description(tr!(new_work_single_file_desc())),
                            )
                            .tile(
                                RadioTile::new()
                                    .icon(Self::tile_icon(res!("assets/icons/new_work/bundle.svg")))
                                    .title(tr!(new_work_bundle()))
                                    .description(tr!(new_work_bundle_desc())),
                            ),
                    )
                    .child(Self::hint(tr!(new_work_convert_later()))),
            )
            // ── Location (FilePicker, browse embedded) + path preview ──────
            // Same inline error indicator as the name field: the folder must
            // exist and be writable.
            .line(
                Self::field_label(tr!(new_work_location())),
                VStack::new()
                    .spacing(6.0)
                    .child(
                        FilePickerField::new(vm.location())
                            .kind(FilePickerKind::PickFolder)
                            .validation(vm.location_validation()),
                    )
                    .child(self.path_preview()),
            )
            .full_width(Divider::new())
            // ── Default language ──────────────────────────────────────────
            .line(
                Self::field_label(tr!(new_work_language())),
                VStack::new()
                    .spacing(6.0)
                    .child(FixedSize::new().width(240.0).child(self.language_combo()))
                    .child(Self::hint(tr!(new_work_language_hint()))),
            )
            .full_width(Divider::new())
            // ── Template: a vertical list of compact rows (radio · icon ·
            // title · trailing count), one per NewWorkTemplate. ────────────
            .line(
                Self::field_label(tr!(new_work_template())),
                RadioTileGroup::new(vm.template_idx())
                    .layout(TileLayout::Vertical)
                    .tile(
                        RadioTile::new()
                            .icon(Self::tile_icon(res!("assets/icons/new_work/none.svg")))
                            .title(tr!(new_work_template_none()))
                            .trailing(tr!(new_work_template_none_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(Self::tile_icon(res!(
                                "assets/icons/new_work/empty-novel.svg"
                            )))
                            .title(tr!(new_work_template_empty_novel()))
                            .trailing(tr!(new_work_template_empty_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(Self::tile_icon(res!(
                                "assets/icons/new_work/light-novel.svg"
                            )))
                            .title(tr!(new_work_template_light_novel()))
                            .trailing(tr!(new_work_template_light_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(Self::tile_icon(res!("assets/icons/new_work/novel.svg")))
                            .title(tr!(new_work_template_novel()))
                            .trailing(tr!(new_work_template_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(Self::tile_icon(res!("assets/icons/new_work/notebook.svg")))
                            .title(tr!(new_work_template_notebook()))
                            .trailing(tr!(new_work_template_notebook_count())),
                    ),
            )
            // ── ChapterScene mode: write directly in chapters (novel templates
            // only; greyed otherwise). The rich tooltip explains both modes. ──
            .full_width(
                Toggle::new(vm.chapter_scene())
                    .label(tr!(new_work_chapter_scene()))
                    .enabled(vm.chapter_scene_applicable())
                    .rich_tooltip_content(Self::chapter_scene_tooltip()),
            )
    }

    /// The rich tooltip for the "write directly in chapters" toggle: a primary
    /// body contrasting the two encodings, plus a `more` accordion with the
    /// writing-model rationale. Inline content (no boot-time registry needed).
    fn chapter_scene_tooltip() -> TooltipContent {
        TooltipContent::new("new-work-chapter-scene", tr!(new_work_chapter_scene_tip()))
            .with_more(tr!(new_work_chapter_scene_tip_more()))
    }

    /// A 16 dp tile icon (monochrome, follows the theme text color).
    fn tile_icon(icon: &'static bastyde::canvas::svg::SvgIcon) -> IconWidget {
        IconWidget::from_svg_icon(icon).icon_size(16.0)
    }
}

impl std::fmt::Debug for NewWorkPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewWorkPanel").finish()
    }
}

impl Widget for NewWorkPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Build the form first so its first focusable descendant — the Work name
        // field — can be captured for `initial_focus_hint`. Without it the modal
        // pipeline falls back to `first_focusable_descendant` of the whole panel,
        // which is the header's close button: the dialog opened with the X focused
        // and typing did nothing until the user clicked the field.
        let form_id = ctx.add(self.form());
        self.name_field.set(ctx.first_focusable_descendant(form_id));

        // Scrollable form column (design body scrolls; `overflow:auto`).
        let body = ScrollArea::new().child(Padding::symmetric(20.0, 22.0).child_id(form_id));

        // The footer's "Create Work" — reactively disabled (greyed, unclickable)
        // until the form is valid: `enabled` tracks `can_create` (name +
        // location valid).
        let create_vm = self.vm.clone();
        let create_can = self.vm.can_create();

        let root = bati!(ctx => FixedSize {
                width: CARD_W
                height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        // ── Header strip: title + close (mirrors Welcome). ──
                        Expand::horizontal {
                            FixedSize {
                                height: 44.0
                                Padding::symmetric(8.0, 14.0) {
                                    HStack {
                                        spacing: 8.0
                                        Expand::horizontal {
                                            TextWidget::new(tr!(new_work_title())) {
                                                style: TextStyleRole::Small
                                                color: TextRole::Secondary
                                            }
                                        }
                                        IconButton::clear() {
                                            tooltip: tr!(new_work_close())
                                            on_activate_fn: |ctx| ctx.dismiss_modal()
                                        }
                                    }
                                }
                            }
                        }
                        Expand::horizontal {
                            Divider
                        }
                        // ── Scrollable form body (fills the slack). ─────────
                        Expand::vertical {
                            child: body
                        }
                        Expand::horizontal {
                            Divider
                        }
                        // ── Footer: Cancel · Create Work (right-aligned). ───
                        FixedSize {
                            height: 56.0
                            Padding::symmetric(10.0, 22.0) {
                                HStack {
                                    spacing: 9.0
                                    Spacer
                                    Button::new(tr!(new_work_cancel())) {
                                        variant: ButtonVariant::Plain
                                        on_activate_fn: |ctx| ctx.dismiss_modal()
                                    }
                                    Button::new(tr!(new_work_create())) {
                                        variant: ButtonVariant::Filled
                                        enabled: create_can
                                        on_activate_fn: move |ctx| create_vm.create(ctx)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        );
        self.root_child = Some(root);
        vec![root]
    }

    /// Open with the Work name field focused, so the dialog is typeable the moment
    /// it appears.
    ///
    /// The modal pipeline's fallback is `first_focusable_descendant` of the whole
    /// panel, and this panel draws its own header chrome (title strip + close X)
    /// *above* the form — so the fallback picked the close button. Every field is
    /// below it, which meant New Work opened focused on "dismiss me" and swallowed
    /// whatever the user typed first.
    fn initial_focus_hint(&self) -> Option<WidgetId> {
        self.name_field.get()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Delegate to the fixed-size root (bounds the greedy inner `Expand`s);
        // delegating to the inner `Panel` would fill the window — the
        // modal-centering trap noted in `WelcomePanel`.
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::core::widget_tree::WidgetTree;

    /// The whole panel — header, the `FormLayout` body with its
    /// `SegmentedControl`s / `FilePickerField` / `ComboBox`, and the footer —
    /// must build and lay out headlessly without panicking. This exercises the
    /// full widget tree (any wrong builder/DSL usage would panic here), and the
    /// panel's own `layout_response` must report the fixed card size the modal
    /// host uses to size/centre it (not zero, not a stretch).
    #[test]
    fn panel_builds_and_lays_out() {
        let ctx = Rc::new(AppContext::new());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(NewWorkPanel::new(ctx)));
        // Lay out at the modal's card size; every child (FormLayout rows, the
        // SegmentedControls, FilePickerField, ComboBox, ScrollArea, footer) must
        // build and place without panicking.
        tree.layout(SizeProposal::exact(CARD_W, CARD_H));
        let b = tree.bounds(id);
        assert_eq!(
            (b.width, b.height),
            (CARD_W, CARD_H),
            "panel fills the card"
        );
    }
}
