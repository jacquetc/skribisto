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
#include "direct_access/project/i_project_repository.h"
#include "entities/project.h"

#include <QString>

namespace Skribisto::DirectAccess::Project
{
namespace SCE = Common::Entities;
namespace SCDProject = Common::DirectAccess::Project;

class IProjectUnitOfWork
{
  public:
    virtual ~IProjectUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::Project> createProject(QList<SCE::Project> projects) = 0;
    virtual QList<SCE::Project> getProject(QList<int> projectIds) = 0;
    virtual QList<SCE::Project> updateProject(QList<SCE::Project> projects) = 0;
    virtual QList<int> removeProject(QList<int> projectIds) = 0;
    virtual QList<int> getProjectRelationship(int projectId, SCDProject::ProjectRelationshipField relationship) = 0;
    virtual void setProjectRelationship(int projectId, SCDProject::ProjectRelationshipField relationship,
                                        QList<int> relatedIds) = 0;
    virtual QHash<int, QList<int>> getProjectRelationshipMany(const QList<int> &projectIds,
                                                              SCDProject::ProjectRelationshipField relationship) = 0;
    virtual int getProjectRelationshipCount(int projectId, SCDProject::ProjectRelationshipField relationship) = 0;
    virtual QList<int> getProjectRelationshipInRange(int projectId, SCDProject::ProjectRelationshipField relationship,
                                                     int offset, int limit) = 0;
};
} // namespace Skribisto::DirectAccess::Project