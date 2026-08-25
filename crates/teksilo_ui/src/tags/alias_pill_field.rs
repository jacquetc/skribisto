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
//!
//! **The live collision hint.** Two items can legally share an alias: a deliberate
//! epithet ("the Captain") or a nickname two characters both go by is a real thing a
//! writer wants, not a mistake, so this is never validation and nothing here blocks
//! Enter. It is a fact stated as the writer types, the same class as a spelling
//! suggestion: [`AliasPillField::collision_lookup`] arms it with the story-bible table
//! and the item being edited, and it is silent whenever nothing else already answers
//! to the exact text in the box. Only the Inspector's own field arms it: the creation
//! modal (`story_bible::modal`) edits an item that does not exist yet, so there is
//! nothing meaningful to exclude, and it stays off there.

use std::rc::Rc;

use skribisto_model::mentions::DiscoverableEntity;
use teksilo::core::BindingLevel;
use teksilo::core::accesskit::Role;
use teksilo::core::widget::WidgetPlacement;
use teksilo::prelude::*;
use teksilo::widgets::{
    IconButton, Padding, Panel, PopoverIconButton, TextInput, TextWidget, VStack, Wrap,
};

use crate::widgets::Pill;

/// Persist a new alias list for the item.
pub type SetAliases = Rc<dyn Fn(Vec<String>, &mut EventContext)>;

/// What the live collision hint needs: the full story-bible table, so a title collides
/// just as an alias does, and the item being edited, so its own name is never reported
/// as colliding with itself.
#[derive(Clone)]
struct CollisionLookup {
    table: Vec<DiscoverableEntity>,
    owner_id: u64,
}

pub struct AliasPillField {
    value: Signal<Vec<String>>,
    set: SetAliases,
    draft: Signal<String>,
    root_child: Option<WidgetId>,
    collision: Option<CollisionLookup>,
}

impl AliasPillField {
    pub fn new(value: Signal<Vec<String>>, set: SetAliases) -> Self {
        Self {
            value,
            set,
            draft: Signal::new(String::new()),
            root_child: None,
            collision: None,
        }
    }

    /// Arm the live "already used by…" hint under the "+" popover's text field.
    ///
    /// `table` is `MentionIndex::discoverable_table()` and `owner_id` is the item this
    /// field belongs to. See the module doc for why only one call site uses this.
    pub fn collision_lookup(mut self, table: Vec<DiscoverableEntity>, owner_id: u64) -> Self {
        self.collision = Some(CollisionLookup { table, owner_id });
        self
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

/// The other item, if any, that already answers to `candidate`: by its title or by
/// one of its own aliases. Same fold as `contains_fold`, so this agrees with what
/// "already there" will mean once the typed text is actually saved. Never `owner_id`:
/// an item's own title or an alias it already holds is not a collision with itself.
fn alias_collision(table: &[DiscoverableEntity], owner_id: u64, candidate: &str) -> Option<String> {
    let key = candidate.trim().to_lowercase();
    if key.is_empty() {
        return None;
    }
    table
        .iter()
        .find(|e| {
            e.id != owner_id
                && (e.title.trim().to_lowercase() == key
                    || e.aliases.iter().any(|a| a.trim().to_lowercase() == key))
        })
        .map(|e| e.title.clone())
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
                Pill::new(alias.clone(), lit!(alias.clone()))
                    .on_remove(tr!(tags_alias_remove(name = alias.clone())), move |c| {
                        on_remove(c)
                    }),
            );
        }

        flow = flow.child(
            PopoverIconButton::new(IconButton::add().tooltip(tr!(tags_alias_add())))
                .bare()
                .content(AliasEntry {
                    draft: self.draft.clone(),
                    value: self.value.clone(),
                    set: self.set.clone(),
                    collision: self.collision.clone(),
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
    collision: Option<CollisionLookup>,
    root_child: Option<WidgetId>,
}

impl std::fmt::Debug for AliasEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AliasEntry").finish()
    }
}

impl Widget for AliasEntry {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Rebuild on every keystroke, same reasoning as `CastAddPopover::query`: the
        // collision hint below reads `self.draft.get()` directly, so nothing repaints
        // it without this.
        self.draft
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

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

        let mut col = VStack::new().spacing(4.0).child(field);

        // The live hint: a fact about the one item in front of the writer, not a
        // warning. A shared alias is a legal thing to want, so this never blocks
        // Enter and is silent the moment nothing else answers to the typed text.
        if let Some(lookup) = &self.collision
            && let Some(other) = alias_collision(&lookup.table, lookup.owner_id, &self.draft.get())
        {
            col = col.child(
                TextWidget::new(tr!(tags_alias_collision(name = other)))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );
        }

        col = col.child(
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

    fn entity(id: u64, title: &str, aliases: &[&str]) -> DiscoverableEntity {
        DiscoverableEntity {
            id,
            title: title.to_string(),
            aliases: aliases.iter().map(|a| a.to_string()).collect(),
        }
    }

    /// The heart of the hint: another item's *alias* already answers to the text
    /// being typed, and its title is what gets named back to the writer.
    #[test]
    fn names_the_item_whose_alias_already_matches() {
        let table = vec![entity(1, "Hap", &["the Colonel"]), entity(2, "Devon", &[])];
        assert_eq!(
            alias_collision(&table, 2, "the colonel"),
            Some("Hap".to_string())
        );
    }

    /// A collision on another item's *title*, not just its alias list: "answering
    /// to" a name includes simply being called by it.
    #[test]
    fn a_bare_title_match_is_also_a_collision() {
        let table = vec![entity(1, "Hap", &[]), entity(2, "Devon", &[])];
        assert_eq!(alias_collision(&table, 2, "hap"), Some("Hap".to_string()));
    }

    /// Case and surrounding space never matter, matching `contains_fold`'s own fold:
    /// this is the same "already there" the writer will get once they hit Enter.
    #[test]
    fn the_hint_folds_case_and_space_like_contains_fold() {
        let table = vec![entity(1, "Hap", &["the Colonel"])];
        assert_eq!(
            alias_collision(&table, 2, "  THE COLONEL  "),
            Some("Hap".to_string())
        );
    }

    /// Silent when nothing else answers to it: the required, and most common, case.
    #[test]
    fn silent_when_nothing_else_answers_to_it() {
        let table = vec![entity(1, "Hap", &["the Colonel"]), entity(2, "Devon", &[])];
        assert_eq!(alias_collision(&table, 2, "Lizzy"), None);
    }

    /// Never reports the item being edited as colliding with its own name.
    #[test]
    fn the_item_being_edited_never_collides_with_itself() {
        let table = vec![entity(1, "Hap", &["the Colonel"])];
        assert_eq!(alias_collision(&table, 1, "the Colonel"), None);
        assert_eq!(alias_collision(&table, 1, "Hap"), None);
    }

    /// Blank or all-space input never claims a collision: an empty alias is never
    /// saved anyway, so there is nothing meaningful to warn about yet.
    #[test]
    fn blank_input_never_collides() {
        let table = vec![entity(1, "Hap", &["the Colonel"])];
        assert_eq!(alias_collision(&table, 2, ""), None);
        assert_eq!(alias_collision(&table, 2, "   "), None);
    }
}
