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
#include <QSet>
#include <QSqlDatabase>
#include <QSqlError>
#include <QSqlQuery>
#include <optional>

namespace Skribisto::Common::Database::JunctionTableOps::OrderedOneToMany
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
    const QString sql =
        QStringLiteral("SELECT left_id, right_id FROM %1 WHERE left_id IN (%2) ORDER BY left_id, order_")
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
    QList<int> cachedResult;
    if (JunctionCache::instance().getCachedRightIds(junctionTableName, leftId, cachedResult))
    {
        return cachedResult;
    }

    QHash<int, QList<int>> result = getRightIdsMany(db, {leftId}, junctionTableName);
    QList<int> rightIds = result.value(leftId, QList<int>());

    // Cache the result
    JunctionCache::instance().setCachedRightIds(junctionTableName, leftId, rightIds);

    return rightIds;
}

inline QHash<int, bool> removeWithLeftIdsMany(QSqlDatabase &db, const QList<int> &leftIds,
                                              const QString &junctionTableName)
{
    QHash<int, bool> result;

    if (leftIds.isEmpty())
    {
        return result;
    }

    // Invalidate cache for affected left IDs
    for (int leftId : leftIds)
    {
        JunctionCache::instance().invalidateLeftId(junctionTableName, leftId);
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

inline bool removeWithLeftIds(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeWithLeftIdsMany(db, {leftId}, junctionTableName);
    return result.value(leftId, false);
}

inline QHash<int, bool> removeWithRightIdsMany(QSqlDatabase &db, const QList<int> &rightIds,
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

inline bool removeWithRightIds(QSqlDatabase &db, int rightId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeWithRightIdsMany(db, {rightId}, junctionTableName);
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

    // Invalidate cache for affected left IDs
    QList<int> leftIds = leftIdToRightIds.keys();
    for (int leftId : leftIds)
    {
        JunctionCache::instance().invalidateLeftId(junctionTableName, leftId);
    }

    // First, remove all existing relationships for these leftIds
    removeWithRightIdsMany(db, leftIds, junctionTableName);

    // Then insert new relationships with proper ordering
    QSqlQuery insertQuery(db);
    insertQuery.prepare(
        QStringLiteral("INSERT INTO %1 (left_id, right_id, order_) VALUES (?, ?, ?)").arg(junctionTableName));

    for (auto it = leftIdToRightIds.begin(); it != leftIdToRightIds.end(); ++it)
    {
        int leftId = it.key();
        const QList<int> &rightIds = it.value();

        for (qsizetype i = 0; i < rightIds.size(); ++i)
        {
            insertQuery.addBindValue(leftId);
            insertQuery.addBindValue(rightIds[i]);
            insertQuery.addBindValue(static_cast<int>(i) * ORDER_GAP);
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
        removeWithRightIds(db, leftId, junctionTableName);
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

inline int getRightIdsCount(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    int cachedResult;
    if (JunctionCache::instance().getCachedRightIdsCount(junctionTableName, leftId, cachedResult))
    {
        return cachedResult;
    }

    const QString sql = QStringLiteral("SELECT COUNT(right_id) FROM %1 WHERE left_id = ?").arg(junctionTableName);

    QSqlQuery query(db);
    query.prepare(sql);
    query.addBindValue(leftId);

    int count = 0;
    if (query.exec() && query.next())
    {
        count = query.value(0).toInt();
    }

    // Cache the result
    JunctionCache::instance().setCachedRightIdsCount(junctionTableName, leftId, count);

    return count;
}

inline QList<int> getRightIdsInRange(QSqlDatabase &db, int leftId, const QString &junctionTableName, int offset,
                                     int limit)
{
    QList<int> cachedResult;
    if (JunctionCache::instance().getCachedRightIdsInRange(junctionTableName, leftId, offset, limit, cachedResult))
    {
        return cachedResult;
    }

    QList<int> result;

    const QString sql = QStringLiteral("SELECT right_id FROM %1 WHERE left_id = ? ORDER BY order_ LIMIT ? OFFSET ?")
                            .arg(junctionTableName);

    QSqlQuery query(db);
    query.prepare(sql);
    query.addBindValue(leftId);
    query.addBindValue(limit);
    query.addBindValue(offset);

    if (query.exec())
    {
        while (query.next())
        {
            result.append(query.value(0).toInt());
        }
    }

    // Cache the result
    JunctionCache::instance().setCachedRightIdsInRange(junctionTableName, leftId, offset, limit, result);

    return result;
}
} // namespace Skribisto::Common::Database::JunctionTableOps::OrderedOneToMany
