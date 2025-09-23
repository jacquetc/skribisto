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

#pragma once

#include "database/db_context.h"
#include "direct_access/binder_item/binder_item_events.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "direct_access/event_registry.h"
#include "entities/binder_item.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::BinderItem
{
namespace SCE = Skribisto::Common::Entities;

class IBinderItemTable
{
  public:
    virtual ~IBinderItemTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::BinderItem> createMany(const QList<SCE::BinderItem> &binderItems) = 0;

    // Update
    virtual QList<SCE::BinderItem> updateMany(const QList<SCE::BinderItem> &binderItems) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::BinderItem> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given BinderItem id (e.g., set project id)
    virtual void setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship,
                                    QList<int> relatedId) = 0;

    // Get the relationship value for a given BinderItem id (e.g., get project id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(
        const QList<int> &binderItemIds, BinderItemRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int binderItemId, BinderItemRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int binderItemId, BinderItemRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

class BinderItemRepository : public IBinderItemRepository
{
  public:
    BinderItemRepository(std::unique_ptr<IBinderItemTable> table, Database::DbSubContext &dbSubContext,
                         QPointer<EventRegistry> eventRegistry);

    ~BinderItemRepository() override = default;

    // CRUD
    QList<SCE::BinderItem> create(const QList<SCE::BinderItem> &binderItems) override;
    QList<SCE::BinderItem> get(const QList<int> &binderItemIds) override;
    QList<SCE::BinderItem> update(const QList<SCE::BinderItem> &binderItems) override;
    QList<int> remove(const QList<int> &binderItemIds) override;

    // Relationships
    void setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationshipIds(int binderItemId, BinderItemRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderItemIds,
                                                  BinderItemRelationshipField relationship) override;
    int getRelationshipIdsCount(int binderItemId, BinderItemRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int binderItemId, BinderItemRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IBinderItemTable> m_table;
    QPointer<BinderItemEvents> m_events;     // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int binderItemId, BinderItemRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::BinderItem
