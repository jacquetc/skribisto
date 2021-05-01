--
-- File generated with SQLiteStudio v3.2.1 on jeu. janv. 30 22:52:23 2020
--
-- Text encoding used: UTF-8
--
-- skribisto_db_version:1.7

PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table: tbl_stat_history
CREATE TABLE tbl_stat_history (l_stat_history_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, dt_saved DATETIME, l_sheet_char_count INTEGER, l_sheet_word_count INTEGER, l_note_char_count INTEGER, l_note_word_count INTEGER);

-- Table: tbl_project
CREATE TABLE tbl_project (l_project_id INTEGER PRIMARY KEY ASC ON CONFLICT ROLLBACK AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, t_project_name TEXT, dbl_database_version DOUBLE NOT NULL DEFAULT (0), dt_created DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL, dt_updated DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), t_author TEXT, t_project_unique_identifier TEXT, t_spell_check_lang TEXT NOT NULL ON CONFLICT ROLLBACK DEFAULT "");

-- Table: tbl_project_dict
CREATE TABLE tbl_project_dict (l_project_dict_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT ROLLBACK, t_word TEXT UNIQUE ON CONFLICT REPLACE NOT NULL ON CONFLICT ROLLBACK);

-- Table: tbl_tree_relationship
CREATE TABLE tbl_tree_relationship (l_tree_relationship_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, l_tree_source_code INTEGER REFERENCES tbl_tree (l_tree_id) NOT NULL ON CONFLICT ROLLBACK, l_tree_receiver_code   INTEGER REFERENCES tbl_tree (l_tree_id) NOT NULL ON CONFLICT ROLLBACK);

-- Table: tbl_tree
CREATE TABLE tbl_tree (l_tree_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, t_title TEXT, t_internal_title TEXT, l_sort_order INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (9999999999), l_indent INTEGER NOT NULL ON CONFLICT ROLLBACK DEFAULT (0), t_type TEXT, m_primary_content BLOB, m_secondary_content BLOB, dt_created DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_trashed DATETIME, b_trashed BOOLEAN NOT NULL ON CONFLICT ROLLBACK DEFAULT (0));

-- Table: tbl_tree_property
CREATE TABLE tbl_tree_property (l_tree_property_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, l_tree_code INTEGER REFERENCES tbl_tree (l_tree_id), t_name TEXT, t_value_type TEXT NOT NULL ON CONFLICT ROLLBACK DEFAULT STRING, m_value BLOB, dt_created DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), b_system BOOLEAN DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK, b_silent BOOLEAN DEFAULT (0) NOT NULL ON CONFLICT ROLLBACK);

-- Table: tbl_tag
CREATE TABLE tbl_tag (l_tag_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL ON CONFLICT ROLLBACK, t_name TEXT NOT NULL UNIQUE ON CONFLICT ROLLBACK, t_color TEXT, t_text_color TEXT, dt_created DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP), dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP));

-- Table: tbl_tag_relationship
CREATE TABLE tbl_tag_relationship (l_tag_relationship_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK, l_tree_code INTEGER REFERENCES tbl_tree (l_tree_id), l_tag_code INTEGER REFERENCES tbl_tag (l_tag_id), dt_created DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP));


-- project item
INSERT INTO tbl_tree (l_tree_id, l_sort_order, l_indent, t_type) VALUES (0, -1, 0, 'PROJECT');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_renamable', 'BOOL', 'true');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_movable', 'BOOL', 'false');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'can_add_sibling_paper', 'BOOL', 'false');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'can_add_child_paper', 'BOOL', 'true');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_trashable', 'BOOL', 'false');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_openable', 'BOOL', 'true');
INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_copyable', 'BOOL', 'false');






COMMIT TRANSACTION;
PRAGMA foreign_keys = on;
