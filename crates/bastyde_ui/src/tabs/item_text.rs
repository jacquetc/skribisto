//! `Item/Text` — an inert text marker carrying no editable content: opens the
//! shared [`placeholder`](super::shared::placeholder).

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::placeholder(tab)
}
