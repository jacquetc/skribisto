// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The project's own addresses, and which edition they belong to.
//!
//! Every surface that sends the writer outside the application (Help ▸ Website,
//! Help ▸ Report a Problem, the Launcher's Learn pane and its sidebar logos) reads
//! its address from [`crate::identity::links`], never from a constant here.
//!
//! **Why the addresses hang off the identity.** They used to be plain constants in
//! this module, and that was a trap with no symptom: an edition links `teksilo_ui`
//! as a library, so every constant compiled in here was compiled into that build
//! too, and its Report a Problem filed the reader's bug on the *community* tracker
//! without either side noticing. It is the trap [`AppIdentity::update_feed`]
//! already closed for the update check, closed the same way: the community
//! identity carries [`ProjectLinks::community`], an identity registered with
//! [`AppIdentity::new`] carries none, and a surface whose link is `None` is not
//! built at all. An edition that wants these rows declares its own addresses with
//! [`AppIdentity::with_links`].
//!
//! Every community address is `https`, which is not incidental:
//! `open_external_link` allows exactly `http`, `https` and `mailto`, so a link in
//! another scheme is refused at the door and toasts at the reader instead of
//! opening. `tests::every_community_link_is_openable` pins it.
//!
//! [`AppIdentity::update_feed`]: crate::identity::AppIdentity::update_feed
//! [`AppIdentity::new`]: crate::identity::AppIdentity::new
//! [`AppIdentity::with_links`]: crate::identity::AppIdentity::with_links

/// The community source repository.
const COMMUNITY_GITHUB: &str = "https://github.com/jacquetc/skribisto";

/// The community chat.
const COMMUNITY_DISCORD: &str = "https://discord.gg/5BSkvQmyVH";

/// The community issue tracker.
const COMMUNITY_ISSUES: &str = "https://github.com/jacquetc/skribisto/issues";

/// The community's translation-error issue form, which a report link prefills.
const COMMUNITY_TRANSLATION_FORM: &str =
    "https://github.com/jacquetc/skribisto/issues/new?template=translation-error.yml";

/// Where an edition sends a reader who leaves the application. Every field is
/// optional, and a `None` field means the surface that would use it **is not
/// built**: no menu row, no button, no registered action. A row that opened
/// somebody else's tracker would be worse than no row.
///
/// The two logo fields are named after the service rather than after the role,
/// because the Launcher draws that service's mark beside them: a field called
/// "chat" holding a Matrix room would get the Discord logo.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProjectLinks {
    /// Help ▸ Website and the Learn pane's Website row.
    pub website: Option<String>,
    /// The GitHub mark in the Launcher sidebar.
    pub github: Option<String>,
    /// The Discord mark in the Launcher sidebar, and the Learn pane's Discord row.
    pub discord: Option<String>,
    /// Help ▸ Report a Problem: the issue **list**, not a new-issue form, so the
    /// reader sees the open reports before writing one (CONTRIBUTING asks for it).
    pub issues: Option<String>,
    /// Help ▸ Report a Translation Error…: the new-issue address of the edition's
    /// translation form, template included. A report link adds the form's fields to
    /// it, so it must already carry a query. Unset, the action and every door to it
    /// are not built.
    pub translation_form: Option<String>,
}

impl ProjectLinks {
    /// The community edition's addresses. The website is the repository: the
    /// project has no separate home page for its contributors.
    pub fn community() -> Self {
        Self {
            website: Some(COMMUNITY_GITHUB.to_string()),
            github: Some(COMMUNITY_GITHUB.to_string()),
            discord: Some(COMMUNITY_DISCORD.to_string()),
            issues: Some(COMMUNITY_ISSUES.to_string()),
            translation_form: Some(COMMUNITY_TRANSLATION_FORM.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all(links: &ProjectLinks) -> Vec<&Option<String>> {
        // Destructured so that a field added later fails to compile here until it is
        // listed, rather than slipping past both tests below.
        let ProjectLinks {
            website,
            github,
            discord,
            issues,
            translation_form,
        } = links;
        vec![website, github, discord, issues, translation_form]
    }

    #[test]
    fn every_community_link_is_openable() {
        for link in all(&ProjectLinks::community()) {
            let url = link
                .as_deref()
                .expect("the community edition sets every link");
            assert!(
                crate::shared::external_link::is_openable(url),
                "{url} would be refused by open_external_link"
            );
            assert!(url.starts_with("https://"), "{url} is not https");
        }
    }

    /// A report link appends the form's fields with `&`, which needs the template's
    /// `?` already there.
    #[test]
    fn the_translation_form_names_its_template() {
        let form = ProjectLinks::community()
            .translation_form
            .unwrap_or_default();
        assert!(form.contains("/issues/new?template="), "{form}");
        let file = form.rsplit('=').next().unwrap_or_default();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../.github/ISSUE_TEMPLATE")
            .join(file);
        assert!(path.is_file(), "{} does not exist", path.display());
    }

    #[test]
    fn the_default_has_no_link_at_all() {
        assert!(all(&ProjectLinks::default()).iter().all(|l| l.is_none()));
    }

    /// **The drift guard.** No module outside this one may name a community
    /// address. A site that keeps its own copy compiles, works in the community
    /// build, and sends an edition's readers to the community project: exactly the
    /// trap this module exists to close, and invisible from the community side. A
    /// directory walk rather than a list, so a file nobody thought of is covered.
    #[test]
    fn no_module_names_a_community_address() {
        use std::path::{Path, PathBuf};

        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let Ok(entries) = std::fs::read_dir(dir) else {
                return;
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    out.push(path);
                }
            }
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut files = Vec::new();
        walk(&root, &mut files);
        assert!(!files.is_empty(), "the walk found no source at {root:?}");

        let this = root.join("shared").join("project_links.rs");
        let needles = [COMMUNITY_GITHUB, COMMUNITY_DISCORD];
        let offenders: Vec<String> = files
            .iter()
            .filter(|f| **f != this)
            .filter_map(|f| {
                let text = std::fs::read_to_string(f).ok()?;
                needles
                    .iter()
                    .any(|n| text.contains(n))
                    .then(|| f.display().to_string())
            })
            .collect();
        assert!(
            offenders.is_empty(),
            "these files name a community address; read it from identity::links() instead: \
             {offenders:?}"
        );
    }
}
