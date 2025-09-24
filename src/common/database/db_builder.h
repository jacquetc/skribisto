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
#include "direct_access/binder/table_definitions.h"
#include "direct_access/work/table_definitions.h"

#include <QDir>
#include <QString>
#include <QUuid>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

#include "direct_access/root/table_definitions.h"

namespace Skribisto::Common::Database
{
using namespace Qt::StringLiterals;
class DbBuilder
{
  public:
    // Creates an SQLite database file in the temp folder and builds schema.
    // Returns the absolute file path used as databaseName for QSqlDatabase.
    static QString buildDatabase()
    {
        // Create a unique database file path in the temp dir
        const QString dbPath =
            QDir::tempPath() + QDir::separator() +
            QStringLiteral("skribisto_%1.sqlite").arg(QUuid::createUuid().toString(QUuid::WithoutBraces));

        // Use a temporary connection just to build the schema
        const QString builderConn =
            QStringLiteral("skri_builder_%1").arg(QUuid::createUuid().toString(QUuid::WithoutBraces));
        QSqlDatabase db = QSqlDatabase::addDatabase("QSQLITE"_L1, builderConn);
        db.setDatabaseName(dbPath);
        if (!db.open())
        {
            // Return path even if open fails; callers will report errors when opening
            return dbPath;
        }

        // Build entity tables first
        {
            QSqlQuery query(db);
            QList<QString> tableDefs;
            tableDefs << Skribisto::Common::DirectAccess::Root::getSqlTableDefinition();
            tableDefs << Skribisto::Common::DirectAccess::Work::getSqlTableDefinition();
            tableDefs << Skribisto::Common::DirectAccess::Binder::getSqlTableDefinition();
            for (const auto &sql : tableDefs)
            {
                query.exec(sql);
                // check for errors
                if (query.lastError().isValid())
                    qCritical() << "Error creating table:" << query.lastError().text();
            }
        }

        // Then build junction tables
        {
            QSqlQuery query(db);
            QList<QString> defs;
            defs << Skribisto::Common::DirectAccess::Root::getSqlJunctionTableDefinitions();
            defs << Skribisto::Common::DirectAccess::Work::getSqlJunctionTableDefinitions();
            defs << Skribisto::Common::DirectAccess::Binder::getSqlJunctionTableDefinitions();
            for (const auto &sql : defs)
            {
                query.exec(sql);
                // check for errors
                if (query.lastError().isValid())
                    qCritical() << "Error creating table:" << query.lastError().text();
            }
        }

        // database optimization options
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY") << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=NORMAL") << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers = ON") << QStringLiteral("PRAGMA foreign_keys = ON");

        // execute each optimization option as a single query within the transaction
        {
            QSqlQuery query(db);
            for (const QString &string : std::as_const(optimization))
            {
                query.prepare(string);
                query.exec();
                // check for errors
                if (query.lastError().isValid())
                    qCritical() << "Error creating table:" << query.lastError().text();
            }
        }
        db.commit();
        // Properly close and remove the connection:
        db.close();
        // Drop the last reference to the connection before removeDatabase
        db = QSqlDatabase();
        QSqlDatabase::removeDatabase(builderConn);

        return dbPath;
    }
};
} // namespace Skribisto::Common::Database
