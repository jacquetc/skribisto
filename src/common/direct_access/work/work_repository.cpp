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

#include "direct_access/work/work_repository.h"
#include "direct_access/repository_factory.h"
#include <QSet>
#include <utility>

namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCE = Skribisto::Common::Entities;

// Original constructor for backward compatibility
SCDWork::WorkRepository::WorkRepository(std::unique_ptr<IWorkTable> table, Database::DbSubContext &dbSubContext,
                                        QPointer<EventRegistry> eventRegistry)
    : m_table(std::move(table)), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<Work::WorkEvents>() : nullptr;
}

QList<SCE::Work> SCDWork::WorkRepository::create(const QList<SCE::Work> &works)
{
    auto created = m_table->createMany(works);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::Work> SCDWork::WorkRepository::get(const QList<int> &workIds)
{
    return m_table->findMany(workIds);
}

QList<SCE::Work> SCDWork::WorkRepository::update(const QList<SCE::Work> &works)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(works.size());
    for (const auto &r : works)
        ids.append(r.id);
    auto existing = m_table->findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::Work> toUpdate;
    toUpdate.reserve(works.size());
    for (const auto &r : works)
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

QList<int> SCDWork::WorkRepository::remove(const QList<int> &workIds)
{
    // cascade deletion on binders
    QHash<int, QList<int>> leftIdToWorkIdsHash = getRelationshipIdsMany(workIds, WorkRelationshipField::Binders);
    // concatenate all rightIds
    QSet<int> binderIds; // renamed to avoid shadowing
    binderIds.reserve(leftIdToWorkIdsHash.size());
    for (const auto &ids : leftIdToWorkIdsHash)
    {
        QSet<int> idsSet(ids.begin(), ids.end());
        binderIds.unite(idsSet); // use unite to combine sets
    }

    if (!binderIds.isEmpty())
    {
        auto binderRepository = RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
        binderRepository->remove(binderIds.values());
    }

    // Remove works and emit events only for the explicitly removed works
    auto removed = m_table->removeMany(workIds);
    emitRemoved(removed);
    return removed;
}

void SCDWork::WorkRepository::setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedIds)
{
    m_table->setRelationshipIds(workId, relationship, relatedIds);

    emitRelationshipChanged(workId, relationship, relatedIds);
    emitUpdated(QList<int>{workId});
}

QList<int> SCDWork::WorkRepository::getRelationshipIds(int workId, WorkRelationshipField relationship)
{
    auto rels = getRelationshipIdsMany(QList<int>{workId}, relationship);
    return rels.value(workId, QList<int>{});
}

QHash<int, QList<int>> SCDWork::WorkRepository::getRelationshipIdsMany(const QList<int> &workIds,
                                                                       WorkRelationshipField relationship)
{
    return m_table->getRelationshipIdsMany(workIds, relationship);
}

int SCDWork::WorkRepository::getRelationshipIdsCount(int rootId, WorkRelationshipField relationship)
{
    return m_table->getRelationshipIdsCount(rootId, relationship);
}

QList<int> SCDWork::WorkRepository::getRelationshipIdsInRange(int rootId, WorkRelationshipField relationship,
                                                              int offset, int limit)
{
    return m_table->getRelationshipIdsInRange(rootId, relationship, offset, limit);
}

void SCDWork::WorkRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDWork::WorkRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDWork::WorkRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDWork::WorkRepository::emitRelationshipChanged(const int workId, WorkRelationshipField rel,
                                                      const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, workId),
                              Q_ARG(WorkRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
