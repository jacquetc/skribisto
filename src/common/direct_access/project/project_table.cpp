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

#include "project_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/project.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDProject = Skribisto::Common::DirectAccess::Project;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// forward relationship junction tables
const QString PROJECT_BINDERS_JUNCTION = "project_binders_to_binder_junction"_L1;
// backward relationship junction tables
const QString ROOT_PROJECTS_JUNCTION = "root_projects_to_project_junction"_L1;

SCDProject::ProjectTable::ProjectTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::Project> SCDProject::ProjectTable::createMany(const QList<SCE::Project> &projects)
{
    QList<SCE::Project> created;
    created.reserve(projects.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);
    for (SCE::Project r : projects)
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
            "INSERT INTO root (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

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
                JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, PROJECT_BINDERS_JUNCTION, r.binders);
            }

            created.append(r);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &project : created)
            createdIds.append(project.id);

        using ProjectCache = Database::TableCache<SCE::Project, ProjectRelationshipField>;
        ProjectCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::Project> SCDProject::ProjectTable::updateMany(const QList<SCE::Project> &projects)
{
    QList<SCE::Project> updated;
    updated.reserve(projects.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "title = :title"_L1
                << "dict_language = :dict_language"_L1;

    QString sqlString = "UPDATE binder_item SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::Project &r : projects)
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
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, PROJECT_BINDERS_JUNCTION, r.binders);

            updated.append(r);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &project : updated)
            updatedIds.append(project.id);

        using ProjectCache = Database::TableCache<SCE::Project, ProjectRelationshipField>;
        ProjectCache::instance().invalidateEntities(updatedIds);
        ProjectCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::Project> SCDProject::ProjectTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Project> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using ProjectCache = Database::TableCache<SCE::Project, ProjectRelationshipField>;
    if (ProjectCache::instance().getCachedEntities(ids, result))
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
                       << "dict_language"_L1;

    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM binder_item WHERE id IN (%2)")
                            .arg(selectPlaceholders.join(","_L1), inPlaceholders.join(","_L1));

    QSqlQuery q(db);
    q.prepare(sql);
    for (int id : ids)
        q.addBindValue(id);

    if (q.exec())
    {
        QList<int> foundIds;
        QHash<int, SCE::Project> projectMap;
        while (q.next())
        {
            foundIds.append(q.value(0).toInt());
            SCE::Project project;
            project.id = q.value(0).toInt();
            project.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            project.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            project.title = q.value(3).toString();
            project.dictLanguage = q.value(4).toString();
            result.append(project);
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> bindersMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, PROJECT_BINDERS_JUNCTION);

        // Build result with relationships populated
        for (auto &project : result)
        {
            project.binders = bindersMap.value(project.id);
        }

        // Cache the result
        ProjectCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDProject::ProjectTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction table relationships first
    JunctionTableOps::OrderedOneToMany::removeWithLeftIdsMany(db, ids, PROJECT_BINDERS_JUNCTION);
    // Clean up junction backward table relationships
    auto rightAndLeftIds = JunctionTableOps::OrderedOneToMany::getLeftIdMany(db, ROOT_PROJECTS_JUNCTION, ids);
    JunctionTableOps::OrderedOneToMany::removeWithRightIdsMany(db, rightAndLeftIds.values(), ROOT_PROJECTS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM project WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using ProjectCache = Database::TableCache<SCE::Project, ProjectRelationshipField>;
        ProjectCache::instance().invalidateEntities(removed);
        ProjectCache::instance().invalidateRelationships(removed);
    }

    return removed;
}
void SCDProject::ProjectTable::setRelationshipIds(int projectId, ProjectRelationshipField relationship,
                                                  QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case ProjectRelationshipField::Binders:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, projectId, PROJECT_BINDERS_JUNCTION, relatedId);
        break;
    }

    // Invalidate cache for relationship changes
    using ProjectCache = Database::TableCache<SCE::Project, ProjectRelationshipField>;
    ProjectCache::instance().invalidateEntity(projectId);
    ProjectCache::instance().invalidateRelationships(projectId);
}

QHash<int, QList<int>> SCDProject::ProjectTable::getRelationshipIdsMany(const QList<int> &projectIds,
                                                                        ProjectRelationshipField relationship) const
{
    // Try cache first
    using ProjectCache = Database::TableCache<SCE::Project, ProjectRelationshipField>;
    QHash<int, QList<int>> result;
    if (ProjectCache::instance().getCachedRelationshipData(projectIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    switch (relationship)
    {
    case ProjectRelationshipField::Binders:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, projectIds, PROJECT_BINDERS_JUNCTION);
        break;
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    ProjectCache::instance().setCachedRelationshipData(projectIds, relationship, result);

    return result;
}

int SCDProject::ProjectTable::getRelationshipIdsCount(int projectId, ProjectRelationshipField relationship)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    int result;

    switch (relationship)
    {
    case ProjectRelationshipField::Binders:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsCount(db, projectId, PROJECT_BINDERS_JUNCTION);
        break;
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDProject::ProjectTable::getRelationshipIdsInRange(int projectId, ProjectRelationshipField relationship,
                                                               int offset, int limit)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QList<int> result;

    switch (relationship)
    {
    case ProjectRelationshipField::Binders:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsInRange(db, projectId, PROJECT_BINDERS_JUNCTION, offset,
                                                                        limit);
        break;

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}