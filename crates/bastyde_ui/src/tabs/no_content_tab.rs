//! Placeholder tab for contentless rows: Item/BookEnd (a delimiter) and
//! Item/Text (an inert marker). They carry no editable content, so opening one
//! shows a quiet explanatory label rather than an empty editor.

use bastyde::prelude::*;
use bastyde::widgets::{Center, TextWidget};

use super::{ContentTab, parts};

pub fn render(_tab: &ContentTab) -> Box<dyn Widget> {
    parts::tab_backdrop(bati!(
        Center {
            child: TextWidget::new(lit!("This item has no editable content.")) {
                color: TextRole::Secondary
            }
        }
    ))
}
