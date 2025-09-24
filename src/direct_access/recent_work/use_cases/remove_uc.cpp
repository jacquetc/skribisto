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

#include "remove_uc.h"

namespace Skribisto::DirectAccess::RecentWork
{
namespace SCE = Common::Entities;

QList<int> RemoveRecentWorkUseCase::execute(const QList<int> &recentWorkIds)
{
    if (m_hasExecuted)
    {
        // If already executed, return cached results
        return m_removedIds;
    }

    // Store original data for undo/redo
    m_originalRecentWorkIds = recentWorkIds;

    m_uow->beginTransaction();
    // Get the recentWork data before removing for potential undo
    auto recentWorkEntities = m_uow->getRecentWork(recentWorkIds);
    m_removedRecentWorks = DtoMapper::toDtoList(recentWorkEntities);

    // Create savepoint before removal
    m_uow->createSavepoint();

    // Perform the removal
    m_removedIds = m_uow->removeRecentWork(recentWorkIds);
    m_uow->commit();

    m_hasExecuted = true;

    return m_removedIds;
}

SCU::Result<void> RemoveRecentWorkUseCase::undo()
{
    if (!m_hasExecuted || m_removedRecentWorks.isEmpty())
    {
        return SCU::Result<void>("Cannot undo: no recentWorks were removed"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Use rollback to savepoint to restore the removed recentWorks
        m_uow->rollbackToSavepoint();
        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Undo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

SCU::Result<void> RemoveRecentWorkUseCase::redo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot redo: execute() must be called first"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Release the savepoint and perform removal again
        m_uow->releaseSavepoint();
        m_uow->removeRecentWork(m_originalRecentWorkIds);
        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::RecentWork