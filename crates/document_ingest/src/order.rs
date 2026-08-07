// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What order a set of documents lands in.
//!
//! The single most reported import failure in the survey behind this feature is
//! not a parsing bug — it is this. Scrivener takes files in whatever order the
//! filesystem hands them over, and a writer's `1 - Scene` … `5 - Scene` landed
//! as 4, 1, 2, 5, 3. Nothing about that is recoverable afterwards except by hand.
//!
//! Two rules, in order:
//!
//! 1. An explicit `order:` in the document's own metadata wins. It is the most
//!    deliberate signal a writer can leave, and it is what every static-site
//!    generator's front matter already means.
//! 2. Otherwise, a **natural** sort of the file name: digit runs compare as
//!    numbers, so `chapter2` precedes `chapter10` — which plain lexicographic
//!    order gets backwards, and which is exactly how manuscripts are named.
//!
//! Neither is trusted to be right. The review step shows the resulting order and
//! lets it be changed, because a file called `interlude.md` sitting among
//! numbered scenes has no correct answer available from its name alone.

use crate::block::SourceDocument;

/// Sort documents into the order they should be imported in.
///
/// Stable: two documents that compare equal keep the order they arrived in, so
/// the caller's own sequence is the final tie-break rather than something
/// arbitrary.
pub fn sort_documents(docs: &mut [SourceDocument]) {
    docs.sort_by(|a, b| {
        match (a.metadata.order_hint, b.metadata.order_hint) {
            // Both declared: trust them.
            (Some(x), Some(y)) => x.cmp(&y),
            // One declared: it goes first. A writer who numbered *some* files
            // meant those to lead.
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => natural_cmp(&a.origin, &b.origin),
        }
    });
}

/// Compare two strings treating maximal digit runs as numbers.
///
/// `"chapter2" < "chapter10"`, `"01_a" < "2_b"`, and leading zeros do not change
/// the value — only the tie-break, where the shorter (fewer leading zeros) form
/// sorts first so the order is total rather than merely consistent.
///
/// Case folds first and only breaks ties at the very end. Doing it per character
/// instead looks equivalent and is not: `Chapter 10` and `chapter 2` differ at
/// the first letter, so a per-character tie-break decides the whole comparison
/// there and the numbers are never reached — which is the same
/// case-dominates-everything bug that puts `Z1` before `a2` in a naive sort.
pub fn natural_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    walk(a, b, Folding::CaseInsensitive).then_with(|| walk(a, b, Folding::CaseSensitive))
}

#[derive(Clone, Copy, PartialEq)]
enum Folding {
    CaseInsensitive,
    CaseSensitive,
}

fn walk(a: &str, b: &str, folding: Folding) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let mut left = a.char_indices().peekable();
    let mut right = b.char_indices().peekable();

    loop {
        match (left.peek().copied(), right.peek().copied()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some((li, lc)), Some((ri, rc))) => {
                if lc.is_ascii_digit() && rc.is_ascii_digit() {
                    let lnum = digit_run(a, li);
                    let rnum = digit_run(b, ri);
                    let lv = lnum.trim_start_matches('0');
                    let rv = rnum.trim_start_matches('0');
                    // Longer digit string (ignoring leading zeros) is the larger
                    // number; same length compares lexicographically, which for
                    // equal-length digits is numeric.
                    let ord = lv.len().cmp(&rv.len()).then_with(|| lv.cmp(rv));
                    if ord != Ordering::Equal {
                        return ord;
                    }
                    // Equal value, different spelling: `1` precedes `01`. Only a
                    // tie-break, so it never overrules the number itself.
                    if lnum.len() != rnum.len() && folding == Folding::CaseSensitive {
                        return lnum.len().cmp(&rnum.len());
                    }
                    for _ in 0..lnum.chars().count() {
                        left.next();
                    }
                    for _ in 0..rnum.chars().count() {
                        right.next();
                    }
                } else {
                    let ord = match folding {
                        Folding::CaseInsensitive => lc.to_lowercase().cmp(rc.to_lowercase()),
                        Folding::CaseSensitive => lc.cmp(&rc),
                    };
                    if ord != Ordering::Equal {
                        return ord;
                    }
                    left.next();
                    right.next();
                }
            }
        }
    }
}

/// The maximal run of ASCII digits starting at byte index `from`.
fn digit_run(s: &str, from: usize) -> &str {
    let end = s[from..]
        .find(|c: char| !c.is_ascii_digit())
        .map(|i| from + i)
        .unwrap_or(s.len());
    &s[from..end]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn doc(origin: &str, order: Option<i64>) -> SourceDocument {
        let mut d = SourceDocument::new("x", origin);
        d.metadata.order_hint = order;
        d
    }

    fn origins(docs: &[SourceDocument]) -> Vec<&str> {
        docs.iter().map(|d| d.origin.as_str()).collect()
    }

    /// The failure this module exists for: Scrivener took a writer's five
    /// numbered scenes and produced 4, 1, 2, 5, 3.
    #[test]
    fn filesystem_order_is_replaced_by_the_writers_numbering() {
        let mut docs = vec![
            doc("4 - Scene.md", None),
            doc("1 - Scene.md", None),
            doc("2 - Scene.md", None),
            doc("5 - Scene.md", None),
            doc("3 - Scene.md", None),
        ];
        sort_documents(&mut docs);
        assert_eq!(
            origins(&docs),
            vec![
                "1 - Scene.md",
                "2 - Scene.md",
                "3 - Scene.md",
                "4 - Scene.md",
                "5 - Scene.md"
            ]
        );
    }

    /// The case plain lexicographic sorting gets backwards, and the reason a
    /// natural sort is not optional.
    #[test]
    fn ten_comes_after_two_not_before_it() {
        let mut docs = vec![
            doc("chapter10.md", None),
            doc("chapter2.md", None),
            doc("chapter1.md", None),
        ];
        sort_documents(&mut docs);
        assert_eq!(
            origins(&docs),
            vec!["chapter1.md", "chapter2.md", "chapter10.md"]
        );
    }

    #[test]
    fn leading_zeros_do_not_change_the_number() {
        assert_eq!(natural_cmp("01_a", "2_a"), Ordering::Less);
        assert_eq!(
            natural_cmp("007", "7"),
            Ordering::Greater,
            "same value, longer form second"
        );
        assert_eq!(natural_cmp("09", "10"), Ordering::Less);
    }

    #[test]
    fn an_explicit_order_key_beats_the_file_name() {
        let mut docs = vec![
            doc("zzz.md", Some(1)),
            doc("aaa.md", Some(2)),
            doc("mmm.md", None),
        ];
        sort_documents(&mut docs);
        assert_eq!(origins(&docs), vec!["zzz.md", "aaa.md", "mmm.md"]);
    }

    #[test]
    fn mixed_naming_conventions_still_interleave_by_number() {
        let mut docs = vec![
            doc("Chapter 10.md", None),
            doc("chapter 2.md", None),
            doc("CHAPTER 1.md", None),
        ];
        sort_documents(&mut docs);
        assert_eq!(
            origins(&docs),
            vec!["CHAPTER 1.md", "chapter 2.md", "Chapter 10.md"]
        );
    }

    #[test]
    fn a_name_with_no_number_sorts_alphabetically_among_the_rest() {
        let mut docs = vec![
            doc("02_b.md", None),
            doc("interlude.md", None),
            doc("01_a.md", None),
        ];
        sort_documents(&mut docs);
        assert_eq!(origins(&docs), vec!["01_a.md", "02_b.md", "interlude.md"]);
    }

    #[test]
    fn equal_names_keep_their_arrival_order() {
        let mut docs = vec![doc("a.md", None), doc("a.md", None)];
        sort_documents(&mut docs);
        assert_eq!(docs.len(), 2);
    }
}
