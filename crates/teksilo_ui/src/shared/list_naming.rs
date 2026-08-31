// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! "Is this name already taken?", for the Work-owned lists the writer names
//! rows in.
//!
//! Tags and note templates both answer it, identically and independently: the
//! two `colliding_name` functions differed only in the row type they took. They
//! do *not* agree on what happens next — a colliding tag is skipped on import
//! and a colliding template is suffixed "… (2)" — but that is the caller's
//! decision, and it needs this answer first either way.

/// A row the writer can name, in a list where names must not collide.
pub trait NamedRow {
    fn row_id(&self) -> u64;
    fn row_name(&self) -> &str;
}

/// The comparison key: case-insensitive, surrounding space ignored.
///
/// Case-insensitive because "Protagonist" and "protagonist" are one tag to
/// everyone except a byte comparison, and a list that holds both is a list the
/// writer has to look at twice.
pub fn name_key(name: &str) -> String {
    name.trim().to_lowercase()
}

/// The existing row whose name collides with `candidate`, or `None`.
///
/// `exclude` is the row being renamed, which never collides with itself. An
/// empty candidate collides with nothing: it is not a name yet, and reporting a
/// collision for it would put an error under a field the writer has not filled
/// in.
pub fn colliding_name<T: NamedRow>(
    rows: &[T],
    candidate: &str,
    exclude: Option<u64>,
) -> Option<String> {
    let key = name_key(candidate);
    if key.is_empty() {
        return None;
    }
    rows.iter()
        .find(|r| Some(r.row_id()) != exclude && name_key(r.row_name()) == key)
        .map(|r| r.row_name().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Row(u64, &'static str);
    impl NamedRow for Row {
        fn row_id(&self) -> u64 {
            self.0
        }
        fn row_name(&self) -> &str {
            self.1
        }
    }

    fn rows() -> Vec<Row> {
        vec![Row(1, "Protagonist"), Row(2, "  Antagonist ")]
    }

    #[test]
    fn a_collision_is_found_across_case_and_surrounding_space() {
        assert_eq!(
            colliding_name(&rows(), "protagonist", None).as_deref(),
            Some("Protagonist")
        );
        assert_eq!(
            colliding_name(&rows(), "ANTAGONIST", None).as_deref(),
            Some("  Antagonist ")
        );
    }

    #[test]
    fn a_row_being_renamed_never_collides_with_itself() {
        assert_eq!(colliding_name(&rows(), "Protagonist", Some(1)), None);
        assert_eq!(
            colliding_name(&rows(), "Protagonist", Some(2)).as_deref(),
            Some("Protagonist")
        );
    }

    #[test]
    fn an_empty_candidate_collides_with_nothing() {
        assert_eq!(colliding_name(&rows(), "   ", None), None);
    }
}
