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
#include "direct_access/root/i_root_repository.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "entities/binder_tag.h"
#include "entities/content.h"
#include "entities/recent_work.h"
#include "entities/root.h"
#include "entities/work.h"
#include "load_work_uc/i_legacy_upgrader.h"
#include "load_work_uc/i_load_work_uow.h"
#include "work_management_dtos.h"
#include <QList>
#include <memory>

namespace Skribisto::WorkManagement
{
namespace LegacyUpgrader = Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule;

class LoadWorkUseCase
{
  public:
    LoadWorkUseCase(std::unique_ptr<ILoadWorkUnitOfWork> uow,
                    std::unique_ptr<LegacyUpgrader::ILegacyUpgrader> legacyUpgrader);
    bool execute(const LoadWorkDto &loadWorkDto) const;

  private:
    std::unique_ptr<ILoadWorkUnitOfWork> m_uow;
    std::unique_ptr<LegacyUpgrader::ILegacyUpgrader> m_legacyUpgrader;
};

} // namespace Skribisto::WorkManagement
