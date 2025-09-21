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
    const QString now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    for (SCE::Root r : roots)
    {
        q.prepare("INSERT INTO root (creation_date, update_date) VALUES (:c, :u)"_L1);
        q.bindValue(":c"_L1, now);
        q.bindValue(":u"_L1, now);
        if (!q.exec())
        {
            qCritical() << "Failed to insert root:" << q.lastError().text();
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

    return created;
}

QList<SCE::Root> SCDRoot::RootTable::updateMany(const QList<SCE::Root> &roots)
{
    QList<SCE::Root> updated;
    updated.reserve(roots.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);
    const QString now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    for (const SCE::Root &r : roots)
    {
        q.prepare("UPDATE root SET update_date = :u WHERE id = :id"_L1);
        q.bindValue(":u"_L1, now);
        q.bindValue(":id"_L1, r.id);
        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, ROOT_PROJECTS_JUNCTION, r.projects);
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, ROOT_RECENT_PROJECTS_JUNCTION,
                                                               r.recentProjects);

            updated.append(r);
        }
    }
    return updated;
}

QList<SCE::Root> SCDRoot::RootTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Root> result;
    result.reserve(ids.size());

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    if (ids.isEmpty())
        return result;

    // Build a dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT id FROM root WHERE id IN (%1)").arg(placeholders.join(","_L1));

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
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> projectsMap =
            JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, foundIds, ROOT_PROJECTS_JUNCTION);
        QHash<int, QList<int>> recentProjectsMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, ROOT_RECENT_PROJECTS_JUNCTION);

        // Build result with relationships populated
        for (int id : foundIds)
        {
            SCE::Root root(id, projectsMap.value(id), recentProjectsMap.value(id));
            result.append(root);
        }
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
    JunctionTableOps::UnorderedOneToMany::removeLeftIdsMany(db, ids, ROOT_PROJECTS_JUNCTION);
    JunctionTableOps::OrderedOneToMany::removeLeftIdsMany(db, ids, ROOT_RECENT_PROJECTS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM root WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    return removed;
}
void SCDRoot::RootTable::setRelationship(int rootId, RootRelationshipField relationship, QList<int> relatedId)
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
}

QHash<int, QList<int>> SCDRoot::RootTable::getRelationshipMany(const QList<int> &rootIds,
                                                               RootRelationshipField relationship) const
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QHash<int, QList<int>> result;

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

    return result;
}