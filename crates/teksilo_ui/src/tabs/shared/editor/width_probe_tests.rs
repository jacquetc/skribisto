// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

use super::scroll_sync::{MODE_SIDE, MODE_TOP};
use super::*;
use teksilo::core::widget_tree::WidgetTree;

/// A leaf that records how many times it was built, so a test can prove the
/// `Switcher` under [`WidthProbe`] *keeps* a branch alive across breakpoint
/// crossings instead of rebuilding it.
#[derive(Debug)]
struct Tagged {
    builds: Rc<Cell<u32>>,
}

impl Tagged {
    fn new() -> (Self, Rc<Cell<u32>>) {
        let builds = Rc::new(Cell::new(0));
        (
            Self {
                builds: builds.clone(),
            },
            builds,
        )
    }
}

impl Widget for Tagged {
    fn build(&mut self, _ctx: &mut BuildContext) -> Vec<WidgetId> {
        self.builds.set(self.builds.get() + 1);
        Vec::new()
    }

    fn layout_response(&self, proposal: SizeProposal, _ctx: &LayoutContext) -> LayoutResponse {
        proposal.resolve(10.0, 10.0).into()
    }

    fn place_children(
        &self,
        _bounds: Rect,
        _proposal: SizeProposal,
        _children: &mut [WidgetPlacement],
        _ctx: &LayoutContext,
    ) {
    }

    fn children(&self) -> Vec<WidgetId> {
        Vec::new()
    }
}

struct Probe {
    tree: WidgetTree,
    mode: Signal<usize>,
    top_builds: Rc<Cell<u32>>,
    side_builds: Rc<Cell<u32>>,
}

impl Probe {
    fn new(enabled: bool, side_width: f32) -> Self {
        let (top, top_builds) = Tagged::new();
        let (side, side_builds) = Tagged::new();
        let probe = WidthProbe::new(
            Signal::new(enabled),
            Signal::new(side_width),
            Box::new(top),
            Box::new(side),
        );
        let mode = probe.mode_signal();
        let mut tree = WidgetTree::new();
        tree.add_boxed(Box::new(probe));
        Self {
            tree,
            mode,
            top_builds,
            side_builds,
        }
    }

    /// Lay out at `width` and let the decision settle. The breakpoint is
    /// published from `place_children`, and the `Switcher` consumes it as a
    /// *deferred* rebuild binding — so a mode change costs one extra pass
    /// before the new branch is mounted. Two passes is the contract; the
    /// third proves it has converged rather than oscillating.
    fn settle(&mut self, width: f32) -> usize {
        for _ in 0..3 {
            self.tree
                .layout(teksilo::prelude::SizeProposal::exact(width, 600.0));
        }
        self.mode.get()
    }
}

/// Wide enough for a synopsis *and* a writable prose column → Side.
/// Too narrow → Top, even though the setting says Side. The setting is a
/// preference; the width is the veto.
#[test]
fn the_breakpoint_vetoes_side_when_the_prose_column_would_not_fit() {
    // threshold = side_width (280) + PROSE_MIN_WIDTH (320) = 600
    let mut wide = Probe::new(true, 280.0);
    assert_eq!(wide.settle(900.0), MODE_SIDE, "900px fits both columns");

    let mut narrow = Probe::new(true, 280.0);
    assert_eq!(
        narrow.settle(500.0),
        MODE_TOP,
        "500px cannot seat a 280px synopsis beside a 320px prose column"
    );
}

/// Placement Top is honoured at every width — the probe never promotes a tab
/// to Side on its own.
#[test]
fn top_placement_is_never_overridden_by_available_width() {
    let mut probe = Probe::new(false, 280.0);
    assert_eq!(probe.settle(1600.0), MODE_TOP);
}

/// The crossing points are asymmetric, so a width parked on the threshold
/// cannot flip the layout back and forth. Entering Side needs
/// `threshold + hysteresis`; leaving it needs to fall below
/// `threshold - hysteresis`.
#[test]
fn the_breakpoint_has_hysteresis_so_a_parked_width_cannot_oscillate() {
    let mut probe = Probe::new(true, 280.0); // threshold 600, band 576..=624
    assert_eq!(probe.settle(900.0), MODE_SIDE);

    assert_eq!(
        probe.settle(590.0),
        MODE_SIDE,
        "inside the band from above: stay in Side rather than flip on jitter"
    );
    assert_eq!(probe.settle(570.0), MODE_TOP, "below the band: leave Side");
    assert_eq!(
        probe.settle(590.0),
        MODE_TOP,
        "the same 590px that kept Side must not re-enter it — that asymmetry \
             is what makes the breakpoint stable"
    );
    assert_eq!(probe.settle(630.0), MODE_SIDE, "above the band: enter Side");
}

/// Crossing the breakpoint must not rebuild the branch being returned to.
/// `Switcher::preserves_children_on_rebuild` is what keeps a scene's caret,
/// scroll offset and spell session alive while the writer drags the window —
/// this pins that we actually get it.
#[test]
fn crossing_the_breakpoint_keeps_each_branch_alive() {
    let mut probe = Probe::new(true, 280.0);

    probe.settle(900.0);
    assert_eq!(probe.side_builds.get(), 1, "Side mounted once");
    let top_after_first = probe.top_builds.get();

    probe.settle(400.0);
    probe.settle(900.0);
    probe.settle(400.0);

    assert_eq!(
        probe.side_builds.get(),
        1,
        "Side was rebuilt on a later crossing — the Switcher is not preserving it"
    );
    assert_eq!(
        probe.top_builds.get(),
        top_after_first,
        "Top was rebuilt on a later crossing — the Switcher is not preserving it"
    );
}

/// A `Probe` whose tree carries a real settings store, so the breakpoint sees the
/// margin lane's width the way it does in the application.
///
/// Built by hand rather than through `test_support::tree_with_settings`, which
/// wants an `AppContext` and a running event hub: this widget needs neither, and
/// borrowing them would make the test fail for reasons that have nothing to do
/// with the breakpoint.
fn probe_with_settings(
    enabled: bool,
    side_width: f32,
    apply: impl FnOnce(&teksilo::settings::SettingsStore),
) -> Probe {
    use std::any::{Any, TypeId};
    use std::collections::HashMap;
    use teksilo::core::event_source::TreeAppContext;

    static N: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = N.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "skribisto_width_probe_settings_{}_{n}.toml",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let store = teksilo::settings::SettingsStore::open(path).expect("open temp settings store");
    apply(&store);

    let (top, top_builds) = Tagged::new();
    let (side, side_builds) = Tagged::new();
    let probe = WidthProbe::new(
        Signal::new(enabled),
        Signal::new(side_width),
        Box::new(top),
        Box::new(side),
    );
    let mode = probe.mode_signal();
    let mut tree = WidgetTree::new();
    let mut state: HashMap<TypeId, Box<dyn Any>> = HashMap::new();
    state.insert(
        TypeId::of::<teksilo::settings::SettingsStore>(),
        Box::new(store),
    );
    tree.set_app_context(Rc::new(TreeAppContext::empty().with_app_state(state)));
    tree.add_boxed(Box::new(probe));
    Probe {
        tree,
        mode,
        top_builds,
        side_builds,
    }
}

/// **The margin lane is inside the manuscript pane, so it comes out of the prose
/// column.** A width that seats a synopsis beside a full-width prose column does
/// not necessarily seat one beside a prose column *and* a mark strip, and the
/// breakpoint that exists to guarantee [`PROSE_MIN_WIDTH`] has to know that.
///
/// Asserted at one width for three lane states rather than at three widths: the
/// number under test is the threshold, and holding the window still is what makes
/// a regression here impossible to read as a rounding difference.
#[test]
fn the_breakpoint_reserves_the_margin_lanes_width_too() {
    // Bare threshold: 280 + 320 = 600, entered at 624 with the hysteresis.
    // The lane adds 12 (marks) or 41 (marks + texture + divider) on top.
    let width = 640.0;

    let mut no_lane = probe_with_settings(true, 280.0, |store| {
        store
            .signal(crate::MARGIN_LANE_ENABLED_KEY, true)
            .set(false);
    });
    assert_eq!(
        no_lane.settle(width),
        MODE_SIDE,
        "640px seats a 280px synopsis beside a 320px prose column"
    );

    let mut marks = probe_with_settings(true, 280.0, |store| {
        store.signal(crate::MARGIN_LANE_ENABLED_KEY, true).set(true);
        store
            .signal(
                &crate::margin_lane_surface_key(crate::margin_lane::LaneSurface::Editor),
                true,
            )
            .set(true);
        store
            .signal(crate::MARGIN_LANE_TEXTURE_KEY, true)
            .set(false);
    });
    assert_eq!(
        marks.settle(width),
        MODE_SIDE,
        "the mark columns are 12dp: 640px still leaves the prose its minimum"
    );

    let mut texture = probe_with_settings(true, 280.0, |store| {
        store.signal(crate::MARGIN_LANE_ENABLED_KEY, true).set(true);
        store
            .signal(
                &crate::margin_lane_surface_key(crate::margin_lane::LaneSurface::Editor),
                true,
            )
            .set(true);
        store.signal(crate::MARGIN_LANE_TEXTURE_KEY, true).set(true);
    });
    assert_eq!(
        texture.settle(width),
        MODE_TOP,
        "with the texture column on the strip is 41dp, and 640px can no longer \
         seat all three: before this the prose column silently came out under \
         its minimum instead"
    );
}
