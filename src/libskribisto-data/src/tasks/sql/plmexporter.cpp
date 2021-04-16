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
#include "skrresult.h"

PLMExporter::PLMExporter(QObject *parent) : QObject(parent)
{}

SKRResult PLMExporter::exportWholeSQLiteDbTo(PLMProject    *db,
                                             const QString& type,
                                             const QUrl   & fileName)
{
    SKRResult result(this);
    QString   finalType = type;

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
            result = SKRResult(SKRResult::Critical, this, "tempfile_cant_be_opened");
            result.addData("fileName", fileName.toLocalFile());
            return result;
        }

        if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate)) {
            result = SKRResult(SKRResult::Warning, this, "path_is_readonly");
            result.addData("fileName", fileName.toLocalFile());
            return result;
        }

        QByteArray array(tempFile.readAll());
        file.write(array);
        file.close();
        return result;
    }

    result = SKRResult(SKRResult::Critical, this, "unsupported_finalType");

    return result;
}
