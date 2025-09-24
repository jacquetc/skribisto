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
#include "direct_access/recent_work/i_recent_work_repository.h"
#include "direct_access/recent_work/recent_work_events.h"
#include "entities/recent_work.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::RecentWork
{
namespace SCE = Skribisto::Common::Entities;

class IRecentWorkTable
{
  public:
    virtual ~IRecentWorkTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::RecentWork> createMany(const QList<SCE::RecentWork> &recentWorks) = 0;

    // Update
    virtual QList<SCE::RecentWork> updateMany(const QList<SCE::RecentWork> &recentWorks) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::RecentWork> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given RecentWork id (e.g., set work id)
    virtual void setRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship,
                                    QList<int> relatedId) = 0;

    // Get the relationship value for a given RecentWork id (e.g., get work id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(
        const QList<int> &recentWorkIds, RecentWorkRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int recentWorkId, RecentWorkRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int recentWorkId, RecentWorkRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

class RecentWorkRepository : public IRecentWorkRepository
{
  public:
    RecentWorkRepository(std::unique_ptr<IRecentWorkTable> table, Database::DbSubContext &dbSubContext,
                         QPointer<EventRegistry> eventRegistry);

    ~RecentWorkRepository() override = default;

    // CRUD
    QList<SCE::RecentWork> create(const QList<SCE::RecentWork> &recentWorks) override;
    QList<SCE::RecentWork> get(const QList<int> &recentWorkIds) override;
    QList<SCE::RecentWork> update(const QList<SCE::RecentWork> &recentWorks) override;
    QList<int> remove(const QList<int> &recentWorkIds) override;

    // Relationships
    void setRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &recentWorkIds,
                                                  RecentWorkRelationshipField relationship) override;
    int getRelationshipIdsCount(int recentWorkId, RecentWorkRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int recentWorkId, RecentWorkRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IRecentWorkTable> m_table;
    QPointer<RecentWorkEvents> m_events;     // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int recentWorkId, RecentWorkRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::RecentWork
