/*
 * Copyright (C) 2025 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.  *
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
#include "../i_legacy_upgrader.h"
#include "skrresult.h"
#include "skrsqltools.h"
#include "upgrader.h"

#include <QString>

namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule
{

class LegacyUpgrader final : public ILegacyUpgrader
{
  public:
    bool upgradeSQLite(const QString &sqlDbConnectionName) override;
    bool isUpgradeNeeded(const QString &sqlDbConnectionName) override;
};

using Upgrader = Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule::Upgrader;
inline bool upgradeSQLite(const QString &sqlDbConnectionName)
{

    if (const SKRResult result = Upgrader::upgradeSQLite(sqlDbConnectionName); !result)
    {
        qCritical() << "Error while upgrading the database:" << result.getLastErrorCode();
        // make sure the result is propagated
        try
        {
            throw std::runtime_error(result.getLastErrorCode().toStdString());
        }
        catch (...)
        {
        }

        return false;
    }
    return true;
}

inline bool isUpgradeNeeded(const QString &sqlDbConnectionName)
{
    double dbVersion = SKRSqlTools::getProjectDBVersion(nullptr, sqlDbConnectionName);
    // the latest version is 2.0
    return dbVersion < 1.9;
}
} // namespace Skribisto::WorkManagement::LoadWorkUseCaseModule::LegacyUpgraderModule