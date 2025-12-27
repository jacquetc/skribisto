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
#include <QMutex>
#include <QString>

using namespace Qt::StringLiterals;

namespace Skribisto::Common::Database::JunctionTableOps
{

/**
 * @brief Thread-safe cache for junction table operations
 *
 * This cache stores query results for junction table operations to improve performance.
 * It uses a key-value structure where keys are composed of table name, left ID, and operation type.
 */
class JunctionCache
{
  public:
    struct CacheKey
    {
        QString tableName;
        int leftId;
        QString operation; // "getRightIds", "getRightIdsCount", "getRightIdsInRange"
        int offset = 0;    // for range queries
        int limit = 0;     // for range queries

        bool operator==(const CacheKey &other) const
        {
            return tableName == other.tableName && leftId == other.leftId && operation == other.operation &&
                   offset == other.offset && limit == other.limit;
        }
    };

    struct CacheValue
    {
        QList<int> rightIds;
        int count = 0;
        bool isValid = false;
    };

    static JunctionCache &instance()
    {
        static JunctionCache cache;
        return cache;
    }

    // Get cached right IDs for a left ID
    bool getCachedRightIds(const QString &tableName, int leftId, QList<int> &result)
    {
        QMutexLocker locker(&m_mutex);
        CacheKey key{tableName, leftId, "getRightIds"_L1, 0, 0};

        auto it = m_cache.find(qHash(key));
        if (it != m_cache.end() && it->isValid)
        {
            result = it->rightIds;
            return true;
        }
        return false;
    }

    // Cache right IDs for a left ID
    void setCachedRightIds(const QString &tableName, int leftId, const QList<int> &rightIds)
    {
        QMutexLocker locker(&m_mutex);
        CacheKey key{tableName, leftId, "getRightIds"_L1, 0, 0};
        CacheValue value;
        value.rightIds = rightIds;
        value.count = rightIds.size();
        value.isValid = true;
        m_cache[qHash(key)] = value;
    }

    // Get cached count for a left ID
    bool getCachedRightIdsCount(const QString &tableName, int leftId, int &result)
    {
        QMutexLocker locker(&m_mutex);
        CacheKey key{tableName, leftId, "getRightIdsCount"_L1, 0, 0};

        auto it = m_cache.find(qHash(key));
        if (it != m_cache.end() && it->isValid)
        {
            result = it->count;
            return true;
        }
        return false;
    }

    // Cache count for a left ID
    void setCachedRightIdsCount(const QString &tableName, int leftId, int count)
    {
        QMutexLocker locker(&m_mutex);
        CacheKey key{tableName, leftId, "getRightIdsCount"_L1, 0, 0};
        CacheValue value;
        value.count = count;
        value.isValid = true;
        m_cache[qHash(key)] = value;
    }

    // Get cached range results
    bool getCachedRightIdsInRange(const QString &tableName, int leftId, int offset, int limit, QList<int> &result)
    {
        QMutexLocker locker(&m_mutex);
        CacheKey key{tableName, leftId, "getRightIdsInRange"_L1, offset, limit};

        auto it = m_cache.find(qHash(key));
        if (it != m_cache.end() && it->isValid)
        {
            result = it->rightIds;
            return true;
        }
        return false;
    }

    // Cache range results
    void setCachedRightIdsInRange(const QString &tableName, int leftId, int offset, int limit,
                                  const QList<int> &rightIds)
    {
        QMutexLocker locker(&m_mutex);
        CacheKey key{tableName, leftId, "getRightIdsInRange"_L1, offset, limit};
        CacheValue value;
        value.rightIds = rightIds;
        value.isValid = true;
        m_cache[qHash(key)] = value;
    }

    // Remove all cached data for a specific left ID
    void invalidateLeftId(const QString &tableName, int leftId)
    {
        QMutexLocker locker(&m_mutex);
        // Remove all cache entries for this leftId
        auto it = m_cache.begin();
        while (it != m_cache.end())
        {
            CacheKey key = keyFromHash(it.key());
            if (key.tableName == tableName && key.leftId == leftId)
            {
                m_keyMap.remove(it.key());
                it = m_cache.erase(it);
            }
            else
            {
                ++it;
            }
        }
    }

    // Remove all cached data for a specific table
    void invalidateTable(const QString &tableName)
    {
        QMutexLocker locker(&m_mutex);
        auto it = m_cache.begin();
        while (it != m_cache.end())
        {
            CacheKey key = keyFromHash(it.key());
            if (key.tableName == tableName)
            {
                m_keyMap.remove(it.key());
                it = m_cache.erase(it);
            }
            else
            {
                ++it;
            }
        }
    }

    // Clear all cached data
    void clear()
    {
        QMutexLocker locker(&m_mutex);
        m_cache.clear();
        m_keyMap.clear();
    }

  private:
    QHash<uint, CacheValue> m_cache;
    QMutex m_mutex;
    QHash<uint, CacheKey> m_keyMap; // To reverse lookup keys from hash

    CacheKey keyFromHash(uint hash) const
    {
        return m_keyMap.value(hash);
    }

    uint qHash(const CacheKey &key)
    {
        uint hash = ::qHash(key.tableName) ^ ::qHash(key.leftId) ^ ::qHash(key.operation) ^ ::qHash(key.offset) ^
                    ::qHash(key.limit);
        m_keyMap[hash] = key; // Store reverse lookup
        return hash;
    }
};

} // namespace Skribisto::Common::Database::JunctionTableOps