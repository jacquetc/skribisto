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

#include "update_uc.h"

namespace Skribisto::DirectAccess::Project
{
namespace SCE = Common::Entities;

QList<ProjectDto> UpdateProjectUseCase::execute(const QList<ProjectDto> &projects)
{
    if (m_hasExecuted)
    {
        // If already executed, return cached results
        return m_updatedProjects;
    }

    // First, get the original data for undo/redo
    QList<int> projectIds;
    for (const auto &project : projects)
    {
        projectIds.append(project.id);
    }

    m_uow->beginTransaction();
    auto originalEntities = m_uow->getProject(projectIds);
    m_originalProjects = DtoMapper::toDtoList(originalEntities);

    // Perform the update
    auto mappedEntities = DtoMapper::toEntityList(projects);

    // Update the updatedAt timestamp
    QDateTime currentTime = QDateTime::currentDateTimeUtc();
    for (auto &entity : mappedEntities)
        entity.updatedAt = currentTime;

    auto updatedEntities = m_uow->updateProject(mappedEntities);
    m_uow->commit();

    m_updatedProjects = DtoMapper::toDtoList(updatedEntities);
    m_hasExecuted = true;

    return m_updatedProjects;
}

SCU::Result<void> UpdateProjectUseCase::undo()
{
    if (!m_hasExecuted || m_originalProjects.isEmpty())
    {
        return SCU::Result<void>("Cannot undo: no projects were updated"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Restore the original projects
        auto mappedEntities = DtoMapper::toEntityList(m_originalProjects);
        m_uow->updateProject(mappedEntities);
        m_uow->commit();

        return {};
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Undo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

SCU::Result<void> UpdateProjectUseCase::redo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot redo: execute() must be called first"_L1);
    }

    try
    {
        m_uow->beginTransaction();
        auto mappedEntities = DtoMapper::toEntityList(m_updatedProjects);
        m_uow->updateProject(mappedEntities);
        m_uow->commit();

        return {};
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::Project