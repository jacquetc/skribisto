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

#include "save_work_uow.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"
namespace Skribisto::WorkManagement
{
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCE = Common::Entities;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;

SaveWorkUnitOfWork::SaveWorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SaveWorkUnitOfWork::~SaveWorkUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SaveWorkUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SaveWorkUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SaveWorkUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SaveWorkUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SaveWorkUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SaveWorkUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SaveWorkUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
bool SaveWorkUnitOfWork::saveDatabaseToFile(const QString &filePath)
{
    QSqlDatabase internalDb = m_dbSubContext.getConnection();
    const QString internalDbPath = m_dbSubContext.getDatabaseName();

    // Checkpoint internal database to consolidate WAL data

    if (internalDb.open())
    {
        QSqlQuery query(internalDb);
        // Consolidate all WAL data into main database file
        if (!query.exec("PRAGMA wal_checkpoint(TRUNCATE);"_L1))
        {
            qWarning() << "Checkpoint failed during export:" << query.lastError();
            return false;
        }

        // Now internal database file contains all data in single file
        internalDb.close();
    }
    else
    {
        qWarning() << "Failed to open internal database for checkpoint:" << internalDb.lastError();
        return false;
    }

    QFile::remove(filePath); // Remove existing file if any

    // Copy/export the consolidated database to user's project file
    return QFile::copy(internalDbPath, filePath);
}
} // namespace Skribisto::WorkManagement