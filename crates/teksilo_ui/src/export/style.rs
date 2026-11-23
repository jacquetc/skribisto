// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The export style a book remembers, and the one place it is resolved.
//!
//! `Work.export_style` holds a style's id. Two surfaces read it: the Export panel opens on
//! it, and a Full Chapter, Part or Book read as exported follows it. Both resolve it here,
//! so they can never disagree about which style an id names, or about what an id this
//! machine does not know falls back to.

use anyhow::{Result, anyhow};

use frontend::AppContext;
use frontend::commands::work_commands;
use frontend::direct_access::UpdateWorkDto;
use skribisto_compiler::{Preset, builtin_presets};

use crate::export::ExportStylesViewModel;

/// Every style a writer can pick: the built-ins, then their own. Without the styles
/// view-model (a headless test) only the built-ins.
pub(crate) fn catalogue(styles: Option<&ExportStylesViewModel>) -> Vec<Preset> {
    styles.map_or_else(builtin_presets, ExportStylesViewModel::all_presets)
}

/// The style `id` names in `catalogue`, or the first built-in.
///
/// The fallback covers both "none chosen yet" (an empty id) and a style this machine does
/// not have: a user style lives in the app's settings, not in the project, so a co-author
/// opening the book may simply not own it. Falling back reads the book in a real style
/// rather than refusing to read it. `None` only if the shipped catalogue were empty.
pub(crate) fn resolve(id: &str, catalogue: &[Preset]) -> Option<Preset> {
    catalogue
        .iter()
        .find(|p| !id.is_empty() && p.id == id)
        .cloned()
        .or_else(|| builtin_presets().into_iter().next())
}

/// The style id the open Work remembers, or empty.
pub(crate) fn remembered(ctx: &AppContext, work_id: u64) -> String {
    work_commands::get_work(ctx, &work_id)
        .ok()
        .flatten()
        .map(|w| w.export_style)
        .unwrap_or_default()
}

/// Remember `id` as this book's export style, on the Work's undo stack.
///
/// A no-op when it is already the remembered one, so neither an export with an unchanged
/// style nor re-picking the same entry leaves an undo step behind. Refuses without an undo
/// stack: a `None` stack lands on the global stack 0, which nothing ever clears.
pub(crate) fn remember(
    ctx: &AppContext,
    work_id: u64,
    stack_id: Option<u64>,
    id: &str,
) -> Result<()> {
    let stack_id = stack_id.ok_or_else(|| anyhow!("no undo stack for this project"))?;
    let work = work_commands::get_work(ctx, &work_id)?
        .ok_or_else(|| anyhow!("the project is no longer open"))?;
    if work.export_style == id {
        return Ok(());
    }
    let mut dto = UpdateWorkDto::from(work);
    dto.export_style = id.to_string();
    dto.updated_at = chrono::Utc::now();
    work_commands::update_work(ctx, Some(stack_id), &dto)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_known_id_resolves_to_its_style() {
        let all = builtin_presets();
        let last = all.last().cloned().expect("built-ins");
        assert_eq!(resolve(&last.id, &all).map(|p| p.id), Some(last.id));
    }

    #[test]
    fn an_empty_or_unknown_id_falls_back_to_the_first_built_in() {
        let all = builtin_presets();
        let first = all.first().cloned().expect("built-ins");
        assert_eq!(resolve("", &all).map(|p| p.id), Some(first.id.clone()));
        assert_eq!(
            resolve("a-style-only-my-co-author-has", &all).map(|p| p.id),
            Some(first.id),
            "a style this machine lacks reads the book in a real style instead"
        );
    }
}
