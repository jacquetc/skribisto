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

#include <QHash>
#include <QList>
#include <QSqlDatabase>
#include <QSqlError>
#include <QSqlQuery>
#include <optional>

namespace Skribisto::Common::Database::JunctionTableOps::UnorderedOneToMany
{
constexpr int ORDER_GAP = 1000;

inline QHash<int, QList<int>> getRightIdsMany(QSqlDatabase &db, const QList<int> &leftIds,
                                              const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    if (leftIds.isEmpty())
    {
        return result;
    }

    // Initialize empty lists for all leftIds
    for (int leftId : leftIds)
    {
        result[leftId] = QList<int>();
    }

    // Build dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?"_L1, leftIds.size());
    const QString sql = QStringLiteral("SELECT left_id, right_id FROM %1 WHERE left_id IN (%2)")
                            .arg(junctionTableName, placeholders.join(","_L1));

    QSqlQuery query(db);
    query.prepare(sql);
    for (int leftId : leftIds)
    {
        query.addBindValue(leftId);
    }

    if (query.exec())
    {
        while (query.next())
        {
            int leftId = query.value(0).toInt();
            int rightId = query.value(1).toInt();
            result[leftId].append(rightId);
        }
    }

    return result;
}

inline QList<int> getRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, QList<int>> result = getRightIdsMany(db, {leftId}, junctionTableName);
    return result.value(leftId, QList<int>());
}

inline QHash<int, bool> removeLeftIdsMany(QSqlDatabase &db, const QList<int> &leftIds, const QString &junctionTableName)
{
    QHash<int, bool> result;

    if (leftIds.isEmpty())
    {
        return result;
    }

    // Build dynamic IN clause for efficient bulk delete
    QStringList placeholders;
    placeholders.fill("?"_L1, leftIds.size());
    const QString sql =
        QStringLiteral("DELETE FROM %1 WHERE left_id IN (%2)").arg(junctionTableName, placeholders.join(","_L1));

    QSqlQuery query(db);
    query.prepare(sql);
    for (int leftId : leftIds)
    {
        query.addBindValue(leftId);
    }

    bool success = query.exec();

    // Initialize all results based on success
    for (int leftId : leftIds)
    {
        result[leftId] = success;
    }

    return result;
}

inline bool removeLeftIds(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeLeftIdsMany(db, {leftId}, junctionTableName);
    return result.value(leftId, false);
}

inline QHash<int, bool> removeRightIdsMany(QSqlDatabase &db, const QList<int> &rightIds,
                                           const QString &junctionTableName)
{
    QHash<int, bool> result;

    if (rightIds.isEmpty())
    {
        return result;
    }

    // Build dynamic IN clause for efficient bulk delete
    QStringList placeholders;
    placeholders.fill("?"_L1, rightIds.size());
    const QString sql =
        QStringLiteral("DELETE FROM %1 WHERE right_id IN (%2)").arg(junctionTableName, placeholders.join(","_L1));

    QSqlQuery query(db);
    query.prepare(sql);
    for (int rightId : rightIds)
    {
        query.addBindValue(rightId);
    }

    bool success = query.exec();

    // Initialize all results based on success
    for (int rightId : rightIds)
    {
        result[rightId] = success;
    }

    return result;
}

inline bool removeRightIds(QSqlDatabase &db, int rightId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeRightIdsMany(db, {rightId}, junctionTableName);
    return result.value(rightId, false);
}

inline QHash<int, QList<int>> upsertRightIdsMany(QSqlDatabase &db, const QHash<int, QList<int>> &leftIdToRightIds,
                                                 const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    if (leftIdToRightIds.isEmpty())
    {
        return result;
    }

    // First, remove all existing relationships for these leftIds
    QList<int> leftIds = leftIdToRightIds.keys();
    removeRightIdsMany(db, leftIds, junctionTableName);

    // Then insert new relationships
    QSqlQuery insertQuery(db);
    insertQuery.prepare(QStringLiteral("INSERT INTO %1 (left_id, right_id) VALUES (?, ?)").arg(junctionTableName));

    for (auto it = leftIdToRightIds.begin(); it != leftIdToRightIds.end(); ++it)
    {
        int leftId = it.key();
        const QList<int> &rightIds = it.value();

        for (int rightId : rightIds)
        {
            insertQuery.addBindValue(leftId);
            insertQuery.addBindValue(rightId);
            insertQuery.exec();
        }

        result[leftId] = rightIds;
    }

    return result;
}

inline QList<int> upsertRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                 const QList<int> &rightIds)
{
    QHash<int, QList<int>> input;
    input[leftId] = rightIds;
    QHash<int, QList<int>> result = upsertRightIdsMany(db, input, junctionTableName);
    return result.value(leftId, QList<int>());
}

// for optional
inline QList<int> upsertRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                 const std::optional<QList<int>> &rightIds)
{
    if (!rightIds.has_value())
    {
        removeRightIds(db, leftId, junctionTableName);
        return {};
    }
    return upsertRightIds(db, leftId, junctionTableName, rightIds.value());
}

inline QMap<int, int> getLeftIdMany(QSqlDatabase &db, const QString &junctionTableName, const QList<int> &rightIds)
{
    QMap<int, int> result;

    if (rightIds.isEmpty())
    {
        return result;
    }

    // Build dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?"_L1, rightIds.size());
    const QString sql = QStringLiteral("SELECT right_id, left_id FROM %1 WHERE right_id IN (%2)")
                            .arg(junctionTableName, placeholders.join(","_L1));

    QSqlQuery query(db);
    query.prepare(sql);
    for (int rightId : rightIds)
    {
        query.addBindValue(rightId);
    }

    if (query.exec())
    {
        while (query.next())
        {
            int rightId = query.value(0).toInt();
            int leftId = query.value(1).toInt();
            result[rightId] = leftId;
        }
    }

    return result;
}

inline int getLeftId(QSqlDatabase &db, const QString &junctionTableName, int rightId)
{
    QMap<int, int> result = getLeftIdMany(db, junctionTableName, {rightId});
    return result.value(rightId, -1);
}

} // namespace Skribisto::Common::Database::JunctionTableOps::UnorderedOneToMany
