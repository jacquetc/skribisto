/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmproject.h                                                   *
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

#ifndef PLMDATABASE_H
#define PLMDATABASE_H

#include <QHash>
#include <QObject>
#include <QUrl>
#include <QtSql/QSqlDatabase>

#include "skribisto_data_global.h"
#include "skrresult.h"

class EXPORT PLMProject : public QObject {
  Q_OBJECT

public:

  explicit PLMProject(QObject *parent,
                      SKRResult *result, const QString &sqlDbConnectionName = QString(), const QUrl &fileName = QUrl());
  ~PLMProject();
  QString getType() const;
  void setType(const QString &value);
  int id() const;
  void setId(int newId);

  QString getTempFileName() const;

  QUrl getFileName() const;
  SKRResult setPath(const QUrl &value);

  QSqlDatabase getSqlDb() const;
  QString getIdNameFromTable(const QString &tableName);

  QString createEmptySQLiteProject(SKRResult &result);

signals:

public slots:

private:
  QString m_sqlDbConnectionName;
  int m_projectId;
  QString m_type;
  QUrl m_fileName;
};

#endif // PLMDATABASE_H
