// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The app's bundled OFL writing serifs, as raw font bytes — a single source of truth shared
//! by the desktop app (the editor's `register_editor_fonts`) and the headless exporter (the
//! PDF backend feeds these bytes to Typst). Keeping the `.ttf` blobs in one crate is why the
//! editor and an exported PDF render in the *same* face and can't silently drift apart.
//!
//! Each accessor returns the bytes of a bundled variable font (`include_bytes!`, so they're
//! embedded in the binary — no runtime file I/O). All three families are OFL-1.1.

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

/// Every bundled face (upright + italic), in editor-registration order — the exact set
/// `register_editor_fonts` installs.
pub fn all_faces() -> [&'static [u8]; 6] {
    [
        literata(),
        literata_italic(),
        eb_garamond(),
        eb_garamond_italic(),
        source_serif_4(),
        source_serif_4_italic(),
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
        _ => return None,
    })
}
