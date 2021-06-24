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
#include "skrdata.h"
#include "skrsqltools.h"


PLMImporter::PLMImporter(QObject *parent) :
    QObject(parent)
{}

// -----------------------------------------------------------------------------------------------

QSqlDatabase PLMImporter::createSQLiteDbFrom(const QString& type,
                                             const QUrl   & fileName,
                                             int            projectId,
                                             SKRResult    & result)
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
            result = SKRResult(SKRResult::Critical, this, "file_does_not_exist");
            result.addData("filePath", fileNameString);
            qWarning() << fileNameString + " doesn't exist";
            return QSqlDatabase();
        }

        if (!file.open(QIODevice::ReadOnly)) {
            result = SKRResult(SKRResult::Critical, this, "file_cant_be_opened");
            result.addData("filePath", fileNameString);
            qWarning() << fileNameString + " can't be opened";
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
            result = SKRResult(SKRResult::Critical, this, "cant_open_database");
            result.addData("filePath", tempFileName);

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

        // upgrade :
        IFOKDO(result, PLMUpgrader::upgradeSQLite(sqlDb));
        IFKO(result) {
            result = SKRResult(SKRResult::Critical, this, "upgrade_sqlite_failed");
            result.addData("filePath", fileNameString);
            return QSqlDatabase();
        }


        for (const QString& string : qAsConst(optimization)) {
            QSqlQuery query(sqlDb);

            query.prepare(string);
            query.exec();
        }

        sqlDb.commit();

        // clean-up :
        sqlDb.transaction();
        PLMSqlQueries treeQueries(sqlDb, "tbl_tree", "l_tree_id");
        IFOKDO(result, treeQueries.renumberSortOrder());
        sqlDb.commit();
        return sqlDb;
    }

    return QSqlDatabase();
}

// -----------------------------------------------------------------------------------------------

QSqlDatabase PLMImporter::createEmptySQLiteProject(int projectId, SKRResult& result, const QString& sqlFile)
{
    bool isSpecificSqlFile = true;

    if (sqlFile.isEmpty()) {
        isSpecificSqlFile = false;
    }

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
        result = SKRResult(SKRResult::Critical, this, "cant_open_database");
        result.addData("filePath", tempFileName);
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

    for (const QString& string : qAsConst(optimization)) {
        QSqlQuery query(sqlDb);

        query.prepare(string);
        query.exec();
    }

    // new project :

    QString sqlFileName;

    if (isSpecificSqlFile) {
        sqlFileName = sqlFile;
    }
    else {
        sqlFileName = ":/sql/sqlite_project.sql";
    }

    IFOKDO(result, SKRSqlTools::executeSQLFile(sqlFileName, sqlDb));

    // fetch db version

    QString dbVersion = "-2";

    IFOK(result) {
        dbVersion = SKRSqlTools::getProjectTemplateDBVersion(&result);
    }


    QString sqlString =
        QString("INSERT INTO tbl_project (dbl_database_version) VALUES (%1)")
        .arg(dbVersion);

    IFOKDO(result, SKRSqlTools::executeSQLString(sqlString, sqlDb));

    // upgrade :
    IFOKDO(result, PLMUpgrader::upgradeSQLite(sqlDb));
    IFKO(result) {
        result = SKRResult(SKRResult::Critical, this, "upgrade_sqlite_failed");
        result.addData("filePath", tempFileName);
        return QSqlDatabase();
    }


    IFOK(result) {
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
    IFOK(result) {
        sqlDb.commit();
    }

    // clean-up :
    sqlDb.transaction();
    PLMSqlQueries treeQueries(sqlDb, "tbl_tree", "l_tree_id");

    IFOKDO(result, treeQueries.renumberSortOrder());
    sqlDb.commit();

    return sqlDb;
}

// QSqlDatabase PLMImporter::copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
// projectId, SKRResult &result)
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

// -----------------------------------------------------------------------------------------------
