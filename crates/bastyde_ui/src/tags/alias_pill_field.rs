// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `AliasPillField` — the other names an item answers to in prose, as a pill flow.
//!
//! Aliases are what make the mention index work at all: a note titled "Elizabeth Bennet" is
//! written as "Lizzy" and "Miss Bennet" on the page, and matching titles alone would find
//! almost nothing. Plume projects arrive with these already filled in; everyone else types
//! them here.
//!
//! Shown only on items carrying a discoverable tag — an alias on a scene would be indexed
//! against nothing.
//!
//! Unlike tags there is no palette: an alias is free text belonging to this item alone, so
//! the "+" is a text field rather than a picker.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::core::accesskit::Role;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::widgets::{
    IconButton, Padding, Panel, PopoverIconButton, TextInput, TextWidget, VStack, Wrap,
};

use crate::widgets::Pill;

/// Persist a new alias list for the item.
pub type SetAliases = Rc<dyn Fn(Vec<String>, &mut EventContext)>;

pub struct AliasPillField {
    value: Signal<Vec<String>>,
    set: SetAliases,
    draft: Signal<String>,
    root_child: Option<WidgetId>,
}

impl AliasPillField {
    pub fn new(value: Signal<Vec<String>>, set: SetAliases) -> Self {
        Self {
            value,
            set,
            draft: Signal::new(String::new()),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for AliasPillField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AliasPillField").finish()
    }
}

/// Case-insensitive membership, so "Lizzy" and "lizzy" are not both added — the matcher
/// would treat them as one anyway.
fn contains_fold(list: &[String], candidate: &str) -> bool {
    let key = candidate.trim().to_lowercase();
    list.iter().any(|a| a.trim().to_lowercase() == key)
}

impl Widget for AliasPillField {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.value
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let aliases = self.value.get();
        let mut flow = Wrap::new().spacing(6.0).line_spacing(6.0);

        for alias in &aliases {
            let on_remove: Rc<dyn Fn(&mut EventContext)> = {
                let set = self.set.clone();
                let value = self.value.clone();
                let alias = alias.clone();
                Rc::new(move |c| {
                    let next: Vec<String> =
                        value.get().into_iter().filter(|a| *a != alias).collect();
                    value.set(next.clone());
                    set(next, c);
                })
            };
            flow = flow.child(
                Pill::new(alias.clone(), lit!(alias.clone())).on_remove(
                    tr!(tags_alias_remove(name = alias.clone())),
                    move |c| on_remove(c),
                ),
            );
        }

        flow = flow.child(
            PopoverIconButton::new(IconButton::add().tooltip(tr!(tags_alias_add())))
                .bare()
                .content(AliasEntry {
                    draft: self.draft.clone(),
                    value: self.value.clone(),
                    set: self.set.clone(),
                    root_child: None,
                }),
        );

        let id = ctx.add(
            flow.access_role(Role::List)
                .access_label(tr!(tags_alias_list())),
        );
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
        for child in children.iter_mut() {
            child.origin = bounds.origin();
            child.size = bounds.size();
        }
    }
}

/// The "+" popover: one text field that commits on Enter.
struct AliasEntry {
    draft: Signal<String>,
    value: Signal<Vec<String>>,
    set: SetAliases,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for AliasEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AliasEntry").finish()
    }
}

impl Widget for AliasEntry {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let draft = self.draft.clone();
        let value = self.value.clone();
        let set = self.set.clone();

        let field = TextInput::new(self.draft.clone())
            .placeholder(tr!(tags_alias_placeholder()))
            .min_width(220.0)
            .on_submit_fn(move |c| {
                let typed = draft.get().trim().to_string();
                // A blank alias would match nothing; a duplicate would be indexed once
                // anyway. Either way, silently do nothing rather than growing the list.
                if typed.is_empty() || contains_fold(&value.get(), &typed) {
                    draft.set(String::new());
                    return;
                }
                let mut next = value.get();
                next.push(typed);
                value.set(next.clone());
                set(next, c);
                draft.set(String::new());
            });

        let col = VStack::new()
            .spacing(4.0)
            .child(field)
            .child(
                TextWidget::new(tr!(tags_alias_hint()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );

        let id = ctx.add(
            Panel::new()
                .child(Padding::uniform(8.0).child(col))
                .access_role(Role::Dialog)
                .access_label(tr!(tags_alias_add())),
        );
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        // Popover content: size to the panel, never to the (unbounded) overlay proposal.
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
    fn membership_ignores_case_and_space() {
        let list = vec!["Lizzy".to_string(), "Miss Bennet".to_string()];
        assert!(contains_fold(&list, "lizzy"));
        assert!(contains_fold(&list, "  LIZZY  "));
        assert!(contains_fold(&list, "miss bennet"));
        assert!(!contains_fold(&list, "Jane"));
    }
}
