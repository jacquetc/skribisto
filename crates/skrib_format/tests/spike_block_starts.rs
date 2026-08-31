// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `djot_plain_text` must return block starts that index correctly into the text it
//! returns *alongside* them.
//!
//! This began as a spike, because the pairing was in doubt — and the doubt was
//! well-founded. text-document keeps two plain-text renderings: `to_plain_text()` (the
//! human-readable export, no `U+FFFC` table anchors) and the addressable text (what
//! search, cursors and `blocks().position()` index, anchors counted). `djot_plain_text`
//! used to pair the *export* with the *addressable* starts, so every block start after a
//! table was two characters too high, and comment anchoring — built on exactly this
//! pair — silently drifted inside any row containing a table.
//!
//! Fixed by pairing the starts with `to_addressable_text()` (added to text-document for
//! this purpose): one string, one offset space, anchors and all. These tests are the
//! regression pin.

use skrib_format::djot_plain_text;

/// The contract the comment anchoring rests on, stated as a test: every reported start
/// must be a real index into the returned text, and the text at that index must be the
/// beginning of a block.
fn assert_starts_index_the_returned_text(djot: &str, label: &str) {
    let (text, starts) = djot_plain_text(djot).expect("convert");
    let chars: Vec<char> = text.chars().collect();

    for (i, &start) in starts.iter().enumerate() {
        assert!(
            start <= chars.len(),
            "{label}: block {i} starts at {start}, past the end of a {} char text\n\
             text = {text:?}\nstarts = {starts:?}",
            chars.len()
        );
        // A block start is either position 0 or immediately after a '\n' separator.
        if start > 0 && start < chars.len() {
            assert_eq!(
                chars[start - 1],
                '\n',
                "{label}: block {i} claims to start at {start}, but the character before it \
                 is {:?}, not a block separator\ntext = {text:?}\nstarts = {starts:?}",
                chars[start - 1]
            );
        }
    }
}

#[test]
fn plain_prose_block_starts_are_true() {
    assert_starts_index_the_returned_text(
        "First paragraph.\n\nSecond paragraph.\n\nThird.",
        "plain prose",
    );
}

/// The case that used to fail: the table's `U+FFFC` anchor occupies two characters
/// (sentinel + separator) of the addressable space, and the starts count them — so the
/// returned text must hold them too.
#[test]
fn block_starts_stay_true_across_a_table() {
    assert_starts_index_the_returned_text(
        "intro\n\n| a | b |\n| - | - |\n| c | d |\n\nafter",
        "prose with a table",
    );
}

#[test]
fn block_starts_stay_true_across_a_blockquote() {
    assert_starts_index_the_returned_text("> quoted\n\nafter", "prose with a blockquote");
}

/// The one that matters most for export: prose, a table, then more prose, with the
/// anchoring question asked the way a comment asks it — "is the text at the offset I
/// was given the text I expected?" Before the fix the tail came back `"e salt-bleached
/// door"`'s neighbourhood shifted by two; now it must land exactly.
#[test]
fn a_quote_after_a_table_resolves_where_its_block_says_it_does() {
    let djot = "intro\n\n| a | b |\n| - | - |\n| c | d |\n\nthe salt-bleached door";
    let (text, starts) = djot_plain_text(djot).expect("convert");
    let chars: Vec<char> = text.chars().collect();

    let last = *starts.last().expect("at least one block");
    let tail: String = chars[last..].iter().collect();

    assert_eq!(
        tail, "the salt-bleached door",
        "the last block's reported start must land exactly on the last block's text\n\
         text = {text:?}\nstarts = {starts:?}"
    );
}
