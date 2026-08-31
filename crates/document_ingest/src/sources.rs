// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! One module per input format — the swappable half of the pipeline.
//!
//! Each implements `SourceScanner` and nothing else depends on which one ran.
//!
//! Each is behind its own cargo feature, which is what this module asked for when
//! `.docx` and `.odt` were still hypothetical: they want heavier dependencies (a
//! zip reader, an XML parser, an OOXML reader), and a consumer that only imports
//! Markdown should not build them. `ScannerRegistry::with_builtin_scanners`
//! registers whatever is compiled in, and every list the UI shows is derived from
//! the registry — so turning a format off removes it from the file filter and the
//! drop zone's accept list too, rather than offering a format that then reports
//! itself unsupported.
//!
//! [`rich`] is the shared half of the two container formats. It is *not* behind a
//! feature of its own: it belongs to whichever of `odt` and `docx` is enabled, and
//! to neither when both are off.

#[cfg(feature = "docx")]
pub mod docx;
#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "odt")]
pub mod odt;
#[cfg(feature = "markdown")]
pub mod plain;
#[cfg(any(feature = "odt", feature = "docx"))]
pub mod rich;
