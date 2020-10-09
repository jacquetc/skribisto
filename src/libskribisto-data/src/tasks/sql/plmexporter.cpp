/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmexporter.cpp                                                   *
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
#include "plmexporter.h"
#include "plmerror.h"

PLMExporter::PLMExporter(QObject *parent) : QObject(parent)
{}

PLMError PLMExporter::exportSQLiteDbTo(PLMProject    *db,
                                       const QString& type,
                                       const QUrl   & fileName)
{
    PLMError error;
    QString  finalType = type;

    if (type == "skrib") {
        finalType = "SQLITE";
    }

    if (finalType == "SQLITE") {
        // shrink the database
        db->getSqlDb().transaction();
        QString   queryStr("VACUUM");
        QSqlQuery query(db->getSqlDb());
        query.prepare(queryStr);
        query.exec();
        db->getSqlDb().commit();
        QString databaseTempFileName =  db->getTempFileName();

        // copy db temp to file
        QFile tempFile(databaseTempFileName);
        QFile file(fileName.toLocalFile());

        if (!tempFile.open(QIODevice::ReadOnly)) {
            qWarning() << fileName.toLocalFile() + " can't be opened";
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_tempfile_cant_be_opened");
            return error;
        }

        if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
            qWarning() << fileName.toLocalFile() + " can't be opened";
            error.setSuccess(false);
            error.setErrorCode("E_PROJECT_path_is_readonly");
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
