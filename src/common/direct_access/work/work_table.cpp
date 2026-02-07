/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#include "work_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/work.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDWork = Skribisto::Common::DirectAccess::Work;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// forward relationship junction tables
const QString WORK_BINDERS_JUNCTION = "work_binders_to_binder_junction"_L1;
const QString WORK_TAGS_JUNCTION = "work_tags_to_binder_tag_junction"_L1;
//  backward relationship junction tables
const QString ROOT_WORKS_JUNCTION = "root_works_to_work_junction"_L1;

SCDWork::WorkTable::WorkTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::Work> SCDWork::WorkTable::createMany(const QList<SCE::Work> &works)
{
    QList<SCE::Work> created;
    created.reserve(works.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);
    for (SCE::Work r : works)
    {
        QStringList columnNames;
        QStringList valuePlaceholders;

        // Conditionally include id only if > 0
        if (r.id > 0)
        {
            columnNames << "id"_L1;
            valuePlaceholders << ":id"_L1;
        }

        columnNames << "created_at"_L1
                    << "updated_at"_L1
                    << "title"_L1
                    << "dict_language"_L1;

        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":title"_L1 << ":dict_language"_L1;
        QString sqlString =
            "INSERT INTO work (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

        q.prepare(sqlString);

        // Set timestamps if not provided
        if (r.createdAt.isNull())
            r.createdAt = QDateTime::currentDateTimeUtc();
        if (r.updatedAt.isNull())
            r.updatedAt = r.createdAt;

        if (r.id > 0)
            q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":title"_L1, r.title);
        q.bindValue(":dict_language"_L1, r.dictLanguage);
        if (!q.exec())
        {
            qCritical() << "Failed to insert Work:" << q.lastError().text() << " SQL:" << sqlString;
            // If insert fails, skip this row
            continue;
        }
        // Retrieve the auto-generated id
        QSqlQuery idq(db);
        if (idq.exec("SELECT last_insert_rowid()"_L1) && idq.next())
        {
            r.id = idq.value(0).toInt();

            // Handle junction table relationships
            if (!r.binders.isEmpty())
            {
                JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, WORK_BINDERS_JUNCTION, r.binders);
            }
            if (!r.tags.isEmpty())
            {
                JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, WORK_TAGS_JUNCTION, r.tags);
            }

            created.append(r);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &work : created)
            createdIds.append(work.id);

        using WorkCache = Database::TableCache<SCE::Work, WorkRelationshipField>;
        WorkCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::Work> SCDWork::WorkTable::updateMany(const QList<SCE::Work> &works)
{
    QList<SCE::Work> updated;
    updated.reserve(works.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "title = :title"_L1
                << "dict_language = :dict_language"_L1;

    QString sqlString = "UPDATE work SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::Work &r : works)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":title"_L1, r.title);
        q.bindValue(":dict_language"_L1, r.dictLanguage);

        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, WORK_BINDERS_JUNCTION, r.binders);
            JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, WORK_TAGS_JUNCTION, r.tags);

            updated.append(r);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &work : updated)
            updatedIds.append(work.id);

        using WorkCache = Database::TableCache<SCE::Work, WorkRelationshipField>;
        WorkCache::instance().invalidateEntities(updatedIds);
        WorkCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::Work> SCDWork::WorkTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Work> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using WorkCache = Database::TableCache<SCE::Work, WorkRelationshipField>;
    if (WorkCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = m_dbSubContext.getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "title"_L1
                       << "dict_language"_L1;

    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM work WHERE id IN (%2)")
                            .arg(selectPlaceholders.join(","_L1), inPlaceholders.join(","_L1));

    QSqlQuery q(db);
    q.prepare(sql);
    for (int id : ids)
        q.addBindValue(id);

    if (q.exec())
    {
        QList<int> foundIds;
        QHash<int, SCE::Work> workMap;
        while (q.next())
        {
            foundIds.append(q.value(0).toInt());
            SCE::Work work;
            work.id = q.value(0).toInt();
            work.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            work.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            work.title = q.value(3).toString();
            work.dictLanguage = q.value(4).toString();
            result.append(work);
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> bindersMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, WORK_BINDERS_JUNCTION);
        QHash<int, QList<int>> tagsMap =
            JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, foundIds, WORK_TAGS_JUNCTION);

        // Build result with relationships populated
        for (auto &work : result)
        {
            work.binders = bindersMap.value(work.id);
            work.tags = tagsMap.value(work.id);
        }

        // Cache the result
        WorkCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDWork::WorkTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction table relationships first
    JunctionTableOps::OrderedOneToMany::removeWithLeftIdsMany(db, ids, WORK_BINDERS_JUNCTION);
    JunctionTableOps::UnorderedOneToMany::removeWithLeftIdsMany(db, ids, WORK_TAGS_JUNCTION);
    // Clean up junction backward table relationships
    JunctionTableOps::OrderedOneToMany::removeWithRightIdsMany(db, ids, ROOT_WORKS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM work WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using WorkCache = Database::TableCache<SCE::Work, WorkRelationshipField>;
        WorkCache::instance().invalidateEntities(removed);
        WorkCache::instance().invalidateRelationships(removed);
    }

    return removed;
}
void SCDWork::WorkTable::setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case WorkRelationshipField::Binders:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, workId, WORK_BINDERS_JUNCTION, relatedId);
        break;
    case WorkRelationshipField::Tags:
        JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, workId, WORK_TAGS_JUNCTION, relatedId);
        break;
    }

    // Invalidate cache for relationship changes
    using WorkCache = Database::TableCache<SCE::Work, WorkRelationshipField>;
    WorkCache::instance().invalidateEntity(workId);
    WorkCache::instance().invalidateRelationships(workId);
}

QHash<int, QList<int>> SCDWork::WorkTable::getRelationshipIdsMany(const QList<int> &workIds,
                                                                  WorkRelationshipField relationship) const
{
    // Try cache first
    using WorkCache = Database::TableCache<SCE::Work, WorkRelationshipField>;
    QHash<int, QList<int>> result;
    if (WorkCache::instance().getCachedRelationshipData(workIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case WorkRelationshipField::Binders:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, workIds, WORK_BINDERS_JUNCTION);
        break;

    case WorkRelationshipField::Tags:
        result = JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, workIds, WORK_TAGS_JUNCTION);
        break;

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    WorkCache::instance().setCachedRelationshipData(workIds, relationship, result);

    return result;
}

int SCDWork::WorkTable::getRelationshipIdsCount(int workId, WorkRelationshipField relationship)
{
    QSqlDatabase db = m_dbSubContext.getConnection();
    int result;

    switch (relationship)
    {
    case WorkRelationshipField::Binders:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsCount(db, workId, WORK_BINDERS_JUNCTION);
        break;

    case WorkRelationshipField::Tags:
        result = JunctionTableOps::UnorderedOneToMany::getRightIdsCount(db, workId, WORK_TAGS_JUNCTION);
        break;

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDWork::WorkTable::getRelationshipIdsInRange(int workId, WorkRelationshipField relationship, int offset,
                                                         int limit)
{
    QSqlDatabase db = m_dbSubContext.getConnection();
    QList<int> result;

    switch (relationship)
    {
    case WorkRelationshipField::Binders:
        result =
            JunctionTableOps::OrderedOneToMany::getRightIdsInRange(db, workId, WORK_BINDERS_JUNCTION, offset, limit);
        break;

    case WorkRelationshipField::Tags:
        result =
            JunctionTableOps::UnorderedOneToMany::getRightIdsInRange(db, workId, WORK_TAGS_JUNCTION, offset, limit);
        break;

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}