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
