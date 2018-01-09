/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmimporter.cpp                                                   *
 *  This file is part of Plume Creator.                                    *
 *                                                                         *
 *  Plume Creator is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Plume Creator is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/

#include "plmimporter.h"
#include "plmupgrader.h"
#include "tasks/plmsqlqueries.h"

#include <QtSql/QSqlQuery>
#include <QtSql/QSqlRecord>
#include <QVariant>
#include <QTemporaryFile>
#include <QDebug>
#include <QDir>
#include <QProcess>
#include <QByteArray>

PLMImporter::PLMImporter(QObject *parent) :
    QObject(parent)
{
}

QSqlDatabase PLMImporter::createSQLiteDbFrom(const QString &type, const QString &fileName, int projectId, PLMError &error)
{
    if (type == "SQLITE") {
        //create temp file
        QTemporaryFile tempFile;
        tempFile.open();
        tempFile.setAutoRemove(false);
        QString tempFileName = tempFile.fileName();
        //copy db file to temp
        QFile file(fileName);

        if (!file.exists()) {
            error.setSuccess(false);
            qWarning() << fileName + " doesn't exist";
            return QSqlDatabase();
        }

        if (!file.open(QIODevice::ReadOnly)) {
            error.setSuccess(false);
            qWarning() << fileName + " can't be copied";
            return QSqlDatabase();
        }

        QByteArray array(file.readAll());
        tempFile.write(array);
        tempFile.close();
        //open temp file
        QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", QString::number(projectId));
        sqlDb.setHostName("localhost");
        sqlDb.setDatabaseName(tempFileName);
        //QSqlDatabase memoryDb = copySQLiteDbToMemory(sqlDb, projectId);
        //        QSqlDatabase db = QSqlDatabase::database("db_to_be_imported", false);
        //        db.removeDatabase("db_to_be_imported");
        bool ok = sqlDb.open();

        if (!ok) {
            error.setSuccess(false);
            //emit plmTaskError->errorSent("E_SQLITE", "",m_sqlDb.lastError().text());
            return QSqlDatabase();
        }

        // upgrade :
        IFOKDO(error, PLMUpgrader::upgradeSQLite(sqlDb));
        IFKO(error) {
            return QSqlDatabase();
        }
        // optimization :
        QStringList optimization;
        optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                     << QStringLiteral("PRAGMA journal_mode=MEMORY")
                     << QStringLiteral("PRAGMA temp_store=MEMORY")
                     << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                     << QStringLiteral("PRAGMA synchronous = OFF")
                     << QStringLiteral("PRAGMA recursive_triggers=true");
        sqlDb.transaction();

        foreach (const QString &string, optimization) {
            QSqlQuery query(sqlDb);
            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();
        //clean-up :
        sqlDb.transaction();
        PLMSqlQueries sheetQueries(sqlDb, "tbl_sheet", "l_sheet_id");
        IFOKDO(error, sheetQueries.renumberSortOrder());
        PLMSqlQueries noteQueries(sqlDb, "tbl_note", "l_note_id");
        IFOKDO(error, noteQueries.renumberSortOrder());
        sqlDb.commit();
        return sqlDb;
    }

    return QSqlDatabase();
}

QSqlDatabase PLMImporter::createEmptySQLiteProject(int projectId, PLMError &error)
{
    //TODO : create minimal DB
    return QSqlDatabase();
}

QSqlDatabase PLMImporter::createUserSQLiteFileFrom(const QString &type, const QString &fileName, int projectId, PLMError &error)
{
    //create temp file
    QTemporaryFile tempFile;
    tempFile.open();
    tempFile.setAutoRemove(false);
    QString tempFileName = tempFile.fileName();
    //copy db file to temp
    QFile file(fileName);

    if (!file.exists()) {
        error.setSuccess(false);
        qWarning() << fileName + " doesn't exist";
        return QSqlDatabase();
    }

    if (!file.open(QIODevice::ReadOnly)) {
        error.setSuccess(false);
        qWarning() << fileName + " can't be copied";
        return QSqlDatabase();
    }

    QByteArray array(file.readAll());
    tempFile.write(array);
    tempFile.close();
    //open temp file
    QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", "user_" + QString::number(projectId));
    sqlDb.setHostName("localhost");
    sqlDb.setDatabaseName(tempFileName);
    //QSqlDatabase memoryDb = copySQLiteDbToMemory(sqlDb, projectId);
    //        QSqlDatabase db = QSqlDatabase::database("db_to_be_imported", false);
    //        db.removeDatabase("db_to_be_imported");
    bool ok = sqlDb.open();

    if (!ok) {
        error.setSuccess(false);
        //emit plmTaskError->errorSent("E_SQLITE", "",m_sqlDb.lastError().text());
        return QSqlDatabase();
    }

    // upgrade :
    IFOKDO(error, PLMUpgrader::upgradeSQLite(sqlDb));
    IFKO(error) {
        return QSqlDatabase();
    }
    // optimization :
    QStringList optimization;
    optimization << QStringLiteral("PRAGMA case_sensitive_like=true")
                 << QStringLiteral("PRAGMA journal_mode=MEMORY")
                 << QStringLiteral("PRAGMA temp_store=MEMORY")
                 << QStringLiteral("PRAGMA locking_mode=EXCLUSIVE")
                 << QStringLiteral("PRAGMA synchronous = OFF")
                 << QStringLiteral("PRAGMA recursive_triggers=true");
    sqlDb.transaction();

    foreach (const QString &string, optimization) {
        QSqlQuery query(sqlDb);
        query.prepare(string);
        query.exec();
    }

    sqlDb.commit();
    return sqlDb;
}

QSqlDatabase PLMImporter::createEmptyUserSQLiteFile(int projectId, PLMError &error)
{
    //TODO : create minimal user DB
    return QSqlDatabase();
}


QSqlDatabase PLMImporter::copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int projectId, PLMError &error)
{
    //TODO: not good, to remove ? !!!!
    QString table("mytable");
    QSqlDatabase srcDB = sourceSqlDb;
    srcDB.open();
    QSqlDatabase destDB = QSqlDatabase::addDatabase("QSQLITE", QString::number(projectId));
    destDB.setHostName("localhost");
    destDB.setDatabaseName(":memory:");
    destDB.open();
    {
        QSqlQuery srcQuery(srcDB);
        QSqlQuery destQuery(destDB);

        // get table schema
        if (!srcQuery.exec(QString("SHOW CREATE TABLE %1").arg(table))) {
            return QSqlDatabase();
        }

        QString tableCreateStr;

        while (srcQuery.next()) {
            tableCreateStr = srcQuery.value(1).toString();
        }

        // drop destTable if exists
        if (!destQuery.exec(QString("DROP TABLE IF EXISTS %1").arg(table))) {
            return QSqlDatabase();
        }

        // create new one
        if (!destQuery.exec(tableCreateStr)) {
            return QSqlDatabase();
        }

        // copy all entries
        if (!srcQuery.exec(QString("SELECT * FROM %1").arg(table))) {
            return QSqlDatabase();
        }

        while (srcQuery.next()) {
            QSqlRecord record = srcQuery.record();
            QStringList names;
            QStringList placeholders;
            QList<QVariant > values;

            for (int i = 0; i < record.count(); ++i) {
                names << record.fieldName(i);
                placeholders << ":" + record.fieldName(i);
                QVariant value = srcQuery.value(i);

                if (value.type() == QVariant::String) {
                    values << "\"" + value.toString() + "\"";
                } else {
                    values << value;
                }
            }

            // build new query
            QString queryStr;
            queryStr.append("INSERT INTO " + table);
            queryStr.append(" (" + names.join(", ") + ") ");
            queryStr.append(" VALUES (" + placeholders.join(", ") + ");");
            destQuery.prepare(queryStr);

            foreach (QVariant value, values) {
                destQuery.addBindValue(value);
            }

            if (!destQuery.exec()) {
                return QSqlDatabase();
            }
        }
    }
    return destDB;
}
