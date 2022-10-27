/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: upgrader.cpp                                                   *
 *  This file is part of Skribisto.                                    *
 *                                                                         *
 *  Skribisto is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Skribisto is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/
#include "upgrader.h"
#include "sql/skrsqltools.h"
#include "markdowntextdocument.h"

#include <QDebug>
#include <QSqlError>
#include <QSqlQuery>
#include <QTextDocument>

Upgrader::Upgrader(QObject *parent) : QObject(parent) {}

SKRResult Upgrader::upgradeSQLite(const QString &sqlDbConnectionName) {
  SKRResult result("SKRSqlTools::upgradeSQLite");

  // find DB version :
  double dbVersion =
      SKRSqlTools::getProjectDBVersion(&result, sqlDbConnectionName);

  QSqlDatabase sqlDb = QSqlDatabase::database(sqlDbConnectionName);

  // ---------------------------------

  // from 1.0 to 1.1
  if (dbVersion == 1.0) {
    double newDbVersion = 1.1;

    sqlDb.transaction();

    QSqlQuery query(sqlDb);
    QString queryStr =
        "CREATE TABLE tbl_project_dict (l_project_dict_id INTEGER PRIMARY KEY "
        "AUTOINCREMENT NOT NULL ON CONFLICT ROLLBACK UNIQUE ON CONFLICT "
        "ROLLBACK, t_word TEXT UNIQUE ON CONFLICT REPLACE NOT NULL ON CONFLICT "
        "ROLLBACK);";
    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
    }

    IFOK(result) { sqlDb.commit(); }

    IFKO(result) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "cant_upgrade");
      result.addData("dbVersion", newDbVersion);
      result.addData("SQLError", query.lastError().text());
    }

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }

  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.1 to 1.2
  if (dbVersion == 1.1) {
    double newDbVersion = 1.2;

    QString queryStr =
        R""""(PRAGMA foreign_keys = 0;

                CREATE TABLE temp_table AS SELECT * FROM tbl_sheet_note;

                DROP TABLE tbl_sheet_note;

                CREATE TABLE tbl_sheet_note (
                l_sheet_note_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                UNIQUE ON CONFLICT ROLLBACK
                NOT NULL ON CONFLICT ROLLBACK,
                l_sheet_code    INTEGER REFERENCES tbl_sheet (l_sheet_id) ON DELETE CASCADE
                NOT NULL ON CONFLICT ROLLBACK,
                l_note_code     INTEGER REFERENCES tbl_note (l_note_id) ON DELETE CASCADE
                NOT NULL ON CONFLICT ROLLBACK,
                b_synopsis      BOOLEAN NOT NULL ON CONFLICT ROLLBACK
                DEFAULT (0)
                );

                INSERT INTO tbl_sheet_note (
                l_sheet_note_id,
                l_sheet_code,
                l_note_code,
                b_synopsis
                )
                SELECT l_sheet_note_id,
                l_sheet_code,
                l_note_code,
                b_synopsis
                FROM temp_table;

                DROP TABLE temp_table;

                PRAGMA foreign_keys = 1;
                )"""";

    result = SKRSqlTools::executeSQLString(queryStr, sqlDb);

    IFKO(result) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "cant_upgrade");
      result.addData("dbVersion", newDbVersion);
    }

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }

  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.2 to 1.3
  if (dbVersion == 1.2) {
    double newDbVersion = 1.3;

    QString queryStr =
        R""""(
                PRAGMA foreign_keys = 0;

                CREATE TABLE tbl_stat_history (
                l_stat_history_id  INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                UNIQUE ON CONFLICT ROLLBACK
                NOT NULL ON CONFLICT ROLLBACK,
                dt_saved           DATETIME,
                l_sheet_char_count INTEGER,
                l_sheet_word_count INTEGER,
                l_note_char_count  INTEGER,
                l_note_word_count  INTEGER
                );

                INSERT INTO tbl_stat_history (
                dt_saved,
                l_sheet_char_count,
                l_sheet_word_count
                )
                SELECT dt_saved,
                l_char_count,
                l_word_count
                FROM tbl_history;

                DROP TABLE tbl_history;

                PRAGMA foreign_keys = 1;

                )"""";

    result = SKRSqlTools::executeSQLString(queryStr, sqlDb);

    IFKO(result) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "cant_upgrade");
      result.addData("dbVersion", newDbVersion);
    }

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }

  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.3 to 1.4
  if (dbVersion == 1.3) {
    double newDbVersion = 1.4;

    QString queryStr =
        R""""(PRAGMA foreign_keys = 0;

                CREATE TABLE sqlitestudio_temp_table AS SELECT *
                FROM tbl_tag;

                DROP TABLE tbl_tag;

                CREATE TABLE tbl_tag (
                l_tag_id     INTEGER  PRIMARY KEY ASC ON CONFLICT ROLLBACK AUTOINCREMENT
                UNIQUE ON CONFLICT ROLLBACK
                NOT NULL ON CONFLICT ROLLBACK,
                t_name       TEXT     NOT NULL
                UNIQUE ON CONFLICT ROLLBACK,
                t_color      TEXT,
                t_text_color TEXT,
                dt_created   DATETIME NOT NULL ON CONFLICT ROLLBACK
                DEFAULT (CURRENT_TIMESTAMP),
                dt_updated   DATETIME NOT NULL ON CONFLICT ROLLBACK
                DEFAULT (CURRENT_TIMESTAMP)
                );

                INSERT INTO tbl_tag (
                l_tag_id,
                t_name,
                t_color,
                dt_created,
                dt_updated
                )
                SELECT l_tag_id,
                t_name,
                t_color,
                dt_created,
                dt_updated
                FROM sqlitestudio_temp_table;

                DROP TABLE sqlitestudio_temp_table;

                PRAGMA foreign_keys = 1;

                )"""";

    result = SKRSqlTools::executeSQLString(queryStr, sqlDb);

    IFKO(result) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "cant_upgrade");
      result.addData("dbVersion", newDbVersion);
    }

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }

  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.4 to 1.5
  if (dbVersion == 1.4) {
    double newDbVersion = 1.5;

    QString queryStr =
        R""""(PRAGMA foreign_keys = 0;

                CREATE TABLE tbl_tree (
                    l_tree_id    INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                          UNIQUE ON CONFLICT ROLLBACK
                                          NOT NULL ON CONFLICT ROLLBACK,
                    t_title      TEXT,
                    l_sort_order INTEGER  NOT NULL ON CONFLICT ROLLBACK
                                          DEFAULT (9999999999),
                    l_indent     INTEGER  NOT NULL ON CONFLICT ROLLBACK
                                          DEFAULT (0),
                    t_type       TEXT,
                    m_primary_content    BLOB,
                    m_secondary_content    BLOB,
                    dt_created   DATETIME NOT NULL ON CONFLICT ROLLBACK
                                          DEFAULT (CURRENT_TIMESTAMP),
                    dt_updated   DATETIME NOT NULL ON CONFLICT ROLLBACK
                                          DEFAULT (CURRENT_TIMESTAMP),
                    dt_trashed   DATETIME,
                    b_trashed    BOOLEAN  NOT NULL ON CONFLICT ROLLBACK
                                          DEFAULT (0)
                );







                CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                          FROM tbl_tag_relationship;

                DROP TABLE tbl_tag_relationship;

                CREATE TABLE tbl_tag_relationship (
                    l_tag_relationship_id INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                   NOT NULL ON CONFLICT ROLLBACK,
                    l_sheet_code          INTEGER  REFERENCES tbl_sheet (l_sheet_id),
                    l_note_code           INTEGER  REFERENCES tbl_note (l_note_id),
                    l_tree_code           INTEGER  REFERENCES tbl_tree (l_tree_id),
                    l_tag_code            INTEGER  REFERENCES tbl_tag (l_tag_id),
                    dt_created            DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                   DEFAULT (CURRENT_TIMESTAMP)
                );

                INSERT INTO tbl_tag_relationship (
                                     l_tag_relationship_id,
                                     l_sheet_code,
                                     l_note_code,
                                     l_tag_code,
                                     dt_created
                                 )
                                 SELECT l_tag_relationship_id,
                                        l_sheet_code,
                                        l_note_code,
                                        l_tag_code,
                                        dt_created
                                   FROM sqlitestudio_temp_table;

                DROP TABLE sqlitestudio_temp_table;




                CREATE TABLE tbl_tree_property (
                    l_tree_property_id INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                UNIQUE ON CONFLICT ROLLBACK
                                                NOT NULL ON CONFLICT ROLLBACK,
                    l_tree_code        INTEGER  REFERENCES tbl_tree (l_tree_id),
                    t_name             TEXT,
                    t_value_type       TEXT     NOT NULL ON CONFLICT ROLLBACK
                                                DEFAULT STRING,
                    m_value            BLOB,
                    dt_created         DATETIME NOT NULL
                                                DEFAULT (CURRENT_TIMESTAMP),
                    dt_updated         DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                DEFAULT (CURRENT_TIMESTAMP),
                    b_system           BOOLEAN  DEFAULT (0)
                                                NOT NULL ON CONFLICT ROLLBACK,
                    b_silent           BOOLEAN  DEFAULT (0)
                                                NOT NULL ON CONFLICT ROLLBACK
                );







                CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                          FROM tbl_sheet_note;

                DROP TABLE tbl_sheet_note;

                CREATE TABLE tbl_sheet_note (
                    l_sheet_note_id      INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                 UNIQUE ON CONFLICT ROLLBACK
                                                 NOT NULL ON CONFLICT ROLLBACK,
                    l_tree_source_code   INTEGER,
                    l_tree_receiver_code INTEGER,
                    l_sheet_code         INTEGER REFERENCES tbl_sheet (l_sheet_id)
                                                 NOT NULL ON CONFLICT ROLLBACK,
                    l_note_code          INTEGER REFERENCES tbl_note (l_note_id)
                                                 NOT NULL ON CONFLICT ROLLBACK,
                    b_synopsis           BOOLEAN NOT NULL ON CONFLICT ROLLBACK
                                                 DEFAULT (0)
                );

                INSERT INTO tbl_sheet_note (
                                               l_sheet_note_id,
                                               l_sheet_code,
                                               l_note_code,
                                               b_synopsis
                                           )
                                           SELECT l_sheet_note_id,
                                                  l_sheet_code,
                                                  l_note_code,
                                                  b_synopsis
                                             FROM sqlitestudio_temp_table;

                DROP TABLE sqlitestudio_temp_table;




                PRAGMA foreign_keys = 1;
                )"""";

    result = SKRSqlTools::executeSQLString(queryStr, sqlDb);

    // migrate tables

    // -- from tbl_sheet to tbl_tree
    IFOKDO(result, Upgrader::movePaperToTree_1_5(sqlDb, "tbl_sheet"));

    // -- from tbl_note to tbl_tree
    IFOKDO(result, Upgrader::movePaperToTree_1_5(sqlDb, "tbl_note"));

    // transform parents sheet/note in tree in folders, move parent as first
    // child of folder
    IFOKDO(result, Upgrader::transformParentsToFolder_1_5(sqlDb));

    // -- remove from paper codes from tbl_tag_relationship

    queryStr =
        R""""(PRAGMA foreign_keys = 0;

                CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                          FROM tbl_tag_relationship;

                DROP TABLE tbl_tag_relationship;

                CREATE TABLE tbl_tag_relationship (
                    l_tag_relationship_id INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                   NOT NULL ON CONFLICT ROLLBACK,
                    l_tree_code           INTEGER  REFERENCES tbl_tree (l_tree_id),
                    l_tag_code            INTEGER  REFERENCES tbl_tag (l_tag_id),
                    dt_created            DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                   DEFAULT (CURRENT_TIMESTAMP)
                );

                INSERT INTO tbl_tag_relationship (
                                                     l_tag_relationship_id,
                                                     l_tree_code,
                                                     l_tag_code,
                                                     dt_created
                                                 )
                                                 SELECT l_tag_relationship_id,
                                                        l_tree_code,
                                                        l_tag_code,
                                                        dt_created
                                                   FROM sqlitestudio_temp_table;

                DROP TABLE sqlitestudio_temp_table;






                CREATE TABLE tbl_tree_relationship (
                    l_tree_relationship_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                   UNIQUE ON CONFLICT ROLLBACK
                                                   NOT NULL ON CONFLICT ROLLBACK,
                    l_tree_source_code     INTEGER REFERENCES tbl_tree (l_tree_id)
                                                   NOT NULL ON CONFLICT ROLLBACK,
                    l_tree_receiver_code   INTEGER REFERENCES tbl_tree (l_tree_id)
                                                   NOT NULL ON CONFLICT ROLLBACK,
                    b_synopsis             BOOLEAN NOT NULL ON CONFLICT ROLLBACK
                                                   DEFAULT (0)
                );

                INSERT INTO tbl_tree_relationship (
                                                      l_tree_relationship_id,
                                                      l_tree_source_code,
                                                      l_tree_receiver_code,
                                                      b_synopsis
                                                  )
                                                  SELECT l_sheet_note_id,
                                                         l_tree_source_code,
                                                         l_tree_receiver_code,
                                                         b_synopsis
                                                    FROM tbl_sheet_note;

                DROP TABLE tbl_sheet_note;



                PRAGMA foreign_keys = 1;
                )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));

    // -- drop deprecated tables

    IFOKDO(result, Upgrader::dropDeprecatedTables_1_5(sqlDb));

    IFOKDO(result, SKRSqlTools::renumberTreeSortOrder(sqlDb));

    IFKO(result) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "cant_upgrade");
      result.addData("dbVersion", newDbVersion);
    }

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }

  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.5 to 1.6
  if (dbVersion == 1.5) {
    double newDbVersion = 1.6;

    // increment all indent in tree
    QString queryStr = "UPDATE tbl_tree SET l_indent = l_indent + 1;";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));

    // create project item in tree

    queryStr =
        R""""(PRAGMA foreign_keys = 0;
                INSERT INTO tbl_tree (l_tree_id,l_sort_order, l_indent, t_type)
                        VALUES (0, -1, 0, 'PROJECT');

                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'is_renamable', 'BOOL', 'true', 1);
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'is_movable', 'BOOL', 'false', 1);
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'can_add_sibling_paper', 'BOOL', 'false', 1);
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'can_add_child_paper', 'BOOL', 'true', 1);
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'is_trashable', 'BOOL', 'false', 1);
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'is_openable', 'BOOL', 'true', 1);
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value, b_system) VALUES (0, 'is_copyable', 'BOOL', 'false', 1);


                PRAGMA foreign_keys = 1;
                )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));

    // move all synopsis to text secondary content

    IFOKDO(result, Upgrader::moveSynopsisToSecondaryContent_1_6(sqlDb));

    queryStr =
        R""""(PRAGMA foreign_keys = 0;

        CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                  FROM tbl_tree_relationship;

        DROP TABLE tbl_tree_relationship;

        CREATE TABLE tbl_tree_relationship (
            l_tree_relationship_id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                           UNIQUE ON CONFLICT ROLLBACK
                                           NOT NULL ON CONFLICT ROLLBACK,
            l_tree_source_code     INTEGER REFERENCES tbl_tree (l_tree_id)
                                           NOT NULL ON CONFLICT ROLLBACK,
            l_tree_receiver_code   INTEGER REFERENCES tbl_tree (l_tree_id)
                                           NOT NULL ON CONFLICT ROLLBACK
        );

        INSERT INTO tbl_tree_relationship (
                                              l_tree_relationship_id,
                                              l_tree_source_code,
                                              l_tree_receiver_code
                                          )
                                          SELECT l_tree_relationship_id,
                                                 l_tree_source_code,
                                                 l_tree_receiver_code
                                            FROM sqlitestudio_temp_table;

        DROP TABLE sqlitestudio_temp_table;

        PRAGMA foreign_keys = 1;
        )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));

    IFOKDO(result, SKRSqlTools::renumberTreeSortOrder(sqlDb));

    IFOKDO(result, SKRSqlTools::trimTagRelationshipTable(sqlDb));
    IFOKDO(result, SKRSqlTools::trimTreePropertyTable(sqlDb));
    IFOKDO(result, SKRSqlTools::trimTreeRelationshipTable(sqlDb));

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }
  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.6 to 1.7
  if (dbVersion == 1.6) {
    double newDbVersion = 1.7;

    // fetch all TEXT items
    QString queryStr = "SELECT l_tree_id FROM tbl_tree WHERE t_type = 'TEXT';";

    QSqlQuery query(sqlDb);

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      return result;
    }

    QList<int> textIdList;

    while (query.next()) {
      textIdList.append(query.value(0).toInt());
    }

    // fetch all properties with can_add_child_paper

    queryStr = "SELECT l_tree_code FROM tbl_tree_property WHERE "
               "t_name='can_add_child_paper'";

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      return result;
    }

    QList<int> treeCodeList;

    while (query.next()) {
      treeCodeList.append(query.value(0).toInt());
    }

    // remove already set properties from idList
    QList<int> otherTextIdList = textIdList;

    for (int treeCode : treeCodeList) {
      otherTextIdList.removeAll(treeCode);
    }

    // apply properties to all TEXT items
    sqlDb.transaction();

    for (int treeItemId : otherTextIdList) {
      queryStr = "INSERT INTO tbl_tree_property(l_tree_code, t_name, "
                 "t_value_type, m_value) VALUES(:treeItemId, "
                 "'can_add_child_paper', 'INT', 'false');";

      query.prepare(queryStr);
      query.bindValue(":treeItemId", treeItemId);
      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
      }
    }

    QTextDocument *textDocument = new QTextDocument();

    for (int treeItemId : qAsConst(textIdList)) {
      // convert TEXT markdown content into HTML

      queryStr =
          "SELECT m_primary_content FROM tbl_tree WHERE l_tree_id=:treeItemId";
      query.prepare(queryStr);
      query.bindValue(":treeItemId", treeItemId);
      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
      }

      QString content;

      while (query.next()) {
        content = query.value(0).toString();
      }

      textDocument->setMarkdown(content);
      QString html = textDocument->toHtml();

      queryStr = "UPDATE tbl_tree SET m_primary_content=:html WHERE "
                 "l_tree_id=:treeItemId";
      query.prepare(queryStr);
      query.bindValue(":treeItemId", treeItemId);
      query.bindValue(":html", html);
      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
      }
    }
    textDocument->deleteLater();

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { sqlDb.commit(); }
    IFOK(result) { dbVersion = newDbVersion; }
  }
  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.7 to 1.8
  if (dbVersion == 1.7) {
    double newDbVersion = 1.8;

    QString queryStr =
        R""""(PRAGMA foreign_keys = 0;

                CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                          FROM tbl_tree;

                DROP TABLE tbl_tree;

                CREATE TABLE tbl_tree (
                    l_tree_id           INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                 UNIQUE ON CONFLICT ROLLBACK
                                                 NOT NULL ON CONFLICT ROLLBACK,
                    t_title             TEXT,
                    t_internal_title    TEXT,
                    l_sort_order        INTEGER  NOT NULL ON CONFLICT ROLLBACK
                                                 DEFAULT (9999999999),
                    l_indent            INTEGER  NOT NULL ON CONFLICT ROLLBACK
                                                 DEFAULT (0),
                    t_type              TEXT,
                    m_primary_content   BLOB,
                    m_secondary_content BLOB,
                    dt_created          DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                 DEFAULT (CURRENT_TIMESTAMP),
                    dt_updated          DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                 DEFAULT (CURRENT_TIMESTAMP),
                    dt_trashed          DATETIME,
                    b_trashed           BOOLEAN  NOT NULL ON CONFLICT ROLLBACK
                                                 DEFAULT (0)
                );

                INSERT INTO tbl_tree (
                                         l_tree_id,
                                         t_title,
                                         l_sort_order,
                                         l_indent,
                                         t_type,
                                         m_primary_content,
                                         m_secondary_content,
                                         dt_created,
                                         dt_updated,
                                         dt_trashed,
                                         b_trashed
                                     )
                                     SELECT l_tree_id,
                                            t_title,
                                            l_sort_order,
                                            l_indent,
                                            t_type,
                                            m_primary_content,
                                            m_secondary_content,
                                            dt_created,
                                            dt_updated,
                                            dt_trashed,
                                            b_trashed
                                       FROM sqlitestudio_temp_table;

                DROP TABLE sqlitestudio_temp_table;

                PRAGMA foreign_keys = 1;
        )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));
    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }
  IFKO(result) { return result; }

  // ---------------------------------

  // from 1.8 to 1.9
  if (dbVersion == 1.8) {
    double newDbVersion = 1.9;

    QString queryStr =
        R""""(PRAGMA foreign_keys = 0;

                UPDATE
                    tbl_tree_property
                SET
                    t_name = REPLACE(t_name,'can_add_sibling_paper','can_add_sibling_tree_item');

                UPDATE
                    tbl_tree_property
                SET
                    t_name = REPLACE(t_name,'can_add_child_paper','can_add_child_tree_item');

                UPDATE
                    tbl_tree_property
                SET
                    t_value_type = 'BOOL'
                WHERE
                    t_name='can_add_sibling_tree_item' OR t_name='can_add_child_tree_item';

                UPDATE
                    tbl_tree_property
                SET
                    t_value_type = 'INT'
                WHERE
                    t_name='word_count_with_children'
                 OR t_name='word_count'
                 OR t_name='char_count'
                 OR t_name='char_count_with_children';

                PRAGMA foreign_keys = 1;
        )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));
    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }

  IFKO(result) { return result; }
  // ---------------------------------

  // from 1.9 to 2.0
  if (dbVersion == 1.9) {
    double newDbVersion = 2.0;

    QString queryStr =
        R""""(PRAGMA foreign_keys = 0;

                    CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                              FROM tbl_tag_relationship;

                    DROP TABLE tbl_tag_relationship;

                    CREATE TABLE tbl_tag_relationship (
                        l_tag_relationship_id INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                       NOT NULL ON CONFLICT ROLLBACK,
                        l_tree_code           INTEGER  REFERENCES tbl_tree (l_tree_id) ON DELETE CASCADE
                                                                                       ON UPDATE CASCADE,
                        l_tag_code            INTEGER  REFERENCES tbl_tag (l_tag_id) ON DELETE CASCADE
                                                                                     ON UPDATE CASCADE,
                        dt_created            DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                       DEFAULT (CURRENT_TIMESTAMP)
                    );

                    INSERT INTO tbl_tag_relationship (
                                                         l_tag_relationship_id,
                                                         l_tree_code,
                                                         l_tag_code,
                                                         dt_created
                                                     )
                                                     SELECT l_tag_relationship_id,
                                                            l_tree_code,
                                                            l_tag_code,
                                                            dt_created
                                                       FROM sqlitestudio_temp_table;

                    DROP TABLE sqlitestudio_temp_table;



                    CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                              FROM tbl_tag;

                    DROP TABLE tbl_tag;

                    CREATE TABLE tbl_tag (
                        l_tag_id     INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                              UNIQUE ON CONFLICT ROLLBACK
                                              NOT NULL ON CONFLICT ROLLBACK,
                        t_name       TEXT     UNIQUE ON CONFLICT ROLLBACK,
                        t_color      TEXT,
                        t_text_color TEXT,
                        dt_created   DATETIME NOT NULL ON CONFLICT ROLLBACK
                                              DEFAULT (CURRENT_TIMESTAMP),
                        dt_updated   DATETIME NOT NULL ON CONFLICT ROLLBACK
                                              DEFAULT (CURRENT_TIMESTAMP)
                    );

                    INSERT INTO tbl_tag (
                                            l_tag_id,
                                            t_name,
                                            t_color,
                                            t_text_color,
                                            dt_created,
                                            dt_updated
                                        )
                                        SELECT l_tag_id,
                                               t_name,
                                               t_color,
                                               t_text_color,
                                               dt_created,
                                               dt_updated
                                          FROM sqlitestudio_temp_table;

                    DROP TABLE sqlitestudio_temp_table;





                    CREATE TABLE sqlitestudio_temp_table AS SELECT *
                                                              FROM tbl_tree_property;


                    DROP TABLE tbl_tree_property;

                    CREATE TABLE tbl_tree_property (
                        l_tree_property_id INTEGER  PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT
                                                    UNIQUE ON CONFLICT ROLLBACK
                                                    NOT NULL ON CONFLICT ROLLBACK,
                        l_tree_code        INTEGER  REFERENCES tbl_tree (l_tree_id) ON DELETE CASCADE
                                                                                    ON UPDATE CASCADE,
                        t_name             TEXT,
                        t_value_type       TEXT     NOT NULL ON CONFLICT ROLLBACK
                                                    DEFAULT STRING,
                        m_value            BLOB,
                        dt_created         DATETIME NOT NULL
                                                    DEFAULT (CURRENT_TIMESTAMP),
                        dt_updated         DATETIME NOT NULL ON CONFLICT ROLLBACK
                                                    DEFAULT (CURRENT_TIMESTAMP),
                        b_system           BOOLEAN  DEFAULT (1)
                                                    NOT NULL ON CONFLICT ROLLBACK,
                        b_silent           BOOLEAN  DEFAULT (0)
                                                    NOT NULL ON CONFLICT ROLLBACK
                    );

                    INSERT INTO tbl_tree_property (
                                                      l_tree_property_id,
                                                      l_tree_code,
                                                      t_name,
                                                      t_value_type,
                                                      m_value,
                                                      dt_created,
                                                      dt_updated,
                                                      b_system,
                                                      b_silent
                                                  )
                                                  SELECT l_tree_property_id,
                                                         l_tree_code,
                                                         t_name,
                                                         t_value_type,
                                                         m_value,
                                                         dt_created,
                                                         dt_updated,
                                                         b_system,
                                                         b_silent
                                                    FROM sqlitestudio_temp_table;

                    DROP TABLE sqlitestudio_temp_table;



                    PRAGMA foreign_keys = 1;

        )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));



    // fetch all TEXT items to convert them to pseudo markdown



    queryStr = "SELECT l_tree_id FROM tbl_tree WHERE t_type = 'TEXT';";

    QSqlQuery query(sqlDb);

    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      return result;
    }

    QList<int> textIdList;

    while (query.next()) {
      textIdList.append(query.value(0).toInt());
    }

    MarkdownTextDocument *textDocument = new MarkdownTextDocument();

    for (int treeItemId : qAsConst(textIdList)) {
      // convert TEXT HTML content into markdown

      queryStr =
          "SELECT m_primary_content FROM tbl_tree WHERE l_tree_id=:treeItemId";
      query.prepare(queryStr);
      query.bindValue(":treeItemId", treeItemId);
      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
      }

      QString content;

      while (query.next()) {
        content = query.value(0).toString();
      }

      textDocument->setHtml(content);
      QString markdown = textDocument->toSkribistoMarkdown();

      queryStr = "UPDATE tbl_tree SET m_primary_content=:markdown WHERE "
                 "l_tree_id=:treeItemId";
      query.prepare(queryStr);
      query.bindValue(":treeItemId", treeItemId);
      query.bindValue(":markdown", markdown);
      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
      }
    }
    textDocument->deleteLater();

    {

      // fetch all secondary contents
      queryStr = "SELECT l_tree_id FROM tbl_tree;";

      QSqlQuery query(sqlDb);

      query.prepare(queryStr);
      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        return result;
      }

      QList<int> textIdList;

      while (query.next()) {
        textIdList.append(query.value(0).toInt());
      }

      MarkdownTextDocument *textDocument = new MarkdownTextDocument();

      for (int treeItemId : qAsConst(textIdList)) {
        // convert TEXT HTML content into markdown

        queryStr = "SELECT m_secondary_content FROM tbl_tree WHERE "
                   "l_tree_id=:treeItemId";
        query.prepare(queryStr);
        query.bindValue(":treeItemId", treeItemId);
        query.exec();

        if (query.lastError().isValid()) {
          result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                             "sql_error");
          result.addData("SQLError", query.lastError().text());
          result.addData("SQL string", queryStr);
          return result;
        }

        QString content;

        while (query.next()) {
          content = query.value(0).toString();
        }

        textDocument->setHtml(content);
        QString markdown = textDocument->toSkribistoMarkdown();

        queryStr = "UPDATE tbl_tree SET m_secondary_content=:markdown WHERE "
                   "l_tree_id=:treeItemId";
        query.prepare(queryStr);
        query.bindValue(":treeItemId", treeItemId);
        query.bindValue(":markdown", markdown);
        query.exec();

        if (query.lastError().isValid()) {
          result = SKRResult(SKRResult::Critical, "Upgrader::upgradeSQLite",
                             "sql_error");
          result.addData("SQLError", query.lastError().text());
          result.addData("SQL string", queryStr);
          return result;
        }
      }
      textDocument->deleteLater();
    }

    //create Trash folder

    queryStr =
            R""""(PRAGMA foreign_keys = 0;

                INSERT INTO tbl_tree (l_sort_order, l_indent, t_type, t_title, t_internal_title) VALUES (1, 1, 'FOLDER', "Trash", "trash_folder");

                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_renamable', 'BOOL', 'false');
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_movable', 'BOOL', 'false');
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'can_add_sibling_tree_item', 'BOOL', 'false');
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'can_add_child_tree_item', 'BOOL', 'false');
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_trashable', 'BOOL', 'false');
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_openable', 'BOOL', 'false');
                INSERT INTO tbl_tree_property (l_tree_code, t_name, t_value_type, m_value) VALUES (0, 'is_copyable', 'BOOL', 'false');



                    PRAGMA foreign_keys = 1;

        )"""";

    IFOKDO(result, SKRSqlTools::executeSQLString(queryStr, sqlDb));



    //move trashed items into Trash folder

    IFOKDO(result, Upgrader::moveTrashedItemsToTrashFolder_2_0(sqlDb));



    IFOKDO(result, SKRSqlTools::renumberTreeSortOrder(sqlDb));

    IFOKDO(result,
           result = Upgrader::setDbVersion(sqlDbConnectionName, newDbVersion));

    IFOK(result) { dbVersion = newDbVersion; }
  }
  IFKO(result) { return result; }

  // ---------------------------------

  // from 2.0 to 2.1
  if (dbVersion == 2.0) {
    double newDbVersion = 2.1;

    // fill here

    // IFOKDO(result, result = Upgrader::setDbVersion(sqlDb, newDbVersion));

    //        IFOK(result) {
    //            dbVersion = newDbVersion;
    //        }
  }
  IFKO(result) { return result; }

  return result;
}

// ---------------------------------------------------------------------------------------

SKRResult Upgrader::setDbVersion(const QString &sqlDbConnectionName,
                                 double newVersion) {
  SKRResult result("SKRSqlTools::setDbVersion");

  QSqlDatabase sqlDb = QSqlDatabase::database(sqlDbConnectionName);
  sqlDb.transaction();
  QSqlQuery query(sqlDb);
  QString queryStr =
      "UPDATE tbl_project SET dbl_database_version = :newVersion;";

  query.prepare(queryStr);
  query.bindValue(":newVersion", newVersion);
  query.exec();

  if (query.lastError().isValid()) {
    result =
        SKRResult(SKRResult::Critical, "Upgrader::setDbVersion", "sql_error");
    result.addData("SQLError", query.lastError().text());
    result.addData("SQL string", queryStr);
  }

  // update date
  IFOK(result) {
    queryStr = "UPDATE tbl_project SET dt_updated = CURRENT_TIMESTAMP;";
    query.prepare(queryStr);
    query.exec();

    if (query.lastError().isValid()) {
      result =
          SKRResult(SKRResult::Critical, "Upgrader::setDbVersion", "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
    }
  }

  sqlDb.commit();

  return result;
}

// ---------------------------------------------------------------------------------------

SKRResult Upgrader::movePaperToTree_1_5(QSqlDatabase sqlDb,
                                        const QString &tableName) {
  SKRResult result("movePaperToTree_1_5");
  QString tableId;
  QString propertyTable;
  QString propertyTableCode;
  QString treeRelationshipCode;

  if (tableName == "tbl_sheet") {
    tableId = "l_sheet_id";
    propertyTable = "tbl_sheet_property";
    propertyTableCode = "l_sheet_code";
    treeRelationshipCode = "l_tree_receiver_code";
  }

  if (tableName == "tbl_note") {
    tableId = "l_note_id";
    propertyTable = "tbl_note_property";
    propertyTableCode = "l_note_code";
    treeRelationshipCode = "l_tree_source_code";
  }

  // retrieve sorted orders list from tree table to find the sort order to
  // start from

  QList<int> treeSortOrderList;
  QSqlQuery query(sqlDb);
  QString queryStr = "SELECT l_sort_order FROM tbl_tree ORDER BY l_sort_order";

  query.prepare(queryStr);

  query.exec();

  if (query.lastError().isValid()) {
    result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                       "sql_error");
    result.addData("SQLError", query.lastError().text());
    result.addData("SQL string", queryStr);

    return result;
  }

  while (query.next()) {
    treeSortOrderList.append(query.value(0).toInt());
  }

  int startingSortOrder = 0;

  if (!treeSortOrderList.isEmpty()) {
    startingSortOrder = treeSortOrderList.last();
  }

  // retrieve sorted id list from paper table

  QList<int> originalIdList;

  queryStr =
      "SELECT " + tableId + " FROM " + tableName + " ORDER BY l_sort_order";

  query.prepare(queryStr);

  query.exec();

  if (query.lastError().isValid()) {
    result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                       "sql_error");
    result.addData("SQLError", query.lastError().text());
    result.addData("SQL string", queryStr);

    return result;
  }

  while (query.next()) {
    originalIdList.append(query.value(0).toInt());
  }

  // for each, retreive it from table

  sqlDb.transaction();

  for (int originalId : originalIdList) {
    queryStr =
        "INSERT INTO tbl_tree (t_title, l_sort_order, l_indent, t_type, "
        "m_primary_content, dt_created, dt_updated, dt_trashed, b_trashed)"
        "VALUES ("
        "                       (SELECT t_title FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId),"
        "                       (SELECT l_sort_order FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId) + :startingSortOrder,"
        "                       (SELECT l_indent FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId),"
        "                       'TEXT',"
        "                       (SELECT m_content FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId),"
        "                       (SELECT dt_created FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId),"
        "                       (SELECT dt_updated FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId),"
        "                       (SELECT dt_trashed FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId),"
        "                       (SELECT b_trashed FROM " +
        tableName + " WHERE " + tableId +
        " = :paperId)"
        ")";

    query.prepare(queryStr);

    query.bindValue(":paperId", originalId);
    query.bindValue(":startingSortOrder", startingSortOrder);

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();
      return result;
    }
    int newTreeId = query.lastInsertId().toInt();

    // retreive property id list from property table

    QList<int> originalPropertyIdList;
    QSqlQuery query(sqlDb);
    QString queryStr = "SELECT l_property_id FROM " + propertyTable +
                       " WHERE " + propertyTableCode + " = :paperId";

    query.prepare(queryStr);

    query.bindValue(":paperId", originalId);

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }

    while (query.next()) {
      originalPropertyIdList.append(query.value(0).toInt());
    }

    for (int originalPropertyId : originalPropertyIdList) {
      // add properties with new paper code

      queryStr = "INSERT INTO tbl_tree_property (l_tree_code , t_name, "
                 "t_value_type, m_value, dt_created, dt_updated, b_system)"
                 "VALUES ("
                 "                       :newTreeId,"
                 "                       (SELECT t_name FROM " +
                 propertyTable +
                 " WHERE l_property_id = :propertyId),"
                 "                       'STRING',"
                 "                       (SELECT t_value FROM " +
                 propertyTable +
                 " WHERE l_property_id = :propertyId),"
                 "                       (SELECT dt_created FROM " +
                 propertyTable +
                 " WHERE l_property_id = :propertyId),"
                 "                       (SELECT dt_updated FROM " +
                 propertyTable +
                 " WHERE l_property_id = :propertyId),"
                 "                       (SELECT b_system FROM " +
                 propertyTable +
                 " WHERE l_property_id = :propertyId)"
                 ")";

      query.prepare(queryStr);

      query.bindValue(":newTreeId", newTreeId);
      query.bindValue(":propertyId", originalPropertyId);

      query.exec();

      if (query.lastError().isValid()) {
        result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                           "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        sqlDb.rollback();

        return result;
      }
    }

    // update tag relationship table to fill tree code depending of paper id

    queryStr =
        "UPDATE tbl_tag_relationship SET l_tree_code = :newTreeId WHERE " +
        propertyTableCode + " = :paperId";

    query.prepare(queryStr);

    query.bindValue(":paperId", originalId);
    query.bindValue(":newTreeId", newTreeId);

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }

    // update sheet note relationship table to fill tree source/receiver
    // code depending of paper id

    queryStr = "UPDATE tbl_sheet_note SET " + treeRelationshipCode +
               " = :newTreeId WHERE " + propertyTableCode + " = :paperId";

    query.prepare(queryStr);

    query.bindValue(":paperId", originalId);
    query.bindValue(":newTreeId", newTreeId);

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical, "Upgrader::movePaperToTree_1_5",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }
  }

  sqlDb.commit();

  return result;
}

// ---------------------------------------------------------------------------------------

SKRResult Upgrader::transformParentsToFolder_1_5(QSqlDatabase sqlDb) {
  SKRResult result("SKRSqlTools::transformParentsToFolder_1_5");

  // retrieve sorted id with indent list

  QList<int> treeIdList;
  QList<int> treeIndentList;
  QList<int> treeSortOrderList;
  QSqlQuery query(sqlDb);
  QString queryStr = "SELECT l_tree_id, l_indent, l_sort_order FROM tbl_tree "
                     "ORDER BY l_sort_order";

  query.prepare(queryStr);

  query.exec();

  if (query.lastError().isValid()) {
    result = SKRResult(SKRResult::Critical,
                       "Upgrader::transformParentsToFolder_1_5", "sql_error");
    result.addData("SQLError", query.lastError().text());
    result.addData("SQL string", queryStr);

    return result;
  }

  while (query.next()) {
    treeIdList.append(query.value(0).toInt());
    treeIndentList.append(query.value(1).toInt());
    treeSortOrderList.append(query.value(2).toInt());
  }

  // create folders

  sqlDb.transaction();

  QString treeQueryStr = "INSERT INTO tbl_tree (t_title, l_indent, "
                         "l_sort_order, t_type, dt_trashed, b_trashed)"
                         "                       VALUES ("
                         "                       (SELECT t_title FROM tbl_tree "
                         "WHERE l_tree_id = :treeId),"
                         "                       (SELECT l_indent FROM "
                         "tbl_tree WHERE l_tree_id = :treeId),"
                         "                       :newSortOrder,"
                         "                       'FOLDER',"
                         "                       (SELECT dt_trashed FROM "
                         "tbl_tree WHERE l_tree_id = :treeId),"
                         "                       (SELECT b_trashed FROM "
                         "tbl_tree WHERE l_tree_id = :treeId)"
                         ")";

  QString incrementIndentQueryStr =
      "UPDATE tbl_tree SET l_indent = :newIndent WHERE l_tree_id = :treeId";

  QString isItSynopsisFolderQueryStr =
      "SELECT l_tree_property_id FROM tbl_tree_property WHERE t_name = "
      "'is_synopsis_folder' AND m_value = 'true' AND l_tree_code = :treeId";

  int previousIdIndent = -1;

  for (int i = treeIdList.count() - 1; i >= 0; i--) {
    int currentTreeId = treeIdList.at(i);
    int currentIndent = treeIndentList.at(i);

    if (i < treeIdList.count() - 1) {
      previousIdIndent = treeIndentList.at(i + 1);
    }

    if (previousIdIndent > currentIndent) {
      int currentSortOrder = treeSortOrderList.at(i);

      // create folder
      query.prepare(treeQueryStr);

      query.bindValue(":treeId", currentTreeId);
      query.bindValue(":newSortOrder", currentSortOrder - 1);

      query.exec();

      if (query.lastError().isValid()) {
        result =
            SKRResult(SKRResult::Critical,
                      "Upgrader::transformParentsToFolder_1_5", "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        result.addData("treeId", currentTreeId);
        result.addData("newSortOrder", currentSortOrder - 1);
        sqlDb.rollback();

        return result;
      }

      int newFolderTreeId = query.lastInsertId().toInt();

      // increment current item's indent

      query.prepare(incrementIndentQueryStr);

      query.bindValue(":treeId", currentTreeId);
      query.bindValue(":newIndent", currentIndent + 1);

      query.exec();

      if (query.lastError().isValid()) {
        result =
            SKRResult(SKRResult::Critical,
                      "Upgrader::transformParentsToFolder_1_5", "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        result.addData("treeId", currentTreeId);
        result.addData("sortOrder", currentSortOrder);
        result.addData("newIndent", currentIndent + 1);
        sqlDb.rollback();

        return result;
      }

      // check if it is synopsis folder

      query.prepare(isItSynopsisFolderQueryStr);

      query.bindValue(":treeId", currentTreeId);
      query.exec();

      if (query.lastError().isValid()) {
        result =
            SKRResult(SKRResult::Critical,
                      "Upgrader::transformParentsToFolder_1_5", "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);
        result.addData("treeId", currentTreeId);
        sqlDb.rollback();

        return result;
      }

      QList<int> synopsisFolderList;

      while (query.next()) {
        synopsisFolderList.append(query.value(0).toInt());
      }

      if (!synopsisFolderList.isEmpty()) {
        SKRSqlTools::addStringTreeProperty(sqlDb, newFolderTreeId,
                                           "is_synopsis_folder", "true");
      }
    }
  }
  IFOK(result) { sqlDb.commit(); }

  // remnumber tree

  return result;
}

// ---------------------------------------------------------------------------------------

SKRResult Upgrader::dropDeprecatedTables_1_5(QSqlDatabase sqlDb) {
  SKRResult result("Upgrader::dropDeprecatedTables_1_5");

  QSqlQuery query(sqlDb);

  // drop deprecated tables

  QString queryStr =
      R""""(PRAGMA foreign_keys = 0;

            DROP TRIGGER trg_delete_properties;
            DROP VIEW v_property_sheet;
            DROP VIEW v_tree_sheet;
            DROP INDEX idx_note;
            DROP TABLE tbl_note;
            DROP TABLE tbl_note_property;
            DROP INDEX idx_sheet;
            DROP TABLE tbl_sheet;
            DROP TABLE tbl_sheet_property;

            PRAGMA foreign_keys = 1;

            )"""";

  result = SKRSqlTools::executeSQLString(queryStr, sqlDb);

  return result;
}

// ---------------------------------------------------------------------------------------

SKRResult Upgrader::moveSynopsisToSecondaryContent_1_6(QSqlDatabase sqlDb) {
  SKRResult result("Upgrader::moveSynopsisToSecondaryContent_1_6");

  // retrieve synopsis lists

  QList<int> synopsisRelationshipIdList;
  QList<int> sourceIdList;
  QList<int> receiverIdList;
  QSqlQuery query(sqlDb);
  QString queryStr =
      "SELECT l_tree_relationship_id, l_tree_source_code, l_tree_receiver_code "
      "FROM tbl_tree_relationship WHERE b_synopsis = 1";

  query.prepare(queryStr);

  query.exec();

  if (query.lastError().isValid()) {
    result =
        SKRResult(SKRResult::Critical,
                  "Upgrader::moveSynopsisToSecondaryContent_1_6", "sql_error");
    result.addData("SQLError", query.lastError().text());
    result.addData("SQL string", queryStr);

    return result;
  }

  while (query.next()) {
    synopsisRelationshipIdList.append(query.value(0).toInt());
    sourceIdList.append(query.value(1).toInt());
    receiverIdList.append(query.value(2).toInt());
  }

  QList<int> synopsisTreeIdList = sourceIdList;
  QList<int> textIdList = receiverIdList;

  // for each synopsis, move to corresponding secondary content

  sqlDb.transaction();

  for (int i = 0; i < synopsisRelationshipIdList.count(); i++) {
    queryStr = "UPDATE tbl_tree SET m_secondary_content = (SELECT "
               "m_primary_content FROM tbl_tree WHERE l_tree_id = "
               ":synopsisTreeId) WHERE l_tree_id = :textId";

    query.prepare(queryStr);
    query.bindValue(":textId", textIdList.at(i));
    query.bindValue(":synopsisTreeId", synopsisTreeIdList.at(i));

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical,
                         "Upgrader::moveSynopsisToSecondaryContent_1_6",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }

    // remove relationship
    int synopsisRelationshipId = synopsisRelationshipIdList.at(i);

    queryStr = "DELETE FROM tbl_tree_relationship WHERE l_tree_relationship_id "
               "= :synopsisRelationshipId";
    query.prepare(queryStr);
    query.bindValue(":synopsisRelationshipId", synopsisRelationshipId);

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical,
                         "Upgrader::moveSynopsisToSecondaryContent_1_6",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }

    // remove synopsis tree row

    queryStr = "DELETE FROM tbl_tree WHERE l_tree_id = :synopsisTreeId";
    query.prepare(queryStr);
    query.bindValue(":synopsisTreeId", synopsisTreeIdList.at(i));

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical,
                         "Upgrader::moveSynopsisToSecondaryContent_1_6",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }
  }

  // delete Outlines folder

  QList<int> synopsisFolderIdList;

  queryStr = "SELECT l_tree_code FROM tbl_tree_property WHERE t_name = "
             "'is_synopsis_folder' AND m_value = 'true'";

  query.prepare(queryStr);

  query.exec();

  if (query.lastError().isValid()) {
    result =
        SKRResult(SKRResult::Critical,
                  "Upgrader::moveSynopsisToSecondaryContent_1_6", "sql_error");
    result.addData("SQLError", query.lastError().text());
    result.addData("SQL string", queryStr);
    sqlDb.rollback();

    return result;
  }

  while (query.next()) {
    synopsisFolderIdList.append(query.value(0).toInt());
  }

  for (int synopsisFolderId : synopsisFolderIdList) {
    queryStr = "DELETE FROM tbl_tree WHERE l_tree_id = :synopsisFolderId";
    query.prepare(queryStr);

    query.bindValue(":synopsisFolderId", synopsisFolderId);

    query.exec();

    if (query.lastError().isValid()) {
      result = SKRResult(SKRResult::Critical,
                         "Upgrader::moveSynopsisToSecondaryContent_1_6",
                         "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);
      sqlDb.rollback();

      return result;
    }
  }

  sqlDb.commit();

  return result;
}

//----------------------------------------------------------

SKRResult Upgrader::moveTrashedItemsToTrashFolder_2_0(QSqlDatabase sqlDb)
{
    SKRResult result("Upgrader::moveTrashedItemsToTrashFolder_2_0");

    QSqlQuery query(sqlDb);
    QString queryStr =
        "SELECT l_tree_id, l_sort_order, l_indent "
        "FROM tbl_tree WHERE b_trashed = 1 ORDER BY l_sort_order";

    query.prepare(queryStr);

    query.exec();

    if (query.lastError().isValid()) {
      result =
          SKRResult(SKRResult::Critical,
                    "Upgrader::moveTrashedItemsToTrashFolder_2_0", "sql_error");
      result.addData("SQLError", query.lastError().text());
      result.addData("SQL string", queryStr);

      return result;
    }

    QList<int> sourceIdList;
    QList<int> sortOrderList;
    QList<int> indentList;

    while (query.next()) {
      sourceIdList.append(query.value(0).toInt());
      sortOrderList.append(query.value(1).toInt());
      indentList.append(query.value(2).toInt());
    }

    // group trashed items

    QList< QList<int> > groupedIds;
    QList< QList<int> > groupedSortOrders;
    QList< QList<int> > groupedIndents;


    while(!sourceIdList.isEmpty()){

        QList<int> idGroup;
        QList<int> sortOrderGroup;
        QList<int> indentGroup;            


        while (!sourceIdList.isEmpty()) {



        idGroup << sourceIdList.takeFirst();
        sortOrderGroup << sortOrderList.takeFirst();
        indentGroup << indentList.takeFirst();


        if(sourceIdList.isEmpty()){

            groupedIds.append(idGroup);
            groupedSortOrders.append(sortOrderGroup);
            groupedIndents.append(indentGroup);

        }
        else{
            int thisSortOrder = sortOrderGroup.last();
            int nextSortOrder = sortOrderList.at(0);
            int sortOrderDelta = nextSortOrder - thisSortOrder;

            int thisIndent = indentGroup.last();
            int nextIndent = indentList.at(0);
            int indentDelta = nextIndent - thisIndent;


            if(sortOrderDelta > 1000 || indentDelta < 0){

                groupedIds.append(idGroup);
                groupedSortOrders.append(sortOrderGroup);
                groupedIndents.append(indentGroup);

                break;
            }
        }
        }
    }

    // get trash_folder sort_order
    queryStr =
            "SELECT l_sort_order "
            "FROM tbl_tree WHERE t_internal_title = :internal_title";


    query.prepare(queryStr);
    query.bindValue(":internal_title", "trash_folder");
    query.exec();

    if (query.lastError().isValid() || query.size() == 0) {
        result =
                SKRResult(SKRResult::Critical,
                          "Upgrader::moveTrashedItemsToTrashFolder_2_0", "sql_error");
        result.addData("SQLError", query.lastError().text());
        result.addData("SQL string", queryStr);

        return result;
    }

    sortOrderList.clear();
    while (query.next()) {
        sortOrderList.append(query.value(0).toInt());
    }


    int newSortOrder = sortOrderList.first() + 1;


    // apply update

    while(!groupedIds.isEmpty()){

        QList<int> idGroup = groupedIds.takeFirst();
        QList<int> indentGroup = groupedIndents.takeFirst();

        int groupIndentDelta = indentGroup.first() - 2; // against 1, the indent of trash_folder

        while(!idGroup.isEmpty()){

            int treeId = idGroup.takeFirst();
            int indent = indentGroup.takeFirst();

            queryStr = "UPDATE tbl_tree SET l_sort_order = :sort_order, l_indent = :indent WHERE l_tree_id = :treeId";

            query.prepare(queryStr);
            query.bindValue(":treeId", treeId);
            query.bindValue(":sort_order", newSortOrder);
            query.bindValue(":indent", indent - groupIndentDelta);

            query.exec();


            if (query.lastError().isValid()) {
                result = SKRResult(SKRResult::Critical,
                                   "Upgrader::moveTrashedItemsToTrashFolder_2_0",
                                   "sql_error");
                result.addData("SQLError", query.lastError().text());
                result.addData("SQL string", queryStr);
                sqlDb.rollback();

                return result;
            }


            newSortOrder += 1;
        }

    }

    sqlDb.commit();
    return result;
}
