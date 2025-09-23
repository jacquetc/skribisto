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

#include "direct_access/recent_project/recent_project_repository.h"

#include "direct_access/repository_factory.h"

#include <utility>

namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;
namespace SCE = Skribisto::Common::Entities;

SCDRecentProject::RecentProjectRepository::RecentProjectRepository(std::unique_ptr<IRecentProjectTable> table,
                                                                   Database::DbSubContext &dbSubContext,
                                                                   QPointer<EventRegistry> eventRegistry)
    : m_table(std::move(table)), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<RecentProject::RecentProjectEvents>() : nullptr;
}

QList<SCE::RecentProject> SCDRecentProject::RecentProjectRepository::create(
    const QList<SCE::RecentProject> &recentProjects)
{
    auto created = m_table->createMany(recentProjects);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::RecentProject> SCDRecentProject::RecentProjectRepository::get(const QList<int> &recentProjectIds)
{
    return m_table->findMany(recentProjectIds);
}

QList<SCE::RecentProject> SCDRecentProject::RecentProjectRepository::update(
    const QList<SCE::RecentProject> &recentProjects)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(recentProjects.size());
    for (const auto &r : recentProjects)
        ids.append(r.id);
    auto existing = m_table->findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::RecentProject> toUpdate;
    toUpdate.reserve(recentProjects.size());
    for (const auto &r : recentProjects)
        if (existingIds.contains(r.id))
            toUpdate.append(r);

    auto updated = m_table->updateMany(toUpdate);
    QList<int> updatedIds;
    updatedIds.reserve(updated.size());
    for (const auto &r : updated)
        updatedIds.append(r.id);
    emitUpdated(updatedIds);
    return updated;
}

QList<int> SCDRecentProject::RecentProjectRepository::remove(const QList<int> &recentProjectIds)
{
    auto removed = m_table->removeMany(recentProjectIds);
    emitRemoved(removed);
    return removed;
}

void SCDRecentProject::RecentProjectRepository::setRelationshipIds(int recentProjectId,
                                                                   RecentProjectRelationshipField relationship,
                                                                   QList<int> relatedIds)
{
    m_table->setRelationshipIds(recentProjectId, relationship, relatedIds);

    emitRelationshipChanged(recentProjectId, relationship, relatedIds);
    emitUpdated(QList<int>{recentProjectId});
}

QList<int> SCDRecentProject::RecentProjectRepository::getRelationshipIds(int recentProjectId,
                                                                         RecentProjectRelationshipField relationship)
{
    auto rels = getRelationshipIdsMany(QList<int>{recentProjectId}, relationship);
    return rels.value(recentProjectId, QList<int>{});
}

QHash<int, QList<int>> SCDRecentProject::RecentProjectRepository::getRelationshipIdsMany(
    const QList<int> &recentProjectIds, RecentProjectRelationshipField relationship)
{
    return m_table->getRelationshipIdsMany(recentProjectIds, relationship);
}

int Skribisto::Common::DirectAccess::RecentProject::RecentProjectRepository::getRelationshipIdsCount(
    int recentProjectId, RecentProjectRelationshipField relationship)
{
    return m_table->getRelationshipIdsCount(recentProjectId, relationship);
}

QList<int> Skribisto::Common::DirectAccess::RecentProject::RecentProjectRepository::getRelationshipIdsInRange(
    int recentProjectId, RecentProjectRelationshipField relationship, int offset, int limit)
{
    return m_table->getRelationshipIdsInRange(recentProjectId, relationship, offset, limit);
}

void SCDRecentProject::RecentProjectRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRecentProject::RecentProjectRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRecentProject::RecentProjectRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRecentProject::RecentProjectRepository::emitRelationshipChanged(const int recentProjectId,
                                                                        RecentProjectRelationshipField rel,
                                                                        const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, recentProjectId),
                              Q_ARG(RecentProjectRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
