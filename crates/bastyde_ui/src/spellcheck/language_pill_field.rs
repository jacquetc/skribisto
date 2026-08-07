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
use bastyde::widgets::tooltip::TooltipContent;
use bastyde::widgets::{IconButton, IconWidget, MenuItem, MenuList, PopoverIconButton, Wrap};

use crate::widgets::{Pill, PillTooltip};

use std::collections::HashMap;

use crate::models::OpenDocsStore;
use crate::spellcheck::SpellcheckService;
use crate::spellcheck::dictionary_registry;
use crate::view_models::DictionariesViewModel;
use skribisto_model::language;

/// A writer for a new `dict_language` list — the caller persists it (and mirrors it into the
/// `value` signal). Takes an `EventContext` so it can run a backend command.
pub type SetLanguages = Rc<dyn Fn(Vec<String>, &mut EventContext)>;

/// The pill-flow language field.
pub struct LanguagePillField {
    /// The current list (a local mirror the caller keeps in sync with the backend).
    value: Signal<Vec<String>>,
    /// Persist a new list.
    set: SetLanguages,
    /// The mute state + a version signal to rebuild the checks on.
    spell: SpellcheckService,
    /// When `value` is empty, the list inherited from the Book/Work (shown, and materialised on
    /// the first edit). `None` for the Work-level field, which inherits from nothing.
    inherited: Option<Vec<String>>,
    /// The open Work's own document store — threaded in from the caller (Tier 2,
    /// see `sessions::WorkSession`'s module doc) rather than resolved via
    /// `ctx.app_state::<OpenDocsStore>()` in [`reattach`]: that slot is one
    /// process-wide value, so with a second Work open in a second window, a mute
    /// toggle in this window could re-attach a *different* Work's open documents.
    open_docs: OpenDocsStore,
    root_child: Option<WidgetId>,
}

impl LanguagePillField {
    pub fn new(
        value: Signal<Vec<String>>,
        set: SetLanguages,
        spell: SpellcheckService,
        inherited: Option<Vec<String>>,
        open_docs: OpenDocsStore,
    ) -> Self {
        Self {
            value,
            set,
            spell,
            inherited,
            open_docs,
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
fn reattach(open_docs: &OpenDocsStore, _ctx: &mut EventContext) {
    open_docs.attach_all();
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

/// The list with `tag` removed.
///
/// Used to split a string and re-join it. Now that the field *is* a list, both of these are
/// the operation they always meant — which is the whole point of the change.
fn without(list: &[String], tag: &str) -> Vec<String> {
    list.iter().filter(|t| *t != tag).cloned().collect()
}

/// The list with `tag` appended if absent.
fn with(list: &[String], tag: &str) -> Vec<String> {
    let mut tags = list.to_vec();
    if !tags.iter().any(|t| t == tag) {
        tags.push(tag.to_string());
    }
    tags
}

impl Widget for LanguagePillField {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild when the list changes (an edit) or the mute set changes (a check toggled from
        // here or the other mount point).
        //
        // `mute_version` is one counter shared by every pill, so muting any one of them
        // rebuilds the whole field — every pill and its tooltip. That was invisible while
        // these were plain tooltips (no state to lose); now that they are Rich and pin
        // themselves after a 2 s dwell, there is one reachable wrinkle: dwell on pill B until
        // its tooltip pins, then click pill A's check, and B's pinned tooltip disappears with
        // the rebuild. Left as-is deliberately. Removing it means making both the check's
        // opacity *and* its mute/unmute accessible label per-pill reactive, which reaches into
        // `SpellcheckService`'s shared-counter API — a real refactor of a working feature to
        // buy back an interaction (pin one pill's tooltip, then click another's check) that a
        // dismissal is a defensible response to anyway.
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
        user_dicts.sort_by_key(|a| a.name.to_lowercase());
        let user_names: HashMap<String, String> = user_dicts
            .iter()
            .map(|u| (u.code.clone(), u.name.clone()))
            .collect();

        let raw = self.value.get();
        let effective: Vec<String> = if raw.iter().all(|t| t.trim().is_empty()) {
            self.inherited.clone().unwrap_or_default()
        } else {
            raw
        };

        let mut flow = Wrap::new().spacing(6.0).line_spacing(6.0);

        // This window's own open Work — read from `open_docs` (Tier 2, threaded in) rather
        // than tracked separately here: `set_muted`/`is_muted` must scope to the *right*
        // Work, never a different one this same field instance happens to be rebuilt for.
        let work_id = self.open_docs.work_id();
        for tag in language::all(&effective) {
            let tag = tag.to_string();
            let active = !self.spell.is_muted(&tag, work_id);

            // Toggle this language's session mute, then re-highlight.
            let on_toggle: Rc<dyn Fn(&mut EventContext)> = {
                let spell = self.spell.clone();
                let tag = tag.clone();
                let open_docs = self.open_docs.clone();
                Rc::new(move |c| {
                    let work_id = open_docs.work_id();
                    spell.set_muted(&tag, !spell.is_muted(&tag, work_id), work_id);
                    reattach(&open_docs, c);
                })
            };
            // Remove this language from the list (persisted).
            let on_remove: Rc<dyn Fn(&mut EventContext)> = {
                let set = self.set.clone();
                let value = self.value.clone();
                let effective = effective.clone();
                let tag = tag.clone();
                let open_docs = self.open_docs.clone();
                Rc::new(move |c| {
                    let new = without(&effective, &tag);
                    value.set(new.clone());
                    set(new, c);
                    reattach(&open_docs, c);
                })
            };

            let name = pill_name(&tag, &user_names);
            let detail = pill_detail(&tag, &user_names);
            // The accessible label carries the full region/variant name, e.g. "…English
            // (United States)", so a screen-reader user hears the specificity the pill face
            // (base name + code) leaves to the tooltip.
            let a11y_label = if active {
                tr!(lang_pill_mute(name = detail.clone()))
            } else {
                tr!(lang_pill_unmute(name = detail.clone()))
            };
            flow = flow.child(
                Pill::new(name.clone(), a11y_label)
                    // Always laid out; opaque only when this language is active (not muted),
                    // so toggling the check never resizes the chip or reflows the row.
                    .leading(IconWidget::checkmark(11.0).color(TextRole::Success), active)
                    // Rich, not plain: the detail is short text that plausibly grows a
                    // "dictionary installed?" disclosure, which is exactly `TooltipContent`'s
                    // text + more shape. It is also the only tier that gains keyboard-focus
                    // promotion — the thing keyboard users had no way to reach before. The
                    // content is per-tag and computed, so it takes the inline path rather
                    // than a registry key.
                    .tooltip(PillTooltip::Rich(TooltipContent::new(
                        format!("lang-pill-{tag}"),
                        lit!(detail),
                    )))
                    .on_remove(tr!(lang_pill_remove(name = name.clone())), move |c| {
                        on_remove(c)
                    })
                    .on_activate(move |c| on_toggle(c)),
            );
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
            let open_docs = self.open_docs.clone();
            menu = menu.item(
                MenuItem::new(lit!(entry.display_name.clone())).on_activate_fn(move |c| {
                    let new = with(&effective, &id);
                    value.set(new.clone());
                    set(new, c);
                    reattach(&open_docs, c);
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
            let open_docs = self.open_docs.clone();
            menu = menu.item(
                MenuItem::new(lit!(ud.name.clone())).on_activate_fn(move |c| {
                    let new = with(&effective, &code);
                    value.set(new.clone());
                    set(new, c);
                    reattach(&open_docs, c);
                }),
            );
        }
        // `.bare()`: a `MenuList` already draws its own popover surface, so skip the second
        // panel chrome the popover would otherwise wrap it in.
        let add_button = PopoverIconButton::new(IconButton::add().tooltip(tr!(lang_pill_add())))
            .bare()
            .content(menu);
        flow = flow.child(add_button);

        let id = ctx.add(
            flow.access_role(Role::List)
                .access_label(tr!(lang_pill_list())),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Same height-for-width as TagPillField: measure the Wrap against a
        // concrete width and claim the full offered column so chips reflow.
        let wrap_w = proposal.width.or(Some(280.0));
        let measured = self
            .root_child
            .and_then(|id| {
                ctx.child_size(
                    id,
                    SizeProposal {
                        width: wrap_w,
                        height: None,
                    },
                )
            })
            .unwrap_or(Size::ZERO);
        Size::new(proposal.width.unwrap_or(measured.width), measured.height).into()
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
