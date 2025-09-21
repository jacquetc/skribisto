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
#include "direct_access/project/i_project_repository.h"
#include "direct_access/project/project_events.h"
#include "entities/project.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::Project
{
namespace SCE = Skribisto::Common::Entities;

class IProjectTable
{
  public:
    virtual ~IProjectTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::Project> createMany(const QList<SCE::Project> &projects) = 0;

    // Update
    virtual QList<SCE::Project> updateMany(const QList<SCE::Project> &projects) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::Project> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Project id (e.g., set project id)
    virtual void setRelationship(int projectId, ProjectRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Project id (e.g., get project id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipMany(const QList<int> &projectIds,
                                                                     ProjectRelationshipField relationship) const = 0;
};

class ProjectRepository : public IProjectRepository
{
  public:
    // Original constructor for backward compatibility
    ProjectRepository(IProjectTable &table, Database::DbSubContext &dbSubContext,
                      QPointer<EventRegistry> eventRegistry);

    // New constructor that accepts EventRegistry for cascade operations
    ProjectRepository(IProjectTable &table, QPointer<ProjectEvents> events, Database::DbContext &db, int dbId,
                      const EventRegistry *eventRegistry = nullptr);

    ~ProjectRepository() override = default;

    // CRUD
    QList<SCE::Project> create(const QList<SCE::Project> &projects) override;
    QList<SCE::Project> get(const QList<int> &projectIds) override;
    QList<SCE::Project> update(const QList<SCE::Project> &projects) override;
    QList<int> remove(const QList<int> &projectIds) override;

    // Relationships
    void setRelationship(int projectId, ProjectRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationship(int projectId, ProjectRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipMany(const QList<int> &projectIds,
                                               ProjectRelationshipField relationship) override;

  private:
    IProjectTable &m_table;
    QPointer<ProjectEvents> m_events;        // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int projectId, ProjectRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::Project
