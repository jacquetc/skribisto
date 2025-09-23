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
#include "direct_access/recent_project/i_recent_project_repository.h"
#include "direct_access/recent_project/recent_project_events.h"
#include "entities/recent_project.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::RecentProject
{
namespace SCE = Skribisto::Common::Entities;

class IRecentProjectTable
{
  public:
    virtual ~IRecentProjectTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::RecentProject> createMany(const QList<SCE::RecentProject> &recentProjects) = 0;

    // Update
    virtual QList<SCE::RecentProject> updateMany(const QList<SCE::RecentProject> &recentProjects) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::RecentProject> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given RecentProject id (e.g., set project id)
    virtual void setRelationshipIds(int recentProjectId, RecentProjectRelationshipField relationship,
                                    QList<int> relatedId) = 0;

    // Get the relationship value for a given RecentProject id (e.g., get project id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(
        const QList<int> &recentProjectIds, RecentProjectRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int recentProjectId, RecentProjectRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int recentProjectId, RecentProjectRelationshipField relationship,
                                                 int offset, int limit) = 0;
};

class RecentProjectRepository : public IRecentProjectRepository
{
  public:
    RecentProjectRepository(std::unique_ptr<IRecentProjectTable> table, Database::DbSubContext &dbSubContext,
                            QPointer<EventRegistry> eventRegistry);

    ~RecentProjectRepository() override = default;

    // CRUD
    QList<SCE::RecentProject> create(const QList<SCE::RecentProject> &recentProjects) override;
    QList<SCE::RecentProject> get(const QList<int> &recentProjectIds) override;
    QList<SCE::RecentProject> update(const QList<SCE::RecentProject> &recentProjects) override;
    QList<int> remove(const QList<int> &recentProjectIds) override;

    // Relationships
    void setRelationshipIds(int recentProjectId, RecentProjectRelationshipField relationship,
                            QList<int> relatedId) override;
    QList<int> getRelationshipIds(int recentProjectId, RecentProjectRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &recentProjectIds,
                                                  RecentProjectRelationshipField relationship) override;
    int getRelationshipIdsCount(int recentProjectId, RecentProjectRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int recentProjectId, RecentProjectRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IRecentProjectTable> m_table;
    QPointer<RecentProjectEvents> m_events;  // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int recentProjectId, RecentProjectRelationshipField rel,
                                 const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::RecentProject
