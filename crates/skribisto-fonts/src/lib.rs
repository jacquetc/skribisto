// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The app's bundled OFL writing serifs, as raw font bytes — a single source of truth shared
//! by the desktop app (the editor's `register_editor_fonts`) and the headless exporter (the
//! PDF backend feeds these bytes to Typst). Keeping the `.ttf` blobs in one crate is why the
//! editor and an exported PDF render in the *same* face and can't silently drift apart.
//!
//! Each accessor returns the bytes of a bundled font (`include_bytes!`, so they're
//! embedded in the binary — no runtime file I/O). Every family here is OFL-1.1; the licence
//! texts live beside the app's other font licences in `bastyde_ui/assets/fonts/`.
//!
//! The three Latin families are variable fonts; Amiri — the Arabic book face — ships as
//! static upright and italic, which is all upstream publishes.

/// The upright bytes of "Literata" (the default writing serif).
pub fn literata() -> &'static [u8] {
    include_bytes!("../assets/fonts/Literata-Variable.ttf")
}
/// The italic bytes of "Literata".
pub fn literata_italic() -> &'static [u8] {
    include_bytes!("../assets/fonts/Literata-Italic-Variable.ttf")
}
/// The upright bytes of "EB Garamond".
pub fn eb_garamond() -> &'static [u8] {
    include_bytes!("../assets/fonts/EBGaramond-Variable.ttf")
}
/// The italic bytes of "EB Garamond".
pub fn eb_garamond_italic() -> &'static [u8] {
    include_bytes!("../assets/fonts/EBGaramond-Italic-Variable.ttf")
}
/// The upright bytes of "Source Serif 4".
pub fn source_serif_4() -> &'static [u8] {
    include_bytes!("../assets/fonts/SourceSerif4-Variable.ttf")
}
/// The italic bytes of "Source Serif 4".
pub fn source_serif_4_italic() -> &'static [u8] {
    include_bytes!("../assets/fonts/SourceSerif4-Italic-Variable.ttf")
}

/// The upright bytes of "Amiri", the Arabic writing face.
///
/// A Naskh book type, chosen because the alternative was no choice at all: with no bundled
/// Arabic serif, Arabic prose fell through to a generic sans fallback that clashes with every
/// Latin serif on the picker. It is a text face designed for setting books, so it sits beside
/// Literata and EB Garamond rather than under them.
pub fn amiri() -> &'static [u8] {
    include_bytes!("../assets/fonts/Amiri-Regular.ttf")
}
/// The italic bytes of "Amiri".
pub fn amiri_italic() -> &'static [u8] {
    include_bytes!("../assets/fonts/Amiri-Italic.ttf")
}

/// Every bundled face (upright + italic), in editor-registration order — the exact set
/// `register_editor_fonts` installs.
pub fn all_faces() -> [&'static [u8]; 8] {
    [
        literata(),
        literata_italic(),
        eb_garamond(),
        eb_garamond_italic(),
        source_serif_4(),
        source_serif_4_italic(),
        amiri(),
        amiri_italic(),
    ]
}

/// The upright bytes for a body-font family named as the export presets + `FontPicker` name it.
/// `None` for a family with no bundled bytes (e.g. a system font like "Times New Roman"), so a
/// caller can substitute or fall back deliberately.
pub fn by_family(family: &str) -> Option<&'static [u8]> {
    Some(match family {
        "Literata" => literata(),
        "EB Garamond" => eb_garamond(),
        "Source Serif 4" => source_serif_4(),
        "Amiri" => amiri(),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every accessor returns real font bytes.
    ///
    /// `include_bytes!` guarantees the file existed at compile time but not
    /// that it is a font — a truncated download or an HTML error page saved
    /// under a `.ttf` name would sail through the build and only fail when
    /// the shaper met it.
    #[test]
    fn every_bundled_face_is_a_real_sfnt() {
        for (name, bytes) in [
            ("Literata", literata()),
            ("Literata Italic", literata_italic()),
            ("EB Garamond", eb_garamond()),
            ("EB Garamond Italic", eb_garamond_italic()),
            ("Source Serif 4", source_serif_4()),
            ("Source Serif 4 Italic", source_serif_4_italic()),
            ("Amiri", amiri()),
            ("Amiri Italic", amiri_italic()),
        ] {
            assert!(
                bytes.len() > 10_000,
                "{name} is only {} bytes — not a font",
                bytes.len()
            );
            let magic = &bytes[..4];
            assert!(
                matches!(magic, [0x00, 0x01, 0x00, 0x00] | b"true" | b"OTTO" | b"ttcf"),
                "{name} does not start with an sfnt magic number; got {magic:?}"
            );
        }
    }

    #[test]
    fn all_faces_covers_every_accessor() {
        // The array's length is part of its type, so a face added without
        // extending `all_faces` would not fail to compile — it would just
        // never reach the editor's font registry.
        assert_eq!(all_faces().len(), 8);
        for (i, face) in all_faces().iter().enumerate() {
            assert!(!face.is_empty(), "face {i} is empty");
        }
    }

    #[test]
    fn every_picker_family_resolves_to_bundled_bytes() {
        // These strings are what the export presets and the FontPicker use.
        // A family that stops resolving silently falls back to a different
        // face in exported PDFs while still looking right in the editor.
        for family in ["Literata", "EB Garamond", "Source Serif 4", "Amiri"] {
            assert!(
                by_family(family).is_some(),
                "{family} should resolve to bundled bytes"
            );
        }
        assert!(
            by_family("Times New Roman").is_none(),
            "a system font must report no bundled bytes so callers can substitute"
        );
    }
}
