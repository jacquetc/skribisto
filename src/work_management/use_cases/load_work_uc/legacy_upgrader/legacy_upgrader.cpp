/*
 * Copyright (C) 2025 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */
#include "legacy_upgrader.h"

#include <QDateTime>
#include <QSqlError>
#include <QSqlQuery>

using namespace Qt::StringLiterals;

namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
{

bool LegacyUpgrader::upgradeSQLite(const QString &filePath)
{
    // Register a Qt SQL connection for the legacy upgrader tools
    const QString connName = u"legacy_upgrade_connection"_s;
    {
        QSqlDatabase db = QSqlDatabase::addDatabase(u"QSQLITE"_s, connName);
        db.setDatabaseName(filePath);
        if (!db.open())
        {
            qCritical() << "Cannot open database for upgrade:" << filePath;
            QSqlDatabase::removeDatabase(connName);
            return false;
        }
    }

    // Step 1: Run incremental schema upgrades (v1.0 → v2.0)
    // Upgrader and SKRSqlTools expect a Qt connection name
    SKRResult result = Upgrader::upgradeSQLite(connName);
    if (!result)
    {
        qCritical() << "Legacy schema upgrade failed:" << result.getLastErrorCode();
        QSqlDatabase::removeDatabase(connName);
        return false;
    }

    // Step 2: Migrate from old tbl_tree schema to v3 Qleany tables
    if (!migrateToV3(connName))
    {
        qCritical() << "Migration from tbl_tree to v3 Qleany format failed";
        QSqlDatabase::removeDatabase(connName);
        return false;
    }

    QSqlDatabase::removeDatabase(connName);
    return true;
}

bool LegacyUpgrader::isUpgradeNeeded(const QString &filePath)
{
    const QString connName = u"legacy_check_connection"_s;
    {
        QSqlDatabase db = QSqlDatabase::addDatabase(u"QSQLITE"_s, connName);
        db.setDatabaseName(filePath);
        if (!db.open())
        {
            QSqlDatabase::removeDatabase(connName);
            return false;
        }
    }

    bool needed = false;
    {
        QSqlDatabase db = QSqlDatabase::database(connName);

        // If tbl_tree exists, this is an old format that needs migration
        QSqlQuery q(db);
        q.exec(u"SELECT name FROM sqlite_master WHERE type='table' AND name='tbl_tree'"_s);
        if (q.next())
        {
            needed = true;
        }
        else
        {
            // Also check tbl_project for pre-2.0 schema versions
            SKRResult result(u"isUpgradeNeeded"_s);
            double dbVersion = SKRSqlTools::getProjectDBVersion(&result, connName);
            if (result.isSuccess() && dbVersion < 2.0)
                needed = true;
        }
    }

    QSqlDatabase::removeDatabase(connName);
    return needed;
}

bool LegacyUpgrader::migrateToV3(const QString &sqlDbConnectionName)
{
    QSqlDatabase db = QSqlDatabase::database(sqlDbConnectionName);
    if (!db.isOpen() && !db.open())
        return false;

    // Verify tbl_tree exists (old format)
    {
        QSqlQuery q(db);
        q.exec(u"SELECT name FROM sqlite_master WHERE type='table' AND name='tbl_tree'"_s);
        if (!q.next())
            return true; // Nothing to migrate — already v3 or empty
    }

    db.transaction();

    const auto now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    // ── Create v3 tables ────────────────────────────────────────

    QSqlQuery q(db);

    // Work
    q.exec(u"CREATE TABLE IF NOT EXISTS work ("
           "id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,"
           "created_at TEXT NOT NULL, updated_at TEXT NOT NULL,"
           "title TEXT NOT NULL DEFAULT '', author_name TEXT NOT NULL DEFAULT '',"
           "dict_language TEXT NOT NULL DEFAULT '',"
           "binders TEXT NOT NULL DEFAULT '[]', tags TEXT NOT NULL DEFAULT '[]',"
           "dict_words TEXT NOT NULL DEFAULT '[]');"_s);

    // Binder
    q.exec(u"CREATE TABLE IF NOT EXISTS binder ("
           "id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,"
           "created_at TEXT NOT NULL, updated_at TEXT NOT NULL,"
           "name TEXT NOT NULL DEFAULT '', activated INTEGER NOT NULL DEFAULT 1,"
           "binder_items TEXT NOT NULL DEFAULT '[]');"_s);

    // BinderItem
    q.exec(u"CREATE TABLE IF NOT EXISTS binder_item ("
           "id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,"
           "created_at TEXT NOT NULL, updated_at TEXT NOT NULL,"
           "title TEXT NOT NULL DEFAULT '', sub_title TEXT NOT NULL DEFAULT '',"
           "role TEXT NOT NULL DEFAULT '', sub_role TEXT NOT NULL DEFAULT '',"
           "label TEXT NOT NULL DEFAULT '', activated INTEGER NOT NULL DEFAULT 1,"
           "is_favorite INTEGER NOT NULL DEFAULT 0, is_printable INTEGER NOT NULL DEFAULT 1,"
           "indent INTEGER NOT NULL DEFAULT 0, word_count_goal INTEGER NOT NULL DEFAULT 0,"
           "char_count_goal INTEGER NOT NULL DEFAULT 0, dict_language TEXT NOT NULL DEFAULT '',"
           "contents TEXT NOT NULL DEFAULT '[]', _references TEXT NOT NULL DEFAULT '[]',"
           "tags TEXT NOT NULL DEFAULT '[]');"_s);

    // BinderTag
    q.exec(u"CREATE TABLE IF NOT EXISTS binder_tag ("
           "id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,"
           "created_at TEXT NOT NULL, updated_at TEXT NOT NULL,"
           "name TEXT NOT NULL DEFAULT '', color TEXT NOT NULL DEFAULT '',"
           "text_color TEXT NOT NULL DEFAULT '');"_s);

    // Content
    q.exec(u"CREATE TABLE IF NOT EXISTS content ("
           "id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,"
           "created_at TEXT NOT NULL, updated_at TEXT NOT NULL,"
           "activated INTEGER NOT NULL DEFAULT 1, role TEXT NOT NULL DEFAULT '',"
           "data TEXT NOT NULL DEFAULT '');"_s);

    // DictWord
    q.exec(u"CREATE TABLE IF NOT EXISTS dict_word ("
           "id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,"
           "created_at TEXT NOT NULL, updated_at TEXT NOT NULL,"
           "word TEXT NOT NULL DEFAULT '');"_s);

    // Junction tables
    q.exec(u"CREATE TABLE IF NOT EXISTS work_binders_to_binder_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL, order_ INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);
    q.exec(u"CREATE TABLE IF NOT EXISTS work_tags_to_binder_tag_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);
    q.exec(u"CREATE TABLE IF NOT EXISTS work_dict_words_to_dict_word_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);
    q.exec(u"CREATE TABLE IF NOT EXISTS binder_binder_items_to_binder_item_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL, order_ INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);
    q.exec(u"CREATE TABLE IF NOT EXISTS binder_item_contents_to_content_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);
    q.exec(u"CREATE TABLE IF NOT EXISTS binder_item_references_to_binder_item_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);
    q.exec(u"CREATE TABLE IF NOT EXISTS binder_item_tags_to_binder_tag_junction ("
           "left_id INTEGER NOT NULL, right_id INTEGER NOT NULL,"
           "PRIMARY KEY (left_id, right_id));"_s);

    // ── Read project metadata ───────────────────────────────────

    QString projectTitle;
    QString authorName;
    QString dictLanguage;
    {
        QSqlQuery pq(db);
        pq.exec(u"SELECT t_project_name, t_author, t_spell_check_lang FROM tbl_project LIMIT 1"_s);
        if (pq.next())
        {
            projectTitle = pq.value(0).isNull() ? u""_s : pq.value(0).toString();
            authorName = pq.value(1).isNull() ? u""_s : pq.value(1).toString();
            dictLanguage = pq.value(2).isNull() ? u""_s : pq.value(2).toString();
        }
    }

    // ── Create Work ─────────────────────────────────────────────

    q.finish();
    q.prepare(u"INSERT INTO work (created_at, updated_at, title, author_name, dict_language) "
              "VALUES (:ca, :ua, :t, :an, :dl)"_s);
    q.bindValue(u":ca"_s, now);
    q.bindValue(u":ua"_s, now);
    q.bindValue(u":t"_s, projectTitle);
    q.bindValue(u":an"_s, authorName);
    q.bindValue(u":dl"_s, dictLanguage);
    q.exec();
    const int workId = q.lastInsertId().toInt();

    // ── Migrate tags ────────────────────────────────────────────

    q.finish();
    q.exec(u"INSERT INTO binder_tag (created_at, updated_at, name, color, text_color) "
           "SELECT dt_created, dt_updated, t_name, COALESCE(t_color, ''), COALESCE(t_text_color, '') "
           "FROM tbl_tag"_s);

    // Work → Tags junction (all tags belong to this work)
    q.finish();
    q.exec(u"INSERT INTO work_tags_to_binder_tag_junction (left_id, right_id) "
           "SELECT %1, id FROM binder_tag"_s.arg(workId));

    // Build old→new tag ID map (old IDs may differ from new auto-incremented IDs)
    QHash<int, int> tagIdMap;
    {
        QSqlQuery oldQ(db);
        oldQ.exec(u"SELECT l_tag_id FROM tbl_tag ORDER BY l_tag_id"_s);
        QSqlQuery newQ(db);
        newQ.exec(u"SELECT id FROM binder_tag ORDER BY id"_s);
        while (oldQ.next() && newQ.next())
            tagIdMap.insert(oldQ.value(0).toInt(), newQ.value(0).toInt());
    }

    // ── Migrate dictionary ──────────────────────────────────────

    q.finish();
    q.exec(u"INSERT INTO dict_word (created_at, updated_at, word) "
           "SELECT '%1', '%1', t_word FROM tbl_project_dict"_s.arg(now));

    q.finish();
    q.exec(u"INSERT INTO work_dict_words_to_dict_word_junction (left_id, right_id) "
           "SELECT %1, id FROM dict_word"_s.arg(workId));

    // ── Load properties ─────────────────────────────────────────

    QHash<int, QString> sectionTypes;
    QHash<int, QString> itemLabels;
    {
        QSqlQuery pq(db);
        pq.exec(u"SELECT l_tree_code, t_name, m_value FROM tbl_tree_property "
                "WHERE t_name IN ('section_type', 'label')"_s);
        while (pq.next())
        {
            int treeId = pq.value(0).toInt();
            QString propName = pq.value(1).toString();
            QString propValue = pq.value(2).toString();
            if (propName == u"section_type"_s)
                sectionTypes.insert(treeId, propValue);
            else if (propName == u"label"_s)
                itemLabels.insert(treeId, propValue);
        }
    }

    // ── Migrate tree items ──────────────────────────────────────
    // Strategy: indent=1 FOLDERs become Binders. Everything else becomes BinderItems.

    struct TreeRow
    {
        int id;
        QString title;
        QString internalTitle;
        int indent;
        QString type;
        QByteArray primaryContent;
        QByteArray secondaryContent;
        QString createdAt;
        QString updatedAt;
        bool trashed;
    };

    QList<TreeRow> allRows;
    {
        QSqlQuery tq(db);
        tq.exec(u"SELECT l_tree_id, t_title, l_indent, t_type, "
                "m_primary_content, m_secondary_content, dt_created, dt_updated, b_trashed, "
                "t_internal_title "
                "FROM tbl_tree WHERE l_indent > 0 ORDER BY l_sort_order"_s);
        while (tq.next())
        {
            TreeRow row;
            row.id = tq.value(0).toInt();
            row.title = tq.value(1).isNull() ? u""_s : tq.value(1).toString();
            row.indent = tq.value(2).toInt();
            row.type = tq.value(3).isNull() ? u""_s : tq.value(3).toString();
            row.primaryContent = tq.value(4).toByteArray();
            row.secondaryContent = tq.value(5).toByteArray();
            row.createdAt = tq.value(6).toString();
            row.updatedAt = tq.value(7).toString();
            row.trashed = tq.value(8).toBool();
            row.internalTitle = tq.value(9).isNull() ? u""_s : tq.value(9).toString();
            allRows.append(row);
        }
    }

    // Group rows into binders
    struct BinderGroup
    {
        TreeRow binderRow;
        bool isImplicit = false;
        bool isNoteBinder = false;
        QList<TreeRow> items;
    };

    QList<BinderGroup> binderGroups;
    BinderGroup *currentGroup = nullptr;

    // Stray top-level non-folders (indent=1, not FOLDER) collected here,
    // then appended to the first binder after the main loop.
    QList<TreeRow> strayTopLevel;

    for (const auto &row : allRows)
    {
        if (row.indent == 1 && row.type == u"FOLDER"_s)
        {
            bool noteBinder = (row.internalTitle == u"note_folder"_s);
            binderGroups.append({row, false, noteBinder, {}});
            currentGroup = &binderGroups.last();
        }
        else if (row.indent == 1)
        {
            // Top-level non-folder — collect for later insertion into first binder
            strayTopLevel.append(row);
        }
        else if (currentGroup)
        {
            currentGroup->items.append(row);
        }
    }

    // Append stray top-level items at the end of the first binder with indent adjusted to 2
    if (!strayTopLevel.isEmpty())
    {
        if (binderGroups.isEmpty())
        {
            // No binders at all — create an implicit one
            TreeRow implicitRow;
            implicitRow.id = -1;
            implicitRow.title = u"Writings"_s;
            implicitRow.indent = 1;
            implicitRow.type = u"FOLDER"_s;
            implicitRow.createdAt = now;
            implicitRow.updatedAt = now;
            implicitRow.trashed = false;
            binderGroups.append({implicitRow, true, false, {}});
        }

        for (auto &stray : strayTopLevel)
        {
            stray.indent = 2; // force to child level within the binder
            binderGroups.first().items.append(stray);
        }
    }

    // Create binders and items
    QHash<int /*old tree id*/, int /*new binder_item id*/> treeToItemMap;
    int binderOrder = 0;

    for (const auto &group : binderGroups)
    {
        // Create Binder
        q.finish();
        q.prepare(u"INSERT INTO binder (created_at, updated_at, name, activated) "
                  "VALUES (:ca, :ua, :n, :a)"_s);
        q.bindValue(u":ca"_s, group.binderRow.createdAt);
        q.bindValue(u":ua"_s, group.binderRow.updatedAt);
        q.bindValue(u":n"_s, group.binderRow.title);
        q.bindValue(u":a"_s, group.binderRow.trashed ? 0 : 1);
        q.exec();
        const int binderId = q.lastInsertId().toInt();

        // Work → Binder junction
        q.finish();
        q.prepare(u"INSERT INTO work_binders_to_binder_junction (left_id, right_id, order_) "
                  "VALUES (:w, :b, :o)"_s);
        q.bindValue(u":w"_s, workId);
        q.bindValue(u":b"_s, binderId);
        q.bindValue(u":o"_s, binderOrder++);
        q.exec();

        // Create BinderItems
        int itemOrder = 0;
        for (const auto &itemRow : group.items)
        {
            // ── Map v1 section_type to v2 sub_role ──────────────
            const QString v1SectionType = sectionTypes.value(itemRow.id, u""_s);

            // Drop separator items — scene breaks are now content markup
            if (itemRow.type == u"SECTION"_s && v1SectionType == u"separator"_s)
                continue;

            // ── Map v1 type to v2 role ──────────────────────────
            QString role;
            if (itemRow.type == u"FOLDER"_s)
                role = u"folder"_s;
            else
                role = u"item"_s; // TEXT and SECTION both become "item"

            // ── Map v1 section_type to v2 sub_role ──────────────
            QString subRole;
            if (itemRow.type == u"SECTION"_s)
            {
                if (v1SectionType == u"book-beginning"_s)
                    subRole = u"book-begin"_s;
                else if (v1SectionType == u"chapter"_s)
                    subRole = u"chapter"_s;
                else if (v1SectionType == u"book-end"_s)
                    subRole = u"book-end"_s;
                // Other/unknown section types: leave sub_role empty
            }
            else if (itemRow.type == u"TEXT"_s)
            {
                subRole = group.isNoteBinder ? u"note"_s : u"scene"_s;
            }
            // FOLDERs get no sub_role by default

            q.finish();
            q.prepare(u"INSERT INTO binder_item (created_at, updated_at, title, role, sub_role, "
                      "label, activated, is_printable, indent, dict_language) "
                      "VALUES (:ca, :ua, :t, :r, :sr, :l, :a, 1, :i, '')"_s);
            q.bindValue(u":ca"_s, itemRow.createdAt);
            q.bindValue(u":ua"_s, itemRow.updatedAt);
            q.bindValue(u":t"_s, itemRow.title);
            q.bindValue(u":r"_s, role);
            q.bindValue(u":sr"_s, subRole);
            q.bindValue(u":l"_s, itemLabels.value(itemRow.id, u""_s));
            q.bindValue(u":a"_s, itemRow.trashed ? 0 : 1);
            q.bindValue(u":i"_s, itemRow.indent - 2); // binder is indent 1, items start at 0
            q.exec();
            const int itemId = q.lastInsertId().toInt();
            treeToItemMap.insert(itemRow.id, itemId);

            // Binder → BinderItem junction
            q.finish();
            q.prepare(u"INSERT INTO binder_binder_items_to_binder_item_junction (left_id, right_id, order_) "
                      "VALUES (:b, :i, :o)"_s);
            q.bindValue(u":b"_s, binderId);
            q.bindValue(u":i"_s, itemId);
            q.bindValue(u":o"_s, itemOrder++);
            q.exec();

            // ── Helper lambda: insert a Content and link it to this item ──
            auto insertContent = [&](const QString &contentRole, const QString &data) {
                q.finish();
                q.prepare(u"INSERT INTO content (created_at, updated_at, activated, role, data) "
                          "VALUES (:ca, :ua, :a, :r, :d)"_s);
                q.bindValue(u":ca"_s, itemRow.createdAt);
                q.bindValue(u":ua"_s, itemRow.updatedAt);
                q.bindValue(u":a"_s, itemRow.trashed ? 0 : 1);
                q.bindValue(u":r"_s, contentRole);
                q.bindValue(u":d"_s, data);
                q.exec();
                int contentId = q.lastInsertId().toInt();

                q.finish();
                q.prepare(u"INSERT INTO binder_item_contents_to_content_junction (left_id, right_id) "
                          "VALUES (:i, :c)"_s);
                q.bindValue(u":i"_s, itemId);
                q.bindValue(u":c"_s, contentId);
                q.exec();
            };

            // ── Create heading Content for structural markers ───
            if (subRole == u"book-begin"_s && !itemRow.title.isEmpty())
            {
                insertContent(u"book-title"_s, itemRow.title);
            }
            else if (subRole == u"chapter"_s && !itemRow.title.isEmpty())
            {
                insertContent(u"chapter-title"_s, itemRow.title);
            }

            // ── Create Content for primary content ──────────────
            if (!itemRow.primaryContent.isEmpty())
            {
                const QString primaryRole = group.isNoteBinder ? u"note-text"_s : u"scene-text"_s;
                insertContent(primaryRole, QString::fromUtf8(itemRow.primaryContent));
            }

            // ── Create Content for secondary content (was the "notes" sidebar in v1) ──
            if (!itemRow.secondaryContent.isEmpty())
            {
                insertContent(u"synopsis-text"_s, QString::fromUtf8(itemRow.secondaryContent));
            }

            // Tag junctions
            {
                QSqlQuery tq(db);
                tq.prepare(u"SELECT l_tag_code FROM tbl_tag_relationship WHERE l_tree_code = :tc"_s);
                tq.bindValue(u":tc"_s, itemRow.id);
                tq.exec();
                while (tq.next())
                {
                    int newTagId = tagIdMap.value(tq.value(0).toInt(), -1);
                    if (newTagId >= 0)
                    {
                        QSqlQuery jq(db);
                        jq.prepare(u"INSERT OR IGNORE INTO binder_item_tags_to_binder_tag_junction "
                                   "(left_id, right_id) VALUES (:i, :t)"_s);
                        jq.bindValue(u":i"_s, itemId);
                        jq.bindValue(u":t"_s, newTagId);
                        jq.exec();
                    }
                }
            }
        }
    }

    // ── Migrate tree relationships (Notes → TEXT cross-refs) ────

    {
        QSqlQuery rq(db);
        rq.exec(u"SELECT l_tree_source_code, l_tree_receiver_code FROM tbl_tree_relationship"_s);
        while (rq.next())
        {
            int sourceItemId = treeToItemMap.value(rq.value(0).toInt(), -1);
            int receiverItemId = treeToItemMap.value(rq.value(1).toInt(), -1);
            if (sourceItemId >= 0 && receiverItemId >= 0)
            {
                QSqlQuery jq(db);
                jq.prepare(u"INSERT OR IGNORE INTO binder_item_references_to_binder_item_junction "
                           "(left_id, right_id) VALUES (:s, :r)"_s);
                jq.bindValue(u":s"_s, sourceItemId);
                jq.bindValue(u":r"_s, receiverItemId);
                jq.exec();
            }
        }
    }

    // ── Drop old tables ─────────────────────────────────────────

    q.finish();
    q.exec(u"DROP TABLE IF EXISTS tbl_tree_relationship"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_tag_relationship"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_tree_property"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_tree"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_tag"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_project_dict"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_stat_history"_s);
    q.exec(u"DROP TABLE IF EXISTS tbl_project"_s);

    if (!db.commit())
    {
        qCritical() << "Failed to commit v3 migration:" << db.lastError();
        db.rollback();
        return false;
    }

    // VACUUM must run outside a transaction
    {
        QSqlQuery vq(db);
        vq.exec(u"VACUUM"_s);
    }

    return true;
}

} // namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
