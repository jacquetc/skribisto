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
    const QString now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    for (SCE::Project r : projects)
    {
        q.prepare("INSERT INTO project (creation_date, update_date, title, dict_language) VALUES (:c, :u, :t, :d)"_L1);
        q.bindValue(":c"_L1, now);
        q.bindValue(":u"_L1, now);
        q.bindValue(":t"_L1, r.title);
        q.bindValue(":d"_L1, r.dictLanguage);
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

    return created;
}

QList<SCE::Project> SCDProject::ProjectTable::updateMany(const QList<SCE::Project> &projects)
{
    QList<SCE::Project> updated;
    updated.reserve(projects.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);
    const QString now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    for (const SCE::Project &r : projects)
    {
        q.prepare("UPDATE project SET update_date = :u, title = :t, dict_language = :d WHERE id = :id"_L1);
        q.bindValue(":u"_L1, now);
        q.bindValue(":t"_L1, r.title);
        q.bindValue(":d"_L1, r.dictLanguage);
        q.bindValue(":id"_L1, r.id);
        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, PROJECT_BINDERS_JUNCTION, r.binders);

            updated.append(r);
        }
    }
    return updated;
}

QList<SCE::Project> SCDProject::ProjectTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Project> result;
    result.reserve(ids.size());

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    if (ids.isEmpty())
        return result;

    // Build a dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?"_L1, ids.size());
    const QString sql =
        QStringLiteral("SELECT id, creation_date, update_date, title, dict_language FROM project WHERE id IN (%1)")
            .arg(placeholders.join(","_L1));

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
            int id = q.value(0).toInt();
            QDateTime creationDate = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            QDateTime updateDate = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            QString title = q.value(3).toString();
            QString dictLanguage = q.value(4).toString();

            foundIds.append(id);
            projectMap[id] = SCE::Project();
            projectMap[id].id = id;
            projectMap[id].creationDate = creationDate;
            projectMap[id].updateDate = updateDate;
            projectMap[id].title = title;
            projectMap[id].dictLanguage = dictLanguage;
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> bindersMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, PROJECT_BINDERS_JUNCTION);

        // Build result with relationships populated
        for (int id : foundIds)
        {
            SCE::Project project = projectMap[id];
            project.binders = bindersMap.value(id);
            result.append(project);
        }
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
    JunctionTableOps::OrderedOneToMany::removeLeftIdsMany(db, ids, PROJECT_BINDERS_JUNCTION);
    // Clean up junction backward table relationships
    auto rightAndleftIds = JunctionTableOps::OrderedOneToMany::getLeftIdMany(db, ROOT_PROJECTS_JUNCTION, ids);
    JunctionTableOps::OrderedOneToMany::removeRightIdsMany(db, rightAndleftIds.values(), ROOT_PROJECTS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM project WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    return removed;
}
void SCDProject::ProjectTable::setRelationship(int projectId, ProjectRelationshipField relationship,
                                               QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case ProjectRelationshipField::Binders:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, projectId, PROJECT_BINDERS_JUNCTION, relatedId);
        break;
    }
}

QHash<int, QList<int>> SCDProject::ProjectTable::getRelationshipMany(const QList<int> &projectIds,
                                                                     ProjectRelationshipField relationship) const
{
    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();
    QHash<int, QList<int>> result;

    switch (relationship)
    {
    case ProjectRelationshipField::Binders:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, projectIds, PROJECT_BINDERS_JUNCTION);
        break;
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}
