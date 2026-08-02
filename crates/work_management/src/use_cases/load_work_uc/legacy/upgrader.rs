// SPDX-License-Identifier: GPL-3.0-only
// SPDX-FileCopyrightText: 2026 Cyril Jacquet

//! In-schema version-step upgrader for legacy `.skrib` SQLite files.
//!
//! Rust port of the C++ `Upgrader::upgradeSQLite` chain (1.0 → 2.0). It runs on a
//! private, writable copy of the project (in practice an in-memory database), so
//! the user's file is never mutated. After this runs the schema is at "2.0" and
//! `super::read_v2` maps `tbl_tree` → entities.
//!
//! Versions are tracked as integer tenths (1.0 → 10, … 2.0 → 20) to avoid float
//! comparison. The chain is a waterfall: each step is applied if the current
//! version is below its target, then the version is bumped.
//!
//! Fidelity notes — we reproduce every *semantically meaningful* step (the
//! sheet/note → tree migration, parent→folder transformation, synopsis →
//! secondary-content move, HTML↔Markdown conversion). We deliberately omit parts
//! that only matter for a *persisted* SQLite file and are invisible to the
//! in-memory read-and-discard model:
//!   * the legacy 2.0 Trash folder + trashed-item relocation — Skribisto-rs keeps
//!     trashed items in their original binder (`activated = false`) and indexes
//!     them via TrashInfo, so relocating them here would only lose their origin,
//!   * table rebuilds whose sole purpose is adding FK `ON DELETE/UPDATE CASCADE`
//!     constraints or flipping column defaults,
//!   * orphan-row trims are done with a single set-based `DELETE` rather than the
//!     C++'s row-by-row loop (and the C++ `trimTagRelationship` bind-name bug is
//!     fixed — it deleted nothing).

use anyhow::{Context, Result, bail};
use rusqlite::{Connection, OptionalExtension};

use skrib_format::convert as content;

/// Bring a legacy database up to schema version 2.0.
pub fn upgrade_to_v2(conn: &Connection) -> Result<()> {
    // The table-rebuild dance assumes FK enforcement is off (SQLite's default;
    // set explicitly in case the caller enabled it).
    conn.pragma_update(None, "foreign_keys", false)?;

    let mut v = detect_version(conn)?;
    if v == 20 {
        return Ok(()); // already current
    }
    // The ceiling the `.skrib` side gets from `skrib_format::version_gate` — the legacy
    // path needs its own, because it never constructs a `WorkBundle` and so never enters
    // `read_bundle`. That asymmetry has bitten this codebase before, in exactly this
    // shape: see `migration.rs`'s module doc on uid minting, and the
    // `a_legacy_project_gets_a_distinct_uid_for_every_row` test that pins it.
    //
    // `v >= 20` used to mean "already current", which silently treated a *newer* database
    // as current and ran the whole importer against a schema it does not understand.
    // Moot in practice — the format is retired and nothing will ever write a 2.1 — but it
    // is three lines and the bug class is not hypothetical here.
    if v > 20 {
        bail!(
            "unsupported legacy database version {} (newer than this build's upgrade chain, which stops at 2.0)",
            v as f64 / 10.0
        );
    }
    if v < 10 {
        bail!("unsupported legacy database version {}", v as f64 / 10.0);
    }

    if v < 11 {
        step_1_0_to_1_1(conn)?;
        v = set_version(conn, 11)?;
    }
    if v < 12 {
        step_1_1_to_1_2(conn)?;
        v = set_version(conn, 12)?;
    }
    if v < 13 {
        step_1_2_to_1_3(conn)?;
        v = set_version(conn, 13)?;
    }
    if v < 14 {
        step_1_3_to_1_4(conn)?;
        v = set_version(conn, 14)?;
    }
    if v < 15 {
        step_1_4_to_1_5(conn)?;
        v = set_version(conn, 15)?;
    }
    if v < 16 {
        step_1_5_to_1_6(conn)?;
        v = set_version(conn, 16)?;
    }
    if v < 17 {
        step_1_6_to_1_7(conn)?;
        v = set_version(conn, 17)?;
    }
    if v < 18 {
        step_1_7_to_1_8(conn)?;
        v = set_version(conn, 18)?;
    }
    if v < 19 {
        step_1_8_to_1_9(conn)?;
        v = set_version(conn, 19)?;
    }
    if v < 20 {
        step_1_9_to_2_0(conn)?;
        set_version(conn, 20)?;
    }

    Ok(())
}

/// Read `tbl_project.dbl_database_version` as integer tenths.
fn detect_version(conn: &Connection) -> Result<i64> {
    let ver: f64 = conn
        .query_row("SELECT dbl_database_version FROM tbl_project", [], |r| {
            r.get(0)
        })
        .context("reading dbl_database_version (not a Skribisto project?)")?;
    Ok((ver * 10.0).round() as i64)
}

/// Write the new version (in tenths) and return it for the waterfall.
fn set_version(conn: &Connection, tenths: i64) -> Result<i64> {
    conn.execute(
        "UPDATE tbl_project SET dbl_database_version = ?1",
        [tenths as f64 / 10.0],
    )?;
    conn.execute("UPDATE tbl_project SET dt_updated = CURRENT_TIMESTAMP", [])?;
    Ok(tenths)
}

// ── 1.0 → 1.1 : add the project dictionary table ──────────────────────────────
fn step_1_0_to_1_1(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE tbl_project_dict (
            l_project_dict_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
            t_word TEXT UNIQUE ON CONFLICT REPLACE NOT NULL
        );",
    )
    .context("step 1.0→1.1")
}

// ── 1.1 → 1.2 : rebuild tbl_sheet_note (add b_synopsis) ───────────────────────
fn step_1_1_to_1_2(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE temp_table AS SELECT * FROM tbl_sheet_note;
         DROP TABLE tbl_sheet_note;
         CREATE TABLE tbl_sheet_note (
            l_sheet_note_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
            l_sheet_code INTEGER NOT NULL,
            l_note_code  INTEGER NOT NULL,
            b_synopsis   BOOLEAN NOT NULL DEFAULT (0)
         );
         INSERT INTO tbl_sheet_note (l_sheet_note_id, l_sheet_code, l_note_code, b_synopsis)
            SELECT l_sheet_note_id, l_sheet_code, l_note_code, b_synopsis FROM temp_table;
         DROP TABLE temp_table;",
    )
    .context("step 1.1→1.2")
}

// ── 1.2 → 1.3 : tbl_history → tbl_stat_history ────────────────────────────────
fn step_1_2_to_1_3(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE tbl_stat_history (
            l_stat_history_id  INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
            dt_saved           DATETIME,
            l_sheet_char_count INTEGER,
            l_sheet_word_count INTEGER,
            l_note_char_count  INTEGER,
            l_note_word_count  INTEGER
         );
         INSERT INTO tbl_stat_history (dt_saved, l_sheet_char_count, l_sheet_word_count)
            SELECT dt_saved, l_char_count, l_word_count FROM tbl_history;
         DROP TABLE tbl_history;",
    )
    .context("step 1.2→1.3")
}

// ── 1.3 → 1.4 : rebuild tbl_tag (add t_text_color) ────────────────────────────
fn step_1_3_to_1_4(conn: &Connection) -> Result<()> {
    conn.execute("ALTER TABLE tbl_tag ADD COLUMN t_text_color TEXT", [])
        .context("step 1.3→1.4")?;
    Ok(())
}

// ── 1.4 → 1.5 : the big one — unify tbl_sheet/tbl_note into tbl_tree ───────────
fn step_1_4_to_1_5(conn: &Connection) -> Result<()> {
    // New tree + property tables; tag_relationship gains l_tree_code; sheet_note
    // gains source/receiver tree codes.
    conn.execute_batch(
        "CREATE TABLE tbl_tree (
            l_tree_id    INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
            t_title      TEXT,
            l_sort_order INTEGER NOT NULL DEFAULT (9999999999),
            l_indent     INTEGER NOT NULL DEFAULT (0),
            t_type       TEXT,
            m_primary_content   BLOB,
            m_secondary_content BLOB,
            dt_created   DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            dt_updated   DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            dt_trashed   DATETIME,
            b_trashed    BOOLEAN NOT NULL DEFAULT (0)
         );
         CREATE TABLE tbl_tree_property (
            l_tree_property_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
            l_tree_code  INTEGER,
            t_name       TEXT,
            t_value_type TEXT NOT NULL DEFAULT STRING,
            m_value      BLOB,
            dt_created   DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            dt_updated   DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
            b_system     BOOLEAN NOT NULL DEFAULT (0),
            b_silent     BOOLEAN NOT NULL DEFAULT (0)
         );
         ALTER TABLE tbl_tag_relationship ADD COLUMN l_tree_code INTEGER;
         ALTER TABLE tbl_sheet_note ADD COLUMN l_tree_source_code INTEGER;
         ALTER TABLE tbl_sheet_note ADD COLUMN l_tree_receiver_code INTEGER;",
    )
    .context("step 1.4→1.5: create tree tables")?;

    move_paper_to_tree_1_5(conn, PaperKind::Sheet)?;
    move_paper_to_tree_1_5(conn, PaperKind::Note)?;
    transform_parents_to_folder_1_5(conn)?;

    // Promote the interim sheet_note links into tbl_tree_relationship, then drop
    // the now-obsolete paper tables, views, triggers and indexes.
    conn.execute_batch(
        "CREATE TABLE tbl_tree_relationship (
            l_tree_relationship_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL,
            l_tree_source_code     INTEGER NOT NULL,
            l_tree_receiver_code   INTEGER NOT NULL,
            b_synopsis             BOOLEAN NOT NULL DEFAULT (0)
         );
         INSERT INTO tbl_tree_relationship
            (l_tree_relationship_id, l_tree_source_code, l_tree_receiver_code, b_synopsis)
            SELECT l_sheet_note_id, l_tree_source_code, l_tree_receiver_code, b_synopsis
            FROM tbl_sheet_note
            WHERE l_tree_source_code IS NOT NULL AND l_tree_receiver_code IS NOT NULL;
         DROP TABLE IF EXISTS tbl_sheet_note;
         DROP TRIGGER IF EXISTS trg_delete_properties;
         DROP VIEW IF EXISTS v_property_sheet;
         DROP VIEW IF EXISTS v_tree_sheet;
         DROP INDEX IF EXISTS idx_note;
         DROP INDEX IF EXISTS idx_sheet;
         DROP TABLE IF EXISTS tbl_note;
         DROP TABLE IF EXISTS tbl_note_property;
         DROP TABLE IF EXISTS tbl_sheet;
         DROP TABLE IF EXISTS tbl_sheet_property;",
    )
    .context("step 1.4→1.5: promote relationships, drop deprecated")?;

    renumber_tree_sort_order(conn)?;
    Ok(())
}

#[derive(Clone, Copy)]
enum PaperKind {
    Sheet,
    Note,
}

/// Copy every `tbl_sheet`/`tbl_note` row (and its properties) into `tbl_tree` as
/// a `TEXT` node, and rewire its tag- and sheet-note relationships to the new id.
fn move_paper_to_tree_1_5(conn: &Connection, kind: PaperKind) -> Result<()> {
    let (table, table_id, prop_table, prop_code, tree_rel_code) = match kind {
        PaperKind::Sheet => (
            "tbl_sheet",
            "l_sheet_id",
            "tbl_sheet_property",
            "l_sheet_code",
            "l_tree_receiver_code",
        ),
        PaperKind::Note => (
            "tbl_note",
            "l_note_id",
            "tbl_note_property",
            "l_note_code",
            "l_tree_source_code",
        ),
    };

    // Offset notes after sheets so the two paper streams don't collide.
    let starting: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(l_sort_order), 0) FROM tbl_tree",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);

    let paper_ids: Vec<i64> = {
        let sql = format!("SELECT {table_id} FROM {table} ORDER BY l_sort_order");
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map([], |r| r.get(0))?;
        rows.collect::<rusqlite::Result<_>>()?
    };

    for paper_id in paper_ids {
        let insert = format!(
            "INSERT INTO tbl_tree
               (t_title, l_sort_order, l_indent, t_type, m_primary_content,
                dt_created, dt_updated, dt_trashed, b_trashed)
             VALUES (
               (SELECT t_title      FROM {table} WHERE {table_id} = ?1),
               (SELECT l_sort_order FROM {table} WHERE {table_id} = ?1) + ?2,
               (SELECT l_indent     FROM {table} WHERE {table_id} = ?1),
               'TEXT',
               (SELECT m_content    FROM {table} WHERE {table_id} = ?1),
               (SELECT dt_created   FROM {table} WHERE {table_id} = ?1),
               (SELECT dt_updated   FROM {table} WHERE {table_id} = ?1),
               (SELECT dt_trashed   FROM {table} WHERE {table_id} = ?1),
               (SELECT b_trashed    FROM {table} WHERE {table_id} = ?1)
             )"
        );
        conn.execute(&insert, (paper_id, starting))?;
        let new_tree_id = conn.last_insert_rowid();

        let prop_ids: Vec<i64> = {
            let sql = format!("SELECT l_property_id FROM {prop_table} WHERE {prop_code} = ?1");
            let mut stmt = conn.prepare(&sql)?;
            let rows = stmt.query_map([paper_id], |r| r.get(0))?;
            rows.collect::<rusqlite::Result<_>>()?
        };
        for prop_id in prop_ids {
            let pins = format!(
                "INSERT INTO tbl_tree_property
                   (l_tree_code, t_name, t_value_type, m_value, dt_created, dt_updated, b_system)
                 VALUES (?1,
                   (SELECT t_name     FROM {prop_table} WHERE l_property_id = ?2),
                   'STRING',
                   (SELECT t_value    FROM {prop_table} WHERE l_property_id = ?2),
                   (SELECT dt_created FROM {prop_table} WHERE l_property_id = ?2),
                   (SELECT dt_updated FROM {prop_table} WHERE l_property_id = ?2),
                   (SELECT b_system   FROM {prop_table} WHERE l_property_id = ?2))"
            );
            conn.execute(&pins, (new_tree_id, prop_id))?;
        }

        let tag_update =
            format!("UPDATE tbl_tag_relationship SET l_tree_code = ?1 WHERE {prop_code} = ?2");
        conn.execute(&tag_update, (new_tree_id, paper_id))?;

        let rel_update =
            format!("UPDATE tbl_sheet_note SET {tree_rel_code} = ?1 WHERE {prop_code} = ?2");
        conn.execute(&rel_update, (new_tree_id, paper_id))?;
    }
    Ok(())
}

/// Old projects encoded "this item has children" implicitly. Walk the tree in
/// reverse sort order; whenever the next item is more deeply indented, the
/// current item was a parent — materialise an explicit `FOLDER` just before it
/// and push the item one level deeper.
fn transform_parents_to_folder_1_5(conn: &Connection) -> Result<()> {
    let rows: Vec<(i64, i64, i64)> = {
        let mut stmt = conn.prepare(
            "SELECT l_tree_id, l_indent, l_sort_order FROM tbl_tree ORDER BY l_sort_order",
        )?;
        let r = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        r.collect::<rusqlite::Result<_>>()?
    };

    let mut next_indent: i64 = -1; // indent of the item just after the current one
    for &(tree_id, indent, sort_order) in rows.iter().rev() {
        if next_indent > indent {
            conn.execute(
                "INSERT INTO tbl_tree (t_title, l_indent, l_sort_order, t_type, dt_trashed, b_trashed)
                 VALUES (
                   (SELECT t_title    FROM tbl_tree WHERE l_tree_id = ?1),
                   (SELECT l_indent   FROM tbl_tree WHERE l_tree_id = ?1),
                   ?2, 'FOLDER',
                   (SELECT dt_trashed FROM tbl_tree WHERE l_tree_id = ?1),
                   (SELECT b_trashed  FROM tbl_tree WHERE l_tree_id = ?1))",
                (tree_id, sort_order - 1),
            )?;
            let new_folder_id = conn.last_insert_rowid();
            conn.execute(
                "UPDATE tbl_tree SET l_indent = ?1 WHERE l_tree_id = ?2",
                (indent + 1, tree_id),
            )?;

            let is_synopsis: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM tbl_tree_property
                     WHERE t_name = 'is_synopsis_folder' AND m_value = 'true' AND l_tree_code = ?1",
                    [tree_id],
                    |r| r.get(0),
                )
                .unwrap_or(0);
            if is_synopsis > 0 {
                conn.execute(
                    "INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value)
                     VALUES (?1, 'is_synopsis_folder', 'STRING', 'true')",
                    [new_folder_id],
                )?;
            }
        }
        next_indent = indent;
    }
    Ok(())
}

// ── 1.5 → 1.6 : add PROJECT root, fold synopsis notes into secondary content ───
fn step_1_5_to_1_6(conn: &Connection) -> Result<()> {
    // Make room for an indent-0 root, then add it.
    conn.execute("UPDATE tbl_tree SET l_indent = l_indent + 1", [])
        .context("step 1.5→1.6: shift indents")?;
    conn.execute(
        "INSERT INTO tbl_tree (l_tree_id, l_sort_order, l_indent, t_type)
         VALUES (0, -1, 0, 'PROJECT')",
        [],
    )
    .context("step 1.5→1.6: insert PROJECT root")?;

    move_synopsis_to_secondary_1_6(conn)?;

    // Drop the now-meaningless b_synopsis flag from tree relationships.
    conn.execute("DELETE FROM tbl_tree_relationship WHERE b_synopsis = 1", [])
        .ok(); // already consumed by the move above; tolerate absence
    renumber_tree_sort_order(conn)?;
    trim_orphans(conn)?;
    Ok(())
}

/// Copy each synopsis note's content into its owner's `m_secondary_content`, then
/// delete the synopsis note and any dedicated "Outline" synopsis folders.
fn move_synopsis_to_secondary_1_6(conn: &Connection) -> Result<()> {
    let rels: Vec<(i64, i64, i64)> = {
        let mut stmt = conn.prepare(
            "SELECT l_tree_relationship_id, l_tree_source_code, l_tree_receiver_code
             FROM tbl_tree_relationship WHERE b_synopsis = 1",
        )?;
        let r = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
        r.collect::<rusqlite::Result<_>>()?
    };
    for (rel_id, synopsis_id, text_id) in rels {
        conn.execute(
            "UPDATE tbl_tree
             SET m_secondary_content = (SELECT m_primary_content FROM tbl_tree WHERE l_tree_id = ?1)
             WHERE l_tree_id = ?2",
            (synopsis_id, text_id),
        )?;
        conn.execute(
            "DELETE FROM tbl_tree_relationship WHERE l_tree_relationship_id = ?1",
            [rel_id],
        )?;
        conn.execute("DELETE FROM tbl_tree WHERE l_tree_id = ?1", [synopsis_id])?;
    }

    let folders: Vec<i64> = {
        let mut stmt = conn.prepare(
            "SELECT l_tree_code FROM tbl_tree_property
             WHERE t_name = 'is_synopsis_folder' AND m_value = 'true'",
        )?;
        let r = stmt.query_map([], |r| r.get(0))?;
        r.collect::<rusqlite::Result<_>>()?
    };
    for folder_id in folders {
        conn.execute("DELETE FROM tbl_tree WHERE l_tree_id = ?1", [folder_id])?;
    }
    Ok(())
}

// ── 1.6 → 1.7 : content was Markdown; the C++ turned it into HTML ──────────────
fn step_1_6_to_1_7(conn: &Connection) -> Result<()> {
    convert_column(
        conn,
        "SELECT l_tree_id FROM tbl_tree WHERE t_type = 'TEXT'",
        "m_primary_content",
        content::markdown_to_html,
    )
    .context("step 1.6→1.7: markdown→html")
}

// ── 1.7 → 1.8 : add t_internal_title (read by the v3 mapping) ──────────────────
fn step_1_7_to_1_8(conn: &Connection) -> Result<()> {
    conn.execute("ALTER TABLE tbl_tree ADD COLUMN t_internal_title TEXT", [])
        .context("step 1.7→1.8")?;
    Ok(())
}

// ── 1.8 → 1.9 : rename a few property keys ────────────────────────────────────
fn step_1_8_to_1_9(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "UPDATE tbl_tree_property SET t_name = REPLACE(t_name, 'can_add_sibling_paper', 'can_add_sibling_tree_item');
         UPDATE tbl_tree_property SET t_name = REPLACE(t_name, 'can_add_child_paper', 'can_add_child_tree_item');
         UPDATE tbl_tree_property SET t_value_type = 'BOOL'
            WHERE t_name = 'can_add_sibling_tree_item' OR t_name = 'can_add_child_tree_item';
         UPDATE tbl_tree_property SET t_value_type = 'INT'
            WHERE t_name IN ('word_count_with_children','word_count','char_count','char_count_with_children');",
    )
    .context("step 1.8→1.9")
}

// ── 1.9 → 2.0 : HTML → Markdown ───────────────────────────────────────────────
fn step_1_9_to_2_0(conn: &Connection) -> Result<()> {
    // The only 1.9→2.0 change meaningful to the in-memory model is the content
    // format: Qt HTML → Markdown — primary content of TEXT items, then secondary
    // content of every item.
    //
    // We deliberately skip the legacy Trash folder + trashed-item relocation:
    // Skribisto-rs leaves trashed items in their original binder as
    // `activated = false` and indexes them via TrashInfo at the mapping stage, so
    // moving them into a separate folder here would only lose their real origin.
    convert_column(
        conn,
        "SELECT l_tree_id FROM tbl_tree WHERE t_type = 'TEXT'",
        "m_primary_content",
        content::html_to_djot,
    )
    .context("step 1.9→2.0: primary html→djot")?;
    convert_column(
        conn,
        "SELECT l_tree_id FROM tbl_tree",
        "m_secondary_content",
        content::html_to_djot,
    )
    .context("step 1.9→2.0: secondary html→djot")?;
    Ok(())
}

// ── Shared helpers ────────────────────────────────────────────────────────────

/// Renumber `tbl_tree` sort orders to 0, 1000, 2000, … in current order.
fn renumber_tree_sort_order(conn: &Connection) -> Result<()> {
    let ids: Vec<i64> = {
        let mut stmt = conn.prepare("SELECT l_tree_id FROM tbl_tree ORDER BY l_sort_order")?;
        let r = stmt.query_map([], |r| r.get(0))?;
        r.collect::<rusqlite::Result<_>>()?
    };
    let mut value = 0i64;
    for id in ids {
        conn.execute(
            "UPDATE tbl_tree SET l_sort_order = ?1 WHERE l_tree_id = ?2",
            (value, id),
        )?;
        value += 1000;
    }
    Ok(())
}

/// Delete junction/property rows pointing at tree ids that no longer exist.
/// (The C++ did this row-by-row; the set-based form is equivalent and also fixes
/// the original `trimTagRelationship` bind-name bug, which deleted nothing.)
fn trim_orphans(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM tbl_tag_relationship
         WHERE l_tree_code NOT IN (SELECT l_tree_id FROM tbl_tree)",
        [],
    )?;
    conn.execute(
        "DELETE FROM tbl_tree_property
         WHERE l_tree_code NOT IN (SELECT l_tree_id FROM tbl_tree)",
        [],
    )?;
    conn.execute(
        "DELETE FROM tbl_tree_relationship
         WHERE l_tree_source_code   NOT IN (SELECT l_tree_id FROM tbl_tree)
            OR l_tree_receiver_code NOT IN (SELECT l_tree_id FROM tbl_tree)",
        [],
    )?;
    Ok(())
}

/// Read a BLOB content column for each selected tree id, run it through `convert`,
/// and write the result back. Used for the HTML↔Markdown conversion steps.
fn convert_column(
    conn: &Connection,
    select_ids: &str,
    column: &str,
    convert: impl Fn(&str) -> Result<String>,
) -> Result<()> {
    let ids: Vec<i64> = {
        let mut stmt = conn.prepare(select_ids)?;
        let r = stmt.query_map([], |r| r.get(0))?;
        r.collect::<rusqlite::Result<_>>()?
    };
    let select_one = format!("SELECT {column} FROM tbl_tree WHERE l_tree_id = ?1");
    let update_one = format!("UPDATE tbl_tree SET {column} = ?1 WHERE l_tree_id = ?2");
    for id in ids {
        // Content columns are BLOB affinity but legacy files store TEXT in them,
        // so read via `value_to_string` (handles Text/Blob/Null uniformly).
        let source: String = conn
            .query_row(&select_one, [id], |r| {
                Ok(super::value_to_string(r.get_ref(0)?))
            })
            .optional()?
            .unwrap_or_default();
        let converted = convert(&source)?;
        conn.execute(&update_one, (converted, id))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal v1.4 project (pre-`tbl_tree`: separate sheets/notes) to
    /// exercise the part of the chain the real fixture (v1.8) can't reach.
    fn make_v1_4_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE tbl_project (dbl_database_version REAL, t_project_name TEXT, t_author TEXT, t_spell_check_lang TEXT, t_project_unique_identifier TEXT, dt_updated DATETIME);
             INSERT INTO tbl_project (dbl_database_version, t_project_name, t_author, t_spell_check_lang, t_project_unique_identifier) VALUES (1.4, 'My Novel', 'Jane', 'en', 'abc123XYZ000');

             CREATE TABLE tbl_sheet (l_sheet_id INTEGER PRIMARY KEY AUTOINCREMENT, t_title TEXT, l_sort_order INTEGER, l_indent INTEGER, m_content BLOB, dt_created DATETIME DEFAULT CURRENT_TIMESTAMP, dt_updated DATETIME DEFAULT CURRENT_TIMESTAMP, dt_trashed DATETIME, b_trashed BOOLEAN DEFAULT 0);
             CREATE TABLE tbl_sheet_property (l_property_id INTEGER PRIMARY KEY AUTOINCREMENT, l_sheet_code INTEGER, t_name TEXT, t_value TEXT, dt_created DATETIME DEFAULT CURRENT_TIMESTAMP, dt_updated DATETIME DEFAULT CURRENT_TIMESTAMP, b_system BOOLEAN DEFAULT 0);
             CREATE TABLE tbl_note (l_note_id INTEGER PRIMARY KEY AUTOINCREMENT, t_title TEXT, l_sort_order INTEGER, l_indent INTEGER, m_content BLOB, dt_created DATETIME DEFAULT CURRENT_TIMESTAMP, dt_updated DATETIME DEFAULT CURRENT_TIMESTAMP, dt_trashed DATETIME, b_trashed BOOLEAN DEFAULT 0);
             CREATE TABLE tbl_note_property (l_property_id INTEGER PRIMARY KEY AUTOINCREMENT, l_note_code INTEGER, t_name TEXT, t_value TEXT, dt_created DATETIME DEFAULT CURRENT_TIMESTAMP, dt_updated DATETIME DEFAULT CURRENT_TIMESTAMP, b_system BOOLEAN DEFAULT 0);
             CREATE TABLE tbl_sheet_note (l_sheet_note_id INTEGER PRIMARY KEY AUTOINCREMENT, l_sheet_code INTEGER, l_note_code INTEGER, b_synopsis BOOLEAN DEFAULT 0);
             CREATE TABLE tbl_tag (l_tag_id INTEGER PRIMARY KEY AUTOINCREMENT, t_name TEXT, t_color TEXT, t_text_color TEXT, dt_created DATETIME DEFAULT CURRENT_TIMESTAMP, dt_updated DATETIME DEFAULT CURRENT_TIMESTAMP);
             CREATE TABLE tbl_tag_relationship (l_tag_relationship_id INTEGER PRIMARY KEY AUTOINCREMENT, l_sheet_code INTEGER, l_note_code INTEGER, l_tag_code INTEGER, dt_created DATETIME DEFAULT CURRENT_TIMESTAMP);
             CREATE TABLE tbl_project_dict (l_project_dict_id INTEGER PRIMARY KEY AUTOINCREMENT, t_word TEXT);
             CREATE TABLE tbl_stat_history (l_stat_history_id INTEGER PRIMARY KEY AUTOINCREMENT, dt_saved DATETIME, l_sheet_char_count INTEGER, l_sheet_word_count INTEGER, l_note_char_count INTEGER, l_note_word_count INTEGER);

             -- 'Chapter One' (indent 0) is the parent of 'Scene A' (indent 1) → folder transform.
             INSERT INTO tbl_sheet (l_sheet_id, t_title, l_sort_order, l_indent, m_content) VALUES (1, 'Chapter One', 0, 0, 'Chapter **intro**.');
             INSERT INTO tbl_sheet (l_sheet_id, t_title, l_sort_order, l_indent, m_content) VALUES (2, 'Scene A', 1, 1, 'Hello *world*.');
             -- a synopsis note attached to Scene A.
             INSERT INTO tbl_note (l_note_id, t_title, l_sort_order, l_indent, m_content) VALUES (1, 'Synopsis', 0, 1, 'Outline of the scene.');
             INSERT INTO tbl_sheet_note (l_sheet_code, l_note_code, b_synopsis) VALUES (2, 1, 1);
             -- a tag on Scene A.
             INSERT INTO tbl_tag (l_tag_id, t_name, t_color, t_text_color) VALUES (1, 'Important', '#f00', '#fff');
             INSERT INTO tbl_tag_relationship (l_sheet_code, l_tag_code) VALUES (2, 1);
             INSERT INTO tbl_project_dict (t_word) VALUES ('Skribisto');",
        )
        .unwrap();
        conn
    }

    #[test]
    fn version_detected_as_tenths() {
        let conn = make_v1_4_db();
        assert_eq!(detect_version(&conn).unwrap(), 14);
    }

    /// The legacy path's own ceiling — the counterpart to `skrib_format::version_gate`,
    /// which it can never reach: `load_work_uc` dispatches a legacy file straight to
    /// `legacy::read_project` without ever constructing a `WorkBundle`, so a fix confined
    /// to the format crate would silently miss every legacy project. That asymmetry has
    /// already bitten this codebase once, over uid minting (see `migration.rs`'s module
    /// doc and `a_legacy_project_gets_a_distinct_uid_for_every_row`).
    ///
    /// `v >= 20` used to read "already current", which quietly treated a *newer* database
    /// as current and then ran the importer against a schema it does not understand.
    /// Moot in practice — the format is retired — but the class of bug is not.
    #[test]
    fn a_legacy_database_above_the_supported_ceiling_is_refused_not_called_current() {
        let conn = make_v1_4_db();
        conn.execute("UPDATE tbl_project SET dbl_database_version = 2.1", [])
            .unwrap();
        assert_eq!(detect_version(&conn).unwrap(), 21);

        let err = upgrade_to_v2(&conn).expect_err("a 2.1 database must be refused");
        let msg = format!("{err:#}");
        assert!(
            msg.contains("2.1") && msg.contains("2.0"),
            "the message should name both the file's version and our ceiling, got: {msg}"
        );
    }

    /// The exact-2.0 fast path must survive the ceiling being added above it.
    #[test]
    fn a_legacy_database_already_at_2_0_is_left_alone() {
        let conn = make_v1_4_db();
        conn.execute("UPDATE tbl_project SET dbl_database_version = 2.0", [])
            .unwrap();
        upgrade_to_v2(&conn).expect("2.0 is current, not an error");
        assert_eq!(detect_version(&conn).unwrap(), 20);
    }

    #[test]
    fn full_chain_from_v1_4() {
        let conn = make_v1_4_db();
        upgrade_to_v2(&conn).expect("upgrade 1.4 → 2.0");

        assert_eq!(detect_version(&conn).unwrap(), 20, "should land on 2.0");

        // The sheet/note model became tbl_tree: a PROJECT root, the chapter folder,
        // and the two text items. No Trash folder is created.
        let tree_count: i64 = conn
            .query_row("SELECT COUNT(*) FROM tbl_tree", [], |r| r.get(0))
            .unwrap();
        assert!(
            tree_count >= 4,
            "expected several tree rows, got {tree_count}"
        );
        let trash: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tbl_tree WHERE t_internal_title = 'trash_folder'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(trash, 0, "no Trash folder — trashed items stay in place");

        // The synopsis note's content moved into Scene A's secondary content.
        let with_secondary: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tbl_tree WHERE length(COALESCE(m_secondary_content,'')) > 0",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(
            with_secondary >= 1,
            "synopsis should move to secondary content"
        );

        // End-to-end: the v3 mapping reads the upgraded tree.
        let project = super::super::read_v2(&conn, ":memory:").expect("read_v2");
        assert_eq!(project.title, "My Novel");
        assert_eq!(project.author, "Jane");
        assert_eq!(
            project.unique_id, "abc123XYZ000",
            "legacy project unique id survives the 1.0→2.0 upgrade + read_v2"
        );
        assert_eq!(project.dict_words, vec!["Skribisto".to_string()]);
        assert_eq!(project.tags.len(), 1, "the tag should carry over");

        let items: Vec<_> = project.binders.iter().flat_map(|b| &b.items).collect();
        assert!(!items.is_empty(), "expected binder items");
        // Content is Djot, never Qt HTML.
        for item in &items {
            for content in &item.contents {
                assert!(
                    !content.data.contains("<!DOCTYPE") && !content.data.contains("qrichtext"),
                    "content should be Djot, got: {:?}",
                    content.data
                );
            }
        }
    }
}
