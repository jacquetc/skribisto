// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! What to tell the writer when a project would not open.
//!
//! Not a view-model — a pure formatter, like its neighbours `long_op` and
//! `save_status`. It lives here because both callers of `load_work` need the identical
//! answer and neither owns the other: [`crate::view_models::ProjectSwitchViewModel`]
//! (File ▸ Open Work, Ctrl+O, the switcher's "Open here", the import toast's "Open now")
//! and `app.rs`'s argv-launch path.
//!
//! # Two things this fixes
//!
//! **The chain must not be thrown away.** `load_work`'s failure travels through three
//! `anyhow` context layers — `skrib_format` states the real cause, `load_work_uc` adds
//! `reading project '…'`, and the frontend command adds `load_work`. `anyhow`'s plain
//! `Display` prints *only the outermost* frame, so `e.to_string()` at the leaf would
//! render the entire toast as the literal words **"Could not open work: load_work"** —
//! naming neither the file nor the cause. `{e:#}` prints the whole chain on one line
//! instead, covering every open failure (corrupt file, missing blob, legacy too old,
//! not a `.skrib`).
//!
//! `{:#}` rather than `{:?}`: `anyhow`'s `Debug` is a multi-line report that *includes a
//! backtrace* when `RUST_BACKTRACE` is set, which is not a toast. The alternate `Display`
//! is the same chain joined by `": "` on one line.
//!
//! **"From the future" deserves its own sentence.** A file this build genuinely cannot
//! open is a recoverable condition with one specific instruction — update Skribisto —
//! and the reader can only act on it if told. It is the one failure worth pulling out of
//! the generic chain, which is why `skrib_format` types it
//! ([`SkribFormatError::TooNew`]) instead of leaving the UI to pattern-match prose.

use skrib_format::SkribFormatError;
use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::Toast;

/// The toast for a failed `load_work` on `path`.
///
/// # Shape: short title, detail in the body
///
/// A toast's title is **one line and it truncates**. Verified live: the first cut of this
/// put the absolute path first, and the whole toast rendered as
/// `"/tmp/claude-1000/-home-cyril-Devel-skribi…"` — every word that told the writer what
/// had happened, or what to do about it, fell off the end. So the title states the
/// problem in a few words and names the file by its **base name** (a `.skrib` lives at a
/// path nobody memorises; the file name is what identifies it on screen), and everything
/// longer goes in the body.
///
/// The generic body is the error chain itself, which is **data** — paths, RON parser
/// output, OS messages — so it is `lit!`, never `tr!`, by the same house rule that keeps
/// entity titles untranslated.
///
/// `downcast_ref` sees through the `.context(…)` layers stacked on top of the format
/// crate's error, so this works unchanged wherever in the call stack the error is caught.
pub fn open_failure_toast(path: &str, error: &anyhow::Error) -> Toast {
    let (title, body) = open_failure_parts(path, error);
    Toast::error(title).body(body)
}

/// The decision, separated from the presentation. `Toast`'s fields are `pub(crate)` to
/// teksilo, so the tests below would otherwise have to assert against its `Debug` output.
fn open_failure_parts(path: &str, error: &anyhow::Error) -> (LocalizedString, LocalizedString) {
    let file = display_name(path);
    match error.downcast_ref::<SkribFormatError>() {
        Some(SkribFormatError::TooNew {
            written_by,
            requires_at_least,
            supported,
        }) => (
            tr!(could_not_open_work_too_new(file = file)),
            tr!(could_not_open_work_too_new_detail(
                written_by = *written_by as i64,
                requires = *requires_at_least as i64,
                supported = *supported as i64,
            )),
        ),
        // Everything else already says something specific and true — it just needed the
        // chain kept intact on the way here.
        _ => (
            tr!(could_not_open_work(file = file)),
            lit!(format!("{error:#}")),
        ),
    }
}

/// The file name to show for `path` — its last component, or the whole thing if it has
/// none (a bare relative name, or a path ending in `..`).
fn display_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path)
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use teksilo::i18n::config::I18nConfig;
    use teksilo::i18n::manager::I18nManager;
    use teksilo::i18n::thread_local::{clear, install};

    /// Install the real message patterns, copied from `locales/en-US/main.ftl`, so these
    /// tests assert the text a writer actually sees rather than a bare key.
    fn with_messages(f: impl FnOnce()) {
        clear();
        let cfg = I18nConfig::test_only(
            "en-US",
            &[
                ("could-not-open-work", "Could not open \"{ $file }\""),
                (
                    "could-not-open-work-too-new",
                    "\"{ $file }\" needs a newer Skribisto",
                ),
                (
                    "could-not-open-work-too-new-detail",
                    "Saved by Skribisto format { $written_by }; opening it needs format \
                     { $requires } or newer, and this build supports up to format \
                     { $supported }. Update Skribisto to open it.",
                ),
            ],
        );
        install(I18nManager::from_config(&cfg));
        f();
        clear();
    }

    fn parts(path: &str, error: &anyhow::Error) -> (String, String) {
        let (title, body) = open_failure_parts(path, error);
        (title.resolve_now(), body.resolve_now())
    }

    /// Three layers of context sat between the real cause and this leaf, and
    /// `e.to_string()` printed only the outermost — so every failed open, whatever went
    /// wrong, rendered as the literal words "Could not open work: load_work".
    #[test]
    fn a_generic_failure_shows_the_whole_chain_not_just_the_outermost_frame() {
        with_messages(|| {
            let err = anyhow::anyhow!("Unexpected variant named \"EpigraphText\"")
                .context("reading project '/tmp/novel.skrib'")
                .context("load_work");

            // The bug, stated as an assertion: this is what the toast used to say.
            assert_eq!(err.to_string(), "load_work");

            let (title, body) = parts("/tmp/novel.skrib", &err);
            assert!(
                body.contains("EpigraphText"),
                "the innermost cause must survive to the toast, got: {body}"
            );
            assert!(title.contains("novel.skrib"), "got: {title}");
        });
    }

    /// The title truncates, so the file must be identified by its base name — the full
    /// path pushed everything that mattered off the end when this shipped first.
    #[test]
    fn the_title_names_the_file_not_its_whole_path() {
        with_messages(|| {
            let err = anyhow::anyhow!("boom");
            let (title, _) = parts("/tmp/claude-1000/-home-cyril-very-long/novel.skrib", &err);
            assert!(title.contains("novel.skrib"), "got: {title}");
            assert!(
                !title.contains("claude-1000"),
                "the path must not crowd out the message, got: {title}"
            );
        });
    }

    /// `anyhow`'s `Debug` would also print the chain, but as a multi-line report that
    /// *includes a backtrace* when `RUST_BACKTRACE` is set. `{:#}` is the one-line form.
    #[test]
    fn the_chain_is_rendered_on_a_single_line() {
        with_messages(|| {
            let err = anyhow::anyhow!("inner").context("middle").context("outer");
            let (_, body) = parts("/tmp/x.skrib", &err);
            assert!(
                !body.contains('\n'),
                "the body must stay one line, got: {body}"
            );
            assert!(body.contains("outer") && body.contains("inner"));
        });
    }

    /// "From the future" is the one failure worth its own wording, because it is the one
    /// the reader can act on. `downcast_ref` must find it through the context layers
    /// stacked on top by the use case and the frontend command.
    #[test]
    fn a_too_new_file_gets_its_own_actionable_message_through_the_context_layers() {
        with_messages(|| {
            let err = anyhow::Error::new(skrib_format::SkribFormatError::TooNew {
                written_by: 9,
                requires_at_least: 7,
                supported: 5,
            })
            .context("reading project '/tmp/future.skrib'")
            .context("load_work");

            let (title, body) = parts("/tmp/future.skrib", &err);

            // The title alone must carry the actionable fact, because it is the part that
            // survives truncation.
            assert!(title.contains("future.skrib"), "got: {title}");
            assert!(title.contains("newer Skribisto"), "got: {title}");

            // All three numbers, and the distinction between them, are the point: the
            // writer's generation is not the same as what the file actually needs.
            for expected in ["9", "7", "5"] {
                assert!(
                    body.contains(expected),
                    "expected {expected} in the body, got: {body}"
                );
            }
            assert!(
                !title.starts_with("Could not open"),
                "this must not fall through to the generic message, got: {title}"
            );
        });
    }

    /// A corrupt file is not a too-new file, and must not borrow its instruction to go
    /// and update Skribisto.
    #[test]
    fn a_non_version_format_error_stays_generic() {
        with_messages(|| {
            let err = anyhow::Error::new(skrib_format::SkribFormatError::Unreadable(
                anyhow::anyhow!("'x.skrib' is not a recognised .skrib file"),
            ))
            .context("load_work");

            let (title, body) = parts("/tmp/x.skrib", &err);
            assert!(title.starts_with("Could not open"), "got: {title}");
            assert!(!title.contains("newer Skribisto"), "got: {title}");
            assert!(body.contains("not a recognised"), "got: {body}");
        });
    }
}
