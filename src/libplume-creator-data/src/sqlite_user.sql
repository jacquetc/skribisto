--
-- Fichier généré par SQLiteStudio v3.1.1 sur dim. oct. 14 22:22:07 2018
--
-- Encodage texte utilisé : UTF-8
--
PRAGMA foreign_keys = off;
BEGIN TRANSACTION;

-- Table : tbl_note_doc_list
CREATE TABLE tbl_note_doc_list (    l_sheet_id   INTEGER  NOT NULL                          PRIMARY KEY                          UNIQUE,    l_stack      INTEGER  DEFAULT (0)                           NOT NULL,    b_hovering   BOOLEAN  DEFAULT (0)                           NOT NULL,    b_visible    BOOLEAN  NOT NULL                          DEFAULT (0),    b_has_focus  BOOLEAN  NOT NULL                          DEFAULT (0),    l_cursor_pos INTEGER  DEFAULT (0)                           NOT NULL,    m_geometry   BLOB     DEFAULT (0)                           NOT NULL,    dt_updated   DATETIME NOT NULL ON CONFLICT ROLLBACK                          DEFAULT (CURRENT_TIMESTAMP) );

-- Table : tbl_note_setting
CREATE TABLE tbl_note_setting (    l_id                 INTEGER  PRIMARY KEY AUTOINCREMENT                                  UNIQUE                                  NOT NULL,    m_splitter_state     BLOB     DEFAULT (0)                                   NOT NULL,    b_stack_0_map        BOOLEAN  NOT NULL                                  DEFAULT (1),    b_stack_1_map        BOOLEAN  NOT NULL                                  DEFAULT (1),    b_stack_0_fit        BOOLEAN  NOT NULL                                  DEFAULT (0),    b_stack_1_fit        BOOLEAN  DEFAULT (0)                                   NOT NULL,    b_stack_0_spellcheck BOOLEAN  NOT NULL                                  DEFAULT (1),    b_stack_1_spellcheck BOOLEAN  DEFAULT (1)                                   NOT NULL,    m_stack_0_state      BLOB     NOT NULL                                  DEFAULT (0),    m_stack_1_state      BLOB     NOT NULL                                  DEFAULT (0),    dt_updated           DATETIME DEFAULT (CURRENT_TIMESTAMP)                                   NOT NULL ON CONFLICT ROLLBACK,    m_window_state       BLOB     NOT NULL                                  DEFAULT (0) );

-- Table : tbl_sheet_doc_list
CREATE TABLE tbl_sheet_doc_list (l_sheet_id INTEGER NOT NULL PRIMARY KEY UNIQUE, l_stack INTEGER DEFAULT (0) NOT NULL, b_hovering BOOLEAN DEFAULT (0) NOT NULL, b_visible BOOLEAN NOT NULL DEFAULT (0), b_has_focus BOOLEAN NOT NULL DEFAULT (0), l_cursor_pos INTEGER DEFAULT (0) NOT NULL, m_geometry BLOB DEFAULT (0) NOT NULL, dt_updated DATETIME NOT NULL ON CONFLICT ROLLBACK DEFAULT (CURRENT_TIMESTAMP));

-- Table : tbl_sheet_setting
CREATE TABLE tbl_sheet_setting (l_id INTEGER PRIMARY KEY AUTOINCREMENT UNIQUE NOT NULL, m_splitter_state BLOB DEFAULT (0) NOT NULL, b_stack_0_map BOOLEAN NOT NULL DEFAULT (1), b_stack_1_map BOOLEAN NOT NULL DEFAULT (1), b_stack_0_fit BOOLEAN NOT NULL DEFAULT (0), b_stack_1_fit BOOLEAN DEFAULT (0) NOT NULL, b_stack_0_spellcheck BOOLEAN NOT NULL DEFAULT (1), b_stack_1_spellcheck BOOLEAN DEFAULT (1) NOT NULL, m_stack_0_state BLOB NOT NULL DEFAULT (0), m_stack_1_state BLOB NOT NULL DEFAULT (0), dt_updated DATETIME DEFAULT (CURRENT_TIMESTAMP) NOT NULL ON CONFLICT ROLLBACK, m_window_state BLOB NOT NULL DEFAULT (0));

COMMIT TRANSACTION;
PRAGMA foreign_keys = on;

