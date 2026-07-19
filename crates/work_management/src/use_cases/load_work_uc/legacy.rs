// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! Reader for legacy Skribisto `.skrib` files (SQLite).
//!
//! Two stages, mirroring the C++ `LegacyUpgrader`:
//!   1. [`upgrader::upgrade_to_v2`] runs the in-schema version steps (1.0 → 2.0)
//!      on a private in-memory copy — including the HTML→Markdown content
//!      conversion (via `text-document`). The user's file is never mutated.
//!   2. [`read_v2`] maps the resulting `tbl_tree` schema to plain data, which the
//!      use case turns into entities. Unlike the C++ `migrateToV3`, no
//!      intermediate v3 SQLite tables are written — the HashMap store is the
//!      target, so the mapping builds structs directly.
//!
//! By the time `read_v2` runs, all content is Markdown (the 2.0 step converted
//! it), so content blobs are taken verbatim.

mod upgrader;

use anyhow::{Context, Result, anyhow};
use common::entities::{BinderItemRole, BinderItemSubRole, ContentRole};
use rusqlite::Connection;
use rusqlite::types::ValueRef;
use skribisto_model::SubRoleExt;
use skribisto_model::content_allowed;
use skribisto_model::scene_break::{self, SceneBreakTier};
use std::collections::HashMap;

pub struct LegacyTag {
    pub name: String,
    pub color: String,
    pub text_color: String,
}

pub struct LegacyContent {
    pub role: ContentRole,
    pub data: String,
}

pub struct LegacyItem {
    pub old_id: i64,
    pub title: String,
    pub sub_title: String,
    pub role: BinderItemRole,
    pub sub_role: BinderItemSubRole,
    pub label: String,
    pub activated: bool,
    pub indent: i64,
    pub word_count_goal: i64,
    pub char_count_goal: i64,
    pub contents: Vec<LegacyContent>,
    pub tag_old_ids: Vec<i64>,
}

/// Append a scene-break marker to the most recent scene-bearing item that can
/// hold one, mapping the legacy separator's own title to a tier. Returns `false`
/// only when no such item exists yet — a separator leading the binder.
///
/// Walks backwards rather than taking the last item because a note or another
/// non-prose row may sit between the scene and its separator, and **keeps
/// walking** past a scene that has no `SceneText` row (a legacy scene with only
/// a synopsis): giving up on the first such item would discard the mark while
/// reporting that no preceding scene existed, which is not true.
fn append_marker_to_previous_scene(items: &mut [LegacyItem], tier: SceneBreakTier) -> bool {
    for item in items.iter_mut().rev() {
        if !item.sub_role.carries_scene() {
            continue;
        }
        let Some(scene_text) = item
            .contents
            .iter_mut()
            .find(|c| c.role == ContentRole::SceneText)
        else {
            continue;
        };
        if !scene_text.data.trim().is_empty() {
            scene_text.data.push_str("\n\n");
        }
        scene_text.data.push_str(scene_break::canonical_djot(tier));
        return true;
    }
    false
}

/// Prepend a marker to the first scene-bearing item that can hold one — used for
/// a separator that arrived before any scene existed to attach it to.
///
/// `work_management` has no warnings channel, so a leading separator would
/// otherwise be dropped in total silence. Carrying it forward to the next scene
/// keeps the import lossless instead of merely reporting the loss.
fn prepend_marker_to_item(item: &mut LegacyItem, tier: SceneBreakTier) -> bool {
    let Some(scene_text) = item
        .contents
        .iter_mut()
        .find(|c| c.role == ContentRole::SceneText)
    else {
        return false;
    };
    let mark = scene_break::canonical_djot(tier);
    if scene_text.data.trim().is_empty() {
        scene_text.data = mark.to_string();
    } else {
        scene_text.data = format!("{mark}\n\n{}", scene_text.data);
    }
    true
}

pub struct LegacyBinder {
    pub name: String,
    pub is_note: bool,
    pub activated: bool,
    pub items: Vec<LegacyItem>,
}

pub struct LegacyProject {
    pub title: String,
    pub author: String,
    pub dict_language: String,
    /// `tbl_project.t_project_unique_identifier` — the old project's stable id.
    /// Empty if the (very old) file lacks the column; the load path then mints one.
    pub unique_id: String,
    pub absolute_path: String,
    pub tags: Vec<(i64, LegacyTag)>,
    pub dict_words: Vec<String>,
    pub binders: Vec<LegacyBinder>,
    pub references: Vec<(i64, i64)>,
}

/// A raw `tbl_tree` row (only the columns the mapping needs).
#[derive(Clone)]
struct TreeRow {
    old_id: i64,
    title: String,
    internal_title: String,
    indent: i64,
    t_type: String,
    primary: String,
    secondary: String,
    trashed: bool,
}

/// Read any column (TEXT or BLOB affinity, both occur across legacy versions)
/// as a UTF-8 string. NULL and non-text types collapse to a best-effort string.
fn value_to_string(v: ValueRef) -> String {
    match v {
        ValueRef::Null => String::new(),
        ValueRef::Text(b) | ValueRef::Blob(b) => String::from_utf8_lossy(b).into_owned(),
        ValueRef::Integer(i) => i.to_string(),
        ValueRef::Real(f) => f.to_string(),
    }
}

/// `section_type` property → `BinderItem.sub_role`.
fn section_type_to_sub_role(section_type: &str) -> BinderItemSubRole {
    match section_type {
        "book-beginning" => BinderItemSubRole::BookBegin,
        // A legacy chapter section is a chapter *marker* whose scenes follow it in
        // the flat stream — the flat encoding, `Item/ChapterScene`. (It carries a
        // title + synopsis; its prose slot simply goes unused.)
        "chapter" => BinderItemSubRole::ChapterScene,
        "book-end" => BinderItemSubRole::BookEnd,
        // Unknown section type → a plain text item (valid, keeps any content).
        _ => BinderItemSubRole::Text,
    }
}

pub fn read_project(path: &str) -> Result<LegacyProject> {
    // Load a private, writable copy into memory; the user's `.skrib` is never
    // touched. `restore` also validates that the file is a real SQLite database.
    let mut conn = Connection::open_in_memory().context("opening in-memory database")?;
    conn.restore(
        rusqlite::DatabaseName::Main,
        path,
        None::<fn(rusqlite::backup::Progress)>,
    )
    .with_context(|| format!("loading '{path}'"))?;

    // Stage 1: bring the schema up to v2.0 (version steps + HTML→Markdown).
    upgrader::upgrade_to_v2(&conn).context("upgrading legacy schema")?;

    // Stage 2: map the v2.0 tree to plain data.
    read_v2(&conn, path)
}

/// Map a v2.0-schema `tbl_tree` database into a [`LegacyProject`].
fn read_v2(conn: &Connection, path: &str) -> Result<LegacyProject> {
    let has_tbl_tree: bool = conn
        .query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='tbl_tree'",
            [],
            |r| r.get::<_, i64>(0),
        )
        .unwrap_or(0)
        > 0;
    if !has_tbl_tree {
        return Err(anyhow!(
            "'{path}' is not a Skribisto project (no tbl_tree after upgrade)"
        ));
    }

    let absolute_path = std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string());

    // --- Project metadata ---
    let (title, author, dict_language, unique_id) = conn
        .query_row(
            "SELECT COALESCE(t_project_name,''), COALESCE(t_author,''), COALESCE(t_spell_check_lang,''), \
                    COALESCE(t_project_unique_identifier,'') \
             FROM tbl_project LIMIT 1",
            [],
            |r| Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            )),
        )
        .unwrap_or_default();

    // --- Tags ---
    let mut tags: Vec<(i64, LegacyTag)> = Vec::new();
    {
        let mut stmt = conn.prepare(
            "SELECT l_tag_id, COALESCE(t_name,''), COALESCE(t_color,''), COALESCE(t_text_color,'') \
             FROM tbl_tag ORDER BY l_tag_id",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                LegacyTag {
                    name: r.get::<_, String>(1)?,
                    color: r.get::<_, String>(2)?,
                    text_color: r.get::<_, String>(3)?,
                },
            ))
        })?;
        for row in rows {
            tags.push(row?);
        }
    }

    // --- Dictionary ---
    let mut dict_words: Vec<String> = Vec::new();
    {
        let mut stmt = conn.prepare("SELECT COALESCE(t_word,'') FROM tbl_project_dict")?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        for row in rows {
            dict_words.push(row?);
        }
    }

    // --- Per-item properties we care about (section_type, label, word/char goals) ---
    // Legacy stored per-item goals as generic key-value rows in `tbl_tree_property`
    // (`word_count_goal` / `char_count_goal`, an integer-as-string, empty/0 = no goal).
    // The old QtWidgets desktop app displayed counts but never surfaced goals; the mobile
    // app did. Either way we migrate both so no writer's goal is silently dropped.
    let mut section_types: HashMap<i64, String> = HashMap::new();
    let mut labels: HashMap<i64, String> = HashMap::new();
    let mut word_count_goals: HashMap<i64, i64> = HashMap::new();
    let mut char_count_goals: HashMap<i64, i64> = HashMap::new();
    {
        let mut stmt = conn.prepare(
            "SELECT l_tree_code, t_name, m_value FROM tbl_tree_property \
             WHERE t_name IN ('section_type','label','word_count_goal','char_count_goal')",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                value_to_string(r.get_ref(2)?),
            ))
        })?;
        for row in rows {
            let (code, name, value) = row?;
            match name.as_str() {
                "section_type" => {
                    section_types.insert(code, value);
                }
                "label" => {
                    labels.insert(code, value);
                }
                // A malformed/empty legacy value degrades to "no goal" (0), matching the
                // `0 == no goal` sentinel the new schema already uses.
                "word_count_goal" => {
                    word_count_goals.insert(code, value.trim().parse().unwrap_or(0));
                }
                "char_count_goal" => {
                    char_count_goals.insert(code, value.trim().parse().unwrap_or(0));
                }
                _ => {}
            }
        }
    }

    // --- Tag relationships (item old id -> tag old ids) ---
    let mut item_tags: HashMap<i64, Vec<i64>> = HashMap::new();
    {
        let mut stmt = conn.prepare("SELECT l_tree_code, l_tag_code FROM tbl_tag_relationship")?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?;
        for row in rows {
            let (tree, tag) = row?;
            item_tags.entry(tree).or_default().push(tag);
        }
    }

    // --- Cross-references (source -> receiver) ---
    let mut references: Vec<(i64, i64)> = Vec::new();
    {
        let mut stmt = conn.prepare(
            "SELECT l_tree_source_code, l_tree_receiver_code FROM tbl_tree_relationship",
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?)))?;
        for row in rows {
            references.push(row?);
        }
    }

    // --- Tree rows (skip the indent=0 PROJECT root) ---
    let mut tree_rows: Vec<TreeRow> = Vec::new();
    {
        let mut stmt = conn.prepare(
            "SELECT l_tree_id, COALESCE(t_title,''), COALESCE(t_internal_title,''), l_indent, \
                    COALESCE(t_type,''), m_primary_content, m_secondary_content, \
                    COALESCE(b_trashed,0) \
             FROM tbl_tree WHERE l_indent > 0 ORDER BY l_sort_order",
        )?;
        let rows = stmt.query_map([], |r| {
            Ok(TreeRow {
                old_id: r.get::<_, i64>(0)?,
                title: r.get::<_, String>(1)?,
                internal_title: r.get::<_, String>(2)?,
                indent: r.get::<_, i64>(3)?,
                t_type: r.get::<_, String>(4)?,
                primary: value_to_string(r.get_ref(5)?),
                secondary: value_to_string(r.get_ref(6)?),
                trashed: r.get::<_, i64>(7)? != 0,
            })
        })?;
        for row in rows {
            tree_rows.push(row?);
        }
    }

    // --- Group rows into binders (indent==1 FOLDER) and their items ---
    let mut binders: Vec<LegacyBinder> = Vec::new();
    let mut strays: Vec<TreeRow> = Vec::new();

    let make_item = |row: &TreeRow,
                     is_note: bool,
                     effective_indent: i64,
                     section_types: &HashMap<i64, String>,
                     labels: &HashMap<i64, String>,
                     item_tags: &HashMap<i64, Vec<i64>>,
                     word_count_goals: &HashMap<i64, i64>,
                     char_count_goals: &HashMap<i64, i64>|
     -> Option<LegacyItem> {
        let section_type = section_types
            .get(&row.old_id)
            .map(String::as_str)
            .unwrap_or("");
        let is_folder = row.t_type == "FOLDER";

        // Drop separator sections entirely.
        if row.t_type == "SECTION" && section_type == "separator" {
            return None;
        }

        let role = if is_folder {
            BinderItemRole::Folder
        } else {
            BinderItemRole::Item
        };
        let sub_role = if is_folder {
            // Legacy folders are pure grouping — no compile semantics.
            BinderItemSubRole::None
        } else if row.t_type == "SECTION" {
            section_type_to_sub_role(section_type)
        } else if row.t_type == "TEXT" {
            if is_note {
                BinderItemSubRole::Note
            } else {
                BinderItemSubRole::Scene
            }
        } else {
            // Unknown leaf → a plain scene so its content survives.
            BinderItemSubRole::Scene
        };

        // Build candidate content, then keep only what the (role, sub_role)
        // permits — so the migration can never construct an invalid item.
        let mut candidates: Vec<LegacyContent> = Vec::new();
        if !row.title.is_empty() {
            if sub_role == BinderItemSubRole::BookBegin {
                candidates.push(LegacyContent {
                    role: ContentRole::BookTitle,
                    data: row.title.clone(),
                });
            } else if sub_role == BinderItemSubRole::ChapterScene {
                candidates.push(LegacyContent {
                    role: ContentRole::ChapterTitle,
                    data: row.title.clone(),
                });
            }
        }
        if !row.primary.is_empty() {
            candidates.push(LegacyContent {
                role: if is_note {
                    ContentRole::NoteText
                } else {
                    ContentRole::SceneText
                },
                data: row.primary.clone(),
            });
        }
        if !row.secondary.is_empty() {
            candidates.push(LegacyContent {
                role: ContentRole::SynopsisText,
                data: row.secondary.clone(),
            });
        }
        let contents: Vec<LegacyContent> = candidates
            .into_iter()
            .filter(|c| content_allowed(&role, &sub_role, &c.role))
            .collect();

        Some(LegacyItem {
            old_id: row.old_id,
            title: row.title.clone(),
            sub_title: String::new(),
            role,
            sub_role,
            label: labels.get(&row.old_id).cloned().unwrap_or_default(),
            activated: !row.trashed,
            indent: (effective_indent - 2).max(0),
            word_count_goal: word_count_goals.get(&row.old_id).copied().unwrap_or(0),
            char_count_goal: char_count_goals.get(&row.old_id).copied().unwrap_or(0),
            contents,
            tag_old_ids: item_tags.get(&row.old_id).cloned().unwrap_or_default(),
        })
    };

    // A separator that arrives before any scene in its binder; carried forward
    // and prepended to the next scene instead of being dropped. Reset per binder
    // so it can never leak across one.
    //
    // If a binder ends with one still pending it had a separator and no scene at
    // all, so there is no boundary for it to divide — dropping it there is
    // correct, not a loss. (Plume's importer warns in the same situation for the
    // same reason.) Every case where a scene *does* exist is now carried.
    let mut pending_leading_break: Option<SceneBreakTier> = None;
    for row in &tree_rows {
        if row.indent == 1 && row.t_type == "FOLDER" {
            pending_leading_break = None;
            binders.push(LegacyBinder {
                name: row.title.clone(),
                is_note: row.internal_title == "note_folder",
                activated: !row.trashed,
                items: Vec::new(),
            });
        } else if row.indent == 1 {
            // Stray top-level non-folder — attached to the first binder later.
            strays.push(row.clone());
        } else if let Some(binder) = binders.last_mut() {
            let is_note = binder.is_note;
            // A legacy `separator` section is exactly a scene break, so it becomes
            // a marker paragraph in the preceding scene's prose rather than being
            // discarded. The walk is ordered (`ORDER BY l_sort_order`), so the
            // scene it belongs to is already in `binder.items`.
            let is_separator = row.t_type == "SECTION"
                && section_types.get(&row.old_id).map(String::as_str) == Some("separator");
            if is_separator {
                let tier = scene_break::tier_of_plain_line(&row.title)
                    .unwrap_or(SceneBreakTier::Minor);
                // If nothing precedes it, hold the mark for the next scene rather
                // than dropping it — there is no warnings channel here, so a
                // silent drop would be indistinguishable from a clean import.
                if !append_marker_to_previous_scene(&mut binder.items, tier) {
                    pending_leading_break = Some(tier);
                }
            } else if let Some(item) = make_item(
                row,
                is_note,
                row.indent,
                &section_types,
                &labels,
                &item_tags,
                &word_count_goals,
                &char_count_goals,
            ) {
                let mut item = item;
                if let Some(tier) = pending_leading_break
                    && item.sub_role.carries_scene()
                    && prepend_marker_to_item(&mut item, tier)
                {
                    pending_leading_break = None;
                }
                binder.items.push(item);
            }
        } else {
            strays.push(row.clone());
        }
    }

    // Ensure at least one binder, then fold strays into the first one (indent 2 → 0).
    if binders.is_empty() {
        binders.push(LegacyBinder {
            name: "Writings".to_string(),
            is_note: false,
            activated: true,
            items: Vec::new(),
        });
    }
    {
        let first_is_note = binders[0].is_note;
        for row in &strays {
            if let Some(item) = make_item(
                row,
                first_is_note,
                2,
                &section_types,
                &labels,
                &item_tags,
                &word_count_goals,
                &char_count_goals,
            ) {
                binders[0].items.push(item);
            }
        }
    }

    Ok(LegacyProject {
        title,
        author,
        dict_language,
        unique_id,
        absolute_path,
        tags,
        dict_words,
        binders,
        references,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_separator_walks_past_a_scene_that_has_no_prose_row() {
        let mut items = vec![
            LegacyItem {
                old_id: 1, title: "A".into(), sub_title: String::new(),
                role: BinderItemRole::Item, sub_role: BinderItemSubRole::Scene,
                label: String::new(), activated: true, indent: 0,
                word_count_goal: 0, char_count_goal: 0,
                contents: vec![LegacyContent { role: ContentRole::SceneText, data: "One.".into() }],
                tag_old_ids: Vec::new(),
            },
            // A scene carrying only a synopsis: no SceneText row to append to.
            LegacyItem {
                old_id: 2, title: "B".into(), sub_title: String::new(),
                role: BinderItemRole::Item, sub_role: BinderItemSubRole::Scene,
                label: String::new(), activated: true, indent: 0,
                word_count_goal: 0, char_count_goal: 0,
                contents: vec![LegacyContent { role: ContentRole::SynopsisText, data: "S".into() }],
                tag_old_ids: Vec::new(),
            },
        ];
        assert!(append_marker_to_previous_scene(&mut items, SceneBreakTier::Minor));
        let a = &items[0].contents[0].data;
        assert!(a.ends_with("\\* \\* \\*"), "must fall back to the scene before: {a:?}");
    }

    #[test]
    fn a_leading_separator_is_carried_forward_not_dropped() {
        // Nothing precedes it, so it must be held and prepended to the next
        // scene — work_management has no warnings channel, so a drop would be
        // completely silent.
        let mut items: Vec<LegacyItem> = Vec::new();
        assert!(!append_marker_to_previous_scene(&mut items, SceneBreakTier::Minor));

        let mut next = LegacyItem {
            old_id: 3, title: "C".into(), sub_title: String::new(),
            role: BinderItemRole::Item, sub_role: BinderItemSubRole::Scene,
            label: String::new(), activated: true, indent: 0,
            word_count_goal: 0, char_count_goal: 0,
            contents: vec![LegacyContent { role: ContentRole::SceneText, data: "Later.".into() }],
            tag_old_ids: Vec::new(),
        };
        assert!(prepend_marker_to_item(&mut next, SceneBreakTier::Minor));
        assert_eq!(next.contents[0].data, "\\* \\* \\*\n\nLater.");
    }
}
