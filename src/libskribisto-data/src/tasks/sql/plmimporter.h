/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmimporter.h                                                   *
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

#ifndef PLMIMPORTER_H
#define PLMIMPORTER_H

#include <QFile>
#include <QObject>
#include <QtSql/QSqlDatabase>
#include <QUrl>
#include <QXmlStreamReader>

#include "skrresult.h"

class PLMImporter : public QObject {
    Q_OBJECT

public:

    explicit PLMImporter(QObject *parent = 0);

    QSqlDatabase createSQLiteDbFrom(const QString& type,
                                    const QUrl   & fileName,
                                    int            projectId,
                                    SKRResult    & result);
    QSqlDatabase createEmptySQLiteProject(int            projectId,
                                          SKRResult    & result,
                                          const QString& sqlFile = "");

    SKRResult importPlumeCreatorProject(const QUrl& plumeFileName,
                                        const QUrl& skribistoFileName);

signals:

public slots:

private:

    //    QSqlDatabase copySQLiteDbToMemory(QSqlDatabase sourceSqlDb, int
    // projectId, SKRResult &result);
    SKRResult transformParentsToFolder(int projectId);

    SKRResult createPapersAndAssociations(int                     projectId,
                                          int                     indent,
                                          const QXmlStreamReader& xml,
                                          const QString         & tempDirPath,
                                          int                     textFolderId,
                                          int                     noteFolderId);
    SKRResult createNote(int            projectId,
                         int            indent,
                         int            plumeId,
                         const QString& name,
                         const QString& tempDirPath,
                         int            parentFolderId);

    SKRResult createTagsFromAttend(int                     projectId,
                                   int                     noteId,
                                   const QXmlStreamReader& xml,
                                   const QString         & attributeName,
                                   const QStringList     & values);

    SKRResult readXMLRecursivelyAndCreatePaper(int               projectId,
                                               int               indent,
                                               QXmlStreamReader *xml,
                                               const QString   & tempDirPath,
                                               int               textFolderId,
                                               int               noteFolderId);

private:

    // used to track old plume id with new skribisto note id
    QHash<int, int>m_attendanceConversionHash;

};

#endif // PLMIMPORTER_H
