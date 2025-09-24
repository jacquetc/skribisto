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

#include "root_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/root.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// forward relationship junction tables
const QString ROOT_PROJECTS_JUNCTION = "root_projects_to_project_junction"_L1;
const QString ROOT_RECENT_PROJECTS_JUNCTION = "root_recent_projects_to_recent_project_junction"_L1;

SCDRoot::RootTable::RootTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::Root> SCDRoot::RootTable::createMany(const QList<SCE::Root> &roots)
{
    QList<SCE::Root> created;
    created.reserve(roots.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    for (SCE::Root r : roots)
    {

        // Set timestamps if not provided
        if (r.createdAt.isNull())
            r.createdAt = QDateTime::currentDateTimeUtc();
        if (r.updatedAt.isNull())
            r.updatedAt = r.createdAt;

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
                    << "author_name"_L1;
        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":author_name"_L1;

        QString sqlString =
            "INSERT INTO root (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

        q.prepare(sqlString);

        if (r.id > 0)
            q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":author_name"_L1, r.authorName);
        if (!q.exec())
        {
            qCritical() << "Failed to insert root:" << q.lastError().text() << " SQL:" << sqlString;
            // If insert fails, skip this row
            continue;
        }
        // Retrieve the auto-generated id
        QSqlQuery idq(db);
        if (idq.exec("SELECT last_insert_rowid()"_L1) && idq.next())
        {
            r.id = idq.value(0).toInt();

            // Handle junction table relationships
            if (!r.projects.isEmpty())
            {
                JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, ROOT_PROJECTS_JUNCTION, r.projects);
            }
            if (!r.recentProjects.isEmpty())
            {
                JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, ROOT_RECENT_PROJECTS_JUNCTION,
                                                                   r.recentProjects);
            }

            created.append(r);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &root : created)
            createdIds.append(root.id);

        using RootCache = Database::TableCache<SCE::Root, RootRelationshipField>;
        RootCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::Root> SCDRoot::RootTable::updateMany(const QList<SCE::Root> &roots)
{
    QList<SCE::Root> updated;
    updated.reserve(roots.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "author_name = :author_name"_L1;

    QString sqlString = "UPDATE root SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::Root &r : roots)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":author_name"_L1, r.authorName);

        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, ROOT_PROJECTS_JUNCTION, r.projects);
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, ROOT_RECENT_PROJECTS_JUNCTION,
                                                               r.recentProjects);

            updated.append(r);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &root : updated)
            updatedIds.append(root.id);

        using RootCache = Database::TableCache<SCE::Root, RootRelationshipField>;
        RootCache::instance().invalidateEntities(updatedIds);
        RootCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::Root> SCDRoot::RootTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Root> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using RootCache = Database::TableCache<SCE::Root, RootRelationshipField>;
    if (RootCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "author_name"_L1;
    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM root WHERE id IN (%2)")
                            .arg(selectPlaceholders.join(","_L1), inPlaceholders.join(","_L1));

    QSqlQuery q(db);
    q.prepare(sql);
    for (int id : ids)
        q.addBindValue(id);

    if (q.exec())
    {
        QList<int> foundIds;
        while (q.next())
        {
            foundIds.append(q.value(0).toInt());
            SCE::Root root;
            root.id = q.value(0).toInt();
            root.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            root.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            root.authorName = q.value(3).toString();
            result.append(root);
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> projectsMap =
            JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, foundIds, ROOT_PROJECTS_JUNCTION);
        QHash<int, QList<int>> recentProjectsMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, ROOT_RECENT_PROJECTS_JUNCTION);

        // Build result with relationships populated
        for (auto &root : result)
        {
            root.projects = projectsMap[root.id];
            root.recentProjects = recentProjectsMap[root.id];
        }

        // Cache the result
        RootCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDRoot::RootTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction table relationships first
    JunctionTableOps::UnorderedOneToMany::removeWithLeftIdsMany(db, ids, ROOT_PROJECTS_JUNCTION);
    JunctionTableOps::OrderedOneToMany::removeWithLeftIdsMany(db, ids, ROOT_RECENT_PROJECTS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM root WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using RootCache = Database::TableCache<SCE::Root, RootRelationshipField>;
        RootCache::instance().invalidateEntities(removed);
        RootCache::instance().invalidateRelationships(removed);
    }

    return removed;
}
void SCDRoot::RootTable::setRelationshipIds(int rootId, RootRelationshipField relationship, QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case RootRelationshipField::Projects:
        JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, rootId, ROOT_PROJECTS_JUNCTION, relatedId);
        break;
    case RootRelationshipField::RecentProjects:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, rootId, ROOT_RECENT_PROJECTS_JUNCTION, relatedId);
        break;
    }

    // Invalidate cache for relationship changes
    using RootCache = Database::TableCache<SCE::Root, RootRelationshipField>;
    RootCache::instance().invalidateEntity(rootId);
    RootCache::instance().invalidateRelationships(rootId);
}

QHash<int, QList<int>> SCDRoot::RootTable::getRelationshipIdsMany(const QList<int> &rootIds,
                                                                  RootRelationshipField relationship) const
{
    // Try cache first
    using RootCache = Database::TableCache<SCE::Root, RootRelationshipField>;
    QHash<int, QList<int>> result;
    if (RootCache::instance().getCachedRelationshipData(rootIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();

    switch (relationship)
    {
    case RootRelationshipField::Projects:
        result = JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, rootIds, ROOT_PROJECTS_JUNCTION);
        break;
    case RootRelationshipField::RecentProjects:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, rootIds, ROOT_RECENT_PROJECTS_JUNCTION);
        break;

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    RootCache::instance().setCachedRelationshipData(rootIds, relationship, result);

    return result;
}

int SCDRoot::RootTable::getRelationshipIdsCount(int rootId, RootRelationshipField relationship)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    int result;

    switch (relationship)
    {
    case RootRelationshipField::Projects:
        result = JunctionTableOps::UnorderedOneToMany::getRightIdsCount(db, rootId, ROOT_PROJECTS_JUNCTION);
        break;
    case RootRelationshipField::RecentProjects:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsCount(db, rootId, ROOT_RECENT_PROJECTS_JUNCTION);
        break;

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDRoot::RootTable::getRelationshipIdsInRange(int rootId, RootRelationshipField relationship, int offset,
                                                         int limit)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QList<int> result;

    switch (relationship)
    {
    case RootRelationshipField::Projects:
        result =
            JunctionTableOps::UnorderedOneToMany::getRightIdsInRange(db, rootId, ROOT_PROJECTS_JUNCTION, offset, limit);
        break;
    case RootRelationshipField::RecentProjects:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsInRange(db, rootId, ROOT_RECENT_PROJECTS_JUNCTION,
                                                                        offset, limit);
        break;

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}