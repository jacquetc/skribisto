// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The two-field modal behind the Link command.
//!
//! A link is the one piece of formatting that cannot be a toggle: it needs a
//! destination, and usually a name to show instead of it. So this asks for both
//! and hands them back to [`FormatViewModel::apply_link`], which does the
//! writing.
//!
//! ## Why a custom modal
//!
//! `InputDialog` captures exactly one string, and says so in its own docs:
//! anything longer belongs in a bespoke dialog. This is that dialog, built on
//! the same `present_modal` shape `goals::distribute_panel` uses.
//!
//! ## Why it holds no editor handle
//!
//! Everything it needs about the document is resolved *before* it opens, into a
//! [`LinkRequest`]. By the time a modal is on screen the editor has lost focus,
//! and a handle grabbed at that point may be the wrong editor — or a stale one,
//! since a rebuild mints fresh editor state. The view-model re-resolves the
//! target when the write actually happens.
//!
//! Removal lives here rather than on a menu of its own: with no context-menu
//! entry for links, this dialog is the only door to it, so an edit that opens
//! on an existing link offers "Remove link" beside the usual pair.

use teksilo::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, FormLayout, HStack, ModalContainer, Spacer, TextInput, VStack,
};

use crate::format::{FormatViewModel, LinkRequest};
use crate::shared::text::field_label;

/// The modal's asked-for size. Wide enough that a real URL is readable without
/// scrolling the field, and tall enough for the third button an edit grows.
const CARD_W: u32 = 460;
const CARD_H: u32 = 190;

/// Open the Link dialog over the current editor.
///
/// A no-op when nothing is focused: every format command has to survive being
/// invoked from a menu that outlived the editor it was opened over.
pub fn present(vm: &FormatViewModel, ctx: &mut EventContext) {
    let Some(req) = vm.link_request() else {
        return;
    };
    let vm = vm.clone();
    let title = if req.editing {
        tr!(link_dialog_edit_title())
    } else {
        tr!(link_dialog_insert_title())
    };
    ctx.present_modal(
        ModalRequest::deferred({
            let vm = vm.clone();
            let req = req.clone();
            let title = title.clone();
            // `ModalContainer` is what draws the panel — rounded card, padding,
            // shadow, and the `Role::Dialog` node a screen reader needs.
            // Without it the fields render bare over the prose behind them.
            move |t| {
                t.add(
                    ModalContainer::new(LinkPanel::new(vm.clone(), req.clone()))
                        .title(title.clone()),
                )
            }
        })
        .presentation(ModalPresentation::Auto)
        .title(title)
        .size(CARD_W, CARD_H)
        .close_behavior(ModalCloseBehavior::EscapeOrClickOutside),
    );
}

struct LinkPanel {
    vm: FormatViewModel,
    req: LinkRequest,
    name: Signal<String>,
    href: Signal<String>,
    root_child: Option<WidgetId>,
}

impl LinkPanel {
    fn new(vm: FormatViewModel, req: LinkRequest) -> Self {
        Self {
            name: Signal::new(req.name.clone()),
            href: Signal::new(req.href.clone()),
            vm,
            req,
            root_child: None,
        }
    }
}

impl std::fmt::Debug for LinkPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LinkPanel").finish()
    }
}

impl Widget for LinkPanel {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        // Deliberately no `bind_to(.., Rebuild)` on either field. The only
        // thing that reacts to typing is the confirm button's enablement, and
        // `enabled` takes a reactive prop — so a derived signal covers it
        // without rebuilding. Rebuilding per keystroke would also re-mint every
        // child widget mid-edit, which loses the field's caret and hands an
        // automation client node ids that are stale by the next character.
        let form = FormLayout::new()
            .label_gap(12.0)
            .row_spacing(12.0)
            .line(
                field_label(tr!(link_dialog_text_label())),
                TextInput::new(self.name.clone()).placeholder(tr!(link_dialog_text_placeholder())),
            )
            .line(
                field_label(tr!(link_dialog_url_label())),
                TextInput::new(self.href.clone()).placeholder(tr!(link_dialog_url_placeholder())),
            );

        // A link with no destination is not a link. The name may be blank —
        // that means "show the URL itself", handled at commit.
        //
        // Derived, so the button follows the field live without this widget
        // rebuilding. `Signal::map` is read-only, which is all `enabled` needs.
        let committable = self.href.map(|s| !s.trim().is_empty());

        let mut actions = HStack::new().spacing(8.0);
        if self.req.editing {
            let vm = self.vm.clone();
            actions = actions.child(
                Button::new(tr!(link_dialog_remove()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(move |c| {
                        vm.remove_link();
                        c.dismiss_modal();
                        c.request_frame();
                        vm.refocus(c);
                    }),
            );
        }

        // The *signals*, not their values. Nothing rebuilds this widget while
        // the writer types, so a value snapshotted here would be whatever the
        // fields held when the dialog opened — i.e. the dialog would always
        // commit an empty destination.
        let name = self.name.clone();
        let href = self.href.clone();
        let vm = self.vm.clone();
        let actions = actions
            .child(Spacer::new())
            .child(
                Button::new(tr!(link_dialog_cancel()))
                    .variant(ButtonVariant::Plain)
                    .on_activate_fn(|c| c.dismiss_modal()),
            )
            .child(
                Button::new(if self.req.editing {
                    tr!(link_dialog_apply())
                } else {
                    tr!(link_dialog_insert())
                })
                .variant(ButtonVariant::Filled)
                .enabled(committable)
                .on_activate_fn(move |c| {
                    let href = normalize_href(&href.get());
                    // A blank name means the writer wants the address itself
                    // on the page — the common case for pasting a bare URL.
                    let typed = name.get();
                    let name = if typed.trim().is_empty() {
                        href.clone()
                    } else {
                        typed
                    };
                    vm.apply_link(&name, &href);
                    c.dismiss_modal();
                    c.request_frame();
                    vm.refocus(c);
                }),
            );

        // No padding or width of its own: `ModalContainer` owns the card's
        // chrome, and a second frame inside it would double the inset.
        let root = ctx.add(VStack::new().spacing(16.0).child(form).child(actions));
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

/// Give a bare address a scheme, so `example.com` reaches the web rather than
/// being handed to the OS as a relative path.
///
/// Deliberately shy: anything that already carries a scheme is left exactly as
/// typed, and so is anything that could be one. Guessing is only safe for the
/// two shapes that are unambiguous — an address with an `@` before any slash is
/// mail, and everything else is web.
pub(crate) fn normalize_href(raw: &str) -> String {
    let s = raw.trim();
    if s.is_empty() {
        return String::new();
    }
    // A scheme, and not a Windows drive letter (`C:\…`), which is a path.
    if let Some(colon) = s.find(':')
        && colon > 1
        && s[..colon]
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    {
        return s.to_string();
    }
    if s.starts_with('/') || s.starts_with('#') {
        return s.to_string();
    }
    // A backslash means a Windows path, not a host. Inventing `https://` for
    // it would produce something that is neither the path the writer typed nor
    // a reachable address, so it is stored exactly as given and the opener
    // declines it later.
    if s.contains('\\') {
        return s.to_string();
    }
    let before_slash = s.split('/').next().unwrap_or(s);
    if before_slash.contains('@') {
        return format!("mailto:{s}");
    }
    format!("https://{s}")
}

#[cfg(test)]
mod tests {
    use super::normalize_href;

    #[test]
    fn an_address_with_a_scheme_is_left_alone() {
        for s in [
            "https://example.com",
            "http://example.com",
            "mailto:a@b.com",
            "ftp://files.example.com",
        ] {
            assert_eq!(normalize_href(s), s);
        }
    }

    #[test]
    fn a_bare_domain_becomes_https() {
        assert_eq!(normalize_href("example.com"), "https://example.com");
        assert_eq!(
            normalize_href("example.com/a/b?c=d"),
            "https://example.com/a/b?c=d"
        );
        assert_eq!(normalize_href("  example.com  "), "https://example.com");
    }

    #[test]
    fn a_bare_mail_address_becomes_mailto() {
        assert_eq!(
            normalize_href("someone@example.com"),
            "mailto:someone@example.com"
        );
    }

    #[test]
    fn an_at_sign_after_a_slash_is_not_mail() {
        // A path can contain `@` — a fediverse profile, say. Only an `@` in the
        // authority position means mail.
        assert_eq!(
            normalize_href("example.com/@someone"),
            "https://example.com/@someone"
        );
    }

    #[test]
    fn a_windows_path_is_left_exactly_as_typed() {
        // `C:` looks like a scheme to a naive check; prefixing `https://`
        // would be worse still, producing neither the path the writer typed
        // nor a reachable address. Stored verbatim; the opener declines it.
        assert_eq!(normalize_href(r"C:\notes\draft.md"), r"C:\notes\draft.md");
    }

    #[test]
    fn an_empty_address_stays_empty() {
        assert_eq!(normalize_href("   "), "");
    }
}
