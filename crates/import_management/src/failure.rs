// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! How a project import's failure is spelled for the one channel it has.
//!
//! A long operation reports failure as a string (`OperationStatus::Failed`, taken
//! from the error's `Display`), so whatever the UI is to act on has to survive as
//! text. Shared by the Plume and Manuskript use cases, which both end in exactly
//! this; a module rather than a use case calling a use case.

/// The message a failed import hands the `LongOperationManager`.
///
/// A refusal of a project nested past the ceiling travels as its own
/// `failure_message`, `XmlTooDeep`'s for XML and `FoldersTooDeep`'s for a
/// Manuskript outline kept as folders, which the UI turns back into the typed
/// value and words in the writer's own language. Anything else is flattened to
/// its full `{:#}` chain: the manager records only `e.to_string()`, which for a
/// plain `anyhow` error is the outermost context alone, losing the root cause the
/// UI's error toast wants to show.
pub(crate) fn for_long_operation(error: anyhow::Error) -> anyhow::Error {
    if let Some(refused) = skrib_format::xml_depth::too_deep(&error) {
        return anyhow::anyhow!(refused.failure_message());
    }
    if let Some(refused) = skrib_format::xml_depth::folders_too_deep(&error) {
        return anyhow::anyhow!(refused.failure_message());
    }
    anyhow::anyhow!("{error:#}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use skrib_format::{FoldersTooDeep, XmlTooDeep};

    #[test]
    fn a_depth_refusal_reaches_the_ui_as_the_typed_value() {
        let refused = XmlTooDeep {
            part: "world.opml".to_string(),
            depth: 257,
            line: 4,
        };
        let error = anyhow::Error::new(refused.clone()).context("reading the project");
        let message = for_long_operation(error).to_string();
        assert_eq!(XmlTooDeep::from_failure_message(&message), Some(refused));
    }

    #[test]
    fn a_folder_refusal_reaches_the_ui_as_the_typed_value() {
        let refused = FoldersTooDeep {
            part: format!("outline/{}x.md", "0-a/".repeat(300)),
            depth: 301,
        };
        let error = anyhow::Error::new(refused.clone()).context("opening the project");
        let message = for_long_operation(error).to_string();
        assert_eq!(
            FoldersTooDeep::from_failure_message(&message),
            Some(refused)
        );
        assert_eq!(XmlTooDeep::from_failure_message(&message), None);
    }

    #[test]
    fn any_other_failure_keeps_its_whole_chain() {
        let error = anyhow::anyhow!("root cause").context("outer");
        let message = for_long_operation(error).to_string();
        assert!(
            message.contains("outer") && message.contains("root cause"),
            "{message}"
        );
        assert_eq!(XmlTooDeep::from_failure_message(&message), None);
        assert_eq!(FoldersTooDeep::from_failure_message(&message), None);
    }
}
