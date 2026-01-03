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

#include "save_work_uc.h"

namespace Skribisto::WorkManagement
{
SaveWorkUseCase::SaveWorkUseCase(std::unique_ptr<ISaveWorkUnitOfWork> uow) : m_uow(std::move(uow))
{
}
bool SaveWorkUseCase::execute(const SaveWorkDto &saveWorkDto) const
{
    // check if it is writable

    // save to file
    m_uow->beginTransaction();
    m_uow->saveDatabaseToFile(saveWorkDto.fileName);
    // m_uow->endTransaction();
    {
        // remove root and recent_project tables from the new file database
        QSqlDatabase savedDb = QSqlDatabase::addDatabase("QSQLITE"_L1, "cleanup_connection"_L1);
        savedDb.setDatabaseName(saveWorkDto.fileName);

        if (savedDb.open())
        {
            {
                QSqlQuery query(savedDb);

                // Drop root table and its junction tables
                query.exec("DROP TABLE IF EXISTS root;"_L1);
                query.exec("DROP TABLE IF EXISTS root_works_to_work_junction;"_L1);
                query.exec("DROP TABLE IF EXISTS root_recent_works_to_recent_work_junction;"_L1);

                // Drop recent_project table if it exists
                query.exec("DROP TABLE IF EXISTS recent_project;"_L1);
            }
            savedDb.close();
        }
    }
    QSqlDatabase::removeDatabase("cleanup_connection"_L1);

    return true;
}
} // namespace Skribisto::WorkManagement