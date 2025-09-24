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

#include "recent_project_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/recent_project.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// backward relationship junction tables
const QString ROOT_RECENT_PROJECTS_JUNCTION = "root_recent_projects_to_recent_project_junction"_L1;

SCDRecentProject::RecentProjectTable::RecentProjectTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::RecentProject> SCDRecentProject::RecentProjectTable::createMany(
    const QList<SCE::RecentProject> &recentProjects)
{
    QList<SCE::RecentProject> created;
    created.reserve(recentProjects.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    for (SCE::RecentProject r : recentProjects)
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
                    << "title"_L1 << "last_opened_at"_L1
                    << "absolute_path"_L1;

        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":title"_L1 << ":last_opened_at"_L1
                          << ":absolute_path"_L1;

        QString sqlString = "INSERT INTO recent_project (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1),
                                                                                 valuePlaceholders.join(","_L1));

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
        q.bindValue(":last_opened_at"_L1, r.lastOpenedAt.toString(Qt::ISODate));
        q.bindValue(":absolute_path"_L1, r.absolutePath);
        if (!q.exec())
        {
            qCritical() << "Failed to insert recentProject:" << q.lastError().text();
            // If insert fails, skip this row
            continue;
        }
        // Retrieve the auto-generated id
        QSqlQuery idq(db);
        if (idq.exec("SELECT last_insert_rowid()"_L1) && idq.next())
        {
            r.id = idq.value(0).toInt();

            created.append(r);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &recentProject : created)
            createdIds.append(recentProject.id);

        using RecentProjectCache = Database::TableCache<SCE::RecentProject, RecentProjectRelationshipField>;
        RecentProjectCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::RecentProject> SCDRecentProject::RecentProjectTable::updateMany(
    const QList<SCE::RecentProject> &recentProjects)
{
    QList<SCE::RecentProject> updated;
    updated.reserve(recentProjects.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "title = :title"_L1 << "last_opened_at = :last_opened_at"_L1
                << "absolute_path = :absolute_path"_L1;

    QString sqlString = "UPDATE recent_project SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::RecentProject &r : recentProjects)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":title"_L1, r.title);
        q.bindValue(":last_opened_at"_L1, r.lastOpenedAt.toString(Qt::ISODate));
        q.bindValue(":absolute_path"_L1, r.absolutePath);

        if (q.exec() && q.numRowsAffected() > 0)
        {
            updated.append(r);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &recentProject : updated)
            updatedIds.append(recentProject.id);

        using RecentProjectCache = Database::TableCache<SCE::RecentProject, RecentProjectRelationshipField>;
        RecentProjectCache::instance().invalidateEntities(updatedIds);
        RecentProjectCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::RecentProject> SCDRecentProject::RecentProjectTable::findMany(const QList<int> &ids) const
{
    QList<SCE::RecentProject> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using RecentProjectCache = Database::TableCache<SCE::RecentProject, RecentProjectRelationshipField>;
    if (RecentProjectCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "title"_L1
                       << "last_opened_at"_L1
                       << "absolute_path"_L1;

    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM recent_project WHERE id IN (%2)")
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
            SCE::RecentProject recentProject;
            recentProject.id = q.value(0).toInt();
            recentProject.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            recentProject.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            recentProject.title = q.value(3).toString();
            recentProject.lastOpenedAt = QDateTime::fromString(q.value(4).toString(), Qt::ISODate);
            recentProject.absolutePath = q.value(5).toString();
            result.append(recentProject);
        }

        // Cache the result
        RecentProjectCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDRecentProject::RecentProjectTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction backward table relationships
    JunctionTableOps::OrderedOneToMany::removeWithRightIdsMany(db, ids, ROOT_RECENT_PROJECTS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM recent_project WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using RecentProjectCache = Database::TableCache<SCE::RecentProject, RecentProjectRelationshipField>;
        RecentProjectCache::instance().invalidateEntities(removed);
        RecentProjectCache::instance().invalidateRelationships(removed);
    }

    return removed;
}
void SCDRecentProject::RecentProjectTable::setRelationshipIds(int recentProjectId,
                                                              RecentProjectRelationshipField relationship,
                                                              QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    // Invalidate cache for relationship changes
    using RecentProjectCache = Database::TableCache<SCE::RecentProject, RecentProjectRelationshipField>;
    RecentProjectCache::instance().invalidateEntity(recentProjectId);
    RecentProjectCache::instance().invalidateRelationships(recentProjectId);
}

QHash<int, QList<int>> SCDRecentProject::RecentProjectTable::getRelationshipIdsMany(
    const QList<int> &recentProjectIds, RecentProjectRelationshipField relationship) const
{
    // Try cache first
    using RecentProjectCache = Database::TableCache<SCE::RecentProject, RecentProjectRelationshipField>;
    QHash<int, QList<int>> result;
    if (RecentProjectCache::instance().getCachedRelationshipData(recentProjectIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();

    switch (relationship)
    {
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    RecentProjectCache::instance().setCachedRelationshipData(recentProjectIds, relationship, result);

    return result;
}

int SCDRecentProject::RecentProjectTable::getRelationshipIdsCount(int recentProjectId,
                                                                  RecentProjectRelationshipField relationship)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    int result;

    switch (relationship)
    {
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDRecentProject::RecentProjectTable::getRelationshipIdsInRange(int recentProjectId,
                                                                           RecentProjectRelationshipField relationship,
                                                                           int offset, int limit)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QList<int> result;

    switch (relationship)
    {
    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}