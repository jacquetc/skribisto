/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrdatabase.cpp * This file is part of Skribisto. *
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
#include "sql/plmsqlqueries.h"
#include "sql/skrsqltools.h"
#include "upgrader.h"
#include <QCoreApplication>
#include <QDebug>
#include <QFileInfo>
#include <QRandomGenerator>
#include <QSqlDriver>
#include <QSqlField>
#include <QSqlRecord>
#include <QString>
#include <QTemporaryFile>
#include <QTimer>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>

PLMProject::PLMProject(QObject *parent, SKRResult *result, const QString &sqlDbConnectionName, const QUrl &fileName)
    : QObject(parent),
m_sqlDbConnectionName(sqlDbConnectionName),
  m_fileName(fileName)
{

  if (!fileName.isEmpty()) {
    QFileInfo info;

    if (fileName.scheme() == "qrc") {
      info.setFile(fileName.toString().replace("qrc:", ":"));
    } else if (fileName.path().at(2) == ':') { // means Windows
      info.setFile(fileName.path().remove(0, 1));
    } else {
      info.setFile(fileName.path());
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

    if (sqlDbConnectionName.isEmpty()) { // virgin project
      m_sqlDbConnectionName = this->createEmptySQLiteProject(*result);

      IFKO(*result) {
        // qWarning << result.getMessage()
        m_projectId = -1;
        qCritical() << "New project not created";

        return;
      }
     }
    this->setPath(fileName);
  }
  setType("skrib");
  IFOK(*result) {
    // verify if db version is usable by this version of Skribisto :
    bool ok;
    double dbTemplateVersion =
        SKRSqlTools::getProjectTemplateDBVersion(result).toDouble(&ok);

    if (!ok) {
      *result = SKRResult(SKRResult::Critical, this,
                          "string_to_double_conversion_failed");
    }

    IFOK(*result) {
      double dbVersion =
          SKRSqlTools::getProjectDBVersion(result, m_sqlDbConnectionName);

      IFOK(*result) {
        if (dbTemplateVersion < dbVersion) {
          // means that Skribisto can't use this db
          *result = SKRResult(SKRResult::Critical, this,
                              "db_version_higher_than_skribisto_can_read");
        }
      }
    }
  }

  IFKO(*result) {
    *result = SKRResult(SKRResult::Critical, this, "project_creation_failed");
    m_projectId = -1;
  }
}

PLMProject::~PLMProject() {
  QString tempFileName = this->getTempFileName();

  //    // close DB :
  //    m_sqlDb.close();

  // remove temporary files :
  QFile tempFile(tempFileName);
  QFileInfo tempFileInfo(tempFile);

  if (tempFileInfo.exists() && tempFileInfo.isWritable()) {
    tempFile.remove();
  }
}

QSqlDatabase PLMProject::getSqlDb() const {
  return QSqlDatabase::database(m_sqlDbConnectionName);
}

QString PLMProject::getIdNameFromTable(const QString &tableName) {
  QSqlDatabase sqlDb = getSqlDb();

  if (!sqlDb.isOpen()) {
    sqlDb.open();
  }

  QSqlRecord record = sqlDb.driver()->record(tableName);
  QString idName;

  for (int i = 0; i < record.count(); ++i) {
    QString field(record.field(i).name());

    if (field.endsWith("_id")) {
      idName = field;
    }
  }

  return idName;
}

QString PLMProject::getTempFileName() const {
  return getSqlDb().databaseName();
}

// PLMProperty * PLMProject::getProperty(const QString& tableName)
// {
//    return m_plmPropertyForTableNameHash.value(tableName);
// }

// PLMTree * PLMProject::getTree(const QString& tableName)
// {
//    return m_plmTreeForTableNameHash.value(tableName);
// }

// PLMSheetTree * PLMProject::sheetTree()
// {
//    return m_sheetTree;
// }

// PLMNoteTree * PLMProject::noteTree()
// {
//    return m_noteTree;
// }

QString PLMProject::getType() const { return m_type; }

void PLMProject::setType(const QString &value) { m_type = value; }

int PLMProject::id() const { return m_projectId; }

void PLMProject::setId(int newId)
{
  m_projectId = newId;
}

QUrl PLMProject::getFileName() const { return m_fileName; }

SKRResult PLMProject::setPath(const QUrl &value) {
  SKRResult result(this);

  // TODO: check for file rights, etc...
  IFOK(result) { m_fileName = value; }
  return result;
}

QString PLMProject::createEmptySQLiteProject(SKRResult &result) {
  // create temp file
  QTemporaryFile tempFile;

  tempFile.open();
  tempFile.setAutoRemove(false);
  QString tempFileName = tempFile.fileName();

  qDebug() << "tempFileName :" << tempFileName;

  // open temp file

  QSqlDatabase sqlDb = QSqlDatabase::addDatabase("QSQLITE", tempFileName);

  sqlDb.setHostName("localhost");
  sqlDb.setDatabaseName(tempFileName);
  bool ok = sqlDb.open();

  if (!ok) {
    result = SKRResult(SKRResult::Critical, this, "cant_open_database");
    result.addData("filePath", tempFileName);
    return "";
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

  for (const QString &string : qAsConst(optimization)) {
    QSqlQuery query(sqlDb);

    query.prepare(string);
    query.exec();
  }

  // new project :

  QString sqlFileName = ":/sql/sqlite_project.sql";


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
  IFOKDO(result, Upgrader::upgradeSQLite(tempFileName));
  IFKO(result) {
    result = SKRResult(SKRResult::Critical, this, "upgrade_sqlite_failed");
    result.addData("filePath", tempFileName);
    return "";
  }

  // create unique identifier

  QString randomString;

  {
    const QString possibleCharacters(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
    const int randomStringLength = 12;

    for (int i = 0; i < randomStringLength; ++i) {
      quint32 generatedNumber = QRandomGenerator::system()->generate();
      int index = generatedNumber % possibleCharacters.length();
      QChar nextChar = possibleCharacters.at(index);
      randomString.append(nextChar);
    }
  }
  IFOK(result) {

    QString value = randomString;
    int id = 1;
    QSqlQuery query(sqlDb);
    QString queryStr =
        "UPDATE tbl_project SET t_project_unique_identifier = :value"
        " WHERE l_project_id = :id";

    query.prepare(queryStr);
    query.bindValue(":value", value);
    query.bindValue(":id", id);
    query.exec();
  }
  IFOK(result) { sqlDb.commit(); }

  // clean-up :
  sqlDb.transaction();
  PLMSqlQueries treeQueries(sqlDb, "tbl_tree", "l_tree_id");

  IFOKDO(result, treeQueries.renumberSortOrder());
  sqlDb.commit();

  return sqlDb.databaseName();
}
