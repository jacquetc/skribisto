// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `Item/Paratext` — a text that belongs to the book but not to its story: a preface, a
//! dedication, an afterword, an *achevé d'imprimer*.
//!
//! It gets the ordinary [`prose`](super::shared::prose) body, because writing one is
//! writing: the same editor, the same synopsis, the same typography. What differs is not
//! the surface but the content role underneath it — `ParatextText` rather than
//! `SceneText` — which is what keeps it out of the word count and every other statistic.
//!
//! No epigraph section appears here even though `prose` can render one: the matrix gives
//! a paratext no `EpigraphText`, so [`ContentTab::epigraph`] is `None` and the disclosure
//! never mounts.

use teksilo::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::prose(tab)
}
