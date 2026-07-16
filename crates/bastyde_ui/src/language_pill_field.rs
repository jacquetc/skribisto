// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `LanguagePillField` — the reusable language selector: a wrapping flow of pill chips (one
//! per language of the text) ending in a "+" popover. Used by both the Settings ▸ Work ▸
//! Language pane and the Inspector, over any `dict_language` string.
//!
//! Each pill shows the language's **endonym** and BCP-47 tag. Clicking a pill toggles its
//! green check — a **session** mute of that language's spell-check (never persisted, never in
//! `dict_language`). Hovering reveals a trailing **×** that removes the language from the list.
//! The "+" opens a menu of the languages Skribisto knows, adding one to the list.
//!
//! It never knows *whose* list it edits: the caller supplies a `value` mirror signal and a
//! `set` writer (the Inspector writes a `BinderItem`, the Settings pane writes the `Work`).
//! Add/remove change `dict_language` (persisted); the mute toggle does not.
//!
//! Accessibility: the flow is a `Role::List`, each pill a focusable `Role::ListItem`; the pill
//! toggles its mute on activation and the "+" opens its menu — keyboard-reachable throughout.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::core::accesskit::Role;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::tokens::{BorderRole, CornerRadius};
use bastyde::widgets::{
    Center, IconButton, IconWidget, MenuItem, MenuList, MinSize, Padding, PopoverIconButton,
    RectWidget, TextWidget, Wrap, ZStack,
};

use std::collections::HashMap;

use crate::dictionary_registry;
use crate::models::OpenDocsStore;
use crate::spellcheck::SpellcheckService;
use crate::view_models::DictionariesViewModel;
use skribisto_model::language;

/// A writer for a new `dict_language` list — the caller persists it (and mirrors it into the
/// `value` signal). Takes an `EventContext` so it can run a backend command.
pub type SetLanguages = Rc<dyn Fn(String, &mut EventContext)>;

/// The pill-flow language field.
pub struct LanguagePillField {
    /// The current list (a local mirror the caller keeps in sync with the backend).
    value: Signal<String>,
    /// Persist a new list.
    set: SetLanguages,
    /// The mute state + a version signal to rebuild the checks on.
    spell: SpellcheckService,
    /// When `value` is empty, the list inherited from the Book/Work (shown, and materialised on
    /// the first edit). `None` for the Work-level field, which inherits from nothing.
    inherited: Option<String>,
    root_child: Option<WidgetId>,
}

impl LanguagePillField {
    pub fn new(
        value: Signal<String>,
        set: SetLanguages,
        spell: SpellcheckService,
        inherited: Option<String>,
    ) -> Self {
        Self {
            value,
            set,
            spell,
            inherited,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for LanguagePillField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguagePillField").finish()
    }
}

/// Re-attach the spell-checker to every open document — after a mute toggle or a list edit.
fn reattach(ctx: &mut EventContext) {
    if let Some(store) = ctx.app_state::<OpenDocsStore>() {
        store.attach_all();
    }
}

/// The base language name — the display name with any region/variant qualifier stripped:
/// `English (United States)` → `English`, `Français — classique` → `Français`.
fn base_name(display: &str) -> &str {
    if let Some(i) = display.find(" (") {
        &display[..i]
    } else if let Some(i) = display.find(" — ") {
        &display[..i]
    } else {
        display
    }
}

/// The **visible** pill face: the base language name followed by its BCP-47 code, e.g.
/// `Français (fr-FR)`. The region/variant specificity is **not** on the face — it lives in the
/// tooltip and the "+" menu (see [`pill_detail`] / the registry `display_name`). The code
/// disambiguates variants that share a base name (the three French dictionaries). A hand-added
/// dictionary's code shows the user's name + code; an otherwise unrecognised tag falls back to
/// its raw string.
fn pill_name(tag: &str, user_names: &HashMap<String, String>) -> String {
    // A hand-added dictionary's given name wins on its own code (consistent with the Installed
    // list and with loading, which both prefer the user's copy over a later catalogue entry).
    if let Some(name) = user_names.get(tag) {
        format!("{name} ({tag})")
    } else if let Some(e) =
        dictionary_registry::resolve_token(tag).and_then(dictionary_registry::by_id)
    {
        format!("{} ({})", base_name(&e.display_name), e.id)
    } else {
        tag.to_string()
    }
}

/// The **full** human name for the tooltip + accessibility: a hand-added dictionary's given name,
/// else the registry's `display_name` (with its region/variant), else the raw tag.
fn pill_detail(tag: &str, user_names: &HashMap<String, String>) -> String {
    if let Some(name) = user_names.get(tag) {
        name.clone()
    } else if let Some(e) =
        dictionary_registry::resolve_token(tag).and_then(dictionary_registry::by_id)
    {
        e.display_name.clone()
    } else {
        tag.to_string()
    }
}

/// The list with `tag` removed (whitespace-normalised).
fn without(list: &str, tag: &str) -> String {
    language::all(list)
        .filter(|t| *t != tag)
        .collect::<Vec<_>>()
        .join(" ")
}

/// The list with `tag` appended if absent (whitespace-normalised).
fn with(list: &str, tag: &str) -> String {
    let mut tags: Vec<&str> = language::all(list).collect();
    if !tags.contains(&tag) {
        tags.push(tag);
    }
    tags.join(" ")
}

impl Widget for LanguagePillField {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the list changes (an edit) or the mute set changes (a check toggled from
        // here or the other mount point).
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);
        self.spell.mute_version().bind_to(
            ctx.self_id(),
            ctx.binding_registry(),
            BindingLevel::Rebuild,
        );

        // Hand-added dictionaries (from the app-state download view-model): a custom code shows
        // its given name here and is offered in the "+" menu. Rebuild when the installed set
        // changes (an add/remove bumps `changed`), sorted by name for a stable menu.
        let mut user_dicts = match ctx.app_state::<DictionariesViewModel>().cloned() {
            Some(vm) => {
                vm.changed_signal().bind_to(
                    ctx.self_id(),
                    ctx.binding_registry(),
                    BindingLevel::Rebuild,
                );
                vm.user_dictionaries()
            }
            None => Vec::new(),
        };
        user_dicts.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        let user_names: HashMap<String, String> = user_dicts
            .iter()
            .map(|u| (u.code.clone(), u.name.clone()))
            .collect();

        let raw = self.value.get();
        let effective = if raw.trim().is_empty() {
            self.inherited.clone().unwrap_or_default()
        } else {
            raw
        };

        let mut flow = Wrap::new().spacing(6.0).line_spacing(6.0);

        for tag in language::all(&effective) {
            let tag = tag.to_string();
            let active = !self.spell.is_muted(&tag);

            // Toggle this language's session mute, then re-highlight.
            let on_toggle: Rc<dyn Fn(&mut EventContext)> = {
                let spell = self.spell.clone();
                let tag = tag.clone();
                Rc::new(move |c| {
                    spell.set_muted(&tag, !spell.is_muted(&tag));
                    reattach(c);
                })
            };
            // Remove this language from the list (persisted).
            let on_remove: Rc<dyn Fn(&mut EventContext)> = {
                let set = self.set.clone();
                let value = self.value.clone();
                let effective = effective.clone();
                let tag = tag.clone();
                Rc::new(move |c| {
                    let new = without(&effective, &tag);
                    value.set(new.clone());
                    set(new, c);
                    reattach(c);
                })
            };

            let name = pill_name(&tag, &user_names);
            let detail = pill_detail(&tag, &user_names);
            flow = flow.child(LanguagePill {
                display: name.clone(),
                tooltip: lit!(detail.clone()),
                active,
                hover: Signal::new(false),
                focused: Signal::new(false),
                remove_label: tr!(lang_pill_remove(name = name.clone())),
                // The accessible label carries the full region/variant name, e.g. "…English
                // (United States)", so a screen-reader user hears the specificity the pill face
                // (base name + code) leaves to the tooltip.
                a11y_label: if active {
                    tr!(lang_pill_mute(name = detail.clone()))
                } else {
                    tr!(lang_pill_unmute(name = detail))
                },
                on_toggle,
                on_remove,
                root_child: None,
            });
        }

        // The "+" popover: every registry language not already in the list. Compare through
        // `resolve_token` so a legacy spelling already present (`en_US`) still counts as its
        // registry id (`en-US`) and isn't offered twice.
        let present: std::collections::HashSet<&str> = language::all(&effective)
            .map(|t| dictionary_registry::resolve_token(t).unwrap_or(t))
            .collect();
        let mut menu = MenuList::new();
        for entry in dictionary_registry::entries() {
            if present.contains(entry.id.as_str()) {
                continue;
            }
            let set = self.set.clone();
            let value = self.value.clone();
            let effective = effective.clone();
            let id = entry.id.clone();
            menu = menu.item(
                MenuItem::new(lit!(entry.display_name.clone())).on_activate_fn(move |c| {
                    let new = with(&effective, &id);
                    value.set(new.clone());
                    set(new, c);
                    reattach(c);
                }),
            );
        }
        // Then the hand-added dictionaries (offered by the name the user gave them), so a custom
        // code can actually be selected for a document — the registry doesn't know it.
        for ud in &user_dicts {
            if present.contains(ud.code.as_str()) {
                continue;
            }
            let set = self.set.clone();
            let value = self.value.clone();
            let effective = effective.clone();
            let code = ud.code.clone();
            menu = menu.item(
                MenuItem::new(lit!(ud.name.clone())).on_activate_fn(move |c| {
                    let new = with(&effective, &code);
                    value.set(new.clone());
                    set(new, c);
                    reattach(c);
                }),
            );
        }
        // `.bare()`: a `MenuList` already draws its own popover surface, so skip the second
        // panel chrome the popover would otherwise wrap it in.
        let add_button = PopoverIconButton::new(IconButton::add().tooltip(tr!(lang_pill_add())))
            .bare()
            .content(menu);
        flow = flow.child(add_button);

        let id = ctx.add(flow.access_role(Role::List).access_label(tr!(lang_pill_list())));
        self.root_child = Some(id);
        vec![id]
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

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
        // Single child fills our bounds — the conventional `Composing`-widget override.
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

/// One language pill: a rounded chip with an optional leading green check, its label, and a
/// hover-revealed trailing remove button. Clicking the chip toggles the check (mute).
struct LanguagePill {
    /// The visible label — the language written in human (no raw code).
    display: String,
    /// The `name (code)` detail, shown as a hover tooltip (the code lives here, not on the face).
    tooltip: bastyde::i18n::LocalizedString,
    active: bool,
    hover: Signal<bool>,
    /// Whether this pill currently holds keyboard focus (drives the focus ring).
    focused: Signal<bool>,
    remove_label: bastyde::i18n::LocalizedString,
    a11y_label: bastyde::i18n::LocalizedString,
    on_toggle: Rc<dyn Fn(&mut EventContext)>,
    on_remove: Rc<dyn Fn(&mut EventContext)>,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for LanguagePill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguagePill")
            .field("display", &self.display)
            .finish()
    }
}

impl Widget for LanguagePill {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // The pill's width and height must NOT change when the check toggles or the × reveals
        // on hover — otherwise the whole flow reflows on every hover. So every slot is *always*
        // present in the layout and only its **paint** (opacity) is toggled: `set_opacity` is
        // paint-only (it never affects size), whereas `visible_when` would collapse the slot out
        // of layout and resize the chip. (The framework's `TabWidget` gets away with
        // `visible_when` only because a tab has a fixed, bar-driven width with a `Spacer` that
        // absorbs the close button — a content-sized chip in a `Wrap` has neither.)

        // Leading check: always laid out; opaque only when this language is active (not muted).
        let check_id = ctx.add(IconWidget::checkmark(11.0).color(TextRole::Success));
        ctx.set_opacity(check_id, if self.active { 1.0 } else { 0.0 });

        let label = TextWidget::new(lit!(self.display.clone())).style(TextStyleRole::Tiny);

        // Trailing ×: the **embedded clear glyph**, built exactly as `TextInput`/`SearchField`
        // build their in-field clear button — the real ✕ SVG at `icon_size(12)` centred in a 16 dp
        // hit target — *not* a 24 dp `IconButton`, whose square set the chip's height floor (a
        // 40 px chip for one line of text). Always laid out (space reserved so the width never
        // shifts), faded in on hover, tappable to remove. A tap on the × dispatches to *it*
        // (hit-test picks the deepest tap handler), so removing never also toggles the pill's mute.
        // Not a focus stop — Tab walks between pills, not onto the ×.
        //
        // The × carries **no `on_hover` and no cursor** — deliberately, exactly like `TabWidget`'s
        // hover-revealed close button. The `on_hover`-firing bubble pass stops at the first node
        // whose `PointerEnter` returns `Handled`, and a node's own cursor makes it do so (to set
        // that cursor). So a cursor on the × would swallow its `PointerEnter` before the bubble
        // reached the chip — the chip's `on_hover(true)` would never re-fire after the earlier
        // `on_hover(false)` (which fired when the pointer left the chip body to enter the ×), and
        // the × would vanish the instant you reached it. With no cursor/`on_hover`, the ×'s
        // `PointerEnter` bubbles up to the chip, which re-fires `on_hover(true)` and keeps `hover`
        // set — so the × stays visible while you hover it.
        let on_remove = self.on_remove.clone();
        let x_glyph = (bastyde::widgets::BuiltInIcons::defaults().clear)()
            .icon_size(12.0)
            .color(TextRole::Secondary);
        let x_id = ctx.add(
            MinSize::new(16.0, 16.0)
                .child(Center::new().child(x_glyph))
                .access_label(self.remove_label.clone())
                .on_tap(move |_e, c| on_remove(c)),
        );
        ctx.set_opacity(x_id, self.hover.map(|&h| if h { 1.0 } else { 0.0 }));
        let x_tip = ctx.add(bastyde::widgets::TooltipWidget::new(self.remove_label.clone()));
        let delay = ctx.theme().motion.tooltip_delay;
        ctx.attach_tooltip(x_id, x_tip, delay);

        let content = bastyde::widgets::HStack::new()
            .spacing(4.0)
            .add_child(check_id)
            .child(label)
            .add_child(x_id);

        // Keyboard-focus ring — the same `StandardListItem`/`Button` idiom: a reactive border on
        // the background rect, `BorderRole::Focused`, revealed only under `:focus-visible` (a
        // keyboard focus, not a mouse click). `focused` is driven by the `.on_focus` handler
        // below; `ctx.focus_visible()` is the input-modality gate.
        let focus_visible = ctx.focus_visible();
        let ring = self
            .focused
            .zip(&focus_visible)
            .map(|(f, v)| if *f && *v { 1.5 } else { 0.0 });
        let bg = RectWidget::new()
            .background(SurfaceRole::AccentSubtle)
            .corner_radius(CornerRadius::uniform(9999.0))
            .border_color(BorderRole::Focused)
            .border_width(ring);
        let chip = ZStack::new()
            .child(bg)
            .child(Padding::symmetric(8.0, 2.0).child(content));

        let on_toggle = self.on_toggle.clone();
        let hover = self.hover.clone();
        let focused = self.focused.clone();
        let pill = chip
            .access_role(Role::ListItem)
            .access_label(self.a11y_label.clone())
            .focusable(true)
            .on_hover(move |h, _c| hover.set(h))
            .on_focus(move |gained, _c| focused.set(gained))
            .on_tap(move |_e, c| on_toggle(c));

        let id = ctx.add(pill);
        // Show the BCP-47 code as a hover tooltip — attached the same way `Badge` does
        // (`TooltipWidget` + `ctx.attach_tooltip`), so the pill face stays name-only.
        let tip = ctx.add(bastyde::widgets::TooltipWidget::new(self.tooltip.clone()));
        let delay = ctx.theme().motion.tooltip_delay;
        ctx.attach_tooltip(id, tip, delay);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Rigid, like `Badge`/`Button`: size to CONTENT, never fill the proposed height. The
        // chip's background is a greedy `RectWidget` that fills whatever height its parent
        // proposes — so a tall form row proposed 32 px would drag the whole pill (and the Wrap
        // row) to 32 px, while the rigid "+" button stayed 24. Measure under an **unbounded
        // height** so the background reports 0 and the content (check + label + ×, ~20 px) drives
        // the pill height; the row then hugs the content instead of the row hugging the pill.
        let content_proposal = SizeProposal {
            height: None,
            ..proposal
        };
        self.root_child
            .and_then(|id| ctx.child_size(id, content_proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }

    fn place_children(
        &self,
        bounds: Rect,
        _proposal: SizeProposal,
        children: &mut [WidgetPlacement],
        ctx: &LayoutContext,
    ) {
        // The row (a FormLayout field) can be taller than the chip — do NOT fill it, or the
        // greedy `RectWidget` background would paint the full row height. Measure the chip's
        // natural (content) height under an unbounded height and place it full-width, vertically
        // centred, so the visible chip hugs its content no matter how tall the row is.
        let size = bounds.size();
        let origin = bounds.origin();
        for child in children.iter_mut() {
            let natural = ctx
                .child_size(
                    child.id,
                    SizeProposal {
                        width: Some(size.width),
                        height: None,
                    },
                )
                .map(|s| s.height)
                .unwrap_or(size.height);
            let h = natural.min(size.height);
            child.size = Size::new(size.width, h);
            child.origin = Point::new(origin.x, origin.y + (size.height - h) / 2.0);
        }
    }
}
