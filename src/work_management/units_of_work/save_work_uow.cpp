#include "save_work_uow.h"

#include <QFile>
#include <QSqlDatabase>
#include <QSqlError>
#include <QSqlQuery>

using namespace Qt::StringLiterals;

namespace Skribisto::WorkManagement
{

bool SaveWorkUnitOfWork::saveDatabaseToFile(const QString &filePath)
{
    const QString internalDbPath = m_dbSubContext.getDatabaseName();

    // Checkpoint internal database to consolidate WAL data into the main file.
    // getConnection() returns an already-open connection; we don't close it.
    {
        QSqlDatabase internalDb = m_dbSubContext.getConnection();
        QSqlQuery query(internalDb);
        if (!query.exec(u"PRAGMA wal_checkpoint(TRUNCATE);"_s))
        {
            qWarning() << "Checkpoint failed during export:" << query.lastError();
            return false;
        }
    }

    // Copy to a temporary file first, clean it up, then move to the final path.
    // This avoids losing the user's file if the copy or cleanup fails.
    const QString tempPath = filePath + u".tmp"_s;
    QFile::remove(tempPath);

    if (!QFile::copy(internalDbPath, tempPath))
    {
        qWarning() << "Failed to copy database to" << tempPath;
        QFile::remove(tempPath);
        return false;
    }

    // Remove Root, System, RecentWork, TrashInfo tables from the saved file.
    // These are runtime-only data, not part of the project file.
    {
        QSqlDatabase savedDb = QSqlDatabase::addDatabase(u"QSQLITE"_s, u"save_cleanup_connection"_s);
        savedDb.setDatabaseName(tempPath);

        if (savedDb.open())
        {
            QSqlQuery query(savedDb);
            query.exec(u"DROP TABLE IF EXISTS root;"_s);
            query.exec(u"DROP TABLE IF EXISTS root_system_to_system_junction;"_s);
            query.exec(u"DROP TABLE IF EXISTS root_works_to_work_junction;"_s);
            query.exec(u"DROP TABLE IF EXISTS system;"_s);
            query.exec(u"DROP TABLE IF EXISTS system_recent_works_to_recent_work_junction;"_s);
            query.exec(u"DROP TABLE IF EXISTS system_trash_infos_to_trash_info_junction;"_s);
            query.exec(u"DROP TABLE IF EXISTS recent_work;"_s);
            query.exec(u"DROP TABLE IF EXISTS trash_info;"_s);
            query.exec(u"DROP TABLE IF EXISTS trash_info_trashed_binder_to_binder_junction;"_s);
            query.exec(u"DROP TABLE IF EXISTS trash_info_trashed_binder_item_to_binder_item_junction;"_s);

            // Stamp the file format version
            query.exec(u"ALTER TABLE work ADD COLUMN version INTEGER NOT NULL DEFAULT 3;"_s);

            query.exec(u"VACUUM;"_s);
            savedDb.close();
        }
        else
        {
            qWarning() << "Failed to open saved database for cleanup:" << savedDb.lastError();
            QSqlDatabase::removeDatabase(u"save_cleanup_connection"_s);
            QFile::remove(tempPath);
            return false;
        }
    }
    QSqlDatabase::removeDatabase(u"save_cleanup_connection"_s);

    // Atomically replace the target file
    QFile::remove(filePath);
    if (!QFile::rename(tempPath, filePath))
    {
        qWarning() << "Failed to move temp file to" << filePath;
        QFile::remove(tempPath);
        return false;
    }

    return true;
}

} // namespace Skribisto::WorkManagement
