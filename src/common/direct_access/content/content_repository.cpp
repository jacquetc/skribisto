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

#include "direct_access/content/content_repository.h"

#include "direct_access/repository_factory.h"

#include <utility>

namespace SCDContent = Skribisto::Common::DirectAccess::Content;
namespace SCE = Skribisto::Common::Entities;

SCDContent::ContentRepository::ContentRepository(std::unique_ptr<IContentTable> table,
                                                 Database::DbSubContext &dbSubContext,
                                                 QPointer<EventRegistry> eventRegistry)
    : m_table(std::move(table)), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<Content::ContentEvents>() : nullptr;
}

QList<SCE::Content> SCDContent::ContentRepository::create(const QList<SCE::Content> &contents)
{
    auto created = m_table->createMany(contents);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::Content> SCDContent::ContentRepository::get(const QList<int> &contentIds)
{
    return m_table->findMany(contentIds);
}

QList<SCE::Content> SCDContent::ContentRepository::update(const QList<SCE::Content> &contents)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(contents.size());
    for (const auto &r : contents)
        ids.append(r.id);
    auto existing = m_table->findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::Content> toUpdate;
    toUpdate.reserve(contents.size());
    for (const auto &r : contents)
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

QList<int> SCDContent::ContentRepository::remove(const QList<int> &contentIds)
{
    auto removed = m_table->removeMany(contentIds);
    emitRemoved(removed);
    return removed;
}

void SCDContent::ContentRepository::setRelationshipIds(int contentId, ContentRelationshipField relationship,
                                                       QList<int> relatedIds)
{
    m_table->setRelationshipIds(contentId, relationship, relatedIds);

    emitRelationshipChanged(contentId, relationship, relatedIds);
    emitUpdated(QList<int>{contentId});
}

QList<int> SCDContent::ContentRepository::getRelationshipIds(int contentId, ContentRelationshipField relationship)
{
    auto rels = getRelationshipIdsMany(QList<int>{contentId}, relationship);
    return rels.value(contentId, QList<int>{});
}

QHash<int, QList<int>> SCDContent::ContentRepository::getRelationshipIdsMany(const QList<int> &contentIds,
                                                                             ContentRelationshipField relationship)
{
    return m_table->getRelationshipIdsMany(contentIds, relationship);
}

int Skribisto::Common::DirectAccess::Content::ContentRepository::getRelationshipIdsCount(
    int contentId, ContentRelationshipField relationship)
{
    return m_table->getRelationshipIdsCount(contentId, relationship);
}

QList<int> Skribisto::Common::DirectAccess::Content::ContentRepository::getRelationshipIdsInRange(
    int contentId, ContentRelationshipField relationship, int offset, int limit)
{
    return m_table->getRelationshipIdsInRange(contentId, relationship, offset, limit);
}

void SCDContent::ContentRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDContent::ContentRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDContent::ContentRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDContent::ContentRepository::emitRelationshipChanged(const int contentId, ContentRelationshipField rel,
                                                            const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, contentId),
                              Q_ARG(ContentRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
