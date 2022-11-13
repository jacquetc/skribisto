/***************************************************************************
*   Copyright (C) 2021 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skribexporter.cpp                                                   *
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
#include "skribexporter.h"
#include "project/plmprojectmanager.h"

#include <QFile>
#include <QSqlQuery>

SkribExporter::SkribExporter(QObject *parent) : QObject(parent)
{

}

// ---------------------------------------------------

SkribExporter::~SkribExporter()
{}

SKRResult SkribExporter::run(int projectId, const QUrl &url, const QString &extension, const QVariantMap &parameters, QList<int> treeItemIds) const
{
    Q_UNUSED(extension)

    PLMProject *project = plmProjectManager->project(projectId);

    QUrl fileName = url;
    SKRResult result(this);

    // shrink the database
    project->getSqlDb().transaction();
    QString   queryStr("VACUUM");
    QSqlQuery query(project->getSqlDb());
    query.prepare(queryStr);
    query.exec();
    project->getSqlDb().commit();
    QString databaseTempFileName =  project->getTempFileName();

    // copy db temp to file
    QFile tempFile(databaseTempFileName);


    QFile file;

    if (fileName.path().at(2) == ':') { // means Windows
        file.setFileName(fileName.path().remove(0, 1));
    }
    else {
        file.setFileName(fileName.path());
    }

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











