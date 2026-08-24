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
use common::types::EntityId;
use std::collections::HashMap;

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
/// It lived in `teksilo_ui::comments` until the document importer needed to capture
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

pub mod footnote_numbering;
/// Which ordinal each structural row carries — one pass over the whole manuscript, so
/// the exporter and the binder's live badge cannot disagree about what chapter this is.
/// Which counting unit a project starts in, from its writing language.
pub mod goal_unit;
/// A person's initials, for the short label a word processor shows beside a comment —
/// seeded once when a row is created, never recomputed over a value an editor supplied.
pub mod initials;
pub mod numbering;
/// Line a returning manuscript up against the one the project holds: which rows are the same
/// rows, who changed what, and where a chapter the editor inserted belongs.
pub mod reconcile;
/// Trigger matching for the writer's custom replacement lexicon.
pub mod replacement;
/// The bookmark names an export writes so a returning file can be recognised as *this*
/// project's — and the digest that says which side changed the prose. Shared by the exporter
/// that mints them and the scanners that read them back, because two spellings of one scheme
/// is a round trip that silently never matches.
pub mod round_trip;
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

/// The nearest row in `ancestors` whose `sub_role` `opens_book()`, if any:
/// which book-opening row, if any, encloses this destination.
///
/// `ancestors` is a destination's already-assembled ancestor chain (typically
/// the target itself, chained with `binder_ordering::ancestors_of` and
/// filtered to rows shallower than the moved/restored root's new indent; see
/// the call sites). Building that chain stays with each caller: it mixes a
/// use-case-specific `base_indent` threshold with `binder_ordering`'s
/// indent-tree walk, and `binder_ordering` is deliberately domain-blind (its
/// own doc comment on `ancestors_of` says as much) so it cannot know what a
/// Book is. This function is the domain-aware half those callers both need,
/// kept in one place instead of two: `binder_item_management::move_items_uc`
/// and `trash_management::restore_items_to_uc` both resolve a destination via
/// `binder_ordering::resolve_item_target`, and both must refuse landing a
/// book-opening row inside another book's subtree, because
/// `skribisto_model::compile`'s book-boundary walk tracks the current book
/// purely by the next `opens_book`/`closes_book` marker in flat stream order
/// (indent does not feed it). Nesting a book inside a book silently folds
/// every row still enclosed by indent in the outer book into the inner book's
/// running total instead, with no error anywhere.
///
/// Covers both book encodings (`Folder/Book` and the flat-marker
/// `Item/BookBegin`) because it goes through `opens_book()`, never a literal
/// `== SubRole::Book`.
pub fn enclosing_book(
    ancestors: impl IntoIterator<Item = EntityId>,
    sub_role: &HashMap<EntityId, SubRole>,
) -> Option<EntityId> {
    ancestors
        .into_iter()
        .find(|id| sub_role.get(id).is_some_and(SubRoleExt::opens_book))
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
mod tests;

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
