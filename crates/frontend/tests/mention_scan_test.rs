// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Integration tests for `mention_management::scan_mentions`, pinning the one property the
//! whole feature rests on: **a scan writes nothing.**
//!
//! The scan runs on every prose edit (throttled) and on every tag change. The design says a
//! *suggestion* is derived, and only becomes persisted when the writer pins it. If a scan
//! ever wrote — even something as innocent as touching `updated_at` — the consequence is not
//! a wrong roster, it is that **merely opening a project and typing marks it permanently
//! unsaved**, autosave fires forever, and every scan lands an entry on the undo stack that
//! the writer did not ask for and cannot explain. `bastyde_ui::app::mutation_origins()` turns
//! any `DirectAccess` entity event into exactly that.
//!
//! `read_only: true`, `undoable: false` and `QueryUnitOfWork` express the intent, but none of
//! them is enforced at compile time against a future edit that adds a write action to the
//! uow trait — the macros would happily generate it. The `scan_mentions_uc` header already
//! claimed "`scan_writes_no_entities` guards that" before this file existed; now it does.
//!
//! The assertion is stated as **no `DirectAccess` event of any kind**, deliberately broader
//! than the UI's mutation allowlist: mirroring that list here would mean maintaining a second
//! copy of it, and a scan has no business emitting an entity event even of a kind the UI
//! currently ignores.

use frontend::AppContext;
use frontend::commands::{
    binder_item_commands, binder_tag_commands, content_commands, mention_management_commands,
    undo_redo_commands, work_commands,
};
use frontend::common::direct_access::binder_item::BinderItemRelationshipField;
use frontend::common::direct_access::work::WorkRelationshipField;
use frontend::common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use frontend::common::event::{Event, Origin};
use frontend::common::types::EntityId;
use frontend::direct_access::{
    BinderItemRelationshipDto, CreateBinderItemDto, CreateBinderTagDto, CreateContentDto,
};
use mention_management::{MentionHit, MentionHits, ScanMentionsDto};
use work_management::{NewWorkDto, NewWorkTemplate};

/// The character the fixture is about. Capitalised, because the matcher is case-sensitive by
/// design — a lowercase needle would be a different test.
const CHARACTER: &str = "Elena";

fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

fn item(sub_role: BinderItemSubRole, title: &str) -> CreateBinderItemDto {
    CreateBinderItemDto {
        uid: common::uid::fixture_uid(
            title.len() as u64 * 1000 + title.chars().map(|c| c as u64).sum::<u64>(),
        ),
        created_at: now(),
        updated_at: now(),
        title: title.to_string(),
        sub_title: String::new(),
        role: BinderItemRole::Item,
        sub_role,
        label: String::new(),
        activated: true,
        is_favorite: false,
        is_exportable: true,
        indent: 0,
        word_count_goal: 0,
        char_count_goal: 0,
        dict_language: Vec::new(),
        aliases: Vec::new(),
        contents: vec![],
        references: vec![],
        point_of_view: vec![],
        tags: vec![],
    }
}

struct Fixture {
    ctx: AppContext,
    setup: u64,
    work: EntityId,
    /// The note the scan should find mentions *of*.
    character: EntityId,
    /// The scene the scan should find mentions *in*.
    scene: EntityId,
}

/// A project with a discoverable character note and a scene that names them — the smallest
/// shape for which a scan has anything at all to report.
fn fixture() -> Fixture {
    let ctx = AppContext::new();
    let setup = undo_redo_commands::create_new_stack(&ctx);

    let dir = std::env::temp_dir().join(format!("skrib-mention-scan-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    work_management_new(&ctx, &dir);
    let _ = std::fs::remove_dir_all(&dir);

    let work = work_commands::get_all_work(&ctx).unwrap().pop().unwrap().id;
    let binder = work_commands::get_work_relationship(&ctx, &work, &WorkRelationshipField::Binders)
        .unwrap()
        .pop()
        .unwrap();

    // The note's *title* is the full name; `Elena` alone is an alias. That split is the point
    // — the scan matches whole declared names, so a scene saying "Elena" is found through the
    // alias and not by fragment-matching the title. It is also exactly the shape the Plume
    // importer produces, where `PlumeObj.aliases` lands in this field.
    let mut character_dto = item(BinderItemSubRole::Note, &format!("{CHARACTER} Sarraute"));
    character_dto.aliases = vec![CHARACTER.to_string()];
    let created = binder_item_commands::create_binder_item_multi(
        &ctx,
        Some(setup),
        &[
            character_dto,
            item(BinderItemSubRole::Scene, "The lighthouse"),
        ],
        binder,
        -1,
    )
    .unwrap();
    let character = created[0].id;
    let scene = created[1].id;

    // A discoverable tag, on the note only: that flag is what puts the note's name into the
    // alias table at all.
    let tag = binder_tag_commands::create_orphan_binder_tag(
        &ctx,
        Some(setup),
        &CreateBinderTagDto {
            created_at: now(),
            updated_at: now(),
            name: "character".into(),
            color: "#4477aa".into(),
            details: String::new(),
            discoverable: true,
        },
    )
    .unwrap()
    .id;
    work_commands::set_work_relationship(
        &ctx,
        Some(setup),
        &frontend::direct_access::WorkRelationshipDto {
            id: work,
            field: WorkRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .unwrap();
    binder_item_commands::set_binder_item_relationship(
        &ctx,
        Some(setup),
        &BinderItemRelationshipDto {
            id: character,
            field: BinderItemRelationshipField::Tags,
            right_ids: vec![tag],
        },
    )
    .unwrap();

    content_commands::create_content_multi(
        &ctx,
        Some(setup),
        &[CreateContentDto {
            created_at: now(),
            updated_at: now(),
            activated: true,
            role: ContentRole::SceneText,
            data: format!("{CHARACTER} climbed the stair. The lamp was already lit."),
        }],
        scene,
        -1,
    )
    .unwrap();

    Fixture {
        ctx,
        setup,
        work,
        character,
        scene,
    }
}

fn work_management_new(ctx: &AppContext, dir: &std::path::Path) {
    frontend::commands::work_management_commands::new_work(
        ctx,
        &NewWorkDto {
            file_name: dir.to_string_lossy().to_string(),
            is_folder: true,
            template_kind: NewWorkTemplate::EmptyNovel,
            labels: vec![],
            language: vec!["en-US".to_string()],
            author_name: String::new(),
            chapter_scene_mode: false,
        },
    )
    .expect("new_work");
}

/// Run a scan to completion and return its hits.
///
/// `AppContext::new()` never starts the event-hub loop, so nothing competes with the test for
/// delivery and events simply accumulate in the channel — which is what makes draining it a
/// reliable record of what the scan emitted rather than a race.
fn scan(fx: &Fixture) -> Vec<MentionHit> {
    let id = mention_management_commands::scan_mentions(&fx.ctx, &ScanMentionsDto { work_id: fx.work })
        .expect("start scan");
    // Take the completion signal and release the manager lock before blocking — waiting
    // while holding it would stall every other operation query for the scan's whole
    // duration (see the qleany 1.9.0 migration guide's long-operation section).
    let completion = fx.ctx.long_operation_manager.lock().unwrap().completion_signal();
    let finished = completion.wait_for(&id, Some(std::time::Duration::from_secs(30)));
    assert!(finished, "the scan did not finish within 30s");

    let dto = mention_management_commands::get_scan_mentions_result(&fx.ctx, &id)
        .expect("scan result")
        .expect("a finished scan has a result");
    match dto.hits {
        MentionHits::Found(hits) => hits,
        MentionHits::Empty => vec![],
    }
}

fn drain(ctx: &AppContext) -> Vec<Event> {
    ctx.event_hub.subscribe_receiver().try_iter().collect()
}

fn hit_targets(hits: &[MentionHit]) -> Vec<(EntityId, EntityId)> {
    hits.iter()
        .filter_map(|h| match h {
            MentionHit::Found {
                owner_id,
                target_id,
                ..
            } => Some((*owner_id, *target_id)),
            MentionHit::Empty => None,
        })
        .collect()
}

/// The guarantee named in `scan_mentions_uc`'s header.
///
/// Note the first assertion: without it the test would pass vacuously on a scan that found
/// nothing at all — and "wrote nothing" is trivially true of a scan that did nothing. The
/// fixture is built so there is exactly one thing to find, and the test insists on finding it
/// before it is willing to conclude anything about writes.
#[test]
fn scan_writes_no_entities() {
    let fx = fixture();
    let before_stack = undo_redo_commands::get_stack_size(&fx.ctx, fx.setup);
    let before_items = binder_item_commands::get_binder_item_multi(&fx.ctx, &[fx.character, fx.scene])
        .expect("read items before");

    // Discard everything the fixture itself emitted, so the drain below sees only the scan.
    let _ = drain(&fx.ctx);

    let hits = scan(&fx);

    assert_eq!(
        hit_targets(&hits),
        vec![(fx.scene, fx.character)],
        "the scene names the character exactly once, so there is exactly one hit to find — \
         if this fails the rest of the test proves nothing"
    );
    // Found through the alias, not the title — so this fixture exercises the path an imported
    // Plume project depends on, and a regression that dropped aliases would fail here rather
    // than silently reduce the scan to title-only matching.
    match &hits[0] {
        MentionHit::Found {
            matched_name,
            is_title_match,
            hit_count,
            evidence,
            ..
        } => {
            assert_eq!(matched_name, CHARACTER, "matched via the alias");
            assert!(!is_title_match, "the title is the full name, not the alias");
            assert_eq!(*hit_count, 1);
            assert!(
                evidence.contains(CHARACTER),
                "the evidence sentence is what lets a writer judge a suggestion; got {evidence:?}"
            );
        }
        MentionHit::Empty => unreachable!("asserted non-empty above"),
    }

    // The actual guarantee. A write of any entity — even one the UI currently ignores —
    // announces itself here.
    let emitted = drain(&fx.ctx);
    let entity_events: Vec<&Event> = emitted
        .iter()
        .filter(|e| matches!(e.origin, Origin::DirectAccess(_)))
        .collect();
    assert!(
        entity_events.is_empty(),
        "a scan must write nothing, but it emitted: {:?}",
        entity_events
            .iter()
            .map(|e| e.origin_string())
            .collect::<Vec<_>>()
    );

    assert_eq!(
        undo_redo_commands::get_stack_size(&fx.ctx, fx.setup),
        before_stack,
        "a scan must not land an undo entry the writer cannot explain"
    );

    // `updated_at` is the field a careless write touches even when the payload is unchanged,
    // and it is what `content_fingerprint` reads — so a scan that bumped it would make every
    // backup think the project had changed.
    let after_items = binder_item_commands::get_binder_item_multi(&fx.ctx, &[fx.character, fx.scene])
        .expect("read items after");
    let stamps = |v: &Vec<Option<frontend::direct_access::BinderItemDto>>| {
        v.iter()
            .map(|i| i.as_ref().map(|i| i.updated_at))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        stamps(&after_items),
        stamps(&before_items),
        "a scan must not touch updated_at — backup's skip-if-unchanged reads it"
    );

    // Prove the detector is armed. Everything above is an assertion that something did *not*
    // happen, which would also hold if `drain` were watching the wrong channel or if entity
    // writes stopped emitting events — either would turn this whole test green and useless.
    // So: do a real write and require that it *is* seen.
    binder_item_commands::set_binder_item_relationship(
        &fx.ctx,
        Some(fx.setup),
        &BinderItemRelationshipDto {
            id: fx.scene,
            field: BinderItemRelationshipField::References,
            right_ids: vec![fx.character],
        },
    )
    .expect("a write the detector must catch");
    assert!(
        drain(&fx.ctx)
            .iter()
            .any(|e| matches!(e.origin, Origin::DirectAccess(_))),
        "the no-write assertions above only mean something if a real write would have \
         tripped them — this one did not, so they prove nothing"
    );
}

/// Pinning is what *does* write, and it is the writer's decision — so the same scan that must
/// not write on its own must report an already-pinned reference as confirmed.
///
/// This is the other half of the contract: proving a scan writes nothing is only reassuring
/// if a scan still reflects what the writer has persisted.
#[test]
fn a_pinned_reference_comes_back_confirmed() {
    let fx = fixture();

    // Stated as "exactly one row, and it is a suggestion" rather than "no confirmed rows":
    // the latter is also true of an empty scan, which would let this test pass while proving
    // nothing — the trap the sibling test's first assertion exists to catch.
    let before = scan(&fx);
    assert_eq!(
        hit_targets(&before),
        vec![(fx.scene, fx.character)],
        "one row before pinning"
    );
    assert!(
        matches!(
            before[0],
            MentionHit::Found {
                is_confirmed: false,
                ..
            }
        ),
        "nothing is pinned yet, so the row must be a suggestion"
    );

    // What the pin control does: a plain, undoable relationship write.
    let stack = undo_redo_commands::create_new_stack(&fx.ctx);
    binder_item_commands::set_binder_item_relationship(
        &fx.ctx,
        Some(stack),
        &BinderItemRelationshipDto {
            id: fx.scene,
            field: BinderItemRelationshipField::References,
            right_ids: vec![fx.character],
        },
    )
    .expect("pin");

    let hits = scan(&fx);
    assert_eq!(
        hits.iter()
            .filter(|h| matches!(h, MentionHit::Found { is_confirmed: true, .. }))
            .count(),
        1,
        "the pinned reference must come back confirmed, not as a fresh suggestion"
    );
    assert_eq!(
        hit_targets(&hits),
        vec![(fx.scene, fx.character)],
        "confirmed and suggested are the same row seen twice, not two rows"
    );
}

/// A scan of a project with no discoverable tag has nothing to match against — and must still
/// write nothing rather than, say, clearing stale state.
#[test]
fn a_project_with_nothing_discoverable_scans_clean() {
    let fx = fixture();
    // Un-discover the only tag.
    let work = work_commands::get_all_work(&fx.ctx).unwrap().pop().unwrap().id;
    let tag = work_commands::get_work_relationship(&fx.ctx, &work, &WorkRelationshipField::Tags)
        .unwrap()
        .pop()
        .unwrap();
    let row = binder_tag_commands::get_binder_tag(&fx.ctx, &tag)
        .expect("read tag")
        .expect("the fixture's tag is still there");
    let mut dto: frontend::direct_access::UpdateBinderTagDto = row.into();
    dto.discoverable = false;
    binder_tag_commands::update_binder_tag(&fx.ctx, Some(fx.setup), &dto).expect("update");

    let _ = drain(&fx.ctx);
    assert!(
        scan(&fx).is_empty(),
        "no discoverable tag means no alias table means no hits"
    );
    let emitted = drain(&fx.ctx);
    assert!(
        emitted
            .iter()
            .all(|e| !matches!(e.origin, Origin::DirectAccess(_))),
        "an empty scan must be as write-free as a full one"
    );
}
