// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The keyboard chords a help page names, written the way a Mac writes them on a Mac.
//!
//! The pages spell a chord in their own language's key names, as a PC keyboard reads:
//! `` `Ctrl+S` ``, `` `Strg+Umschalt+E` ``, `` `Ctrl+Alt+Maj+M` ``. A shortcut declared
//! on `Ctrl` is the platform's primary accelerator, so on macOS the app answers ⌘S and
//! the page has to say so; it used to say "Press Ctrl+S" to a Mac writer. On macOS,
//! [`platform_chords`] rewrites every inline code span that reads as such a chord with
//! teksilo's own [`format_keystroke`], the rendering the menus and the shortcuts sheet
//! use: glyphs (`⇧⌘E`), which read the same in every language. Elsewhere the page is
//! right as written, in its own words, and is left alone, as is any code span that is
//! not a chord.
//!
//! The chord is read from the page rather than looked up by command: the Help window is
//! a window of its own, and its shortcut registry does not hold the project window's
//! commands, so a lookup would find nothing to show there.

use teksilo::core::event::{Key, Modifiers};
use teksilo::core::shortcut::KeyStroke;
use teksilo::widgets::keystroke_format::format_keystroke;

/// `source` with each `` `Ctrl+…` `` chord written for this platform: rewritten on
/// macOS, unchanged elsewhere.
pub(crate) fn platform_chords(source: &str) -> String {
    if cfg!(target_os = "macos") {
        rewrite_chords(source)
    } else {
        source.to_string()
    }
}

/// Every chord span in `source`, rewritten with [`format_keystroke`].
fn rewrite_chords(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(open) = rest.find('`') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        // A double backtick opens a span that may itself hold a backtick; it is never
        // a chord, so copy it through untouched along with its closing fence.
        if let Some(inner) = after.strip_prefix('`') {
            let Some(close) = inner.find("``") else {
                out.push_str(&rest[open..]);
                return out;
            };
            let end = open + 1 + 1 + close + 2;
            out.push_str(&rest[open..end]);
            rest = &rest[end..];
            continue;
        }
        let Some(close) = after.find('`') else {
            out.push_str(&rest[open..]);
            return out;
        };
        let text = &after[..close];
        match parse_chord(text) {
            Some(chord) => {
                out.push('`');
                out.push_str(&format_keystroke(chord));
                out.push('`');
            }
            None => {
                out.push('`');
                out.push_str(text);
                out.push('`');
            }
        }
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

/// A chord as the pages spell it: modifiers then one key, joined by `+`, in any of the
/// pages' languages. `Ctrl` (`Strg`) is the primary accelerator, as `KeyStroke::ctrl`
/// declares it; `Alt` and `Shift` (`Maj`, `Mayús`, `Maiusc`, `Umschalt`) are
/// themselves. Anything else is not a chord.
fn parse_chord(text: &str) -> Option<KeyStroke> {
    let mut parts: Vec<&str> = text.split('+').collect();
    let key = key_named(parts.pop()?)?;
    if parts.is_empty() {
        return None;
    }
    let mut modifiers = Modifiers::NONE;
    for part in parts {
        modifiers = modifiers
            | match part {
                "Ctrl" | "Strg" => Modifiers::COMMAND,
                "Alt" => Modifiers::ALT,
                "Shift" | "Maj" | "Mayús" | "Maiusc" | "Umschalt" => Modifiers::SHIFT,
                _ => return None,
            };
    }
    Some(KeyStroke::new(key, modifiers))
}

fn key_named(name: &str) -> Option<Key> {
    const LETTERS: [Key; 26] = [
        Key::A,
        Key::B,
        Key::C,
        Key::D,
        Key::E,
        Key::F,
        Key::G,
        Key::H,
        Key::I,
        Key::J,
        Key::K,
        Key::L,
        Key::M,
        Key::N,
        Key::O,
        Key::P,
        Key::Q,
        Key::R,
        Key::S,
        Key::T,
        Key::U,
        Key::V,
        Key::W,
        Key::X,
        Key::Y,
        Key::Z,
    ];
    let mut chars = name.chars();
    if let (Some(c), None) = (chars.next(), chars.next())
        && c.is_ascii_uppercase()
    {
        return Some(LETTERS[(c as u8 - b'A') as usize]);
    }
    Some(match name {
        "Enter" | "Entrée" | "Intro" | "Invio" | "Eingabe" => Key::Enter,
        "Space" => Key::Space,
        "Tab" => Key::Tab,
        "Escape" => Key::Escape,
        "Backspace" => Key::Backspace,
        "Delete" => Key::Delete,
        "Home" => Key::Home,
        "End" => Key::End,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn written(chord: KeyStroke) -> String {
        format!("`{}`", format_keystroke(chord))
    }

    /// Each chord the shipped pages name comes out in this platform's spelling.
    #[test]
    fn a_chord_is_written_for_this_platform() {
        let cases = [
            ("`Ctrl+S`", KeyStroke::command(Key::S)),
            ("`Ctrl+D`", KeyStroke::command(Key::D)),
            ("`Ctrl+Enter`", KeyStroke::command(Key::Enter)),
            ("`Ctrl+Shift+E`", KeyStroke::command_shift(Key::E)),
            (
                "`Ctrl+Alt+M`",
                KeyStroke::new(Key::M, Modifiers::COMMAND | Modifiers::ALT),
            ),
            (
                "`Ctrl+Alt+Shift+M`",
                KeyStroke::new(
                    Key::M,
                    Modifiers::COMMAND | Modifiers::ALT | Modifiers::SHIFT,
                ),
            ),
        ];
        for (source, chord) in cases {
            assert_eq!(rewrite_chords(source), written(chord), "{source}");
        }
        // The pages' own languages name the same chords.
        assert_eq!(
            rewrite_chords("`Strg+Umschalt+E`"),
            rewrite_chords("`Ctrl+Shift+E`")
        );
        assert_eq!(
            rewrite_chords("`Ctrl+Alt+Maj+M`"),
            rewrite_chords("`Ctrl+Alt+Shift+M`")
        );
        assert_eq!(
            rewrite_chords("`Ctrl+Intro`"),
            rewrite_chords("`Ctrl+Enter`")
        );
        let page = "Press `Ctrl+S` to save.";
        if cfg!(target_os = "macos") {
            assert_eq!(platform_chords(page), rewrite_chords(page));
        } else {
            assert_eq!(
                platform_chords(page),
                page,
                "off macOS a page is right as written"
            );
        }
    }

    /// Code that is not a chord is left exactly as written: a file name, a single
    /// key with no modifier, an unknown modifier, a double-backtick span.
    #[test]
    fn other_code_is_untouched() {
        for source in [
            "choose *Word* (`.docx`)",
            "press `F11`",
            "`Cmd+S` is not how a page spells it",
            "`Ctrl+`",
            "a ``literal ` backtick`` span",
            "an unclosed `Ctrl+S",
            "no code at all",
        ] {
            assert_eq!(rewrite_chords(source), source, "{source}");
        }
    }

    /// Every chord a built-in page names is one `rewrite_chords` reads, in every
    /// language: a page that wrote `Cmd+S`, `Ctrl-S` or a key name missing above
    /// would reach a Mac writer unrewritten, and nothing else would notice.
    #[test]
    fn every_chord_the_pages_name_is_recognised() {
        let pages = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("help");
        let mut seen = 0;
        for locale in std::fs::read_dir(&pages).expect("help/").flatten() {
            for page in std::fs::read_dir(locale.path())
                .expect("a locale folder")
                .flatten()
            {
                let text = std::fs::read_to_string(page.path()).expect("a page");
                for span in text.split('`').skip(1).step_by(2) {
                    let looks_like_a_chord = ["Ctrl", "Alt", "Shift", "Cmd", "Strg", "Maj"]
                        .iter()
                        .any(|m| {
                            span.starts_with(&format!("{m}+")) || span.starts_with(&format!("{m}-"))
                        });
                    if looks_like_a_chord {
                        seen += 1;
                        assert!(
                            parse_chord(span).is_some(),
                            "{}: `{span}` is not a chord platform_chords can rewrite",
                            page.path().display()
                        );
                    }
                }
            }
        }
        assert!(
            seen > 0,
            "the pages name no chord at all: the walk is broken"
        );
    }
}
