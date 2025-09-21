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

#include "direct_access/project/project_repository.h"
#include "direct_access/repository_factory.h"
#include <QSet>
#include <utility>

namespace SCDProject = Skribisto::Common::DirectAccess::Project;
namespace SCE = Skribisto::Common::Entities;

// Original constructor for backward compatibility
SCDProject::ProjectRepository::ProjectRepository(IProjectTable &table, Database::DbSubContext &dbSubContext,
                                                 QPointer<EventRegistry> eventRegistry)
    : m_table(table), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<Project::ProjectEvents>() : nullptr;
}

QList<SCE::Project> SCDProject::ProjectRepository::create(const QList<SCE::Project> &projects)
{
    auto created = m_table.createMany(projects);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::Project> SCDProject::ProjectRepository::get(const QList<int> &projectIds)
{
    return m_table.findMany(projectIds);
}

QList<SCE::Project> SCDProject::ProjectRepository::update(const QList<SCE::Project> &projects)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(projects.size());
    for (const auto &r : projects)
        ids.append(r.id);
    auto existing = m_table.findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::Project> toUpdate;
    toUpdate.reserve(projects.size());
    for (const auto &r : projects)
        if (existingIds.contains(r.id))
            toUpdate.append(r);

    auto updated = m_table.updateMany(toUpdate);
    QList<int> updatedIds;
    updatedIds.reserve(updated.size());
    for (const auto &r : updated)
        updatedIds.append(r.id);
    emitUpdated(updatedIds);
    return updated;
}

QList<int> SCDProject::ProjectRepository::remove(const QList<int> &projectIds)
{
    // cascade deletion on binders
    QHash<int, QList<int>> leftIdToProjectIdsHash = getRelationshipMany(projectIds, ProjectRelationshipField::Binders);
    // concatenate all rightIds
    QSet<int> binderIds; // renamed to avoid shadowing
    binderIds.reserve(leftIdToProjectIdsHash.size());
    for (const auto &ids : leftIdToProjectIdsHash)
    {
        QSet<int> idsSet(ids.begin(), ids.end());
        binderIds.unite(idsSet); // use unite to combine sets
    }
    RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry).remove(binderIds.values());

    // Remove projects and emit events only for the explicitly removed projects
    auto removed = m_table.removeMany(projectIds);
    emitRemoved(removed);
    return removed;
}

void SCDProject::ProjectRepository::setRelationship(int projectId, ProjectRelationshipField relationship,
                                                    QList<int> relatedIds)
{
    m_table.setRelationship(projectId, relationship, relatedIds);

    emitRelationshipChanged(projectId, relationship, relatedIds);
    emitUpdated(QList<int>{projectId});
}

QList<int> SCDProject::ProjectRepository::getRelationship(int projectId, ProjectRelationshipField relationship)
{
    auto rels = getRelationshipMany(QList<int>{projectId}, relationship);
    return rels.value(projectId, QList<int>{});
}

QHash<int, QList<int>> SCDProject::ProjectRepository::getRelationshipMany(const QList<int> &projectIds,
                                                                          ProjectRelationshipField relationship)
{
    return m_table.getRelationshipMany(projectIds, relationship);
}

void SCDProject::ProjectRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDProject::ProjectRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDProject::ProjectRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDProject::ProjectRepository::emitRelationshipChanged(const int projectId, ProjectRelationshipField rel,
                                                            const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, projectId),
                              Q_ARG(ProjectRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
