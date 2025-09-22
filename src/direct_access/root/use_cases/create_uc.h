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
#include "entities/root.h"
#include "i_root_unit_of_work.h"
#include "undo_redo/undo_redo_command.h"
#include <memory>

namespace Skribisto::DirectAccess::Root
{
namespace SCE = Common::Entities;
namespace SCU = Common::UndoRedo;

class CreateRootUseCase
{
  public:
    explicit CreateRootUseCase(std::unique_ptr<IRootUnitOfWork> uow) : m_uow(std::move(uow))
    {
    }
    ~CreateRootUseCase() = default;

    QList<RootDto> execute(const QList<CreateRootDto> &roots);
    SCU::Result<void> undo();
    SCU::Result<void> redo();

  private:
    std::unique_ptr<IRootUnitOfWork> m_uow;
    QList<CreateRootDto> m_originalRoots;
    QList<RootDto> m_createdRoots;
    bool m_hasExecuted = false;
};

} // namespace Skribisto::DirectAccess::Root