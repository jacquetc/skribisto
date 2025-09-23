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
#include "direct_access/binder/binder_events.h"
#include "direct_access/binder/i_binder_repository.h"
#include "direct_access/event_registry.h"
#include "entities/binder.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::Binder
{
namespace SCE = Skribisto::Common::Entities;

class IBinderTable
{
  public:
    virtual ~IBinderTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::Binder> createMany(const QList<SCE::Binder> &binders) = 0;

    // Update
    virtual QList<SCE::Binder> updateMany(const QList<SCE::Binder> &binders) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::Binder> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;

    // Relationship setters/getters
    // Set the relationship value for a given Binder id (e.g., set binder item id)
    virtual void setRelationshipIds(int binderId, BinderRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Binder id (e.g., get binder item id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderIds,
                                                                        BinderRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int rootId, BinderRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int rootId, BinderRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

class BinderRepository : public IBinderRepository
{
  public:
    BinderRepository(std::unique_ptr<IBinderTable> table, Database::DbSubContext &dbSubContext,
                     QPointer<EventRegistry> eventRegistry);
    ~BinderRepository() override = default;

    // CRUD
    QList<SCE::Binder> create(const QList<SCE::Binder> &binders) override;
    QList<SCE::Binder> get(const QList<int> &binderIds) override;
    QList<SCE::Binder> update(const QList<SCE::Binder> &binders) override;
    QList<int> remove(const QList<int> &binderIds) override;

    // Relationships
    void setRelationshipIds(int binderId, BinderRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationshipIds(int binderId, BinderRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderIds,
                                                  BinderRelationshipField relationship) override;
    int getRelationshipIdsCount(int rootId, BinderRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int rootId, BinderRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IBinderTable> m_table;
    QPointer<BinderEvents> m_events;         // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int binderId, BinderRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::Binder
