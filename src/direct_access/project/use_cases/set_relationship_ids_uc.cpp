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

#include "set_relationship_ids_uc.h"

namespace Skribisto::DirectAccess::Project
{
namespace SCE = Common::Entities;

void SetRelationshipIdsUseCase::execute(int projectId, ProjectRelationshipField relationship,
                                        const QList<int> &relatedIds)
{
    if (m_hasExecuted)
    {
        // If already executed, don't execute again
        return;
    }

    // Store parameters for undo/redo
    m_projectId = projectId;
    m_relationship = relationship;
    m_newRelatedIds = relatedIds;

    m_uow->beginTransaction();

    // Get original relationship IDs for undo functionality
    m_originalRelatedIds = m_uow->getProjectRelationship(projectId, DtoMapper::toCommonRelationshipField(relationship));

    // Set the new relationship IDs
    m_uow->setProjectRelationship(projectId, DtoMapper::toCommonRelationshipField(relationship), relatedIds);

    // update date

    auto project = m_uow->getProject({projectId}).at(0); // we are sure that it exists
    project.updatedAt = QDateTime::currentDateTimeUtc();
    m_uow->updateProject({project});

    m_uow->commit();

    m_hasExecuted = true;
}

SCU::Result<void> SetRelationshipIdsUseCase::undo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot undo: no relationship was set"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Restore original relationship IDs
        m_uow->setProjectRelationship(m_projectId, DtoMapper::toCommonRelationshipField(m_relationship),
                                      m_originalRelatedIds);

        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Undo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

SCU::Result<void> SetRelationshipIdsUseCase::redo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot redo: execute() must be called first"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Re-apply the new relationship IDs
        m_uow->setProjectRelationship(m_projectId, DtoMapper::toCommonRelationshipField(m_relationship),
                                      m_newRelatedIds);

        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::Project