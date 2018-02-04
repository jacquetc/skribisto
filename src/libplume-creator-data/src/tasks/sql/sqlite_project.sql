--
-- File generated with SQLiteStudio v3.1.1 on mar. janv. 9 23:12:38 2018
--
-- Text encoding used: UTF-8
--
PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: tbl_history
CREATE TABLE tbl_history (dt_saved DATETIME, l_char_count INTEGER, l_word_count INTEGER);

-- Table: tbl_info
CREATE TABLE tbl_info (dbl_version DOUBLE);

-- Table: tbl_note
CREATE TABLE tbl_note (l_note_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, t_title TEXT, l_sheet_code INTEGER REFERENCES tbl_sheet (l_sheet_id) ON DELETE SET NULL ON UPDATE NO ACTION, l_sort_order INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (9999999999), l_indent INTEGER DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK, l_version INTEGER, l_dna INTEGER, l_xpos INTEGER, l_ypos INTEGER, m_content BLOB, dt_created DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL ON CONFLICT ROLLBACK, dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_content DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL ON CONFLICT ROLLBACK, b_deleted BOOLEAN DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK, l_word_count INTEGER, l_char_count INTEGER, l_size INTEGER, b_synopsis BOOLEAN DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK);

-- Table: tbl_note_property
CREATE TABLE tbl_note_property (l_property_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE, l_note_code INTEGER REFERENCES tbl_sheet ON DELETE CASCADE, t_name TEXT NOT NULL DEFAULT nothing, t_value TEXT, dt_created DATETIME, dt_updated DATETIME, b_system BOOLEAN DEFAULT (0) NOT NULL);

-- Table: tbl_project
CREATE TABLE tbl_project (t_title TEXT, l_plume_maj_version INTEGER, l_plume_min_version INTEGER, l_db_maj_version INTEGER, l_db_min_version INTEGER, dt_created DATETIME, dt_updated DATETIME, t_author TEXT);

-- Table: tbl_sheet
CREATE TABLE tbl_sheet (l_sheet_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, l_sort_order INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (9999999999), l_indent INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (0), l_version INTEGER, l_dna INTEGER, t_title TEXT, m_content BLOB, l_char_count INTEGER, l_word_count INTEGER, dt_created DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_content DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), b_deleted BOOLEAN NOT NULL ON CONFLICT ROLLBACK DEFAULT (0));

-- Table: tbl_sheet_property
CREATE TABLE tbl_sheet_property (l_property_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE, l_sheet_code INTEGER REFERENCES tbl_sheet (l_sheet_id) ON DELETE CASCADE, t_name TEXT NOT NULL DEFAULT nothing, t_value TEXT, dt_created DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL, b_system DEFAULT (0) NOT NULL);

-- Index: idx1
CREATE INDEX idx1 ON tbl_sheet (l_sheet_id, l_sort_order ASC, l_indent, t_title, b_deleted);

-- Trigger: trg_delete_properties
CREATE TRIGGER trg_delete_properties AFTER DELETE ON tbl_sheet FOR EACH ROW BEGIN DELETE FROM tbl_sheet_property WHERE l_sheet_code = OLD.l_sheet_id; END;

-- View: v_property_sheet
CREATE VIEW v_property_sheet AS SELECT l_property_id, l_sheet_code, t_name, t_value FROM tbl_sheet_property;

-- View: v_tree_sheet
CREATE VIEW v_tree_sheet AS SELECT l_sheet_id, l_sort_order, l_indent, t_title, b_deleted FROM tbl_sheet;

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
