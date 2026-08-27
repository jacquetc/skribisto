// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `note.details.sections` slot: what an extension may add to **one entry's**
//! Details page, and why that is a different door from `container.segments`.
//!
//! ## The question this exists for
//!
//! [`crate::tabs::shared::segments`] lets an extension add a whole *page* to a
//! container's tab bar. That is the right shape for a reading about a container —
//! a Book's Pace, a Book's Analysis. It is the wrong shape for a reading about
//! **one story-bible entry**, and the difference is not size.
//!
//! A per-entry reading has to answer "where does *she* turn up", and the app
//! already knows which entry: the writer opened her note. An extension with no
//! door here has to ask again — which is what a picker is, and a picker is the
//! binder rebuilt badly. It also has to live somewhere, and the only somewhere on
//! offer was a modal opened from the Tools row: undiscoverable, detached from the
//! item it is about, and on a platform with no input-blocking modal protocol,
//! subject to a presentation fallback with its own failure modes.
//!
//! So: a section on the page, beside the community edition's own reading of the
//! same rows. What is written stays free — "Appears in the manuscript" lists the
//! documents; an extension may add what shape that list has.
//!
//! ## Where a section lands, and why it is the right column
//!
//! Under the manuscript column, after the backlinks. The left column is the
//! entry's **fields** — its name, tags, aliases, filing, links: things the writer
//! sets. The right column is what the manuscript already says about it. A reading
//! is the second kind, and putting one among the fields would invite the reading
//! to look editable.
//!
//! A page with no manuscript column at all — an ordinary note that no scan ever
//! reaches — grows one when a section registers, because a section is entitled to
//! say something about an entry the scan found nothing for ("not searched yet",
//! "no hits"). That is a fact about the entry, not an empty promise.
//!
//! ## Registration is a snapshot, not a subscription
//!
//! Resolved once, when a Details page is built, exactly as
//! [`crate::tabs::shared::segments`]'s own note records: an extension registers
//! during startup, before any project window exists, so in practice every page
//! sees the complete set.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::*;

use frontend::AppContext;

use crate::app_ids::AppIds;
use crate::mentions::MentionIndex;

/// Everything a section is given about the entry whose page it is on.
///
/// The **entry's own id** and this Work's live handles — never `ctx.app_state`,
/// which answers with whatever the bootstrap session built and is empty for the
/// life of a process started without a project. See [`crate::docks::DockContext`]
/// for the full account of that trap.
#[derive(Clone)]
pub struct NoteSectionContext {
    pub app_ctx: Rc<AppContext>,
    pub ids: AppIds,
    /// The `BinderItem` this page is about: the story-bible entry itself.
    pub item_id: u64,
    /// Whether this entry carries a tag flagged discoverable — that is, whether
    /// the scan was ever given its names at all.
    ///
    /// Handed over rather than left to each section to re-derive, because the
    /// distinction it carries is one every section needs and one that is easy to
    /// get wrong in the same direction: an untagged entry's zero hits mean "never
    /// looked", not "looked and found none", and a section that conflates them
    /// asserts an absence it never measured.
    pub discoverable: bool,
    /// This Work's own index. Same handle the page's own backlinks list reads.
    pub mention_index: MentionIndex,
}

/// Builds a registered section's body for the entry it is shown on.
pub type NoteSectionViewFn = Rc<dyn Fn(&NoteSectionContext) -> Box<dyn Widget>>;

/// One section on an entry's Details page.
#[derive(Clone)]
pub struct NoteSectionSpec {
    /// Stable and namespaced (`"pro.bible.appearances"`). Not persisted anywhere —
    /// a section has no remembered state, unlike a segment — but still the identity
    /// a second registration collides on.
    pub id: String,
    /// Resolved per build, so a runtime locale switch reaches the heading.
    pub label: crate::docks::LabelFn,
    /// The section's body.
    pub view: NoteSectionViewFn,
}

struct Registered {
    namespace: String,
    spec: NoteSectionSpec,
}

thread_local! {
    static SECTIONS: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// Register a section on every entry's Details page.
///
/// Re-registering under the same `namespace` replaces that namespace's section, so
/// an extension reinstalling itself does not accumulate copies. A different
/// namespace claiming an `id` another already holds is refused: two sections under
/// one id would be two headings a writer cannot tell apart, and the second would
/// silently win nothing.
pub fn register_note_details_section(
    namespace: impl Into<String>,
    spec: NoteSectionSpec,
) -> Result<NoteSectionHandle, String> {
    let namespace = namespace.into();
    SECTIONS.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.spec.id == spec.id && r.namespace != namespace)
        {
            return Err(format!(
                "note section id '{}' is already registered by '{}'",
                spec.id, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            spec,
        });
        Ok(NoteSectionHandle {
            namespace: namespace.clone(),
        })
    })
}

/// Unregisters its section when dropped.
#[derive(Debug)]
pub struct NoteSectionHandle {
    namespace: String,
}

impl Drop for NoteSectionHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = SECTIONS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Every registered section, in registration order.
pub fn registered_note_sections() -> Vec<NoteSectionSpec> {
    SECTIONS.with(|reg| reg.borrow().iter().map(|r| r.spec.clone()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::i18n::lit;

    fn spec(id: &str) -> NoteSectionSpec {
        NoteSectionSpec {
            id: id.to_string(),
            label: Rc::new(|| lit!("Section")),
            view: Rc::new(|_| Box::new(teksilo::widgets::Spacer::new())),
        }
    }

    #[test]
    fn a_registered_section_is_offered_and_withdrawn_with_its_handle() {
        {
            let _h = register_note_details_section("test.one", spec("ext.one")).expect("register");
            assert!(
                registered_note_sections().iter().any(|s| s.id == "ext.one"),
                "a live registration is offered to every Details page"
            );
        }
        assert!(
            !registered_note_sections().iter().any(|s| s.id == "ext.one"),
            "and dropping the handle withdraws it"
        );
    }

    /// Two extensions cannot claim one id: a writer would see two headings they
    /// cannot tell apart, and neither owner would know.
    #[test]
    fn a_second_namespace_cannot_claim_an_id_another_holds() {
        let _first = register_note_details_section("test.a", spec("ext.contested")).expect("ok");
        let err = register_note_details_section("test.b", spec("ext.contested"))
            .expect_err("the second must be refused");
        assert!(
            err.contains("test.a"),
            "the error must name the holder, or the clash is unfixable: {err}"
        );
    }

    /// Re-registering under one namespace replaces, so an extension reinstalling
    /// itself does not stack copies of its own section.
    #[test]
    fn re_registering_one_namespace_replaces_rather_than_stacks() {
        let _a = register_note_details_section("test.same", spec("ext.first")).expect("a");
        let _b = register_note_details_section("test.same", spec("ext.second")).expect("b");
        let ids: Vec<String> = registered_note_sections()
            .into_iter()
            .map(|s| s.id)
            .collect();
        assert!(!ids.contains(&"ext.first".to_string()));
        assert!(ids.contains(&"ext.second".to_string()));
    }
}
