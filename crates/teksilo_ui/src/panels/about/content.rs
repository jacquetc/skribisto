// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The About content both surfaces show: the modal (Help ▸ About) inside its card,
//! the Launcher's About pane on its own. One builder, so the two cannot drift
//! apart again: the Launcher's pane was a single placeholder line while the modal
//! had grown a version, an update line and a licence.
//!
//! Neither surface prints the application's name and version here: the modal's
//! header does, and the Launcher's sidebar does, right beside the pane.

use teksilo::core::accesskit::Live;
use teksilo::platform::clipboard::ClipboardHandle;
use teksilo::prelude::*;
use teksilo::widgets::{
    Button, ButtonVariant, Expand, GroupHeader, HStack, Link, Padding, TextWidget, VStack,
};

use super::credits::{self, Font, Library};
use super::news::{self, ReleaseNotes};
use crate::identity::ProjectLinks;

/// The About content, top to bottom: what the application is, who keeps it
/// current, the two actions, then the links, the credits and the licence.
pub(crate) fn about_body() -> VStack {
    let managed = crate::updates::Channel::current()
        .managed_by()
        .map(|by| match by {
            crate::updates::ManagedBy::Flathub => tr!(about_update_flathub()),
            crate::updates::ManagedBy::Distribution => tr!(about_update_distro()),
        });

    let mut body = VStack::new()
        .spacing(10.0)
        .child(TextWidget::new(tr!(about_tagline())).style(TextStyleRole::Body))
        .child_opt(managed.map(|line| {
            TextWidget::new(line)
                .style(TextStyleRole::Small)
                .color(TextRole::Secondary)
        }))
        .child(actions(news::current_notes()));

    if let Some(rows) = link_rows(&crate::identity::links()) {
        body = body.child(section(tr!(about_links()), rows));
    }
    body.child(section(
        tr!(about_built_with()),
        library_rows(&credits::libraries()),
    ))
    .child(section(tr!(about_fonts()), font_rows(credits::FONTS)))
    .child(section(tr!(about_license_section()), licence_rows()))
}

/// A titled group of rows.
fn section(title: LocalizedString, rows: VStack) -> impl Widget + 'static {
    teksu!(
        Padding::new(8.0, 0.0, 0.0, 0.0) {
            VStack {
                spacing: 6.0
                GroupHeader::new(title) {
                    style: TextStyleRole::SmallBold
                    color: TextRole::Secondary
                }
                child: rows
            }
        }
    )
}

/// **What's new in this version**, when `NEWS.yml` has a block for it, and **Copy
/// version info**, with the line that says once what the copy did.
fn actions(notes: Option<ReleaseNotes>) -> impl Widget + 'static {
    let status = Signal::new(String::new());
    let copy_status = status.clone();
    let whats_new = notes.map(|notes| {
        Button::new(tr!(about_whats_new()))
            .variant(ButtonVariant::Plain)
            .on_activate_fn(move |ctx| show_release_notes(ctx, &notes))
    });
    teksu!(
        HStack {
            spacing: 8.0
            child_opt: whats_new
            Button::new(tr!(about_copy_version())) {
                variant: ButtonVariant::Plain
                on_activate_fn: move |ctx| {
                    let clipboard = ctx.app_state::<ClipboardHandle>().cloned();
                    copy_version_info(clipboard.as_ref(), &copy_status);
                }
            }
            // Empty until the version info is copied, then said once.
            Expand::horizontal {
                TextWidget::new(lit!("")) {
                    text: status
                    style: TextStyleRole::Small
                    color: TextRole::Secondary
                    access_live: Live::Polite
                    single_line
                }
            }
        }
    )
}

/// Put [`BuildInfo::summary`](crate::version::BuildInfo::summary) on `clipboard`
/// and say on `status` whether it went: `false`, and said so, with no clipboard or
/// one that refused it.
fn copy_version_info(clipboard: Option<&ClipboardHandle>, status: &Signal<String>) -> bool {
    let summary = crate::version::BuildInfo::current().summary(&crate::identity::display_name());
    let copied = clipboard.is_some_and(|c| c.set_text(&summary).is_ok());
    let said = if copied {
        tr!(about_copy_version_done())
    } else {
        tr!(about_copy_version_failed())
    };
    status.set(said.resolve_now());
    copied
}

/// This version's notes, in the scrolling card a licence opens in. They are in
/// English whatever the interface language, and the header says so outside English.
fn show_release_notes(ctx: &mut EventContext, notes: &ReleaseNotes) {
    let in_english = crate::shared::ui_locale::current_tag()
        .is_none_or(|tag| tag == "en" || tag.starts_with("en-"));
    let subtitle = if in_english {
        tr!(about_whats_new_date(date = notes.date.clone()))
    } else {
        tr!(about_whats_new_date_english(date = notes.date.clone()))
    };
    crate::panels::license::present_text_view(
        ctx,
        tr!(about_whats_new_title(version = notes.version.clone())).resolve_now(),
        subtitle.resolve_now(),
        notes.text(),
    );
}

/// One row per address the running edition declares, `None` when it declares none.
/// The website is left out when it is the repository, as the community edition's is.
fn link_rows(links: &ProjectLinks) -> Option<VStack> {
    let website = links
        .website
        .clone()
        .filter(|site| links.github.as_ref() != Some(site));
    let rows: Vec<(LocalizedString, String)> = [
        (tr!(about_link_website()), website),
        (tr!(about_link_source()), links.github.clone()),
        (tr!(welcome_discord()), links.discord.clone()),
        (tr!(about_link_report()), links.issues.clone()),
    ]
    .into_iter()
    .filter_map(|(label, url)| url.map(|url| (label, url)))
    .collect();
    if rows.is_empty() {
        return None;
    }
    Some(
        rows.into_iter()
            .fold(VStack::new().spacing(4.0), |stack, (label, url)| {
                let open = url.clone();
                stack.child(
                    Link::new(label)
                        .tooltip(lit!(url))
                        .on_activate_fn(move |ctx| {
                            crate::shared::external_link::open_external_link(&open, ctx)
                        }),
                )
            }),
    )
}

/// "Built with": each library's name, a link to its home, and its version when the
/// build could tell it.
fn library_rows(libraries: &[Library]) -> VStack {
    libraries
        .iter()
        .fold(VStack::new().spacing(4.0), |stack, library| {
            let url = library.url;
            let version = library.version.map(|v| {
                TextWidget::new(lit!(v.to_string()))
                    .style(TextStyleRole::Small)
                    .color(TextRole::Secondary)
            });
            stack.child(teksu!(
                HStack {
                    spacing: 6.0
                    Link::new(lit!(library.name.to_string())) {
                        tooltip: lit!(url.to_string())
                        on_activate_fn: move |ctx| {
                            crate::shared::external_link::open_external_link(url, ctx)
                        }
                    }
                    child_opt: version
                }
            ))
        })
}

/// One row per bundled font: its name over its licence's, and the link that opens
/// the licence text.
fn font_rows(fonts: &'static [Font]) -> VStack {
    fonts
        .iter()
        .fold(VStack::new().spacing(6.0), |stack, font| {
            let font = *font;
            stack.child(teksu!(
                HStack {
                    spacing: 8.0
                    Expand::horizontal {
                        VStack {
                            spacing: 0.0
                            TextWidget::new(lit!(font.name.to_string())) {
                                style: TextStyleRole::Body
                            }
                            TextWidget::new(lit!(font.licence_name.to_string())) {
                                style: TextStyleRole::Small
                                color: TextRole::Secondary
                            }
                        }
                    }
                    Link::new(tr!(about_read_license())) {
                        access_label: tr!(about_read_license_of(name = font.name.to_string()))
                        on_activate_fn: move |ctx| {
                            crate::panels::license::present_text_view(
                                ctx,
                                font.name.to_string(),
                                font.licence_name.to_string(),
                                font.licence_text,
                            )
                        }
                    }
                }
            ))
        })
}

/// The application's licence, the link to its full text, and the copyright.
fn licence_rows() -> VStack {
    let name = crate::identity::display_name();
    let title = name.clone();
    teksu!(
        VStack {
            spacing: 6.0
            TextWidget::new(tr!(about_license())) {
                style: TextStyleRole::Small
            }
            Link::new(tr!(about_read_license())) {
                access_label: tr!(about_read_license_of(name = name))
                on_activate_fn: move |ctx| {
                    crate::panels::license::present_text_view(
                        ctx,
                        title.clone(),
                        credits::GPL_NAME.to_string(),
                        credits::GPL_TEXT,
                    )
                }
            }
            TextWidget::new(tr!(about_copyright(
                years = news::copyright_years(),
                holder = crate::identity::copyright_holder()
            ))) {
                style: TextStyleRole::Small
                color: TextRole::Secondary
            }
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::core::widget_tree::WidgetTree;

    fn lay_out(body: VStack, width: f32) -> WidgetTree {
        let mut tree = WidgetTree::new();
        tree.add(body);
        tree.layout(SizeProposal::exact(width, 2000.0));
        tree
    }

    /// The same content has to lay out at the modal's width and at the Launcher
    /// pane's, the two places it is shown.
    #[test]
    fn the_content_lays_out_in_both_places() {
        for width in [480.0, 640.0] {
            let tree = lay_out(about_body(), width);
            for label in ["Built with", "Fonts", "License", "Copy version info"] {
                assert!(
                    tree.find_by_label(label).is_some(),
                    "{label} missing at width {width}"
                );
            }
        }
    }

    /// Every bundled font is named, each with its own link to its licence, told
    /// apart for a screen reader by the font's name.
    #[test]
    fn every_font_has_a_link_to_its_licence() {
        let tree = lay_out(about_body(), 480.0);
        for font in credits::FONTS {
            assert!(
                tree.find_by_label(font.name).is_some(),
                "{} not named",
                font.name
            );
            let link = format!("Read the license of {}", font.name);
            assert!(tree.find_by_label(&link).is_some(), "no “{link}”");
        }
        assert!(
            tree.find_by_label("Read the license of Skribisto")
                .is_some()
        );
    }

    #[test]
    fn the_community_links_are_listed_once_each() {
        let rows = link_rows(&ProjectLinks::community()).expect("the community has links");
        let tree = lay_out(rows, 480.0);
        for label in ["Source code", "Discord", "Report a problem"] {
            assert!(tree.find_by_label(label).is_some(), "{label} missing");
        }
        // The community website is the repository: no second row for it.
        assert!(tree.find_by_label("Website").is_none());
    }

    /// An edition that declares no Discord and no tracker shows no row for them,
    /// and one that declares nothing gets no Links section at all.
    #[test]
    fn an_edition_shows_only_the_links_it_declares() {
        let links = ProjectLinks {
            website: Some("https://example.org".to_string()),
            github: Some("https://example.org/code".to_string()),
            ..ProjectLinks::default()
        };
        let tree = lay_out(link_rows(&links).expect("two links"), 480.0);
        assert!(tree.find_by_label("Website").is_some());
        assert!(tree.find_by_label("Source code").is_some());
        assert!(tree.find_by_label("Discord").is_none());
        assert!(tree.find_by_label("Report a problem").is_none());

        assert!(link_rows(&ProjectLinks::default()).is_none());
    }

    #[test]
    fn the_libraries_are_named_with_their_versions() {
        let tree = lay_out(library_rows(&credits::libraries()), 480.0);
        assert!(tree.find_by_label("Teksilo").is_some());
        assert!(tree.find_by_label("text-document").is_some());
        assert!(tree.find_by_label("Rust").is_some());
        let teksilo = env!("SKRIBISTO_TEKSILO_VERSION");
        if !teksilo.is_empty() {
            assert!(tree.find_by_label(teksilo).is_some(), "no Teksilo version");
        }
    }

    #[test]
    fn the_copyright_runs_from_the_first_year() {
        let tree = lay_out(licence_rows(), 480.0);
        let line = format!(
            "© {} {}",
            news::copyright_years(),
            crate::identity::copyright_holder()
        );
        assert!(line.starts_with("© 2015"), "{line}");
        assert!(tree.find_by_label(&line).is_some(), "no “{line}”");
    }

    /// Without a clipboard the copy fails, and says so rather than staying silent.
    #[test]
    fn a_failed_copy_says_so() {
        let status = Signal::new(String::new());
        assert!(!copy_version_info(None, &status));
        assert_eq!(status.get(), "The version info could not be copied.");
    }
}
