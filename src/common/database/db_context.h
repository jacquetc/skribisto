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

class DbContext
{
  public:
    DbContext()
    {
        m_databaseName = DbBuilder::buildDatabase();
    }
    ~DbContext() = default;

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

    void closeDatabaseConnection(const int dbId) const
    {
        QWriteLocker guard(&m_lock);
        const auto it = m_connections.constFind(dbId);
        if (it == m_connections.cend())
        {
            qCritical() << "DbContext::closeDatabaseConnection - unknown dbId" << dbId;
            return;
        }
        QSqlDatabase db = QSqlDatabase::database(it.value());
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
        m_dbId = m_parentDbContext.createDatabaseConnection();
    }
    DbSubContext(const DbSubContext &) = delete;
    DbSubContext &operator=(const DbSubContext &) = delete;

    ~DbSubContext()
    {
        m_parentDbContext.closeDatabaseConnection(m_dbId);
    }

    [[nodiscard]] QSqlDatabase getConnection() const
    {
        return m_parentDbContext.getConnection(m_dbId);
    }

    [[nodiscard]] int getDbId() const
    {
        return m_dbId;
    }

    // Transaction API
    void beginTransaction() const
    {
        QSqlDatabase db = m_parentDbContext.getConnection(m_dbId);
        if (!db.isValid())
            return;
        if (!db.transaction())
        {
            qWarning() << "BEGIN TRANSACTION failed:" << db.lastError().text();
        }
    }

    void commit() const
    {
        QSqlDatabase db = m_parentDbContext.getConnection(m_dbId);
        if (!db.isValid())
            return;
        if (!db.commit())
        {
            qWarning() << "COMMIT failed:" << db.lastError().text();
        }
    }

    void endTransaction() const
    {
        // In SQLite, END TRANSACTION is equivalent to COMMIT.
        commit();
    }

    void rollback() const
    {
        QSqlDatabase db = m_parentDbContext.getConnection(m_dbId);
        if (!db.isValid())
            return;
        if (!db.rollback())
        {
            qWarning() << "ROLLBACK failed:" << db.lastError().text();
        }
    }

    void createSavepoint()
    {
        if (not m_savepointName.isEmpty())
        {
            qCritical() << "RootUnitOfWork::createSavepoint - savepoint already exists";
            return;
        }

        QSqlDatabase db = m_parentDbContext.getConnection(m_dbId);
        if (!db.isValid())
            return;
        QSqlQuery q(db);
        const QString name = QStringLiteral("sp_%1").arg(QUuid::createUuid().toString(QUuid::WithoutBraces));

        if (!q.exec(QStringLiteral("SAVEPOINT %1").arg(name)))
        {
            qWarning() << "SAVEPOINT failed:" << q.lastError().text();
        }
        m_savepointName = name;
    }

    void rollbackToSavepoint() const
    {
        if (m_savepointName.isEmpty())
        {
            qCritical() << "RootUnitOfWork::rollbackToSavepoint - no savepoint";
            return;
        }

        QSqlDatabase db = m_parentDbContext.getConnection(m_dbId);
        if (!db.isValid())
            return;
        QSqlQuery q(db);
        if (!q.exec(QStringLiteral("ROLLBACK TO SAVEPOINT %1").arg(m_savepointName)))
        {
            qWarning() << "ROLLBACK TO SAVEPOINT failed:" << q.lastError().text();
        }
    }

    void releaseSavepoint() const
    {
        if (m_savepointName.isEmpty())
        {
            qCritical() << "RootUnitOfWork::releaseSavepoint - no savepoint";
            return;
        }

        QSqlDatabase db = m_parentDbContext.getConnection(m_dbId);
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
    int m_dbId;
    QString m_savepointName;
};

} // namespace Skribisto::Common::Database
