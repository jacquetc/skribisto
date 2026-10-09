// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The Scrivener source layer's integration tests, in one test binary so the
//! suite links once.
//!
//! * `generations`: a project of every generation and platform reads into the
//!   model it was built from;
//! * `containers`: the same project as a folder, a picked manifest, a zip with and
//!   without a wrapping folder, and a template in both encodings;
//! * `traps`: what real projects hold that a naive reader gets wrong, one test per
//!   trap;
//! * `hostile`: archives and folders built to hurt;
//! * `properties`: statements about every binder and every archive;
//! * `structure`: the structure plan an analysis returns, and its invariants
//!   over any binder the builder writes;
//! * `convert`: the converter end to end, its `.skrib` read back, loaded into a real
//!   store, and every text put through the editor's save;
//! * `metadata`: the title, authors and language a project states, its labels and its
//!   status list, its dates, what stays in Scrivener, for every generation that writes
//!   them, and their round trip through the store;
//! * `s2`: the second half of the metadata (keywords, custom metadata, collections,
//!   bookmarks, targets, the deadline, the auto-complete list, the "From Scrivener"
//!   folder) for every generation that writes each, and its round trip through the store;
//! * `corpus`: real projects, opt-in (see its module doc);
//! * `ground_truth`: the project owner's own projects, written by Scrivener 3.5.2 for Mac
//!   and committed under `tests/fixtures/`, one test per change they asked for.

mod containers;
mod convert;
mod corpus;
mod generations;
mod ground_truth;
mod hostile;
mod metadata;
mod properties;
mod s2;
mod structure;
mod support;
mod traps;
