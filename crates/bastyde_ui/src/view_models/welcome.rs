//! `WelcomeViewModel` — facade for the Launcher's Welcome content.
//!
//! Store-backed like `SettingsViewModel` (owns only the `show_welcome` signal,
//! the app handle, and the project-window factory), so the Welcome panel
//! rebuilds it anywhere from `WelcomeViewModel::new(ctx.settings(), app_ctx,
//! factory)`. The business actions (open a recent/example work, pick a file,
//! create a new work) live here, not in the view's `build()`.
//!
//! **Launcher-window model**: none of these methods touch the backend
//! directly any more. Loading/creating a work here — in the Launcher window,
//! before any project window's `App` exists — would race that window's
//! `LoadWork`/`NewWork` subscription and silently skip the seed flow
//! (`AppIds::seed`, `SingleWork::set_id`, the tree reload, …). Instead every
//! action here opens a **project window** carrying the action as a
//! [`PendingAction`], performed on that window's own first build once its
//! subscriptions are live (mirrors the pre-existing argv-launch mechanism),
//! then closes the Launcher — opening the new window *before* closing this
//! one, per the ordering rule in `main.rs`'s module docs.

use std::rc::Rc;

use bastyde::core::modal::{ModalCloseBehavior, ModalPresentation, ModalRequest};
use bastyde::prelude::*; // EventContext, Signal, tr!, FileDialogRequest/Result
use bastyde::settings::SettingsStore;
use bastyde::widgets::Toast;

use frontend::AppContext;

use crate::SHOW_WELCOME_KEY;
use crate::app::PendingAction;
use crate::new_work_panel::NewWorkPanel;
use crate::windows::ProjectWindowFactory;

#[derive(Clone)]
pub struct WelcomeViewModel {
    show_welcome: Signal<bool>,
    app_ctx: Rc<AppContext>,
    /// Builds the project window a successful open/create/import opens,
    /// before this (Launcher) window closes.
    factory: ProjectWindowFactory,
}

#[allow(dead_code)]
impl WelcomeViewModel {
    pub fn new(
        store: &SettingsStore,
        app_ctx: Rc<AppContext>,
        factory: ProjectWindowFactory,
    ) -> Self {
        Self {
            show_welcome: store.signal(SHOW_WELCOME_KEY, true),
            app_ctx,
            factory,
        }
    }

    /// The persisted "show at startup" signal — bound by the Launcher's
    /// inline checkbox and the Settings toggle (same cached
    /// `SHOW_WELCOME_KEY` signal).
    pub fn show_welcome(&self) -> Signal<bool> {
        self.show_welcome.clone()
    }

    /// Open a recent/known work by path: opens a project window carrying
    /// `PendingAction::Load(path)`, then closes the Launcher.
    ///
    /// The backup sniff (a blocking `File::open` + zip parse with no timeout —
    /// see `crate::backup::is_backup_path`) runs off the UI thread (T2-3):
    /// clicking any recent entry must never hang the app on a disconnected
    /// network/FUSE mount.
    pub fn open_work(&self, path: String, ctx: &mut EventContext) {
        let factory = self.factory.clone();
        let path_for_check = path.clone();
        ctx.spawn_local_with(
            async move {
                spawn_blocking(move || crate::backup::is_backup_path(&path_for_check))
                    .await
                    .unwrap_or(false)
            },
            move |is_backup, ectx| {
                // A backup always opens in its own instance (never as this
                // process's project) — see the backup-mode invariant.
                if is_backup {
                    ectx.request_activation_token_self(Box::new(move |tok| {
                        crate::project_switcher_button::spawn_new_process(&path, tok);
                    }));
                    return;
                }
                // Open the project window *before* closing the Launcher — the
                // ordering rule in `main.rs`'s module docs.
                ectx.open_window(factory.window_config(PendingAction::Load(path.clone())));
                ectx.close_window();
            },
        )
        .detach();
    }

    /// Open a bundled example. Its bytes are embedded in the binary; write them
    /// to a per-user temp copy (so the read-only repo original is never mutated
    /// or saved over) and load that.
    pub fn open_example(&self, file_name: &str, bytes: &[u8], ctx: &mut EventContext) {
        match write_temp_example(file_name, bytes) {
            Ok(path) => self.open_work(path, ctx),
            Err(e) => {
                ctx.show_toast(Toast::error(tr!(could_not_open_example(
                    error = e.to_string()
                ))));
            }
        }
    }

    /// "Open" button — native picker for an existing `.skrib`, then open a
    /// project window carrying `PendingAction::Load`. The backup sniff runs
    /// off the UI thread (T2-3), same rationale as [`Self::open_work`].
    pub fn pick_open(&self, ctx: &mut EventContext) {
        let factory = self.factory.clone();
        let req = FileDialogRequest::pick_file()
            .title("Open Skribisto work")
            .add_filter("Skribisto work", &["skrib"]);
        let _ = ctx.pick_file(req, move |res, ectx| {
            if let FileDialogResult::File(Some(path)) = res {
                let file = path.to_string_lossy().into_owned();
                let factory = factory.clone();
                let file_for_check = file.clone();
                ectx.spawn_local_with(
                    async move {
                        spawn_blocking(move || crate::backup::is_backup_path(&file_for_check))
                            .await
                            .unwrap_or(false)
                    },
                    move |is_backup, ectx2| {
                        if is_backup {
                            ectx2.request_activation_token_self(Box::new(move |tok| {
                                crate::project_switcher_button::spawn_new_process(&file, tok);
                            }));
                            return;
                        }
                        ectx2.open_window(factory.window_config(PendingAction::Load(file.clone())));
                        ectx2.close_window();
                    },
                )
                .detach();
            }
        });
    }

    /// "New Work" button — present the New Work modal directly in the
    /// Launcher window. There is no `App`/`work.new` global action to
    /// dispatch an intent to here (that action only exists inside an
    /// already-open project window's tree), so this builds
    /// [`NewWorkPanel::new_for_launcher`] directly: submitting the form opens
    /// a project window carrying `PendingAction::New`, then closes the
    /// Launcher — see `NewWorkViewModel::create`.
    pub fn new_work(&self, ctx: &mut EventContext) {
        let app_ctx = self.app_ctx.clone();
        let factory = self.factory.clone();
        ctx.present_modal(
            ModalRequest::deferred(move |t| {
                t.add(NewWorkPanel::new_for_launcher(
                    app_ctx.clone(),
                    factory.clone(),
                ))
            })
            .presentation(ModalPresentation::InTree)
            .title("New Work")
            .close_behavior(ModalCloseBehavior::EscapeOrClickOutside)
            .size(600, 680),
        );
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
    use std::cell::RefCell;

    use crate::app::PendingExit;
    use crate::app_ids::AppIds;
    use crate::models::BackupSettingsService;
    use crate::singles::{SingleWork, SingleWorkInfo};
    use crate::view_models::{
        BackupSchedulerViewModel, BackupSettingsViewModel, OutlineViewModel, SaveAsViewModel,
    };

    /// A minimal, fully in-memory `ProjectWindowFactory` — enough plumbing to
    /// construct a `WelcomeViewModel` in a test; these particular tests never
    /// exercise the factory's `window_config`.
    fn test_factory(app_ctx: Rc<AppContext>) -> ProjectWindowFactory {
        let ids = AppIds::new();
        let outline = OutlineViewModel::new_default(app_ctx.clone(), ids.clone());
        let single_work = SingleWork::new(app_ctx.clone());
        let single_work_info = SingleWorkInfo::new(app_ctx.clone());
        let backup_mode = Signal::new(false);
        let backup_context = Signal::new(None);
        let save_as_vm = SaveAsViewModel::new(
            app_ctx.clone(),
            ids.clone(),
            single_work.clone(),
            backup_mode.clone(),
            backup_context.clone(),
        );
        let backup_settings =
            BackupSettingsViewModel::new(BackupSettingsService::in_memory_default());
        let backup_scheduler = BackupSchedulerViewModel::new(
            app_ctx.clone(),
            backup_settings,
            single_work.clone(),
            single_work_info.clone(),
            backup_mode.clone(),
        );
        ProjectWindowFactory::new(
            app_ctx,
            outline,
            single_work,
            single_work_info,
            Signal::new(false),
            save_as_vm,
            backup_mode,
            backup_context,
            Signal::new(false),
            Signal::new(PendingExit::None),
            backup_scheduler,
            Rc::new(RefCell::new(None)),
        )
    }

    #[test]
    fn welcome_show_default_on_and_persists() {
        // A real TOML store at a unique temp path (no in-memory store exists).
        let path = std::env::temp_dir().join(format!(
            "skribisto-welcome-test-{}.toml",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);
        let store = SettingsStore::open(path.clone()).expect("open settings store");
        let app_ctx = Rc::new(AppContext::new());

        let vm = WelcomeViewModel::new(&store, app_ctx.clone(), test_factory(app_ctx.clone()));
        assert!(vm.show_welcome().get(), "defaults to on");

        vm.show_welcome().set(false);
        // A second facade over the same store observes the change (same cached
        // signal per key) — the store-backed-facade invariant.
        let vm2 = WelcomeViewModel::new(&store, app_ctx.clone(), test_factory(app_ctx));
        assert!(
            !vm2.show_welcome().get(),
            "toggle persists across instances"
        );

        let _ = std::fs::remove_file(&path);
    }
}
