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
//! Format and Template are, *for now*, rendered as [`SegmentedControl`]s (the
//! design's rich tile / radio-row variants are kept as comments for a later
//! pass). Location uses a [`FilePickerField`] with its embedded browse
//! affordance — no separate Browse button. The only runtime-computed string is
//! the "Will create …/<slug>.skrib" path preview (`bind_text` on the VM's derived
//! signal); every other string is `tr!`-localized.

use std::collections::HashMap;
use std::rc::Rc;

use bastyde::core::styles::PanelVariant;
use bastyde::i18n::{LocalizedString, localized};
use bastyde::prelude::*;
use bastyde::res;
use bastyde::widgets::{
    Button, ButtonVariant, ComboBox, Divider, Expand, FilePickerField, FilePickerKind, FixedSize,
    FormLayout, HStack, IconWidget, IconButton, Padding, Panel, RadioTile, RadioTileGroup,
    ScrollArea, Spacer, TextInput, TextWidget, TileLayout, VStack,
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
}

impl NewWorkPanel {
    pub fn new(app_ctx: Rc<AppContext>) -> Self {
        Self {
            vm: NewWorkViewModel::new(app_ctx),
            root_child: None,
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

    /// The `"<endonym> (<tag>)"` language dropdown, populated from the app's
    /// supported locales and bound to the VM's language tag. Endonyms are data
    /// (never translated), so the item labels use `localized(..)`, not `tr!`.
    /// A plain `ComboBox` — NOT `LanguageSwitcher`, which would change the app's
    /// own UI locale on select.
    fn language_combo(&self) -> ComboBox<String> {
        let locales = bastyde::i18n::current_supported_locales().unwrap_or_default();
        let tags: Vec<String> = locales.iter().map(|l| l.to_string()).collect();
        let labels: HashMap<String, String> = locales
            .iter()
            .map(|l| {
                let tag = l.to_string();
                let endonym = bastyde::i18n::language_endonym(l).unwrap_or_else(|| tag.clone());
                (tag.clone(), format!("{endonym} ({tag})"))
            })
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
                // this is the sole `lit!`/`bind_text` runtime string.
                TextWidget::new(lit!(""))
                    .bind_text(self.vm.target_path())
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
                    .child(FixedSize::new().bind_width(240.0).child(self.language_combo()))
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
                            .icon(Self::tile_icon(res!("assets/icons/new_work/empty-novel.svg")))
                            .title(tr!(new_work_template_empty_novel()))
                            .trailing(tr!(new_work_template_empty_novel_count())),
                    )
                    .tile(
                        RadioTile::new()
                            .icon(Self::tile_icon(res!("assets/icons/new_work/light-novel.svg")))
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
        // Scrollable form column (design body scrolls; `overflow:auto`).
        let body = ScrollArea::new().child(Padding::symmetric(20.0, 22.0).child(self.form()));

        // The footer's "Create Work" — reactively disabled (greyed, unclickable)
        // until the form is valid: `bind_enabled` tracks `can_create` (name +
        // location valid).
        let create_vm = self.vm.clone();
        let create_can = self.vm.can_create();

        let root = bati!(ctx =>
            FixedSize {
                bind_width: CARD_W
                bind_height: CARD_H
                Panel {
                    variant: PanelVariant::Raised
                    corner_radius: 10.0
                    padding: 0.0
                    VStack {
                        spacing: 0.0
                        // ── Header strip: title + close (mirrors Welcome). ──
                        Expand::horizontal {
                            FixedSize {
                                bind_height: 44.0
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
                            bind_height: 56.0
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
                                        bind_enabled: create_can
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
        assert_eq!((b.width, b.height), (CARD_W, CARD_H), "panel fills the card");
    }
}
