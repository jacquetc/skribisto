// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The `container.segments` slot: the bar on a Book / Part / Chapter / Notes-folder tab,
//! and how something outside this crate adds a segment to it.
//!
//! ## Why segments carry a string id
//!
//! A `SegmentedControl` pairs with its `Switcher` by position, and until now this bar was
//! addressed that way throughout — `tabs.rs` set `tab.segment` to `6` and meant "the Book's
//! Overview". That works exactly as long as nothing can be inserted ahead of it, which is
//! the property an extension slot removes.
//!
//! So every segment now has a stable string id, and its `SegmentId` is **derived** from
//! that id rather than stored. Two consequences worth stating, because both were nearly
//! got wrong:
//!
//! - `Segment::new` mints `SegmentId::fresh()` — a *process-global counter*. A segment left
//!   un-pinned gets a different id every launch, so a remembered view keyed on it would be
//!   unrecoverable the next time the app starts. Every segment this crate builds pins its
//!   id explicitly.
//! - The derivation is one-way, so nothing can recover the string from a `SegmentId` alone.
//!   Anything that needs to persist a selection (see [`crate::settings::EditorViewMemory`])
//!   is handed the ordered `(id, SegmentId)` list to look it back up — it persists the
//!   **string**, and re-derives the number on the next launch. That is also why the hash
//!   below is a fixed algorithm rather than `DefaultHasher`, whose output std explicitly
//!   does not guarantee across Rust versions: nothing depends on it surviving a toolchain
//!   bump today, and this makes sure nothing quietly starts to.
//!
//! ## Registration is a snapshot, not a subscription
//!
//! `folder_segmented` resolves the segment list once, when
//! a tab is built. A segment registered afterwards reaches tabs opened *after* it, not the
//! ones already on screen; one unregistered afterwards stays on the tabs that already have
//! it until they are rebuilt.
//!
//! That is the contract, not an oversight. An extension registers during startup, before any
//! project window exists, so in practice every tab sees the complete set. Making it live
//! would mean driving a rebuild through `RememberSegment` and `Boxed`, both of which are
//! deliberately build-once, to buy correctness for enable/disable-at-runtime — which nothing
//! supports yet.

use std::cell::RefCell;
use std::rc::Rc;

use teksilo::prelude::*;
use teksilo::widgets::SegmentId;

use frontend::common::entities::BinderItemSubRole;

use crate::tabs::ContentTab;

/// The container's own page — its title, subtitle and synopsis.
pub const SEG_OWN: &str = "own";
/// The manuscript stream (Full Book / Full Part / Full Chapter).
pub const SEG_MANUSCRIPT: &str = "manuscript";
/// The same rows as an editable outline.
pub const SEG_SYNOPSIS: &str = "full-synopsis";
/// The Book-only writing plan.
pub const SEG_PACE: &str = "pace";
/// The Book-only measurements panel.
pub const SEG_ANALYSIS: &str = "analysis";
pub const SEG_CORKBOARD: &str = "corkboard";
pub const SEG_OVERVIEW: &str = "overview";
/// A notes folder's own page (`folder_synopsis_with_overview`, which has no streams).
pub const SEG_NOTES: &str = "notes";

/// The `SegmentId` for a stable string id.
///
/// FNV-1a, folded into 48 bits so the result always lands below `SegmentId`'s `FRESH_BASE`
/// (2^48) and can therefore never collide with a framework-allocated id, and `| 1` so it is
/// never zero (`SegmentId` wraps a `NonZeroU64`).
///
/// Deterministic across runs *and across Rust versions* — see the module docs for why that
/// second half matters even though nothing persists the number itself.
pub fn segment_id(id: &str) -> SegmentId {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in id.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    SegmentId::from_u64((hash & 0xFFFF_FFFF_FFFF) | 1)
}

/// Builds a registered segment's body from the tab it is shown on.
pub type SegmentViewFn = Rc<dyn Fn(&ContentTab) -> Box<dyn Widget>>;

/// Decides which containers a registered segment appears on.
pub type ShowsOnFn = Rc<dyn Fn(&BinderItemSubRole) -> bool>;

/// One segment on a container tab's bar.
#[derive(Clone)]
pub struct ContainerSegmentSpec {
    /// Stable and namespaced for anything not built in (`"ext.structure"`). This is what is
    /// persisted, and what [`segment_id`] derives the widget key from.
    pub id: String,
    /// Resolved per build, so a runtime locale switch reaches the label. Storing a
    /// `LocalizedString` at registration would pin it to whichever locale was active when
    /// the extension loaded.
    pub label: crate::docks::LabelFn,
    /// Builds the segment's body.
    pub view: SegmentViewFn,
    /// Which containers show it. The Book's Pace and Analysis are the built-in precedent for
    /// a segment that is not on every container.
    pub shows_on: ShowsOnFn,
}

impl ContainerSegmentSpec {
    pub fn segment_id(&self) -> SegmentId {
        segment_id(&self.id)
    }
}

struct Registered {
    namespace: String,
    spec: ContainerSegmentSpec,
}

// Thread-local rather than a `static RwLock`: a spec holds `Rc` closures that build widgets,
// and `Rc` is not `Send`. Same shape as the analysis-category registry and teksilo's own
// tooltip registry.
thread_local! {
    static EXTENSION_SEGMENTS: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// The ids this crate builds itself, which a registration may not claim.
fn is_builtin(id: &str) -> bool {
    matches!(
        id,
        SEG_OWN
            | SEG_MANUSCRIPT
            | SEG_SYNOPSIS
            | SEG_PACE
            | SEG_ANALYSIS
            | SEG_CORKBOARD
            | SEG_OVERVIEW
            | SEG_NOTES
    )
}

/// Add a segment to container tabs.
///
/// Refused when `id` is one this crate already uses, or one another namespace registered:
/// the id is both the persisted key and the widget key, so two claimants make a remembered
/// view ambiguous rather than merely crowded.
///
/// Registered segments are inserted **after** the container's own extras (the Book's Pace and
/// Analysis) and **before** Corkboard and Overview — the same place an analytical view
/// belongs, rather than trailing the two views *of* the manuscript.
///
/// The returned handle unregisters on drop; re-registering a namespace replaces its entry.
pub fn register_container_segment(
    namespace: impl Into<String>,
    spec: ContainerSegmentSpec,
) -> Result<ContainerSegmentHandle, String> {
    let namespace = namespace.into();
    if is_builtin(&spec.id) {
        return Err(format!("segment id '{}' is a built-in", spec.id));
    }
    EXTENSION_SEGMENTS.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.spec.id == spec.id && r.namespace != namespace)
        {
            return Err(format!(
                "segment id '{}' is already registered by '{}'",
                spec.id, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            spec,
        });
        Ok(ContainerSegmentHandle {
            namespace: namespace.clone(),
        })
    })
}

/// Unregisters its segment when dropped.
#[derive(Debug)]
pub struct ContainerSegmentHandle {
    namespace: String,
}

impl Drop for ContainerSegmentHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = EXTENSION_SEGMENTS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Registered segments that apply to `sub_role`, in registration order.
pub fn registered_for(sub_role: &BinderItemSubRole) -> Vec<ContainerSegmentSpec> {
    EXTENSION_SEGMENTS.with(|reg| {
        reg.borrow()
            .iter()
            .filter(|r| (r.spec.shows_on)(sub_role))
            .map(|r| r.spec.clone())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::widgets::TextWidget;

    fn spec(id: &str) -> ContainerSegmentSpec {
        ContainerSegmentSpec {
            id: id.to_string(),
            label: Rc::new(|| lit!("Structure".to_string())),
            view: Rc::new(|_| Box::new(TextWidget::new(lit!("body".to_string())))),
            shows_on: Rc::new(|s| matches!(s, BinderItemSubRole::Book)),
        }
    }

    fn ids_for(sub_role: &BinderItemSubRole) -> Vec<String> {
        registered_for(sub_role).into_iter().map(|s| s.id).collect()
    }

    /// The derivation must be stable across runs — it is what a remembered view is looked
    /// up by — and must land where it cannot collide with a framework-allocated id.
    #[test]
    fn segment_ids_are_deterministic_and_in_the_app_owned_range() {
        assert_eq!(segment_id("overview"), segment_id("overview"));
        assert_ne!(segment_id("overview"), segment_id("corkboard"));
        // `SegmentId::fresh` allocates from 2^48 upward; everything derived here must sit
        // below that, or an app id could one day equal a framework one.
        for id in [
            SEG_OWN,
            SEG_MANUSCRIPT,
            SEG_SYNOPSIS,
            SEG_PACE,
            SEG_ANALYSIS,
            SEG_CORKBOARD,
            SEG_OVERVIEW,
            SEG_NOTES,
        ] {
            assert!(
                segment_id(id).get() < (1u64 << 48),
                "`{id}` derives an id inside the framework's reserved range"
            );
        }
    }

    /// Every built-in id is distinct. A collision would make two segments the same segment.
    #[test]
    fn built_in_ids_are_all_distinct() {
        let all = [
            SEG_OWN,
            SEG_MANUSCRIPT,
            SEG_SYNOPSIS,
            SEG_PACE,
            SEG_ANALYSIS,
            SEG_CORKBOARD,
            SEG_OVERVIEW,
            SEG_NOTES,
        ];
        let mut seen = std::collections::HashSet::new();
        for id in all {
            assert!(
                seen.insert(segment_id(id)),
                "`{id}` collides with another built-in"
            );
        }
        assert_eq!(seen.len(), all.len());
    }

    /// `shows_on` filters, so a Book-only segment does not appear on a Part.
    #[test]
    fn a_registered_segment_only_appears_where_it_says() {
        let _h = register_container_segment("test.scope", spec("ext.scope")).expect("register");
        assert!(ids_for(&BinderItemSubRole::Book).contains(&"ext.scope".to_string()));
        assert!(!ids_for(&BinderItemSubRole::Part).contains(&"ext.scope".to_string()));
    }

    #[test]
    fn a_built_in_id_is_refused() {
        let err =
            register_container_segment("test.shadow", spec(SEG_OVERVIEW)).expect_err("must refuse");
        assert!(err.contains("built-in"), "unhelpful message: {err}");
    }

    #[test]
    fn a_taken_id_is_refused_and_names_its_holder() {
        let _first = register_container_segment("test.a", spec("ext.contested")).expect("ok");
        let err =
            register_container_segment("test.b", spec("ext.contested")).expect_err("must refuse");
        assert!(err.contains("test.a"), "unhelpful message: {err}");
    }

    #[test]
    fn drop_unregisters_and_re_registration_replaces() {
        let book = BinderItemSubRole::Book;
        {
            let _a = register_container_segment("test.same", spec("ext.first")).expect("a");
            assert!(ids_for(&book).contains(&"ext.first".to_string()));
            let _b = register_container_segment("test.same", spec("ext.second")).expect("b");
            assert!(ids_for(&book).contains(&"ext.second".to_string()));
            assert!(
                !ids_for(&book).contains(&"ext.first".to_string()),
                "re-registering a namespace must replace, not stack"
            );
        }
        assert!(
            !ids_for(&book).contains(&"ext.second".to_string()),
            "a dropped handle must leave no segment behind"
        );
    }
}
