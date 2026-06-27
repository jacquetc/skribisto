//! Skribisto desktop UI (Bastyde). Wires the Qleany backend to a Bastyde shell.

mod app;
mod editor_tab;
mod intents;
mod models;
mod recent_projects_button;
mod settings_panel;
mod view_models;

use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::event_source::{EventSource, SubscriptionHandle};
use bastyde::widgets::{Center, HStack, ImageWidget};

use bastyde::core::modal::ModalRequest;
use bastyde::prelude::*;
use bastyde::res;
use bastyde::settings::{AppPaths, SettingsStore};
use bastyde::widgets::{
    CollapsePolicy, Expand, IconButtonSize, MenuBar, MenuEntry, MenuModel, TextWidget, TitleBar,
    Toast, VStack, WindowFrame, framework_locales,
};
use recent_projects_button::RecentProjectsButton;

use frontend::AppContext;
use frontend::EventHubClient;
use frontend::commands::work_management_commands;
use frontend::common::event::{Event, Origin};
use frontend::work_management::LoadWorkDto;

use app::App;
use settings_panel::SettingsPanel;
use view_models::OutlineViewModel;

/// Path to the bundled sample project (opened from the File menu).
fn sample_project_path() -> String {
    format!(
        "{}/../../resources/test/skribisto_test_project.skrib",
        env!("CARGO_MANIFEST_DIR")
    )
}

/// Persisted-setting keys (also read at startup in `main`).
pub const DARK_KEY: &str = "ui.dark";
pub const LOCALE_KEY: &str = "ui.locale";
/// Max width (px) of the centered main-text writing column.
pub const EDITOR_WIDTH_KEY: &str = "editor.column_width";
pub const EDITOR_WIDTH_DEFAULT: f32 = 700.0;

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

    // Read persisted UI prefs before constructing the app (same AppPaths the
    // builder will use via `.application(...)`).
    let (dark, locale_str) = read_prefs();

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
    // The outline view-model is created here (no settings dependency) so the
    // title-bar menu can bind its reactive checkmark and the whole app can reach
    // it via `ctx.app_state::<OutlineViewModel>()`.
    let outline = OutlineViewModel::new_default(app_ctx.clone());
    BastydeAppBuilder::new()
        .theme(theme)
        .application("eu", "skribisto", "Skribisto")
        .settings(SettingsBundle::new().with_window_state(true))
        .i18n(i18n)
        .install_inspector_in_debug()
        .install_toast_default()
        .event_source(EventHubSource { client })
        .app_state(outline.clone())
        .initial_window(
            WindowConfig::new()
                .id("main")
                .title("Skribisto")
                .size(1200, 800)
                .decorations(DecorationsMode::CustomChrome)
                .root(move |tree, _state| {
                    let theme = tree.theme().clone();

                    // Custom Bastyde title bar with a model-driven hamburger menu
                    // in the leading slot (falls back to a plain label on any
                    // platform whose host is unavailable).
                    let title_bar = match tree.title_bar_host() {
                        Some(host) => {
                            // Model-style menu, collapsed to a hamburger (☰).
                            let menu_ctx = app_ctx_root.clone();
                            let menu = MenuModel::new().menu(lit!("File"), move |m| {
                                m.item(MenuEntry::new(lit!("Open Project")).on_activate(
                                    move |ectx| {
                                        if let Err(e) = work_management_commands::load_work(
                                            &menu_ctx,
                                            &LoadWorkDto { file_name: sample_project_path() },
                                        ) {
                                            ectx.show_toast(Toast::error(lit!(format!(
                                                "Could not open project: {e}"
                                            ))));
                                        }
                                    },
                                ))
                                .item(MenuEntry::new(lit!("Settings")).on_activate(|ectx| {
                                    ectx.present_modal(
                                        ModalRequest::deferred(|t| t.add(SettingsPanel::new()))
                                            .presentation(ModalPresentation::InTree)
                                            .title("Settings")
                                            .size(520, 320),
                                    );
                                }))
                                .separator()
                                .item(
                                    MenuEntry::new(lit!("Quit"))
                                        .on_activate(|ectx| ectx.close_window()),
                                )
                            })
                            .menu(lit!("View"), {
                                // Reflect-only checkmark: mirrors the dock's truth
                                // (`is_visible`) without writing it; the toggle is
                                // driven by the `outline.toggle` intent (Ctrl+B).
                                let outline = outline.clone();
                                move |m| {
                                    m.item(
                                        MenuEntry::new(lit!("Outline"))
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
                                            ImageWidget::new(res!("../../resources/icons/skribisto.png")) {
                                                alt: lit!("Skribisto")
                                                size: 25.0, 25.0
                                                a11y_hidden
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
                        tree.add(Expand::new().child(App::new(app_ctx_root.clone(), outline.clone())));
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

    app_ctx.shutdown();
}

/// Best-effort read of persisted theme/locale; defaults if anything is missing.
fn read_prefs() -> (bool, String) {
    let Some(paths) = AppPaths::new("eu", "skribisto", "Skribisto") else {
        return (false, "en-US".to_string());
    };
    // `config_file` appends `.toml`, and the settings bundle opens its K/V
    // store under the name "general" (-> general.toml). Pass the bare name
    // here too, otherwise this reads `general.toml.toml` and never sees the
    // values the settings panel wrote, so prefs don't restore on restart.
    match SettingsStore::open(paths.config_file("general")) {
        Ok(store) => (
            store.signal(DARK_KEY, false).get(),
            store.signal(LOCALE_KEY, "en-US".to_string()).get(),
        ),
        Err(_) => (false, "en-US".to_string()),
    }
}
