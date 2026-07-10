//! `Item/BookBegin` — the book-start marker: the book title + subtitle (no
//! synopsis). Shares the [`heading`](super::shared::heading) form.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::heading(tab)
}
