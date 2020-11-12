/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmdatabase.cpp                                                   *
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

#include "plmproject.h"
#include "plmimporter.h"
#include "plmexporter.h"
#include <QtSql/QSqlError>
#include <QDebug>
#include <QString>
#include <QSqlDriver>
#include <QSqlRecord>
#include <QSqlField>
#include <QCoreApplication>
#include <QFileInfo>
#include <QTimer>

PLMProject::PLMProject(QObject *parent, int projectId, const QUrl& fileName, SKRResult *result) :
    QObject(parent)
{
    qRegisterMetaType<PLMProject::DBType>("PLMProject::DBType");
    m_projectId = projectId;


    if (!fileName.isEmpty()) {
        QFileInfo info;

        if (fileName.scheme() == "qrc") {
            info.setFile(fileName.toString().replace("qrc:", ":"));
        }
        else {
            info.setFile(fileName.toLocalFile());
        }

        if (!info.exists()) {
            *result = SKRResult(SKRResult::Critical, this, "file_not_existing");
        }

        if (!info.isReadable()) {
            *result = SKRResult(SKRResult::Critical, this, "file_not_readable");
        }

        IFOKDO(*result, setPath(fileName))
    }


    IFOK(*result) {
        PLMImporter importer;

        if (fileName.isEmpty()) { // virgin project
            m_sqlDb = importer.createEmptySQLiteProject(projectId, *result);

            IFKO(*result) {
                // qWarning << result.getMessage()
                m_projectId = -1;
                qCritical() << "New project not created";

                return;
            }
        } else {
            m_sqlDb = importer.createSQLiteDbFrom("SQLITE", fileName, projectId, *result);
        }
        this->setPath(fileName);
    }
    setType("skrib");
    IFOK(*result) {
    }


    IFKO(*result) {
        *result = SKRResult(SKRResult::Critical, this, "project_creation_failed");
        m_projectId = -1;
    }
}

PLMProject::~PLMProject()
{
    // close DB :
    m_sqlDb.close();

    // remove temporary files :
    QFile tempFile(this->getTempFileName());

    if (tempFile.exists() && tempFile.isWritable()) {
        tempFile.remove();
    }
}

QSqlDatabase PLMProject::getSqlDb() const
{
    return m_sqlDb;
}

QString PLMProject::getIdNameFromTable(const QString& tableName)
{
    QSqlDatabase sqlDb;

    sqlDb = m_sqlDb;


    if (!sqlDb.isOpen()) {
        sqlDb.open();
    }

    QSqlRecord record =  sqlDb.driver()->record(tableName);
    QString    idName;

    for (int i = 0; i < record.count(); ++i) {
        QString field(record.field(i).name());

        if (field.endsWith("_id")) {
            idName = field;
        }
    }

    return idName;
}

QString PLMProject::getTempFileName() const
{
    return m_sqlDb.databaseName();
}

//PLMProperty * PLMProject::getProperty(const QString& tableName)
//{
//    return m_plmPropertyForTableNameHash.value(tableName);
//}

//PLMTree * PLMProject::getTree(const QString& tableName)
//{
//    return m_plmTreeForTableNameHash.value(tableName);
//}

//PLMSheetTree * PLMProject::sheetTree()
//{
//    return m_sheetTree;
//}

//PLMNoteTree * PLMProject::noteTree()
//{
//    return m_noteTree;
//}

QString PLMProject::getType() const
{
    return m_type;
}

void PLMProject::setType(const QString& value)
{
    m_type = value;
}

int PLMProject::id() const
{
    return m_projectId;
}

QUrl PLMProject::getPath() const
{
    return m_path;
}

SKRResult PLMProject::setPath(const QUrl& value)
{
    SKRResult result(this);

    // TODO: check for file rights, etc...
    IFOK(result) {
        m_path = value;
    }
    return result;
}
