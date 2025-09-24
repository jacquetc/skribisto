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

#include "common/dto_mapper.h"
#include "entities/recent_work.h"
#include "i_recent_work_unit_of_work.h"
#include "undo_redo/undo_redo_command.h"
#include <memory>

namespace Skribisto::DirectAccess::RecentWork
{
namespace SCE = Common::Entities;
namespace SCU = Common::UndoRedo;

class CreateRecentWorkUseCase
{
  public:
    explicit CreateRecentWorkUseCase(std::unique_ptr<IRecentWorkUnitOfWork> uow) : m_uow(std::move(uow))
    {
    }
    ~CreateRecentWorkUseCase() = default;

    QList<RecentWorkDto> execute(const QList<CreateRecentWorkDto> &recentWorks);
    SCU::Result<void> undo();
    SCU::Result<void> redo();

  private:
    std::unique_ptr<IRecentWorkUnitOfWork> m_uow;
    QList<CreateRecentWorkDto> m_originalRecentWorks;
    QList<RecentWorkDto> m_createdRecentWorks;
    bool m_hasExecuted = false;
};

} // namespace Skribisto::DirectAccess::RecentWork