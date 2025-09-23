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

#include "direct_access/binder_item/binder_item_repository.h"

#include "direct_access/repository_factory.h"

#include <utility>

namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCE = Skribisto::Common::Entities;

SCDBinderItem::BinderItemRepository::BinderItemRepository(std::unique_ptr<IBinderItemTable> table,
                                                          Database::DbSubContext &dbSubContext,
                                                          QPointer<EventRegistry> eventRegistry)
    : m_table(std::move(table)), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<BinderItem::BinderItemEvents>() : nullptr;
}

QList<SCE::BinderItem> SCDBinderItem::BinderItemRepository::create(const QList<SCE::BinderItem> &binderItems)
{
    auto created = m_table->createMany(binderItems);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::BinderItem> SCDBinderItem::BinderItemRepository::get(const QList<int> &binderItemIds)
{
    return m_table->findMany(binderItemIds);
}

QList<SCE::BinderItem> SCDBinderItem::BinderItemRepository::update(const QList<SCE::BinderItem> &binderItems)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(binderItems.size());
    for (const auto &r : binderItems)
        ids.append(r.id);
    auto existing = m_table->findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::BinderItem> toUpdate;
    toUpdate.reserve(binderItems.size());
    for (const auto &r : binderItems)
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

QList<int> SCDBinderItem::BinderItemRepository::remove(const QList<int> &binderItemIds)
{
    // cascade deletion on contents
    QHash<int, QList<int>> leftIdToContentIdsHash =
        getRelationshipIdsMany(binderItemIds, BinderItemRelationshipField::Contents);
    // concatenate all rightIds
    QSet<int> contentIds;
    contentIds.reserve(leftIdToContentIdsHash.size());
    for (const auto &ids : leftIdToContentIdsHash)
    {
        QSet<int> idsSet(ids.begin(), ids.end());
        contentIds.unite(idsSet); // use unite to combine sets
    }

    // if (!contentIds.isEmpty())
    // {
    //     auto contentRepository = RepositoryFactory::createContentRepository(m_dbSubContext, m_eventRegistry);
    //     contentRepository->remove(contentIds.values());
    // }

    auto removed = m_table->removeMany(binderItemIds);
    emitRemoved(removed);
    return removed;
}

void SCDBinderItem::BinderItemRepository::setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship,
                                                             QList<int> relatedIds)
{
    m_table->setRelationshipIds(binderItemId, relationship, relatedIds);

    emitRelationshipChanged(binderItemId, relationship, relatedIds);
    emitUpdated(QList<int>{binderItemId});
}

QList<int> SCDBinderItem::BinderItemRepository::getRelationshipIds(int binderItemId,
                                                                   BinderItemRelationshipField relationship)
{
    auto rels = getRelationshipIdsMany(QList<int>{binderItemId}, relationship);
    return rels.value(binderItemId, QList<int>{});
}

QHash<int, QList<int>> SCDBinderItem::BinderItemRepository::getRelationshipIdsMany(
    const QList<int> &binderItemIds, BinderItemRelationshipField relationship)
{
    return m_table->getRelationshipIdsMany(binderItemIds, relationship);
}

int Skribisto::Common::DirectAccess::BinderItem::BinderItemRepository::getRelationshipIdsCount(
    int binderItemId, BinderItemRelationshipField relationship)
{
    return m_table->getRelationshipIdsCount(binderItemId, relationship);
}

QList<int> Skribisto::Common::DirectAccess::BinderItem::BinderItemRepository::getRelationshipIdsInRange(
    int binderItemId, BinderItemRelationshipField relationship, int offset, int limit)
{
    return m_table->getRelationshipIdsInRange(binderItemId, relationship, offset, limit);
}

void SCDBinderItem::BinderItemRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinderItem::BinderItemRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinderItem::BinderItemRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinderItem::BinderItemRepository::emitRelationshipChanged(const int binderItemId,
                                                                  BinderItemRelationshipField rel,
                                                                  const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, binderItemId),
                              Q_ARG(BinderItemRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
