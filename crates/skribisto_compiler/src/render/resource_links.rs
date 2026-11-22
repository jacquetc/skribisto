// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A link to a resource prints only its words.
//!
//! A writer links a passage to a file or a page they keep beside the book (a
//! `resource:<uid>` link, made by dragging a resource from the Resources dock into the
//! text). A reader of the exported book has no project to open it in, so the link has
//! nowhere to go: in every format, the export keeps the link's text and drops the link.
//!
//! Done once, on the compiled Djot before it is parsed ([`strip_resource_links`]), which
//! is the one place every format's text passes through: the body, the footnote bodies
//! and the title page alike. Removing only the markup leaves the text exactly as long as
//! it was, so nothing that maps positions in the text (the comment rebase) moves.
//!
//! A scan rather than a parse, and a careful one: it skips verbatim and code spans (a
//! backtick run up to the next run of the same length), honours backslash escapes, and
//! matches a link's brackets by nesting, so a `](resource:` written as literal text in
//! code is left alone. A web link is never touched.

use std::borrow::Cow;

/// The scheme of a link to a resource, as `teksilo_ui::resources::links` writes it.
const SCHEME: &str = "resource:";

/// `djot` with every `[words](resource:…)` link replaced by its `words`.
pub(crate) fn strip_resource_links(djot: &str) -> Cow<'_, str> {
    if !djot.contains("](resource:") && !djot.contains("](RESOURCE:") {
        return Cow::Borrowed(djot);
    }
    Cow::Owned(strip(djot))
}

fn strip(djot: &str) -> String {
    let bytes = djot.as_bytes();
    let mut out = String::with_capacity(djot.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' => {
                // An escape and the character it escapes go through together.
                let end = next_char_end(djot, i + 1);
                out.push_str(&djot[i..end]);
                i = end;
            }
            b'`' => {
                let end = verbatim_end(bytes, i);
                out.push_str(&djot[i..end]);
                i = end;
            }
            b'[' if !preceded_by_bang(bytes, i) => match resource_link_at(djot, i) {
                Some((text_start, text_end, link_end)) => {
                    // The words may hold links of their own: strip those too.
                    out.push_str(&strip(&djot[text_start..text_end]));
                    i = link_end;
                }
                None => {
                    out.push('[');
                    i += 1;
                }
            },
            _ => {
                let end = next_char_end(djot, i);
                out.push_str(&djot[i..end]);
                i = end;
            }
        }
    }
    out
}

/// The byte after the character starting at `i` (or `i` itself at the end).
fn next_char_end(s: &str, i: usize) -> usize {
    if i >= s.len() {
        return s.len();
    }
    let mut end = i + 1;
    while end < s.len() && !s.is_char_boundary(end) {
        end += 1;
    }
    end
}

fn preceded_by_bang(bytes: &[u8], i: usize) -> bool {
    i > 0 && bytes[i - 1] == b'!'
}

/// Where a verbatim span opened by the backtick run at `start` ends: after the next run
/// of exactly as many backticks, or at the end of the text when there is none.
fn verbatim_end(bytes: &[u8], start: usize) -> usize {
    let run = bytes[start..].iter().take_while(|b| **b == b'`').count();
    let mut i = start + run;
    while i < bytes.len() {
        if bytes[i] == b'`' {
            let n = bytes[i..].iter().take_while(|b| **b == b'`').count();
            if n == run {
                return i + n;
            }
            i += n;
        } else {
            i += 1;
        }
    }
    bytes.len()
}

/// If a link to a resource opens at `open` (a `[`), the span of its words and the byte
/// after the whole link, attributes included.
fn resource_link_at(djot: &str, open: usize) -> Option<(usize, usize, usize)> {
    let bytes = djot.as_bytes();
    let mut depth = 0usize;
    let mut i = open;
    let close = loop {
        if i >= bytes.len() {
            return None;
        }
        match bytes[i] {
            b'\\' => i += 2,
            b'`' => i = verbatim_end(bytes, i),
            b'[' => {
                depth += 1;
                i += 1;
            }
            b']' => {
                depth -= 1;
                if depth == 0 {
                    break i;
                }
                i += 1;
            }
            _ => i += 1,
        }
    };
    let dest = close + 1;
    if bytes.get(dest) != Some(&b'(') {
        return None;
    }
    let after_paren = &djot[dest + 1..];
    if !after_paren
        .get(..SCHEME.len())
        .is_some_and(|s| s.eq_ignore_ascii_case(SCHEME))
    {
        return None;
    }
    let mut j = dest + 1;
    while j < bytes.len() && bytes[j] != b')' {
        if bytes[j] == b'\n' {
            return None;
        }
        j += if bytes[j] == b'\\' { 2 } else { 1 };
    }
    if j >= bytes.len() {
        return None;
    }
    let mut end = j + 1;
    // Attributes on the link (`{.class}`) go with it.
    if bytes.get(end) == Some(&b'{')
        && let Some(close_brace) = djot[end..].find('}')
    {
        end += close_brace + 1;
    }
    Some((open + 1, close, end))
}

#[cfg(test)]
mod tests {
    use super::strip_resource_links;

    const UID: &str = "12345678-9abc-def0-1234-56789abcdef0";

    #[test]
    fn a_resource_link_keeps_only_its_words() {
        let djot = format!("See the [village map](resource:{UID}) before the storm.");
        assert_eq!(
            strip_resource_links(&djot),
            "See the village map before the storm."
        );
    }

    #[test]
    fn a_web_link_and_a_picture_are_left_alone() {
        for djot in [
            "A [harbour page](https://example.org/harbour).",
            "![gull](assets/abc.png){width=10 height=10}",
            "No link at all.",
        ] {
            assert_eq!(strip_resource_links(djot), djot);
        }
    }

    #[test]
    fn emphasis_inside_the_words_survives_and_attributes_go() {
        let djot = format!("[the _old_ map](resource:{UID}){{.ref}} ends.");
        assert_eq!(strip_resource_links(&djot), "the _old_ map ends.");
    }

    #[test]
    fn brackets_in_the_words_are_matched_by_nesting() {
        let djot = format!("[map [1887]](resource:{UID})");
        assert_eq!(strip_resource_links(&djot), "map [1887]");
    }

    #[test]
    fn code_and_escapes_are_not_links() {
        let code = format!("`[x](resource:{UID})` and \\[y](resource:{UID})");
        assert_eq!(strip_resource_links(&code), code);
    }

    #[test]
    fn several_links_and_accented_words() {
        let djot =
            format!("« [Carte du village](resource:{UID}) » et [la marée](RESOURCE:{UID}) ; fin.");
        assert_eq!(
            strip_resource_links(&djot),
            "« Carte du village » et la marée ; fin."
        );
    }
}
