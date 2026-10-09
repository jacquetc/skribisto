// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reading real Scrivener projects: ones nobody on this side wrote.
//!
//! Every other test runs against a fixture built here, which proves the reader
//! agrees with this project's reading of the format. Only real projects can show
//! the reading itself is wrong.
//!
//! # Running it
//!
//! The corpus is not in the repository and never may be: most projects carry no
//! licence, and Literature & Latte's downloads are theirs. Rebuild it anywhere
//! outside a git work tree, then point the test at it:
//!
//! ```text
//! python3 scripts/scrivener_corpus/harvest.py --dest ~/scrivener-corpus
//! SKRIBISTO_SCRIVENER_CORPUS=~/scrivener-corpus \
//!     cargo test -p skribisto-scrivener-import --test all -- --ignored corpus --nocapture
//! ```
//!
//! `scripts/scrivener_corpus/corpus.lock.json` pins each source and lists, for
//! every project root in it, the picks a writer could make (the folder or the
//! archive, each manifest at its top, and the folder around the whole entry) with
//! the outcome expected (`import`, or `refuse`, `choose` or `accept` with a key). The expectations were not produced by this reader:
//! they come from an independent prototype, adjusted for the decisions taken
//! since (Scrivener 1 for Mac refused; damaged manifests refused until spare
//! binders are read). `SKRIBISTO_SCRIVENER_CORPUS_LOCK` points at another lock.
//!
//! A second test, `corpus_every_project_plans_within_the_invariants`, plans
//! every project the lock expects to import: the structure plan must pass its
//! invariants, survive JSON, and stay within them under every choice the review
//! offers. It prints the structure it inferred per project, as counts.
//!
//! # What it prints
//!
//! Counts, and for each disagreement its entry id and a hash of the root's path.
//! Never a title, a path or a word of prose: the projects are unlicensed, and a
//! test log is a place text leaks from. Never run it on public CI.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use scrivener_import::structure::{
    self, Adjustment, BooksChoice, ContainerRung, Destination, Shape, StructurePlan, TextKind,
    TrashChoice,
};
use scrivener_import::{Analysis, Answers, Options, ReadProject, analyze};

/// The projects the importer must hold up on: the largest, the deepest, the ones
/// with the most notes and the most sync debris.
const STRESS: &[&str] = &[
    "m2-buhler-dissertation",
    "m3-kebra-nagast",
    "w1-pith",
    "m2-nano2011-fr",
    "w1-devolution",
    "m2-monsters-conflicts",
    "m3-modern-retro",
    "ll-manual-source",
];

fn lock_path() -> PathBuf {
    match std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS_LOCK") {
        Some(path) => PathBuf::from(path),
        None => Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../scripts/scrivener_corpus/corpus.lock.json"),
    }
}

/// An outcome as the lock spells it.
fn outcome_of(analysis: &Analysis) -> (String, String) {
    match analysis {
        Analysis::Refused { refusal, .. } => ("refuse".to_string(), refusal.key().to_string()),
        Analysis::Choose { question, .. } => ("choose".to_string(), question.key().to_string()),
        Analysis::Read(read) => match read.pending.first() {
            Some(question) => ("accept".to_string(), question.key().to_string()),
            None => ("import".to_string(), String::new()),
        },
    }
}

/// A short hash of a path, so a disagreement can be found locally without the
/// log naming it.
fn hash_of(text: &str) -> String {
    blake3::hash(text.as_bytes()).to_hex()[..12].to_string()
}

#[test]
#[ignore = "needs SKRIBISTO_SCRIVENER_CORPUS, a harvest of scripts/scrivener_corpus/corpus.lock.json"]
fn corpus_every_pinned_root_reaches_its_expected_outcome() {
    let Some(corpus) = std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS").map(PathBuf::from) else {
        eprintln!(
            "SKRIBISTO_SCRIVENER_CORPUS unset: skipping the real-project run. \
             Harvest the corpus with scripts/scrivener_corpus/harvest.py to run it."
        );
        return;
    };
    let lock_text = match std::fs::read_to_string(lock_path()) {
        Ok(text) => text,
        Err(e) => panic!("reading the corpus lock: {e}"),
    };
    let lock: serde_json::Value = match serde_json::from_str(&lock_text) {
        Ok(lock) => lock,
        Err(e) => panic!("the corpus lock is not JSON: {e}"),
    };
    let entries = lock["entries"].as_array().cloned().unwrap_or_default();
    let options = Options { this_host: None };

    let mut roots = 0usize;
    let mut agreed = 0usize;
    let mut absent: Vec<String> = Vec::new();
    let mut by_expected: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut disagreements: BTreeMap<(String, String), Vec<String>> = BTreeMap::new();
    let mut stress_seen: BTreeMap<&str, bool> = STRESS.iter().map(|s| (*s, false)).collect();
    let mut notices: BTreeMap<String, usize> = BTreeMap::new();
    let mut slowest = (0.0f64, String::new());
    // One line per root, counts only, for comparing two runs.
    let mut report: Vec<serde_json::Value> = Vec::new();
    let started = Instant::now();

    for entry in &entries {
        let id = entry["id"].as_str().unwrap_or_default().to_string();
        for root in entry["roots"].as_array().cloned().unwrap_or_default() {
            let rel = root["path"].as_str().unwrap_or_default();
            let pick = root["pick"].as_str().unwrap_or("folder").to_string();
            let expected = (
                root["expect"].as_str().unwrap_or_default().to_string(),
                root["key"].as_str().unwrap_or_default().to_string(),
            );
            let path = corpus.join(rel);
            if !path.exists() {
                absent.push(format!("{id} #{}", hash_of(rel)));
                continue;
            }
            roots += 1;
            if let Some(seen) = stress_seen.get_mut(id.as_str()) {
                *seen = true;
            }
            let t = Instant::now();
            let analysis =
                std::panic::catch_unwind(|| analyze(&path, &Answers::default(), &options));
            let seconds = t.elapsed().as_secs_f64();
            if seconds > slowest.0 {
                slowest = (seconds, format!("{id} #{}", hash_of(rel)));
            }
            let got = match &analysis {
                Ok(analysis) => {
                    let mut keys: BTreeMap<String, usize> = BTreeMap::new();
                    for notice in analysis.notices() {
                        *notices.entry(notice.key().to_string()).or_default() += 1;
                        *keys.entry(notice.key().to_string()).or_default() += 1;
                    }
                    let (outcome, key) = outcome_of(analysis);
                    report.push(serde_json::json!({
                        "entry": id,
                        "pick": pick,
                        "root": hash_of(rel),
                        "outcome": outcome,
                        "key": key,
                        "notices": keys,
                        "seconds": seconds,
                    }));
                    if let Analysis::Read(read) = analysis {
                        // The model must survive the long operation's JSON, and be a tree.
                        assert!(
                            read.project.check_shape().is_ok(),
                            "{id} #{}: model not a tree",
                            hash_of(rel)
                        );
                        let json = serde_json::to_string(analysis).unwrap_or_default();
                        let back: Result<Analysis, _> = serde_json::from_str(&json);
                        assert!(
                            back.as_ref().is_ok_and(|b| b == analysis),
                            "{id} #{}: the analysis does not survive a JSON round trip",
                            hash_of(rel)
                        );
                    }
                    outcome_of(analysis)
                }
                Err(_) => ("panic".to_string(), String::new()),
            };
            let tally = by_expected
                .entry(format!("{pick:<8} {} {}", expected.0, expected.1))
                .or_default();
            tally.0 += 1;
            if got == expected {
                agreed += 1;
                tally.1 += 1;
            } else {
                disagreements
                    .entry((
                        format!("{pick} {} {}", expected.0, expected.1),
                        format!("{} {}", got.0, got.1),
                    ))
                    .or_default()
                    .push(format!("{id} #{}", hash_of(rel)));
            }
        }
    }

    eprintln!(
        "corpus: {} entries, {roots} roots read, {agreed} agree with the lock",
        entries.len()
    );
    eprintln!(
        "corpus: {:.1} s in all, slowest root {:.2} s ({})",
        started.elapsed().as_secs_f64(),
        slowest.0,
        slowest.1
    );
    for (expected, (count, ok)) in &by_expected {
        eprintln!("  expected {expected:<60} {ok:>3} of {count:>3}");
    }
    if !absent.is_empty() {
        eprintln!(
            "corpus: {} roots not in this harvest: {}",
            absent.len(),
            absent.join(", ")
        );
    }
    eprintln!(
        "corpus: stress projects present: {}",
        stress_seen
            .iter()
            .map(|(id, seen)| format!("{id}={}", if *seen { "yes" } else { "no" }))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let report_path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("scrivener-corpus-report.json");
    match serde_json::to_string_pretty(&report) {
        Ok(json) if std::fs::write(&report_path, &json).is_ok() => {
            eprintln!(
                "corpus: counts per root written to {}",
                report_path.display()
            );
        }
        _ => eprintln!("corpus: the per-root report could not be written"),
    }
    eprintln!("corpus: notices raised (key: how many):");
    for (key, count) in &notices {
        eprintln!("  {key:<60} {count}");
    }
    for ((expected, got), roots) in &disagreements {
        eprintln!(
            "DISAGREE expected [{expected}] got [{got}]: {}",
            roots.join(", ")
        );
    }
    // A corpus variable pointing at the wrong folder reads nothing, and nothing
    // read would disagree with anything: that run proves nothing and must say so.
    assert!(
        roots > 0,
        "SKRIBISTO_SCRIVENER_CORPUS holds none of the {} pinned roots",
        absent.len()
    );
    assert!(
        disagreements.is_empty(),
        "{} roots disagree with the lock",
        roots - agreed
    );
}

/// Every choice the review offers in `plan`, each control set away from its
/// default.
fn every_offered_choice(plan: &StructurePlan) -> Vec<Adjustment> {
    let mut out = Vec::new();
    for c in &plan.review.containers {
        for rung in [
            ContainerRung::Part,
            ContainerRung::Chapter,
            ContainerRung::Plain,
        ] {
            out.push(Adjustment::Containers {
                depth: c.depth,
                rung: Some(rung),
            });
        }
    }
    for t in &plan.review.texts {
        out.push(Adjustment::Texts {
            depth: t.depth,
            kind: Some(TextKind::Chapters),
        });
    }
    for r in &plan.review.roots {
        for destination in [
            Destination::StoryBible,
            Destination::Manuscript,
            Destination::LeaveOut,
        ] {
            out.push(Adjustment::Root {
                item: r.item,
                destination: Some(destination),
            });
        }
    }
    if plan.review.books.is_some() {
        out.push(Adjustment::Books(Some(BooksChoice::One)));
        out.push(Adjustment::Books(Some(BooksChoice::Several)));
    }
    for s in &plan.review.splits {
        out.push(Adjustment::Split {
            item: s.item,
            split: Some(false),
        });
    }
    if plan.review.trash.is_some() {
        out.push(Adjustment::Trash(TrashChoice::LeaveOut));
    }
    out
}

/// What kind of book the plan made: the review's first question.
fn shape_class(plan: &StructurePlan) -> &'static str {
    let s = &plan.shape;
    if s.books > 1 {
        "several books"
    } else if s.parts > 0 && s.chapter_folders + s.flat_chapters > 0 {
        "parts and chapters"
    } else if s.parts > 0 {
        "parts of scenes"
    } else if s.chapter_folders + s.flat_chapters > 0 {
        "chapters"
    } else {
        "scenes only"
    }
}

#[test]
#[ignore = "needs SKRIBISTO_SCRIVENER_CORPUS, a harvest of scripts/scrivener_corpus/corpus.lock.json"]
fn corpus_every_project_plans_within_the_invariants() {
    let Some(corpus) = std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS").map(PathBuf::from) else {
        eprintln!("SKRIBISTO_SCRIVENER_CORPUS unset: skipping the real-project structure run.");
        return;
    };
    let lock_text = match std::fs::read_to_string(lock_path()) {
        Ok(text) => text,
        Err(e) => panic!("reading the corpus lock: {e}"),
    };
    let lock: serde_json::Value = match serde_json::from_str(&lock_text) {
        Ok(lock) => lock,
        Err(e) => panic!("the corpus lock is not JSON: {e}"),
    };
    let entries = lock["entries"].as_array().cloned().unwrap_or_default();
    let options = Options { this_host: None };

    let mut planned = 0usize;
    let mut adjusted = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut totals = Shape::default();
    let mut classes: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut notices: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut review: BTreeMap<String, usize> = BTreeMap::new();
    let mut lines: Vec<String> = Vec::new();
    let mut report: Vec<serde_json::Value> = Vec::new();
    let mut facts = MetadataFacts::default();
    let started = Instant::now();

    // Folders a writer imports whole: a manifest inside one reads the same binder.
    let imported_folders: Vec<String> = entries
        .iter()
        .flat_map(|e| e["roots"].as_array().cloned().unwrap_or_default())
        .filter(|r| r["pick"].as_str() == Some("folder") && r["expect"].as_str() == Some("import"))
        .filter_map(|r| r["path"].as_str().map(str::to_string))
        .collect();
    for entry in &entries {
        let id = entry["id"].as_str().unwrap_or_default().to_string();
        let roots = entry["roots"].as_array().cloned().unwrap_or_default();
        for (k, root) in roots.iter().enumerate() {
            // One pick per binder: the folder or the archive a writer would
            // choose, and a manifest only where its folder holds several (the
            // folder pick then asks which, as the dissertation's does).
            let pick = root["pick"].as_str().unwrap_or_default();
            let rel = root["path"].as_str().unwrap_or_default();
            let folder_of = rel.rsplit_once('/').map_or("", |(folder, _)| folder);
            let wanted = match pick {
                "folder" | "archive" => true,
                "manifest" => !imported_folders.iter().any(|f| f == folder_of),
                _ => false,
            };
            if !wanted || root["expect"].as_str() != Some("import") {
                continue;
            }
            let path = corpus.join(rel);
            if !path.exists() {
                continue;
            }
            let label = format!("{id}#{k}");
            let analysis =
                std::panic::catch_unwind(|| analyze(&path, &Answers::default(), &options));
            let read = match analysis {
                Ok(Analysis::Read(read)) => read,
                Ok(other) => {
                    let (outcome, key) = outcome_of(&other);
                    failures.push(format!("{label}: {outcome} {key}"));
                    continue;
                }
                Err(_) => {
                    failures.push(format!("{label}: panic"));
                    continue;
                }
            };
            planned += 1;
            facts.add(&read);
            let plan = &read.structure;
            if let Err(v) = structure::check(&read.project, &read.content, plan) {
                failures.push(format!("{label}: {v:?}"));
            }
            let json = serde_json::to_string(&*read).unwrap_or_default();
            let back: Result<ReadProject, _> = serde_json::from_str(&json);
            if !back.as_ref().is_ok_and(|b| b == &*read) {
                failures.push(format!("{label}: the plan does not survive JSON"));
            }
            for adjustment in every_offered_choice(plan) {
                match structure::adjust(&read.project, &read.content, plan, &adjustment) {
                    Ok(_) => adjusted += 1,
                    Err(e) => failures.push(format!("{label}: {adjustment:?} gives {e}")),
                }
            }
            let s = &plan.shape;
            for (total, value) in [
                (&mut totals.books, s.books),
                (&mut totals.parts, s.parts),
                (&mut totals.chapter_folders, s.chapter_folders),
                (&mut totals.flat_chapters, s.flat_chapters),
                (&mut totals.scenes, s.scenes),
                (&mut totals.paratexts, s.paratexts),
                (&mut totals.plain_folders, s.plain_folders),
                (&mut totals.manuscript_notes, s.manuscript_notes),
                (&mut totals.notes_rows, s.notes_rows),
                (&mut totals.trashed_rows, s.trashed_rows),
                (&mut totals.templates, s.templates),
                (&mut totals.tags, s.tags),
            ] {
                *total += value;
            }
            *classes.entry(shape_class(plan)).or_default() += 1;
            let mut keys: BTreeMap<&'static str, usize> = BTreeMap::new();
            for notice in &plan.notices {
                *notices.entry(notice.key()).or_default() += 1;
                *keys.entry(notice.key()).or_default() += 1;
            }
            let r = &plan.review;
            let mut seen = |what: String| *review.entry(what).or_default() += 1;
            for c in &r.containers {
                seen(format!(
                    "depth {} containers, detected {:?}{}",
                    c.depth,
                    c.detected,
                    if c.mixed { ", mixed" } else { "" }
                ));
                for flag in &c.flags {
                    seen(format!("depth {} flagged {flag:?}", c.depth));
                }
            }
            if !r.texts.is_empty() {
                seen("unclaimed texts offered as chapters".to_string());
            }
            if !r.roots.is_empty() {
                seen("root folders to place".to_string());
            }
            if r.roots.iter().any(|x| x.looks_like_manuscript) {
                seen("a root folder that looks like manuscript".to_string());
            }
            if r.books.is_some() {
                seen("book-claimed Draft folders".to_string());
            }
            if !r.splits.is_empty() {
                seen("a one-document manuscript".to_string());
            }
            if r.trash.is_some() {
                seen("a Trash to bring across".to_string());
            }
            lines.push(format!(
                "  {label:<34} {:<15} {:<18} bk {} pt {} chf {} chl {} sc {} pa {} pl {} msn {} nb {} tr {} tpl {} tag {}",
                format!("{:?}", read.project.source.generation),
                shape_class(plan),
                s.books,
                s.parts,
                s.chapter_folders,
                s.flat_chapters,
                s.scenes,
                s.paratexts,
                s.plain_folders,
                s.manuscript_notes,
                s.notes_rows,
                s.trashed_rows,
                s.templates,
                s.tags
            ));
            report.push(serde_json::json!({
                "entry": id,
                "root": k,
                "path_hash": hash_of(rel),
                "generation": format!("{:?}", read.project.source.generation),
                "items": read.project.binder.len(),
                "shape": s,
                "class": shape_class(plan),
                "chapter_encoding": format!("{:?}", plan.chapter_encoding),
                "containers": r.containers.iter().map(|c| serde_json::json!({
                    "depth": c.depth,
                    "containers": c.containers,
                    "adjustable": c.adjustable,
                    "detected": format!("{:?}", c.detected),
                    "mixed": c.mixed,
                    "flags": c.flags.iter().map(|f| format!("{f:?}")).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
                "texts": r.texts.iter().map(|t| (t.depth, t.texts, t.median_words)).collect::<Vec<_>>(),
                "roots_offered": r.roots.len(),
                "splits": r.splits.iter().map(|s| s.headings).collect::<Vec<_>>(),
                "books_claimed": r.books.as_ref().map_or(0, |b| b.claimed.len()),
                "trash": r.trash.as_ref().map(|t| (t.items, t.with_writing)),
                "set_aside": (r.set_aside.trashed, r.set_aside.recovered),
                "notices": keys,
            }));
        }
    }

    eprintln!(
        "structure: {planned} projects planned in {:.1} s; invariants held on {} of {planned}; \
         {adjusted} re-planned with a choice changed, all within the invariants: {}",
        started.elapsed().as_secs_f64(),
        planned - failures.iter().filter(|f| !f.contains(" gives ")).count(),
        failures.iter().all(|f| !f.contains(" gives "))
    );
    eprintln!("structure: per project (entry#root, counts only):");
    for line in &lines {
        eprintln!("{line}");
    }
    eprintln!(
        "structure: totals: books {} parts {} chapter folders {} flat chapters {} scenes {} \
         paratexts {} plain folders {} manuscript notes {} notes rows {} trashed rows {} \
         templates {} story-bible tags {}",
        totals.books,
        totals.parts,
        totals.chapter_folders,
        totals.flat_chapters,
        totals.scenes,
        totals.paratexts,
        totals.plain_folders,
        totals.manuscript_notes,
        totals.notes_rows,
        totals.trashed_rows,
        totals.templates,
        totals.tags
    );
    eprintln!("structure: shapes:");
    for (class, count) in &classes {
        eprintln!("  {class:<20} {count}");
    }
    eprintln!("structure: what the review shows (projects):");
    for (what, count) in &review {
        eprintln!("  {what:<60} {count}");
    }
    eprintln!("structure: notices (key: how many):");
    for (key, count) in &notices {
        eprintln!("  {key:<55} {count}");
    }
    let report_path =
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("scrivener-structure-report.json");
    match serde_json::to_string_pretty(&report) {
        Ok(json) if std::fs::write(&report_path, &json).is_ok() => {
            eprintln!(
                "structure: per-project report written to {}",
                report_path.display()
            );
        }
        _ => eprintln!("structure: the per-project report could not be written"),
    }
    facts.print();
    for failure in &failures {
        eprintln!("FAIL {failure}");
    }
    assert!(
        planned > 0,
        "SKRIBISTO_SCRIVENER_CORPUS holds no pinned project"
    );
    assert!(failures.is_empty(), "{} failures", failures.len());
}

/// What the projects state about themselves, counted over one read of each binder: never
/// a title, a name or a date, only how many and from where.
#[derive(Default)]
struct MetadataFacts {
    items: u64,
    dates_read: u64,
    dates_refused: u64,
    dates_missing: u64,
    snapshots: u64,
    snapshot_projects: u64,
    of_deleted: u64,
    history_projects: u64,
    history_days: u64,
    history_unreadable: u64,
    titles: BTreeMap<String, u64>,
    authors: BTreeMap<String, u64>,
    languages: BTreeMap<String, u64>,
    struck_compiles: u64,
    formats_leaving_styles_out: u64,
    notices: BTreeMap<&'static str, u64>,
}

impl MetadataFacts {
    fn add(&mut self, read: &ReadProject) {
        use scrivener_import::Notice;
        let project = &read.project;
        for row in &project.binder {
            self.items += 1;
            for date in [&row.created, &row.modified] {
                match date {
                    Some(stamp) if stamp.unix_seconds.is_some() => self.dates_read += 1,
                    Some(_) => self.dates_refused += 1,
                    None => self.dates_missing += 1,
                }
            }
        }
        let kept = project.counts.snapshots + project.counts.orphan_snapshots;
        if kept > 0 {
            self.snapshot_projects += 1;
            self.snapshots += u64::from(kept);
            self.of_deleted += u64::from(project.counts.orphan_snapshots);
        }
        for notice in &read.notices {
            if let Notice::WritingHistoryNotImported {
                days, unreadable, ..
            } = notice
            {
                self.history_projects += 1;
                self.history_days += u64::from(*days);
                self.history_unreadable += u64::from(*unreadable);
            }
            let key = notice.key();
            if [
                "scrivener-notice-title-placeholder-rejected",
                "scrivener-notice-author-placeholder-rejected",
                "scrivener-notice-title-from-project",
                "scrivener-notice-author-from-project",
                "scrivener-notice-language-not-stated",
                "scrivener-notice-language-from-script",
                "scrivener-notice-snapshots-not-imported",
                "scrivener-notice-writing-history-not-imported",
            ]
            .contains(&key)
            {
                *self.notices.entry(key).or_default() += 1;
            }
        }
        let proposals = &read.proposals;
        *self
            .titles
            .entry(format!("{:?}", proposals.title.source))
            .or_default() += 1;
        *self
            .authors
            .entry(
                proposals
                    .author
                    .as_ref()
                    .map_or("none".to_string(), |a| format!("{:?}", a.source)),
            )
            .or_default() += 1;
        *self
            .languages
            .entry(
                proposals
                    .language
                    .as_ref()
                    .map_or("none".to_string(), |l| format!("{:?} {}", l.source, l.tag)),
            )
            .or_default() += 1;
        if project.compile.delete_struck_through {
            self.struck_compiles += 1;
        }
        if project
            .compile
            .formats
            .iter()
            .any(|f| !f.deleted_styles.is_empty() || !f.raw_styles.is_empty())
        {
            self.formats_leaving_styles_out += 1;
        }
    }

    fn print(&self) {
        eprintln!(
            "metadata: {} items; dates read {}, refused {}, absent {}",
            self.items, self.dates_read, self.dates_refused, self.dates_missing
        );
        eprintln!(
            "metadata: snapshots kept in Scrivener {} in {} projects ({} of deleted documents); \
             writing history {} days in {} projects ({} entries unreadable)",
            self.snapshots,
            self.snapshot_projects,
            self.of_deleted,
            self.history_days,
            self.history_projects,
            self.history_unreadable
        );
        eprintln!("metadata: title from {:?}", self.titles);
        eprintln!("metadata: authors from {:?}", self.authors);
        eprintln!("metadata: language {:?}", self.languages);
        eprintln!(
            "metadata: compiles removing struck text {}; projects whose compile formats \
             leave styles out {}",
            self.struck_compiles, self.formats_leaving_styles_out
        );
        for (key, count) in &self.notices {
            eprintln!("  {key:<55} {count}");
        }
    }
}

/// The peak memory this process has held since the last [`reset_peak`], in megabytes, where
/// the system says (Linux).
fn peak_megabytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find(|l| l.starts_with("VmHWM:"))?;
    let kilobytes: u64 = line.split_whitespace().nth(1)?.parse().ok()?;
    Some(kilobytes / 1024)
}

/// Start measuring the peak again (Linux: `5` to `clear_refs` resets the high-water mark).
fn reset_peak() {
    let _ = std::fs::write("/proc/self/clear_refs", "5");
}

/// What the editor reads a Djot text as, or `None` when it cannot.
fn editor_text(djot: &str) -> Option<String> {
    let doc = text_document::TextDocument::new();
    doc.set_djot_sync(djot).ok()?;
    doc.to_addressable_text().ok()
}

/// Every importable project of the corpus converted to a `.skrib`, read back, turned into
/// the graph `load_work` builds, and counted. With `SKRIBISTO_SCRIVENER_CORPUS_EDITOR=1`,
/// every stored text also goes through the editor's save and must read back the same.
///
/// Prints counts per project and, for the stress projects, the time and the peak memory
/// the conversion took. Never a title, a path or a word of prose.
#[test]
#[ignore = "needs SKRIBISTO_SCRIVENER_CORPUS, a harvest of scripts/scrivener_corpus/corpus.lock.json"]
fn corpus_every_project_converts_and_loads_back() {
    use scrivener_import::structure::Choices;
    use scrivener_import::{ConvertError, ProseChoices, Request, TITLE_SLOT, Words, convert};
    use std::sync::atomic::AtomicBool;

    let Some(corpus) = std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS").map(PathBuf::from) else {
        eprintln!("SKRIBISTO_SCRIVENER_CORPUS unset: skipping the real-project conversion.");
        return;
    };
    let editor = std::env::var_os("SKRIBISTO_SCRIVENER_CORPUS_EDITOR").is_some();
    // Notice keys to report per project, for looking into one kind: comma-separated.
    let watch: Vec<String> = std::env::var("SKRIBISTO_SCRIVENER_CORPUS_WATCH")
        .map(|w| w.split(',').map(str::to_string).collect())
        .unwrap_or_default();
    let lock_text = match std::fs::read_to_string(lock_path()) {
        Ok(text) => text,
        Err(e) => panic!("reading the corpus lock: {e}"),
    };
    let lock: serde_json::Value = match serde_json::from_str(&lock_text) {
        Ok(lock) => lock,
        Err(e) => panic!("the corpus lock is not JSON: {e}"),
    };
    let entries = lock["entries"].as_array().cloned().unwrap_or_default();
    let options = Options { this_host: None };
    let words = Words {
        manuscript_binder: "Manuscript".to_string(),
        notes_binder: "Notes".to_string(),
        notes_of: format!("{TITLE_SLOT} (notes)"),
        project_notes: "Project notes".to_string(),
        recovered: "Recovered".to_string(),
        untitled_template: "Template".to_string(),
        research_folder: format!("{TITLE_SLOT} research files"),
        list_separator: ", ".to_string(),
        label_details: format!("{TITLE_SLOT} in Scrivener"),
        status_details: format!("{TITLE_SLOT} in Scrivener"),
        unnamed_label: format!("Label {TITLE_SLOT}"),
        unnamed_status: format!("Status {TITLE_SLOT}"),
        label_tag: format!("{TITLE_SLOT} (label)"),
        untitled_tag: "Untitled".to_string(),
        default_statuses: vec![
            scrivener_import::DefaultRung {
                name: "Idea".to_string(),
                category: common::entities::StatusCategory::Planned,
            },
            scrivener_import::DefaultRung {
                name: "Draft".to_string(),
                category: common::entities::StatusCategory::Drafting,
            },
            scrivener_import::DefaultRung {
                name: "Done".to_string(),
                category: common::entities::StatusCategory::Final,
            },
        ],
    };
    let scratch = match tempfile::tempdir_in(env!("CARGO_TARGET_TMPDIR")) {
        Ok(dir) => dir,
        Err(e) => panic!("a scratch folder: {e}"),
    };

    let imported_folders: Vec<String> = entries
        .iter()
        .flat_map(|e| e["roots"].as_array().cloned().unwrap_or_default())
        .filter(|r| r["pick"].as_str() == Some("folder") && r["expect"].as_str() == Some("import"))
        .filter_map(|r| r["path"].as_str().map(str::to_string))
        .collect();
    let mut converted = 0usize;
    let mut failures: Vec<String> = Vec::new();
    let mut editor_changes: Vec<String> = Vec::new();
    let mut notices: BTreeMap<String, usize> = BTreeMap::new();
    let mut metadata_notices: BTreeMap<String, usize> = BTreeMap::new();
    let (mut labelled_rows, mut rung_rows, mut label_tags, mut rungs) = (0u64, 0u64, 0u64, 0u64);
    let mut categories: BTreeMap<String, u64> = BTreeMap::new();
    let (mut projects_with_labels, mut projects_with_rungs_worn) = (0u64, 0u64);
    let (mut rows, mut words_total, mut footnotes, mut comments, mut pictures) =
        (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut texts, mut orphans) = (0u64, 0u64);
    let started = Instant::now();

    for entry in &entries {
        let id = entry["id"].as_str().unwrap_or_default().to_string();
        let roots = entry["roots"].as_array().cloned().unwrap_or_default();
        for (k, root) in roots.iter().enumerate() {
            let pick = root["pick"].as_str().unwrap_or_default();
            let rel = root["path"].as_str().unwrap_or_default();
            let folder_of = rel.rsplit_once('/').map_or("", |(folder, _)| folder);
            let wanted = match pick {
                "folder" | "archive" => true,
                "manifest" => !imported_folders.iter().any(|f| f == folder_of),
                _ => false,
            };
            if !wanted || root["expect"].as_str() != Some("import") {
                continue;
            }
            let path = corpus.join(rel);
            if !path.exists() {
                continue;
            }
            let label = format!("{id}#{k}");
            let out = scratch.path().join(format!("{converted}.skrib"));
            let output = out.to_string_lossy().into_owned();
            let choices = Choices::default();
            let answers = Answers::default();
            let request = Request {
                source: &path,
                answers: &answers,
                options: &options,
                choices: &choices,
                prose: ProseChoices::default(),
                fingerprint: None,
                output: &output,
                overwrite: true,
                words: &words,
                title: None,
                author: None,
                language: None,
                copy_research_files: true,
                now: chrono::Utc::now(),
            };
            reset_peak();
            let t = Instant::now();
            let result =
                std::panic::catch_unwind(|| convert(&request, &|_, _| {}, &AtomicBool::new(false)));
            let seconds = t.elapsed().as_secs_f64();
            let peak = peak_megabytes();
            let summary = match result {
                Ok(Ok(summary)) => summary,
                Ok(Err(ConvertError::NotAccepted(_))) => continue,
                Ok(Err(e)) => {
                    failures.push(format!("{label}: {e}"));
                    continue;
                }
                Err(_) => {
                    failures.push(format!("{label}: panic"));
                    continue;
                }
            };
            converted += 1;
            let bundle = match skrib_format::read_bundle(&output) {
                Ok(bundle) => bundle,
                Err(e) => {
                    failures.push(format!("{label}: does not read back: {e:#}"));
                    continue;
                }
            };
            let mut project_texts = 0u64;
            let mut project_orphans = 0u64;
            let labels: std::collections::BTreeSet<u64> = bundle
                .tags
                .iter()
                .filter(|t| !t.discoverable)
                .map(|t| t.file_id)
                .collect();
            label_tags += labels.len() as u64;
            rungs += bundle.statuses.len() as u64;
            for rung in &bundle.statuses {
                *categories
                    .entry(format!("{:?}", rung.category))
                    .or_default() += 1;
            }
            let (mut labelled, mut worn) = (0u64, 0u64);
            for item in bundle.binders.iter().flat_map(|b| b.items.iter()) {
                if item.item.tag_ids.iter().any(|id| labels.contains(id)) {
                    labelled += 1;
                }
                if item.item.status_id.is_some() {
                    worn += 1;
                }
            }
            labelled_rows += labelled;
            rung_rows += worn;
            projects_with_labels += u64::from(labelled > 0);
            projects_with_rungs_worn += u64::from(worn > 0);
            for item in bundle.binders.iter().flat_map(|b| b.items.iter()) {
                project_texts += item.prose.len() as u64;
                project_orphans += item
                    .comments
                    .values()
                    .flatten()
                    .filter(|c| c.orphaned)
                    .count() as u64;
                if editor {
                    for (content, djot) in &item.prose {
                        let before = editor_text(djot);
                        let after = before.as_ref().and_then(|_| {
                            let doc = text_document::TextDocument::new();
                            doc.set_djot_sync(djot).ok()?;
                            editor_text(&doc.to_djot().ok()?)
                        });
                        if before.is_none() || before != after {
                            editor_changes.push(format!(
                                "{label}: text {content} ({})",
                                &blake3::hash(djot.as_bytes()).to_hex()[..12]
                            ));
                        }
                    }
                }
            }
            if let Err(e) = skrib_format::bundle_to_loaded(bundle, &output) {
                failures.push(format!("{label}: does not load: {e:#}"));
            }
            let _ = std::fs::remove_file(&out);
            if let Some(folder) = &summary.research_folder {
                let _ = std::fs::remove_dir_all(scratch.path().join(folder));
            }
            for notice in &summary.metadata {
                *metadata_notices
                    .entry(notice.key().to_string())
                    .or_default() += 1;
            }
            let mut watched: BTreeMap<&str, usize> = BTreeMap::new();
            for notice in &summary.prose {
                *notices.entry(notice.key().to_string()).or_default() += 1;
                if watch.iter().any(|w| w == notice.key()) {
                    *watched.entry(notice.key()).or_default() += 1;
                }
            }
            if !watched.is_empty() {
                eprintln!("  WATCH {label}: {watched:?}");
            }
            let totals = summary.totals;
            rows += u64::from(totals.rows);
            words_total += totals.words;
            footnotes += u64::from(totals.footnotes);
            comments += u64::from(totals.comments);
            pictures += u64::from(totals.pictures);
            texts += project_texts;
            orphans += project_orphans;
            let stress = STRESS.contains(&id.as_str());
            let line = format!(
                "  {label:<34} rows {:>5} words {:>8} notes {:>4} comments {:>4} (orphaned {}) pictures {:>3} {:>6.2} s{}{}",
                totals.rows,
                totals.words,
                totals.footnotes,
                totals.comments,
                project_orphans,
                totals.pictures,
                seconds,
                peak.map_or(String::new(), |mb| format!(" peak {mb} MB")),
                if stress { " [stress]" } else { "" }
            );
            // As it goes: a debug run of the whole corpus takes minutes.
            eprintln!("{line}");
        }
    }

    eprintln!(
        "convert: {converted} projects converted in {:.1} s; {} failed",
        started.elapsed().as_secs_f64(),
        failures.len()
    );
    eprintln!(
        "convert: totals: rows {rows} manuscript words {words_total} texts {texts} footnotes \
         {footnotes} comments {comments} (orphaned {orphans}) pictures {pictures}"
    );
    eprintln!("convert: prose notices (key: how many):");
    for (key, count) in &notices {
        eprintln!("  {key:<60} {count}");
    }
    eprintln!(
        "convert: label tags {label_tags}, rows wearing one {labelled_rows} (in \
         {projects_with_labels} projects); rungs {rungs}, rows wearing one {rung_rows} (in \
         {projects_with_rungs_worn} projects)"
    );
    eprintln!("convert: rungs by kind {categories:?}");
    eprintln!("convert: metadata notices (key: how many):");
    for (key, count) in &metadata_notices {
        eprintln!("  {key:<60} {count}");
    }
    if editor {
        eprintln!(
            "convert: texts the editor's save changes: {}",
            editor_changes.len()
        );
        for change in editor_changes.iter().take(50) {
            eprintln!("  EDITOR {change}");
        }
    }
    for failure in &failures {
        eprintln!("FAIL {failure}");
    }
    assert!(
        converted > 0,
        "SKRIBISTO_SCRIVENER_CORPUS holds no pinned project"
    );
    assert!(failures.is_empty(), "{} failures", failures.len());
}
