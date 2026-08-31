// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The names Skribisto writes into a `.docx`/`.odt` so it can recognise its own work when the
//! file comes back from an editor — and the digest that says whether anything changed.
//!
//! One module, because the exporter and the importer must agree exactly. A writer that mints
//! `skrb_r…` and a reader that looks for `skrb-row-…` produce a round trip that silently never
//! matches anything, and nothing in either half would look wrong on its own.
//!
//! # Why a bookmark name
//!
//! Identity has to survive being opened and saved by Word or LibreOffice, and a private XML
//! attribute does not: measured against a real returning file, LibreOffice 25.8 deleted every
//! `skrb:uid` this app had written and the namespace declaration with them. A **bookmark** is
//! first-class in both formats and is preserved. So the name is the whole channel, and
//! everything below is about fitting an identity and a change-detector into one.
//!
//! # The two shapes
//!
//! ```text
//! skrb_r0123456789abcdef_0a1b2c3d4e5f     a row:     16 hex of uid + 12 hex of digest
//! skrb_c0123456789abcdef                  a comment: 16 hex of uid
//! ```
//!
//! 35 and 22 characters. The cap that matters is **Word's**: 40 characters, ASCII alphanumerics
//! and underscore, leading letter. ODF is far more permissive, but a name legal in only one of
//! the two would round-trip through one editor and lose identity in the other.
//!
//! A truncated uid, therefore, and not the full 36-character UUID — which would not fit beside
//! anything else, let alone a digest. 64 bits is far more than enough to tell one row of one
//! manuscript from another, and lookup is always *within* the project that minted them, never
//! across a global namespace.
//!
//! # The digest, and what it is for
//!
//! A row mark carries a digest of that row's prose **as it was exported**. On re-import that
//! gives a genuine three-way comparison with nothing stored on the project's side — the baseline
//! travels in the file:
//!
//! | local vs baseline | incoming vs baseline | what happened |
//! |---|---|---|
//! | same | same | nobody touched it |
//! | same | changed | the editor edited it |
//! | changed | same | you edited it |
//! | changed | changed | both — a conflict worth showing before choosing |
//!
//! Which is only as good as `normalize` is at cancelling what a round trip does to text that
//! nobody edited. That is an empirical question, not a design one, and it is settled by an
//! empirical test: `document_ingest`'s round-trip suite pushes a real document through a real
//! LibreOffice and asserts an untouched row still digests equal. Read that test before changing
//! anything here.

use uuid::Uuid;

/// Prefix of a row mark — one per exported `Content`, anchored where that row's prose begins.
pub const ROW_PREFIX: &str = "skrb_r";
/// Prefix of a comment mark — a range bracketing exactly the characters a comment covers.
pub const COMMENT_PREFIX: &str = "skrb_c";

/// Hex digits of uid carried in a mark name (64 bits).
pub const UID_HEX: usize = 16;
/// Hex digits of digest carried in a row mark name (48 bits).
pub const DIGEST_HEX: usize = 12;

/// A 64-bit tag for `uid`, as lowercase hex — the identity half of a mark name.
///
/// **A hash of all sixteen bytes, not a slice of them.** Truncating looks simpler and is a trap:
/// it makes identity depend on *which* bytes of a uuid happen to vary. UUIDv7 puts a timestamp in
/// the high bits; this repo's own `common::uid::fixture_uid` is `Uuid::from_u128(n)`, whose first
/// eight bytes are zero for every fixture ever made. Under a leading-bytes prefix every one of
/// those rows shares one name, and a returning file matches all of them to whichever was written
/// last. Hashing spreads all 128 bits over the 64 the name can carry, whatever the minting scheme.
///
/// 64 bits is far more than enough: the lookup is always *within* the project that minted them,
/// never across a global namespace, and a manuscript has thousands of rows rather than billions.
pub fn uid_tag(uid: &Uuid) -> String {
    format!("{:016x}", fnv1a64(uid.as_bytes()))
}

/// Whether `uid` is the one a mark's `tag` names.
pub fn uid_matches(uid: &Uuid, tag: &str) -> bool {
    uid_tag(uid) == tag
}

/// FNV-1a, 64-bit. Offset basis `0xcbf29ce484222325`, prime `0x100000001b3` — both written in
/// full 16-digit form, because the prime grouped as `0x1000_0000_01b3` is a *different*, wrong
/// constant that still produces plausible-looking hex.
///
/// Not a security boundary and not trying to be. Chosen for being dependency-free and stable
/// across Rust versions: a hash that changed with the toolchain would make every mark already
/// written into a file out in the world stop matching.
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Reduce prose to the form a digest is taken over.
///
/// Everything removed here is something a round trip through an editor is known to change
/// without anyone having edited the text:
///
/// * **Scene-break markers.** Measured, not anticipated: on a real 90 000-word manuscript
///   exported to `.odt` and read straight back, nine chapters of twenty-three digested
///   differently, and every one of them for this reason. A break is stored as `* * *` but
///   *rendered* by the export preset — and the default minor break in nearly every built-in
///   preset is `SceneBreak::BlankLine`, which renders to nothing at all. The marker is
///   therefore absent from the file, absent from what comes back, and present in the prose the
///   project holds. Comparing the two without dropping it reports "the editor rewrote this
///   chapter" on a chapter nobody touched.
///
///   The cost is that *moving* a scene break, and changing nothing else, does not register as
///   an edit. That is the same trade [`counting`](crate::counting) already makes for word
///   count, for the same stated reason: a break is furniture the writer placed, not words they
///   wrote.
///
/// * `U+FFFC` — the object-replacement character standing in for a table in this app's
///   *addressable* text. The exporter's plain text carries it; a scanner's does not. Comparing
///   the two without dropping it would report every row containing a table as edited.
/// * Zero-width and formatting characters (`U+200B` zero-width space, `U+FEFF`,
///   `U+00AD` soft hyphen) — inserted and removed freely by layout engines.
/// * Non-breaking spaces (`U+00A0`, `U+202F`) folded to a plain space. French typography inserts
///   a narrow no-break space before `;:!?` and a round trip does not reliably preserve *which*
///   no-break space it was; the distinction is real typography but it is not an edit.
/// * Runs of whitespace collapsed to one, and the ends trimmed — paragraph boundaries and
///   indentation are re-derived by every writer and reader in the chain.
///
/// What is deliberately **kept**: letter case, punctuation, and typographic characters
/// (curly quotes, dashes, ellipsis). Those are content. Folding them would make the digest blind
/// to a real edit — an editor changing `"` to `«` has changed the manuscript.
pub fn normalize(text: &str) -> String {
    // Break markers first, while lines are still lines — `strip_markers_plain` works on whole
    // lines, and the whitespace collapse below destroys the line structure it needs.
    let text = crate::scene_break::strip_markers_plain(text);
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for c in text.chars() {
        match c {
            '\u{FFFC}' | '\u{200B}' | '\u{FEFF}' | '\u{00AD}' => continue,
            '\u{00A0}' | '\u{202F}' => {
                pending_space = true;
            }
            c if c.is_whitespace() => {
                pending_space = true;
            }
            c => {
                if pending_space && !out.is_empty() {
                    out.push(' ');
                }
                pending_space = false;
                out.push(c);
            }
        }
    }
    out
}

/// The 48-bit digest of `text`, as 12 lowercase hex digits.
///
/// FNV-1a (64-bit), truncated. Not a security boundary and not trying to be: the question it
/// answers is "is this the same prose I exported", over text the user is looking at either way,
/// and a 48-bit space makes an accidental collision within one manuscript a non-event. Chosen
/// over a real hash function for being dependency-free and stable across Rust versions — a
/// digest that changed with the toolchain would make every previously-exported file report its
/// rows as edited.
pub fn digest(text: &str) -> String {
    let hash = fnv1a64(normalize(text).as_bytes());
    format!("{:012x}", hash & 0x0000_ffff_ffff_ffff)
}

/// The bookmark name for a row: its `BinderItem.uid` and a digest of the prose being exported.
pub fn row_mark_name(uid: &Uuid, text: &str) -> String {
    row_mark_name_with_digest(uid, &digest(text))
}

/// The same name, from a digest already in hand.
///
/// Exists so a caller that needs the digest *as well as* the name — the export
/// receipt keeps it, because a mark name is one-way and cannot be read back —
/// computes it once. Two computations of one digest are two chances for them to
/// disagree, and a receipt that disagrees with the file it describes is worse
/// than no receipt.
pub fn row_mark_name_with_digest(uid: &Uuid, digest: &str) -> String {
    format!("{ROW_PREFIX}{}_{}", uid_tag(uid), digest)
}

/// The bookmark name for a comment: its `Comment.uid`, and nothing else — a comment's *body*
/// living in the file's own comment machinery, where the editor can edit it and we read it back
/// wholesale. There is nothing a digest would add.
pub fn comment_mark_name(uid: &Uuid) -> String {
    format!("{COMMENT_PREFIX}{}", uid_tag(uid))
}

/// What a recognised mark name says.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkName {
    Row {
        /// 16 lowercase hex — match against a candidate with [`uid_matches`], never by string
        /// comparison against a uuid, since the tag is derived and not a substring of one.
        uid_tag: String,
        /// 12 lowercase hex, the row's prose as it was exported. Compare with [`digest`] of the
        /// current prose to learn who changed what.
        digest: String,
    },
    Comment {
        uid_tag: String,
    },
}

/// Read a bookmark name back, if it is one of ours.
///
/// `None` for every other bookmark in the file, of which a manuscript from a real editor may
/// have many — a cross-reference target, a LibreOffice `__Fieldmark__`, a table of contents
/// entry. Being strict about the shape is what keeps those from being mistaken for identity:
/// the length and the alphabet are both checked, not just the prefix.
pub fn parse_mark_name(name: &str) -> Option<MarkName> {
    fn is_hex(s: &str, len: usize) -> bool {
        s.len() == len
            && s.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
    }

    if let Some(rest) = name.strip_prefix(ROW_PREFIX) {
        let (uid_tag, digest) = rest.split_once('_')?;
        if !is_hex(uid_tag, UID_HEX) || !is_hex(digest, DIGEST_HEX) {
            return None;
        }
        return Some(MarkName::Row {
            uid_tag: uid_tag.to_string(),
            digest: digest.to_string(),
        });
    }
    if let Some(rest) = name.strip_prefix(COMMENT_PREFIX) {
        if !is_hex(rest, UID_HEX) {
            return None;
        }
        return Some(MarkName::Comment {
            uid_tag: rest.to_string(),
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uid(n: u128) -> Uuid {
        Uuid::from_u128(n)
    }

    #[test]
    fn a_row_name_round_trips_through_its_own_parser() {
        let u = uid(0x0123_4567_89ab_cdef_0000_0000_0000_0001);
        let name = row_mark_name(&u, "She turned the corner.");
        match parse_mark_name(&name).expect("our own name parses") {
            MarkName::Row { uid_tag, digest } => {
                assert!(uid_matches(&u, &uid_tag));
                assert_eq!(digest, super::digest("She turned the corner."));
            }
            other => panic!("parsed as {other:?}"),
        }
    }

    #[test]
    fn a_comment_name_round_trips_through_its_own_parser() {
        let u = uid(0xfedc_ba98_7654_3210_0000_0000_0000_0002);
        let name = comment_mark_name(&u);
        match parse_mark_name(&name).expect("our own name parses") {
            MarkName::Comment { uid_tag } => assert!(uid_matches(&u, &uid_tag)),
            other => panic!("parsed as {other:?}"),
        }
    }

    /// Uids that differ only in their **low** bytes must still get different tags.
    ///
    /// This is not hypothetical, and it is why the tag is a hash rather than a truncation. This
    /// repo's own fixture uids are `Uuid::from_u128(n)`, whose leading eight bytes are zero for
    /// every value of `n`; UUIDv7 has the mirror problem, putting a timestamp in the high bits.
    /// Under a leading-bytes prefix, every fixture row in the project shared one mark name, and a
    /// returning file matched all of them to whichever had been written last.
    #[test]
    fn uids_differing_only_in_their_low_bytes_get_different_tags() {
        let a = uid(101);
        let b = uid(102);
        assert_ne!(uid_tag(&a), uid_tag(&b));
        assert!(uid_matches(&a, &uid_tag(&a)));
        assert!(!uid_matches(&a, &uid_tag(&b)));
    }

    /// And the mirror case: uids differing only in their **high** bytes.
    #[test]
    fn uids_differing_only_in_their_high_bytes_get_different_tags() {
        let a = uid(0x0000_0000_0000_0001_0000_0000_0000_0000);
        let b = uid(0x0000_0000_0000_0002_0000_0000_0000_0000);
        assert_ne!(uid_tag(&a), uid_tag(&b));
    }

    #[test]
    fn a_tag_is_sixteen_lowercase_hex_digits() {
        let t = uid_tag(&uid(1));
        assert_eq!(t.len(), UID_HEX);
        assert!(
            t.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
        );
    }

    /// Word's cap is the binding one, and both shapes have to clear it with room to spare —
    /// there is no diagnostic when they do not, just a name Word quietly mangles.
    #[test]
    fn both_names_fit_words_bookmark_rules() {
        let names = [
            row_mark_name(&uid(u128::MAX), "any prose at all"),
            comment_mark_name(&uid(u128::MAX)),
        ];
        for name in names {
            assert!(name.len() <= 40, "{name} is {} characters", name.len());
            assert!(name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'));
            assert!(name.starts_with(|c: char| c.is_ascii_alphabetic()));
        }
    }

    /// The names a real returning file is full of, none of which are ours.
    #[test]
    fn a_foreign_bookmark_is_not_mistaken_for_identity() {
        for foreign in [
            "__Fieldmark__0",
            "_Toc123456789",
            "Chapter_One",
            "skrb_rNOTHEX0000000_0a1b2c3d4e5f",
            "skrb_r0123456789abcdef",            // no digest
            "skrb_r0123456789abcdef_0a1b2c3d4e", // digest too short
            "skrb_c0123456789abcde",             // uid too short
            "skrb_c0123456789ABCDEF",            // upper-case hex is not what we mint
            "skrb_x0123456789abcdef",
        ] {
            assert_eq!(parse_mark_name(foreign), None, "{foreign} was accepted");
        }
    }

    // --- normalization -------------------------------------------------------------------

    #[test]
    fn whitespace_shape_does_not_change_the_digest() {
        let a = "She turned the corner.\n\nThe street was gone.";
        let b = "  She turned the corner.   The street was gone.  ";
        assert_eq!(digest(a), digest(b));
    }

    /// The table anchor is in this app's addressable text and not in a scanner's plain text.
    /// Without dropping it, every row containing a table reports as edited on every re-import.
    #[test]
    fn a_table_anchor_does_not_change_the_digest() {
        assert_eq!(
            digest("Before\u{FFFC}after"),
            digest("Beforeafter"),
            "the object-replacement character must not reach the digest"
        );
    }

    /// The one that was measured rather than guessed, on a real manuscript.
    ///
    /// A scene break is stored as `* * *` and rendered by the export preset — to a blank line
    /// under nearly every built-in, which means it is simply not in the exported file. Prose
    /// that came back therefore has no marker where the project's copy has one, and without
    /// this the row reads as edited on every single round trip.
    #[test]
    fn a_scene_break_marker_does_not_change_the_digest() {
        let with = "She turned the corner.\n\n* * *\n\nLater, she would say she had known.";
        let without = "She turned the corner.\n\nLater, she would say she had known.";
        assert_eq!(digest(with), digest(without));

        // The major mark too, and the alternative spellings the model accepts.
        for marker in ["# # #", "***", "###", "⁂"] {
            let text = format!("One.\n\n{marker}\n\nTwo.");
            assert_eq!(
                digest(&text),
                digest("One.\n\nTwo."),
                "{marker} reached the digest"
            );
        }
    }

    /// …and an asterisk that is *part of a sentence* is not a break and must still count.
    #[test]
    fn an_asterisk_inside_prose_still_changes_the_digest() {
        assert_ne!(
            digest("She turned the * corner."),
            digest("She turned the corner.")
        );
    }

    #[test]
    fn no_break_spaces_fold_to_ordinary_ones() {
        assert_eq!(
            digest("Vraiment\u{202F}? Oui\u{00A0}!"),
            digest("Vraiment ? Oui !")
        );
    }

    #[test]
    fn zero_width_characters_are_ignored() {
        assert_eq!(digest("wor\u{200B}d"), digest("word"));
        assert_eq!(digest("\u{FEFF}word"), digest("word"));
        assert_eq!(digest("soft\u{00AD}hyphen"), digest("softhyphen"));
    }

    /// The other half of the contract: the digest has to still *see* a real edit, or every row
    /// reports as untouched and the whole three-way comparison says nothing.
    #[test]
    fn a_real_edit_changes_the_digest() {
        let base = "She turned the corner.";
        for edited in [
            "She turned the corners.",  // a letter
            "She turned the corner!",   // punctuation
            "she turned the corner.",   // case
            "She turned the “corner”.", // typographic quotes are content
            "She turned the corner",    // a dropped full stop
        ] {
            assert_ne!(digest(base), digest(edited), "{edited:?} digested equal");
        }
    }

    #[test]
    fn the_digest_is_twelve_lowercase_hex_digits() {
        let d = digest("anything at all");
        assert_eq!(d.len(), DIGEST_HEX);
        assert!(
            d.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase())
        );
    }

    /// Pinned by value, not by construction. The digest is written into files that outlive this
    /// build; changing the hash or the normalization silently invalidates every mark already in
    /// the wild, and this is the test that says so out loud.
    #[test]
    fn the_digest_is_stable_across_builds() {
        assert_eq!(digest("She turned the corner."), "3a84291b094f");
        // The offset basis itself, masked — the empty string runs no rounds at all, so this
        // also pins that the basis was not mistyped.
        assert_eq!(digest(""), "9ce484222325");
    }
}
