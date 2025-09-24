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

#include "../dtos.h"
#include "common/dto_mapper.h"
#include "i_binder_unit_of_work.h"
#include <memory>

namespace Skribisto::DirectAccess::Binder
{

class GetRelationshipIdsInRangeUseCase
{
  public:
    explicit GetRelationshipIdsInRangeUseCase(std::unique_ptr<IBinderUnitOfWork> uow) : m_uow(std::move(uow))
    {
    }
    ~GetRelationshipIdsInRangeUseCase() = default;

    QList<int> execute(int binderId, BinderRelationshipField relationship, int offset, int limit);

  private:
    std::unique_ptr<IBinderUnitOfWork> m_uow;
};

} // namespace Skribisto::DirectAccess::Binder