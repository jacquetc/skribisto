//! Skribisto desktop UI (Bastyde). Wires the Qleany backend to a Bastyde shell.

mod activity_icons;
mod app;
mod app_ids;
mod binder_icons;
mod binder_switcher_button;
mod intents;
mod models;
mod new_work_panel;
mod recent_projects_button;
mod settings_panel;
mod singles;
mod tabs;
mod view_models;
mod welcome_panel;

use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::event_source::{EventSource, SubscriptionHandle};
use bastyde::widgets::{Center, HStack};

use bastyde::prelude::*; // also brings the file-dialog ext + FileDialogRequest/Result
use bastyde::res;
use bastyde::settings::{AppPaths, SettingsStore};
use bastyde::widgets::primitives::icon_widget::IconMode;
use bastyde::widgets::{
    CollapsePolicy, EventContextMessageBoxExt, Expand, IconButton, IconButtonSize, IconWidget,
    MenuBar, MenuEntry, MenuModel, MessageBox, MessageBoxButtons, StandardButton, TextWidget,
    TitleBar, Toast, VStack, WindowFrame, framework_locales,
};
use recent_projects_button::RecentProjectsButton;

use frontend::AppContext;
use frontend::EventHubClient;
use frontend::commands::{
    handling_app_lifecycle_commands, work_info_commands, work_management_commands,
};
use frontend::common::entities::WorkShape;
use frontend::common::event::{Event, Origin};
use frontend::work_management::{BackupNowDto, SaveAsDto};

use app::{App, PendingExit};
use app_ids::AppIds;
use singles::{SingleWork, SingleWorkInfo};
use view_models::OutlineViewModel;

/// The currently-open project's path (from `WorkInfo`), if any.
fn current_project_path(ctx: &AppContext) -> Option<String> {
    work_info_commands::get_all_work_info(ctx)
        .ok()?
        .into_iter()
        .next()?
        .file_name
}

/// The open work's base name (no extension), or `"work"` — pre-fills the
/// "Save as" dialog's file name. A folder work's entry is `…/project.skrib`
/// (the on-disk manifest name), so its directory name is used.
fn project_stem(ctx: &AppContext) -> String {
    use std::path::Path;
    current_project_path(ctx)
        .as_deref()
        .map(|cur| {
            let p = Path::new(cur);
            let base = if p.file_name().and_then(|n| n.to_str()) == Some("project.skrib") {
                p.parent().unwrap_or(p)
            } else {
                p
            };
            base.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("work")
                .to_string()
        })
        .unwrap_or_else(|| "work".to_string())
}

/// Sanitize a `Work` title into a folder name valid on Linux, Windows and macOS.
///
/// Replaces every character forbidden on *any* of the three (`/ \ : * ? " < > |`
/// and NUL) with `_`, strips leading/trailing dots and spaces (Windows rejects
/// them), rejects the Windows reserved device names, caps the length well under
/// the 255-byte component limit, and falls back to `"work"` when nothing
/// usable remains.
fn sanitize_folder_name(raw: &str) -> String {
    /// Reserved device names on Windows (case-insensitive, with or without an
    /// extension); a folder named any of these is unusable there.
    const WINDOWS_RESERVED: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    let trim = |s: &str| {
        s.trim_matches(|c: char| c == '.' || c == ' ' || c.is_whitespace())
            .to_string()
    };
    let mut name: String = raw
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if (c as u32) < 0x20 => '_', // control chars incl. NUL
            c => c,
        })
        .collect();
    name = trim(&name);
    // Cap to 200 chars (leaves headroom under the 255-byte NTFS/ext4 limit), then
    // re-trim in case truncation exposed a trailing dot/space.
    name = trim(&name.chars().take(200).collect::<String>());
    // A reserved stem (the part before the first `.`) makes the whole name invalid.
    let stem = name.split('.').next().unwrap_or("").to_ascii_uppercase();
    if name.is_empty() || WINDOWS_RESERVED.contains(&stem.as_str()) {
        return "work".to_string();
    }
    name
}

/// Persisted-setting keys (also read at startup in `main`).
pub const DARK_KEY: &str = "ui.dark";
pub const LOCALE_KEY: &str = "ui.locale";
/// Max width (px) of the centered main-text writing column.
pub const EDITOR_WIDTH_KEY: &str = "editor.column_width";
pub const EDITOR_WIDTH_DEFAULT: f32 = 700.0;
/// When on, autosave to disk (and hide the manual Save / Ctrl+S affordances).
pub const AUTOSAVE_KEY: &str = "editor.autosave";
/// When on (default), the Welcome modal pops at startup if no work was passed
/// on the command line. Toggled in Settings and via the Welcome dialog's inline
/// checkbox; both bind the same `SettingsStore` signal.
pub const SHOW_WELCOME_KEY: &str = "ui.show_welcome";

// ── Manuscript & Fonts (Settings ▸ Editor ▸ Manuscript & Fonts) ──────────────
/// Manuscript typeface family (the writing-editor font). A persisted preference.
pub const FONT_FAMILY_KEY: &str = "editor.font_family";
pub const FONT_FAMILY_DEFAULT: &str = "Spectral";
/// Manuscript line height (leading), as a multiple of the font size.
pub const LINE_HEIGHT_KEY: &str = "editor.line_height";
pub const LINE_HEIGHT_DEFAULT: f32 = 1.72;
/// Show the synopsis pane above the manuscript in the dual-pane writing editor
/// (Skribisto's signature layout). Consumed live by `item_scene_tab`.
pub const SYNOPSIS_PANE_KEY: &str = "editor.synopsis_pane";
pub const SYNOPSIS_PANE_DEFAULT: bool = true;
/// Keep the caret line vertically centred while typing.
pub const TYPEWRITER_KEY: &str = "editor.typewriter_scroll";
pub const TYPEWRITER_DEFAULT: bool = true;
/// Highlight the sentence the caret is in.
pub const HIGHLIGHT_SENTENCE_KEY: &str = "editor.highlight_sentence";
pub const HIGHLIGHT_SENTENCE_DEFAULT: bool = false;

/// Adapts the Qleany-generated `EventHubClient` to Bastyde's `EventSource`
/// (orphan rule prevents implementing the trait directly on the client).
#[derive(Clone)]
struct EventHubSource {
    client: EventHubClient,
}

impl EventSource for EventHubSource {
    type Origin = Origin;
    type Event = Event;

    fn subscribe(
        &self,
        origin: Self::Origin,
        callback: Arc<dyn Fn(Self::Event) + Send + Sync + 'static>,
    ) -> SubscriptionHandle {
        let token = self.client.subscribe(origin, move |event| callback(event));
        SubscriptionHandle::new(token)
    }
}

fn main() {
    let app_ctx = Rc::new(AppContext::new());

    // Background event-dispatch thread.
    let client = EventHubClient::new(&app_ctx.event_hub);
    client.start(app_ctx.shutdown_rx.clone());

    // Seed the single shared Root + System frame into the (empty) store at
    // startup — before any work is opened — and keep the returned Root id to
    // point `AppIds` at it. `initialize_app` is idempotent: a later load/new
    // reuses this frame instead of creating a second Root/System.
    let init_root_id = match handling_app_lifecycle_commands::initialize_app(&app_ctx) {
        Ok(res) => Some(res.root_id),
        Err(e) => {
            eprintln!("initialize_app failed: {e:#}");
            None
        }
    };

    // Read persisted UI prefs before constructing the app (same AppPaths the
    // builder will use via `.application(...)`).
    let (dark, locale_str, autosave_init) = read_prefs();

    let theme = if dark { intui::dark() } else { intui::light() };

    let i18n = I18nConfig::new()
        .source_locale("en-US".parse().unwrap())
        .supported_locales(["en-US".parse().unwrap(), "fr-FR".parse().unwrap()])
        .compile_in(&[
            ("en-US", &[include_str!("../locales/en-US.ftl")]),
            ("fr-FR", &[include_str!("../locales/fr-FR.ftl")]),
        ])
        .user_locale(locale_str.parse().ok())
        .auto_detect_os_locale(false)
        .fallback_locale("en-US".parse().unwrap())
        .framework_locales(framework_locales());

    let app_ctx_root = app_ctx.clone();
    // The app's id-only global state (root/work/work-info/undo-stack ids). Created
    // here, shared into the outline, the singles, and the title-bar menu, and
    // registered as `app_state` so any widget can reach it.
    let ids = AppIds::new();
    // Point the app at the shared Root seeded by `initialize_app` above, so the
    // root id is known before any work is opened (a load/new refreshes it later).
    if let Some(root_id) = init_root_id {
        ids.root_id.set(Some(root_id));
    }
    // Reactive single-entity handles (Layer A). Created here so the title-bar menu
    // can bind the project title (Bug 1) and shape (Bug 2); `App::build` wires
    // their event subscriptions and re-points them on each `LoadWork`.
    let single_work = SingleWork::new(app_ctx.clone());
    let single_work_info = SingleWorkInfo::new(app_ctx.clone());
    // The outline view-model is created here (no settings dependency) so the
    // title-bar menu can bind its reactive checkmark and the whole app can reach
    // it via `ctx.app_state::<OutlineViewModel>()`.
    let outline = OutlineViewModel::new_default(app_ctx.clone(), ids.clone());
    // The title-bar menu lives outside `App` (no `ctx.settings()` there), so the
    // autosave setting is mirrored into this plain signal by `App::build` and read
    // by the menu to hide the "Save" item. Seeded from the persisted value.
    let autosave_menu = Signal::new(autosave_init);
    // Exit-guard state shared between the window close guard / Close Work menu and
    // `App` (which maintains `unsaved` and performs the deferred close on save).
    let unsaved = Signal::new(false);
    let pending_exit = Signal::new(PendingExit::None);
    // Optional `.skrib` path to open on launch (`skribisto <path>`); `App` opens it
    // once on first build.
    let initial_project = std::env::args().nth(1).filter(|s| !s.trim().is_empty());
    BastydeAppBuilder::new()
        .theme(theme)
        .application("eu", "skribisto", "Skribisto")
        .settings(SettingsBundle::new().with_window_state(true))
        .i18n(i18n)
        .install_inspector_in_debug()
        .install_automation_bridge_in_debug()
        .install_file_dialog()
        .install_toast_default()
        .event_source(EventHubSource { client })
        .app_state(ids.clone())
        .app_state(single_work.clone())
        .app_state(single_work_info.clone())
        .app_state(outline.clone())
        .initial_window(
            WindowConfig::new()
                .id("main")
                .title("Skribisto")
                .size(1200, 800)
                .decorations(DecorationsMode::CustomChrome)
                // Unsaved-changes guard for every interactive close (title-bar X,
                // Alt+F4, and the Quit menu — all route through `close_window()`).
                // Autosave on: just ensure the save runs, then close (no prompt).
                // Autosave off + unsaved: Save / Discard / Cancel. The save is
                // async, so we veto now and `App` re-issues the close on SaveWork.
                .on_close_requested({
                    let unsaved = unsaved.clone();
                    let autosave = autosave_menu.clone();
                    let pending = pending_exit.clone();
                    move |ctx| {
                        if !unsaved.get() {
                            return CloseResponse::Close;
                        }
                        if autosave.get() {
                            pending.set(PendingExit::CloseWindow);
                            return CloseResponse::Veto;
                        }
                        let pe = pending.clone();
                        ctx.present_message_box(
                            MessageBox::question(tr!(close_question()))
                                .text(tr!(unsaved_changes()))
                                .buttons(MessageBoxButtons::SaveDiscardCancel)
                                .default_button(StandardButton::Save)
                                .escape_button(StandardButton::Cancel)
                                .on_result(move |r, ctx| match r.button {
                                    StandardButton::Save => pe.set(PendingExit::CloseWindow),
                                    StandardButton::Discard => ctx.close_window_forced(),
                                    _ => {}
                                }),
                        );
                        CloseResponse::Veto
                    }
                })
                .root(move |tree, _state| {
                    let theme = tree.theme().clone();

                    // Custom Bastyde title bar with a model-driven hamburger menu
                    // in the leading slot (falls back to a plain label on any
                    // platform whose host is unavailable).
                    let title_bar = match tree.title_bar_host() {
                        Some(host) => {
                            // Model-style menu, collapsed to a hamburger (☰).
                            let menu_ctx = app_ctx_root.clone();
                            let menu_work = single_work.clone();
                            let menu_work_info = single_work_info.clone();
                            let menu_autosave = autosave_menu.clone();
                            let menu = MenuModel::new().menu(tr!(menu_file()), move |m| {
                                let file_ctx = menu_ctx.clone();
                                let folder_ctx = menu_ctx.clone();
                                let backup_ctx = menu_ctx.clone();
                                let folder_work = menu_work.clone();
                                // A work is open iff its WorkInfo shape is known.
                                let show_open = menu_work_info.shape().map(|s| s.is_some());
                                // Bug 2: offer only the *other* shape — a zip project
                                // shows "Save as folder", a folder project shows
                                // "Save as single file". Both collapse when no
                                // project is open (`shape` is `None`). Reactive via
                                // the overlay menu's `visible_when`.
                                let show_save_file =
                                    menu_work_info.shape().map(|s| *s == Some(WorkShape::Folder));
                                let show_save_folder =
                                    menu_work_info.shape().map(|s| *s == Some(WorkShape::Zip));
                                // Autosave hides the manual "Save" item (+ its Ctrl+S
                                // accelerator); the save then runs on the debounce timer.
                                let show_manual_save = menu_autosave.map(|a| !*a);
                                // New / Open route through the global `work.new` /
                                // `work.open` actions (registered in `App::build`), so
                                // the same code path serves the menu and the Ctrl+N /
                                // Ctrl+O shortcuts.
                                m.item(
                                    MenuEntry::new(tr!(menu_new_work()))
                                        .intent("work.new")
                                        .shortcut("work.new"),
                                )
                                .item(
                                    MenuEntry::new(tr!(menu_open_work()))
                                        .intent("work.open")
                                        .shortcut("work.open"),
                                )
                                .separator()
                                // Flush editors to the store + write to disk (also Ctrl+S).
                                .item(
                                    MenuEntry::new(tr!(menu_save()))
                                        .visible(show_manual_save)
                                        .intent("editor.save")
                                        .shortcut("editor.save"),
                                )
                                // Convert the open project to a single zipped `.skrib`
                                // at a user-chosen location (native save dialog).
                                .item(MenuEntry::new(tr!(menu_save_as_file())).visible(show_save_file).on_activate(
                                    move |ectx| {
                                        let ctx = file_ctx.clone();
                                        let req = FileDialogRequest::save_file()
                                            .title("Save as single .skrib file")
                                            .default_file_name(format!("{}.skrib", project_stem(&ctx)))
                                            .add_filter("Skribisto work", &["skrib"]);
                                        let _ = ectx.save_file(req, move |res, ectx2| {
                                            if let FileDialogResult::Saved(Some(path)) = res {
                                                let target = path.to_string_lossy().into_owned();
                                                match work_management_commands::save_as(
                                                    &ctx,
                                                    &SaveAsDto {
                                                        file_name: target.clone(),
                                                        as_folder: false,
                                                    },
                                                ) {
                                                    Ok(_) => ectx2.show_toast(Toast::info(
                                                        tr!(saving_as_file(target = target)),
                                                    )),
                                                    Err(e) => ectx2.show_toast(Toast::error(
                                                        tr!(save_error(error = e.to_string())),
                                                    )),
                                                };
                                            }
                                        });
                                    },
                                ))
                                // Convert the open project to an exploded folder at a
                                // user-chosen directory (native folder picker).
                                .item(MenuEntry::new(tr!(menu_save_as_folder())).visible(show_save_folder).on_activate(
                                    move |ectx| {
                                        let ctx = folder_ctx.clone();
                                        // Bug 1: the picked folder is the *parent* —
                                        // write into a subfolder named after the Work
                                        // title (sanitized), falling back to the
                                        // project file stem when the title is empty.
                                        let title = folder_work.title().get();
                                        let raw = if title.trim().is_empty() {
                                            project_stem(&ctx)
                                        } else {
                                            title
                                        };
                                        let name = sanitize_folder_name(&raw);
                                        let req = FileDialogRequest::pick_folder()
                                            .title("Choose a parent folder for the work");
                                        let _ = ectx.pick_folder(req, move |res, ectx2| {
                                            if let FileDialogResult::Folder(Some(path)) = res {
                                                let target = path
                                                    .join(&name)
                                                    .to_string_lossy()
                                                    .into_owned();
                                                match work_management_commands::save_as(
                                                    &ctx,
                                                    &SaveAsDto {
                                                        file_name: target.clone(),
                                                        as_folder: true,
                                                    },
                                                ) {
                                                    Ok(_) => ectx2.show_toast(Toast::info(
                                                        tr!(saving_as_folder(target = target)),
                                                    )),
                                                    Err(e) => ectx2.show_toast(Toast::error(
                                                        tr!(save_error(error = e.to_string())),
                                                    )),
                                                };
                                            }
                                        });
                                    },
                                ))
                                // Timestamped single-file backup next to the project.
                                .item(MenuEntry::new(tr!(menu_backup())).on_activate(
                                    move |ectx| {
                                        match work_management_commands::backup_now(
                                            &backup_ctx,
                                            &BackupNowDto { directory: String::new() },
                                        ) {
                                            Ok(_) => {
                                                ectx.show_toast(Toast::info(tr!(backing_up())));
                                            }
                                            Err(e) => {
                                                ectx.show_toast(Toast::error(tr!(backup_error(
                                                    error = e.to_string()
                                                ))));
                                            }
                                        }
                                    },
                                ))
                                // Close the open work — routed through the guarded
                                // `work.close` action (unsaved-changes prompt /
                                // autosave-ensure live in `App`).
                                .item(
                                    MenuEntry::new(tr!(menu_close_work()))
                                        .visible(show_open)
                                        .intent("work.close")
                                        .shortcut("work.close"),
                                )
                                .separator()
                                .item(MenuEntry::new(tr!(menu_welcome())).intent("welcome.show"))
                                .item(
                                    MenuEntry::new(tr!(menu_settings()))
                                        .intent("app.settings")
                                        .shortcut("app.settings"),
                                )
                                .separator()
                                .item(
                                    MenuEntry::new(tr!(menu_quit()))
                                        .intent("app.quit")
                                        .shortcut("app.quit"),
                                )
                            })
                            .menu(tr!(menu_view()), {
                                // Reflect-only checkmark: mirrors the dock's truth
                                // (`is_visible`) without writing it; the toggle is
                                // driven by the `outline.toggle` intent (F9).
                                let outline = outline.clone();
                                move |m| {
                                    m.item(
                                        MenuEntry::new(tr!(menu_outline()))
                                            .checked(outline.is_visible())
                                            .intent("outline.toggle")
                                            .shortcut("outline.toggle"),
                                    )
                                }
                            });
                            let menubar = MenuBar::from_model(menu)
                                .collapse_policy(CollapsePolicy::Always)
                                .hamburger_size(IconButtonSize::Large);

                            tree.add_boxed(Box::new(bati!(

                                TitleBar::new(host) {
                                    background: SurfaceRole::Main
                                    leading: menubar
                                    center: Expand::horizontal {
                                        HStack {
                                            spacing: 5.0
                                            alignment: bastyde::tokens::VAlignment::Center
                                            IconButton::new(
                                                IconWidget::from_raster(
                                                    res!("../../resources/icons/skribisto.png"),
                                                    25.0,
                                                )
                                                .mode(IconMode::FullColor)
                                            ) {
                                                tooltip: tr!(tooltip_welcome())
                                                size: IconButtonSize::Large
                                                on_activate_fn: |ctx| ctx.send_intent(Intent::new("welcome.show"))
                                                
                                            }
                                            RecentProjectsButton::new(app_ctx_root.clone())
                                            Expand::horizontal {
                                                Center {
                                                    TextWidget::new(lit!("Skribisto")) {
                                                        style: theme.typography.body_bold.clone()
                                                        color: TextRole::Primary
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    close_action: |ctx| ctx.close_window()
                                }
                            )))
                        }
                        None => tree.add(TextWidget::new(lit!("Skribisto"))),
                    };

                    let body =
                        tree.add(Expand::new().child(App::new(
                            app_ctx_root.clone(),
                            outline.clone(),
                            autosave_menu.clone(),
                            unsaved.clone(),
                            pending_exit.clone(),
                            initial_project.clone(),
                        )));
                    let inner =
                        tree.add(VStack::new().spacing(0.0).add_child(title_bar).add_child(body));

                    // Add edge resize handles only where the host needs the app
                    // to drive them (skipped on macOS — NSWindow handles edges).
                    // App-global commands reach the menu/shortcut via
                    // `register_action_global` (no root wrapper needed).
                    match tree.title_bar_host() {
                        Some(host) if host.needs_custom_resize_handles() => {
                            tree.add(WindowFrame::new(host).thickness(6.0).content_id(inner))
                        }
                        _ => inner,
                    }
                }),
        )
        .run();

    // Flush the recent-works MRU synchronously so a just-opened project isn't
    // lost inside the debounce window, then tear the shared Root/System frame
    // down and fire `CleanUpBeforeExit` before the event thread is stopped.
    crate::models::RecentWorkListModel::flush_now();
    if let Err(e) = handling_app_lifecycle_commands::clean_up_before_exit(&app_ctx) {
        eprintln!("clean_up_before_exit failed: {e:#}");
    }
    app_ctx.shutdown();
}

/// Best-effort read of persisted theme/locale; defaults if anything is missing.
fn read_prefs() -> (bool, String, bool) {
    let Some(paths) = AppPaths::new("eu", "skribisto", "Skribisto") else {
        return (false, "en-US".to_string(), false);
    };
    // `config_file` appends `.toml`, and the settings bundle opens its K/V
    // store under the name "general" (-> general.toml). Pass the bare name
    // here too, otherwise this reads `general.toml.toml` and never sees the
    // values the settings panel wrote, so prefs don't restore on restart.
    match SettingsStore::open(paths.config_file("general")) {
        Ok(store) => (
            store.signal(DARK_KEY, false).get(),
            store.signal(LOCALE_KEY, "en-US".to_string()).get(),
            store.signal(AUTOSAVE_KEY, false).get(),
        ),
        Err(_) => (false, "en-US".to_string(), false),
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_folder_name;

    #[test]
    fn keeps_a_clean_title_verbatim() {
        assert_eq!(sanitize_folder_name("My Novel"), "My Novel");
        assert_eq!(sanitize_folder_name("Война и мир"), "Война и мир");
    }

    #[test]
    fn replaces_cross_os_forbidden_chars() {
        // `/ \ : * ? " < > |` and control chars → `_`.
        assert_eq!(
            sanitize_folder_name("a/b\\c:d*e?f\"g<h>i|j"),
            "a_b_c_d_e_f_g_h_i_j"
        );
        assert_eq!(sanitize_folder_name("tab\there"), "tab_here");
        // All-forbidden becomes underscores (a valid, if ugly, folder name) —
        // the `project` fallback is only for empty/dots/reserved.
        assert_eq!(sanitize_folder_name("///"), "___");
    }

    #[test]
    fn strips_leading_and_trailing_dots_and_spaces() {
        assert_eq!(sanitize_folder_name("  .hidden.  "), "hidden");
        assert_eq!(sanitize_folder_name("trailing."), "trailing");
    }

    #[test]
    fn rejects_windows_reserved_names() {
        // Case-insensitive, with or without an extension.
        assert_eq!(sanitize_folder_name("CON"), "work");
        assert_eq!(sanitize_folder_name("nul"), "work");
        assert_eq!(sanitize_folder_name("LPT1.txt"), "work");
        // A reserved word as a substring is fine.
        assert_eq!(sanitize_folder_name("Console"), "Console");
    }

    #[test]
    fn falls_back_to_work_when_empty() {
        assert_eq!(sanitize_folder_name(""), "work");
        assert_eq!(sanitize_folder_name("   "), "work");
        assert_eq!(sanitize_folder_name("..."), "work");
    }

    #[test]
    fn caps_length_and_re_trims() {
        let long = "a".repeat(500);
        let out = sanitize_folder_name(&long);
        assert_eq!(out.chars().count(), 200);
    }
}
