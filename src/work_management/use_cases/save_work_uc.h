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
#include "database/db_context.h"
#include "i_save_work_uow.h"
#include "work_management_dtos.h"
#include <QList>
#include <memory>

namespace Skribisto::WorkManagement
{

class SaveWorkUseCase
{
  public:
    SaveWorkUseCase(std::unique_ptr<ISaveWorkUnitOfWork> uow);
    bool execute(const SaveWorkDto &saveWorkDto) const;

  private:
    std::unique_ptr<ISaveWorkUnitOfWork> m_uow;
};

} // namespace Skribisto::WorkManagement
