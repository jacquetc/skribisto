// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Recording every marked copy of the manuscript that goes out.
//!
//! `export_work` publishes what it sent — the rows it marked and the digest each
//! carried — on the `ExportWork` event's payload (`export_management::events`).
//! Nothing subscribed to that event before this: the UI listens to
//! `Origin::LongOperation` for progress and completion, and the export's own
//! event went nowhere.
//!
//! This is the listener, and it does one thing: put the receipt in
//! [`ExchangeService`] so the questions a written file cannot answer — how many
//! copies are out, what a row said when it left, whether a copy ever carried a
//! given thread — have somewhere to be answered from.
//!
//! **Keyed on `Work.unique_id` from the payload, never on the event's `ids`.**
//! Several `Work`s are open at once and the ids are store ids, re-minted by every
//! `load_work`; the payload carries the durable key precisely so a listener does
//! not have to resolve one.
//!
//! Failure here is silent by design. An export that wrote the writer's file
//! correctly has succeeded, and a record that could not be kept must not turn
//! that into an error — the readouts that depend on it simply have less to say.

use teksilo::prelude::*;

use frontend::common::event::{Event, ExportManagementEvent, Origin};

use crate::models::{ExchangeService, SentPackage, SentRow};

/// Subscribe the exchange record to the export event.
pub(crate) fn wire(ctx: &mut BuildContext, exchange: &ExchangeService, title: Signal<String>) {
    let exchange = exchange.clone();
    ctx.subscribe_event(
        Origin::ExportManagement(ExportManagementEvent::ExportWork),
        move |e: &Event| {
            let Some(data) = e.data.as_deref() else {
                return;
            };
            let Some(origins) = export_management::events::ExportOrigins::from_payload(data) else {
                return;
            };
            // An export written without round-trip marks carries no rows, and
            // `record` refuses it — a copy nothing can be matched against is not
            // a copy that is meaningfully "out". Checked here too so the common
            // case does not build a package only to have it dropped.
            if origins.rows.is_empty() {
                return;
            }
            exchange.record(
                &origins.work_unique_id,
                &title.get(),
                SentPackage {
                    sent_at: origins.exported_at,
                    output_path: origins.output_path,
                    format: origins.format,
                    rows: origins
                        .rows
                        .into_iter()
                        .map(|r| SentRow {
                            item_uid: r.item_uid,
                            digest: r.digest,
                        })
                        .collect(),
                    comment_uids: origins.comment_uids,
                },
            );
        },
    );
}
