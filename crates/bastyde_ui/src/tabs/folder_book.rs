//! `Folder/Book` — the manuscript root container: a `SegmentedControl` over the
//! shared **Synopsis** view (book title + subtitle + synopsis), with Corkboard and
//! Overview as 🚧 future segments. Shares the
//! [`folder_segmented`](super::shared::folder_segmented) body with Folder/Part.

use bastyde::prelude::*;

use super::{ContentTab, shared};

pub fn render(tab: &ContentTab) -> Box<dyn Widget> {
    shared::folder_segmented(tab)
}
