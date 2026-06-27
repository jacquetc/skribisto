//! Domain rules for Skribisto's writing model — the hand-written counterpart to
//! the Qleany-generated enums in `common::entities`.
//!
//! Skribisto deliberately departs from the classic Book→Chapter→Scene hierarchy:
//! the visible binder tree is *organisational only* (the `role` axis: folder vs
//! item, which the compiler ignores), while the actual book structure is a state
//! machine over the flat, ordered item list, driven by the composable `sub_role`
//! axis (see [`SubRoleExt`]). `Content` is fully explicit — each row carries a
//! [`ContentRole`] rather than inferring meaning from position.
//!
//! This module is the single source of truth for which `(role, sub_role)`
//! combinations are valid and which content roles each permits. The same table
//! drives three consumers: backend validation, migration content-filtering (so
//! invalid rows are never constructed), and UI affordances. It is a candidate to
//! upstream into Qleany once that gains a Rust model layer.

use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::entities::ContentRole;
use common::entities::ContentRole::*;

/// One row of the constraint matrix: a valid `(role, sub_role)` pair and the
/// content roles it permits.
struct Combination {
    role: Role,
    sub_role: SubRole,
    allowed: &'static [ContentRole],
}

/// The constraint matrix — a 1:1 encoding of the writing model. Anything not
/// listed here is an invalid combination *by construction*; the structural rule
/// "folders cannot carry text / book-begin / book-end" therefore needs no
/// special case (no such row exists).
const COMBINATIONS: &[Combination] = &[
    // role = Item (leaf; the compile-stream markers live here)
    Combination {
        role: Role::Item,
        sub_role: SubRole::BookBegin,
        allowed: &[BookTitle, BookSubtitle],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::BookEnd,
        allowed: &[],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::Scene,
        allowed: &[SceneText, SynopsisText],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::ChapterScene,
        allowed: &[ChapterTitle, SceneText, SynopsisText],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::Part,
        allowed: &[PartTitle, SynopsisText],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::Chapter,
        allowed: &[ChapterTitle, SynopsisText],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::Note,
        allowed: &[NoteText, SynopsisText],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::Text,
        allowed: &[],
    },
    // role = Folder (UI container; same subroles, expressed by extent not markers)
    Combination {
        role: Role::Folder,
        sub_role: SubRole::None,
        allowed: &[SynopsisText],
    },
    Combination {
        role: Role::Folder,
        sub_role: SubRole::Chapter,
        allowed: &[ChapterTitle, SynopsisText],
    },
    Combination {
        role: Role::Folder,
        sub_role: SubRole::Part,
        allowed: &[PartTitle, SynopsisText],
    },
    Combination {
        role: Role::Folder,
        sub_role: SubRole::Book,
        allowed: &[BookTitle, BookSubtitle, SynopsisText],
    },
    Combination {
        role: Role::Folder,
        sub_role: SubRole::Note,
        allowed: &[SynopsisText],
    },
];

/// Why a `(role, sub_role, content)` triple is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelError {
    /// The `(role, sub_role)` pair is not a valid combination.
    UnknownCombination { role: Role, sub_role: SubRole },
    /// The content role is not permitted for this `(role, sub_role)`.
    DisallowedContent {
        role: Role,
        sub_role: SubRole,
        content: ContentRole,
    },
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelError::UnknownCombination { role, sub_role } => {
                write!(f, "invalid combination: {role:?} / {sub_role:?}")
            }
            ModelError::DisallowedContent {
                role,
                sub_role,
                content,
            } => {
                write!(
                    f,
                    "content {content:?} not allowed on {role:?} / {sub_role:?}"
                )
            }
        }
    }
}

impl std::error::Error for ModelError {}

fn lookup(role: &Role, sub_role: &SubRole) -> Option<&'static Combination> {
    COMBINATIONS
        .iter()
        .find(|c| &c.role == role && &c.sub_role == sub_role)
}

/// Whether `(role, sub_role)` is a valid combination of the writing model.
pub fn is_valid_combination(role: &Role, sub_role: &SubRole) -> bool {
    lookup(role, sub_role).is_some()
}

/// The content roles permitted for `(role, sub_role)`. Empty for invalid
/// combinations (and for combinations that genuinely carry no content, e.g.
/// `Item / BookEnd`).
pub fn allowed_content(role: &Role, sub_role: &SubRole) -> &'static [ContentRole] {
    lookup(role, sub_role).map(|c| c.allowed).unwrap_or(&[])
}

/// Whether a single content role is permitted for `(role, sub_role)`.
pub fn content_allowed(role: &Role, sub_role: &SubRole, content: &ContentRole) -> bool {
    allowed_content(role, sub_role).contains(content)
}

/// Validate a complete item: the `(role, sub_role)` must be a known combination
/// and every present content role must be permitted by it.
pub fn validate_item(
    role: &Role,
    sub_role: &SubRole,
    present: &[ContentRole],
) -> Result<(), ModelError> {
    let combo = lookup(role, sub_role).ok_or(ModelError::UnknownCombination {
        role: role.clone(),
        sub_role: sub_role.clone(),
    })?;
    for content in present {
        if !combo.allowed.contains(content) {
            return Err(ModelError::DisallowedContent {
                role: role.clone(),
                sub_role: sub_role.clone(),
                content: content.clone(),
            });
        }
    }
    Ok(())
}

/// Compile-semantics predicates over a `sub_role`. The compiler folds the flat
/// item list and uses these as its transition alphabet (folders/`role` and
/// `indent` do not feed it).
pub trait SubRoleExt {
    /// Opens a new book in the compile stream.
    fn opens_book(&self) -> bool;
    /// Closes the current book.
    fn closes_book(&self) -> bool;
    /// Opens a new part.
    fn opens_part(&self) -> bool;
    /// Opens a new chapter.
    fn opens_chapter(&self) -> bool;
    /// Contributes scene prose to the stream.
    fn carries_scene(&self) -> bool;
}

impl SubRoleExt for SubRole {
    fn opens_book(&self) -> bool {
        matches!(self, SubRole::Book | SubRole::BookBegin)
    }
    fn closes_book(&self) -> bool {
        matches!(self, SubRole::BookEnd)
    }
    fn opens_part(&self) -> bool {
        matches!(self, SubRole::Part)
    }
    fn opens_chapter(&self) -> bool {
        matches!(self, SubRole::Chapter | SubRole::ChapterScene)
    }
    fn carries_scene(&self) -> bool {
        matches!(self, SubRole::Scene | SubRole::ChapterScene)
    }
}

/// UI-axis predicates over a `role`.
pub trait RoleExt {
    /// Whether this node can contain children in the UI. The compiler ignores
    /// containment entirely; this drives drag-and-drop only.
    fn is_container(&self) -> bool;
}

impl RoleExt for Role {
    fn is_container(&self) -> bool {
        matches!(self, Role::Folder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_combinations_pass() {
        assert!(validate_item(&Role::Item, &SubRole::Scene, &[SceneText, SynopsisText]).is_ok());
        assert!(validate_item(&Role::Folder, &SubRole::None, &[SynopsisText]).is_ok());
        assert!(
            validate_item(
                &Role::Item,
                &SubRole::ChapterScene,
                &[ChapterTitle, SceneText, SynopsisText]
            )
            .is_ok()
        );
    }

    #[test]
    fn folders_cannot_carry_text_or_book_boundaries() {
        assert!(!is_valid_combination(&Role::Folder, &SubRole::Text));
        assert!(!is_valid_combination(&Role::Folder, &SubRole::BookBegin));
        assert!(!is_valid_combination(&Role::Folder, &SubRole::BookEnd));
    }

    #[test]
    fn disallowed_content_is_rejected() {
        // a book-begin marker cannot hold scene prose
        assert!(matches!(
            validate_item(&Role::Item, &SubRole::BookBegin, &[SceneText]),
            Err(ModelError::DisallowedContent { .. })
        ));
        // a plain text item carries no recognised content
        assert!(matches!(
            validate_item(&Role::Item, &SubRole::Text, &[SynopsisText]),
            Err(ModelError::DisallowedContent { .. })
        ));
    }

    #[test]
    fn default_item_is_a_valid_combination() {
        // Default BinderItem = Item + Text (the generated #[default]s).
        assert!(is_valid_combination(&Role::default(), &SubRole::default()));
    }

    #[test]
    fn compile_predicates() {
        assert!(SubRole::ChapterScene.opens_chapter());
        assert!(SubRole::ChapterScene.carries_scene());
        assert!(SubRole::Chapter.opens_chapter());
        assert!(!SubRole::Chapter.carries_scene());
        assert!(SubRole::BookBegin.opens_book());
        assert!(SubRole::BookEnd.closes_book());
        assert!(Role::Folder.is_container());
        assert!(!Role::Item.is_container());
    }
}
