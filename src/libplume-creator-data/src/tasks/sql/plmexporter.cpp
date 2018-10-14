/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmexporter.cpp                                                   *
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
#include "plmexporter.h"
#include "plmerror.h"

PLMExporter::PLMExporter(QObject *parent) : QObject(parent)
{
}

PLMError PLMExporter::exportSQLiteDbTo(PLMProject *db, const QString &type, const QString &fileName)
{
    PLMError error;

    if (type == "SQLITE") {
// shrink the database
        db->getSqlDb().transaction();
        QString queryStr("VACUUM");
        QSqlQuery query(db->getSqlDb());
        query.prepare(queryStr);
        query.exec();
        db->getSqlDb().commit();
        QString databaseTempFileName =  db->getTempFileName();
        //copy db temp to file
        QFile tempFile(databaseTempFileName);
        QFile file(fileName);

        if (!tempFile.open(QIODevice::ReadOnly)) {
            qWarning() << fileName + " can't be opened";
            error.setSuccess(false);
            return error;
        }

        if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
            qWarning() << fileName + " can't be opened";
            error.setSuccess(false);
            return error;
        }

        QByteArray array(tempFile.readAll());
        file.write(array);
        file.close();
        return error;
    }

    error.setSuccess(false);
    return error;
}

PLMError PLMExporter::exportUserSQLiteDbTo(PLMProject *db, const QString &fileName)
{
    PLMError error;
// shrink the database
    db->getUserSqlDb().transaction();
    QString queryStr("VACUUM");
    QSqlQuery query(db->getUserSqlDb());
    query.prepare(queryStr);
    query.exec();
    db->getUserSqlDb().commit();
    QString databaseTempFileName =  db->getUserDBTempFileName();
    //copy db temp to file
    QFile tempFile(databaseTempFileName);
    QFile file(fileName);
// add suffix if not done :
    QFileInfo fileInfo(file);

    if (!fileInfo.fileName().contains(".plume.user")) {
        QString newFileName = fileInfo.path() + QDir::separator() + fileInfo.baseName() + ".plume.user";
        file.setFileName(newFileName);
    }

    if (!tempFile.open(QIODevice::ReadOnly)) {
        qWarning() << fileName + " can't be opened";
        error.setSuccess(false);
        return error;
    }

    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
        qWarning() << fileName + " can't be opened";
        error.setSuccess(false);
        return error;
    }

    QByteArray array(tempFile.readAll());
    file.write(array);
    file.close();
    return error;
}
