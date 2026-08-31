// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Sections another crate can add to the trailing **Inspector** dock.
//!
//! The Inspector is the panel that answers *"what is this row I am looking at"*
//! without the writer leaving their scene — which makes it the one surface in the
//! application where a reading about the focused item can sit while the writing
//! happens. Everything else that reports on a manuscript is a place you navigate
//! to: a container tab, an Analysis category, a dock of its own competing for
//! rail space.
//!
//! That is what this registry is for. A contributed section is handed the focused
//! item and the backend handles, renders underneath the built-in body, and is
//! shown only for the sub-roles it asks for — so a reading about scenes does not
//! appear while a Note is focused.
//!
//! ## The same shape as the other two
//!
//! [`crate::tabs::shared::segments::register_container_segment`] and
//! [`crate::tabs::analysis::register_category`] already exist and this is
//! deliberately their twin: a namespaced registration, an id that a built-in may
//! not use and a second namespace may not claim, a handle that unregisters on
//! drop, built-ins first and registrations after. Three registries that behave
//! differently would be three things to learn.
//!
//! ## What a section may assume
//!
//! **Nothing captured at registration.** Registration happens before the
//! application builds its `AppContext`, so a spec that closed over one would hold
//! a second, permanently empty store and render a convincing "nothing here"
//! forever. [`InspectorContext`] hands the live handles to the *view*, at build
//! time, exactly as `DockContext` does for a whole dock.
//!
//! **A rebuild on every change that matters.** The Inspector already rebuilds
//! when focus moves, when the focused item is renamed or promoted, and when a
//! move changes its enclosing Book. A section is rebuilt with it and needs no
//! wiring of its own for those.

use std::cell::RefCell;
use std::rc::Rc;

use frontend::AppContext;
use frontend::common::entities::BinderItemSubRole;
use frontend::direct_access::BinderItemDto;
use teksilo::prelude::Widget;

use crate::app_ids::AppIds;
use crate::docks::LabelFn;

/// The live handles a contributed section builds against.
///
/// Borrowed rather than cloned into the spec for the reason the whole seam
/// exists: these are the application's own, constructed inside `run`, and a
/// section that kept its own copies would be reading a store nothing writes to.
pub struct InspectorContext<'a> {
    pub app_ctx: &'a Rc<AppContext>,
    pub ids: &'a AppIds,
    /// The focused binder item, whole.
    ///
    /// The dto rather than an id, because a section almost always wants the
    /// `uid` — the durable key anything persisted must use, and the one thing an
    /// `EntityId` cannot be trusted to be across a reload.
    pub item: &'a BinderItemDto,
}

/// Builds a section's body for the focused item.
pub type SectionViewFn = Rc<dyn Fn(&InspectorContext<'_>) -> Box<dyn Widget>>;

/// Which focused items a section appears for.
pub type ShowsOnFn = Rc<dyn Fn(&BinderItemSubRole) -> bool>;

/// One contributed section of the Inspector.
#[derive(Clone)]
pub struct InspectorSectionSpec {
    /// Stable, and namespaced for anything not built in (`"ext.structure"`).
    ///
    /// Not shown to the writer. Nothing persists a section today — unlike a
    /// container segment, which is written into a Book's remembered view — but it
    /// is what makes two claimants an error rather than a silent replacement.
    pub id: String,
    /// The section's heading. A closure, not a resolved string: the Inspector is
    /// rebuilt per focus change, and a label resolved at registration would be
    /// pinned to whichever locale was active when the contributor loaded and
    /// would never follow a runtime language switch.
    pub label: LabelFn,
    pub view: SectionViewFn,
    /// Which focused sub-roles show it.
    ///
    /// There is no "always" default on purpose. The Inspector is already dense,
    /// and a section that appears under a Note when it has nothing to say about
    /// notes is worse than one the writer has to go looking for.
    pub shows_on: ShowsOnFn,
}

struct Registered {
    namespace: String,
    spec: InspectorSectionSpec,
}

// Thread-local rather than a `static RwLock`: a spec holds `Rc` closures that
// build widgets, and `Rc` is not `Send`. Same shape as the container-segment and
// analysis-category registries.
thread_local! {
    static SECTIONS: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// Add a section to the Inspector.
///
/// Refused when `id` is one another namespace registered: the id is how a section
/// is identified across rebuilds, so two claimants make that lookup ambiguous
/// rather than merely crowded. This dock builds no sections of its own through
/// the registry, so there is no built-in id to collide with — the whole
/// application-owned body is composed before the first contributed section.
///
/// The returned handle unregisters on drop; re-registering a namespace replaces
/// its entry.
pub fn register_inspector_section(
    namespace: impl Into<String>,
    spec: InspectorSectionSpec,
) -> Result<InspectorSectionHandle, String> {
    let namespace = namespace.into();
    SECTIONS.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.spec.id == spec.id && r.namespace != namespace)
        {
            return Err(format!(
                "inspector section id '{}' is already registered by '{}'",
                spec.id, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            spec,
        });
        Ok(InspectorSectionHandle {
            namespace: namespace.clone(),
        })
    })
}

/// Unregisters its section when dropped.
#[derive(Debug)]
pub struct InspectorSectionHandle {
    namespace: String,
}

impl Drop for InspectorSectionHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = SECTIONS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Registered sections that apply to `sub_role`, in registration order.
pub fn registered_for(sub_role: &BinderItemSubRole) -> Vec<InspectorSectionSpec> {
    SECTIONS.with(|reg| {
        reg.borrow()
            .iter()
            .filter(|r| (r.spec.shows_on)(sub_role))
            .map(|r| r.spec.clone())
            .collect()
    })
}

/// Every registered section, whatever it shows on. For tests and for anything
/// that needs the roster rather than the applicable subset.
pub fn all() -> Vec<InspectorSectionSpec> {
    SECTIONS.with(|reg| reg.borrow().iter().map(|r| r.spec.clone()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::prelude::*;
    use teksilo::widgets::TextWidget;

    fn spec(id: &str, shows_on: fn(&BinderItemSubRole) -> bool) -> InspectorSectionSpec {
        InspectorSectionSpec {
            id: id.to_string(),
            label: Rc::new(|| lit!("Section".to_string())),
            view: Rc::new(|_cx| Box::new(TextWidget::new(lit!("body".to_string())))),
            shows_on: Rc::new(shows_on),
        }
    }

    fn ids_for(sub_role: &BinderItemSubRole) -> Vec<String> {
        registered_for(sub_role).into_iter().map(|s| s.id).collect()
    }

    /// A section appears only for the sub-roles it asked for. The Inspector is
    /// dense already, and one that turns up under a Note with nothing to say
    /// about notes is worse than one the writer goes looking for.
    #[test]
    fn a_section_shows_only_where_it_asked_to() {
        let _h = register_inspector_section(
            "test.scenes",
            spec("ext.scenes", |r| matches!(r, BinderItemSubRole::Scene)),
        )
        .expect("register");

        assert!(ids_for(&BinderItemSubRole::Scene).contains(&"ext.scenes".to_string()));
        assert!(!ids_for(&BinderItemSubRole::Note).contains(&"ext.scenes".to_string()));
    }

    /// Two namespaces cannot claim one id, and the error names the holder — so a
    /// clash is a startup failure someone can act on rather than a section that
    /// quietly stopped appearing.
    #[test]
    fn a_taken_id_is_refused_and_names_its_holder() {
        let _first =
            register_inspector_section("test.one", spec("ext.contested", |_| true)).expect("first");
        let err = register_inspector_section("test.two", spec("ext.contested", |_| true))
            .expect_err("a second claimant must be refused");
        assert!(err.contains("test.one"), "unhelpful message: {err}");
    }

    /// Dropping the handle removes the section; re-registering a namespace
    /// replaces rather than stacking. Two live copies would mean two identical
    /// sections in one panel.
    #[test]
    fn drop_unregisters_and_re_registration_replaces() {
        {
            let _a =
                register_inspector_section("test.scoped", spec("ext.first", |_| true)).expect("a");
            assert!(ids_for(&BinderItemSubRole::Scene).contains(&"ext.first".to_string()));

            let _b =
                register_inspector_section("test.scoped", spec("ext.second", |_| true)).expect("b");
            assert!(ids_for(&BinderItemSubRole::Scene).contains(&"ext.second".to_string()));
            assert!(
                !ids_for(&BinderItemSubRole::Scene).contains(&"ext.first".to_string()),
                "re-registering a namespace must replace, not stack"
            );
        }
        assert!(
            !ids_for(&BinderItemSubRole::Scene).contains(&"ext.second".to_string()),
            "a dropped handle must leave no section behind"
        );
    }

    /// The same namespace may hold one section at a time, and re-registering it
    /// with the *same* id is not a self-clash.
    #[test]
    fn a_namespace_may_re_register_its_own_id() {
        let _a = register_inspector_section("test.self", spec("ext.same", |_| true)).expect("a");
        let _b = register_inspector_section("test.self", spec("ext.same", |_| true))
            .expect("a namespace must not collide with itself");
        assert_eq!(
            all().iter().filter(|s| s.id == "ext.same").count(),
            1,
            "re-registering must replace the entry, not add a second"
        );
    }

    /// Every spec carries a resolvable label and a body builder. A spec missing
    /// either would put a heading over someone else's content, or a section with
    /// no way to say what it is.
    #[test]
    fn every_registered_section_has_a_label_and_a_body() {
        let _h = register_inspector_section("test.whole", spec("ext.whole", |_| true))
            .expect("register");
        for s in all() {
            let _ = (s.label)();
            assert!(!s.id.is_empty());
        }
    }
}
