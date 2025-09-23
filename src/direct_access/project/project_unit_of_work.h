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
#include "direct_access/project/project_events.h"
#include "use_cases/i_project_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::Project
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDProject = Skribisto::Common::DirectAccess::Project;
namespace SCD = Skribisto::Common::DirectAccess;

class ProjectUnitOfWork final : public IProjectUnitOfWork
{

  public:
    ProjectUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~ProjectUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::Project> createProject(QList<SCE::Project> projects) override;
    QList<SCE::Project> getProject(QList<int> projectIds) override;
    QList<SCE::Project> updateProject(QList<SCE::Project> projects) override;
    QList<int> removeProject(QList<int> projectIds) override;
    QList<int> getProjectRelationship(int projectId, SCDProject::ProjectRelationshipField relationship) override;
    void setProjectRelationship(int projectId, SCDProject::ProjectRelationshipField relationship,
                                QList<int> relatedIds) override;
    QHash<int, QList<int>> getProjectRelationshipMany(const QList<int> &projectIds,
                                                      SCDProject::ProjectRelationshipField relationship) override;
    int getProjectRelationshipCount(int projectId, SCDProject::ProjectRelationshipField relationship) override;
    QList<int> getProjectRelationshipInRange(int projectId, SCDProject::ProjectRelationshipField relationship,
                                             int offset, int limit) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::Project