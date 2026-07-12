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
        // Symmetric with the `Folder/Book` container: the flat book-start marker
        // carries the book's synopsis too, so a book outline has no hole whichever
        // encoding the book uses.
        allowed: &[BookTitle, BookSubtitle, SynopsisText],
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
        // A chapter folder carries its own prose, so it is *the same thing* as the
        // flat `Item/ChapterScene` — same sub_role, same allowed content — differing
        // only on the UI-only `role` axis (a Folder can contain children). A chapter has
        // exactly two encodings, `Item/ChapterScene` (extent by marker) and
        // `Folder/ChapterScene` (extent by containment), and promote/demote between them
        // is lossless by construction. There is no prose-less chapter, so there is no
        // separate `Chapter` sub_role at all.
        sub_role: SubRole::ChapterScene,
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
        matches!(self, SubRole::ChapterScene)
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
            // One "Chapter" in the UI, two encodings: the `role` axis is the whole
            // difference — the sub_role is `ChapterScene` either way.
            CreateType::Chapter => match mode {
                ChapterMode::Folder => (Role::Folder, SubRole::ChapterScene),
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
        (Folder, S::ChapterScene) => vec![(T::Scene, Child), (T::Chapter, Sibling)],
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

/// A concrete type a binder item can be promoted/demoted **to**.
///
/// Promote is no longer a single paired toggle: a folder can become *any other kind of
/// folder* (a plain folder → a chapter, a part, a book, a note folder…), so a target
/// has to be named rather than derived. Each variant carries a **stable numeric code**
/// which is what crosses the DTO boundary — `PromoteDto` sends the code, and the use
/// case resolves it back here, so the wire format never depends on the *order* of
/// [`promote_targets`] (a list index would silently pick the wrong type if the item's
/// type changed between opening a menu and clicking it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromoteTarget {
    /// `Folder/None` — a plain grouping folder.
    Folder,
    /// `Folder/ChapterScene` — a chapter that holds its scenes.
    ChapterFolder,
    /// `Folder/Part`.
    PartFolder,
    /// `Folder/Book`.
    BookFolder,
    /// `Folder/Note` — a notes folder.
    NoteFolder,
    /// `Item/ChapterScene` — a flat chapter (one row, no scenes under it).
    FlatChapter,
    /// `Item/Scene`.
    Scene,
    /// `Item/Note`.
    Note,
}

impl PromoteTarget {
    /// The concrete `(role, sub_role)` this target is.
    pub fn combo(self) -> (Role, SubRole) {
        use PromoteTarget as T;
        match self {
            T::Folder => (Role::Folder, SubRole::None),
            T::ChapterFolder => (Role::Folder, SubRole::ChapterScene),
            T::PartFolder => (Role::Folder, SubRole::Part),
            T::BookFolder => (Role::Folder, SubRole::Book),
            T::NoteFolder => (Role::Folder, SubRole::Note),
            T::FlatChapter => (Role::Item, SubRole::ChapterScene),
            T::Scene => (Role::Item, SubRole::Scene),
            T::Note => (Role::Item, SubRole::Note),
        }
    }

    /// The stable wire code. **Append-only** — these cross the `PromoteDto` boundary.
    pub fn code(self) -> u64 {
        use PromoteTarget as T;
        match self {
            T::Folder => 0,
            T::ChapterFolder => 1,
            T::PartFolder => 2,
            T::BookFolder => 3,
            T::NoteFolder => 4,
            T::FlatChapter => 5,
            T::Scene => 6,
            T::Note => 7,
        }
    }

    pub fn from_code(code: u64) -> Option<Self> {
        use PromoteTarget as T;
        Some(match code {
            0 => T::Folder,
            1 => T::ChapterFolder,
            2 => T::PartFolder,
            3 => T::BookFolder,
            4 => T::NoteFolder,
            5 => T::FlatChapter,
            6 => T::Scene,
            7 => T::Note,
            _ => return None,
        })
    }
}

/// Every type `(role, sub_role)` may be promoted/demoted to, in menu order.
///
/// **A folder may become any other kind of folder** — that is the point: the writer
/// drafts an outline in plain folders and then declares "this one is a chapter, that
/// one is a part". A chapter folder additionally demotes to the flat chapter it is the
/// container form of; Scene and Note remain a pair.
///
/// The list never contains the item's current type. It says nothing about whether the
/// conversion is *safe* — a container becoming a leaf needs an empty folder
/// ([`demote_blocked_children`](crate) on the caller), and a conversion that would drop
/// text is caught by [`promote_content_loss`].
pub fn promote_targets(role: &Role, sub_role: &SubRole) -> Vec<PromoteTarget> {
    use PromoteTarget as T;
    use Role::{Folder, Item};
    use SubRole as S;
    match (role, sub_role) {
        // Any folder ↔ any other folder. A chapter folder also demotes to its flat form.
        (Folder, S::None) => vec![
            T::ChapterFolder,
            T::PartFolder,
            T::BookFolder,
            T::NoteFolder,
        ],
        (Folder, S::ChapterScene) => vec![
            T::FlatChapter,
            T::PartFolder,
            T::BookFolder,
            T::NoteFolder,
            T::Folder,
        ],
        (Folder, S::Part) => vec![T::ChapterFolder, T::BookFolder, T::NoteFolder, T::Folder],
        (Folder, S::Book) => vec![T::ChapterFolder, T::PartFolder, T::NoteFolder, T::Folder],
        (Folder, S::Note) => vec![T::ChapterFolder, T::PartFolder, T::BookFolder, T::Folder],
        // Leaves: the chapter's two encodings, and the Scene ↔ Note pair.
        (Item, S::ChapterScene) => vec![T::ChapterFolder],
        (Item, S::Scene) => vec![T::Note],
        (Item, S::Note) => vec![T::Scene],
        _ => Vec::new(),
    }
}

/// The single title role a combination carries, if any (`BookTitle` / `PartTitle` /
/// `ChapterTitle`). Used to carry a *name* across a type change: a chapter folder that
/// becomes a part keeps its title, it just becomes a part title.
fn title_role_of(role: &Role, sub_role: &SubRole) -> Option<ContentRole> {
    [BookTitle, PartTitle, ChapterTitle]
        .into_iter()
        .find(|r| content_allowed(role, sub_role, r))
}

/// Remap one content role into the promote target's vocabulary, so the writer's text
/// survives a type change:
///
/// * anything the target already allows is kept;
/// * a **title** becomes the target's title (`ChapterTitle` → `PartTitle` → `BookTitle`)
///   — the name of the thing outlives what kind of thing it is;
/// * `SceneText` ↔ `NoteText` for Scene ↔ Note.
///
/// `None` means the role has no home in the target and its text would be **dropped** —
/// see [`promote_content_loss`], which is what stops that happening silently.
pub fn remap_content(
    target_role: &Role,
    target_sub_role: &SubRole,
    content: &ContentRole,
) -> Option<ContentRole> {
    if content_allowed(target_role, target_sub_role, content) {
        return Some(content.clone());
    }
    if matches!(content, BookTitle | PartTitle | ChapterTitle)
        && let Some(t) = title_role_of(target_role, target_sub_role)
    {
        return Some(t);
    }
    let swapped = match content {
        SceneText => NoteText,
        NoteText => SceneText,
        other => other.clone(),
    };
    content_allowed(target_role, target_sub_role, &swapped).then_some(swapped)
}

/// Which of `present` content roles would be **lost** by promoting to
/// `(target_role, target_sub_role)` — i.e. have no home there even after remapping.
///
/// The caller passes only the roles whose text is *non-empty*, so an empty leftover row
/// never blocks a conversion. A chapter folder holding prose cannot become a Part (a
/// Part has no `SceneText`); the use case refuses rather than quietly discarding it, and
/// the UI explains why.
pub fn promote_content_loss(
    target_role: &Role,
    target_sub_role: &SubRole,
    present: &[ContentRole],
) -> Vec<ContentRole> {
    present
        .iter()
        .filter(|c| remap_content(target_role, target_sub_role, c).is_none())
        .cloned()
        .collect()
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
        // The chapter folder *is* the flat ChapterScene in container form — same
        // sub_role, same content — so promote/demote is lossless.
        assert!(content_allowed(
            &Role::Folder,
            &SubRole::ChapterScene,
            &SceneText
        ));
        // A chapter *is* a ChapterScene in both encodings; the `role` axis is the
        // whole difference.
        assert!(content_allowed(
            &Role::Item,
            &SubRole::ChapterScene,
            &SceneText
        ));
    }

    #[test]
    fn book_begin_now_carries_synopsis() {
        // Symmetric with the Folder/Book container, so a book's synopsis exists
        // whichever encoding the book uses.
        assert!(content_allowed(
            &Role::Item,
            &SubRole::BookBegin,
            &SynopsisText
        ));
        assert!(
            validate_item(
                &Role::Item,
                &SubRole::BookBegin,
                &[BookTitle, BookSubtitle, SynopsisText]
            )
            .is_ok()
        );
    }

    /// The matrix has exactly 12 rows, and `bastyde_ui` mirrors them 1:1 (one tab
    /// module per combination — see `tabs::tab_pane`). Pinned so the docs and the tab
    /// dispatch can't silently drift from the model.
    #[test]
    fn the_matrix_has_twelve_combinations() {
        assert_eq!(COMBINATIONS.len(), 12);
    }

    /// Every combination that carries *any* content also carries a synopsis — the
    /// only exceptions are the two genuinely contentless markers. This is what lets
    /// the Full Synopsis stream render every row without a hole.
    #[test]
    fn every_content_bearing_combination_carries_a_synopsis() {
        for c in COMBINATIONS {
            if c.allowed.is_empty() {
                continue; // Item/BookEnd, Item/Text — contentless by design
            }
            assert!(
                c.allowed.contains(&SynopsisText),
                "{:?}/{:?} carries content but no synopsis",
                c.role,
                c.sub_role
            );
        }
    }

    #[test]
    fn chapter_type_resolves_by_mode() {
        assert_eq!(
            CreateType::Chapter.combo(ChapterMode::Folder),
            (Role::Folder, SubRole::ChapterScene)
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
        let recs = recommendations(&Role::Folder, &SubRole::ChapterScene);
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

    /// Every offered target is a valid combination, is never the item's *current* type,
    /// and is reachable back again — promote stays a closed, reversible graph.
    #[test]
    fn promote_targets_are_valid_distinct_and_reversible() {
        for c in COMBINATIONS {
            for t in promote_targets(&c.role, &c.sub_role) {
                let (tr, tsr) = t.combo();
                assert!(
                    is_valid_combination(&tr, &tsr),
                    "{:?} is not a valid combination",
                    t
                );
                assert!(
                    !(tr == c.role && tsr == c.sub_role),
                    "{:?}/{:?} offers itself as a promote target",
                    c.role,
                    c.sub_role
                );
                // The way back exists.
                let back: Vec<(Role, SubRole)> = promote_targets(&tr, &tsr)
                    .into_iter()
                    .map(|t| t.combo())
                    .collect();
                assert!(
                    back.contains(&(c.role.clone(), c.sub_role.clone())),
                    "{:?}/{:?} -> {:?} is a one-way street",
                    c.role,
                    c.sub_role,
                    t
                );
            }
        }
    }

    /// A folder may become any *other* kind of folder — that is the whole point: outline
    /// in plain folders, then declare what each one is.
    #[test]
    fn any_folder_promotes_to_any_other_folder() {
        use PromoteTarget as T;
        let folders = [
            (SubRole::None, T::Folder),
            (SubRole::ChapterScene, T::ChapterFolder),
            (SubRole::Part, T::PartFolder),
            (SubRole::Book, T::BookFolder),
            (SubRole::Note, T::NoteFolder),
        ];
        for (sr, _self_t) in &folders {
            let offered = promote_targets(&Role::Folder, sr);
            for (other_sr, other_t) in &folders {
                if other_sr == sr {
                    continue;
                }
                assert!(
                    offered.contains(other_t),
                    "Folder/{:?} does not offer {:?}",
                    sr,
                    other_t
                );
            }
        }
        // The chapter folder additionally demotes to its flat form.
        assert!(promote_targets(&Role::Folder, &SubRole::ChapterScene).contains(&T::FlatChapter));
    }

    #[test]
    fn non_promotable_types_have_no_targets() {
        assert!(promote_targets(&Role::Item, &SubRole::BookEnd).is_empty());
        assert!(promote_targets(&Role::Item, &SubRole::Text).is_empty());
        assert!(promote_targets(&Role::Item, &SubRole::BookBegin).is_empty());
    }

    /// The wire codes are a stable bijection — they cross the `PromoteDto` boundary, so
    /// a silent renumbering would promote items to the wrong type.
    #[test]
    fn promote_target_codes_round_trip() {
        use PromoteTarget as T;
        for t in [
            T::Folder,
            T::ChapterFolder,
            T::PartFolder,
            T::BookFolder,
            T::NoteFolder,
            T::FlatChapter,
            T::Scene,
            T::Note,
        ] {
            assert_eq!(PromoteTarget::from_code(t.code()), Some(t));
        }
        assert_eq!(PromoteTarget::from_code(99), None);
    }

    /// A plain folder carries only a synopsis, which every folder type allows — so the
    /// requested "outline in folders, then declare their type" flow never loses a word.
    #[test]
    fn promoting_a_plain_folder_is_always_lossless() {
        for t in promote_targets(&Role::Folder, &SubRole::None) {
            let (tr, tsr) = t.combo();
            assert!(
                promote_content_loss(&tr, &tsr, &[SynopsisText]).is_empty(),
                "Folder -> {:?} would lose the synopsis",
                t
            );
        }
    }

    /// A name outlives the kind of thing it names: a chapter that becomes a part keeps
    /// its title, as a part title.
    #[test]
    fn a_title_survives_a_type_change() {
        assert_eq!(
            remap_content(&Role::Folder, &SubRole::Part, &ChapterTitle),
            Some(PartTitle)
        );
        assert_eq!(
            remap_content(&Role::Folder, &SubRole::Book, &ChapterTitle),
            Some(BookTitle)
        );
        assert_eq!(
            remap_content(&Role::Folder, &SubRole::ChapterScene, &BookTitle),
            Some(ChapterTitle)
        );
        // A plain folder has no title role at all — the name would be lost.
        assert_eq!(
            remap_content(&Role::Folder, &SubRole::None, &ChapterTitle),
            Option::None
        );
    }

    /// Prose cannot be smuggled into a type that has nowhere to put it. A chapter
    /// holding text cannot become a Part; the caller must clear or move it first.
    #[test]
    fn prose_that_has_no_home_is_reported_as_lost() {
        let loss = promote_content_loss(
            &Role::Folder,
            &SubRole::Part,
            &[ChapterTitle, SceneText, SynopsisText],
        );
        assert_eq!(loss, vec![SceneText], "a Part carries no scene prose");

        // ...but an empty chapter converts cleanly (the caller only passes non-empty roles).
        assert!(
            promote_content_loss(&Role::Folder, &SubRole::Part, &[ChapterTitle, SynopsisText])
                .is_empty()
        );
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
                remap_content(&Role::Folder, &SubRole::ChapterScene, &c),
                Some(c.clone())
            );
        }
    }
}
