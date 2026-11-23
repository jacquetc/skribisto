// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What the About box credits: the libraries Skribisto is built on, and every font
//! compiled into the binary with the licence it is shipped under.
//!
//! **The font list is written by hand**, and has to follow two places:
//!
//! - `skribisto-fonts`, the writing faces (`skribisto_fonts::all_faces`);
//! - teksilo-text, the interface faces teksilo builds in: Inter and JetBrains Mono
//!   always, Noto Sans Arabic and Hebrew under its default `fonts-arabic` and
//!   `fonts-hebrew`. teksilo-theme-material3 carries Roboto Flex, but only under a
//!   `bundled-fonts` feature nothing turns on, so it is not in the binary and not here.
//!
//! teksilo exposes neither its list nor its licence texts (see
//! `teksilo-issues/pending/bundled-font-licences-not-exposed.md`), so the texts of its
//! three licences are copied into `assets/fonts/`, and
//! `tests::the_copied_licences_match_teksilos` compares them with the crate's own.

/// A library, with the version this binary was built against when the build could
/// tell it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Library {
    pub name: &'static str,
    pub version: Option<&'static str>,
    pub url: &'static str,
}

/// A bundled font family and the licence it is shipped under.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Font {
    pub name: &'static str,
    /// The licence's own name, as its text titles it. Not translated: it is a legal title.
    pub licence_name: &'static str,
    pub licence_text: &'static str,
}

const OFL: &str = "SIL Open Font License 1.1";

/// The libraries named under "Built with", in the order they are listed.
pub(crate) fn libraries() -> Vec<Library> {
    vec![
        Library {
            name: "Teksilo",
            version: stamped(env!("SKRIBISTO_TEKSILO_VERSION")),
            url: "https://teksilo.rs",
        },
        Library {
            name: "text-document",
            version: stamped(env!("SKRIBISTO_TEXT_DOCUMENT_VERSION")),
            url: "https://github.com/ferntech-eu/text-document",
        },
        Library {
            name: "Rust",
            version: None,
            url: "https://www.rust-lang.org",
        },
    ]
}

fn stamped(version: &'static str) -> Option<&'static str> {
    Some(version).filter(|v| !v.is_empty())
}

/// Every font family compiled into the binary: the writing faces first, then the
/// interface's.
pub(crate) const FONTS: &[Font] = &[
    Font {
        name: "Literata",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/Literata-OFL.txt"),
    },
    Font {
        name: "EB Garamond",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/EBGaramond-OFL.txt"),
    },
    Font {
        name: "Source Serif 4",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/SourceSerif4-OFL.txt"),
    },
    Font {
        name: "Amiri",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/Amiri-OFL.txt"),
    },
    Font {
        name: "Inter",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/Inter-OFL.txt"),
    },
    Font {
        name: "JetBrains Mono",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/JetBrainsMono-OFL.txt"),
    },
    Font {
        name: "Noto Sans Arabic",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/NotoSans-OFL.txt"),
    },
    Font {
        name: "Noto Sans Hebrew",
        licence_name: OFL,
        licence_text: include_str!("../../../assets/fonts/NotoSans-OFL.txt"),
    },
];

/// The application's own licence, the full text shipped with the source.
pub(crate) const GPL_NAME: &str = "GNU General Public License, version 3";
pub(crate) const GPL_TEXT: &str = include_str!("../../../../../COPYING");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_licence_is_the_text_it_claims_to_be() {
        for font in FONTS {
            assert!(
                font.licence_text
                    .contains("licensed under the SIL Open Font License, Version 1.1"),
                "{}: the bundled text is not its OFL licence",
                font.name
            );
        }
        assert!(GPL_TEXT.contains("GNU GENERAL PUBLIC LICENSE"));
        assert!(GPL_TEXT.contains("Version 3, 29 June 2007"));
    }

    /// Each copyright line names the project the font comes from, so a licence
    /// copied under the wrong name shows up here.
    #[test]
    fn every_font_is_paired_with_its_own_licence() {
        let expected = [
            ("Literata", "The Literata Project Authors"),
            ("EB Garamond", "The EB Garamond Project Authors"),
            ("Source Serif 4", "The Source Serif 4 Project Authors"),
            ("Amiri", "The Amiri Project Authors"),
            ("Inter", "The Inter Project Authors"),
            ("JetBrains Mono", "The JetBrains Mono Project Authors"),
            ("Noto Sans Arabic", "notofonts/arabic"),
            ("Noto Sans Hebrew", "notofonts/hebrew"),
        ];
        assert_eq!(FONTS.len(), expected.len());
        for (font, (name, holder)) in FONTS.iter().zip(expected) {
            assert_eq!(font.name, name);
            assert!(
                font.licence_text.contains(holder),
                "{name}: its licence does not name {holder}"
            );
        }
    }

    /// The writing faces credited are the ones `skribisto-fonts` registers: each
    /// credited family it knows brings an upright and an italic face, and together
    /// they are every face it bundles.
    #[test]
    fn every_writing_face_is_credited() {
        let writing = FONTS
            .iter()
            .filter(|f| skribisto_fonts::by_family(f.name).is_some())
            .count();
        assert_eq!(
            writing * 2,
            skribisto_fonts::all_faces().len(),
            "skribisto-fonts bundles a face the About box does not credit"
        );
    }

    /// The interface fonts' licences are copies of teksilo's, found through
    /// `cargo metadata` at the version this workspace resolves. A teksilo release
    /// that changes one of them fails here, not silently in the About box.
    ///
    /// The graph is filtered to the host: offline, an unfiltered `cargo metadata`
    /// needs every platform's crates (`accesskit_ios`…), and a Linux CI runner has
    /// downloaded only its own.
    #[test]
    fn the_copied_licences_match_teksilos() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let version = std::process::Command::new(env!("CARGO"))
            .arg("-vV")
            .output()
            .expect("cargo -vV runs");
        let host = String::from_utf8_lossy(&version.stdout)
            .lines()
            .find_map(|line| line.strip_prefix("host: ").map(str::to_owned))
            .expect("cargo -vV names the host");
        let output = std::process::Command::new(env!("CARGO"))
            .args(["metadata", "--format-version", "1", "--offline"])
            .args(["--filter-platform", &host])
            .current_dir(manifest)
            .output()
            .expect("cargo metadata runs");
        assert!(
            output.status.success(),
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("cargo metadata prints JSON");
        let teksilo_text = metadata["packages"]
            .as_array()
            .and_then(|packages| {
                packages
                    .iter()
                    .find(|p| p["name"] == "teksilo-text")
                    .and_then(|p| p["manifest_path"].as_str())
            })
            .and_then(|path| {
                std::path::Path::new(path)
                    .parent()
                    .map(std::path::Path::to_path_buf)
            })
            .expect("the workspace depends on teksilo-text");
        for (ours, theirs) in [
            ("Inter-OFL.txt", "inter-LICENSE.txt"),
            ("JetBrainsMono-OFL.txt", "jetbrains-mono-LICENSE.txt"),
            ("NotoSans-OFL.txt", "noto-LICENSE.txt"),
        ] {
            let ours_text = std::fs::read_to_string(manifest.join("assets/fonts").join(ours))
                .expect("our copy is readable");
            let theirs_text = std::fs::read_to_string(teksilo_text.join("fonts").join(theirs))
                .expect("teksilo-text ships its licence");
            assert_eq!(
                ours_text, theirs_text,
                "{ours} differs from teksilo-text's {theirs}: copy it again and check the font list"
            );
        }
    }
}
