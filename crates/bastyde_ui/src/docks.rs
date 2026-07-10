//! Dock widgets placed on the app's `DockingLayout`. Each dock owns its own
//! content builder and packages it as a `DockWidget` for `App` to mount on a
//! side; `App` only wires the cross-view-model effects around them.
//!
//! Currently the sole dock is the binder [`outline`]; further container docks
//! (corkboard, search results, …) will land here beside it.

pub mod create_split_button;
pub mod inspector;
pub mod outline;
