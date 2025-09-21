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

#include "direct_access/binder/binder_repository.h"

#include "direct_access/event_registry.h"

#include <QSet>
#include <utility>

namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCE = Skribisto::Common::Entities;

SCDBinder::BinderRepository::BinderRepository(IBinderTable &table, Database::DbSubContext &dbSubContext,
                                              QPointer<EventRegistry> eventRegistry)
    : m_table(table), m_eventRegistry(std::move(eventRegistry)), m_dbSubContext(dbSubContext)
{
    m_events = m_eventRegistry ? m_eventRegistry->getEvents<Binder::BinderEvents>() : nullptr;
}

QList<SCE::Binder> SCDBinder::BinderRepository::create(const QList<SCE::Binder> &binders)
{
    auto created = m_table.createMany(binders);
    QList<int> ids;
    ids.reserve(created.size());
    for (const auto &b : created)
        ids.append(b.id);
    emitCreated(ids);
    return created;
}

QList<SCE::Binder> SCDBinder::BinderRepository::get(const QList<int> &binderIds)
{
    return m_table.findMany(binderIds);
}

QList<SCE::Binder> SCDBinder::BinderRepository::update(const QList<SCE::Binder> &binders)
{
    // Only update existing entries
    QList<int> ids;
    ids.reserve(binders.size());
    for (const auto &b : binders)
        ids.append(b.id);
    auto existing = m_table.findMany(ids);
    QSet<int> existingIds;
    existingIds.reserve(existing.size());
    for (const auto &e : existing)
        existingIds.insert(e.id);

    QList<SCE::Binder> toUpdate;
    toUpdate.reserve(binders.size());
    for (const auto &b : binders)
        if (existingIds.contains(b.id))
            toUpdate.append(b);

    auto updated = m_table.updateMany(toUpdate);
    QList<int> updatedIds;
    updatedIds.reserve(updated.size());
    for (const auto &b : updated)
        updatedIds.append(b.id);
    emitUpdated(updatedIds);
    return updated;
}

QList<int> SCDBinder::BinderRepository::remove(const QList<int> &binderIds)
{
    auto removed = m_table.removeMany(binderIds);
    emitRemoved(removed);
    return removed;
}

void SCDBinder::BinderRepository::setRelationship(int binderId, BinderRelationshipField relationship,
                                                  QList<int> relatedIds)
{
    m_table.setRelationship(binderId, relationship, relatedIds);

    emitRelationshipChanged(binderId, relationship, relatedIds);
    emitUpdated(QList<int>{binderId});
}

QList<int> SCDBinder::BinderRepository::getRelationship(int binderId, BinderRelationshipField relationship)
{
    auto rels = getRelationshipMany(QList<int>{binderId}, relationship);
    return rels.value(binderId, QList<int>{});
}

QHash<int, QList<int>> SCDBinder::BinderRepository::getRelationshipMany(const QList<int> &binderIds,
                                                                        BinderRelationshipField relationship)
{
    return m_table.getRelationshipMany(binderIds, relationship);
}

void SCDBinder::BinderRepository::emitCreated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishCreated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinder::BinderRepository::emitUpdated(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishUpdated", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinder::BinderRepository::emitRemoved(const QList<int> &ids) const
{
    if (!m_events || ids.isEmpty())
        return;
    QMetaObject::invokeMethod(m_events, "publishRemoved", Qt::QueuedConnection, Q_ARG(QList<int>, ids));
}

void SCDBinder::BinderRepository::emitRelationshipChanged(const int binderId, BinderRelationshipField rel,
                                                          const QList<int> &relatedIds) const
{
    if (!m_events)
        return;
    QMetaObject::invokeMethod(m_events, "publishRelationshipChanged", Qt::QueuedConnection, Q_ARG(int, binderId),
                              Q_ARG(BinderRelationshipField, rel), Q_ARG(QList<int>, relatedIds));
}
