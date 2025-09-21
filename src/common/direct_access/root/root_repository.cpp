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

#include "direct_access/root/root_repository.h"

#include "direct_access/repository_factory.h"

#include <utility>

namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCE = Skribisto::Common::Entities;

SCDRoot::RootRepository::RootRepository(IRootTable &table, Database::DbSubContext &dbSubContext,
                                        QPointer<EventRegistry> eventRegistry)
    : m_table(table), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<Root::RootEvents>() : nullptr;
}

QList<SCE::Root> SCDRoot::RootRepository::create(const QList<SCE::Root> &roots)
{
    auto created = m_table.createMany(roots);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::Root> SCDRoot::RootRepository::get(const QList<int> &rootIds)
{
    return m_table.findMany(rootIds);
}

QList<SCE::Root> SCDRoot::RootRepository::update(const QList<SCE::Root> &roots)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(roots.size());
    for (const auto &r : roots)
        ids.append(r.id);
    auto existing = m_table.findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::Root> toUpdate;
    toUpdate.reserve(roots.size());
    for (const auto &r : roots)
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

QList<int> SCDRoot::RootRepository::remove(const QList<int> &rootIds)
{
    // cascade deletion on projects
    QHash<int, QList<int>> leftIdToProjectIdsHash = getRelationshipMany(rootIds, RootRelationshipField::Projects);
    // concatenate all rightIds
    QSet<int> projectIds;
    projectIds.reserve(leftIdToProjectIdsHash.size());
    for (const auto &ids : leftIdToProjectIdsHash)
    {
        QSet<int> idsSet(ids.begin(), ids.end());
        projectIds.unite(idsSet); // use unite to combine sets
    }
    RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry).remove(projectIds.values());

    // cascade deletion on recent projects
    QHash<int, QList<int>> leftIdToRecentProjectIdsHash =
        getRelationshipMany(rootIds, RootRelationshipField::RecentProjects);
    // concatenate all rightIds
    QSet<int> recentProjectIds;
    recentProjectIds.reserve(leftIdToRecentProjectIdsHash.size());
    for (const auto &ids : leftIdToRecentProjectIdsHash)
    {
        QSet<int> idsSet(ids.begin(), ids.end());
        recentProjectIds.unite(idsSet); // use unite to combine sets
    }
    // RepositoryFactory::createRecentProjectRepository(m_dbSubContext, m_eventRegistry)
    //     .remove(recentProjectIds.values());

    auto removed = m_table.removeMany(rootIds);
    emitRemoved(removed);
    return removed;
}

void SCDRoot::RootRepository::setRelationship(int rootId, RootRelationshipField relationship, QList<int> relatedIds)
{
    m_table.setRelationship(rootId, relationship, relatedIds);

    emitRelationshipChanged(rootId, relationship, relatedIds);
    emitUpdated(QList<int>{rootId});
}

QList<int> SCDRoot::RootRepository::getRelationship(int rootId, RootRelationshipField relationship)
{
    auto rels = getRelationshipMany(QList<int>{rootId}, relationship);
    return rels.value(rootId, QList<int>{});
}

QHash<int, QList<int>> SCDRoot::RootRepository::getRelationshipMany(const QList<int> &rootIds,
                                                                    RootRelationshipField relationship)
{
    return m_table.getRelationshipMany(rootIds, relationship);
}

void SCDRoot::RootRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRoot::RootRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRoot::RootRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRoot::RootRepository::emitRelationshipChanged(const int rootId, RootRelationshipField rel,
                                                      const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, rootId),
                              Q_ARG(RootRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
