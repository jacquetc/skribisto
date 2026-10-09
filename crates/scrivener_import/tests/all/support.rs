// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What every test here does: build a fixture, write it somewhere, analyse it,
//! and look at one kind of result.

use std::path::{Path, PathBuf};

use scrivener_fixtures::Spec;
use scrivener_import::model::{BinderRow, ItemKey};
use scrivener_import::{
    Analysis, Answers, Notice, Options, Question, ReadProject, Refusal, analyze,
};

/// The host name the tests pretend to run on.
pub const HOST: &str = "Test-Mac.local";

pub fn options() -> Options {
    Options {
        this_host: Some(HOST.to_string()),
    }
}

pub fn temp() -> tempfile::TempDir {
    match tempfile::tempdir() {
        Ok(dir) => dir,
        Err(e) => panic!("a scratch folder: {e}"),
    }
}

/// Write `spec` as a project folder inside `parent`; its path.
pub fn write(spec: &Spec, parent: &Path) -> PathBuf {
    let folder = parent.join(spec.folder());
    if let Err(e) = spec.build().write_to(&folder) {
        panic!("writing the fixture: {e}");
    }
    folder
}

/// Write `files` as the folder `name` inside `parent`; its path.
pub fn write_files(files: &scrivener_fixtures::Files, parent: &Path, name: &str) -> PathBuf {
    let folder = parent.join(name);
    if let Err(e) = files.write_to(&folder) {
        panic!("writing the fixture: {e}");
    }
    folder
}

/// Write `bytes` to the file `name` inside `parent`; its path.
pub fn write_file(bytes: &[u8], parent: &Path, name: &str) -> PathBuf {
    let path = parent.join(name);
    if let Err(e) = std::fs::write(&path, bytes) {
        panic!("writing the fixture: {e}");
    }
    path
}

pub fn run(path: &Path) -> Analysis {
    analyze(path, &Answers::default(), &options())
}

pub fn run_with(path: &Path, answers: &Answers) -> Analysis {
    analyze(path, answers, &options())
}

/// The project read, or a failure naming what happened instead.
pub fn read(analysis: Analysis) -> ReadProject {
    match analysis {
        Analysis::Read(read) => *read,
        other => panic!("expected the project to be read, got {other:?}"),
    }
}

pub fn refusal(analysis: Analysis) -> Refusal {
    match analysis {
        Analysis::Refused { refusal, .. } => refusal,
        other => panic!("expected a refusal, got {}", summary(&other)),
    }
}

pub fn question(analysis: Analysis) -> Question {
    match analysis {
        Analysis::Choose { question, .. } => question,
        other => panic!("expected a question, got {}", summary(&other)),
    }
}

/// An analysis in a line: what it ended in and the keys of what it said.
pub fn summary(analysis: &Analysis) -> String {
    let keys: Vec<&str> = analysis.notices().iter().map(Notice::key).collect();
    match analysis {
        Analysis::Refused { refusal, .. } => format!("refused {refusal:?} {keys:?}"),
        Analysis::Choose { question, .. } => format!("asked {question:?} {keys:?}"),
        Analysis::Read(read) => format!("read, pending {:?}, {keys:?}", read.pending),
    }
}

/// Notices the fixture novel raises wherever it is read from: what stays in the Scrivener
/// project and that it states no language.
pub fn about_any_project(key: &str) -> bool {
    matches!(
        key,
        "scrivener-notice-snapshots-not-imported"
            | "scrivener-notice-writing-history-not-imported"
            | "scrivener-notice-language-not-stated"
    )
}

pub fn has(read: &ReadProject, key: &str) -> bool {
    read.notices.iter().any(|n| n.key() == key)
}

/// The row of the item titled `title`.
pub fn row<'a>(read: &'a ReadProject, title: &str) -> &'a BinderRow {
    match read
        .project
        .binder
        .iter()
        .find(|r| r.title.as_deref() == Some(title))
    {
        Some(row) => row,
        None => panic!("no item titled {title}"),
    }
}

/// The key of the item titled `title`.
pub fn key(read: &ReadProject, title: &str) -> ItemKey {
    match row(read, title).key {
        Some(key) => key,
        None => panic!("{title} has no key"),
    }
}

/// Run `work` on a thread with the stack a long operation gets.
pub fn on_long_operation_stack<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    let handle = std::thread::Builder::new().stack_size(2 << 20).spawn(work);
    match handle.map(|h| h.join()) {
        Ok(Ok(value)) => value,
        _ => panic!("the work did not finish on a 2 MiB stack"),
    }
}
