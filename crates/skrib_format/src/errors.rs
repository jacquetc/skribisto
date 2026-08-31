// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The typed failure contract of [`read_bundle`](super::read_bundle).
//!
//! Everything this crate can fail with used to be an opaque `anyhow::Error`, which
//! made "this file is from the future" indistinguishable from "this file is corrupt"
//! at the UI leaf — so both got the same generic toast. One of those is a *recoverable*
//! condition with a specific instruction for the reader ("update Skribisto"); the other
//! is not. They deserve different messages, and only the format crate can tell them
//! apart, so the distinction is made here rather than re-derived from message text.
//!
//! [`Unreadable`](SkribFormatError::Unreadable) stays an `anyhow::Error` on purpose:
//! its variants (corrupt RON, missing blob, unknown header, I/O failure) already carry
//! good, specific prose at the point of failure, and enumerating them would buy nothing
//! the UI acts on differently.

use std::fmt;

/// Why a `.skrib` could not be read.
///
/// Carries `Send + Sync + 'static`, so every existing `.context(…)` / `.with_context(…)`
/// call site keeps compiling through `anyhow`'s blanket `From` — and a leaf can recover
/// the typed value with `anyhow::Error::downcast_ref`, which sees through those context
/// layers.
#[derive(Debug)]
pub enum SkribFormatError {
    /// The bundle declares it needs a format this build does not implement.
    ///
    /// `written_by` is the writer's own generation, `requires_at_least` the floor it
    /// actually needs (see [`format_min_read_version`](crate::ProjectManifest::format_min_read_version)).
    /// The two differ whenever a newer build wrote a project that happens to contain
    /// nothing new — which is the common case, and precisely the case this scheme keeps
    /// openable.
    TooNew {
        written_by: u32,
        requires_at_least: u32,
        supported: u32,
    },
    /// `format_version` is 0 — no build ever wrote that, so the manifest is bogus.
    InvalidVersion,
    /// Everything else: corrupt RON, a missing blob, an unrecognised header, I/O failure,
    /// or the legacy SQLite shape handed to the wrong reader.
    Unreadable(anyhow::Error),
}

impl fmt::Display for SkribFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooNew {
                written_by,
                requires_at_least,
                supported,
            } => write!(
                f,
                "this .skrib needs format {requires_at_least} or newer to open \
                 (written by format {written_by}; this build supports up to {supported})"
            ),
            Self::InvalidVersion => write!(f, "invalid .skrib: format_version is 0"),
            Self::Unreadable(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for SkribFormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Unreadable(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

impl From<anyhow::Error> for SkribFormatError {
    fn from(e: anyhow::Error) -> Self {
        Self::Unreadable(e)
    }
}
