// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

// Panic hygiene: this crate is at zero `unwrap()`/`expect()`/`panic!` outside
// tests, so the lint is switched on here to keep it that way — CI lints with
// `-D warnings`, which makes any new panic path a build failure. See the note
// in the workspace `Cargo.toml` for why this is per-crate and not workspace-wide.
#![warn(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

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

/// Which language a scene is written in: the per-item → Work resolution chain,
/// plus the `dict_language` tag-list grammar shared by search folding and spell-checking.
#[cfg(test)]
mod image_policy_tests;

pub mod casing;
pub mod language;
pub mod mentions;

/// Locale typography facts — quote glyphs, pre-punctuation spacing, dialogue dashes.
/// The data half of what used to live in the UI's `text_replacement::typography`; the
/// per-keystroke engine that applies it stays there.
pub mod typography;

/// Manuscript analysis: repetition, prose shape, lexical diversity, synopsis coverage.
/// Pure measurement over strings and ids — no store, no entities, no UI.
pub mod analysis;

/// Where a comment points, and how it keeps pointing there — capture, live shift,
/// and the three-tier re-anchor. Pure functions over plain text, on the same terms
/// as `scene_break` and `mentions`.
///
/// It lived in `bastyde_ui::comments` until the document importer needed to capture
/// an anchor for an editor's comment coming out of a `.docx` or `.odt`. Capturing
/// with a *copy* of these rules is how a stored quote and the matcher that resolves
/// it silently drift apart — and this is the one module whose own doc says being
/// subtly wrong here "is invisible until someone's comment has silently moved to
/// the wrong sentence". One authority, two callers.
pub mod comment_anchor;

/// The compile spine: fold the flat `(role, sub_role)` item stream into an export scope.
/// Shares its structural predicates with the UI's Full Chapter/Part/Book stream view.
pub mod compile;

/// Word/char counting **policy** (which method) + a content-addressed cache over scene
/// prose. The mechanical primitive lives in `text-document`; this owns the method choice.
pub mod counting;

/// Which ordinal each structural row carries — one pass over the whole manuscript, so
/// the exporter and the binder's live badge cannot disagree about what chapter this is.
pub mod footnote_numbering;
pub mod numbering;
pub mod scene_break;

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
        allowed: &[ChapterTitle, EpigraphText, SceneText, SynopsisText],
    },
    Combination {
        role: Role::Item,
        sub_role: SubRole::Part,
        allowed: &[PartTitle, EpigraphText, SynopsisText],
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
        allowed: &[ChapterTitle, EpigraphText, SceneText, SynopsisText],
    },
    Combination {
        role: Role::Folder,
        sub_role: SubRole::Part,
        allowed: &[PartTitle, EpigraphText, SynopsisText],
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
    // A text that cannot be part of the book body: a preface, a dedication, an afterword,
    // an *achevé d'imprimer*. Excluded from the word count and every other statistic —
    // which is what `ParatextText` buys, since the count and the three other places that
    // measure prose all key off `SceneText` independently — but exported in stream order
    // like a scene.
    //
    // Deliberately carries no *kind*. Ordering conventions are national (a French book
    // puts the table of contents at the back; an American one at the front), so a kind
    // that drove placement would be wrong for half the writers, and a kind that drove
    // nothing would be a field nobody maintained. Where a paratext goes is the binder's
    // business, which is where this model already keeps structure.
    Combination {
        role: Role::Item,
        sub_role: SubRole::Paratext,
        allowed: &[ParatextText, SynopsisText],
    },
    // Organisational only — somewhere to keep the paratexts so they do not clutter the
    // binder. It emits nothing into the export, so its name never reaches the book.
    Combination {
        role: Role::Folder,
        sub_role: SubRole::Paratext,
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

/// Whether this `(role, sub_role)` carries countable manuscript prose — i.e. it owns a
/// `SceneText` body. The single predicate word counting keys off, so "words counted" tracks
/// exactly the scene prose the exporter emits (`Item/Scene`, `Item/ChapterScene`,
/// `Folder/ChapterScene`). Notes carry `NoteText`, not `SceneText`, and are deliberately
/// excluded from the manuscript count.
pub fn counts_prose(role: &Role, sub_role: &SubRole) -> bool {
    content_allowed(role, sub_role, &SceneText)
}

/// Whether this `(role, sub_role)` gets an **Overview** table — the dense, sortable
/// outliner of everything under a container.
///
/// Deliberately *not* [`compile::StreamLevel::for_container`], which answers a different
/// question ("does this container host a manuscript stream?") and returns `None` for a
/// notes folder. A `Folder/Note` has no manuscript extent, so it has no stream — but it
/// still holds a subtree worth tabulating, so it does get an Overview.
///
/// `Folder/None` is excluded: a plain grouping folder is an organisational bag with no
/// structural meaning, and its synopsis-only page is the whole of it.
///
/// Only folder containers qualify — a *leaf* has no subtree to tabulate, whatever its
/// sub-role, so the flat `Item/ChapterScene` encoding of a chapter is excluded exactly as
/// it is from the stream.
pub fn overview_capable(role: &Role, sub_role: &SubRole) -> bool {
    *role == Role::Folder
        && matches!(
            sub_role,
            SubRole::ChapterScene
                | SubRole::Part
                | SubRole::Book
                | SubRole::Note
                // A paratext folder has no manuscript extent, but it does hold a subtree
                // worth tabulating — the same reason a notes folder qualifies. Its tab
                // offers the Overview segment, so leaving it out here mounted a segment
                // over an empty pane.
                | SubRole::Paratext
        )
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

/// Which "kind" of writing item the Go menu's Next/Previous commands act on.
///
/// Deliberately its own axis, not a reuse of an existing one: [`SearchFacet`] buckets by
/// the same structural chip a writer *filters* by, which lumps `Folder/Note` (a notes
/// container) in with `Item/Note` (an actual note) — wrong here, since a Go command must
/// land *in* a note, never on its container. `tabs::prose_kind_for` collapses Scene and
/// Chapter onto one `ProseKind::Scene` (right for typography, since both use the Scene
/// bundle) but wrong for Go, which must let a writer jump to a chapter head specifically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoKind {
    Scene,
    Chapter,
    Note,
}

/// Which way a Go command steps through the binder's flat order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoDirection {
    Next,
    Previous,
}

/// Which [`GoKind`] `(role, sub_role)` counts as for the Go menu, or `None` if it is not a
/// Go target at all (a folder container that is not itself a chapter head, a structural
/// marker, the legacy `Item/Text` separator…).
///
/// Chapter identity spans **two** role encodings (`Item/ChapterScene` and
/// `Folder/ChapterScene`) — checked role-agnostically via `opens_chapter()` first, so
/// both land on `GoKind::Chapter` without a role match. Everything else keys on the exact
/// `(role, sub_role)` pair: only the *leaf* forms of Scene and Note are targets — a Note's
/// *container* (`Folder/Note`) is deliberately excluded, matching [`search_facet_of`]'s own
/// exclusion of the analogous case in the other direction, though for a different reason
/// here (there is nowhere useful to "land" on a folder from a Go command).
pub fn go_kind_of(role: &Role, sub_role: &SubRole) -> Option<GoKind> {
    if sub_role.opens_chapter() {
        return Some(GoKind::Chapter);
    }
    match (role, sub_role) {
        (Role::Item, SubRole::Scene) => Some(GoKind::Scene),
        (Role::Item, SubRole::Note) => Some(GoKind::Note),
        _ => None,
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
    /// A text that belongs to the book but not to its story.
    Paratext,
    /// A folder to keep paratexts in — organisational only.
    ParatextFolder,
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
            CreateType::Paratext => (Role::Item, SubRole::Paratext),
            CreateType::ParatextFolder => (Role::Folder, SubRole::Paratext),
            CreateType::EndOfBook => (Role::Item, SubRole::BookEnd),
        }
    }

    /// Whether creating this type closes a book (the single `EndOfBook`) — drives
    /// the "hide once the book already has one" gating.
    pub fn closes_book(self) -> bool {
        matches!(self, CreateType::EndOfBook)
    }

    /// The inverse of [`combo`](Self::combo): which `CreateType` a stored row *is*.
    ///
    /// `combo` has always been one-way, so every caller that needed to ask "what kind
    /// of thing did the writer just point at" either guessed or did without. The
    /// document importer needs it to answer the only question that matters about a
    /// destination — what should land inside it — and guessing there is what put an
    /// imported book between a chapter and its own scenes.
    ///
    /// **Mode-independent, and that is not a shortcut.** `Chapter` is the one type
    /// whose encoding depends on `ChapterMode`, and both encodings share the
    /// `ChapterScene` sub_role — so `(Folder, ChapterScene)` and `(Item, ChapterScene)`
    /// both answer `Chapter` without needing to know which mode produced them. Asking
    /// for the mode would only invite a caller to pass the wrong one.
    ///
    /// `None` for a pair outside `COMBINATIONS` — including the two title-bearing
    /// markers (`Text`, `BookBegin`) that no `CreateType` creates.
    pub fn of(role: &Role, sub_role: &SubRole) -> Option<CreateType> {
        match (role, sub_role) {
            (Role::Folder, SubRole::Book) => Some(CreateType::Book),
            (Role::Folder, SubRole::Part) => Some(CreateType::Part),
            (Role::Folder, SubRole::ChapterScene) | (Role::Item, SubRole::ChapterScene) => {
                Some(CreateType::Chapter)
            }
            (Role::Item, SubRole::Scene) => Some(CreateType::Scene),
            (Role::Item, SubRole::Note) => Some(CreateType::Note),
            (Role::Folder, SubRole::Note) => Some(CreateType::NoteFolder),
            (Role::Folder, SubRole::None) => Some(CreateType::Folder),
            (Role::Item, SubRole::Paratext) => Some(CreateType::Paratext),
            (Role::Folder, SubRole::Paratext) => Some(CreateType::ParatextFolder),
            (Role::Item, SubRole::BookEnd) => Some(CreateType::EndOfBook),
            _ => None,
        }
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
    // Offered from every anchor, like Note and Folder. Nothing restricts where a paratext
    // may go — a writer may want an interleaved author's note between two parts — so the
    // generic tail must carry them, or the only way to make one is from the three anchors
    // that name them explicitly.
    CreateType::ParatextFolder,
    CreateType::Paratext,
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
        (Folder, S::Book) => vec![
            (T::Chapter, Child),
            (T::Part, Child),
            (T::ParatextFolder, Child),
            (T::EndOfBook, Child),
        ],
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
        // Inside a paratext folder the obvious next thing is another paratext; from a
        // paratext leaf, a sibling. Nothing here restricts where one may go — the binder
        // does not police placement — these are only what the button offers first.
        (Folder, S::Paratext) => vec![(T::Paratext, Child), (T::ParatextFolder, Sibling)],
        (Item, S::Paratext) => vec![(T::Paratext, Sibling)],
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
/// one is a part". The same freedom, narrower, applies to the leaves: a chapter folder
/// demotes to the flat chapter it is the container form of, a Scene rises to that flat
/// chapter (and back), and Scene ↔ Note remain a pair.
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
        // Leaves: the chapter's two encodings, the Scene ↔ Note pair, and Scene ↔ the
        // flat chapter. A writer who drafts in scenes and then decides one of them *is*
        // the chapter should not have to retype it: the prose carries straight over
        // (both allow `SceneText` + `SynopsisText`), so the raise is lossless. The way
        // back only costs a chapter title, if one was actually written.
        (Item, S::ChapterScene) => vec![T::ChapterFolder, T::Scene],
        (Item, S::Scene) => vec![T::FlatChapter, T::Note],
        (Item, S::Note) => vec![T::Scene],
        _ => Vec::new(),
    }
}

/// What *kind of thing* a match was found in, as a reader would name it.
///
/// The constraint matrix has **twelve** `(role, sub_role)` combinations, and a writer filtering
/// their search results does not think in twelve. They think "show me the scenes" — and a
/// scene is a scene whether the project stores its chapters flat or as folders, which is a
/// storage decision they made once and should never have to remember again.
///
/// So this collapses the twelve onto six, and it is the *only* place that collapse is written
/// down. Derived from the matrix rather than listed alongside it: a thirteenth combination
/// added to `COMBINATIONS` fails [`the_facets_cover_every_combination`](self) until someone
/// says which chip it belongs under, rather than quietly becoming unfindable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchFacet {
    /// A book: its container (`Folder/Book`) and both of its flat markers.
    Book,
    Part,
    /// A chapter, in either of its two encodings — the writer chose one when they created the
    /// project and does not think of it as a difference.
    Chapter,
    Scene,
    Note,
    /// Structure with no place in the book's spine: a plain folder, and the inert `Item/Text`
    /// separator that only the Plume importer and the legacy upgrader produce.
    Folder,
    /// A text that is part of the published book but not of its body — a preface, a
    /// dedication, an afterword. Its own chip because it is neither structure the writer
    /// added to organise themselves nor prose the story is told in, and a search for a
    /// phrase in the manuscript usually does not want the acknowledgements.
    Paratext,
}

impl SearchFacet {
    /// Every facet, in the order the chips are shown — outermost structure first.
    pub const ALL: [SearchFacet; 7] = [
        SearchFacet::Book,
        SearchFacet::Part,
        SearchFacet::Chapter,
        SearchFacet::Scene,
        SearchFacet::Note,
        SearchFacet::Paratext,
        SearchFacet::Folder,
    ];

    /// A stable code, so a facet can cross a DTO (which carries scalars, not enums) and be
    /// persisted in `search.toml` without the numbers shifting when a variant is added.
    pub fn code(self) -> u64 {
        match self {
            SearchFacet::Book => 1,
            SearchFacet::Part => 2,
            SearchFacet::Chapter => 3,
            SearchFacet::Scene => 4,
            SearchFacet::Note => 5,
            SearchFacet::Folder => 6,
            // Appended, never inserted: the codes are persisted in `search.toml`, so
            // renumbering an existing facet would silently retarget a saved filter.
            SearchFacet::Paratext => 7,
        }
    }

    /// `None` for a code that names no facet — a stale value in a settings file must be
    /// ignored, not panic the search.
    pub fn from_code(code: u64) -> Option<Self> {
        Self::ALL.into_iter().find(|f| f.code() == code)
    }
}

/// Which chip a `(role, sub_role)` belongs under. `None` for a combination that is not in the
/// matrix at all — i.e. one that cannot exist.
pub fn search_facet_of(role: &Role, sub_role: &SubRole) -> Option<SearchFacet> {
    if !is_valid_combination(role, sub_role) {
        return None;
    }
    Some(match sub_role {
        // A book, however it is encoded: the container, and the two flat markers that stand
        // for its beginning and its end.
        SubRole::Book | SubRole::BookBegin | SubRole::BookEnd => SearchFacet::Book,
        SubRole::Part => SearchFacet::Part,
        // Both chapter encodings. `Item/ChapterScene` and `Folder/ChapterScene` differ only on
        // the UI-only `role` axis; to a reader they are the same chapter.
        SubRole::ChapterScene => SearchFacet::Chapter,
        SubRole::Scene => SearchFacet::Scene,
        SubRole::Note => SearchFacet::Note,
        // `Folder/None` is a plain folder. `Item/Text` is the inert separator the Plume
        // importer makes — no prose, no place in the book, nothing but a title. Both are
        // structure the writer put there to organise themselves, so they share a chip.
        SubRole::Paratext => SearchFacet::Paratext,
        SubRole::None | SubRole::Text => SearchFacet::Folder,
    })
}

/// The single title role a combination carries, if any (`BookTitle` / `PartTitle` /
/// `ChapterTitle`).
///
/// Two things use it. Promote carries a *name* across a type change (a chapter folder
/// that becomes a part keeps its title, it just becomes a part title). And the UI keeps
/// `BinderItem.title` — the name in the outline and on the tab — in step with the title
/// `Content` row, which is the one that actually gets compiled into the manuscript.
/// They are one title with two homes; nothing good comes of letting them drift.
pub fn title_role_of(role: &Role, sub_role: &SubRole) -> Option<ContentRole> {
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

    /// The matrix has exactly 14 rows, and `bastyde_ui` mirrors them 1:1 (one tab
    /// module per combination — see `tabs::tab_pane`). Pinned so the docs and the tab
    /// dispatch can't silently drift from the model.
    #[test]
    fn the_matrix_has_fourteen_combinations() {
        assert_eq!(COMBINATIONS.len(), 14);
    }

    /// A paratext is prose that is not the book's body: it carries its own content role,
    /// and never `SceneText`. That is what keeps it out of the word count — not one
    /// special case, but the absence of the role all four counting sites key off
    /// independently (`counts_prose`, the Overview rows, the corkboard card, and
    /// `count_words_uc`). Give it `SceneText` and every one of them starts counting the
    /// acknowledgements into the manuscript, silently.
    #[test]
    fn a_paratext_is_prose_that_is_not_the_body() {
        assert!(content_allowed(
            &Role::Item,
            &SubRole::Paratext,
            &ParatextText
        ));
        assert!(!content_allowed(
            &Role::Item,
            &SubRole::Paratext,
            &SceneText
        ));
        assert!(!counts_prose(&Role::Item, &SubRole::Paratext));
        assert!(!counts_prose(&Role::Folder, &SubRole::Paratext));

        // And no other combination carries the role.
        for c in COMBINATIONS {
            assert_eq!(
                c.allowed.contains(&ParatextText),
                c.sub_role == SubRole::Paratext && c.role == Role::Item,
                "{:?}/{:?} disagrees about carrying paratext prose",
                c.role,
                c.sub_role
            );
        }
    }

    /// The folder is organisational only — it holds nothing but a synopsis, so it can
    /// never become a second place the writer's words hide.
    #[test]
    fn a_paratext_folder_holds_only_a_synopsis() {
        assert_eq!(
            allowed_content(&Role::Folder, &SubRole::Paratext),
            &[SynopsisText]
        );
    }

    /// A paratext opens no structural level, so it takes no number and cannot disturb the
    /// numbering of the chapters around it — an interleaved preface must not renumber the
    /// book.
    #[test]
    fn a_paratext_opens_no_level() {
        assert!(!SubRole::Paratext.opens_book());
        assert!(!SubRole::Paratext.opens_part());
        assert!(!SubRole::Paratext.opens_chapter());
        assert!(!SubRole::Paratext.carries_scene());
    }

    /// An epigraph heads a **part or a chapter** — and nothing else. Four rows, and
    /// exactly four: both encodings of each of the two levels, so a Part written flat and
    /// a Part written as a folder offer the same thing.
    ///
    /// Not the Book. A book's opening quotation is a paratext item, which is truer to how
    /// a book is assembled — it is a page of its own, placed wherever the writer's
    /// tradition puts it — and it stops the epigraph having two different shapes at two
    /// different levels. A Scene, a Note and the two contentless markers get none either:
    /// no editorial convention puts an epigraph there, and giving them one would mean
    /// rewriting the hardcoded role lists in `merge_two_scenes` and `split_scene` for a
    /// placement nobody uses.
    #[test]
    fn an_epigraph_belongs_only_to_the_headed_combinations() {
        let headed = [
            (Role::Item, SubRole::Part),
            (Role::Item, SubRole::ChapterScene),
            (Role::Folder, SubRole::Part),
            (Role::Folder, SubRole::ChapterScene),
        ];
        for c in COMBINATIONS {
            let expected = headed.iter().any(|(r, s)| r == &c.role && s == &c.sub_role);
            assert_eq!(
                c.allowed.contains(&EpigraphText),
                expected,
                "{:?}/{:?} disagrees about carrying an epigraph",
                c.role,
                c.sub_role
            );
        }
        assert_eq!(
            COMBINATIONS
                .iter()
                .filter(|c| c.allowed.contains(&EpigraphText))
                .count(),
            headed.len()
        );
    }

    /// An epigraph is quoted matter, not the author's manuscript, so it must never reach
    /// the word count — which it cannot, because the count keys off `SceneText` alone.
    /// Pinned because the failure is silent: an epigraph swept into the total would
    /// inflate every pace goal and progress snapshot in the project by a few dozen words
    /// per chapter, and nothing would look wrong.
    #[test]
    fn an_epigraph_is_never_counted_as_prose() {
        assert!(!counts_prose(&Role::Folder, &SubRole::Part));
        assert!(!counts_prose(&Role::Item, &SubRole::Part));
        assert!(!counts_prose(&Role::Folder, &SubRole::Book));
        // The chapter *does* count — for its own SceneText, not for its epigraph.
        assert!(counts_prose(&Role::Folder, &SubRole::ChapterScene));
        assert!(content_allowed(
            &Role::Folder,
            &SubRole::ChapterScene,
            &EpigraphText
        ));
    }

    /// The Overview table is offered by exactly the four folder containers — and by no
    /// leaf, whatever its sub-role, since a leaf has no subtree to tabulate.
    ///
    /// The truth table is spelled out over the *whole* matrix rather than spot-checked, so
    /// a thirteenth combination cannot quietly inherit an answer nobody chose: a new row
    /// fails here until someone decides which side of the line it falls on.
    #[test]
    fn overview_is_offered_by_the_folder_containers_only() {
        let expected = |role: &Role, sub_role: &SubRole| {
            matches!(
                (role, sub_role),
                (Role::Folder, SubRole::ChapterScene)
                    | (Role::Folder, SubRole::Part)
                    | (Role::Folder, SubRole::Book)
                    | (Role::Folder, SubRole::Note)
                    | (Role::Folder, SubRole::Paratext)
            )
        };
        for c in COMBINATIONS {
            assert_eq!(
                overview_capable(&c.role, &c.sub_role),
                expected(&c.role, &c.sub_role),
                "{:?}/{:?} disagrees about offering an Overview",
                c.role,
                c.sub_role
            );
        }
        // The two exclusions that are decisions, not accidents — pinned by name so
        // flipping either has to be deliberate.
        assert!(
            !overview_capable(&Role::Folder, &SubRole::None),
            "a plain grouping folder has no structure to tabulate"
        );
        assert!(
            !overview_capable(&Role::Item, &SubRole::ChapterScene),
            "the flat chapter encoding is a leaf — it has no subtree"
        );
    }

    /// Overview is **not** the stream predicate. The two answer different questions and
    /// differ on exactly the two folders that hold a subtree without holding a manuscript
    /// extent: a notes folder and a paratext folder. Pinned because reaching for
    /// `StreamLevel::for_container` here is the obvious wrong shortcut.
    #[test]
    fn overview_and_stream_differ_only_on_the_subtree_only_folders() {
        for c in COMBINATIONS {
            let overview = overview_capable(&c.role, &c.sub_role);
            let stream = compile::StreamLevel::for_container(&c.role, &c.sub_role).is_some();
            let differs =
                (c.role == Role::Folder) && matches!(c.sub_role, SubRole::Note | SubRole::Paratext);
            assert_eq!(
                overview != stream,
                differs,
                "{:?}/{:?}: overview={overview} stream={stream}",
                c.role,
                c.sub_role
            );
        }
    }

    /// **Every** combination has a search facet. A thirteenth row added to the matrix without
    /// one would not fail to compile — it would just never appear under any chip, which the
    /// writer meets as "the search cannot find my thing" with no error anywhere.
    #[test]
    fn the_facets_cover_every_combination() {
        for c in COMBINATIONS {
            assert!(
                search_facet_of(&c.role, &c.sub_role).is_some(),
                "{:?}/{:?} is a valid combination with no search facet — it would be \
                 unfindable",
                c.role,
                c.sub_role
            );
        }
    }

    /// Twelve rows onto six chips, and **every chip is used**. A facet nothing maps to is a
    /// filter that always returns nothing — a dead chip in the UI.
    #[test]
    fn every_facet_is_reachable_from_the_matrix() {
        for facet in SearchFacet::ALL {
            assert!(
                COMBINATIONS
                    .iter()
                    .any(|c| search_facet_of(&c.role, &c.sub_role) == Some(facet)),
                "{facet:?} is a chip no combination maps to — it would always show nothing"
            );
        }
    }

    /// A chapter is a chapter in either encoding. The writer picked `chapter_mode` once, when
    /// they made the project; being asked to remember it while filtering a search would be a
    /// storage detail leaking into their afternoon.
    #[test]
    fn both_chapter_encodings_land_on_the_same_chip() {
        assert_eq!(
            search_facet_of(&Role::Item, &SubRole::ChapterScene),
            search_facet_of(&Role::Folder, &SubRole::ChapterScene),
        );
        assert_eq!(
            search_facet_of(&Role::Item, &SubRole::ChapterScene),
            Some(SearchFacet::Chapter)
        );
    }

    /// …and the same for a book: its container and its two flat markers are one book.
    #[test]
    fn every_encoding_of_a_book_lands_on_the_book_chip() {
        for (role, sub_role) in [
            (Role::Folder, SubRole::Book),
            (Role::Item, SubRole::BookBegin),
            (Role::Item, SubRole::BookEnd),
        ] {
            assert_eq!(
                search_facet_of(&role, &sub_role),
                Some(SearchFacet::Book),
                "{role:?}/{sub_role:?}"
            );
        }
    }

    /// A combination that is not in the matrix has no facet — it cannot exist, so it cannot
    /// be found.
    #[test]
    fn an_invalid_combination_has_no_facet() {
        assert_eq!(search_facet_of(&Role::Folder, &SubRole::Text), None);
        assert_eq!(search_facet_of(&Role::Folder, &SubRole::Scene), None);
    }

    /// The codes are stable and round-trip: they cross a DTO and land in a settings file, so a
    /// shifted number would silently re-point a writer's saved filter at a different chip.
    #[test]
    fn facet_codes_round_trip() {
        for facet in SearchFacet::ALL {
            assert_eq!(SearchFacet::from_code(facet.code()), Some(facet));
        }
        assert_eq!(SearchFacet::from_code(0), None);
        assert_eq!(SearchFacet::from_code(99), None, "a stale code is ignored");
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
        let leading: Vec<_> = recs.iter().take(4).copied().collect();
        assert_eq!(
            leading,
            vec![
                rec(CreateType::Chapter, Relation::Child),
                rec(CreateType::Part, Relation::Child),
                // A book's front and back matter live in a paratext folder, offered
                // ahead of the structural end marker the UI filters out anyway.
                rec(CreateType::ParatextFolder, Relation::Child),
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

    /// A scene the writer decides *is* a chapter converts in place, keeping its prose —
    /// both types carry `SceneText` + `SynopsisText`, so nothing is remapped and nothing
    /// is lost. The way back is offered too; it only costs a chapter title, and only if
    /// one was written.
    #[test]
    fn a_scene_becomes_a_flat_chapter_without_losing_its_prose() {
        use PromoteTarget as T;
        assert!(promote_targets(&Role::Item, &SubRole::Scene).contains(&T::FlatChapter));
        assert!(promote_targets(&Role::Item, &SubRole::ChapterScene).contains(&T::Scene));

        assert!(
            promote_content_loss(
                &Role::Item,
                &SubRole::ChapterScene,
                &[SceneText, SynopsisText]
            )
            .is_empty(),
            "a flat chapter keeps everything a scene can hold"
        );
        for c in [SceneText, SynopsisText] {
            assert_eq!(
                remap_content(&Role::Item, &SubRole::ChapterScene, &c),
                Some(c.clone()),
                "{c:?} must survive the raise unchanged"
            );
        }

        // Coming back down, a scene has no title role at all — so a chapter that was
        // actually named cannot silently drop it.
        assert_eq!(
            promote_content_loss(
                &Role::Item,
                &SubRole::Scene,
                &[ChapterTitle, SceneText, SynopsisText]
            ),
            vec![ChapterTitle]
        );
        // ...but an unnamed one (the caller only passes non-empty roles) converts cleanly.
        assert!(
            promote_content_loss(&Role::Item, &SubRole::Scene, &[SceneText, SynopsisText])
                .is_empty()
        );
    }

    /// Both chapter encodings land on `GoKind::Chapter` — checked role-agnostically, so
    /// this must hold whichever `role` the project's `ChapterMode` picked.
    #[test]
    fn both_chapter_encodings_are_go_chapter() {
        assert_eq!(
            go_kind_of(&Role::Item, &SubRole::ChapterScene),
            Some(GoKind::Chapter)
        );
        assert_eq!(
            go_kind_of(&Role::Folder, &SubRole::ChapterScene),
            Some(GoKind::Chapter)
        );
    }

    #[test]
    fn a_leaf_scene_is_go_scene() {
        assert_eq!(
            go_kind_of(&Role::Item, &SubRole::Scene),
            Some(GoKind::Scene)
        );
    }

    #[test]
    fn a_leaf_note_is_go_note() {
        assert_eq!(go_kind_of(&Role::Item, &SubRole::Note), Some(GoKind::Note));
    }

    /// A notes *folder* is deliberately not a Go target — there is nowhere useful for
    /// "Next Note" to land on a container, only on the actual note items inside it. This
    /// is the one place `go_kind_of` and `search_facet_of` disagree on purpose.
    #[test]
    fn a_notes_folder_is_not_a_go_target() {
        assert_eq!(go_kind_of(&Role::Folder, &SubRole::Note), None);
    }

    /// Every other structural row (a plain folder, a part, a book in either encoding, the
    /// legacy text separator) has no Go kind — the matrix's remaining eight combinations,
    /// once the four Chapter/Scene/Note rows above are excluded.
    #[test]
    fn structural_rows_have_no_go_kind() {
        for c in COMBINATIONS {
            let is_scene_or_note_leaf = matches!(
                (&c.role, &c.sub_role),
                (Role::Item, SubRole::Scene) | (Role::Item, SubRole::Note)
            );
            if c.sub_role.opens_chapter() || is_scene_or_note_leaf {
                continue;
            }
            assert_eq!(
                go_kind_of(&c.role, &c.sub_role),
                None,
                "{:?}/{:?} should have no Go kind",
                c.role,
                c.sub_role
            );
        }
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

#[cfg(test)]
mod create_type_inverse_tests {
    use super::*;

    /// `of` must be the exact inverse of `combo`, under **both** chapter modes —
    /// otherwise a caller asking "what is this row" gets a different answer than the
    /// one that created it, which is how an import ends up nesting under the wrong
    /// thing.
    #[test]
    fn every_create_type_round_trips_through_its_stored_pair() {
        for mode in [ChapterMode::Folder, ChapterMode::Flat] {
            for kind in CANONICAL {
                let (role, sub_role) = kind.combo(mode.clone());
                assert_eq!(
                    CreateType::of(&role, &sub_role),
                    Some(*kind),
                    "{kind:?} under {mode:?} came back as something else"
                );
            }
        }
    }

    /// Both chapter encodings answer `Chapter`, which is what lets `of` take no mode.
    #[test]
    fn a_chapter_is_a_chapter_whichever_way_it_is_stored() {
        assert_eq!(
            CreateType::of(&Role::Folder, &SubRole::ChapterScene),
            Some(CreateType::Chapter)
        );
        assert_eq!(
            CreateType::of(&Role::Item, &SubRole::ChapterScene),
            Some(CreateType::Chapter)
        );
    }

    /// A pair no `CreateType` creates has no answer, rather than a plausible wrong one.
    #[test]
    fn a_pair_outside_the_create_vocabulary_has_no_create_type() {
        assert_eq!(CreateType::of(&Role::Item, &SubRole::Text), None);
        assert_eq!(CreateType::of(&Role::Item, &SubRole::BookBegin), None);
    }
}
