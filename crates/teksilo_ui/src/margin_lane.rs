// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The margin lane's **provider registry** — one lane, many sources of marks.
//!
//! [`MarginLane`](crate::widgets::MarginLane) knows nothing about manuscripts;
//! it draws marks it is handed. This is the half that knows what a mark *means*:
//! a provider says which column and shape it wants, when it needs recomputing,
//! and, given a surface and the document on it, produces the marks.
//!
//! ## Why a registry rather than a match
//!
//! Comments, search hits and document boundaries are four different features
//! that happen to share a strip. Hard-coding them would mean the strip grows a
//! new arm every time one is added, and would leave the commercial edition no
//! way in at all. A registry makes each of them the same shape as a dock or an
//! Analysis category, which is a seam this app already has three of.
//!
//! ## What a provider does *not* decide
//!
//! **Its colour.** A provider names a slot in the theme's chart palette and the
//! lane resolves it. Palette slots are colour-blind safe by construction and
//! carry no severity reading, and going through the theme is what stops a
//! provider shipping a mark that fails contrast on a theme it never saw. It is
//! also what stops anything painting a red stripe down the side of a
//! manuscript, which is the one thing this feature must never do.
//!
//! **Whether it is on.** Each registration gets a settings key, so the writer
//! decides. A provider only says what it *defaults* to.
//!
//! ⚠ **`id` is persisted.** It becomes `editor.margin_lane.provider.<id>` in the
//! writer's settings, so it is frozen the moment it ships.

use std::cell::RefCell;
use std::rc::Rc;

use crate::widgets::{LaneColumn, LaneMark, LaneShape};
use teksilo::prelude::LocalizedString;

use crate::docks::LabelFn;

pub mod locate;
pub mod providers;
pub mod query;
pub mod resolve;
pub mod rows;
pub mod surface;
pub mod texture;

pub use locate::{LaneExtent, locate_offset, locate_span, locator};
pub use query::{
    LaneQuery, LaneQuerySource, active_query, clear_active_query_from, set_active_query,
};
pub use resolve::{LaneCall, group_of, is_enabled, texture_enabled};
pub use rows::{RowExtent, RowExtents};
pub use surface::{LaneInputs, LaneRow, LaneRows, lane_for};

/// Register every provider the community edition owns.
///
/// Called once, at startup, from the application's own wiring. The returned handles
/// unregister on drop, so the application holds them for as long as it runs — the
/// same contract an extension is held to, deliberately, because a built-in that
/// leaked its registration would be the one case where the drop-handle shape was
/// never exercised.
pub fn install_builtin_providers() -> Vec<LaneProviderHandle> {
    providers::install()
}

/// Which text surface a lane is mounted on.
///
/// A provider lists the surfaces it makes sense on, because several do not make
/// sense everywhere: document boundaries exist only in a stream, and a synopsis
/// is rarely long enough to have a position worth mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LaneSurface {
    /// The scene / chapter-scene / note editor — the main writing surface.
    Editor,
    /// A Full Chapter / Part / Book stream: many documents on one axis.
    Stream,
    /// The search preview band.
    SearchPreview,
}

impl LaneSurface {
    /// The stable settings-key fragment for this surface.
    ///
    /// ⚠ Persisted as `editor.margin_lane.surface.<this>`, so frozen once shipped.
    pub fn key(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Stream => "stream",
            Self::SearchPreview => "search_preview",
        }
    }

    /// Every surface, for iterating the settings page.
    ///
    /// **Three, and each of them is somewhere a lane is actually mounted.** Two more
    /// were listed here and had to go, because a settings row with nothing behind it
    /// is worse than a missing feature: the writer turns it on and concludes the
    /// thing is broken.
    ///
    /// *Distraction-free* renders through `tab_pane`, the same dispatch every other
    /// writing page uses, so it already carries the [`Editor`](Self::Editor) lane —
    /// a switch of its own would either do nothing or contradict the one above it.
    ///
    /// *Synopsis* has nothing to mount beside: those editors scroll themselves
    /// (`ScrollPolicy::Auto`) rather than flowing inside an outer `ScrollArea`, so
    /// there is no sibling whose extent a lane could map, and a synopsis is a few
    /// lines by design. Adding it would mean giving the editor a scroll area it does
    /// not want, to map a position nobody loses.
    ///
    /// Both were removed while nothing had shipped and their settings keys were
    /// still free. That window is closed now: [`key`](Self::key) is persisted.
    pub fn all() -> [LaneSurface; 3] {
        [Self::Editor, Self::Stream, Self::SearchPreview]
    }
}

/// How a provider's marks stay correct as the writer works.
///
/// The choice matters more than it looks. A provider that recomputes on every
/// paint walks the document sixty times a second; one that never recomputes
/// draws marks against text that has since moved. Neither is a safe default, so
/// the provider has to say.
#[derive(Clone)]
pub enum LaneRefresh {
    /// Recompute when the document changes.
    ///
    /// The right answer for anything that owns character offsets into the
    /// document in front of the writer, and the same contract
    /// `rich_text::FindSession` already implements for search highlighting.
    OnDocumentChange,
    /// Recompute when this counter changes.
    ///
    /// For providers backed by a model rather than by a walk of the text —
    /// comments are the case: the anchors move themselves, and what changes is
    /// the set of comments, not the prose.
    OnSignal(teksilo::core::signal::Signal<u64>),
    /// Recompute only when explicitly invalidated.
    ///
    /// For anything expensive and deliberately stale — an analysis result that
    /// only moves when the writer asks it to.
    Manual,
}

impl std::fmt::Debug for LaneRefresh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnDocumentChange => f.write_str("OnDocumentChange"),
            Self::OnSignal(_) => f.write_str("OnSignal(..)"),
            Self::Manual => f.write_str("Manual"),
        }
    }
}

/// One comment's live span in the document a lane maps.
///
/// A flattened, feature-free view of what the comment margin already tracks: an
/// identity, a range, and the line assistive technology reads. Deliberately not the
/// comment row itself — a mark source has no business with a thread's replies, its
/// author or its resolved state, and a seam that handed those over would make every
/// one of them a compatibility promise.
#[derive(Debug, Clone)]
pub struct CommentAnchor {
    /// The comment's durable id. Stable across repaints, which is what an
    /// accessibility node needs.
    pub id: u64,
    /// Document character offsets, in the space cursors and `FindMatch` speak.
    pub start: usize,
    pub end: usize,
    /// What assistive technology says for this mark. Supplied by the host so the
    /// lane and the comment dock quote the same snippet.
    pub label: LocalizedString,
}

/// What a provider is handed to produce its marks.
///
/// Deliberately narrow, like every other seam context in this app: enough to
/// read the manuscript and locate a position in it, and nothing else.
pub struct LaneContext<'a> {
    pub app_ctx: &'a Rc<frontend::AppContext>,
    pub ids: &'a crate::app_ids::AppIds,
    /// Which surface this lane is on.
    pub surface: LaneSurface,
    /// The document this lane maps. For a stream this is the row's document,
    /// and the provider is called once per row.
    pub doc: &'a teksilo::text_document::TextDocument,
    /// The `BinderItem` the document belongs to.
    pub item_id: common::types::EntityId,
    /// The comment threads anchored in this document, at the offsets they are at
    /// **right now**.
    ///
    /// Passed in rather than looked up, and it is the difference between a mark that
    /// tracks a sentence and one that drifts. A comment row's stored `range_start` is
    /// only rewritten when some editor's comment margin rebuilds; the offsets here
    /// are the highlight session's, shifted on every keystroke. A provider reaching
    /// for the stored ones would put its marks a paragraph out for as long as the
    /// writer kept typing above them, and would put them nowhere at all on a stream
    /// row, which mounts no comment margin to do the rewriting.
    ///
    /// Empty where the host has no comment binding for the surface.
    pub comment_anchors: &'a [CommentAnchor],
    /// The colour this provider's marks are drawn in, already resolved from its
    /// palette slot against the live theme.
    ///
    /// Write it into every mark you return. The host overwrites the field regardless
    /// of what a provider set, which is what stops an extension painting a red
    /// stripe down the side of a manuscript, or shipping a mark that fails contrast
    /// on a theme it never saw.
    pub color: teksilo::tokens::Color,
    /// This provider's group ordinal, likewise decided by the host and likewise
    /// overwritten. Half of a mark's accessibility node identity, and all of what
    /// may merge with what.
    pub group: u16,
    /// The misspellings **currently shown** in this document, as
    /// `(character offset, length)`.
    ///
    /// Passed in for the reason the comment anchors are: these are the live set,
    /// filtered by the caret exemption, so a provider marks exactly what the
    /// reader can see underlined. A provider re-running the spell check itself
    /// would flag the word being typed, which the editor deliberately does not.
    ///
    /// Empty where the surface has no spell session, and empty for a document
    /// whose language has no dictionary installed -- which is not the same as
    /// "no mistakes", and is why a provider must not read anything into it.
    pub misspellings: &'a [(usize, usize)],
    /// **A document character offset to a fraction of the mapped extent.**
    ///
    /// The host owns this conversion and providers must not reinvent it. It
    /// goes through the editor's own laid-out geometry, because
    /// `offset / character_count` is *not* proportional to vertical position
    /// once a document has headings, images and paragraphs of unequal length —
    /// it is wrong in exactly the places a reader looks.
    ///
    /// `None` when the offset is not in this document, or before the first
    /// layout.
    pub locate: &'a dyn Fn(usize) -> Option<f32>,
}

/// Produces one provider's marks for one surface.
pub type LaneMarksFn = Rc<dyn Fn(&LaneContext<'_>) -> Vec<LaneMark>>;

/// One source of marks on the lane.
#[derive(Clone)]
pub struct LaneProviderSpec {
    /// Stable and namespaced (`"pro.beats"`).
    ///
    /// ⚠ Persisted as part of this provider's settings key. Frozen once shipped.
    pub id: String,
    /// The row label on the settings page. Resolved per build, so a runtime
    /// locale switch reaches it.
    pub label: LabelFn,
    /// One line under that label, saying what the marks mean.
    pub hint: LabelFn,
    /// Which of the three mark columns this provider draws in. Two providers in
    /// different columns can never fight for the same pixel.
    pub column: LaneColumn,
    /// The mark's shape class, which is what carries the distinction — colour is
    /// the third signal, never the first.
    pub shape: LaneShape,
    /// An index into the theme's chart palette, **not** a colour.
    ///
    /// Resolved and contrast-tuned by the host against the live theme. A
    /// provider naming a `Color` could ship a mark that fails contrast on a
    /// theme it never tested, or paint a red one.
    pub palette_slot: u8,
    /// The surfaces this provider appears on at all. A surface not listed here
    /// never shows it, whatever the writer's settings say.
    pub surfaces: &'static [LaneSurface],
    /// Whether it is on for a writer who has never touched the setting.
    ///
    /// Three on by default is the budget. A lane that lights up with everything
    /// is a cockpit, and this application is not one.
    pub default_on: bool,
    /// When its marks need recomputing.
    pub refresh: LaneRefresh,
    /// Produces the marks.
    pub marks: LaneMarksFn,
}

impl LaneProviderSpec {
    /// The settings key controlling this provider.
    ///
    /// ⚠ Written into the writer's `general.toml`. Frozen once shipped.
    pub fn settings_key(&self) -> String {
        format!("editor.margin_lane.provider.{}", self.id)
    }
}

impl std::fmt::Debug for LaneProviderSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LaneProviderSpec")
            .field("id", &self.id)
            .field("column", &self.column)
            .field("shape", &self.shape)
            .field("palette_slot", &self.palette_slot)
            .field("surfaces", &self.surfaces)
            .field("default_on", &self.default_on)
            .field("refresh", &self.refresh)
            .finish_non_exhaustive()
    }
}

struct Registered {
    namespace: String,
    spec: LaneProviderSpec,
}

// Thread-local rather than a `static RwLock`, for the same reason the segment
// and analysis-category registries are: a spec holds `Rc` closures, and `Rc` is
// not `Send`.
thread_local! {
    static PROVIDERS: RefCell<Vec<Registered>> = const { RefCell::new(Vec::new()) };
}

/// Ids the community edition reserves, which an extension may not claim.
///
/// Kept as a function rather than a list of consts so the community providers
/// and this check cannot drift: [`builtin_ids`] is what the community edition
/// registers under, and nothing else may.
///
/// Three of these are live ([`providers`]); `spelling` and `footnotes` are held
/// against features that already exist elsewhere in this application and will want
/// a column. Reserving them now costs nothing and is the only moment it can be
/// done: the id is a persisted settings key, so once an extension has shipped
/// under one, taking it back would silently repoint a writer's saved preference.
pub fn builtin_ids() -> &'static [&'static str] {
    &["comments", "search", "boundaries", "spelling", "footnotes"]
}

fn is_builtin(id: &str) -> bool {
    builtin_ids().contains(&id)
}

/// Add a source of marks to the margin lane.
///
/// Refused when `id` is one this crate already uses, or one another namespace
/// registered: the id is the persisted settings key, so two claimants would make
/// a writer's saved preference ambiguous rather than merely crowded.
///
/// The returned handle unregisters on drop; re-registering a namespace replaces
/// its entry.
pub fn register_lane_provider(
    namespace: impl Into<String>,
    spec: LaneProviderSpec,
) -> Result<LaneProviderHandle, String> {
    register(namespace, spec, Claim::Extension)
}

/// Who is registering, and therefore whether [`builtin_ids`] is a wall or a key.
///
/// The reserved ids exist to stop an *extension* claiming one, and the community
/// edition's own providers register under exactly those ids. A single entry point
/// refusing them unconditionally would have refused the community's own three, and
/// silently: [`providers::install`] logs and carries on rather than failing a
/// launch over a margin strip, so the lane would simply have come up empty.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Claim {
    /// Anything outside this crate. May not take a reserved id.
    Extension,
    /// This crate's own providers. The reserved ids are theirs.
    Builtin,
}

/// Register this crate's own provider, which is allowed the reserved ids.
pub(crate) fn register_builtin_lane_provider(
    namespace: impl Into<String>,
    spec: LaneProviderSpec,
) -> Result<LaneProviderHandle, String> {
    register(namespace, spec, Claim::Builtin)
}

fn register(
    namespace: impl Into<String>,
    spec: LaneProviderSpec,
    claim: Claim,
) -> Result<LaneProviderHandle, String> {
    let namespace = namespace.into();
    if spec.id.is_empty() {
        return Err("a lane provider needs an id".to_string());
    }
    if claim == Claim::Extension && is_builtin(&spec.id) {
        return Err(format!("lane provider id '{}' is a built-in", spec.id));
    }
    if spec.surfaces.is_empty() {
        return Err(format!(
            "lane provider '{}' lists no surfaces, so it could never appear",
            spec.id
        ));
    }
    PROVIDERS.with(|reg| {
        let mut reg = reg.borrow_mut();
        if let Some(other) = reg
            .iter()
            .find(|r| r.spec.id == spec.id && r.namespace != namespace)
        {
            return Err(format!(
                "lane provider id '{}' is already registered by '{}'",
                spec.id, other.namespace
            ));
        }
        reg.retain(|r| r.namespace != namespace);
        reg.push(Registered {
            namespace: namespace.clone(),
            spec,
        });
        Ok(LaneProviderHandle { namespace })
    })
}

/// Unregisters its provider when dropped.
#[derive(Debug)]
pub struct LaneProviderHandle {
    namespace: String,
}

impl Drop for LaneProviderHandle {
    fn drop(&mut self) {
        // `try_with`: a handle released during thread teardown must not panic.
        let _ = PROVIDERS.try_with(|reg| {
            reg.borrow_mut().retain(|r| r.namespace != self.namespace);
        });
    }
}

/// Every registered provider, in registration order.
///
/// A **snapshot**, like every registry in this seam: read when a surface is
/// built. Register at startup, on the main thread, before `run()`.
pub fn registered() -> Vec<LaneProviderSpec> {
    PROVIDERS.with(|reg| reg.borrow().iter().map(|r| r.spec.clone()).collect())
}

/// Registered providers that apply to `surface`, in registration order.
pub fn registered_for(surface: LaneSurface) -> Vec<LaneProviderSpec> {
    PROVIDERS.with(|reg| {
        reg.borrow()
            .iter()
            .filter(|r| r.spec.surfaces.contains(&surface))
            .map(|r| r.spec.clone())
            .collect()
    })
}

/// Every enabled-*capable* provider's [`LaneRefresh::OnSignal`] counter, mixed
/// into one number.
///
/// Separate from [`registered_for`] because of where it is read: the lane's
/// recompute guard, on **every** layout pass, including the ones the guard then
/// turns away. `registered_for` clones each spec -- an id `String` and four `Rc`s
/// apiece -- so asking it this question would allocate once per provider per
/// frame for the whole of a scroll, to compute a number that usually has not
/// changed.
///
/// The surface filter is the same one the resolve pass applies, so a provider
/// that could never draw here cannot make this move either. The writer's own
/// switch is deliberately *not* consulted: reading it would need the store, this
/// is called from layout, and a counter that ticks while a provider is off costs
/// one wasted resolve rather than a wrong one.
pub(crate) fn refresh_generations(surface: LaneSurface) -> u64 {
    PROVIDERS.with(|reg| {
        let mut mixed: u64 = 0;
        for registered in reg.borrow().iter() {
            if !registered.spec.surfaces.contains(&surface) {
                continue;
            }
            if let LaneRefresh::OnSignal(signal) = &registered.spec.refresh {
                // Rotate before mixing, so two providers whose counters swap
                // values are not one unchanged number.
                mixed = mixed.rotate_left(17) ^ signal.get();
            }
        }
        mixed
    })
}

/// How much width a lane will take on `surface`, right now.
///
/// **What a layout has to reserve for it, and could not ask before.** The strip
/// is the scroll area's sibling: it takes its width out of the pane the prose is
/// in. A breakpoint that decides "is there room for two columns" by counting the
/// prose's minimum and the other pane's width is short by exactly this, and the
/// column it was protecting comes out narrower than the minimum it enforced.
///
/// Zero when the lane is not shown here at all, which is the same question
/// [`resolve::is_enabled`] answers and is asked through it so the two cannot
/// disagree. Otherwise the mark columns, plus the texture column and its divider
/// when the writer has that on.
///
/// Reads the same keys the lane binds at `Rebuild`, so a layout that binds this
/// re-runs when the writer flips either switch.
pub fn reserved_width(store: &teksilo::settings::SettingsStore, surface: LaneSurface) -> f32 {
    width_of(
        resolve::is_enabled(store, surface),
        resolve::texture_enabled(store),
    )
}

/// [`reserved_width`] as a signal, for a layout that has to re-decide when it
/// changes.
///
/// Derived from the same three keys, so a writer turning the lane or the texture
/// on gets the new arrangement on that frame rather than on the next window
/// resize. Derived signals cannot be `observe`d but can be `bind_to`-ed, which is
/// what a breakpoint wants.
pub fn reserved_width_signal(
    store: &teksilo::settings::SettingsStore,
    surface: LaneSurface,
) -> teksilo::prelude::Signal<f32> {
    let lane = store.signal(
        crate::MARGIN_LANE_ENABLED_KEY,
        crate::MARGIN_LANE_ENABLED_DEFAULT,
    );
    let here = store.signal(
        &crate::margin_lane_surface_key(surface),
        crate::margin_lane_surface_default(surface),
    );
    let texture = store.signal(
        crate::MARGIN_LANE_TEXTURE_KEY,
        crate::MARGIN_LANE_TEXTURE_DEFAULT,
    );
    lane.zip3(&here, &texture)
        .map(|(lane, here, texture)| width_of(*lane && *here, *texture))
}

/// The arithmetic both readings share, so they cannot disagree about a number a
/// layout reserves and the widget then spends.
fn width_of(shown: bool, texture: bool) -> f32 {
    if !shown {
        return 0.0;
    }
    let marks = crate::widgets::DEFAULT_LANE_WIDTH;
    if texture {
        marks + crate::widgets::DEFAULT_TEXTURE_WIDTH + crate::widgets::TEXTURE_DIVIDER
    } else {
        marks
    }
}

/// The namespace that registered `id`, if any. For diagnostics and tests.
pub fn namespace_of(id: &str) -> Option<String> {
    PROVIDERS.with(|reg| {
        reg.borrow()
            .iter()
            .find(|r| r.spec.id == id)
            .map(|r| r.namespace.clone())
    })
}

/// Resolve a provider's palette slot against the live theme, tuned so the mark
/// clears WCAG 1.4.11's 3:1 against the lane's own ground.
///
/// The raw chart palette is designed for chart *areas*, where the shape is
/// large. Measured against the lane ground, several of its entries fall well
/// under 3:1 on a light theme — a 3 px mark is a graphical object needed to
/// understand the content, so the criterion applies to it. Hue and saturation
/// are preserved, because that is what carries the colour-blind separation;
/// only lightness moves.
pub fn resolve_slot(colors: &teksilo::tokens::ColorTokens, slot: u8) -> teksilo::tokens::Color {
    let palette = &colors.chart_palette;
    if palette.is_empty() {
        return colors.text_primary;
    }
    let base = palette[slot as usize % palette.len()];
    let ground = colors.surface_main;
    tune_for_contrast(base, ground)
}

/// Move `c`'s lightness toward or away from `ground` until it clears 3:1.
fn tune_for_contrast(
    c: teksilo::tokens::Color,
    ground: teksilo::tokens::Color,
) -> teksilo::tokens::Color {
    const TARGET: f32 = 3.0;
    if c.contrast_ratio(ground) >= TARGET {
        return c;
    }
    // Darken against a light ground, lighten against a dark one. Stepping rather
    // than solving: the perceptual curve is not worth inverting for a value that
    // is checked against the same ratio afterwards anyway.
    let darken = ground.relative_luminance() > 0.5;
    let mut best = c;
    for i in 1..=40 {
        let amount = i as f32 / 40.0;
        let candidate = if darken {
            c.darken(amount)
        } else {
            c.lighten(amount)
        };
        best = candidate;
        if candidate.contrast_ratio(ground) >= TARGET {
            break;
        }
    }
    best
}

/// The default label for a lane, used when nothing overrides it.
pub fn default_lane_label() -> LocalizedString {
    crate::tr!(margin_lane_name())
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::prelude::lit;

    fn spec(id: &str) -> LaneProviderSpec {
        LaneProviderSpec {
            id: id.to_string(),
            label: Rc::new(|| lit!("Test")),
            hint: Rc::new(|| lit!("A test provider")),
            column: LaneColumn::Left,
            shape: LaneShape::Square,
            palette_slot: 0,
            surfaces: &[LaneSurface::Editor],
            default_on: true,
            refresh: LaneRefresh::Manual,
            marks: Rc::new(|_| Vec::new()),
        }
    }

    /// Serialised: the registry is a thread-local and these tests share it.
    fn clear() {
        PROVIDERS.with(|r| r.borrow_mut().clear());
    }

    #[test]
    fn a_registered_provider_is_visible_to_its_surface() {
        clear();
        let _h = register_lane_provider("test.a", spec("alpha")).unwrap();
        assert_eq!(registered_for(LaneSurface::Editor).len(), 1);
        assert!(registered_for(LaneSurface::Stream).is_empty());
        clear();
    }

    #[test]
    fn dropping_the_handle_unregisters() {
        clear();
        {
            let _h = register_lane_provider("test.b", spec("beta")).unwrap();
            assert_eq!(registered().len(), 1);
        }
        assert!(registered().is_empty(), "the handle did not unregister");
        clear();
    }

    /// The id is the persisted settings key, so two claimants would make a
    /// writer's saved preference ambiguous.
    #[test]
    fn a_second_namespace_cannot_claim_a_taken_id() {
        clear();
        let _h = register_lane_provider("test.c", spec("gamma")).unwrap();
        let err = register_lane_provider("test.d", spec("gamma")).unwrap_err();
        assert!(
            err.contains("test.c"),
            "the error must name the holder: {err}"
        );
        clear();
    }

    #[test]
    fn re_registering_a_namespace_replaces_rather_than_duplicates() {
        clear();
        let _h1 = register_lane_provider("test.e", spec("one")).unwrap();
        let _h2 = register_lane_provider("test.e", spec("two")).unwrap();
        let ids: Vec<String> = registered().into_iter().map(|s| s.id).collect();
        assert_eq!(ids, vec!["two".to_string()]);
        clear();
    }

    #[test]
    fn a_builtin_id_is_refused() {
        clear();
        for id in builtin_ids() {
            assert!(
                register_lane_provider("test.f", spec(id)).is_err(),
                "'{id}' is a built-in and must be refused"
            );
        }
        clear();
    }

    /// A provider that lists no surfaces could never appear, so saying so at
    /// registration is better than a mark that silently never draws.
    #[test]
    fn a_provider_with_no_surfaces_is_refused() {
        clear();
        let mut s = spec("nowhere");
        s.surfaces = &[];
        assert!(register_lane_provider("test.g", s).is_err());
        clear();
    }

    #[test]
    fn the_settings_key_is_namespaced_under_the_lane() {
        assert_eq!(
            spec("beats").settings_key(),
            "editor.margin_lane.provider.beats"
        );
    }

    /// Every palette slot must clear 3:1 against the lane's ground, on both
    /// themes. The raw chart palette does not — it is built for chart areas,
    /// where the shape is large — so the tuning is what makes a 3 px mark
    /// legible, and this is what pins it.
    #[test]
    fn every_palette_slot_clears_contrast_on_both_themes() {
        for colors in [
            teksilo::tokens::ColorTokens::light_default(),
            teksilo::tokens::ColorTokens::dark_default(),
        ] {
            for slot in 0..colors.chart_palette.len() as u8 {
                let c = resolve_slot(&colors, slot);
                let ratio = c.contrast_ratio(colors.surface_main);
                assert!(
                    ratio >= 3.0,
                    "slot {slot} resolved to {c:?} at {ratio:.2}:1 against the lane ground"
                );
            }
        }
    }

    #[test]
    fn a_slot_past_the_palette_wraps_rather_than_panicking() {
        let colors = teksilo::tokens::ColorTokens::light_default();
        let n = colors.chart_palette.len() as u8;
        assert_eq!(resolve_slot(&colors, 0), resolve_slot(&colors, n));
        let _ = resolve_slot(&colors, 255);
    }

    #[test]
    fn every_surface_has_a_distinct_stable_key() {
        let keys: Vec<&str> = LaneSurface::all().iter().map(|s| s.key()).collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            keys.len(),
            "two surfaces share a settings key"
        );
    }
}
