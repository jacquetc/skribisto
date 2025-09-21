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
#include "direct_access/event_registry.h"
#include "direct_access/root/i_root_repository.h"
#include "direct_access/root/root_events.h"
#include "entities/root.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::Root
{
namespace SCE = Skribisto::Common::Entities;

class IRootTable
{
  public:
    virtual ~IRootTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::Root> createMany(const QList<SCE::Root> &roots) = 0;

    // Update
    virtual QList<SCE::Root> updateMany(const QList<SCE::Root> &roots) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::Root> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Root id (e.g., set project id)
    virtual void setRelationship(int rootId, RootRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Root id (e.g., get project id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipMany(const QList<int> &rootIds,
                                                                     RootRelationshipField relationship) const = 0;
};

class RootRepository : public IRootRepository
{
  public:
    RootRepository(IRootTable &table, Database::DbSubContext &dbSubContext, QPointer<EventRegistry> eventRegistry);

    ~RootRepository() override = default;

    // CRUD
    QList<SCE::Root> create(const QList<SCE::Root> &roots) override;
    QList<SCE::Root> get(const QList<int> &rootIds) override;
    QList<SCE::Root> update(const QList<SCE::Root> &roots) override;
    QList<int> remove(const QList<int> &rootIds) override;

    // Relationships
    void setRelationship(int rootId, RootRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationship(int rootId, RootRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipMany(const QList<int> &rootIds, RootRelationshipField relationship) override;

  private:
    IRootTable &m_table;
    QPointer<RootEvents> m_events;           // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int rootId, RootRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::Root
