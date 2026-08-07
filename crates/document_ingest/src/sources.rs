// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One module per input format — the swappable half of the pipeline.
//!
//! Each implements `SourceScanner` and nothing else depends on which one ran.
//! A `.docx` or `.odt` scanner belongs here beside these two; when it arrives it
//! will want heavier dependencies (a zip reader, an XML parser), which is the
//! point at which these should each go behind a cargo feature so a consumer that
//! only imports Markdown does not build an OOXML parser.

pub mod markdown;
pub mod plain;
