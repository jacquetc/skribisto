/***************************************************************************
 *   Copyright (C) 2017 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: upgrader.h                                                   *
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
#ifndef UPGRADER_H
#define UPGRADER_H

#include <QObject>
#include <QSqlDatabase>

#include "skrresult.h"
#include "skribisto_data_global.h"

class EXPORT Upgrader : public QObject {
  Q_OBJECT

public:
  explicit Upgrader(QObject *parent = nullptr);
  static SKRResult upgradeSQLite(const QString &sqlDbConnectionName);

  static SKRResult setDbVersion(const QString &sqlDbConnectionName,
                                double newVersion);

signals:

public slots:

private:
  static SKRResult movePaperToTree_1_5(QSqlDatabase sqlDb,
                                       const QString &tableName);
  static SKRResult transformParentsToFolder_1_5(QSqlDatabase sqlDb);
  static SKRResult dropDeprecatedTables_1_5(QSqlDatabase sqlDb);
  static SKRResult moveSynopsisToSecondaryContent_1_6(QSqlDatabase sqlDb);
  static SKRResult moveTrashedItemsToTrashFolder_2_0(QSqlDatabase sqlDb);
};

#endif // UPGRADER_H
