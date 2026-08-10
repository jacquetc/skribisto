// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! A hand-run harness for reading a real returning file: scan a `.docx`/`.odt` from the
//! command line and report what came back — how many rows the marks identified, what the
//! digests say changed, and what the editor's comments look like.
//!
//! `#[ignore]`d and env-driven, for the same reason `frontend`'s `round_trip_harness` is: the
//! interesting inputs are manuscripts this repo does not own and cannot commit. The automated
//! proof runs over the fixtures in `tests/fixtures/`.
//!
//! ```bash
//! SKRIB_SCAN=~/Documents/tests/elise-roundtrip.odt \
//!   cargo test -p skribisto-document-ingest --test scan_harness -- --ignored --nocapture
//! ```

use document_ingest::block::SourceBlock;
use document_ingest::scanner::ScannerRegistry;

#[test]
#[ignore = "hand-run: needs SKRIB_SCAN"]
fn scan_a_returning_file() {
    let Ok(path) = std::env::var("SKRIB_SCAN") else {
        eprintln!("skipping: set SKRIB_SCAN to a .docx/.odt");
        return;
    };
    let path = std::path::PathBuf::from(path);
    let bytes = std::fs::read(&path).expect("read the file");
    let doc = ScannerRegistry::with_builtin_scanners().scan_bytes(&path, &bytes);

    let headings = doc
        .blocks
        .iter()
        .filter(|b| matches!(b, SourceBlock::Heading { .. }))
        .count();
    println!("{}", path.display());
    println!(
        "  {} block(s), {headings} heading(s), {} annotation(s), {} row mark(s)",
        doc.blocks.len(),
        doc.annotations.len(),
        doc.row_marks.len()
    );

    for m in doc.row_marks.iter().take(5) {
        let at = doc
            .blocks
            .get(m.block_index)
            .map(|b| b.plain_text().chars().take(48).collect::<String>())
            .unwrap_or_default();
        println!("  row {} digest {} → {at:?}", m.uid_tag, m.digest);
    }
    if doc.row_marks.len() > 5 {
        println!("  … {} more", doc.row_marks.len() - 5);
    }

    for a in &doc.annotations {
        let quoted = doc
            .blocks
            .get(a.block_index)
            .map(|b| {
                b.plain_text()
                    .chars()
                    .skip(a.anchor.start)
                    .take(a.anchor.length.max(1))
                    .collect::<String>()
            })
            .unwrap_or_default();
        println!(
            "  comment {:?} by {:?} tag={:?} uid={:?} on {quoted:?} (kind {:?})",
            a.body, a.author, a.uid_tag, a.uid, a.kind
        );
        for r in &a.replies {
            println!("    reply {:?} by {:?}", r.body, r.author);
        }
    }

    for d in doc.diagnostics.iter().take(8) {
        println!("  diagnostic: {d:?}");
    }

    // The measurement the whole three-way comparison rests on: for a row nobody edited, does
    // the digest the file carries still equal the digest of the prose that came back?
    //
    // The mark's digest was taken over the *exported* row's prose; this recomputes it over the
    // prose the scanner produced from the file. Everything between the two — a preset's
    // rendering, ODF or OOXML's own idea of whitespace, an editor's save — is what
    // `round_trip::normalize` has to cancel. If these disagree on an untouched manuscript, the
    // status of every row reads "the editor edited it" and the merge step is worthless.
    let levels: Vec<u8> = doc
        .blocks
        .iter()
        .filter_map(|b| match b {
            SourceBlock::Heading { level, .. } => Some(*level),
            _ => None,
        })
        .collect();
    let rules = document_ingest::structure::infer_rules(&levels, skribisto_model::CreateType::Book);
    let plan = document_ingest::plan::build_plan(
        std::slice::from_ref(&doc),
        &rules,
        skribisto_model::ChapterMode::Folder,
        0,
    );

    let mut marked = 0usize;
    let mut same = 0usize;
    for row in &plan.rows {
        let Some(want) = row.source_digest.as_deref() else {
            continue;
        };
        marked += 1;
        let plain = skrib_format::djot_plain_text(&row.djot)
            .map(|(t, _)| t)
            .unwrap_or_default();
        let got = skribisto_model::round_trip::digest(&plain);
        if got == want {
            same += 1;
        } else {
            println!(
                "  DIGEST DIFFERS on {:?}: file says {want}, prose here digests {got} \
                 (breaks={}, words={})",
                row.title, row.scene_breaks, row.word_count
            );
            let stripped = skribisto_model::scene_break::strip_markers_djot(&row.djot);
            let stripped_plain = skrib_format::djot_plain_text(&stripped)
                .map(|(t, _)| t)
                .unwrap_or_default();
            println!(
                "    without break markers digests {}",
                skribisto_model::round_trip::digest(&stripped_plain)
            );
            // Given the row's *stored* Djot (SKRIB_COMPARE_DJOT), say where the two normalised
            // forms first part company. Guessing at this from digests alone is hopeless — they
            // are hashes — and the difference is by definition something normalisation was
            // supposed to cancel and did not.
            if let Ok(p) = std::env::var("SKRIB_COMPARE_DJOT")
                && let Ok(stored) = std::fs::read_to_string(&p)
            {
                let stored_plain = skrib_format::djot_plain_text(&stored)
                    .map(|(t, _)| t)
                    .unwrap_or_default();
                let a = skribisto_model::round_trip::normalize(&stored_plain);
                let b = skribisto_model::round_trip::normalize(&plain);
                println!(
                    "    stored digest {}",
                    skribisto_model::round_trip::digest(&stored_plain)
                );
                println!(
                    "    normalised lengths: stored {} / here {}",
                    a.chars().count(),
                    b.chars().count()
                );
                let ac: Vec<char> = a.chars().collect();
                let bc: Vec<char> = b.chars().collect();
                match ac.iter().zip(bc.iter()).position(|(x, y)| x != y) {
                    Some(i) => {
                        let lo = i.saturating_sub(60);
                        println!("    first difference at char {i}:");
                        println!(
                            "      stored …{}…",
                            ac[lo..(i + 60).min(ac.len())].iter().collect::<String>()
                        );
                        println!(
                            "      here   …{}…",
                            bc[lo..(i + 60).min(bc.len())].iter().collect::<String>()
                        );
                    }
                    None => println!("    one is a prefix of the other"),
                }
            }
        }
    }
    println!("  {marked} marked row(s), {same} digest-identical");

    assert!(
        !doc.blocks.is_empty(),
        "the scanner produced nothing from {}",
        path.display()
    );
}
