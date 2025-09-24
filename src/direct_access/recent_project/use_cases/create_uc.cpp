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

#include "create_uc.h"

namespace Skribisto::DirectAccess::RecentProject
{
namespace SCE = Common::Entities;

QList<RecentProjectDto> CreateRecentProjectUseCase::execute(const QList<CreateRecentProjectDto> &recentProjects)
{
    if (m_hasExecuted)
    {
        // If already executed, return cached results
        return m_createdRecentProjects;
    }

    // Store original data for undo/redo
    m_originalRecentProjects = recentProjects;

    m_uow->beginTransaction();
    auto mappedEntities = DtoMapper::toEntityList(recentProjects);

    // set dates:
    QDateTime currentTime = QDateTime::currentDateTimeUtc();
    for (auto &entity : mappedEntities)
    {
        entity.createdAt = currentTime;
        entity.updatedAt = entity.createdAt;
    }

    auto createdEntities = m_uow->createRecentProject(mappedEntities);
    m_uow->commit();

    m_createdRecentProjects = DtoMapper::toDtoList(createdEntities);
    m_hasExecuted = true;

    return m_createdRecentProjects;
}

SCU::Result<void> CreateRecentProjectUseCase::undo()
{
    if (!m_hasExecuted || m_createdRecentProjects.isEmpty())
    {
        return SCU::Result<void>("Cannot undo: no recentProjects were created"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Remove the created recentProjects by their IDs
        QList<int> idsToRemove;
        for (const auto &recentProject : m_createdRecentProjects)
        {
            idsToRemove.append(recentProject.id);
        }

        // Use the unit of work to remove the created recentProjects
        m_uow->removeRecentProject(idsToRemove);
        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Undo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

SCU::Result<void> CreateRecentProjectUseCase::redo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot redo: execute() must be called first"_L1);
    }

    try
    {
        m_uow->beginTransaction();
        auto mappedEntities = DtoMapper::toEntityList(m_originalRecentProjects);
        auto createdEntities = m_uow->createRecentProject(mappedEntities);
        m_uow->commit();

        m_createdRecentProjects = DtoMapper::toDtoList(createdEntities);

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::RecentProject
