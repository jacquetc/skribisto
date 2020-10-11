--
-- File generated with SQLiteStudio v3.2.1 on jeu. janv. 30 22:52:23 2020
--
-- Text encoding used: UTF-8
--
PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: tbl_history
CREATE TABLE tbl_history (dt_saved DATETIME, l_char_count INTEGER, l_word_count INTEGER);

-- Table: tbl_note
CREATE TABLE tbl_note (l_note_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, t_title TEXT, l_sort_order INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (9999999999), l_indent INTEGER DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK, l_version INTEGER, l_dna INTEGER, l_xpos INTEGER, l_ypos INTEGER, m_content BLOB, dt_created DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL ON CONFLICT ROLLBACK, dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_content DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL ON CONFLICT ROLLBACK, dt_trashed DATETIME, b_trashed BOOLEAN DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK, l_word_count INTEGER, l_char_count INTEGER, l_size INTEGER);

-- Index: idx_note
CREATE INDEX idx_note ON tbl_note (l_note_id, t_title, l_sort_order ASC, l_indent, b_trashed);

-- Table: tbl_note_property
CREATE TABLE tbl_note_property (l_property_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE, l_note_code INTEGER REFERENCES tbl_sheet ON DELETE CASCADE, t_name TEXT DEFAULT NULL, t_value TEXT, dt_created DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL, dt_updated DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), b_system BOOLEAN DEFAULT (0) NOT NULL);

-- Table: tbl_project
CREATE TABLE tbl_project (l_project_id INTEGER PRIMARY KEY ASC ON CONFLICT ROLLBACK AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, t_project_name TEXT, dbl_database_version DOUBLE NOT NULL DEFAULT (0), dt_created DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL, dt_updated DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), t_author TEXT, t_project_unique_identifier TEXT, t_spell_check_lang TEXT NOT NULL ON CONFLICT ROLLBACK DEFAULT "");

-- Table: tbl_project_dict
CREATE TABLE tbl_project_dict (l_project_dict_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, t_word TEXT UNIQUE ON CONFLICT REPLACE NOT NULL ON CONFLICT ROLLBACK);

-- Table: tbl_sheet
CREATE TABLE tbl_sheet (l_sheet_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, l_sort_order INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (9999999999), l_indent INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (0), l_version INTEGER, l_dna INTEGER, t_title TEXT, m_content BLOB, l_char_count INTEGER, l_word_count INTEGER, dt_created DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_content DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_trashed DATETIME, b_trashed BOOLEAN NOT NULL ON CONFLICT ROLLBACK DEFAULT (0));

-- Index: idx_sheet
CREATE INDEX idx_sheet ON tbl_sheet (l_sheet_id, l_sort_order ASC, l_indent, t_title, b_trashed);

-- Table: tbl_sheet_property
CREATE TABLE tbl_sheet_property (l_property_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE, l_sheet_code INTEGER REFERENCES tbl_sheet (l_sheet_id) ON DELETE CASCADE, t_name TEXT DEFAULT NULL, t_value TEXT, dt_created DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL, b_system BOOLEAN DEFAULT (0) NOT NULL);

-- Table: tbl_sheet_note
CREATE TABLE tbl_sheet_note (l_sheet_note_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE, l_sheet_code INTEGER REFERENCES tbl_sheet (l_sheet_id) ON DELETE CASCADE NOT NULL, l_note_code INTEGER REFERENCES tbl_note (l_note_id) ON DELETE CASCADE NOT NULL, b_synopsis BOOLEAN NOT NULL ON CONFLICT ROLLBACK DEFAULT (0));

-- Table: tbl_tag
CREATE TABLE tbl_tag (l_tag_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, t_name TEXT NOT NULL UNIQUE ON CONFLICT ROLLBACK, t_color TEXT DEFAULT lightblue, dt_created DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP));

-- Table: tbl_tag_relationship
CREATE TABLE tbl_tag_relationship (l_tag_relationship_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK, l_sheet_code INTEGER REFERENCES tbl_sheet (l_sheet_id) ON DELETE CASCADE, l_note_code INTEGER REFERENCES tbl_note (l_note_id) ON DELETE CASCADE, l_tag_code INTEGER REFERENCES tbl_tag (l_tag_id) ON DELETE CASCADE, dt_created DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP));

-- Trigger: trg_delete_properties
CREATE TRIGGER trg_delete_properties AFTER DELETE ON tbl_sheet FOR EACH ROW BEGIN DELETE FROM tbl_sheet_property WHERE l_sheet_code = OLD.l_sheet_id; END;

-- View: v_property_sheet
CREATE VIEW v_property_sheet AS SELECT l_property_id, l_sheet_code, t_name, t_value FROM tbl_sheet_property;

-- View: v_tree_sheet
CREATE VIEW v_tree_sheet AS SELECT l_sheet_id, l_sort_order, l_indent, t_title, b_trashed FROM tbl_sheet;

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
