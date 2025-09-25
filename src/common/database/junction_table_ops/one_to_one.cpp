#include "one_to_one.h"
#include "junction_cache.h"
#include <QSqlError>
#include <QSqlQuery>

namespace Skribisto::Common::Database::JunctionTableOps
{

QHash<int, std::optional<int>> OneToOne::getRightIdMany(QSqlDatabase &db, const QList<int> &leftIds,
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
    placeholders.fill("?"_L1, leftIds.size());
    const QString sql = QStringLiteral("SELECT left_id, right_id FROM %1 WHERE left_id IN (%2)")
                            .arg(junctionTableName, placeholders.join(","_L1));

    QSqlQuery query(db);
    query.prepare(sql);
    for (int leftId : leftIds)
    {
        query.addBindValue(leftId);
    }

    if (!query.exec())
    {
        qCritical() << "Failed to execute getRightIdMany query:" << query.lastError().text();
        return result;
    }

    while (query.next())
    {
        int leftId = query.value(0).toInt();
        int rightId = query.value(1).toInt();
        result[leftId] = rightId;
    }

    return result;
}

std::optional<int> OneToOne::getRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, std::optional<int>> result = getRightIdMany(db, {leftId}, junctionTableName);
    return result.value(leftId, std::nullopt);
}

QHash<int, bool> OneToOne::removeWithLeftIdMany(QSqlDatabase &db, const QList<int> &leftIds,
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
        qCritical() << "Failed to execute removeWithLeftIdMany query:" << query.lastError().text();
    }

    // Initialize all results based on success
    for (int leftId : leftIds)
    {
        result[leftId] = success;
    }

    return result;
}

bool OneToOne::removeWithLeftId(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QHash<int, bool> result = removeWithLeftIdMany(db, {leftId}, junctionTableName);
    return result.value(leftId, false);
}

QHash<int, QList<int>> OneToOne::upsertRightIdMany(QSqlDatabase &db, const QHash<int, int> &leftIdToRightId,
                                                   const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    if (leftIdToRightId.isEmpty())
    {
        return result;
    }

    // Invalidate cache for affected left IDs
    QList<int> leftIds = leftIdToRightId.keys();
    for (int leftId : leftIds)
    {
        JunctionCache::instance().invalidateLeftId(junctionTableName, leftId);
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
            // If the update failed due to SQL error, log it
            if (query.lastError().isValid())
            {
                qCritical() << "Failed to execute upsertRightIdMany update query:" << query.lastError().text();
            }

            // If no rows were affected, insert new record
            query.prepare(QStringLiteral("INSERT INTO %1 (left_id, right_id) VALUES (?, ?)").arg(junctionTableName));
            query.addBindValue(leftId);
            query.addBindValue(rightId);
            if (!query.exec())
            {
                qCritical() << "Failed to execute upsertRightIdMany insert query:" << query.lastError().text();
            }
        }

        result[leftId] = QList<int>{rightId};
    }

    return result;
}

QHash<int, QList<int>> OneToOne::upsertRightIdMany(QSqlDatabase &db,
                                                   const QHash<int, std::optional<int>> &leftIdToRightId,
                                                   const QString &junctionTableName)
{
    QHash<int, QList<int>> result;

    for (auto it = leftIdToRightId.begin(); it != leftIdToRightId.end(); ++it)
    {
        int leftId = it.key();
        std::optional<int> rightId = it.value();

        if (!rightId.has_value())
        {
            removeWithLeftId(db, leftId, junctionTableName);
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

QList<int> OneToOne::upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName, int right_id)
{
    QHash<int, int> input;
    input[leftId] = right_id;
    QHash<int, QList<int>> result = upsertRightIdMany(db, input, junctionTableName);
    return result.value(leftId, QList<int>());
}

QList<int> OneToOne::upsertRightId(QSqlDatabase &db, int leftId, const QString &junctionTableName,
                                   std::optional<int> right_id)
{
    QHash<int, std::optional<int>> input;
    input[leftId] = right_id;
    QHash<int, QList<int>> result = upsertRightIdMany(db, input, junctionTableName);
    return result.value(leftId, QList<int>());
}

QHash<int, int> OneToOne::getLeftIdMany(QSqlDatabase &db, const QString &junctionTableName, const QList<int> &rightIds)
{
    QHash<int, int> result;

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

    if (!query.exec())
    {
        qCritical() << "Failed to execute getLeftIdMany query:" << query.lastError().text();
        return result;
    }

    while (query.next())
    {
        int rightId = query.value(0).toInt();
        int leftId = query.value(1).toInt();
        result[rightId] = leftId;
    }

    return result;
}

int OneToOne::getLeftId(QSqlDatabase &db, const QString &junctionTableName, int rightId)
{
    QHash<int, int> result = getLeftIdMany(db, junctionTableName, {rightId});
    return result.value(rightId, -1);
}

int OneToOne::getRightIdCount(QSqlDatabase &db, int leftId, const QString &junctionTableName)
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
        qCritical() << "Failed to execute getRightIdCount query:" << query.lastError().text();
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

QList<int> OneToOne::getRightIdInRange(QSqlDatabase &db, int leftId, const QString &junctionTableName)
{
    QList<int> cachedResult;
    if (JunctionCache::instance().getCachedRightIdsInRange(junctionTableName, leftId, 0, 1, cachedResult))
    {
        return cachedResult;
    }

    std::optional<int> result;

    const QString sql = QStringLiteral("SELECT right_id FROM %1 WHERE left_id = ?").arg(junctionTableName);

    QSqlQuery query(db);
    query.prepare(sql);
    query.addBindValue(leftId);

    if (!query.exec())
    {
        qCritical() << "Failed to execute getRightIdInRange query:" << query.lastError().text();
        return {};
    }

    if (query.next())
    {
        result = query.value(0).toInt();
    }

    if (!result.has_value())
    {
        return {};
    }

    QList<int> finalResult;
    if (result.has_value())
    {
        finalResult.append(result.value());
    }

    // Cache the result
    JunctionCache::instance().setCachedRightIdsInRange(junctionTableName, leftId, 0, 1, finalResult);

    return finalResult;
}

} // namespace Skribisto::Common::Database::JunctionTableOps