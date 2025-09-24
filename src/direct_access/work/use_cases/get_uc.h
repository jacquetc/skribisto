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
#include "entities/work.h"
#include "i_work_unit_of_work.h"
#include <memory>

namespace Skribisto::DirectAccess::Work
{
namespace SCE = Common::Entities;

class GetWorkUseCase
{
  public:
    explicit GetWorkUseCase(std::unique_ptr<IWorkUnitOfWork> uow) : m_uow(std::move(uow))
    {
    }
    ~GetWorkUseCase() = default;

    QList<WorkDto> execute(const QList<int> &workIds);

  private:
    std::unique_ptr<IWorkUnitOfWork> m_uow;
};

} // namespace Skribisto::DirectAccess::Work