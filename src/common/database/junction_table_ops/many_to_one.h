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

class ManyToOne
{
  public:
    static QHash<int, std::optional<int>> getRightIdMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                         const QString &junctionTableName);

    static std::optional<int> getRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QHash<int, bool> removeWithLeftIdMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                 const QString &junctionTableName);

    static bool removeWithLeftId(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QHash<int, bool> removeWithRightIdsMany(QSqlDatabase &db, const QList<int> &rightIds,
                                                   const QString &junctionTableName);

    static bool removeWithRightIds(QSqlDatabase &db, int rightId, const QString &junctionTableName);

    static QHash<int, QList<int>> upsertRightIdMany(QSqlDatabase &db, const QHash<int, int> &leftIdToRightId,
                                                    const QString &junctionTableName);

    // for optional variant
    static QHash<int, QList<int>> upsertRightIdMany(QSqlDatabase &db,
                                                    const QHash<int, std::optional<int>> &leftIdToRightId,
                                                    const QString &junctionTableName);

    static QList<int> upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName, int right_id);

    // for optional
    static QList<int> upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                    std::optional<int> right_id);

    static QMap<int, QList<int>> getLeftIdsMany(QSqlDatabase &db, const QString &junctionTableName,
                                                const QList<int> &rightIds);

    static QList<int> getLeftIds(QSqlDatabase &db, const QString &junctionTableName, int rightId);

    static int getRightIdCount(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    static QList<int> getRightIdInRange(QSqlDatabase &db, int leftId, const QString &junctionTableName);

    // Validation function to check if left_id already exists with a different right_id
    static bool validateUniqueLeftId(QSqlDatabase &db, int leftId, int rightId, const QString &junctionTableName);
};

} // namespace Skribisto::Common::Database::JunctionTableOps
