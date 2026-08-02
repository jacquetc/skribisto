// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Add to cast…" — pick a story-bible entry that is not yet pinned on this item.
//!
//! References-first: the catalogue is the alias table from the last mention scan
//! (every discoverable item), not free-form binder search. Filtering is local and
//! cheap; selecting a row calls the same pin path as a suggestion.

use std::rc::Rc;

use bastyde::core::BindingLevel;
use bastyde::core::accesskit::Role;
use bastyde::core::widget::WidgetPlacement;
use bastyde::prelude::*;
use bastyde::widgets::{
    FocusScope, MaxSize, Padding, Panel, PopoverButton, ScrollArea, TextInput, TextWidget,
    TraversalScopePolicy, VStack,
};
use skribisto_model::mentions::DiscoverableEntity;

use super::mention_list::PinReference;

/// One story-bible entry offered in the Add popover.
#[derive(Clone, Debug)]
pub struct CastCandidate {
    pub id: u64,
    pub title: String,
}

impl From<&DiscoverableEntity> for CastCandidate {
    fn from(e: &DiscoverableEntity) -> Self {
        Self {
            id: e.id,
            title: e.title.clone(),
        }
    }
}

/// Filter field + scrollable list of story-bible items not already in the cast.
pub struct CastAddPopover {
    candidates: Vec<CastCandidate>,
    /// Target ids already confirmed on the focused item.
    already: std::collections::HashSet<u64>,
    /// The focused item itself — never offered as cast of itself.
    owner_id: u64,
    pin: PinReference,
    query: Signal<String>,
    root_child: Option<WidgetId>,
}

impl CastAddPopover {
    pub fn new(
        candidates: Vec<CastCandidate>,
        already: impl IntoIterator<Item = u64>,
        owner_id: u64,
        pin: PinReference,
    ) -> Self {
        Self {
            candidates,
            already: already.into_iter().collect(),
            owner_id,
            pin,
            query: Signal::new(String::new()),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for CastAddPopover {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CastAddPopover")
            .field("candidates", &self.candidates.len())
            .finish()
    }
}

fn name_key(s: &str) -> String {
    s.trim().to_lowercase()
}

impl Widget for CastAddPopover {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.query
            .bind_to(ctx.self_id(), ctx.binding_registry(), BindingLevel::Rebuild);

        let key = name_key(&self.query.get());
        let rows: Vec<&CastCandidate> = self
            .candidates
            .iter()
            .filter(|c| c.id != self.owner_id && !self.already.contains(&c.id))
            .filter(|c| key.is_empty() || name_key(&c.title).contains(&key))
            .collect();

        let mut col = VStack::new().spacing(6.0).child(
            TextInput::new(self.query.clone()).placeholder(tr!(cast_add_filter_placeholder())),
        );

        if rows.is_empty() {
            col = col.child(
                TextWidget::new(tr!(cast_add_empty()))
                    .style(TextStyleRole::Tiny)
                    .color(TextRole::Secondary),
            );
        } else {
            let mut list = VStack::new().spacing(2.0);
            for c in rows {
                let pin = self.pin.clone();
                let id = c.id;
                let title = c.title.clone();
                list = list.child(
                    TextWidget::new(lit!(title.clone()))
                        .style(TextStyleRole::Small)
                        .max_lines(1)
                        .access_role(Role::ListItem)
                        .access_label(lit!(title.clone()))
                        .focusable(true)
                        .on_tap({
                            let pin = pin.clone();
                            move |_e, c| pin(id, c)
                        })
                        .on_key(move |ev, c| {
                            if let WidgetEvent::KeyDown { key, .. } = ev
                                && matches!(key, Key::Enter | Key::Space)
                            {
                                pin(id, c);
                                return EventResponse::Handled;
                            }
                            EventResponse::Ignored
                        }),
                );
            }
            col = col.child(
                MaxSize::height(220.0).child(ScrollArea::new().child(list.access_role(Role::List))),
            );
        }

        let id = ctx.add(Panel::new().child(Padding::symmetric(8.0, 8.0).child(col)));
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

/// Trigger button that opens the Add-to-cast popover.
pub fn cast_add_button(
    candidates: Vec<CastCandidate>,
    already: Vec<u64>,
    owner_id: u64,
    pin: PinReference,
) -> impl Widget {
    PopoverButton::new(
        bastyde::widgets::Button::new(tr!(cast_add()))
            .variant(bastyde::widgets::ButtonVariant::Plain),
    )
    .content(
        FocusScope::new(TraversalScopePolicy::Cycle)
            .child(CastAddPopover::new(candidates, already, owner_id, pin)),
    )
}

/// Build candidate list from the scan's alias table.
pub fn candidates_from_table(table: &[DiscoverableEntity]) -> Vec<CastCandidate> {
    let mut out: Vec<CastCandidate> = table.iter().map(CastCandidate::from).collect();
    out.sort_by(|a, b| a.title.cmp(&b.title));
    out
}

/// Debounce delay before a live prose overlay is scanned for cast suggestions.
///
/// Writing must never wait on this work: the timer is only armed from idle /
/// mutation signals, and the expensive `to_djot` + match run when the deadline
/// fires — never on the key that typed a character.
pub const LIVE_CAST_DEBOUNCE: std::time::Duration = std::time::Duration::from_millis(600);

use std::cell::{Cell, RefCell};
use std::time::Instant;

/// Editor-safe live prose cache for the focused item's cast suggestions.
///
/// - Does **not** bind Inspector rebuild to open-doc edit counters.
/// - Arms a `wake_at` deadline on focus change / edit; when the frame tick
///   notices the deadline, runs `to_djot` once and bumps [`Self::version`].
/// - Callers must re-register frame-tick / edit `ctx.effect`s on **every**
///   build — bastyde drops effects on rebuild (see Inspector's move subscription).
#[derive(Clone)]
pub struct LiveCastOverlay {
    item_id: Rc<Cell<Option<u64>>>,
    prose: Rc<RefCell<Option<String>>>,
    due: Rc<Cell<Option<Instant>>>,
    version: Signal<u64>,
    /// Last `OpenDoc::edit_gen` we already reacted to — so re-registering the
    /// edit effect on rebuild does not re-arm a debounce from a stale value.
    last_edit_gen: Rc<Cell<u64>>,
}

impl LiveCastOverlay {
    pub fn new() -> Self {
        Self {
            item_id: Rc::new(Cell::new(None)),
            prose: Rc::new(RefCell::new(None)),
            due: Rc::new(Cell::new(None)),
            version: Signal::new(0),
            last_edit_gen: Rc::new(Cell::new(0)),
        }
    }

    pub fn version(&self) -> Signal<u64> {
        self.version.clone()
    }

    /// The item the overlay is currently tracking, if any.
    pub fn focused_item(&self) -> Option<u64> {
        self.item_id.get()
    }

    /// Prose for `item_id` if a prior tick produced a **non-empty** export; `None`
    /// until then or when the export is empty (so callers keep the batch half).
    pub fn prose_for(&self, item_id: u64) -> Option<String> {
        if self.item_id.get() != Some(item_id) {
            return None;
        }
        self.prose.borrow().clone().filter(|s| !s.is_empty())
    }

    /// Focus moved: clear stale prose and schedule a refresh soon.
    ///
    /// Same-item rebuilds (pin, batch scan, version bump) do **not** re-arm —
    /// that would re-export the open doc on every Inspector rebuild. Typing is
    /// covered by [`on_edit`]; a still-pending first paint re-arms only when
    /// nothing is scheduled and prose is still missing.
    pub fn on_focus(&self, item_id: u64, wake: &Rc<Cell<Option<Instant>>>) {
        if self.item_id.get() != Some(item_id) {
            self.item_id.set(Some(item_id));
            *self.prose.borrow_mut() = None;
            // Reset edit tracking so the first effect seed for the new item
            // does not look like a fresh keystroke.
            self.last_edit_gen.set(0);
            self.arm(Instant::now() + std::time::Duration::from_millis(50), wake);
            return;
        }
        // Same focus: only re-arm if the first tick never produced prose and
        // nothing is already scheduled (e.g. effects were re-registered mid-flight).
        if self.prose.borrow().is_none() && self.due.get().is_none() {
            self.arm(Instant::now() + std::time::Duration::from_millis(50), wake);
        }
    }

    /// An edit happened on the **focused** cast item (caller must not rebuild
    /// Inspector from this): reschedule only when `edit_gen` actually advanced.
    ///
    /// `ctx.effect` re-runs with the current value on every rebuild re-register;
    /// without this gate every pin/scan rebuild would schedule a 600 ms `to_djot`.
    pub fn on_edit_gen(&self, edit_gen: u64, wake: &Rc<Cell<Option<Instant>>>) {
        if edit_gen == 0 || edit_gen == self.last_edit_gen.get() {
            return;
        }
        self.last_edit_gen.set(edit_gen);
        self.arm(Instant::now() + LIVE_CAST_DEBOUNCE, wake);
    }

    fn arm(&self, at: Instant, wake: &Rc<Cell<Option<Instant>>>) {
        self.due.set(Some(at));
        let cur = wake.get();
        match cur {
            Some(existing) if existing <= at => {}
            _ => wake.set(Some(at)),
        }
    }

    /// Frame-tick handler: if due, run `fetch_prose(item_id)` and store the result.
    ///
    /// `fetch_prose` is the only place that may call `to_djot` for cast — keep it
    /// off the typing path by only invoking this from a frame tick after idle.
    pub fn tick(&self, now: Instant, fetch_prose: impl FnOnce(u64) -> Option<String>) {
        let Some(due) = self.due.get() else {
            return;
        };
        if now < due {
            return;
        }
        self.due.set(None);
        let Some(id) = self.item_id.get() else {
            return;
        };
        let next = fetch_prose(id);
        let changed = self.prose.borrow().as_ref() != next.as_ref();
        *self.prose.borrow_mut() = next;
        if changed {
            self.version.set(self.version.get().wrapping_add(1));
        }
    }
}

impl Default for LiveCastOverlay {
    fn default() -> Self {
        Self::new()
    }
}
