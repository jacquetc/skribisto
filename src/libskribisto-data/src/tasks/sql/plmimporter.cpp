/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmimporter.cpp                                                   *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
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
#include <QRegularExpression>
#include <QSqlDriver>
#include <QSqlError>
#include <QRandomGenerator>
#include <QUrl>

PLMImporter::PLMImporter(QObject *parent) :
    QObject(parent)
{}

QSqlDatabase PLMImporter::createSQLiteDbFrom(const QString& type,
                                             const QUrl   & fileName,
                                             int            projectId,
                                             PLMError     & error)
{
    if (type == "SQLITE") {
        // create temp file
        QTemporaryFile tempFile;
        tempFile.open();
        tempFile.setAutoRemove(false);
        QString tempFileName = tempFile.fileName();

        // copy db file to temp
        QString fileNameString;

        if (fileName.scheme() == "qrc") {
            fileNameString = fileName.toString(QUrl::RemoveScheme);
            fileNameString = ":" + fileNameString;
        }
        else {
            fileNameString = fileName.toLocalFile();
        }

        QFile file(fileNameString);

        if (!file.exists()) {
            error.setSuccess(false);
            qWarning() << fileNameString + " doesn't exist";
            return QSqlDatabase();
        }

        if (!file.open(QIODevice::ReadOnly)) {
            error.setSuccess(false);
            qWarning() << fileNameString + " can't be copied";
            return QSqlDatabase();
        }

        QByteArray array(file.readAll());
        tempFile.write(array);
        tempFile.close();

        // open temp file
        QSqlDatabase sqlDb =
            QSqlDatabase::addDatabase("QSQLITE", QString::number(projectId));
        sqlDb.setHostName("localhost");
        sqlDb.setDatabaseName(tempFileName);

        // QSqlDatabase memoryDb = copySQLiteDbToMemory(sqlDb, projectId);
        //        QSqlDatabase db = QSqlDatabase::database("db_to_be_imported",
        // false);
        //        db.removeDatabase("db_to_be_imported");
        bool ok = sqlDb.open();

        if (!ok) {
            error.setSuccess(false);

            // emit plmTaskError->errorSent("E_SQLITE",
            // "",m_sqlDb.lastError().text());
            return QSqlDatabase();
        }

        // upgrade :
        IFOKDO(error, PLMUpgrader::upgradeSQLite(sqlDb));
        IFKO(error) {
            error.setSuccess(false);
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

        for(const QString& string : optimization) {
            QSqlQuery query(sqlDb);

            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();

        // clean-up :
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

QSqlDatabase PLMImporter::createEmptySQLiteProject(int projectId, PLMError& error)
{
    // create temp file
    QTemporaryFile tempFile;

    tempFile.open();
    tempFile.setAutoRemove(false);
    QString tempFileName = tempFile.fileName();

    qDebug() << "tempFileName :" << tempFileName;

    // open temp file

    QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", QString::number(projectId));

    sqlDb.setHostName("localhost");
    sqlDb.setDatabaseName(tempFileName);
    bool ok = sqlDb.open();

    if (!ok) {
        error.setSuccess(false);
        return QSqlDatabase();
    }

    // upgrade :
    IFOKDO(error, PLMUpgrader::upgradeSQLite(sqlDb));
    IFKO(error) {
        error.setSuccess(false);
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

    for(const QString& string : optimization) {
        QSqlQuery query(sqlDb);

        query.prepare(string);
        query.exec();
    }

    // new project :
    IFOKDO(error, this->executeSQLFile(":/sql/sqlite_project.sql", sqlDb));
    QString sqlString =
        "INSERT INTO tbl_project (l_skribisto_maj_version, l_skribisto_min_version, l_db_maj_version, l_db_min_version) VALUES (2, 0, 1, 0)";

    IFOKDO(error, this->executeSQLString(sqlString, sqlDb));


    IFOK(error) {
        // create unique identifier


        QString randomString;

        {
            const QString possibleCharacters(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
            const int randomStringLength = 12;

            for (int i = 0; i < randomStringLength; ++i)
            {
                quint32 generatedNumber = QRandomGenerator::system()->generate();
                int     index           = generatedNumber % possibleCharacters.length();
                QChar   nextChar        = possibleCharacters.at(index);
                randomString.append(nextChar);
            }
        }
        qDebug() << "randomString" << randomString;


        QString   value = randomString;
        int       id    = 1;
        QSqlQuery query(sqlDb);
        QString   queryStr = "UPDATE tbl_project SET t_project_unique_identifier = :value"
                             " WHERE l_project_id = :id"
        ;

        query.prepare(queryStr);
        query.bindValue(":value", value);
        query.bindValue(":id",    id);
        query.exec();
    }
    IFOK(error) {
        sqlDb.commit();
    }

    // clean-up :
    sqlDb.transaction();
    PLMSqlQueries sheetQueries(sqlDb, "tbl_sheet", "l_sheet_id");

    IFOKDO(error, sheetQueries.renumberSortOrder());
    PLMSqlQueries noteQueries(sqlDb, "tbl_note", "l_note_id");

    IFOKDO(error, noteQueries.renumberSortOrder());
    sqlDb.commit();

    return sqlDb;
}

// QSqlDatabase PLMImporter::copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
// projectId, PLMError &error)
// {
//    //TODO: not good, to remove ? !!!!
//    QString table("mytable");
//    QSqlDatabase srcDB = sourceSqlDb;
//    srcDB.open();
//    QSqlDatabase destDB = QSqlDatabase::addDatabase("QSQLITE",
// QString::number(projectId));
//    destDB.setHostName("localhost");
//    destDB.setDatabaseName(":memory:");
//    destDB.open();
//    {
//        QSqlQuery srcQuery(srcDB);
//        QSqlQuery destQuery(destDB);

//        // get table schema
//        if (!srcQuery.exec(QString("SHOW CREATE TABLE %1").arg(table))) {
//            return QSqlDatabase();
//        }

//        QString tableCreateStr;

//        while (srcQuery.next()) {
//            tableCreateStr = srcQuery.value(1).toString();
//        }

//        // drop destTable if exists
//        if (!destQuery.exec(QString("DROP TABLE IF EXISTS %1").arg(table))) {
//            return QSqlDatabase();
//        }

//        // create new one
//        if (!destQuery.exec(tableCreateStr)) {
//            return QSqlDatabase();
//        }

//        // copy all entries
//        if (!srcQuery.exec(QString("SELECT * FROM %1").arg(table))) {
//            return QSqlDatabase();
//        }

//        while (srcQuery.next()) {
//            QSqlRecord record = srcQuery.record();
//            QStringList names;
//            QStringList placeholders;
//            QList<QVariant > values;

//            for (int i = 0; i < record.count(); ++i) {
//                names << record.fieldName(i);
//                placeholders << ":" + record.fieldName(i);
//                QVariant value = srcQuery.value(i);

//                if (value.type() == QVariant::String) {
//                    values << "\"" + value.toString() + "\"";
//                } else {
//                    values << value;
//                }
//            }

//            // build new query
//            QString queryStr;
//            queryStr.append("INSERT INTO " + table);
//            queryStr.append(" (" + names.join(", ") + ") ");
//            queryStr.append(" VALUES (" + placeholders.join(", ") + ");");
//            destQuery.prepare(queryStr);

//            foreach (QVariant value, values) {
//                destQuery.addBindValue(value);
//            }

//            if (!destQuery.exec()) {
//                return QSqlDatabase();
//            }
//        }
//    }
//    return destDB;
// }

PLMError PLMImporter::executeSQLFile(const QString& fileName, QSqlDatabase& sqlDB) {
    PLMError error;
    QFile    file(fileName);

    // Read query file content
    file.open(QIODevice::ReadOnly);
    error = this->executeSQLString(file.readAll(), sqlDB);
    file.close();

    return error;
}

PLMError PLMImporter::executeSQLString(const QString& sqlString, QSqlDatabase& sqlDB)
{
    PLMError error;

    QSqlQuery query(sqlDB);


    QString queryStr = sqlString + "\n";

    // Check if SQL Driver supports Transactions
    if (sqlDB.driver()->hasFeature(QSqlDriver::Transactions)) {
        // Replace comments and tabs and new lines with space
        queryStr =
            queryStr.replace(QRegularExpression("(\\/\\*(.)*?\\*\\/|^--.*\\n|\\t)",
                                                QRegularExpression::CaseInsensitiveOption
                                                | QRegularExpression::MultilineOption),
                             " ");
        queryStr = queryStr.trimmed();


        // Extracting queries
        QStringList qList = queryStr.split('\n', QString::SkipEmptyParts);


        QRegularExpression re_transaction("\\bbegin.transaction.*",
                                          QRegularExpression::CaseInsensitiveOption);
        QRegularExpression re_commit("\\bcommit.*",
                                     QRegularExpression::CaseInsensitiveOption);

        // Check if query file is already wrapped with a transaction
        bool isStartedWithTransaction = false;

        if (qList.size() > 1) {
            isStartedWithTransaction = qMax(re_transaction.match(qList.at(0)).hasMatch(),
                                            re_transaction.match(qList.at(1)).hasMatch());
        }

        if (!isStartedWithTransaction) sqlDB.transaction();

        // Execute each individual queries
        bool success = true;

        for (const QString& s : qList) {
            if (re_transaction.match(s).hasMatch()) sqlDB.transaction();
            else if (re_commit.match(s).hasMatch()) sqlDB.commit();
            else {
                query.exec(s);

                if (query.lastError().type() != QSqlError::NoError) {
                    qDebug() << query.lastError().text();
                    sqlDB.rollback();

                    success = false;

                    //
                }
            }
        }

        if (!isStartedWithTransaction) sqlDB.commit();

        if (success == false) {
            error.setSuccess(false);
            return error;
        }

        // Sql Driver doesn't supports transaction
    } else {
        // ...so we need to remove special queries (`begin transaction` and
        // `commit`)
        queryStr =
            queryStr.replace(QRegularExpression(
                                 "(\\bbegin.transaction.*;|\\bcommit.*;|\\/\\*(.|\\n)*?\\*\\/|^--.*\\n|\\t|\\n)",
                                 QRegularExpression::CaseInsensitiveOption
                                 | QRegularExpression::MultilineOption),
                             " ");
        queryStr = queryStr.trimmed();

        // Execute each individual queries
        QStringList qList = queryStr.split(';', QString::SkipEmptyParts);
        for(const QString& s : qList) {
            query.exec(s);

            if (query.lastError().type() != QSqlError::NoError) {
                qDebug() << query.lastError().text();
                error.setSuccess(false);
                return error;
            }
        }
    }
    return error;
}
