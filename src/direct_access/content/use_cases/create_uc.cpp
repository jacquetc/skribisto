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

namespace Skribisto::DirectAccess::Content
{
namespace SCE = Common::Entities;

QList<ContentDto> CreateContentUseCase::execute(const QList<CreateContentDto> &contents)
{
    if (m_hasExecuted)
    {
        // If already executed, return cached results
        return m_createdContents;
    }

    // Store original data for undo/redo
    m_originalContents = contents;

    m_uow->beginTransaction();
    auto mappedEntities = DtoMapper::toEntityList(contents);

    // set dates:
    QDateTime currentTime = QDateTime::currentDateTimeUtc();
    for (auto &entity : mappedEntities)
    {
        entity.createdAt = currentTime;
        entity.updatedAt = entity.createdAt;
    }

    auto createdEntities = m_uow->createContent(mappedEntities);
    m_uow->commit();

    m_createdContents = DtoMapper::toDtoList(createdEntities);
    m_hasExecuted = true;

    return m_createdContents;
}

SCU::Result<void> CreateContentUseCase::undo()
{
    if (!m_hasExecuted || m_createdContents.isEmpty())
    {
        return SCU::Result<void>("Cannot undo: no contents were created"_L1);
    }

    try
    {
        m_uow->beginTransaction();

        // Remove the created contents by their IDs
        QList<int> idsToRemove;
        for (const auto &content : m_createdContents)
        {
            idsToRemove.append(content.id);
        }

        // Use the unit of work to remove the created contents
        m_uow->removeContent(idsToRemove);
        m_uow->commit();

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Undo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

SCU::Result<void> CreateContentUseCase::redo()
{
    if (!m_hasExecuted)
    {
        return SCU::Result<void>("Cannot redo: execute() must be called first"_L1);
    }

    try
    {
        m_uow->beginTransaction();
        auto mappedEntities = DtoMapper::toEntityList(m_originalContents);
        auto createdEntities = m_uow->createContent(mappedEntities);
        m_uow->commit();

        m_createdContents = DtoMapper::toDtoList(createdEntities);

        return SCU::Result<void>();
    }
    catch (const std::exception &e)
    {
        m_uow->rollback();
        return SCU::Result<void>("Redo failed: "_L1 + QString::fromStdString(e.what()));
    }
}

} // namespace Skribisto::DirectAccess::Content
