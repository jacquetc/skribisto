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

PLMProject::PLMProject(QObject *parent, int projectId, const QString& fileName) :
    QObject(parent)
{
    qRegisterMetaType<PLMProject::DBType>("PLMProject::DBType");
    m_projectId = projectId;
    PLMError error;

    if (!fileName.isEmpty()) {
        QFileInfo info(fileName);

        if (!info.exists()) {
            error.setSuccess(false);
        }

        if (!info.isReadable()) {
            error.setSuccess(false);
        }

        IFOKDO(error, setPath(fileName))
    }


    IFOK(error) {
        PLMImporter importer;

        if (fileName == "") { // virgin project
            m_sqlDb = importer.createEmptySQLiteProject(projectId, error);

            IFKO(error) {
                // qWarning << error.getMessage()
                m_projectId = -1;
                qCritical() << "New project not created";

                return;
            }
        } else {
            m_sqlDb = importer.createSQLiteDbFrom("SQLITE", fileName, projectId, error);
        }
        this->setPath(fileName);
    }
    setType("SQLITE");
    IFOK(error) {
        // sheet and note sql tree :
        m_sheetTree = new PLMSheetTree(this, "tbl_sheet", "l_sheet_id", m_sqlDb);
        m_plmTreeForTableNameHash.insert("tbl_sheet", m_sheetTree);
        m_noteTree = new PLMNoteTree(this, "tbl_note", "l_note_id", m_sqlDb);
        m_plmTreeForTableNameHash.insert("tbl_note",  m_noteTree);
        PLMProperty *sheetProperty = new PLMProperty(this,
                                                     "tbl_sheet_property",
                                                     "l_sheet_code",
                                                     m_sqlDb);
        m_plmPropertyForTableNameHash.insert("tbl_sheet_property", sheetProperty);
        PLMProperty *noteProperty = new PLMProperty(this,
                                                    "tbl_note_property",
                                                    "l_note_code",
                                                    m_sqlDb);
        m_plmPropertyForTableNameHash.insert("tbl_note_property", noteProperty);
        PLMProperty *sheetSystemProperty = new PLMProperty(this,
                                                           "tbl_sheet_property",
                                                           "l_sheet_code",
                                                           m_sqlDb);
        m_plmPropertyForTableNameHash.insert("tbl_sheet_system_property",
                                             sheetSystemProperty);
        PLMProperty *noteSystemProperty = new PLMProperty(this,
                                                          "tbl_note_property",
                                                          "l_note_code",
                                                          m_sqlDb);
        m_plmPropertyForTableNameHash.insert("tbl_note_system_property",
                                             noteSystemProperty);
    }


    IFKO(error) {
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

PLMProperty * PLMProject::getProperty(const QString& tableName)
{
    return m_plmPropertyForTableNameHash.value(tableName);
}

PLMTree * PLMProject::getTree(const QString& tableName)
{
    return m_plmTreeForTableNameHash.value(tableName);
}

PLMSheetTree * PLMProject::sheetTree()
{
    return m_sheetTree;
}

PLMNoteTree * PLMProject::noteTree()
{
    return m_noteTree;
}

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

QString PLMProject::getPath() const
{
    return m_path;
}

PLMError PLMProject::setPath(const QString& value)
{
    PLMError error;

    // TODO: check for file rights, etc...
    IFOK(error) {
        m_path = value;
    }
    return error;
}
