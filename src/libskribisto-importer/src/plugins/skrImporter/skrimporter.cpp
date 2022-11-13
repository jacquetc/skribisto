/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrexporter.cpp                                                   *
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
#include "skrimporter.h"
#include "project/upgrader.h"
#include "sql/plmsqlqueries.h"
#include "sql/skrsqltools.h"

#include <QSqlDatabase>
#include <QSqlQuery>
#include <QTemporaryFile>
#include <QUrl>

SkrImporter::SkrImporter(QObject *parent) : QObject(parent)
{

}

// ---------------------------------------------------

SkrImporter::~SkrImporter()
{}



QString SkrImporter::importProject(const QUrl &url, const QVariantMap &parameters , SKRResult &result) const
{
        // create temp file
        QTemporaryFile tempFile;
        tempFile.open();
        tempFile.setAutoRemove(false);
        QString tempFileName = tempFile.fileName();

        // copy db file to temp
        QString fileNameString;

        if (url.scheme() == "qrc") {
            fileNameString = url.toString(QUrl::RemoveScheme);
            fileNameString = ":" + fileNameString;
        }
        else if (url.path().at(2) == ':') { // means Windows
            fileNameString = url.path().remove(0, 1);
        }
        else {
            fileNameString = url.path();
        }

        QFile file(fileNameString);

        if (!file.exists()) {
            result = SKRResult(SKRResult::Critical, this, "file_does_not_exist");
            result.addData("filePath", fileNameString);
            qWarning() << fileNameString + " doesn't exist";
            return QString();
        }

        if (!file.open(QIODevice::ReadOnly)) {
            result = SKRResult(SKRResult::Critical, this, "file_cant_be_opened");
            result.addData("filePath", fileNameString);
            qWarning() << fileNameString + " can't be opened";
            return QString();
        }

        QByteArray array(file.readAll());
        tempFile.write(array);
        tempFile.close();

        // open temp file
        QSqlDatabase sqlDb =
            QSqlDatabase::addDatabase("QSQLITE", tempFileName);
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

            return QString();
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
        IFOKDO(result, Upgrader::upgradeSQLite(tempFileName));
        IFKO(result) {
            result = SKRResult(SKRResult::Critical, this, "upgrade_sqlite_failed");
            result.addData("filePath", fileNameString);
            return QString();
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
        return sqlDb.databaseName();

}
