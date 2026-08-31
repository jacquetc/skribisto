// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

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

/// The matrix has exactly 14 rows, and `teksilo_ui` mirrors them 1:1 (one tab
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
        promote_content_loss(&Role::Item, &SubRole::Scene, &[SceneText, SynopsisText]).is_empty()
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

/// The two exhaustive lists of [`CreateType`] must agree on membership.
///
/// [`CreateType::ALL`] is the set; `CANONICAL` is the ordered tail the "＋ Create"
/// menu appends after an anchor's own recommendations. A variant added to the enum
/// and to `ALL` but forgotten in `CANONICAL` is invisible in the UI — creatable by
/// no menu row anywhere — which is a silent failure the compiler cannot catch,
/// because both are hand-written arrays rather than matches.
#[test]
fn canonical_tail_covers_every_create_type() {
    let mut all = CreateType::ALL.to_vec();
    let mut canonical = super::CANONICAL.to_vec();
    let sort_key = |c: &CreateType| format!("{c:?}");
    all.sort_by_key(sort_key);
    canonical.sort_by_key(sort_key);
    assert_eq!(
        canonical, all,
        "CANONICAL (the create menu's tail) and CreateType::ALL have drifted apart — \
         a type in ALL but not CANONICAL cannot be created from any menu"
    );
}
