// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Rendering a stored timestamp for the writer to read.

use chrono::{DateTime, Datelike, Timelike, Utc};

/// `DD/MM/YYYY HH:MM` in the machine's own timezone.
///
/// The conversion is the whole point, and it is easy to get wrong in a way that
/// looks right: `DateTime<Utc>::naive_local` is a no-op, because chrono reads
/// "local" as local-to-the-`Tz`-parameter and UTC's offset is zero by
/// definition. Calling it reads like a conversion and performs none, which is
/// how every timestamp in the app once came out in UTC while claiming to be the
/// writer's own clock. `with_timezone(&Local)` first is what was meant.
///
/// This lived as two byte-identical copies — including this comment — in the
/// outline card and the comment card.
pub fn stamp(t: DateTime<Utc>) -> String {
    let t = t.with_timezone(&chrono::Local).naive_local();
    format!(
        "{:02}/{:02}/{:04} {:02}:{:02}",
        t.day(),
        t.month(),
        t.year(),
        t.hour(),
        t.minute()
    )
}
