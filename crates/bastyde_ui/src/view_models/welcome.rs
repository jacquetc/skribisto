//! `WelcomeViewModel` — facade for the Welcome start screen.
//!
//! Store-backed like `SettingsViewModel` (owns only the `show_welcome` signal +
//! the app handle), so the Welcome panel rebuilds it anywhere from
//! `WelcomeViewModel::new(ctx.settings(), app_ctx)`. The business actions (open a
//! recent/example work, pick a file, create a new work) live here, not in the
//! view's `build()`.

use std::rc::Rc;

use bastyde::prelude::*; // EventContext, Signal, tr!, FileDialogRequest/Result
use bastyde::settings::SettingsStore;
use bastyde::widgets::Toast;

use frontend::AppContext;
use frontend::commands::work_management_commands;
use frontend::work_management::LoadWorkDto;

use crate::SHOW_WELCOME_KEY;
use crate::intents::AppIntent;

#[derive(Clone)]
pub struct WelcomeViewModel {
    show_welcome: Signal<bool>,
    app_ctx: Rc<AppContext>,
}

#[allow(dead_code)]
impl WelcomeViewModel {
    pub fn new(store: &SettingsStore, app_ctx: Rc<AppContext>) -> Self {
        Self {
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            app_ctx,
        }
    }

    /// The persisted "show at startup" signal — bound by the dialog's inline
    /// checkbox and the Settings toggle (same cached `SHOW_WELCOME_KEY` signal).
    pub fn show_welcome(&self) -> Signal<bool> {
        self.show_welcome.clone()
    }

    /// Open a recent/known work by path. Dismisses the modal first so the loaded
    /// work is revealed behind it (mirrors `RecentProjectsButton`'s row click).
    pub fn open_work(&self, path: String, ctx: &mut EventContext) {
        ctx.dismiss_modal();
        if let Err(e) =
            work_management_commands::load_work(&self.app_ctx, &LoadWorkDto { file_name: path })
        {
            ctx.show_toast(Toast::error(tr!(could_not_open_work(error = e.to_string()))));
        }
    }

    /// Open a bundled example. Its bytes are embedded in the binary; write them
    /// to a per-user temp copy (so the read-only repo original is never mutated
    /// or saved over) and load that.
    pub fn open_example(&self, file_name: &str, bytes: &[u8], ctx: &mut EventContext) {
        match write_temp_example(file_name, bytes) {
            Ok(path) => self.open_work(path, ctx),
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(could_not_open_example(error = e.to_string()))));
            }
        }
    }

    /// "Open" button — native picker for an existing `.skrib`, then load.
    pub fn pick_open(&self, ctx: &mut EventContext) {
        let app_ctx = self.app_ctx.clone();
        let req = FileDialogRequest::pick_file()
            .title("Open Skribisto work")
            .add_filter("Skribisto work", &["skrib"]);
        let _ = ctx.pick_file(req, move |res, ectx| {
            if let FileDialogResult::File(Some(path)) = res {
                // NOT `dismiss_modal()`: this runs in the async file-dialog
                // result callback, whose `EventContext` is anchored at the tree
                // root (no source widget), so `dismiss_modal`'s walk up to the
                // enclosing modal overlay finds nothing and silently no-ops. The
                // Welcome modal is the topmost overlay when the native picker
                // returns, so pop it directly.
                ectx.dismiss_top_overlay();
                let file = path.to_string_lossy().into_owned();
                if let Err(e) =
                    work_management_commands::load_work(&app_ctx, &LoadWorkDto { file_name: file })
                {
                    ectx.show_toast(Toast::error(tr!(could_not_open_work(error = e.to_string()))));
                }
            }
        });
    }

    /// "New Work" button — dismiss the Welcome modal and open the New Work
    /// dialog. Routed through the global `work.new` command (App presents the
    /// `NewWorkPanel` modal), so this VM stays decoupled from that peer.
    pub fn new_work(&self, ctx: &mut EventContext) {
        ctx.dismiss_modal();
        ctx.send_intent(AppIntent::NewWork);
    }
}

/// Write an embedded example's bytes to a per-user temp dir (always rewritten,
/// so a stale/partial copy never blocks a fresh open) and return its path.
fn write_temp_example(file_name: &str, bytes: &[u8]) -> std::io::Result<String> {
    let mut dir = std::env::temp_dir();
    dir.push("skribisto-examples");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(file_name);
    std::fs::write(&path, bytes)?;
    Ok(path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn welcome_show_default_on_and_persists() {
        // A real TOML store at a unique temp path (no in-memory store exists).
        let path = std::env::temp_dir()
            .join(format!("skribisto-welcome-test-{}.toml", std::process::id()));
        let _ = std::fs::remove_file(&path);
        let store = SettingsStore::open(path.clone()).expect("open settings store");
        let app_ctx = Rc::new(AppContext::new());

        let vm = WelcomeViewModel::new(&store, app_ctx.clone());
        assert!(vm.show_welcome().get(), "defaults to on");

        vm.show_welcome().set(false);
        // A second facade over the same store observes the change (same cached
        // signal per key) — the store-backed-facade invariant.
        let vm2 = WelcomeViewModel::new(&store, app_ctx);
        assert!(!vm2.show_welcome().get(), "toggle persists across instances");

        let _ = std::fs::remove_file(&path);
    }
}
