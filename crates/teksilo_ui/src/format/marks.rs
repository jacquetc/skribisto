// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The invisible typographic marks a writer places by hand: a space that will
//! not break, a hyphen that will not, a hint about where a word may.
//!
//! **One table, three surfaces.** The Format menu's submenu, the global
//! shortcuts, and the actions those shortcuts invoke all read [`all`], so a mark
//! cannot reach one surface and not the others, and no two of them can disagree
//! about which character a row writes. That last point is the reason this is a
//! table at all rather than six hand-written rows: every one of these characters
//! is invisible on screen and most are invisible in a diff, so a transposed
//! escape would be wrong in exactly the way nobody notices.
//!
//! **Why a writer needs them.** French typography puts a narrow no-break space
//! inside guillemets and before `? ! ;`, and the project's own
//! [`SmartPunctuation`](crate::singles::SingleSmartPunctuation) places those
//! while typing. This is the manual door, for the places no rule can see: a
//! non-breaking space between a number and its unit, or between a character's
//! title and their name, so a line never breaks between the two.
//!
//! Nothing here is a Skribisto convention. Each mark is ordinary Unicode in the
//! prose, so it travels through the bundle's Djot and out through every exporter
//! as itself.

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;

/// The chord that reaches one mark, and the id that ties its shortcut to its
/// action and to the menu row that renders the chord.
pub struct MarkShortcut {
    /// Shared by `Shortcut::new`, `Action::new` and `MenuEntry::shortcut`.
    pub id: &'static str,
    /// What the shortcut is called wherever shortcuts are listed.
    pub name: LocalizedString,
    pub primary: KeyStroke,
    /// The same chord as the layout that needs a `Shift` to produce the
    /// character actually delivers it. See [`all`]'s note on `Ctrl+/`.
    pub secondary: Option<KeyStroke>,
}

/// One insertable mark: the character it writes, the row that offers it, and
/// the chord that reaches it, if it has one.
pub struct FormattingMark {
    /// Written at the caret, verbatim.
    pub text: &'static str,
    /// The menu row, mnemonic and all.
    pub label: LocalizedString,
    /// `None` for a mark the menu alone offers.
    pub shortcut: Option<MarkShortcut>,
}

fn chord(id: &'static str, name: LocalizedString, primary: KeyStroke) -> Option<MarkShortcut> {
    Some(MarkShortcut {
        id,
        name,
        primary,
        secondary: None,
    })
}

/// A chord plus the shifted form of itself, for a character a common layout
/// only produces with `Shift` held.
fn chord_or_shifted(
    id: &'static str,
    name: LocalizedString,
    primary: KeyStroke,
    shifted: KeyStroke,
) -> Option<MarkShortcut> {
    Some(MarkShortcut {
        id,
        name,
        primary,
        secondary: Some(shifted),
    })
}

/// The marks, in the order they appear in the menu.
///
/// The order is LibreOffice Writer's, which is where a writer coming to this
/// will have met these rows before. So are four of the five chords.
///
/// **The soft hyphen deliberately has none.** Writer gives it `Ctrl+-`, which
/// here already shrinks the editor's text (`editor.size.decrease`), and a global
/// shortcut resolves before the focused widget sees the key: taking it would
/// silently remove a binding writers already use, to serve the rarest of these
/// six marks in a novel. `Ctrl+Shift+Space`, `Ctrl+Shift+-`, `Alt+Shift+Space`
/// and `Ctrl+/` are all free, so the other four keep the chords Writer taught.
///
/// **`Ctrl+/` is declared twice, and has to be.** A keystroke arrives here as the
/// *logical* key the layout produced, matched against the declaration by exact
/// equality including modifiers. On a French AZERTY keyboard `/` is `Shift`+`:`,
/// so the chord reaches the app as `Ctrl+Shift+/` and a single `Ctrl+/`
/// declaration would never fire on the layout most of this application's writers
/// use. Writer does not have this problem because it resolves by physical key.
/// The other three are safe as declared: `Space` is a named key, and `-` is
/// unshifted on both AZERTY and QWERTY.
pub fn all() -> Vec<FormattingMark> {
    vec![
        FormattingMark {
            // U+00A0. The one most writers reach for.
            text: "\u{00A0}",
            label: tr!(menu_format_typo_nbsp()),
            shortcut: chord(
                "format.mark.nbsp",
                tr!(shortcut_name_format_mark_nbsp()),
                KeyStroke::new(Key::Space, Modifiers::CTRL | Modifiers::SHIFT),
            ),
        },
        FormattingMark {
            // U+2011. A hyphen that holds the two halves on one line, unlike
            // the ordinary `-`, which is a break opportunity.
            text: "\u{2011}",
            label: tr!(menu_format_typo_nbhyphen()),
            shortcut: chord(
                "format.mark.nbhyphen",
                tr!(shortcut_name_format_mark_nbhyphen()),
                // `Key` has no punctuation variants; `Character` is how the
                // editor-size trio declares its own `-` and `=` too.
                KeyStroke::new(Key::Character('-'), Modifiers::CTRL | Modifiers::SHIFT),
            ),
        },
        FormattingMark {
            // U+00AD. Invisible until the line actually needs to break there,
            // and then it prints as a hyphen.
            text: "\u{00AD}",
            label: tr!(menu_format_typo_soft_hyphen()),
            shortcut: None,
        },
        FormattingMark {
            // U+202F. The French thin space, and the one the app's own
            // punctuation engine writes inside « » and before ? ! ;.
            text: "\u{202F}",
            label: tr!(menu_format_typo_nnbsp()),
            shortcut: chord(
                "format.mark.nnbsp",
                tr!(shortcut_name_format_mark_nnbsp()),
                KeyStroke::new(Key::Space, Modifiers::ALT | Modifiers::SHIFT),
            ),
        },
        FormattingMark {
            // U+200B. Says "a break may go here" without printing anything,
            // which is how a long URL or a compound stops overflowing.
            text: "\u{200B}",
            label: tr!(menu_format_typo_zwsp()),
            shortcut: chord_or_shifted(
                "format.mark.zwsp",
                tr!(shortcut_name_format_mark_zwsp()),
                KeyStroke::new(Key::Character('/'), Modifiers::CTRL),
                KeyStroke::new(Key::Character('/'), Modifiers::CTRL | Modifiers::SHIFT),
            ),
        },
        FormattingMark {
            // U+2060. The inverse of the one above: says "no break here",
            // without the space a no-break space would add.
            text: "\u{2060}",
            label: tr!(menu_format_typo_word_joiner()),
            shortcut: None,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Every character, spelled out independently of the table. An escape is
    /// unreadable and each of these renders as nothing, so the only way a
    /// transposition gets caught is by writing the codepoints down twice.
    #[test]
    fn each_row_writes_the_character_its_name_claims() {
        let marks = all();
        let got: Vec<&str> = marks.iter().map(|m| m.text).collect();
        assert_eq!(
            got,
            vec![
                "\u{00A0}", // no-break space
                "\u{2011}", // non-breaking hyphen
                "\u{00AD}", // soft hyphen
                "\u{202F}", // narrow no-break space
                "\u{200B}", // zero-width space
                "\u{2060}", // word joiner
            ],
            "in LibreOffice Writer's order, which is the order of the menu"
        );
    }

    /// Each is exactly one character. A mark that arrived as two (a stray
    /// combining character, a copied-in pair) would insert silently and be
    /// impossible to see in the prose afterwards.
    #[test]
    fn every_mark_is_a_single_character() {
        for m in all() {
            assert_eq!(
                m.text.chars().count(),
                1,
                "{:?} is not one character",
                m.text.escape_unicode().to_string()
            );
        }
    }

    /// The id ties the shortcut, the action and the menu row's rendered chord
    /// together. Two marks sharing one would give the second row the first's
    /// character, from the keyboard and in the label alike.
    #[test]
    fn no_two_marks_share_a_shortcut_id() {
        let ids: Vec<&str> = all()
            .iter()
            .filter_map(|m| m.shortcut.as_ref())
            .map(|s| s.id)
            .collect();
        let unique: HashSet<&str> = ids.iter().copied().collect();
        assert_eq!(ids.len(), unique.len(), "duplicate shortcut id in {ids:?}");
    }

    /// **AZERTY.** A keystroke is matched by exact equality, modifiers included,
    /// against the *logical* key the layout produced. `/` is `Shift`+`:` on a
    /// French keyboard, so `Ctrl+/` arrives as `Ctrl+Shift+/` there: without the
    /// second declaration the zero-width space would be unreachable from the
    /// keyboard for most of this application's writers, while looking perfectly
    /// correct in the menu.
    #[test]
    fn the_slash_chord_is_reachable_from_a_shifted_layout() {
        let marks = all();
        let zwsp = marks
            .iter()
            .find(|m| m.text == "\u{200B}")
            .expect("the zero-width space is in the table");
        let sc = zwsp.shortcut.as_ref().expect("it carries a chord");
        assert_eq!(
            sc.primary,
            KeyStroke::new(Key::Character('/'), Modifiers::CTRL)
        );
        assert_eq!(
            sc.secondary,
            Some(KeyStroke::new(
                Key::Character('/'),
                Modifiers::CTRL | Modifiers::SHIFT
            )),
            "AZERTY delivers this chord with Shift held"
        );
    }

    /// The other three need no such twin, and saying so keeps the next reader
    /// from adding one out of symmetry: `Space` is a named key that no layout
    /// shifts, and `-` is unshifted on AZERTY and QWERTY alike.
    #[test]
    fn only_the_slash_chord_needs_a_shifted_twin() {
        for m in all() {
            let Some(sc) = &m.shortcut else { continue };
            if sc.id == "format.mark.zwsp" {
                continue;
            }
            assert!(
                sc.secondary.is_none(),
                "{} declares a shifted twin it does not need",
                sc.id
            );
        }
    }

    /// The soft hyphen and the word joiner are menu-only on purpose, and the
    /// other four carry a chord. Stated as a test so that adding one later is a
    /// decision somebody makes rather than a line somebody copies.
    #[test]
    fn four_of_the_six_carry_a_chord() {
        let with = all().iter().filter(|m| m.shortcut.is_some()).count();
        assert_eq!(with, 4);
        let marks = all();
        assert!(
            marks[2].shortcut.is_none() && marks[5].shortcut.is_none(),
            "the soft hyphen and the word joiner are the two without one"
        );
    }
}
