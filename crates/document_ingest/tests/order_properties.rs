// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! `natural_cmp` and `sort_documents`, over every pair rather than the pairs someone drew.
//!
//! The module's own docs call the order "total rather than merely consistent". That is a
//! claim with teeth, and not only a pedantic one: `slice::sort_by` is entitled to panic when
//! a comparator does not implement a total order, so a comparator with one bad triple turns
//! an import into a crash. Nothing about `natural_cmp` makes it obviously total either — it
//! is a lexicographic composition of two walks over a string, each of which switches between
//! comparing characters and comparing whole digit runs, with a case fold on one pass and not
//! the other, and a leading-zero tie-break that fires on only one of them.
//!
//! Checking that by hand means picking triples. There are `n³` of them.
//!
//! # The alphabet
//!
//! Narrow and deliberate. Digits and leading zeros are what the natural sort is for; mixed
//! case is what the two passes exist to separate (`Chapter 10` against `chapter 2`, where a
//! per-character tie-break would decide on the first letter and never reach the numbers); the
//! dot and space are the separators real manuscript file names use; and `İ`/`ı` are there
//! because the case-insensitive pass compares `char::to_lowercase` *iterators*, and `İ`
//! lowercases to two code points while every other letter lowercases to one.

use document_ingest::block::SourceDocument;
use document_ingest::order::{natural_cmp, sort_documents};
use proptest::prelude::*;
use std::cmp::Ordering;

/// File-name-shaped strings over the characters that make this comparator interesting.
fn name_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec![
            "0", "1", "2", "9", "10", "a", "A", "b", ".", " ", "-", "\u{130}", "\u{131}", "i",
        ]),
        0..5,
    )
    .prop_map(|parts| parts.concat())
}

proptest! {
    /// The comparator is a total order.
    ///
    /// Three laws, all three required by `sort_by`: comparing a value with itself is `Equal`,
    /// swapping the arguments reverses the answer, and the order composes. A violation is not
    /// a mis-sorted list — it is a sort that may panic, on a writer's own file names, during
    /// an import.
    #[test]
    fn the_natural_order_is_total(a in name_strategy(), b in name_strategy(), c in name_strategy()) {
        prop_assert_eq!(natural_cmp(&a, &a), Ordering::Equal, "{:?} is not equal to itself", a);
        prop_assert_eq!(
            natural_cmp(&a, &b),
            natural_cmp(&b, &a).reverse(),
            "{:?} and {:?} do not compare antisymmetrically",
            a,
            b
        );

        let (ab, bc, ac) = (natural_cmp(&a, &b), natural_cmp(&b, &c), natural_cmp(&a, &c));
        let consistent = match (ab, bc) {
            (Ordering::Less, Ordering::Less) => ac == Ordering::Less,
            (Ordering::Greater, Ordering::Greater) => ac == Ordering::Greater,
            (Ordering::Equal, Ordering::Equal) => ac == Ordering::Equal,
            (Ordering::Equal, other) | (other, Ordering::Equal) => ac == other,
            _ => true,
        };
        prop_assert!(
            consistent,
            "transitivity fails: {a:?} {ab:?} {b:?}, {b:?} {bc:?} {c:?}, but {a:?} {ac:?} {c:?}"
        );
    }

    /// A digit run compares as a number, whatever its length or its leading zeros.
    ///
    /// The whole reason the comparator exists. Plain lexicographic order puts `chapter10`
    /// before `chapter2`, and a writer's scenes are named exactly that way — which the
    /// module's own docs record as the single most reported import failure behind this
    /// feature, and one nothing downstream can recover from except by hand.
    #[test]
    fn a_digit_run_compares_as_a_number(
        n in 0u32..2000,
        m in 0u32..2000,
        pad_n in 0usize..3,
        pad_m in 0usize..3,
        stem in prop::sample::select(vec!["chapter", "scene ", "", "Part-"]),
    ) {
        let a = format!("{stem}{:0>width$}", n, width = n.to_string().len() + pad_n);
        let b = format!("{stem}{:0>width$}", m, width = m.to_string().len() + pad_m);
        let got = natural_cmp(&a, &b);
        match n.cmp(&m) {
            Ordering::Equal => {
                // Equal value, possibly different spelling: fewer leading zeros sorts first,
                // which is a tie-break and must never overrule the number itself.
                prop_assert_eq!(
                    got,
                    pad_n.cmp(&pad_m),
                    "{:?} and {:?} hold the same number but ordered by something else",
                    a,
                    b
                );
            }
            expected => prop_assert_eq!(
                got,
                expected,
                "{:?} vs {:?}: ordered {:?}, but {} vs {} is {:?}",
                a,
                b,
                got,
                n,
                m,
                expected
            ),
        }
    }

    /// Case never decides a comparison that the numbers can decide.
    ///
    /// The reason the comparator folds case on a whole first pass instead of per character.
    /// `Chapter 10` and `chapter 2` differ at their first letter, so a per-character tie-break
    /// settles the whole comparison there and the numbers are never reached — the same bug
    /// that puts `Z1` before `a2`.
    #[test]
    fn case_never_outranks_a_number(
        n in 0u32..500,
        m in 0u32..500,
        upper_a in any::<bool>(),
        upper_b in any::<bool>(),
    ) {
        prop_assume!(n != m);
        let stem = |upper: bool| if upper { "Chapter " } else { "chapter " };
        let a = format!("{}{n}", stem(upper_a));
        let b = format!("{}{m}", stem(upper_b));
        prop_assert_eq!(
            natural_cmp(&a, &b),
            n.cmp(&m),
            "{:?} vs {:?}: case decided the order instead of the number",
            a,
            b
        );
    }

    /// Sorting keeps every document, and puts the ones that declared an order first.
    ///
    /// Two rules, in that order: a writer who numbered *some* of their files meant those to
    /// lead, and everything else falls back to the natural sort of the name. The permutation
    /// half is the one a writer would notice — a scene dropped by the sort is a scene missing
    /// from the import, with nothing to say so.
    #[test]
    fn sorting_keeps_every_document_and_honours_an_explicit_order(
        docs in prop::collection::vec((name_strategy(), prop::option::of(0i64..20)), 0..10)
    ) {
        let mut sorted: Vec<SourceDocument> = docs
            .iter()
            .enumerate()
            .map(|(i, (origin, hint))| {
                // The origin carries the index too, so two documents with the same generated
                // name stay distinguishable and the permutation check means something.
                let mut d = SourceDocument::new("x", format!("{origin}#{i}"));
                d.metadata.order_hint = *hint;
                d
            })
            .collect();
        let before: Vec<String> = sorted.iter().map(|d| d.origin.clone()).collect();

        sort_documents(&mut sorted);

        let mut a = before.clone();
        let mut b: Vec<String> = sorted.iter().map(|d| d.origin.clone()).collect();
        a.sort();
        b.sort();
        prop_assert_eq!(a, b, "the sort lost or duplicated a document");

        // Declared order leads, and within it the declared values ascend.
        let hints: Vec<Option<i64>> = sorted.iter().map(|d| d.metadata.order_hint).collect();
        let first_none = hints.iter().position(Option::is_none).unwrap_or(hints.len());
        prop_assert!(
            hints[first_none..].iter().all(Option::is_none),
            "a document with an explicit order sorted after one without: {hints:?}"
        );
        let declared: Vec<i64> = hints[..first_none].iter().flatten().copied().collect();
        prop_assert!(
            declared.windows(2).all(|w| w[0] <= w[1]),
            "declared orders are not ascending: {declared:?}"
        );
    }

    /// Sorting is idempotent.
    ///
    /// A sort that moved something on a second pass would not be a sort, and it is the
    /// cheapest available check that the comparator's ties are genuinely ties rather than an
    /// inconsistency the first pass happened to hide.
    #[test]
    fn sorting_twice_changes_nothing(
        docs in prop::collection::vec((name_strategy(), prop::option::of(0i64..20)), 0..10)
    ) {
        let build = || -> Vec<SourceDocument> {
            docs.iter()
                .enumerate()
                .map(|(i, (origin, hint))| {
                    let mut d = SourceDocument::new("x", format!("{origin}#{i}"));
                    d.metadata.order_hint = *hint;
                    d
                })
                .collect()
        };
        let mut once = build();
        sort_documents(&mut once);
        let after_once: Vec<String> = once.iter().map(|d| d.origin.clone()).collect();

        sort_documents(&mut once);
        let after_twice: Vec<String> = once.iter().map(|d| d.origin.clone()).collect();
        prop_assert_eq!(after_once, after_twice, "a second sort moved something");
    }
}
