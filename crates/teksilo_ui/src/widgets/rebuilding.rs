// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A block of widgets rebuilt from scratch when a signal changes.

use teksilo::prelude::*;

/// A block rebuilt from scratch whenever one of the signals `bind` binds changes.
///
/// For the parts of a window that are a function of a view-model's state. `bind` binds
/// the signals (at `BindingLevel::Rebuild`) and `make` builds the block afresh each time.
///
/// Whatever it holds is rebuilt whole, so an editor inside one loses its caret on every
/// change: keep it to blocks a writer is not typing into, or to changes they asked for.
pub struct Rebuilding {
    bind: Box<dyn Fn(&mut BuildContext)>,
    make: Box<dyn Fn(&mut BuildContext) -> WidgetId>,
    root_child: Option<WidgetId>,
}

impl Rebuilding {
    pub fn new(
        bind: impl Fn(&mut BuildContext) + 'static,
        make: impl Fn(&mut BuildContext) -> WidgetId + 'static,
    ) -> Self {
        Self {
            bind: Box::new(bind),
            make: Box::new(make),
            root_child: None,
        }
    }
}

impl std::fmt::Debug for Rebuilding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rebuilding").finish_non_exhaustive()
    }
}

impl Widget for Rebuilding {
    fn build(&mut self, ctx: &mut BuildContext) -> Vec<WidgetId> {
        (self.bind)(ctx);
        let id = (self.make)(ctx);
        self.root_child = Some(id);
        vec![id]
    }

    fn layout_response(
        &self,
        proposal: SizeProposal,
        ctx: &LayoutContext,
    ) -> teksilo::core::widget::LayoutResponse {
        self.root_child
            .and_then(|id| ctx.child_size(id, proposal))
            .map(Into::into)
            .unwrap_or_else(|| proposal.resolve(0.0, 0.0).into())
    }

    fn children(&self) -> Vec<WidgetId> {
        self.root_child.into_iter().collect()
    }
}
