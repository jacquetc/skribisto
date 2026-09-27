// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A zipped `.msk` built to unpack far larger than it is is refused before it is read.
//!
//! Two shapes, the two an untrusted zip can carry: a compressible member whose header
//! honestly states a size past what a project may hold, and a zip64 over-declared
//! member holding a few real bytes under an enormous declared size. Both are refused
//! over the central directory, before a byte is inflated, as the typed
//! `skrib_format::zip_guard::ZipRefused`. The test is instant and never allocates the
//! claimed size; the `ulimit -v` proof that each aborted before the guard is in the
//! SubagentHandback report.

use std::sync::atomic::AtomicBool;

use manuskript_import::map::Names;
use skrib_format::zip_guard::fixtures::{compressible_zip, over_declared_zip};

const BOMB_MIB: u64 = 96;
const OVER_DECLARED: u64 = 16 << 30;

fn names() -> Names {
    Names {
        manuscript_binder: "Manuscript".into(),
        story_bible_binder: "Story bible".into(),
        characters_group: "Characters".into(),
        world_group: "World".into(),
        plots_group: "Plots".into(),
        project_info_note: "Project information".into(),
        summary_note: "Summary".into(),
        importance: ["Minor".into(), "Secondary".into(), "Main".into()],
    }
}

fn import_refusal(archive: &[u8]) -> anyhow::Error {
    let dir = tempfile::tempdir().expect("temp dir");
    let source = dir.path().join("hostile.msk");
    std::fs::write(&source, archive).expect("write source");
    let output = dir.path().join("out.skrib");
    match manuskript_import::import_with_progress(
        &source.to_string_lossy(),
        &output.to_string_lossy(),
        true,
        &names(),
        &|_, _| {},
        &AtomicBool::new(false),
    ) {
        Ok(_) => panic!("a hostile .msk must be refused"),
        Err(e) => e,
    }
}

#[test]
fn a_msk_decompression_bomb_is_refused() {
    let err = import_refusal(&compressible_zip("outline/scene.xml", BOMB_MIB << 20));
    assert!(
        skrib_format::zip_guard::refused(&err).is_some(),
        "expected a zip refusal, got: {err:#}"
    );
}

#[test]
fn a_msk_zip64_over_declared_member_is_refused() {
    let err = import_refusal(&over_declared_zip("outline/scene.xml", OVER_DECLARED));
    assert!(
        skrib_format::zip_guard::refused(&err).is_some(),
        "expected a zip refusal, got: {err:#}"
    );
}
