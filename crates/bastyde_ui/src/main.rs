//! Skribisto desktop UI (Bastyde). Wires the Qleany backend to a Bastyde shell.

mod app;
mod models;
mod settings_panel;
#[cfg(feature = "mocks")]
mod mock;

use std::rc::Rc;
use std::sync::Arc;

use bastyde::core::event_source::{EventSource, SubscriptionHandle};
use bastyde::prelude::*;
use bastyde::settings::{AppPaths, SettingsStore};
use bastyde::widgets::{Expand, TextWidget, TitleBar, VStack, WindowFrame, framework_locales};

use frontend::AppContext;
use frontend::EventHubClient;
use frontend::common::event::{Event, Origin};

use app::App;

/// Persisted-setting keys (also read at startup in `main`).
pub const DARK_KEY: &str = "ui.dark";
pub const LOCALE_KEY: &str = "ui.locale";

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
    BastydeAppBuilder::new()
        .theme(theme)
        .application("eu", "skribisto", "Skribisto")
        .settings(SettingsBundle::new().with_window_state(true))
        .i18n(i18n)
        .install_inspector_in_debug()
        .install_toast_default()
        .event_source(EventHubSource { client })
        .initial_window(
            WindowConfig::new()
                .id("main")
                .title("Skribisto")
                .size(1200, 800)
                .decorations(DecorationsMode::CustomChrome)
                .root(move |tree, _state| {
                    let theme = tree.theme().clone();

                    // Custom Bastyde title bar (falls back to a plain label on
                    // any platform whose host is unavailable).
                    let title_bar = match tree.title_bar_host() {
                        Some(host) => tree.add_boxed(Box::new(
                            TitleBar::new(host)
                                .height(38.0)
                                .background(theme.colors.surface_pressed)
                                .leading(
                                    TextWidget::new(lit!("  Skribisto"))
                                        .style(theme.typography.body_bold.clone())
                                        .color(theme.colors.text_primary),
                                )
                                .close_action(|ctx| ctx.close_window()),
                        )),
                        None => tree.add(TextWidget::new(lit!("Skribisto"))),
                    };

                    let body = tree.add(Expand::new().child(App::new(app_ctx_root.clone())));
                    let inner =
                        tree.add(VStack::new().spacing(0.0).add_child(title_bar).add_child(body));

                    // Add edge resize handles only where the host needs the app
                    // to drive them (skipped on macOS — NSWindow handles edges).
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
