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
#include "direct_access/project/project_repository.h"
#include "dtos.h"
#include <QCoro/QCoroTask>

#include <QPointer>

namespace Skribisto::Common::UndoRedo
{
class UndoRedoSystem;
}

namespace Skribisto::DirectAccess::Project
{
namespace SCDatabase = Skribisto::Common::Database;

class ProjectController : public QObject
{
    Q_OBJECT
  public:
    ProjectController(const ProjectController &) = delete;
    ProjectController &operator=(const ProjectController &) = delete;
    ProjectController(ProjectController &&) = delete;
    ProjectController &operator=(ProjectController &&) = delete;
    explicit ProjectController(QObject *parent = nullptr);
    QCoro::Task<QList<ProjectDto>> create(const QList<CreateProjectDto> &projects);
    static CreateProjectDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<ProjectDto>> get(const QList<int> &projectIds);
    QCoro::Task<QList<ProjectDto>> update(const QList<ProjectDto> &projects);
    QCoro::Task<QList<int>> remove(const QList<int> &projectIds);
    QCoro::Task<QList<int>> getRelationshipIds(int projectId, ProjectRelationshipField relationship);
    QCoro::Task<void> setRelationshipIds(int projectId, ProjectRelationshipField relationship, QList<int> relatedIds);
    QCoro::Task<QHash<int, QList<int>>> getRelationshipIdsMany(const QList<int> &projectIds,
                                                               ProjectRelationshipField relationship);
    QCoro::Task<int> getRelationshipIdsCount(int projectId, ProjectRelationshipField relationship);
    QCoro::Task<QList<int>> getRelationshipIdsInRange(int projectId, ProjectRelationshipField relationship, int offset,
                                                      int limit);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::Project
