// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Links in the text that point at a resource: `[Village map](resource:<uid>)`.
//!
//! A resource dragged from the dock into the prose becomes a link to it, named after its
//! title, addressed by its durable `uid` (an `EntityId` is minted again at every load).
//! Ctrl+click on one opens the resource; an export prints its words only, since a reader
//! of the book has no project to open it in (see `skribisto_compiler`'s
//! `strip_resource_links`).
//!
//! **The scheme is routed before the external-link allow-list, and never added to it.**
//! [`activate_link`] is the one door every text surface's Ctrl+click goes through: a
//! `resource:` link becomes [`AppIntent::OpenResource`], and anything else goes on to
//! [`crate::shared::external_link::open_external_link`] unchanged. Listing `resource`
//! among the schemes the desktop may open would hand `resource:…` to `xdg-open`, which
//! is the opposite of what is meant.
//!
//! Not a view-model: a scheme, a parser and two editor writes.

use teksilo::prelude::*;
use teksilo::widgets::rich_text::EditorHandle;

use crate::intents::AppIntent;

/// The scheme of a link to a resource.
pub const RESOURCE_SCHEME: &str = "resource";

/// The address of a link to the resource `uid`.
pub fn resource_href(uid: uuid::Uuid) -> String {
    format!("{RESOURCE_SCHEME}:{}", uid.hyphenated())
}

/// The resource a link points at, `None` for any other link.
///
/// The scheme is case-insensitive, as every URL scheme is; what follows must be a uid
/// and nothing else, so `resource:` followed by a path names nothing.
pub fn resource_uid(href: &str) -> Option<uuid::Uuid> {
    let href = href.trim();
    let (scheme, rest) = href.split_once(':')?;
    if !scheme.eq_ignore_ascii_case(RESOURCE_SCHEME) {
        return None;
    }
    uuid::Uuid::parse_str(rest.trim())
        .ok()
        .filter(|u| !u.is_nil())
}

/// Follow a link the writer Ctrl+clicked in a text: a resource link opens the resource,
/// any other goes to the browser or mail client through the allow-list.
pub fn activate_link(href: &str, ctx: &mut EventContext) {
    match resource_uid(href) {
        Some(uid) => ctx.send_intent(AppIntent::OpenResource {
            uid: uid.hyphenated().to_string(),
        }),
        None => crate::shared::external_link::open_external_link(href, ctx),
    }
}

/// What a link insertion put into the text: the span it covers, and its words, so a
/// later Undo can check the span still holds them before deleting it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InsertedSpan {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

/// Insert one link per `(words, href)` at `at`, separated by `", "`, as one undo step
/// of the editor's own history. Returns what was inserted.
///
/// Written as text plus a link mark rather than as Djot: a title can hold any character
/// and an address can hold parentheses, and neither then needs escaping into a syntax
/// the parser reads back.
pub fn insert_links(handle: &EditorHandle, at: usize, links: &[(String, String)]) -> InsertedSpan {
    let mut text = String::new();
    let mut marks: Vec<(usize, usize, &str)> = Vec::new();
    let mut cursor = at;
    for (i, (words, href)) in links.iter().enumerate() {
        if i > 0 {
            text.push_str(", ");
            cursor += 2;
        }
        let words = if words.trim().is_empty() {
            href.as_str()
        } else {
            words.as_str()
        };
        let len = words.chars().count();
        text.push_str(words);
        marks.push((cursor, cursor + len, href.as_str()));
        cursor += len;
    }
    handle.edit_block(|| {
        handle.replace_range(at, at, &text);
        for (start, end, href) in &marks {
            handle.select_range(*start, *end);
            handle.set_link(href);
        }
        handle.select_range(cursor, cursor);
    });
    InsertedSpan {
        start: at,
        end: cursor,
        text,
    }
}

/// Take back a span [`insert_links`] put in, if it still holds the same words. `false`
/// when the text moved under it since, and nothing was deleted: removing whatever now
/// sits at those offsets would delete the writer's words.
pub fn remove_span(handle: &EditorHandle, span: &InsertedSpan) -> bool {
    let plain: Vec<char> = handle.to_plain_text().chars().collect();
    let here: String = plain
        .get(span.start..span.end)
        .map(|s| s.iter().collect())
        .unwrap_or_default();
    if here != span.text {
        return false;
    }
    handle.edit_block(|| {
        handle.select_range(span.start, span.end);
        handle.delete_selection();
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_resource_link_round_trips_through_its_address() {
        let uid = uuid::Uuid::from_u128(0x1234_5678_9abc_def0_1234_5678_9abc_def0);
        let href = resource_href(uid);
        assert_eq!(href, "resource:12345678-9abc-def0-1234-56789abcdef0");
        assert_eq!(resource_uid(&href), Some(uid));
        assert_eq!(
            resource_uid("RESOURCE:12345678-9abc-def0-1234-56789abcdef0"),
            Some(uid),
            "a scheme is case-insensitive"
        );
    }

    #[test]
    fn nothing_else_is_a_resource_link() {
        for href in [
            "https://example.com",
            "resource:",
            "resource:/etc/passwd",
            "resource:00000000-0000-0000-0000-000000000000",
            "resources:12345678-9abc-def0-1234-56789abcdef0",
            "file:///tmp/x",
            "",
        ] {
            assert_eq!(resource_uid(href), None, "{href}");
        }
    }

    /// The scheme never reaches the desktop: the allow-list the opener applies keeps
    /// refusing it, so a route that forgot to branch first would show a refusal rather
    /// than launch anything.
    #[test]
    fn the_external_opener_still_refuses_the_scheme() {
        assert!(!crate::shared::external_link::is_openable(
            "resource:12345678-9abc-def0-1234-56789abcdef0"
        ));
    }

    /// Owns the `resources.open` action the way `app::commands::resources` does, with a
    /// button that follows a link the way a Ctrl+click does.
    struct LinkHost {
        href: &'static str,
        opened: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
        button: Option<teksilo::prelude::WidgetId>,
    }

    impl std::fmt::Debug for LinkHost {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("LinkHost").finish()
        }
    }

    impl Widget for LinkHost {
        fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
            let opened = self.opened.clone();
            ctx.register_action_global(Action::new("resources.open").on_invoke(move |i, _c| {
                if let Some(AppIntent::OpenResource { uid }) = AppIntent::from_intent(i) {
                    opened.borrow_mut().push(uid.clone());
                }
            }));
            let href = self.href;
            let id = ctx.add(
                teksilo::widgets::Button::new(lit!("link"))
                    .on_activate_fn(move |c| activate_link(href, c)),
            );
            self.button = Some(id);
            vec![id]
        }

        fn layout_response(&self, proposal: SizeProposal, ctx: &LayoutContext) -> LayoutResponse {
            self.button
                .and_then(|id| ctx.child_size(id, proposal))
                .map(LayoutResponse::from)
                .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
        }

        fn children(&self) -> Vec<WidgetId> {
            self.button.into_iter().collect()
        }
    }

    /// Ctrl+click on a `resource:` link reaches the Resources command with the uid, and
    /// never the desktop.
    #[test]
    fn a_resource_link_is_routed_to_the_resources_command() {
        let opened = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let mut tree = teksilo::core::widget_tree::WidgetTree::new();
        tree.add(LinkHost {
            href: "resource:12345678-9abc-def0-1234-56789abcdef0",
            opened: opened.clone(),
            button: None,
        });
        tree.layout(SizeProposal::exact(300.0, 100.0));
        let button = tree.find_by_label("link").expect("the link button");
        crate::test_support::click(&mut tree, button);
        tree.layout(SizeProposal::exact(300.0, 100.0));
        assert_eq!(
            *opened.borrow(),
            vec!["12345678-9abc-def0-1234-56789abcdef0".to_string()]
        );
    }

    fn editor(text: &str) -> EditorHandle {
        let doc = teksilo::text_document::TextDocument::new();
        doc.set_plain_text(text).expect("text");
        teksilo::widgets::rich_text::RichTextEditor::editor(doc).handle()
    }

    #[test]
    fn links_go_in_at_the_offset_as_their_words_and_come_back_out() {
        let handle = editor("Before after");
        let span = insert_links(
            &handle,
            7,
            &[
                ("Village map".into(), "resource:a".into()),
                ("Tide tables".into(), "https://example.org/tides".into()),
            ],
        );
        assert_eq!(span.text, "Village map, Tide tables");
        assert_eq!(
            handle.to_plain_text(),
            "Before Village map, Tide tablesafter"
        );
        assert!(handle.to_djot().contains("[Village map](resource:a)"));
        assert!(handle.to_djot().contains("(https://example.org/tides)"));

        assert!(remove_span(&handle, &span));
        assert_eq!(handle.to_plain_text(), "Before after");
    }

    #[test]
    fn a_span_the_writer_has_since_edited_is_left_alone() {
        let handle = editor("Before after");
        let span = insert_links(&handle, 7, &[("Map".into(), "resource:a".into())]);
        handle.replace_range(0, 0, "Typed. ");
        assert!(
            !remove_span(&handle, &span),
            "the words moved: delete nothing"
        );
        assert!(handle.to_plain_text().contains("Map"));
    }
}
