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

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCE = Common::Entities;

QList<int> RemoveBinderItemUseCase::execute(const QList<int> &binderItemIds)
{
    if (m_hasExecuted)
    {
        // If already executed, return cached results
        return m_removedIds;
    }

    // Store original data for undo/redo
    m_originalBinderItemIds = binderItemIds;

    m_uow->beginTransaction();
    // Get the binderItem data before removing for potential undo
    auto binderItemEntities = m_uow->getBinderItem(binderItemIds);
    m_removedBinderItems = DtoMapper::toDtoList(binderItemEntities);

    // Create savepoint before removal
    m_uow->createSavepoint();

    // Perform the removal
    m_removedIds = m_uow->removeBinderItem(binderItemIds);
    m_uow->commit();

    m_hasExecuted = true;

    return m_removedIds;
}

SCU::Result<void> RemoveBinderItemUseCase::undo()
{
    if (!m_hasExecuted || m_removedBinderItems.isEmpty())
    {
        return SCU::Result<void>("Cannot undo: no binderItems were removed"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Use rollback to savepoint to restore the removed binderItems
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

SCU::Result<void> RemoveBinderItemUseCase::redo()
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
        m_uow->removeBinderItem(m_originalBinderItemIds);
        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::BinderItem