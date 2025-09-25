/******************************************************************************
 Copyright (C) 2025 by Cyril Jacquet                                          *
 cyril.jacquet@skribisto.eu                                                   *
                                                                              *
 This file is part of Skribisto.                                              *
                                                                              *
 Skribisto is free software: you can redistribute it and/or modify            *
 it under the terms of the GNU General Public License as published by         *
 the Free Software Foundation, either version 3 of the License, or            *
 (at your option) any later version.                                          *
                                                                              *
 Skribisto is distributed in the hope that it will be useful,                 *
 but WITHOUT ANY WARRANTY; without even the implied warranty of               *
 MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                *
 GNU General Public License for more details.                                 *
                                                                              *
 You should have received a copy of the GNU General Public License            *
 along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.           *
 ******************************************************************************/

#pragma once
#include "junction_cache.h"
#include <QHash>
#include <QList>
#include <QMap>
#include <QSqlDatabase>
#include <optional>

namespace Skribisto::Common::Database::JunctionTableOps
{

class UnorderedOneToMany
{
  public:
    static QHash<int, QList<int>> getRightIdsMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                  const QString &junctionTableName);

    static QList<int> getRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QHash<int, bool> removeWithLeftIdsMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                  const QString &junctionTableName);

    static bool removeWithLeftIds(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QHash<int, bool> removeWithRightIdsMany(QSqlDatabase &db, const QList<int> &rightIds,
                                                   const QString &junctionTableName);

    static bool removeWithRightIds(QSqlDatabase &db, int rightId, const QString &junctionTableName);

    static QHash<int, QList<int>> upsertRightIdsMany(QSqlDatabase &db, const QHash<int, QList<int>> &leftIdToRightIds,
                                                     const QString &junctionTableName);

    static QList<int> upsertRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                     const QList<int> &rightIds);

    // for optional
    static QList<int> upsertRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                     const std::optional<QList<int>> &rightIds);

    static QMap<int, int> getLeftIdMany(QSqlDatabase &db, const QString &junctionTableName, const QList<int> &rightIds);

    static int getLeftId(QSqlDatabase &db, const QString &junctionTableName, int rightId);

    static int getRightIdsCount(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QList<int> getRightIdsInRange(QSqlDatabase &db, int leftId, const QString &junctionTableName, int offset,
                                         int limit);
};

} // namespace Skribisto::Common::Database::JunctionTableOps