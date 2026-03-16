/*
 * Copyright (C) 2025 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

#pragma once
#include <QString>

namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
{
class ILegacyUpgrader
{
  public:
    virtual ~ILegacyUpgrader() = default;
    virtual bool upgradeSQLite(const QString &filePath) = 0;
    virtual bool isUpgradeNeeded(const QString &filePath) = 0;
};
} // namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgrader