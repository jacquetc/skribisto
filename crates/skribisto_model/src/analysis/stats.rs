// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The small-sample statistics every measure in [`crate::analysis`] shares.
//!
//! Tiny, and that is the point. Three of these grew independently while this module tree was
//! written — a mean here, a mean-and-variance there, a median in the UI — which is exactly
//! the drift [`crate::analysis`]'s own doc warns about for tokenization: two definitions of
//! one thing eventually disagree, and a panel that reports a mean computed two ways is
//! reporting neither.
//!
//! **Population, not sample.** Every caller here has the whole population in hand — all of a
//! scene's sentences, all of a book's scenes — so there is no estimation and no `n − 1`
//! correction. Using the sample form would understate nothing and overstate the spread of
//! every short scene.

/// Arithmetic mean, or `None` for an empty slice.
///
/// `None` rather than `0.0`: the mean of nothing is not zero, and a zero here would draw a
/// bar and read as a measurement.
pub fn mean(xs: &[f64]) -> Option<f64> {
    (!xs.is_empty()).then(|| xs.iter().sum::<f64>() / xs.len() as f64)
}

/// Population standard deviation, or `None` for fewer than two values.
///
/// Two is the floor because the spread of a single sample is not a number — not zero, which
/// would read as "perfectly consistent".
pub fn population_stddev(xs: &[f64]) -> Option<f64> {
    if xs.len() < 2 {
        return None;
    }
    let m = mean(xs)?;
    let var = xs.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / xs.len() as f64;
    Some(var.sqrt())
}

/// Median, or `None` for an empty slice.
///
/// Preferred over the mean wherever a *threshold* is wanted rather than a summary: one
/// 6,000-word chapter drags a mean far enough to change how every other chapter is
/// classified, and leaves a median untouched.
pub fn median(xs: &[f64]) -> Option<f64> {
    if xs.is_empty() {
        return None;
    }
    let mut v = xs.to_vec();
    v.sort_by(f64::total_cmp);
    let n = v.len();
    Some(if n % 2 == 1 { v[n / 2] } else { (v[n / 2 - 1] + v[n / 2]) / 2.0 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_of_nothing_is_absent_not_zero() {
        assert_eq!(mean(&[]), None);
        assert_eq!(mean(&[2.0, 4.0]), Some(3.0));
    }

    #[test]
    fn one_sample_has_no_deviation_to_report() {
        assert_eq!(population_stddev(&[5.0]), None);
        assert_eq!(population_stddev(&[]), None);
    }

    #[test]
    fn deviation_is_the_population_form() {
        // Population sd of [2, 6] is 2.0; the sample form would give ~2.83.
        assert_eq!(population_stddev(&[2.0, 6.0]), Some(2.0));
    }

    /// The property that motivates preferring it for thresholds.
    #[test]
    fn the_median_ignores_an_outlier_the_mean_chases() {
        let xs = [1.0, 2.0, 3.0, 4.0, 100.0];
        assert_eq!(median(&xs), Some(3.0));
        assert_eq!(mean(&xs), Some(22.0));
    }

    #[test]
    fn median_of_an_even_count_is_the_midpoint_of_the_middle_two() {
        assert_eq!(median(&[2.0, 4.0]), Some(3.0));
        assert_eq!(median(&[7.0]), Some(7.0));
        assert_eq!(median(&[]), None);
    }

    /// `sort_by(f64::total_cmp)` rather than `partial_cmp().unwrap()`: a NaN reaching this
    /// from a degenerate measurement must not panic the whole analysis.
    #[test]
    fn a_nan_does_not_panic_the_sort() {
        let m = median(&[3.0, f64::NAN, 1.0]);
        assert!(m.is_some());
    }
}
