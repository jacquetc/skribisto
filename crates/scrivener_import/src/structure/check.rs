// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! The invariants every plan holds before it leaves [`plan`](super::plan)
//! (D3.2, as the critique amended it).
//!
//! The checker is independent of the mapper: it reads only the plan, the model
//! and the measured facts, never the mapper's state and never one of its
//! helpers, so a rule the mapper gets wrong cannot also blind the check. Where
//! each item went is read back from the rows here, not by the function the
//! mapper wrote them with; a reason an item has no row is held against the
//! project and the writer's choices, not taken on the plan's word. It stops at
//! the first violation, which names the row or the source concerned.
//!
//! Every pass is linear in the size of the binder and of the plan: the review
//! re-plans, and so re-checks, on every choice.

use std::collections::{HashMap, HashSet};

use common::entities::BinderItemRole as Role;
use common::entities::BinderItemSubRole as SubRole;
use common::entities::ContentRole;
use serde::{Deserialize, Serialize};

use super::{
    Absence, BinderKind, Body, Destination, Origin, Placement, PlannedRow, PlannedTemplate, RowRef,
    Source, StructureNotice, StructurePlan, TrashChoice, Why,
};
use crate::content::ContentFacts;
use crate::model::{ItemKind, Project};

/// An invariant a plan broke.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Violation {
    /// A `(role, sub_role)` outside the writing model's matrix, or a content
    /// that combination does not allow.
    Combination { binder: BinderKind, row: u32 },
    /// Two contents of one role on one row.
    DuplicateContent { binder: BinderKind, row: u32 },
    /// A title body outside the title roles, or a title role with prose.
    TitleBody { binder: BinderKind, row: u32 },
    /// A binder whose first row is not at depth 0.
    FirstRowIndented { binder: BinderKind },
    /// A row more than one level deeper than the row above it.
    IndentJump { binder: BinderKind, row: u32 },
    /// A row under an item, which cannot hold children.
    UnderAnItem { binder: BinderKind, row: u32 },
    /// A note that would export.
    NoteExported { binder: BinderKind, row: u32 },
    /// A book, part or chapter inside one at its own level or below it.
    LevelInversion { row: u32 },
    /// A row that can export, outside every book.
    OutsideBook { row: u32 },
    /// A book end with no book open.
    StrayBookEnd { row: u32 },
    /// A book opened while another is open, or never closed.
    BookNotClosed { row: u32 },
    /// A deleted row outside the trash, or a trashed row that is not deleted.
    Activation { binder: BinderKind, row: u32 },
    /// A notes-binder row that is not a note.
    NotANote { row: u32 },
    /// A trash entry that does not open a deleted subtree at the top level.
    TrashRoot { row: u32 },
    /// A tag on something other than a note entry, or on a story-bible
    /// folder's own text.
    TagMisplaced { binder: BinderKind, row: u32 },
    /// A tag whose folder is not a live notes folder.
    TagFolder { tag: u32 },
    /// A source naming nothing in the project.
    SourceOutOfRange { source: Source },
    /// Writing that carries something and lands nowhere, unreported.
    NotPlaced { source: Source },
    /// Writing that lands twice.
    PlacedTwice { source: Source },
    /// An item left out with no notice saying so.
    LeftOutUnreported { item: u32 },
    /// An item represented by two rows.
    ItemTwice { item: u32 },
    /// An item with no row, no template and no reason.
    ItemUnaccounted { item: u32 },
    /// The placements say something the rows do not.
    PlacementsDisagree { item: u32 },
    /// Siblings out of their order.
    OrderChanged { binder: BinderKind, row: u32 },
    /// An item whose parent has a row, not under that row.
    SubtreeSplit { item: u32 },
    /// A source in a content, or on a row, it does not belong to: a synopsis
    /// written as prose, document notes folded into a scene, a picture on a
    /// row that prints, one item's writing on another item's row.
    Misrouted {
        binder: BinderKind,
        row: u32,
        source: Source,
    },
    /// A note template whose body is not its own item's text.
    TemplateBody { template: u32 },
    /// Inside a notes or a paratext folder of the manuscript, a row of another
    /// kind: the export reads the stream, not the folders, so it would print.
    ModeBroken { row: u32 },
    /// An item with no row for a reason the project and the writer's choices
    /// do not give.
    AbsenceUnfounded { item: u32 },
    /// An item the writer left out, brought across all the same.
    LeftOutKept { item: u32 },
    /// An item of the Trash brought across live, or an item outside it deleted.
    TrashMismatch { item: u32 },
    /// A card image reported left behind on an item whose note could hold it.
    CardLeftOnSheet { item: u32 },
}

fn rank(row: &PlannedRow) -> Option<u8> {
    match (&row.role, &row.sub_role) {
        (Role::Folder, SubRole::Book) | (Role::Item, SubRole::BookBegin) => Some(0),
        (_, SubRole::Part) => Some(1),
        (_, SubRole::ChapterScene) => Some(2),
        _ => None,
    }
}

/// Rows the export reads, which must sit inside a book to reach it (C2 T17).
fn can_export(row: &PlannedRow) -> bool {
    matches!(
        (&row.role, &row.sub_role),
        (Role::Item, SubRole::Scene)
            | (_, SubRole::ChapterScene)
            | (_, SubRole::Part)
            | (Role::Item, SubRole::Paratext)
    )
}

fn is_title_role(role: &ContentRole) -> bool {
    matches!(
        role,
        ContentRole::BookTitle | ContentRole::PartTitle | ContentRole::ChapterTitle
    )
}

/// One past the last row of each row's subtree in a planned binder.
fn subtree_ends(rows: &[PlannedRow]) -> Vec<u32> {
    let mut end = vec![rows.len() as u32; rows.len()];
    let mut open: Vec<usize> = Vec::new();
    for (i, row) in rows.iter().enumerate() {
        while let Some(&top) = open.last() {
            if rows[top].depth >= row.depth {
                end[top] = i as u32;
                open.pop();
            } else {
                break;
            }
        }
        open.push(i);
    }
    end
}

/// Check every invariant of `plan`, mapped from `project` with `facts`.
pub fn check(
    project: &Project,
    facts: &ContentFacts,
    plan: &StructurePlan,
) -> Result<(), Violation> {
    let binders = [
        (BinderKind::Manuscript, plan.manuscript.as_slice()),
        (BinderKind::Notes, plan.notes.as_slice()),
    ];
    for (binder, rows) in binders {
        rows_shape(binder, rows, plan.tags.len())?;
    }
    manuscript(&plan.manuscript)?;
    modes(&plan.manuscript)?;
    notes(plan)?;
    tags(plan)?;
    routing(plan)?;
    conservation(project, facts, plan)?;
    items(project, plan)?;
    absences(project, plan)?;
    order(project, plan)?;
    stream_order(project, plan)
}

/// One past the last row of each item's subtree in the source binder.
fn source_ends(project: &Project) -> Vec<u32> {
    let n = project.binder.len();
    let mut end = vec![n as u32; n];
    let mut open: Vec<usize> = Vec::new();
    for (i, row) in project.binder.iter().enumerate() {
        while let Some(&top) = open.last() {
            if project.binder[top].depth >= row.depth {
                end[top] = i as u32;
                open.pop();
            } else {
                break;
            }
        }
        open.push(i);
    }
    end
}

/// The top-level item each item sits under: itself for a top-level one.
fn roots_of(project: &Project) -> Vec<u32> {
    let mut root = 0u32;
    project
        .binder
        .iter()
        .enumerate()
        .map(|(i, row)| {
            if row.depth == 0 {
                root = i as u32;
            }
            root
        })
        .collect()
}

/// Inside a notes folder of the manuscript only notes; inside a paratext
/// folder only paratext and notes (D3.7). A scene under a notes folder is
/// prose the export prints, whatever the folder says.
fn modes(rows: &[PlannedRow]) -> Result<(), Violation> {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    enum Mode {
        Free,
        Paratext,
        Note,
    }
    let mut open: Vec<(u16, Mode)> = Vec::new();
    for (r, row) in rows.iter().enumerate() {
        while open.last().is_some_and(|(d, _)| *d >= row.depth) {
            open.pop();
        }
        let inherited = open.last().map_or(Mode::Free, |(_, m)| *m);
        let allowed = match inherited {
            Mode::Free => true,
            Mode::Paratext => matches!(row.sub_role, SubRole::Paratext | SubRole::Note),
            Mode::Note => row.sub_role == SubRole::Note,
        };
        if !allowed {
            return Err(Violation::ModeBroken { row: r as u32 });
        }
        let own = match (&row.role, &row.sub_role) {
            (Role::Folder, SubRole::Note) => Mode::Note,
            (Role::Folder, SubRole::Paratext) => Mode::Paratext,
            _ => Mode::Free,
        };
        open.push((row.depth, inherited.max(own)));
    }
    Ok(())
}

/// Whether a row of this origin speaks for `item`'s own writing: the row that
/// represents it, its own-text child, the Draft's note, a template's leftovers.
fn speaks_for(origin: &Origin, item: u32) -> bool {
    match *origin {
        Origin::Item { item: o }
        | Origin::OwnText { item: o }
        | Origin::DraftContent { item: o }
        | Origin::TemplateLeftover { item: o } => o == item,
        _ => false,
    }
}

/// Every source sits in a content of its kind, on a row of its own item: a
/// synopsis only as a synopsis, document notes only on their notes row, a
/// picture or file only where no export prints it as prose, a card image only
/// in a note, each part of a split text on its own chapter.
fn routing(plan: &StructurePlan) -> Result<(), Violation> {
    let binders = [
        (BinderKind::Manuscript, plan.manuscript.as_slice()),
        (BinderKind::Notes, plan.notes.as_slice()),
    ];
    for (binder, rows) in binders {
        for (r, row) in rows.iter().enumerate() {
            for content in &row.contents {
                let Body::From(sources) = &content.body else {
                    continue;
                };
                let role = &content.role;
                for source in sources {
                    let ok = match *source {
                        Source::Text { item } => {
                            *role != ContentRole::SynopsisText && speaks_for(&row.origin, item)
                        }
                        Source::TextPart { item, part } => {
                            *role != ContentRole::SynopsisText
                                && match row.origin {
                                    Origin::Item { item: o } => o == item,
                                    Origin::SplitPart { item: o, part: p } => {
                                        o == item && p == part
                                    }
                                    _ => false,
                                }
                        }
                        Source::Notes { item } => {
                            *role == ContentRole::NoteText
                                && row.origin == Origin::DocumentNotes { item }
                        }
                        Source::Synopsis { item } => {
                            *role == ContentRole::SynopsisText && speaks_for(&row.origin, item)
                        }
                        Source::Media { item } => {
                            matches!(role, ContentRole::NoteText | ContentRole::ParatextText)
                                && speaks_for(&row.origin, item)
                        }
                        Source::CardImage { item } => {
                            *role == ContentRole::NoteText && speaks_for(&row.origin, item)
                        }
                        Source::ProjectNote { note } => {
                            *role == ContentRole::NoteText
                                && row.origin == Origin::ProjectNote { note }
                        }
                        Source::Recovered { text } => {
                            *role == ContentRole::NoteText
                                && row.origin == Origin::RecoveredText { text }
                        }
                    };
                    if !ok {
                        return Err(Violation::Misrouted {
                            binder,
                            row: r as u32,
                            source: *source,
                        });
                    }
                }
            }
        }
    }
    for (t, template) in plan.templates.iter().enumerate() {
        if template.body.is_some_and(|body| {
            body != Source::Text {
                item: template.item,
            }
        }) {
            return Err(Violation::TemplateBody { template: t as u32 });
        }
    }
    Ok(())
}

fn rows_shape(binder: BinderKind, rows: &[PlannedRow], tags: usize) -> Result<(), Violation> {
    let mut previous: Option<&PlannedRow> = None;
    for (r, row) in rows.iter().enumerate() {
        let r32 = r as u32;
        let present: Vec<ContentRole> = row.contents.iter().map(|c| c.role.clone()).collect();
        skribisto_model::validate_item(&row.role, &row.sub_role, &present)
            .map_err(|_| Violation::Combination { binder, row: r32 })?;
        for (k, role) in present.iter().enumerate() {
            if present[..k].contains(role) {
                return Err(Violation::DuplicateContent { binder, row: r32 });
            }
        }
        for content in &row.contents {
            let title_body = matches!(content.body, Body::Title);
            if title_body != is_title_role(&content.role) {
                return Err(Violation::TitleBody { binder, row: r32 });
            }
        }
        match previous {
            None if row.depth != 0 => return Err(Violation::FirstRowIndented { binder }),
            Some(prev) if row.depth > prev.depth.saturating_add(1) => {
                return Err(Violation::IndentJump { binder, row: r32 });
            }
            Some(prev) if row.depth > prev.depth && prev.role == Role::Item => {
                return Err(Violation::UnderAnItem { binder, row: r32 });
            }
            _ => {}
        }
        if row.sub_role == SubRole::Note && row.exportable {
            return Err(Violation::NoteExported { binder, row: r32 });
        }
        if let Some(tag) = row.tag
            && (tag as usize >= tags
                || binder != BinderKind::Notes
                || row.role != Role::Item
                || row.sub_role != SubRole::Note)
        {
            return Err(Violation::TagMisplaced { binder, row: r32 });
        }
        previous = Some(row);
    }
    Ok(())
}

fn manuscript(rows: &[PlannedRow]) -> Result<(), Violation> {
    // Book, part and chapter strictly deepen along the nesting.
    let mut stack: Vec<(u16, Option<u8>)> = Vec::new();
    for (r, row) in rows.iter().enumerate() {
        while stack.last().is_some_and(|(d, _)| *d >= row.depth) {
            stack.pop();
        }
        let own = rank(row);
        if let Some(own) = own
            && let Some(above) = stack.iter().rev().find_map(|(_, rank)| *rank)
            && above >= own
        {
            return Err(Violation::LevelInversion { row: r as u32 });
        }
        stack.push((row.depth, own));
        if !row.activated {
            return Err(Violation::Activation {
                binder: BinderKind::Manuscript,
                row: r as u32,
            });
        }
    }
    // Every row that can export sits inside a book, in stream order.
    let mut open = false;
    for (r, row) in rows.iter().enumerate() {
        let r = r as u32;
        if matches!(row.sub_role, SubRole::Book | SubRole::BookBegin) {
            if open {
                return Err(Violation::BookNotClosed { row: r });
            }
            open = true;
        } else if row.sub_role == SubRole::BookEnd {
            if !open {
                return Err(Violation::StrayBookEnd { row: r });
            }
            open = false;
        } else if can_export(row) && !open {
            return Err(Violation::OutsideBook { row: r });
        }
    }
    if open {
        return Err(Violation::BookNotClosed {
            row: rows.len() as u32,
        });
    }
    Ok(())
}

fn notes(plan: &StructurePlan) -> Result<(), Violation> {
    let rows = &plan.notes;
    for (r, row) in rows.iter().enumerate() {
        if row.sub_role != SubRole::Note {
            return Err(Violation::NotANote { row: r as u32 });
        }
    }
    let ends = subtree_ends(rows);
    let mut trashed = vec![false; rows.len()];
    for &root in &plan.trash_roots {
        let Some(row) = rows.get(root as usize) else {
            return Err(Violation::TrashRoot { row: root });
        };
        if row.depth != 0 || row.activated {
            return Err(Violation::TrashRoot { row: root });
        }
        for flag in &mut trashed[root as usize..ends[root as usize] as usize] {
            *flag = true;
        }
    }
    for (r, row) in rows.iter().enumerate() {
        if row.activated == trashed[r] {
            return Err(Violation::Activation {
                binder: BinderKind::Notes,
                row: r as u32,
            });
        }
    }
    Ok(())
}

fn tags(plan: &StructurePlan) -> Result<(), Violation> {
    for (t, tag) in plan.tags.iter().enumerate() {
        let ok = plan.notes.get(tag.creates_in as usize).is_some_and(|row| {
            row.role == Role::Folder
                && row.activated
                && row.origin == Origin::Item { item: tag.folder }
        });
        if !ok {
            return Err(Violation::TagFolder { tag: t as u32 });
        }
    }
    // A story-bible folder's own text is not an entry: tagged, the folder's
    // name would become a word the mention index looks for.
    let folders: HashSet<u32> = plan.tags.iter().map(|t| t.folder).collect();
    for (r, row) in plan.notes.iter().enumerate() {
        if let (Some(_), Origin::OwnText { item }) = (row.tag, row.origin)
            && folders.contains(&item)
        {
            return Err(Violation::TagMisplaced {
                binder: BinderKind::Notes,
                row: r as u32,
            });
        }
    }
    Ok(())
}

fn in_range(project: &Project, facts: &ContentFacts, source: &Source) -> bool {
    let items = project.binder.len() as u32;
    match *source {
        Source::Text { item }
        | Source::TextPart { item, .. }
        | Source::Notes { item }
        | Source::Synopsis { item }
        | Source::Media { item }
        | Source::CardImage { item } => item < items,
        Source::ProjectNote { note } => (note as usize) < facts.project_notes.len(),
        Source::Recovered { text } => (text as usize) < facts.recovered.len(),
    }
}

/// Every piece of writing that carries anything lands exactly once, or is
/// reported as left behind.
fn conservation(
    project: &Project,
    facts: &ContentFacts,
    plan: &StructurePlan,
) -> Result<(), Violation> {
    let mut placed: HashMap<Source, u32> = HashMap::new();
    let bodies = plan
        .manuscript
        .iter()
        .chain(&plan.notes)
        .flat_map(|row| &row.contents)
        .filter_map(|c| match &c.body {
            Body::From(sources) => Some(sources.iter().copied()),
            Body::Title => None,
        })
        .flatten()
        .chain(plan.templates.iter().filter_map(|t| t.body));
    for source in bodies {
        if !in_range(project, facts, &source) {
            return Err(Violation::SourceOutOfRange { source });
        }
        let count = placed.entry(source).or_default();
        *count += 1;
        if *count > 1 {
            return Err(Violation::PlacedTwice { source });
        }
    }
    let split: Vec<bool> = {
        let mut split = vec![false; project.binder.len()];
        for source in placed.keys() {
            if let Source::TextPart { item, .. } = source
                && let Some(flag) = split.get_mut(*item as usize)
            {
                *flag = true;
            }
        }
        split
    };
    // What the notices report, gathered once: looking each item up in the
    // list of notices costs the square of a large binder.
    let n = project.binder.len();
    let mut cards_left: HashSet<u32> = HashSet::new();
    let mut trash_left_out = false;
    let mut left_out_runs = vec![0i64; n + 1];
    for notice in &plan.notices {
        match *notice {
            StructureNotice::CardImageLeftBehind { item } => {
                cards_left.insert(item);
            }
            StructureNotice::TrashLeftOut { .. } => trash_left_out = true,
            StructureNotice::LeftOut { item, items } => {
                let start = (item as usize).min(n);
                let end = (item as usize).saturating_add(items as usize).min(n);
                if start < end {
                    left_out_runs[start] += 1;
                    left_out_runs[end] -= 1;
                }
            }
            _ => {}
        }
    }
    let mut left_out = vec![false; n];
    let mut depth = 0i64;
    for (i, flag) in left_out.iter_mut().enumerate() {
        depth += left_out_runs[i];
        *flag = depth > 0;
    }
    let need = |source: Source| {
        if placed.contains_key(&source) {
            Ok(())
        } else {
            Err(Violation::NotPlaced { source })
        }
    };
    for (i, row) in project.binder.iter().enumerate() {
        let item = i as u32;
        let absence = plan.placements.get(i).and_then(|p| p.absence);
        match absence {
            Some(Absence::TrashLeftOut) => {
                if !trash_left_out {
                    return Err(Violation::LeftOutUnreported { item });
                }
                continue;
            }
            Some(Absence::LeftOut) => {
                if !left_out[i] {
                    return Err(Violation::LeftOutUnreported { item });
                }
                continue;
            }
            _ => {}
        }
        let f = &facts.items[i];
        if f.text.carries.counts() {
            if split[i] {
                if placed.contains_key(&Source::Text { item }) {
                    return Err(Violation::PlacedTwice {
                        source: Source::Text { item },
                    });
                }
                if f.text.lead.counts() {
                    need(Source::TextPart { item, part: 0 })?;
                }
                for part in 1..=f.text.headings.len() as u32 {
                    need(Source::TextPart { item, part })?;
                }
            } else {
                need(Source::Text { item })?;
            }
        }
        if f.notes.counts() {
            need(Source::Notes { item })?;
        }
        if f.synopsis.counts() {
            need(Source::Synopsis { item })?;
        }
        if row.payload.file.is_some() {
            need(Source::Media { item })?;
        }
        if row.payload.card_image.is_some() && !placed.contains_key(&Source::CardImage { item }) {
            if !cards_left.contains(&item) {
                return Err(Violation::NotPlaced {
                    source: Source::CardImage { item },
                });
            }
            // Left behind only where it would print: a note holds it (X-32).
            let on_a_note = plan
                .placements
                .get(i)
                .and_then(|p| p.item)
                .and_then(|at| row_at(plan, at))
                .is_some_and(|r| r.sub_role == SubRole::Note);
            if on_a_note {
                return Err(Violation::CardLeftOnSheet { item });
            }
        }
    }
    for (k, carries) in facts.project_notes.iter().enumerate() {
        if carries.counts() {
            need(Source::ProjectNote { note: k as u32 })?;
        }
    }
    for (k, carries) in facts.recovered.iter().enumerate() {
        if carries.counts() {
            need(Source::Recovered { text: k as u32 })?;
        }
    }
    Ok(())
}

fn row_at(plan: &StructurePlan, at: RowRef) -> Option<&PlannedRow> {
    match at.binder {
        BinderKind::Manuscript => plan.manuscript.get(at.row as usize),
        BinderKind::Notes => plan.notes.get(at.row as usize),
    }
}

/// Where each item went, read back from the rows (D3.21): the first row that
/// represents it, the first row and content holding its text or any part of
/// it, the first rows holding its notes and its synopsis, the template it
/// became, and the plan's reason when it has no row (which [`absences`]
/// holds against the project).
fn read_back(
    items: usize,
    plan: &StructurePlan,
    templates: &[PlannedTemplate],
    absence: &[Option<Absence>],
) -> Vec<Placement> {
    let mut out = vec![Placement::default(); items];
    let binders = [
        (BinderKind::Manuscript, plan.manuscript.as_slice()),
        (BinderKind::Notes, plan.notes.as_slice()),
    ];
    for (binder, rows) in binders {
        for (r, row) in rows.iter().enumerate() {
            let at = RowRef {
                binder,
                row: r as u32,
            };
            if let Origin::Item { item } = row.origin
                && let Some(p) = out.get_mut(item as usize)
            {
                p.item.get_or_insert(at);
            }
            for content in &row.contents {
                let Body::From(sources) = &content.body else {
                    continue;
                };
                for source in sources {
                    let (item, slot) = match *source {
                        Source::Text { item } | Source::TextPart { item, .. } => (item, 0),
                        Source::Notes { item } => (item, 1),
                        Source::Synopsis { item } => (item, 2),
                        _ => continue,
                    };
                    let Some(p) = out.get_mut(item as usize) else {
                        continue;
                    };
                    match slot {
                        0 => {
                            p.text.get_or_insert((at, content.role.clone()));
                        }
                        1 => {
                            p.notes.get_or_insert(at);
                        }
                        _ => {
                            p.synopsis.get_or_insert(at);
                        }
                    }
                }
            }
        }
    }
    for (k, template) in templates.iter().enumerate() {
        if let Some(p) = out.get_mut(template.item as usize) {
            p.template = Some(k as u32);
        }
    }
    for (p, reason) in out.iter_mut().zip(absence) {
        if p.item.is_none() {
            p.absence = *reason;
        }
    }
    out
}

/// Every item is represented once: by a row, a note template, or a reason.
fn items(project: &Project, plan: &StructurePlan) -> Result<(), Violation> {
    let n = project.binder.len();
    let mut seen = vec![false; n];
    for row in plan.manuscript.iter().chain(&plan.notes) {
        if let Origin::Item { item } = row.origin {
            let Some(flag) = seen.get_mut(item as usize) else {
                return Err(Violation::ItemUnaccounted { item });
            };
            if *flag {
                return Err(Violation::ItemTwice { item });
            }
            *flag = true;
        }
    }
    if plan.placements.len() != n {
        return Err(Violation::PlacementsDisagree { item: n as u32 });
    }
    let absences: Vec<Option<Absence>> = plan.placements.iter().map(|p| p.absence).collect();
    let expected = read_back(n, plan, &plan.templates, &absences);
    for i in 0..n {
        let item = i as u32;
        if expected[i] != plan.placements[i] {
            return Err(Violation::PlacementsDisagree { item });
        }
        let p = &plan.placements[i];
        if !seen[i] && p.template.is_none() && p.absence.is_none() {
            return Err(Violation::ItemUnaccounted { item });
        }
    }
    Ok(())
}

/// Siblings keep their order, and a child whose parent has a row sits under it.
fn order(project: &Project, plan: &StructurePlan) -> Result<(), Violation> {
    let parents = project.parents();
    let mut at: HashMap<u32, (BinderKind, u32)> = HashMap::new();
    let binders = [
        (BinderKind::Manuscript, plan.manuscript.as_slice()),
        (BinderKind::Notes, plan.notes.as_slice()),
    ];
    for (binder, rows) in binders {
        for (r, row) in rows.iter().enumerate() {
            if let Origin::Item { item } = row.origin {
                at.insert(item, (binder, r as u32));
            }
        }
    }
    for (binder, rows) in binders {
        let ends = subtree_ends(rows);
        let mut stack: Vec<usize> = Vec::new();
        // The last item placed under each planned parent, per source parent.
        // Root front and back matter are two groups of their own: front matter
        // opens the book and back matter closes it, whatever their root order.
        type Group = (Option<usize>, Option<usize>, Option<bool>);
        let mut last: HashMap<Group, u32> = HashMap::new();
        for (r, row) in rows.iter().enumerate() {
            while stack
                .last()
                .is_some_and(|&top| rows[top].depth >= row.depth)
            {
                stack.pop();
            }
            let planned_parent = stack.last().copied();
            stack.push(r);
            let Origin::Item { item } = row.origin else {
                continue;
            };
            let source_parent = parents.get(item as usize).copied().flatten();
            let matter = match row.why {
                Why::Matter { back } => Some(back),
                _ => None,
            };
            let key = (planned_parent, source_parent, matter);
            if let Some(previous) = last.get(&key)
                && *previous >= item
            {
                return Err(Violation::OrderChanged {
                    binder,
                    row: r as u32,
                });
            }
            last.insert(key, item);
            if let Some(p) = source_parent
                && let Some(&(parent_binder, parent_row)) = at.get(&(p as u32))
            {
                let inside = parent_binder == binder
                    && parent_row < r as u32
                    && (r as u32) < ends[parent_row as usize];
                if !inside {
                    return Err(Violation::SubtreeSplit { item });
                }
            }
        }
    }
    Ok(())
}

/// Every reason for having no row is one the project and the writer's choices
/// give (D3.16, D3.17, D3.20), and every choice to leave something out is kept:
/// the Draft, the Trash and the Templates folder stand for what they hold; a
/// template's items became templates; an item is left out only inside a root
/// the writer left out, or inside the Trash the writer left out. An item of an
/// imported Trash is a deleted row, and nothing else is.
fn absences(project: &Project, plan: &StructurePlan) -> Result<(), Violation> {
    let ends = source_ends(project);
    let roots = roots_of(project);
    let within = |item: u32, ancestor: u32| {
        item >= ancestor && ends.get(ancestor as usize).is_some_and(|end| item < *end)
    };
    let templates_folder = project.template_folder.and_then(|key| {
        project
            .binder
            .iter()
            .position(|row| row.owns_key && row.key == Some(key))
            .map(|i| i as u32)
    });
    // Scrivener has one Trash: the first at the top level, as the mapper reads it.
    let trash = project
        .binder
        .iter()
        .position(|row| row.depth == 0 && row.kind == ItemKind::TrashFolder)
        .map(|i| i as u32);
    let trash_left_out = plan.choices.trash == TrashChoice::LeaveOut;
    let left_out_root = |item: u32| {
        roots
            .get(item as usize)
            .is_some_and(|root| plan.choices.roots.get(root) == Some(&Destination::LeaveOut))
    };
    for (i, row) in project.binder.iter().enumerate() {
        let item = i as u32;
        let Some(placement) = plan.placements.get(i) else {
            continue;
        };
        let founded = match placement.absence {
            None => true,
            Some(Absence::SpecialFolder) => {
                (row.depth == 0
                    && matches!(row.kind, ItemKind::DraftFolder | ItemKind::TrashFolder))
                    || templates_folder == Some(item)
            }
            Some(Absence::Template) => {
                templates_folder.is_some_and(|t| item > t && within(item, t))
            }
            Some(Absence::LeftOut) => left_out_root(item),
            Some(Absence::TrashLeftOut) => trash_left_out && trash.is_some_and(|t| within(item, t)),
        };
        if !founded {
            return Err(Violation::AbsenceUnfounded { item });
        }
        let gone = placement.item.is_none() && placement.template.is_none();
        if left_out_root(item) && !(gone && placement.absence == Some(Absence::LeftOut)) {
            return Err(Violation::LeftOutKept { item });
        }
        let Some(t) = trash.filter(|t| item > *t && within(item, *t)) else {
            continue;
        };
        let deleted_row = placement
            .item
            .filter(|at| at.binder == BinderKind::Notes)
            .and_then(|at| row_at(plan, at))
            .is_some_and(|r| !r.activated);
        let as_chosen = if trash_left_out {
            gone && placement.absence == Some(Absence::TrashLeftOut)
        } else {
            deleted_row
        };
        if !as_chosen {
            let _ = t;
            return Err(Violation::TrashMismatch { item });
        }
    }
    // A deleted row speaks for an item of the Trash, the Trash itself included.
    for row in &plan.notes {
        if row.activated {
            continue;
        }
        let owner = match row.origin {
            Origin::Item { item }
            | Origin::OwnText { item }
            | Origin::DocumentNotes { item }
            | Origin::SplitPart { item, .. } => Some(item),
            _ => None,
        };
        if !owner.is_some_and(|item| trash.is_some_and(|t| within(item, t))) {
            return Err(Violation::TrashMismatch {
                item: owner.unwrap_or(u32::MAX),
            });
        }
    }
    Ok(())
}

/// Within a binder, the items of one top-level item keep Scrivener's order,
/// wherever the mapping put them: the stream the export reads follows the
/// binder, so an item moved past another's rows changes the book even when
/// no two siblings trade places (a Draft text before the first of several
/// books, say). A book's own row is left out of the comparison: the texts
/// before the first book open it. The parts of a split text follow each other.
fn stream_order(project: &Project, plan: &StructurePlan) -> Result<(), Violation> {
    let roots = roots_of(project);
    let binders = [
        (BinderKind::Manuscript, plan.manuscript.as_slice()),
        (BinderKind::Notes, plan.notes.as_slice()),
    ];
    for (binder, rows) in binders {
        let mut last: HashMap<u32, u32> = HashMap::new();
        let mut parts: HashMap<u32, u32> = HashMap::new();
        for (r, row) in rows.iter().enumerate() {
            let out_of_order = Violation::OrderChanged {
                binder,
                row: r as u32,
            };
            if let Origin::Item { item } = row.origin
                && row.sub_role != SubRole::Book
                && let Some(&root) = roots.get(item as usize)
            {
                if last.get(&root).is_some_and(|previous| *previous >= item) {
                    return Err(out_of_order);
                }
                last.insert(root, item);
            }
            for content in &row.contents {
                let Body::From(sources) = &content.body else {
                    continue;
                };
                for source in sources {
                    if let Source::TextPart { item, part } = *source {
                        if parts.get(&item).is_some_and(|previous| *previous >= part) {
                            return Err(out_of_order);
                        }
                        parts.insert(item, part);
                    }
                }
            }
        }
    }
    Ok(())
}
