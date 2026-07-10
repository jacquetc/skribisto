//! `Item/Chapter` — a chapter heading in the flat stream (its extent is the
//! following scenes): a chapter-title field above a synopsis. Shares the
//! [`heading`](super::shared::heading) form.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::heading(tab)
}
