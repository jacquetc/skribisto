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

#include "direct_access/binder_tag/binder_tag_repository.h"

#include "direct_access/repository_factory.h"

#include <utility>

namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;
namespace SCE = Skribisto::Common::Entities;

SCDBinderTag::BinderTagRepository::BinderTagRepository(std::unique_ptr<IBinderTagTable> table,
                                                       Database::DbSubContext &dbSubContext,
                                                       QPointer<EventRegistry> eventRegistry)
    : m_table(std::move(table)), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<BinderTag::BinderTagEvents>() : nullptr;
}

QList<SCE::BinderTag> SCDBinderTag::BinderTagRepository::create(const QList<SCE::BinderTag> &binderTags)
{
    auto created = m_table->createMany(binderTags);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &r : created)
        ids.append(r.id);
    emitCreated(ids);
    return created;
}

QList<SCE::BinderTag> SCDBinderTag::BinderTagRepository::get(const QList<int> &binderTagIds)
{
    return m_table->findMany(binderTagIds);
}

QList<SCE::BinderTag> SCDBinderTag::BinderTagRepository::update(const QList<SCE::BinderTag> &binderTags)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(binderTags.size());
    for (const auto &r : binderTags)
        ids.append(r.id);
    auto existing = m_table->findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::BinderTag> toUpdate;
    toUpdate.reserve(binderTags.size());
    for (const auto &r : binderTags)
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

QList<int> SCDBinderTag::BinderTagRepository::remove(const QList<int> &binderTagIds)
{

    auto removed = m_table->removeMany(binderTagIds);
    emitRemoved(removed);
    return removed;
}

void SCDBinderTag::BinderTagRepository::setRelationshipIds(int binderTagId, BinderTagRelationshipField relationship,
                                                           QList<int> relatedIds)
{
    m_table->setRelationshipIds(binderTagId, relationship, relatedIds);

    emitRelationshipChanged(binderTagId, relationship, relatedIds);
    emitUpdated(QList<int>{binderTagId});
}

QList<int> SCDBinderTag::BinderTagRepository::getRelationshipIds(int binderTagId,
                                                                 BinderTagRelationshipField relationship)
{
    auto rels = getRelationshipIdsMany(QList<int>{binderTagId}, relationship);
    return rels.value(binderTagId, QList<int>{});
}

QHash<int, QList<int>> SCDBinderTag::BinderTagRepository::getRelationshipIdsMany(
    const QList<int> &binderTagIds, BinderTagRelationshipField relationship)
{
    return m_table->getRelationshipIdsMany(binderTagIds, relationship);
}

int Skribisto::Common::DirectAccess::BinderTag::BinderTagRepository::getRelationshipIdsCount(
    int binderTagId, BinderTagRelationshipField relationship)
{
    return m_table->getRelationshipIdsCount(binderTagId, relationship);
}

QList<int> Skribisto::Common::DirectAccess::BinderTag::BinderTagRepository::getRelationshipIdsInRange(
    int binderTagId, BinderTagRelationshipField relationship, int offset, int limit)
{
    return m_table->getRelationshipIdsInRange(binderTagId, relationship, offset, limit);
}

void SCDBinderTag::BinderTagRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinderTag::BinderTagRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinderTag::BinderTagRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinderTag::BinderTagRepository::emitRelationshipChanged(const int binderTagId, BinderTagRelationshipField rel,
                                                                const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, binderTagId),
                              Q_ARG(BinderTagRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
