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

namespace Skribisto::DirectAccess::Binder
{
namespace SCE = Common::Entities;

QList<BinderDto> UpdateBinderUseCase::execute(const QList<BinderDto> &binders)
{
    if (m_hasExecuted)
    {
        // If already executed, return cached results
        return m_updatedBinders;
    }

    // First, get the original data for undo/redo
    QList<int> binderIds;
    for (const auto &binder : binders)
    {
        binderIds.append(binder.id);
    }

    m_uow->beginTransaction();
    auto originalEntities = m_uow->getBinder(binderIds);
    m_originalBinders = DtoMapper::toDtoList(originalEntities);

    // Perform the update
    auto mappedEntities = DtoMapper::toEntityList(binders);

    // Update the updatedAt timestamp
    QDateTime currentTime = QDateTime::currentDateTimeUtc();
    for (auto &entity : mappedEntities)
        entity.updatedAt = currentTime;

    auto updatedEntities = m_uow->updateBinder(mappedEntities);
    m_uow->commit();

    m_updatedBinders = DtoMapper::toDtoList(updatedEntities);
    m_hasExecuted = true;

    return m_updatedBinders;
}

SCU::Result<void> UpdateBinderUseCase::undo()
{
    if (!m_hasExecuted || m_originalBinders.isEmpty())
    {
        return SCU::Result<void>("Cannot undo: no binders were updated"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Restore the original binders
        auto mappedEntities = DtoMapper::toEntityList(m_originalBinders);
        m_uow->updateBinder(mappedEntities);
        m_uow->commit();

        return {};
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Undo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

SCU::Result<void> UpdateBinderUseCase::redo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot redo: execute() must be called first"_L1);
    }

    try
    {
        m_uow->beginTransaction();
        auto mappedEntities = DtoMapper::toEntityList(m_updatedBinders);
        m_uow->updateBinder(mappedEntities);
        m_uow->commit();

        return {};
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::Binder