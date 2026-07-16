// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Shared helpers for consuming `Origin::LongOperation(...)` events in view-models.
//!
//! The long-operation manager emits its lifecycle events with a JSON `data`
//! payload (`{"id": …, "percentage": …, "message": …, "error": …}`). Several
//! view-models (import, save-as, …) filter these by the in-flight operation id,
//! so the payload parsing lives here once rather than being copied per feature.

use frontend::common::event::Event;

/// Parse a `LongOperation` event's JSON payload (`{"id":…, "percentage":…, …}`).
pub(crate) fn parse_payload(event: &Event) -> Option<serde_json::Value> {
    event
        .data
        .as_ref()
        .and_then(|s| serde_json::from_str(s).ok())
}

/// The operation id inside an already-parsed payload.
pub(crate) fn payload_id(payload: &serde_json::Value) -> Option<&str> {
    payload.get("id").and_then(|i| i.as_str())
}

/// The operation id carried by an event (parse + extract in one step).
pub(crate) fn event_id(event: &Event) -> Option<String> {
    parse_payload(event)?
        .get("id")
        .and_then(|i| i.as_str())
        .map(str::to_string)
}
