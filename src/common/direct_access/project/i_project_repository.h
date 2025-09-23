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

#include "entities/project.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::Project
{

// Relationships for Project entity derived from its relational fields
// Currently, Project has a single relationship field: `projects`
enum class ProjectRelationshipField
{
    Binders,
};

class IProjectRepository
{
  public:
    virtual ~IProjectRepository() = default;

    // CRUD
    virtual QList<Entities::Project> create(const QList<Entities::Project> &projects) = 0;
    virtual QList<Entities::Project> get(const QList<int> &projectIds) = 0;
    virtual QList<Entities::Project> update(const QList<Entities::Project> &projects) = 0;
    virtual QList<int> remove(const QList<int> &projectIds) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Project id (e.g., set project id)
    virtual void setRelationshipIds(int projectId, ProjectRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Project id (e.g., get project id)
    virtual QList<int> getRelationshipIds(int projectId, ProjectRelationshipField relationship) = 0;
    virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &projectIds,
                                                          ProjectRelationshipField relationship) = 0;
    virtual int getRelationshipIdsCount(int rootId, ProjectRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int rootId, ProjectRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

} // namespace Skribisto::Common::DirectAccess::Project
