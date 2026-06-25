//! App-wide commands as typed intents — the scriptable command surface.
//!
//! An intent is a named command any handler can fire (`ctx.send_intent(...)`),
//! consumed by an `Action` registered in `App::build`. Unit intents
//! (`ToggleOutline`) react on name alone, so a menu (`MenuEntry::intent(..)`) or
//! a `Shortcut` can trigger them without constructing the variant; data-bearing
//! intents (`OpenItem`) carry a typed payload recovered via `from_intent`.
//!
//! The variants are the command catalog: fired by name from menus/shortcuts, and
//! constructed by handler-driven callers added incrementally (a command palette,
//! recents, search results), so not all are constructed in app code yet.

use bastyde::IntentKind; // derive macro

#[allow(dead_code)]
#[derive(Debug, IntentKind)]
pub enum AppIntent {
    /// Toggle the binder/outline dock's visibility.
    #[name = "outline.toggle"]
    ToggleOutline,

    /// Open (or focus) the editor tab for a binder item.
    #[name = "editor.open_item"]
    OpenItem { item_id: u64, title: String },
}

#[cfg(test)]
mod tests {
    use super::*;
    use bastyde::prelude::{Intent, IntentKind as _};

    #[test]
    fn open_item_round_trips_its_payload() {
        let intent: Intent = AppIntent::OpenItem { item_id: 7, title: "Scene".into() }.into();
        match AppIntent::from_intent(&intent) {
            Some(AppIntent::OpenItem { item_id, title }) => {
                assert_eq!(*item_id, 7);
                assert_eq!(title, "Scene");
            }
            other => panic!("expected OpenItem, got {other:?}"),
        }
    }

    #[test]
    fn toggle_outline_unit_intent_bridges() {
        let intent: Intent = AppIntent::ToggleOutline.into();
        assert!(matches!(
            AppIntent::from_intent(&intent),
            Some(AppIntent::ToggleOutline)
        ));
    }
}
