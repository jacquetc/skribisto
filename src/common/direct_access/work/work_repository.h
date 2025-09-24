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
#include "direct_access/work/i_work_repository.h"
#include "direct_access/work/work_events.h"
#include "entities/work.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::Work
{
namespace SCE = Skribisto::Common::Entities;

class IWorkTable
{
  public:
    virtual ~IWorkTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::Work> createMany(const QList<SCE::Work> &works) = 0;

    // Update
    virtual QList<SCE::Work> updateMany(const QList<SCE::Work> &works) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::Work> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Work id (e.g., set work id)
    virtual void setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Work id (e.g., get work id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &workIds,
                                                                        WorkRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int rootId, WorkRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int rootId, WorkRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

class WorkRepository : public IWorkRepository
{
  public:
    // Original constructor for backward compatibility
    WorkRepository(std::unique_ptr<IWorkTable> table, Database::DbSubContext &dbSubContext,
                   QPointer<EventRegistry> eventRegistry);

    ~WorkRepository() override = default;

    // CRUD
    QList<SCE::Work> create(const QList<SCE::Work> &works) override;
    QList<SCE::Work> get(const QList<int> &workIds) override;
    QList<SCE::Work> update(const QList<SCE::Work> &works) override;
    QList<int> remove(const QList<int> &workIds) override;

    // Relationships
    void setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationshipIds(int workId, WorkRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &workIds,
                                                  WorkRelationshipField relationship) override;
    int getRelationshipIdsCount(int rootId, WorkRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int rootId, WorkRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IWorkTable> m_table;
    QPointer<WorkEvents> m_events;           // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int workId, WorkRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::Work
