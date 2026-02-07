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

#include "ordered_many_to_many.h"
#include "junction_cache.h"
#include <QSet>
#include <QSqlDriver>
#include <QSqlError>
#include <QSqlQuery>

namespace Skribisto::Common::Database::JunctionTableOps
{
constexpr int ORDER_GAP = 1000;

QHash<int, QList<int>> OrderedManyToMany::getRightIdsMany(QSqlDatabase &db, const QList<int> &leftIds,
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

    // Build dynamic IN clause with ORDER BY
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

    if (!query.exec())
    {
        qCritical() << "Failed to execute getRightIdsMany query:" << query.lastError().text();
        return result;
    }

    while (query.next())
    {
        int leftId = query.value(0).toInt();
        int rightId = query.value(1).toInt();
        result[leftId].append(rightId);
    }

    return result;
}

QList<int> OrderedManyToMany::getRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName)
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

QHash<int, bool> OrderedManyToMany::removeWithLeftIdsMany(QSqlDatabase &db, const QList<int> &leftIds,
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

    if (!success)
    {
        qCritical() << "Failed to execute removeWithLeftIdsMany query:" << query.lastError().text();
    }

    // Initialize all results based on success
    for (int leftId : leftIds)
    {
        result[leftId] = success;
    }

    return result;
}

bool OrderedManyToMany::removeWithLeftIds(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeWithLeftIdsMany(db, {leftId}, junctionTableName);
    return result.value(leftId, false);
}

QHash<int, bool> OrderedManyToMany::removeWithRightIdsMany(QSqlDatabase &db, const QList<int> &rightIds,
                                                           const QString &junctionTableName)
{
    QHash<int, bool> result;

    if (rightIds.isEmpty())
    {
        return result;
    }

    // Get affected left IDs for cache invalidation
    QMap<int, QList<int>> leftIdsMap = getLeftIdsMany(db, junctionTableName, rightIds);
    QSet<int> uniqueLeftIds;
    for (const QList<int> &leftIds : leftIdsMap.values())
    {
        for (int leftId : leftIds)
        {
            uniqueLeftIds.insert(leftId);
        }
    }
    for (int leftId : uniqueLeftIds)
    {
        JunctionCache::instance().invalidateLeftId(junctionTableName, leftId);
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

    if (!success)
    {
        qCritical() << "Failed to execute removeWithRightIdsMany query:" << query.lastError().text();
    }

    // Initialize all results based on success
    for (int rightId : rightIds)
    {
        result[rightId] = success;
    }

    return result;
}

bool OrderedManyToMany::removeWithRightIds(QSqlDatabase &db, int rightId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeWithRightIdsMany(db, {rightId}, junctionTableName);
    return result.value(rightId, false);
}

QHash<int, QList<int>> OrderedManyToMany::upsertRightIdsMany(QSqlDatabase &db,
                                                             const QHash<int, QList<int>> &leftIdToRightIds,
                                                             const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    if (leftIdToRightIds.isEmpty())
    {
        return result;
    }

    bool transactionStarted = false;
    if (!db.driver()->hasFeature(QSqlDriver::Transactions) || !db.transaction())
    {
        qWarning() << "Failed to start transaction for upsertRightIdsMany";
    }
    else
    {
        transactionStarted = true;
    }

    // Invalidate cache for affected left IDs
    QList<int> leftIds = leftIdToRightIds.keys();
    for (int leftId : leftIds)
    {
        JunctionCache::instance().invalidateLeftId(junctionTableName, leftId);
    }

    // First, remove all existing relationships for these leftIds
    removeWithLeftIdsMany(db, leftIds, junctionTableName);

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
            if (!insertQuery.exec())
            {
                qCritical() << "Failed to execute upsertRightIdsMany insert query:" << insertQuery.lastError().text();
            }
        }

        result[leftId] = rightIds;
    }

    if (transactionStarted)
    {
        if (!db.commit())
        {
            qCritical() << "Failed to commit transaction for upsertRightIdsMany:" << db.lastError().text();
            db.rollback();
        }
    }

    return result;
}

QList<int> OrderedManyToMany::upsertRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                             const QList<int> &rightIds)
{
    QHash<int, QList<int>> input;
    input[leftId] = rightIds;
    QHash<int, QList<int>> result = upsertRightIdsMany(db, input, junctionTableName);
    return result.value(leftId, QList<int>());
}

QList<int> OrderedManyToMany::upsertRightIds(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                             const std::optional<QList<int>> &rightIds)
{
    if (!rightIds.has_value())
    {
        removeWithLeftIds(db, leftId, junctionTableName);
        return {};
    }
    return upsertRightIds(db, leftId, junctionTableName, rightIds.value());
}

QMap<int, QList<int>> OrderedManyToMany::getLeftIdsMany(QSqlDatabase &db, const QString &junctionTableName,
                                                        const QList<int> &rightIds)
{
    QMap<int, QList<int>> result;

    if (rightIds.isEmpty())
    {
        return result;
    }

    // Initialize empty lists for all rightIds
    for (int rightId : rightIds)
    {
        result[rightId] = QList<int>();
    }

    // Build dynamic IN clause with ORDER BY for ordered results from right to left
    QStringList placeholders;
    placeholders.fill("?"_L1, rightIds.size());
    const QString sql =
        QStringLiteral("SELECT right_id, left_id FROM %1 WHERE right_id IN (%2) ORDER BY right_id, order_")
            .arg(junctionTableName, placeholders.join(","_L1));

    QSqlQuery query(db);
    query.prepare(sql);
    for (int rightId : rightIds)
    {
        query.addBindValue(rightId);
    }

    if (!query.exec())
    {
        qCritical() << "Failed to execute getLeftIdsMany query:" << query.lastError().text();
        return result;
    }

    while (query.next())
    {
        int rightId = query.value(0).toInt();
        int leftId = query.value(1).toInt();
        result[rightId].append(leftId);
    }

    return result;
}

QList<int> OrderedManyToMany::getLeftIds(QSqlDatabase &db, const QString &junctionTableName, int rightId)
{
    QMap<int, QList<int>> result = getLeftIdsMany(db, junctionTableName, {rightId});
    return result.value(rightId, QList<int>());
}

int OrderedManyToMany::getRightIdsCount(QSqlDatabase &db, int leftId, const QString &junctionTableName)
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
    if (!query.exec())
    {
        qCritical() << "Failed to execute getRightIdsCount query:" << query.lastError().text();
        return count;
    }

    if (query.next())
    {
        count = query.value(0).toInt();
    }

    // Cache the result
    JunctionCache::instance().setCachedRightIdsCount(junctionTableName, leftId, count);

    return count;
}

QList<int> OrderedManyToMany::getRightIdsInRange(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                                 int offset, int limit)
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

    if (!query.exec())
    {
        qCritical() << "Failed to execute getRightIdsInRange query:" << query.lastError().text();
        return result;
    }

    while (query.next())
    {
        result.append(query.value(0).toInt());
    }

    // Cache the result
    JunctionCache::instance().setCachedRightIdsInRange(junctionTableName, leftId, offset, limit, result);

    return result;
}

} // namespace Skribisto::Common::Database::JunctionTableOps
