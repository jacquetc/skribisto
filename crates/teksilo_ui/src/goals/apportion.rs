// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Splitting one target across several children so the parts sum to the whole, exactly.
//!
//! Largest-remainder (Hamilton) apportionment, the method seat allocation uses: hand every
//! child the floor of its quota, then give the leftover units to the largest fractional
//! parts. The point is the guarantee — `sum(apportion(g, w)) == g` for every input — which
//! is what makes the preview table's footer a fact rather than a rounding coincidence.
//!
//! Ties break by position, i.e. by binder stream order, so the same outline always
//! distributes the same way. Deterministic beats fair-looking here: a writer who runs the
//! action twice must get the same numbers.

/// Split `total` across `weights`, preserving the sum exactly.
///
/// * An empty `weights` returns an empty vec (the caller disables the action instead).
/// * All-zero weights degenerate to an even split, which is the only sensible reading of
///   "distribute by length" over an outline where nothing is written yet.
/// * A negative `total` is clamped to zero: there is no such thing as a negative target.
pub fn apportion(total: i64, weights: &[u64]) -> Vec<i64> {
    let n = weights.len();
    if n == 0 {
        return Vec::new();
    }
    let total = total.max(0);
    let sum: u128 = weights.iter().map(|w| u128::from(*w)).sum();
    // Nothing to weight by (a fresh outline, or every child excluded from the measure):
    // an even split is the honest fallback, not a division by zero.
    let even = sum == 0;
    let sum = if even { n as u128 } else { sum };

    let mut base = Vec::with_capacity(n);
    let mut remainders: Vec<(u128, usize)> = Vec::with_capacity(n);
    let mut assigned: i64 = 0;
    for (i, w) in weights.iter().enumerate() {
        let w = if even { 1u128 } else { u128::from(*w) };
        // Exact integer arithmetic: floor(total*w/sum) and the numerator of the fraction
        // left over. Doing this in f64 would make the sum guarantee depend on rounding.
        let scaled = u128::from(total as u64) * w;
        let q = (scaled / sum) as i64;
        base.push(q);
        remainders.push((scaled % sum, i));
        assigned += q;
    }
    // Largest fractional part first; ties keep their original order, so the earliest item
    // in the binder stream wins. The sort is stable, so the index tiebreak is implicit.
    remainders.sort_by_key(|(rem, _)| std::cmp::Reverse(*rem));
    let mut leftover = total - assigned;
    for (_, i) in remainders {
        if leftover == 0 {
            break;
        }
        base[i] += 1;
        leftover -= 1;
    }
    base
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sums_to(total: i64, weights: &[u64]) -> Vec<i64> {
        let out = apportion(total, weights);
        assert_eq!(
            out.iter().sum::<i64>(),
            total.max(0),
            "apportion must preserve the total exactly: {out:?}"
        );
        out
    }

    /// The worked example from the design: an even split that does not divide cleanly
    /// hands the odd word to the first chapter in stream order.
    #[test]
    fn ninety_thousand_over_seven_chapters_evenly() {
        let out = sums_to(90_000, &[1; 7]);
        assert_eq!(
            out,
            vec![12_858, 12_857, 12_857, 12_857, 12_857, 12_857, 12_857]
        );
    }

    /// Weighting by chapter count across three parts, exact by construction.
    #[test]
    fn ninety_thousand_over_parts_weighted_four_nine_two() {
        assert_eq!(sums_to(90_000, &[4, 9, 2]), vec![24_000, 54_000, 12_000]);
    }

    /// One word more, so a remainder exists: it goes to the largest fractional part, not
    /// to the first item.
    #[test]
    fn the_remainder_goes_to_the_largest_fraction() {
        assert_eq!(sums_to(90_001, &[4, 9, 2]), vec![24_000, 54_001, 12_000]);
    }

    #[test]
    fn all_zero_weights_split_evenly() {
        assert_eq!(sums_to(10, &[0, 0, 0, 0]), vec![3, 3, 2, 2]);
    }

    #[test]
    fn no_children_is_not_a_division_by_zero() {
        assert!(apportion(90_000, &[]).is_empty());
    }

    #[test]
    fn a_negative_total_clamps_to_zero() {
        assert_eq!(apportion(-5, &[1, 1]), vec![0, 0]);
    }

    #[test]
    fn one_child_takes_everything() {
        assert_eq!(sums_to(7_777, &[3]), vec![7_777]);
    }

    /// A weight of zero beside real ones still gets nothing rather than absorbing a
    /// rounding unit ahead of a child that actually has length.
    #[test]
    fn a_zero_weight_beside_real_ones_gets_nothing() {
        let out = sums_to(100, &[0, 50, 50]);
        assert_eq!(out[0], 0);
        assert_eq!(out[1] + out[2], 100);
    }

    /// A book-scale total across a book-scale outline, checked for the property rather
    /// than for exact figures.
    #[test]
    fn the_sum_holds_across_many_awkward_shapes() {
        for total in [1, 7, 999, 90_001, 1_234_567] {
            for weights in [
                vec![1; 3],
                vec![1; 30],
                vec![7, 11, 13, 17],
                vec![0, 1, 0, 1],
                vec![1_000_000, 1, 1],
            ] {
                sums_to(total, &weights);
            }
        }
    }
}
