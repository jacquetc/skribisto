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

namespace Skribisto::Common::Database::JunctionTableOps::OneToOne
{
constexpr int ORDER_GAP = 1000;

inline QHash<int, std::optional<int>> getRightIdMany(QSqlDatabase &db, const QList<int> &leftIds,
                                                     const QString &junctionTableName)
{
    QHash<int, std::optional<int>> result;

    if (leftIds.isEmpty())
    {
        return result;
    }

    // Initialize with nullopt for all leftIds
    for (int leftId : leftIds)
    {
        result[leftId] = std::nullopt;
    }

    // Build dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?", leftIds.size());
    const QString sql = QStringLiteral("SELECT left_id, right_id FROM %1 WHERE left_id IN (%2)")
                            .arg(junctionTableName, placeholders.join(","));

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
            result[leftId] = rightId;
        }
    }

    return result;
}

inline std::optional<int> getRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, std::optional<int>> result = getRightIdMany(db, {leftId}, junctionTableName);
    return result.value(leftId, std::nullopt);
}

inline QHash<int, bool> removeRightIdMany(QSqlDatabase &db, const QList<int> &leftIds, const QString &junctionTableName)
{
    QHash<int, bool> result;

    if (leftIds.isEmpty())
    {
        return result;
    }

    // Build dynamic IN clause for efficient bulk delete
    QStringList placeholders;
    placeholders.fill("?", leftIds.size());
    const QString sql =
        QStringLiteral("DELETE FROM %1 WHERE left_id IN (%2)").arg(junctionTableName, placeholders.join(","));

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

inline bool removeRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeRightIdMany(db, {leftId}, junctionTableName);
    return result.value(leftId, false);
}

inline QHash<int, QList<int>> upsertRightIdMany(QSqlDatabase &db, const QHash<int, int> &leftIdToRightId,
                                                const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    if (leftIdToRightId.isEmpty())
    {
        return result;
    }

    // Process each left_id to right_id mapping
    for (auto it = leftIdToRightId.begin(); it != leftIdToRightId.end(); ++it)
    {
        int leftId = it.key();
        int rightId = it.value();

        QSqlQuery query(db);

        // First try to update existing record
        query.prepare(QStringLiteral("UPDATE %1 SET right_id = ? WHERE left_id = ?").arg(junctionTableName));
        query.addBindValue(rightId);
        query.addBindValue(leftId);

        if (!query.exec() || query.numRowsAffected() == 0)
        {
            // If no rows were affected, insert new record
            query.prepare(QStringLiteral("INSERT INTO %1 (left_id, right_id) VALUES (?, ?)").arg(junctionTableName));
            query.addBindValue(leftId);
            query.addBindValue(rightId);
            query.exec();
        }

        result[leftId] = QList<int>{rightId};
    }

    return result;
}

// for optional variant
inline QHash<int, QList<int>> upsertRightIdMany(QSqlDatabase &db, const QHash<int, std::optional<int>> &leftIdToRightId,
                                                const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    for (auto it = leftIdToRightId.begin(); it != leftIdToRightId.end(); ++it)
    {
        int leftId = it.key();
        std::optional<int> rightId = it.value();

        if (!rightId.has_value())
        {
            removeRightId(db, leftId, junctionTableName);
            result[leftId] = {};
        }
        else
        {
            QHash<int, int> singleMapping;
            singleMapping[leftId] = rightId.value();
            QHash<int, QList<int>> singleResult = upsertRightIdMany(db, singleMapping, junctionTableName);
            result[leftId] = singleResult.value(leftId, QList<int>());
        }
    }

    return result;
}

inline QList<int> upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName, int right_id)
{
    QHash<int, int> input;
    input[leftId] = right_id;
    QHash<int, QList<int>> result = upsertRightIdMany(db, input, junctionTableName);
    return result.value(leftId, QList<int>());
}

// for optional
inline QList<int> upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                std::optional<int> right_id)
{
    QHash<int, std::optional<int>> input;
    input[leftId] = right_id;
    QHash<int, QList<int>> result = upsertRightIdMany(db, input, junctionTableName);
    return result.value(leftId, QList<int>());
}

inline QHash<int, int> getLeftIdMany(QSqlDatabase &db, const QString &junctionTableName, const QList<int> &rightIds)
{
    QHash<int, int> result;

    if (rightIds.isEmpty())
    {
        return result;
    }

    // Build dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?", rightIds.size());
    const QString sql = QStringLiteral("SELECT right_id, left_id FROM %1 WHERE right_id IN (%2)")
                            .arg(junctionTableName, placeholders.join(","));

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
    QHash<int, int> result = getLeftIdMany(db, junctionTableName, {rightId});
    return result.value(rightId, -1);
}
} // namespace Skribisto::Common::Database::JunctionTableOps::OneToOne
