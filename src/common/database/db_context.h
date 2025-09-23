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

#include <QDebug>
#include <QDir>
#include <QHash>
#include <QPointer>
#include <QReadWriteLock>
#include <QString>
#include <QTemporaryFile>
#include <QWriteLocker>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

#include "database/db_builder.h"

namespace Skribisto::Common::Database
{

class DbContext : public QObject
{
    Q_OBJECT
  public:
    DbContext(QObject *parent = nullptr) : QObject(parent)
    {
        m_databaseName = DbBuilder::buildDatabase();
    }

    ~DbContext()
    {
        // Close all connections
        QWriteLocker guard(&m_lock);
        for (const QString &connName : m_connections)
        {
            QSqlDatabase::removeDatabase(connName);
        }
        m_connections.clear();

        // Delete the temporary database file
        if (!m_databaseName.isEmpty() && QFile::exists(m_databaseName))
        {
            if (!QFile::remove(m_databaseName))
            {
                qWarning() << "DbContext: failed to remove temporary database file:" << m_databaseName;
            }
        }
    }

    // Connection pool management
    int createDatabaseConnection()
    {
        QWriteLocker guard(&m_lock);
        const int id = ++m_nextId;
        const QString connName = QStringLiteral("skri_conn_%1").arg(id);
        QSqlDatabase db = QSqlDatabase::addDatabase("QSQLITE"_L1, connName);
        db.setDatabaseName(m_databaseName);
        if (!db.open())
        {
            qCritical() << "DbContext: failed to open database:" << db.lastError().text();
        }
        m_connections.insert(id, connName);
        return id;
    }

    void closeDatabaseConnection(const int dbId)
    {
        QWriteLocker guard(&m_lock);
        const auto it = m_connections.constFind(dbId);
        if (it == m_connections.cend())
        {
            qCritical() << "DbContext::closeDatabaseConnection - unknown dbId" << dbId;
            return;
        }

        QSqlDatabase::removeDatabase(it.value());
        m_connections.erase(it);
    }

    QSqlDatabase getConnection(const int dbId) const
    {
        QReadLocker guard(&m_lock);
        const auto it = m_connections.constFind(dbId);
        if (it == m_connections.cend())
        {
            qCritical() << "DbContext::getConnection - unknown dbId" << dbId;
            return {};
        }
        return QSqlDatabase::database(it.value());
    }

  private:
    mutable QReadWriteLock m_lock;
    QString m_databaseName;
    int m_nextId = 0;
    // map id -> connectionName
    QHash<int, QString> m_connections;
};

struct DbSubContext
{
    explicit DbSubContext(DbContext &parentDbContext) : m_parentDbContext(parentDbContext)
    {
        // Don't create connection here - defer until needed
    }

    DbSubContext(const DbSubContext &) = delete;
    DbSubContext &operator=(const DbSubContext &) = delete;

    ~DbSubContext()
    {
        // Only close if connection was created
        if (m_dbId != -1)
        {
            m_parentDbContext.closeDatabaseConnection(m_dbId);
        }
    }

    [[nodiscard]] QSqlDatabase getConnection()
    {
        // Lazy initialization - create connection on first use
        if (m_dbId == -1)
        {
            m_dbId = m_parentDbContext.createDatabaseConnection();
        }
        return m_parentDbContext.getConnection(m_dbId);
    }

    [[nodiscard]] int getDbId()
    {
        // Ensure connection exists before returning ID
        if (m_dbId == -1)
        {
            m_dbId = m_parentDbContext.createDatabaseConnection();
        }
        return m_dbId;
    }

    // Transaction API - all methods need to ensure connection exists
    void beginTransaction()
    {
        QSqlDatabase db = getConnection(); // This will create connection if needed
        if (!db.isValid())
            return;
        if (!db.transaction())
        {
            qWarning() << "BEGIN TRANSACTION failed:" << db.lastError().text();
        }
    }

    void commit()
    {
        QSqlDatabase db = getConnection();
        if (!db.isValid())
            return;
        if (!db.commit())
        {
            qWarning() << "COMMIT failed:" << db.lastError().text();
        }
    }

    void endTransaction()
    {
        commit();
    }

    void rollback()
    {
        QSqlDatabase db = getConnection();
        if (!db.isValid())
            return;
        if (!db.rollback())
        {
            qWarning() << "ROLLBACK failed:" << db.lastError().text();
        }
    }

    void createSavepoint()
    {
        if (!m_savepointName.isEmpty())
        {
            qCritical() << "RootUnitOfWork::createSavepoint - savepoint already exists";
            return;
        }

        QSqlDatabase db = getConnection();
        if (!db.isValid())
            return;
        QSqlQuery q(db);
        const QString name =
            QStringLiteral("sp_%1").arg(QUuid::createUuid().toString(QUuid::WithoutBraces).remove("-"_L1));

        if (!q.exec(QStringLiteral("SAVEPOINT %1").arg(name)))
        {
            qWarning() << "SAVEPOINT failed:" << q.lastError().text();
        }
        m_savepointName = name;
    }

    void rollbackToSavepoint()
    {
        if (m_savepointName.isEmpty())
        {
            qCritical() << "RootUnitOfWork::rollbackToSavepoint - no savepoint";
            return;
        }

        QSqlDatabase db = getConnection();
        if (!db.isValid())
            return;
        QSqlQuery q(db);
        if (!q.exec(QStringLiteral("ROLLBACK TO SAVEPOINT %1").arg(m_savepointName)))
        {
            qWarning() << "ROLLBACK TO SAVEPOINT failed:" << q.lastError().text();
        }
    }

    void releaseSavepoint()
    {
        if (m_savepointName.isEmpty())
        {
            qCritical() << "RootUnitOfWork::releaseSavepoint - no savepoint";
            return;
        }

        QSqlDatabase db = getConnection();
        if (!db.isValid())
            return;
        QSqlQuery q(db);
        if (!q.exec(QStringLiteral("RELEASE SAVEPOINT %1").arg(m_savepointName)))
        {
            qWarning() << "RELEASE SAVEPOINT failed:" << q.lastError().text();
        }
    }

  private:
    DbContext &m_parentDbContext;
    int m_dbId = -1; // Initialize to invalid ID
    QString m_savepointName;
};

} // namespace Skribisto::Common::Database
