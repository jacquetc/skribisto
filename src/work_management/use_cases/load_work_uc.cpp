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

#include "load_work_uc.h"

namespace Skribisto::WorkManagement
{
LoadWorkUseCase::LoadWorkUseCase(
    std::unique_ptr<ILoadWorkUnitOfWork> uow,
    std::unique_ptr<LoadWorkUseCaseModule::LegacyUpgraderModule::ILegacyUpgrader> legacyUpgrader)
    : m_uow(std::move(uow)), m_legacyUpgrader(std::move(legacyUpgrader))
{
}
bool LoadWorkUseCase::execute(const LoadWorkDto &loadWorkDto)
{
    // first check if the file exists and is readable
    QFileInfo fileInfo(loadWorkDto.fileName);
    if (!fileInfo.exists() || !fileInfo.isReadable())
        return false;

    // then check if the database needs to be upgraded with the legacy upgrader
    if (m_legacyUpgrader->isUpgradeNeeded(loadWorkDto.fileName))
    {
        // if upgrade is needed, do a backup of the file
        QFile::copy(loadWorkDto.fileName, loadWorkDto.fileName + ".bak"_L1);

        // then do the upgrade

        if (!m_legacyUpgrader->upgradeSQLite(loadWorkDto.fileName))
        {
            qCritical() << "Error while upgrading the database";

            // if upgrade fails, restore the backup and return false

            QFile::remove(loadWorkDto.fileName);
            QFile::rename(loadWorkDto.fileName + ".bak"_L1, loadWorkDto.fileName);
            return false;
        }
    }
    qInfo() << "Database upgraded successfully from legacy Skribisto project version.";

    qInfo() << "Database upgraded successfully to the latest version.";

    m_uow->publishWorkLoaded(0);
    return true;
}
} // namespace Skribisto::WorkManagement