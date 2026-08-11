// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Settings ▸ User — who is using this installation.
//!
//! Two fields, both app-level and both optional. They answer "who wrote this
//! remark", which is a property of the *person at the keyboard* and not of the
//! manuscript: `Work: <name> ▸ Author` is the book's byline, it rides inside the
//! `.skrib`, and signing comments with it means an editor who opens someone
//! else's project signs their notes with the novelist's name. Hence a top-level
//! page rather than a corner of Appearance — and hence the cross-reference in
//! each field's hint, since the two are otherwise easy to confuse.
//!
//! Bound **straight** to the settings-store signals rather than mirrored into a
//! local one and committed on blur, the way `work_author` has to be: a store
//! signal is two-way and its disk write is already debounced (500 ms), so there
//! is no per-keystroke file write to defend against and no entity `save()` to
//! route through. Typing here therefore reaches the next comment immediately,
//! which is what the effect in `App::build` is watching for.

use teksilo::core::BindingLevel;
use teksilo::prelude::*;
use teksilo::widgets::TextInput;
use teksilo::widgets::tooltip::TooltipContent;

#[allow(unused_imports)]
use super::super::*;

/// Settings ▸ User — the signing name and initials.
///
/// Shaped like every other pane in the window: a `group` header opening the
/// rows, `field_label` + a 240 px control cell per line, and a `full_width` hint
/// closing it. (`FormLayout` currently stretches the control cell to the pane
/// regardless of that `FixedSize` — Appearance's language dropdown renders full
/// width for the same reason — so the wrapper is here to match how the sibling
/// panes are *written*, not because it changes what is drawn today.)
pub(in crate::settings) fn user_pane(vm: &SettingsViewModel) -> impl Widget {
    let name = FixedSize::new().width(240.0).child(
        TextInput::new(vm.user_name())
            .placeholder(tr!(settings_field_user_name_placeholder()))
            .rich_tooltip_content(TooltipContent::new(
                "settings.user_name",
                tr!(settings_field_user_name_hint()),
            )),
    );

    let form = FormLayout::new()
        .label(tr!(settings_page_user()))
        .label_gap(16.0)
        .row_spacing(14.0)
        .full_width(group(tr!(settings_group_identity())))
        .line(field_label(tr!(settings_field_user_name())), name)
        .line(
            field_label(tr!(settings_field_user_initials())),
            FixedSize::new()
                .width(240.0)
                .child(InitialsField::new(vm.user_name(), vm.user_initials())),
        )
        .full_width(hint(tr!(settings_field_user_hint())));

    pane_frame(crumb(None, tr!(settings_page_user())), form)
}

/// The initials field, showing what would be **derived** from the name as ghost
/// text whenever nothing has been typed here.
///
/// A widget of its own rather than a plain `TextInput`, because a placeholder is
/// fixed at construction (`TextInput::placeholder` takes a `LocalizedString`, not
/// a `Prop`) and this one has to follow the name field keystroke by keystroke.
/// Binding `user_name` at `BindingLevel::Rebuild` re-runs `build` — and therefore
/// re-reads the placeholder — on every change to it.
///
/// Deliberately **only** `user_name` is bound. Binding `user_initials` too would
/// rebuild this field while the writer is typing *into* it, which is how a
/// half-typed value loses its caret; and it would buy nothing, since the ghost
/// text is not drawn once the field has content. The `TextInput` inside is
/// two-way bound to the same signal, so a rebuild re-reads the live value and
/// nothing typed is lost either way.
///
/// The derivation is from `user_name` alone, never from the open project's
/// byline: this page applies to every project on the machine, so showing
/// initials derived from *this* project's author would be a different answer
/// tomorrow with a different project open — for a field whose whole job is to be
/// stable. When the name is empty there is nothing to infer and the static
/// placeholder stands.
struct InitialsField {
    user_name: Signal<String>,
    user_initials: Signal<String>,
    root_child: Option<WidgetId>,
}

/// The ghost text for the initials field, given the name currently typed.
///
/// Empty in, empty out — `initials_from_name` answers `""` for a name with no
/// letters in it, and a blank placeholder would read as a rendering fault rather
/// than as "nothing to infer yet". The static string stands in that case, saying
/// where the value *would* come from.
fn ghost_initials(name: &str) -> LocalizedString {
    let derived = skribisto_model::initials::initials_from_name(name);
    if derived.is_empty() {
        tr!(settings_field_user_initials_placeholder())
    } else {
        lit!(derived)
    }
}

impl InitialsField {
    fn new(user_name: Signal<String>, user_initials: Signal<String>) -> Self {
        Self {
            user_name,
            user_initials,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for InitialsField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InitialsField").finish()
    }
}

impl Widget for InitialsField {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        let sid = ctx.self_id();
        let reg = &ctx.binding_registry().clone();
        self.user_name.bind_to(sid, reg, BindingLevel::Rebuild);

        let field = TextInput::new(self.user_initials.clone())
            .placeholder(ghost_initials(&self.user_name.get()))
            .rich_tooltip_content(TooltipContent::new(
                "settings.user_initials",
                tr!(settings_field_user_initials_hint()),
            ));
        self.root_child = Some(ctx.add(field));
        self.root_child.into_iter().collect()
    }

    fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(LayoutResponse::from)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    /// The ghost text is the initials the app would actually store.
    #[test]
    fn the_ghost_text_is_what_would_be_derived() {
        assert_eq!(ghost_initials("Mary-Jane O'Brien").resolve_now(), "MJO");
        assert_eq!(ghost_initials("Rae Okafor").resolve_now(), "RO");
        assert_eq!(ghost_initials("Colette").resolve_now(), "C");
    }

    /// Nothing to infer yet, so the static line stands rather than a blank field
    /// that reads as broken.
    #[test]
    fn a_nameless_field_keeps_the_static_placeholder() {
        let fallback = tr!(settings_field_user_initials_placeholder()).resolve_now();
        assert_eq!(ghost_initials("").resolve_now(), fallback);
        assert_eq!(ghost_initials("   ").resolve_now(), fallback);
        // Digits are not letters: "1984" has no initials to take.
        assert_eq!(ghost_initials("1984").resolve_now(), fallback);
    }

    /// **The point of the widget.** A placeholder is fixed at construction, so
    /// the field has to be rebuilt for new ghost text to appear — this pins that
    /// the name signal is actually bound at `Rebuild` level. Without the
    /// `bind_to`, the ghost text would be whatever the name was when Settings
    /// opened and would never move again.
    #[test]
    fn typing_a_name_rebuilds_the_field() {
        let name = Signal::new(String::new());
        let initials = Signal::new(String::new());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(InitialsField::new(name.clone(), initials.clone())));
        tree.layout(SizeProposal::exact(240.0, 40.0));

        let before = tree.children(id);
        assert_eq!(before.len(), 1, "the field builds exactly one child");

        name.set("Mary-Jane O'Brien".into());
        tree.layout(SizeProposal::exact(240.0, 40.0));

        assert_ne!(
            tree.children(id),
            before,
            "the name changing must rebuild the field, or the ghost initials are \
             frozen at whatever the name was when the page opened"
        );
        // And the value the writer may have typed is untouched by that rebuild —
        // the `TextInput` is two-way bound, so it re-reads rather than resets.
        assert_eq!(initials.get(), "");
    }

    /// Typed initials are the value, not the ghost: rebuilding must never write
    /// the derivation over what someone chose about their own name.
    #[test]
    fn a_typed_value_survives_a_rebuild() {
        let name = Signal::new("Mary-Jane O'Brien".to_string());
        let initials = Signal::new("MO".to_string());
        let mut tree = WidgetTree::new();
        let id = tree.add_boxed(Box::new(InitialsField::new(name.clone(), initials.clone())));
        tree.layout(SizeProposal::exact(240.0, 40.0));
        let before = tree.children(id);

        name.set("Mary-Jane O'Brien-Smith".into());
        tree.layout(SizeProposal::exact(240.0, 40.0));

        assert_ne!(tree.children(id), before, "the rebuild did happen");
        assert_eq!(
            initials.get(),
            "MO",
            "a rebuild must not overwrite a typed value with the derivation"
        );
    }
}
