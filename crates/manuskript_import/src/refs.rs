// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Manuskript's inline cross-references, and turning them back into readable
//! prose.
//!
//! A reference is `{C:12:Peter}` — a letter naming what kind of thing it points
//! at, the target's numeric id, and a **display cache** Manuskript refreshes when
//! the target is renamed. They appear inline in a scene's text and in its notes,
//! and Manuskript's own matcher is `{(\w):(\d+):?.*?}`, which binds on the letter
//! and the id and throws the rest away.
//!
//! | Letter | Points at |
//! |---|---|
//! | `C` | a character |
//! | `T` | another outline row |
//! | `P` | a plot |
//! | `W` | a world entry |
//!
//! Skribisto expresses the same idea two ways, and both are better than a literal
//! marker in the prose: a row's `references` list holds the link, and the mention
//! index finds a story-bible entry's name in the text on its own. So the marker is
//! **replaced by its display text** — the words the writer meant to read — and the
//! link is recorded as a reference. A marker whose display text is missing falls
//! back to the target's own name, and one that points at nothing resolvable is
//! left exactly as written rather than silently deleted.

/// One reference found in a piece of prose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// `C`, `T`, `P` or `W`.
    pub kind: char,
    /// The target's Manuskript id.
    pub id: String,
    /// The display cache, empty when the marker carried none.
    pub display: String,
}

/// What a scan produced: the prose with its markers replaced, and what they named.
#[derive(Debug, Clone, Default)]
pub struct Scanned {
    pub text: String,
    pub references: Vec<Reference>,
}

/// Find every reference in `text` and replace each with readable words.
///
/// `resolve` is given a reference and returns the name to show, or `None` when the
/// target does not exist — a plot deleted after a scene mentioned it, say. An
/// unresolvable marker keeps its original form, because a writer who sees
/// `{P:4:The rift}` in their prose can act on it, and one who sees nothing cannot.
///
/// `text` is plain text. For prose that is already Djot, use [`rewrite_djot`].
pub fn rewrite(text: &str, resolve: impl Fn(&Reference) -> Option<String>) -> Scanned {
    let mut out = String::with_capacity(text.len());
    let mut references: Vec<Reference> = Vec::new();
    let mut copied_to = 0;
    for (span, reference) in markers(text) {
        // Nothing to show and nothing to link: the marker stays so the writer can
        // see what was there.
        let Some(shown) = shown_for(&reference, &resolve) else {
            continue;
        };
        out.push_str(&text[copied_to..span.start]);
        out.push_str(&shown);
        copied_to = span.end;
        record(&mut references, reference);
    }
    out.push_str(&text[copied_to..]);

    Scanned {
        text: out,
        references,
    }
}

/// [`rewrite`] for prose that is already Djot, which a row's body is by the time the
/// mapper sees it.
///
/// The markers are found in the prose as the editor will show it, and each is replaced
/// through the document, so the name is escaped against the text around it and keeps the
/// marker's formatting. Scanning the Djot string itself cannot work: a typed `{C:0:Peter}`
/// is stored as `\{C\:0\:Peter\}`, and replacing from the `{` kept the backslash in front
/// of it, so the writer read `\Peter`.
///
/// Returns `djot` unchanged when it holds no marker to replace.
pub fn rewrite_djot(
    djot: &str,
    resolve: impl Fn(&Reference) -> Option<String>,
) -> anyhow::Result<Scanned> {
    let mut references: Vec<Reference> = Vec::new();
    let text = skrib_format::rewrite_djot_text(djot, |prose| {
        let mut edits = Vec::new();
        // Markers come back as byte ranges; the document counts characters.
        let (mut counted_to, mut chars_before) = (0, 0);
        for (span, reference) in markers(prose) {
            let Some(shown) = shown_for(&reference, &resolve) else {
                continue;
            };
            chars_before += prose[counted_to..span.start].chars().count();
            let len = prose[span.clone()].chars().count();
            edits.push(skrib_format::TextEdit {
                start: chars_before,
                len,
                replacement: shown,
            });
            chars_before += len;
            counted_to = span.end;
            record(&mut references, reference);
        }
        edits
    })?;
    Ok(Scanned { text, references })
}

/// Every marker in `text`, with the byte range it occupies, in order. A `{` that opens
/// no marker is skipped and the scan carries on from the next character, so prose that
/// legitimately contains a brace is not mangled.
fn markers(text: &str) -> Vec<(std::ops::Range<usize>, Reference)> {
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(offset) = text[from..].find('{') {
        let open = from + offset;
        match parse_marker(&text[open..]) {
            Some((reference, len)) => {
                found.push((open..open + len, reference));
                from = open + len;
            }
            None => from = open + 1,
        }
    }
    found
}

/// The words a marker is replaced by: the target's name, else the marker's own display
/// text. `None` when both are empty.
fn shown_for(
    reference: &Reference,
    resolve: &impl Fn(&Reference) -> Option<String>,
) -> Option<String> {
    let shown = resolve(reference).unwrap_or_else(|| reference.display.clone());
    (!shown.trim().is_empty()).then_some(shown)
}

/// Record `reference` once, however many times the prose names it.
fn record(references: &mut Vec<Reference>, reference: Reference) {
    if !references.contains(&reference) {
        references.push(reference);
    }
}

/// Parse a marker at the start of `s`, returning it and the bytes it occupies.
///
/// Mirrors `{(\w):(\d+):?.*?}`: one word character, a colon, at least one digit,
/// then optionally a colon and a display string, then the first `}`.
fn parse_marker(s: &str) -> Option<(Reference, usize)> {
    let bytes = s.as_bytes();
    if bytes.first() != Some(&b'{') {
        return None;
    }
    let close = s.find('}')?;
    let inner = &s[1..close];
    // Manuskript's `.*?` does not cross a line, and neither does a marker here: in
    // converted prose a line break is a paragraph boundary.
    if inner.contains('\n') {
        return None;
    }
    let mut parts = inner.splitn(3, ':');
    let kind_part = parts.next()?;
    let mut kind_chars = kind_part.chars();
    let kind = kind_chars.next()?;
    // `\w` is one character, so anything longer is not a marker.
    if kind_chars.next().is_some() || !(kind.is_alphanumeric() || kind == '_') {
        return None;
    }
    let id = parts.next()?;
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let display = parts.next().unwrap_or_default();
    Some((
        Reference {
            kind,
            id: id.to_string(),
            display: display.to_string(),
        },
        close + 1,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(name: &str) -> impl Fn(&Reference) -> Option<String> + '_ {
        move |_| Some(name.to_string())
    }

    fn unresolved(_: &Reference) -> Option<String> {
        None
    }

    #[test]
    fn a_marker_becomes_the_name_it_pointed_at_and_is_recorded() {
        let out = rewrite("Then {C:0:Peter} spoke.", named("Peter"));
        assert_eq!(out.text, "Then Peter spoke.");
        assert_eq!(out.references.len(), 1);
        assert_eq!(out.references[0].kind, 'C');
        assert_eq!(out.references[0].id, "0");
    }

    /// The third field is a cache Manuskript refreshes; the live name wins.
    #[test]
    fn a_stale_display_cache_is_replaced_by_the_current_name() {
        let out = rewrite("{C:0:Simon} spoke.", named("Peter"));
        assert_eq!(out.text, "Peter spoke.");
    }

    #[test]
    fn an_unresolvable_marker_falls_back_to_its_own_display_text() {
        let out = rewrite("About {P:0:The good news}.", unresolved);
        assert_eq!(out.text, "About The good news.");
        assert_eq!(out.references[0].kind, 'P');
    }

    /// Nothing to show and nothing to link: keep the marker rather than delete
    /// words the writer cannot then go looking for.
    #[test]
    fn a_marker_with_nothing_to_show_is_left_as_written() {
        let out = rewrite("See {W:7:}.", unresolved);
        assert_eq!(out.text, "See {W:7:}.");
        assert!(out.references.is_empty());
    }

    #[test]
    fn all_four_kinds_are_recognised() {
        let out = rewrite("{C:1:a} {T:2:b} {P:3:c} {W:4:d}", |r| {
            Some(r.display.clone())
        });
        assert_eq!(out.text, "a b c d");
        assert_eq!(
            out.references.iter().map(|r| r.kind).collect::<Vec<_>>(),
            ['C', 'T', 'P', 'W']
        );
    }

    #[test]
    fn the_same_target_twice_is_one_reference() {
        let out = rewrite("{C:0:Peter} and {C:0:Peter}", named("Peter"));
        assert_eq!(out.text, "Peter and Peter");
        assert_eq!(out.references.len(), 1);
    }

    #[test]
    fn a_marker_with_no_display_field_still_resolves() {
        let out = rewrite("{C:5}", named("Alice"));
        assert_eq!(out.text, "Alice");
        assert_eq!(out.references[0].id, "5");
    }

    /// Prose that happens to contain braces must survive untouched.
    #[test]
    fn braces_that_are_not_markers_are_left_alone() {
        for text in [
            "A {brace} in the prose.",
            "An {unclosed brace",
            "{C:notanumber:x}",
            "{CC:0:x}",
            "{:0:x}",
            "{}",
        ] {
            let out = rewrite(text, named("X"));
            assert_eq!(out.text, text, "{text}");
            assert!(out.references.is_empty(), "{text}");
        }
    }

    #[test]
    fn a_display_string_may_contain_colons() {
        let out = rewrite("{P:0:Act one: the rift}", |r| {
            assert_eq!(r.display, "Act one: the rift");
            None
        });
        assert_eq!(out.text, "Act one: the rift");
    }

    /// Manuskript's own pattern does not cross a line, and in converted prose a line
    /// break is a paragraph boundary no replacement can span.
    #[test]
    fn a_marker_never_spans_a_line() {
        let out = rewrite("{C:0:Pe\nter}", named("Peter"));
        assert_eq!(out.text, "{C:0:Pe\nter}");
        assert!(out.references.is_empty());
    }

    /// In Djot a typed marker is escaped, `\{C\:0\:Peter\}`. It is found in the prose
    /// the writer reads and replaced there, leaving no backslash behind, and the name is
    /// escaped against its neighbours on the way back to Djot.
    #[test]
    fn a_marker_in_djot_is_replaced_in_the_prose_it_shows() {
        let djot = skrib_format::markdown_to_djot("Then {C:0:Peter} spoke at 10:{T:1:30}:45.")
            .expect("convert");
        let out = rewrite_djot(&djot, |r| {
            Some(if r.kind == 'C' { "Peter" } else { "30" }.into())
        })
        .expect("rewrite");
        let (text, _) = skrib_format::djot_plain_text(&out.text).expect("read back");
        assert_eq!(
            text, "Then Peter spoke at 10:30:45.",
            "stored as {:?}",
            out.text
        );
        assert_eq!(out.references.len(), 2);
    }

    /// The marker's formatting is the name's: an italic marker gives an italic name.
    #[test]
    fn a_replaced_marker_keeps_its_formatting() {
        let djot = skrib_format::markdown_to_djot("See _{C:0:Peter}_ now.").expect("convert");
        let out = rewrite_djot(&djot, named("Paul")).expect("rewrite");
        assert_eq!(out.text, "See _Paul_ now.");
    }

    /// Prose without a marker is not rewritten at all, not even re-canonicalised.
    #[test]
    fn djot_with_no_marker_is_returned_byte_for_byte() {
        for djot in ["A *strong* word.", "A \\{brace\\} alone.", ""] {
            let out = rewrite_djot(djot, named("X")).expect("rewrite");
            assert_eq!(out.text, djot);
            assert!(out.references.is_empty());
        }
    }

    #[test]
    fn text_with_no_markers_at_all_is_returned_unchanged() {
        let out = rewrite("Just prose.", named("X"));
        assert_eq!(out.text, "Just prose.");
        assert!(out.references.is_empty());
    }
}
