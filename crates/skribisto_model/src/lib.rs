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

/// The per-project chapter storage mode — generated on the `Work` entity, re-exported
/// here so `CreateType::combo` and the UI can name it via `skribisto_model`.
pub use common::entities::ChapterMode;

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
        // A Chapter folder carries its own prose (SceneText) — symmetric with the
        // flat `Item/ChapterScene` — so it can *contain* child Scenes AND hold
        // prose directly, and promote/demote between the two encodings is lossless.
        allowed: &[ChapterTitle, SceneText, SynopsisText],
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

/// How a recommended new item is placed relative to the anchor the writer
/// selected/right-clicked. Purely topological — this crate has no access to the
/// live item stream, so it cannot resolve a `Relation` to a concrete index
/// itself; the view-model does that (see `OutlineViewModel::insertion_point_for`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
    /// Append **inside** a container anchor — after its whole subtree, but before
    /// any direct child that `closes_book()` (so a Book's `BookEnd` stays last).
    /// Only ever emitted for `Folder` anchors (`RoleExt::is_container`).
    Child,
    /// After the anchor's **entire subtree**, at the anchor's own indent (a
    /// sibling of a populated folder lands after its children, not among them).
    Sibling,
    /// As `Sibling` of the nearest ancestor whose `sub_role` `opens_chapter()`
    /// or `opens_book()` — "close what I'm inside and start the next one".
    ParentSibling,
}

/// The user-facing "create" vocabulary — a *logical* type, decoupled from its
/// storage encoding. Notably `Chapter` maps to either a `Folder/Chapter` or an
/// `Item/ChapterScene` depending on the project's [`ChapterMode`], so the writer
/// only ever sees one "Chapter" (they promote/demote to switch encoding). This
/// is a deliberately curated subset of the valid `COMBINATIONS` — the internal
/// forms (`Item/Part`, `Item/BookBegin`, `Item/Chapter`, `Item/Text`) are never
/// offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateType {
    Book,
    Part,
    Chapter,
    Scene,
    Note,
    NoteFolder,
    Folder,
    EndOfBook,
}

impl CreateType {
    /// Resolve to a concrete `(role, sub_role)` for creation. Only `Chapter`
    /// depends on the project's `ChapterMode`; every other type is fixed.
    pub fn combo(self, mode: ChapterMode) -> (Role, SubRole) {
        match self {
            CreateType::Book => (Role::Folder, SubRole::Book),
            CreateType::Part => (Role::Folder, SubRole::Part),
            CreateType::Chapter => match mode {
                ChapterMode::Folder => (Role::Folder, SubRole::Chapter),
                ChapterMode::Flat => (Role::Item, SubRole::ChapterScene),
            },
            CreateType::Scene => (Role::Item, SubRole::Scene),
            CreateType::Note => (Role::Item, SubRole::Note),
            CreateType::NoteFolder => (Role::Folder, SubRole::Note),
            CreateType::Folder => (Role::Folder, SubRole::None),
            CreateType::EndOfBook => (Role::Item, SubRole::BookEnd),
        }
    }

    /// Whether creating this type closes a book (the single `EndOfBook`) — drives
    /// the "hide once the book already has one" gating.
    pub fn closes_book(self) -> bool {
        matches!(self, CreateType::EndOfBook)
    }
}

/// One ranked "create" offer: which logical type to create and where to place
/// it. UI labels/tooltips live in the view, not here (this crate stays
/// string-free).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Recommendation {
    pub create_type: CreateType,
    pub relation: Relation,
}

/// Canonical order for the "…and the others" tail — every offerable
/// [`CreateType`], biggest container first, the book-end marker last.
const CANONICAL: &[CreateType] = &[
    CreateType::Book,
    CreateType::Part,
    CreateType::Chapter,
    CreateType::Scene,
    CreateType::NoteFolder,
    CreateType::Note,
    CreateType::Folder,
    CreateType::EndOfBook,
];

/// The ordered "create" recommendations for an item anchor with the given
/// `(role, sub_role)`. The first entry is the default (the SplitButton title);
/// the rest form the dropdown, ending with the canonical tail of every remaining
/// type.
///
/// Note: the Book context includes an `EndOfBook` pick unconditionally — the
/// "hide it once the book already has one" gating needs live data and is applied
/// by the caller, not here.
pub fn recommendations(role: &Role, sub_role: &SubRole) -> Vec<Recommendation> {
    use CreateType as T;
    use Relation::{Child, ParentSibling, Sibling};
    use Role::{Folder, Item};
    use SubRole as S;

    let picks: Vec<(CreateType, Relation)> = match (role, sub_role) {
        (Folder, S::Book) => vec![(T::Chapter, Child), (T::Part, Child), (T::EndOfBook, Child)],
        (Folder, S::Part) => vec![(T::Chapter, Child), (T::Part, Sibling)],
        (Folder, S::Chapter) => vec![(T::Scene, Child), (T::Chapter, Sibling)],
        (Folder, S::None) => vec![(T::Note, Child), (T::Folder, Sibling), (T::Folder, Child)],
        (Folder, S::Note) => vec![
            (T::Note, Child),
            (T::NoteFolder, Sibling),
            (T::NoteFolder, Child),
        ],
        (Item, S::Scene) => vec![(T::Scene, Sibling), (T::Chapter, ParentSibling)],
        (Item, S::ChapterScene) => vec![(T::Chapter, Sibling)],
        (Item, S::Note) => vec![(T::Note, Sibling)],
        (Item, S::BookEnd) => vec![(T::Book, ParentSibling)],
        // Legacy anchors — no longer offered as *types*, but existing data may
        // still hold them; recommend a sensible offerable sibling if selected.
        (Item, S::BookBegin) => vec![(T::Chapter, Sibling)],
        (Item, S::Chapter) => vec![(T::Scene, Sibling)],
        (Item, S::Part) => vec![(T::Chapter, Sibling)],
        (Item, S::Text) => vec![(T::Scene, Sibling)],
        _ => vec![],
    };

    assemble(picks)
}

/// Recommendations for the top level — no selection, or a Binder row selected: a
/// Book first (the entry point of the compile stream), then every other type,
/// all inserted at the binder's top level.
pub fn recommendations_root() -> Vec<Recommendation> {
    assemble(vec![(CreateType::Book, Relation::Sibling)])
}

/// The paired type a binder item promotes/demotes to — a bidirectional toggle.
/// `None` if the item's `(role, sub_role)` has no promote pair.
///
/// Pairs: flat Chapter (`Item/ChapterScene`) ↔ Chapter folder (`Folder/Chapter`),
/// Scene ↔ Note, and Folder ↔ Note folder. Converting a container to a leaf
/// (Chapter folder → flat Chapter) requires the folder to be empty first — the
/// caller enforces that; this function only names the target.
pub fn promote_target(role: &Role, sub_role: &SubRole) -> Option<(Role, SubRole)> {
    use Role::{Folder, Item};
    use SubRole as S;
    Some(match (role, sub_role) {
        (Item, S::ChapterScene) => (Folder, S::Chapter),
        (Folder, S::Chapter) => (Item, S::ChapterScene),
        (Item, S::Scene) => (Item, S::Note),
        (Item, S::Note) => (Item, S::Scene),
        (Folder, S::None) => (Folder, S::Note),
        (Folder, S::Note) => (Folder, S::None),
        _ => return None,
    })
}

/// Remap one content role into the promote target's vocabulary, so prose
/// survives a type change: `SceneText` ↔ `NoteText` for Scene↔Note; anything the
/// target already allows is kept. `None` means the content role has no home in
/// the target (not expected for the supported pairs — the caller may drop it).
pub fn remap_content(
    target_role: &Role,
    target_sub_role: &SubRole,
    content: &ContentRole,
) -> Option<ContentRole> {
    if content_allowed(target_role, target_sub_role, content) {
        return Some(content.clone());
    }
    let swapped = match content {
        SceneText => NoteText,
        NoteText => SceneText,
        other => other.clone(),
    };
    content_allowed(target_role, target_sub_role, &swapped).then_some(swapped)
}

/// Build the final ordered list: the context picks, then the canonical tail of
/// every remaining `CreateType`, each as `Sibling`.
fn assemble(picks: Vec<(CreateType, Relation)>) -> Vec<Recommendation> {
    let mut out: Vec<Recommendation> = picks
        .iter()
        .map(|&(create_type, relation)| Recommendation {
            create_type,
            relation,
        })
        .collect();
    for &create_type in CANONICAL {
        if picks.iter().any(|&(t, _)| t == create_type) {
            continue;
        }
        out.push(Recommendation {
            create_type,
            relation: Relation::Sibling,
        });
    }
    out
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

    fn rec(create_type: CreateType, relation: Relation) -> Recommendation {
        Recommendation {
            create_type,
            relation,
        }
    }

    #[test]
    fn chapter_folder_now_carries_scene_prose() {
        // Symmetric with the flat Item/ChapterScene so promote/demote is lossless.
        assert!(content_allowed(
            &Role::Folder,
            &SubRole::Chapter,
            &SceneText
        ));
    }

    #[test]
    fn chapter_type_resolves_by_mode() {
        assert_eq!(
            CreateType::Chapter.combo(ChapterMode::Folder),
            (Role::Folder, SubRole::Chapter)
        );
        assert_eq!(
            CreateType::Chapter.combo(ChapterMode::Flat),
            (Role::Item, SubRole::ChapterScene)
        );
    }

    #[test]
    fn every_anchor_yields_valid_offerable_recommendations() {
        for c in COMBINATIONS {
            let recs = recommendations(&c.role, &c.sub_role);
            assert!(
                !recs.is_empty(),
                "no recommendations for anchor {:?}/{:?}",
                c.role,
                c.sub_role
            );
            for r in &recs {
                // Every offered type resolves to a valid combination in *both* modes.
                for mode in [ChapterMode::Folder, ChapterMode::Flat] {
                    let (role, sub_role) = r.create_type.combo(mode.clone());
                    assert!(
                        is_valid_combination(&role, &sub_role),
                        "{:?} in {:?} mode is not a valid combination",
                        r.create_type,
                        mode
                    );
                }
            }
        }
    }

    #[test]
    fn leaf_anchors_never_offer_a_child_relation() {
        for c in COMBINATIONS {
            if c.role == Role::Item {
                let recs = recommendations(&c.role, &c.sub_role);
                assert!(
                    recs.iter().all(|r| r.relation != Relation::Child),
                    "leaf anchor {:?}/{:?} offered a Child relation",
                    c.role,
                    c.sub_role
                );
            }
        }
    }

    #[test]
    fn book_recommends_chapter_first_then_the_worked_example() {
        let recs = recommendations(&Role::Folder, &SubRole::Book);
        let leading: Vec<_> = recs.iter().take(3).copied().collect();
        assert_eq!(
            leading,
            vec![
                rec(CreateType::Chapter, Relation::Child),
                rec(CreateType::Part, Relation::Child),
                rec(CreateType::EndOfBook, Relation::Child),
            ]
        );
        // "…and the others (even Book)".
        assert!(recs.iter().any(|r| r.create_type == CreateType::Book));
    }

    #[test]
    fn chapter_folder_recommends_scene_then_sibling_chapter() {
        let recs = recommendations(&Role::Folder, &SubRole::Chapter);
        assert_eq!(recs[0], rec(CreateType::Scene, Relation::Child));
        assert_eq!(recs[1], rec(CreateType::Chapter, Relation::Sibling));
    }

    #[test]
    fn folder_recommends_note_then_sibling_and_child_folder() {
        let recs = recommendations(&Role::Folder, &SubRole::None);
        assert_eq!(recs[0], rec(CreateType::Note, Relation::Child));
        assert_eq!(recs[1], rec(CreateType::Folder, Relation::Sibling));
        assert_eq!(recs[2], rec(CreateType::Folder, Relation::Child));
    }

    #[test]
    fn scene_recommends_sibling_scene_then_parent_sibling_chapter() {
        let recs = recommendations(&Role::Item, &SubRole::Scene);
        assert_eq!(recs[0], rec(CreateType::Scene, Relation::Sibling));
        assert_eq!(recs[1], rec(CreateType::Chapter, Relation::ParentSibling));
    }

    #[test]
    fn root_recommends_book_first() {
        let recs = recommendations_root();
        assert_eq!(recs[0], rec(CreateType::Book, Relation::Sibling));
    }

    #[test]
    fn promote_pairs_are_symmetric_toggles() {
        for (r, sr) in [
            (Role::Item, SubRole::ChapterScene),
            (Role::Folder, SubRole::Chapter),
            (Role::Item, SubRole::Scene),
            (Role::Item, SubRole::Note),
            (Role::Folder, SubRole::None),
            (Role::Folder, SubRole::Note),
        ] {
            let (tr, tsr) = promote_target(&r, &sr).expect("has a promote pair");
            assert!(is_valid_combination(&tr, &tsr));
            // Toggling twice returns to the original type.
            assert_eq!(promote_target(&tr, &tsr), Some((r, sr)));
        }
    }

    #[test]
    fn non_promotable_types_have_no_pair() {
        assert_eq!(promote_target(&Role::Folder, &SubRole::Book), None);
        assert_eq!(promote_target(&Role::Item, &SubRole::BookEnd), None);
        assert_eq!(promote_target(&Role::Item, &SubRole::Text), None);
    }

    #[test]
    fn scene_note_promote_remaps_prose_losslessly() {
        // Scene → Note: SceneText becomes NoteText; SynopsisText kept.
        assert_eq!(
            remap_content(&Role::Item, &SubRole::Note, &SceneText),
            Some(NoteText)
        );
        assert_eq!(
            remap_content(&Role::Item, &SubRole::Note, &SynopsisText),
            Some(SynopsisText)
        );
        // Note → Scene: NoteText becomes SceneText.
        assert_eq!(
            remap_content(&Role::Item, &SubRole::Scene, &NoteText),
            Some(SceneText)
        );
    }

    #[test]
    fn chapter_promote_is_content_lossless() {
        // Every ChapterScene content role is already allowed by Folder/Chapter.
        for c in [ChapterTitle, SceneText, SynopsisText] {
            assert_eq!(
                remap_content(&Role::Folder, &SubRole::Chapter, &c),
                Some(c.clone())
            );
        }
    }
}
