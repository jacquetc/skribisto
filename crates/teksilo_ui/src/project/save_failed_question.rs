// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The question asked when the save a quit, a close or a project switch was
//! waiting on fails.
//!
//! Not a view-model: the one wording and the one button set those three flows
//! share, so they cannot drift apart. What each answer *does* stays with the
//! flow that asked ([`crate::project::QuitSequencer`],
//! [`crate::project::ProjectSwitchViewModel`], the deferred close in
//! `app::wiring::save_and_exit`).
//!
//! # Why a question and not a toast
//!
//! With autosave on, every one of those flows saves without asking first
//! ([`crate::project::UnsavedDecision::SaveThenProceed`]). When that save fails
//! (a network drive that dropped, a full or write-protected disk, a USB key
//! taken out), dropping the request and saying so in a toast left the writer
//! in a loop: the next Quit saved again, failed again, and stayed again, so the
//! only ways out were to force-quit (losing the edits with no word about it) or
//! to find the autosave switch. Asking breaks the loop without discarding
//! anything the writer did not choose to discard: leaving without saving is one
//! button whose consequence the dialog states, and Cancel, Escape, a press
//! outside and closing the dialog all keep every edit and every window.
//!
//! # Why there is no Save As button
//!
//! Save As is a per-window view-model (`SaveAsViewModel`, two commands: a single
//! file or a folder), and the quit asks this from whichever window started the
//! quit, which need not be the window of the project that failed. Offering it
//! here would mean either saving the wrong project or reaching across windows,
//! and then resuming a quit from the end of a Save As. Cancel gets the writer
//! there in one step instead, with nothing lost, and the dialog says so.

use teksilo::i18n::LocalizedString;
use teksilo::prelude::*;
use teksilo::widgets::{MessageBox, MessageBoxButton, MessageBoxButtons, StandardButton};

/// What the writer was doing when the save failed: it decides the dialog's
/// title and the label of the button that goes on without saving.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Leaving {
    /// Quitting the application (Ctrl+Q / Cmd+Q, or a quit the system asked for
    /// and the writer can answer).
    Quit,
    /// Closing the work (Ctrl+W, File ▸ Close Work, the window's close button).
    Close,
    /// Replacing the work in this window (New Work, Open Work, "Open here", the
    /// import toast's "Open now").
    Switch,
}

/// The writer's answer.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SaveFailedAnswer {
    /// Go on without saving: the changes since the last successful save are lost.
    LeaveWithoutSaving,
    /// Stay: nothing is lost, the request is dropped.
    Stay,
}

impl SaveFailedAnswer {
    /// Only the explicit button leaves. Cancel, Escape, a press outside and a
    /// dialog closed any other way all keep the edits: losing them must be a
    /// choice the writer made, never what a stray key did.
    pub fn from_button(button: StandardButton) -> Self {
        match button {
            StandardButton::Discard => Self::LeaveWithoutSaving,
            _ => Self::Stay,
        }
    }
}

/// The title the dialog names the work by: its own, or "Untitled".
pub fn display_title(title: Option<String>) -> String {
    match title {
        Some(title) if !title.trim().is_empty() => title,
        _ => tr!(untitled()).resolve_now(),
    }
}

/// The title of the open Work `work_id`, read from its session (Layer A), for a
/// flow that knows the Work only by id. `None` when it is not open (the dialog
/// then says "Untitled").
///
/// The registry is Tier 1, one per process, so `app_state` is the right way to
/// reach it, unlike any per-Work or per-window handle.
pub fn work_title(ctx: &EventContext, work_id: Option<u64>) -> Option<String> {
    let registry = ctx.app_state::<crate::sessions::WorkRegistry>()?;
    let session = registry.session_for(work_id?)?;
    Some(session.single_work.title().get())
}

/// The dialog's title, which is also how a test or a script finds it.
pub fn question_title(leaving: Leaving, title: &str) -> LocalizedString {
    let title = title.to_string();
    match leaving {
        Leaving::Quit => tr!(save_failed_quit_title(title = title)),
        Leaving::Close => tr!(save_failed_close_title(title = title)),
        Leaving::Switch => tr!(save_failed_switch_title(title = title)),
    }
}

/// The label of the button that goes on without saving.
pub fn leave_label(leaving: Leaving) -> LocalizedString {
    match leaving {
        Leaving::Quit => tr!(save_failed_quit()),
        Leaving::Close => tr!(save_failed_close()),
        Leaving::Switch => tr!(save_failed_switch()),
    }
}

/// Ask what to do now that the save `leaving` was waiting on has failed.
///
/// `title` is the work's title (`None` or blank reads "Untitled"); `error` is
/// the backend's message, `None` (or empty) when the save could not even be
/// started. `on_answer` runs exactly once, however the dialog goes away.
///
/// The backend's message names paths and system errors at length, in English,
/// so it goes under "Show details" rather than in the text a writer reads
/// first: in the text it pushed the buttons out of the dialog.
///
/// Cancel is the default button, so Enter on a dialog not yet read keeps
/// everything.
pub fn ask_after_failed_save(
    ctx: &mut EventContext,
    leaving: Leaving,
    title: Option<String>,
    error: Option<&str>,
    on_answer: impl Fn(SaveFailedAnswer, &mut EventContext) + 'static,
) {
    let title = display_title(title);
    let mut question = MessageBox::warning(question_title(leaving, &title))
        .text(tr!(save_failed_text(title = title.clone())))
        .informative_text(tr!(save_failed_consequence()));
    if let Some(error) = error.map(str::trim).filter(|e| !e.is_empty()) {
        question = question.detailed_text(lit!(error.to_string()));
    }
    question
        .buttons(MessageBoxButtons::Custom(vec![
            MessageBoxButton::standard(StandardButton::Discard).label(leave_label(leaving)),
            MessageBoxButton::standard(StandardButton::Cancel),
        ]))
        .default_button(StandardButton::Cancel)
        .escape_button(StandardButton::Cancel)
        .on_result(move |r, c| on_answer(SaveFailedAnswer::from_button(r.button), c))
        .present(ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Only the one button that says "without saving" may lose anything.
    #[test]
    fn only_the_leave_button_leaves() {
        assert_eq!(
            SaveFailedAnswer::from_button(StandardButton::Discard),
            SaveFailedAnswer::LeaveWithoutSaving
        );
        for stay in [
            StandardButton::Cancel,
            StandardButton::Close,
            StandardButton::Ok,
            StandardButton::Save,
        ] {
            assert_eq!(SaveFailedAnswer::from_button(stay), SaveFailedAnswer::Stay);
        }
    }

    #[test]
    fn a_work_without_a_title_is_called_untitled() {
        crate::test_support::with_real_messages(|| {
            assert_eq!(display_title(Some("Le Phare".into())), "Le Phare");
            assert_eq!(display_title(Some("  ".into())), "Untitled");
            assert_eq!(display_title(None), "Untitled");
        });
    }

    /// The dialog names the work and the failure, says what leaving costs and
    /// offers the way out, in the words a writer reads.
    #[test]
    fn the_question_names_the_work_the_failure_and_the_cost() {
        crate::test_support::with_real_messages(|| {
            let mut tree = teksilo::core::widget_tree::WidgetTree::new();
            crate::test_support::press(&mut tree, |c| {
                ask_after_failed_save(
                    c,
                    Leaving::Quit,
                    Some("Le Phare".into()),
                    Some("Permission denied (os error 13)"),
                    |_, _| {},
                )
            });
            let request = tree
                .drain_pending_modal_requests()
                .pop()
                .expect("a question is asked")
                .request;
            assert_eq!(
                request.title.as_deref(),
                Some("Quit without saving Le Phare?")
            );
            let teksilo::core::ModalContent::Deferred(builder) = request.content else {
                panic!("a MessageBox presents deferred content");
            };
            builder(&mut tree);
            tree.layout(SizeProposal::exact(900.0, 600.0));
            for label in ["Quit without saving", "Cancel"] {
                assert!(tree.find_by_label(label).is_some(), "offers {label}");
            }
            let names = tree
                .sync_accessibility()
                .nodes
                .iter()
                .filter_map(|(_, node)| node.description().map(str::to_string))
                .collect::<Vec<_>>()
                .join("\n");
            assert!(names.contains("Le Phare could not be saved."), "{names}");
            assert!(
                tree.find_by_label("Show details").is_some(),
                "the system's own message is one step away"
            );
            assert!(
                names.contains("the changes made since the last successful save are lost"),
                "{names}"
            );
        });
    }
}
