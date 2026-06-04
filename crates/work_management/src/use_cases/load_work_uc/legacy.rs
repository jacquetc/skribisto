//! Reader for legacy Skribisto `.skrib` files (SQLite, `tbl_tree` schema).
//!
//! This is the Rust port of the C++ `migrateToV3` mapping. It reads the legacy
//! tables and returns plain data; the use case turns that into entities. Unlike
//! the C++ path it does NOT write intermediate v3 SQLite tables — the in-memory
//! HashMap store is the target.
//!
//! Scope note (milestone-1 slice): opens the file read-only and maps the current
//! structure directly. Content blobs are stored verbatim (HTML in ≤2.0 files);
//! the in-place version-step upgrades (1.0→2.0) and the HTML→Markdown conversion
//! are deferred to a later phase — they don't affect the navigation tree.

use anyhow::{Context, Result, anyhow};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};
use std::collections::HashMap;

pub struct LegacyTag {
    pub name: String,
    pub color: String,
    pub text_color: String,
}

pub struct LegacyContent {
    pub role: String,
    pub data: String,
}

pub struct LegacyItem {
    pub old_id: i64,
    pub title: String,
    pub sub_title: String,
    pub role: String,
    pub sub_role: String,
    pub label: String,
    pub activated: bool,
    pub indent: i64,
    pub contents: Vec<LegacyContent>,
    pub tag_old_ids: Vec<i64>,
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
fn section_type_to_sub_role(section_type: &str) -> &'static str {
    match section_type {
        "book-beginning" => "book-begin",
        "chapter" => "chapter",
        "book-end" => "book-end",
        _ => "",
    }
}

pub fn read_project(path: &str) -> Result<LegacyProject> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .with_context(|| format!("opening SQLite file '{path}'"))?;

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
            "'{path}' is not a legacy tbl_tree project; new-format reading is not implemented yet"
        ));
    }

    let absolute_path = std::fs::canonicalize(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string());

    // --- Project metadata ---
    let (title, author, dict_language) = conn
        .query_row(
            "SELECT COALESCE(t_project_name,''), COALESCE(t_author,''), COALESCE(t_spell_check_lang,'') \
             FROM tbl_project LIMIT 1",
            [],
            |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)),
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

    // --- Per-item properties we care about (section_type, label) ---
    let mut section_types: HashMap<i64, String> = HashMap::new();
    let mut labels: HashMap<i64, String> = HashMap::new();
    {
        let mut stmt = conn.prepare(
            "SELECT l_tree_code, t_name, m_value FROM tbl_tree_property \
             WHERE t_name IN ('section_type','label')",
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
            if name == "section_type" {
                section_types.insert(code, value);
            } else {
                labels.insert(code, value);
            }
        }
    }

    // --- Tag relationships (item old id -> tag old ids) ---
    let mut item_tags: HashMap<i64, Vec<i64>> = HashMap::new();
    {
        let mut stmt =
            conn.prepare("SELECT l_tree_code, l_tag_code FROM tbl_tag_relationship")?;
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
                     item_tags: &HashMap<i64, Vec<i64>>|
     -> Option<LegacyItem> {
        let section_type = section_types.get(&row.old_id).map(String::as_str).unwrap_or("");
        let is_folder = row.t_type == "FOLDER";

        // Drop separator sections entirely.
        if row.t_type == "SECTION" && section_type == "separator" {
            return None;
        }

        let role = if is_folder { "folder" } else { "item" }.to_string();
        let sub_role = if is_folder {
            String::new()
        } else if row.t_type == "SECTION" {
            section_type_to_sub_role(section_type).to_string()
        } else if row.t_type == "TEXT" {
            if is_note { "note" } else { "scene" }.to_string()
        } else {
            String::new()
        };

        let mut contents: Vec<LegacyContent> = Vec::new();
        if sub_role == "book-begin" && !row.title.is_empty() {
            contents.push(LegacyContent { role: "book-title".into(), data: row.title.clone() });
        } else if sub_role == "chapter" && !row.title.is_empty() {
            contents.push(LegacyContent { role: "chapter-title".into(), data: row.title.clone() });
        }
        if !row.primary.is_empty() {
            contents.push(LegacyContent {
                role: if is_note { "note-text" } else { "scene-text" }.into(),
                data: row.primary.clone(),
            });
        }
        if !row.secondary.is_empty() {
            contents.push(LegacyContent {
                role: "synopsis-text".into(),
                data: row.secondary.clone(),
            });
        }

        Some(LegacyItem {
            old_id: row.old_id,
            title: row.title.clone(),
            sub_title: String::new(),
            role,
            sub_role,
            label: labels.get(&row.old_id).cloned().unwrap_or_default(),
            activated: !row.trashed,
            indent: (effective_indent - 2).max(0),
            contents,
            tag_old_ids: item_tags.get(&row.old_id).cloned().unwrap_or_default(),
        })
    };

    for row in &tree_rows {
        if row.indent == 1 && row.t_type == "FOLDER" {
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
            if let Some(item) =
                make_item(row, is_note, row.indent, &section_types, &labels, &item_tags)
            {
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
            if let Some(item) =
                make_item(row, first_is_note, 2, &section_types, &labels, &item_tags)
            {
                binders[0].items.push(item);
            }
        }
    }

    Ok(LegacyProject {
        title,
        author,
        dict_language,
        absolute_path,
        tags,
        dict_words,
        binders,
        references,
    })
}
