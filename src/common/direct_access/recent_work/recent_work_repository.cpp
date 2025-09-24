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

#include "direct_access/recent_work/recent_work_repository.h"

#include "direct_access/repository_factory.h"

#include <utility>

namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;
namespace SCE = Skribisto::Common::Entities;

SCDRecentWork::RecentWorkRepository::RecentWorkRepository(std::unique_ptr<IRecentWorkTable> table,
                                                          Database::DbSubContext &dbSubContext,
                                                          QPointer<EventRegistry> eventRegistry)
    : m_table(std::move(table)), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<RecentWork::RecentWorkEvents>() : nullptr;
}

QList<SCE::RecentWork> SCDRecentWork::RecentWorkRepository::create(const QList<SCE::RecentWork> &recentWorks)
{
    auto created = m_table->createMany(recentWorks);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::RecentWork> SCDRecentWork::RecentWorkRepository::get(const QList<int> &recentWorkIds)
{
    return m_table->findMany(recentWorkIds);
}

QList<SCE::RecentWork> SCDRecentWork::RecentWorkRepository::update(const QList<SCE::RecentWork> &recentWorks)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(recentWorks.size());
    for (const auto &r : recentWorks)
        ids.append(r.id);
    auto existing = m_table->findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::RecentWork> toUpdate;
    toUpdate.reserve(recentWorks.size());
    for (const auto &r : recentWorks)
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

QList<int> SCDRecentWork::RecentWorkRepository::remove(const QList<int> &recentWorkIds)
{
    auto removed = m_table->removeMany(recentWorkIds);
    emitRemoved(removed);
    return removed;
}

void SCDRecentWork::RecentWorkRepository::setRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship,
                                                             QList<int> relatedIds)
{
    m_table->setRelationshipIds(recentWorkId, relationship, relatedIds);

    emitRelationshipChanged(recentWorkId, relationship, relatedIds);
    emitUpdated(QList<int>{recentWorkId});
}

QList<int> SCDRecentWork::RecentWorkRepository::getRelationshipIds(int recentWorkId,
                                                                   RecentWorkRelationshipField relationship)
{
    auto rels = getRelationshipIdsMany(QList<int>{recentWorkId}, relationship);
    return rels.value(recentWorkId, QList<int>{});
}

QHash<int, QList<int>> SCDRecentWork::RecentWorkRepository::getRelationshipIdsMany(
    const QList<int> &recentWorkIds, RecentWorkRelationshipField relationship)
{
    return m_table->getRelationshipIdsMany(recentWorkIds, relationship);
}

int Skribisto::Common::DirectAccess::RecentWork::RecentWorkRepository::getRelationshipIdsCount(
    int recentWorkId, RecentWorkRelationshipField relationship)
{
    return m_table->getRelationshipIdsCount(recentWorkId, relationship);
}

QList<int> Skribisto::Common::DirectAccess::RecentWork::RecentWorkRepository::getRelationshipIdsInRange(
    int recentWorkId, RecentWorkRelationshipField relationship, int offset, int limit)
{
    return m_table->getRelationshipIdsInRange(recentWorkId, relationship, offset, limit);
}

void SCDRecentWork::RecentWorkRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRecentWork::RecentWorkRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRecentWork::RecentWorkRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDRecentWork::RecentWorkRepository::emitRelationshipChanged(const int recentWorkId,
                                                                  RecentWorkRelationshipField rel,
                                                                  const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, recentWorkId),
                              Q_ARG(RecentWorkRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
