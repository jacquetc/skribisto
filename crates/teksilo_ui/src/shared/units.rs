// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Numbers with a unit, for a reader: `12 KB`, `1.4 MB`, `85%`, `14 px`.
//!
//! The unit and the spacing around it belong to the language (French writes
//! `85 %` and `1,4 Mo`), so the number goes to the catalogue and comes back
//! with its unit.
//!
//! For file sizes there were three private copies (backup sizes, the backups list,
//! the dictionary catalogue), each with its own unit spelling ("kB" beside
//! "KB") and all with an English decimal point. The unit symbols and the
//! decimal mark belong to the language — French writes `1,4 Mo` — so the
//! number is split here and the catalogue puts it back together.

use teksilo::prelude::*;

const KIB: u64 = 1024;
const MIB: u64 = KIB * 1024;
const GIB: u64 = MIB * 1024;

/// `bytes` with one unit: whole bytes and kilobytes, megabytes and gigabytes
/// to one decimal. Kilobytes round up, so a non-empty file never reads as 0.
pub fn byte_size(bytes: u64) -> String {
    if bytes < KIB {
        return tr!(byte_size_bytes(value = bytes as i64)).resolve_now();
    }
    if bytes < MIB {
        return tr!(byte_size_kb(value = bytes.div_ceil(KIB) as i64)).resolve_now();
    }
    let (unit, tenths) = if bytes < GIB {
        ("mb", tenths_of(bytes, MIB))
    } else {
        ("gb", tenths_of(bytes, GIB))
    };
    let whole = (tenths / 10) as i64;
    let tenth = (tenths % 10) as i64;
    match unit {
        "mb" => tr!(byte_size_mb(whole = whole, tenth = tenth)),
        _ => tr!(byte_size_gb(whole = whole, tenth = tenth)),
    }
    .resolve_now()
}

/// A whole percentage: `fraction` is `0.85` for `85%`.
pub fn percent(fraction: f64) -> String {
    tr!(unit_percent(value = (fraction * 100.0).round() as i64)).resolve_now()
}

/// A whole number of pixels.
pub fn pixels(value: f64) -> String {
    tr!(unit_pixels(value = value.round() as i64)).resolve_now()
}

/// `bytes / unit`, in tenths, rounded to the nearest.
fn tenths_of(bytes: u64, unit: u64) -> u64 {
    (bytes as u128 * 10 + unit as u128 / 2)
        .checked_div(unit as u128)
        .unwrap_or(0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_scale_reads_with_its_unit() {
        assert_eq!(byte_size(0), "0 B");
        assert_eq!(byte_size(1), "1 B");
        assert_eq!(byte_size(1024), "1 KB");
        assert_eq!(byte_size(1025), "2 KB", "kilobytes round up");
        assert_eq!(byte_size(MIB), "1.0 MB");
        assert_eq!(byte_size(MIB + MIB / 2), "1.5 MB");
        assert_eq!(byte_size(GIB), "1.0 GB");
    }

    #[test]
    fn percent_and_pixels_round_to_whole_numbers() {
        assert_eq!(percent(0.854), "85%");
        assert_eq!(pixels(13.6), "14 px");
    }

    #[test]
    fn a_tenth_that_rounds_up_carries_into_the_whole() {
        assert_eq!(byte_size(2 * MIB - 1), "2.0 MB");
    }
}
