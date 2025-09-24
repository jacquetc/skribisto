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
#include "entities/binder_item.h"
#include "i_binder_item_unit_of_work.h"
#include "undo_redo/undo_redo_command.h"
#include <memory>

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCE = Common::Entities;
namespace SCU = Common::UndoRedo;

class RemoveBinderItemUseCase
{
  public:
    explicit RemoveBinderItemUseCase(std::unique_ptr<IBinderItemUnitOfWork> uow) : m_uow(std::move(uow))
    {
    }
    ~RemoveBinderItemUseCase() = default;

    QList<int> execute(const QList<int> &binderItemIds);
    SCU::Result<void> undo();
    SCU::Result<void> redo();

  private:
    std::unique_ptr<IBinderItemUnitOfWork> m_uow;
    QList<int> m_originalBinderItemIds;
    QList<BinderItemDto> m_removedBinderItems;
    QList<int> m_removedIds;
    bool m_hasExecuted = false;
};

} // namespace Skribisto::DirectAccess::BinderItem