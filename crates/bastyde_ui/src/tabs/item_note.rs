//! `Item/Note` — a standalone note: the dual-pane writing editor in Notes
//! typography. Shares the [`prose`](super::shared::prose) body; the Notes font is
//! selected by the tab's prose `kind`.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::prose(tab)
}
