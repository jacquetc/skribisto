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

#include <QDateTime>
#include <QHash>
#include <QList>
#include <QMutex>
#include <QSet>
#include <QString>

namespace Skribisto::Common::Database
{

/**
 * @brief Generic thread-safe cache template for table operations
 *
 * This template provides caching functionality for any table type with CRUD operations.
 * It caches entities, relationship data, counts, and range queries.
 *
 * @tparam EntityType The entity type (e.g., Root, Work, Binder)
 * @tparam RelationshipFieldType The relationship field enum type (e.g., RootRelationshipField)
 */
template <typename EntityType, typename RelationshipFieldType> class TableCache
{
  public:
    struct EntityCacheKey
    {
        QList<int> ids;

        bool operator==(const EntityCacheKey &other) const
        {
            return ids == other.ids;
        }

        friend uint qHash(const EntityCacheKey &key, uint seed = 0)
        {
            uint hash = seed;
            for (int id : key.ids)
            {
                hash = ::qHashMulti(hash, id);
            }
            return hash;
        }
    };

    struct RelationshipCacheKey
    {
        QList<int> entityIds;
        RelationshipFieldType relationshipType;
        QString operation; // "getMany", "getCount", "getRange"
        int entityId = 0;  // for single operations
        int offset = 0;    // for range queries
        int limit = 0;     // for range queries

        bool operator==(const RelationshipCacheKey &other) const
        {
            return entityIds == other.entityIds && relationshipType == other.relationshipType &&
                   operation == other.operation && entityId == other.entityId && offset == other.offset &&
                   limit == other.limit;
        }

        friend uint qHash(const RelationshipCacheKey &key, uint seed = 0)
        {
            uint hash = ::qHashMulti(seed, static_cast<int>(key.relationshipType), key.operation, key.entityId,
                                     key.offset, key.limit);

            for (int id : key.entityIds)
            {
                hash = ::qHashMulti(hash, id);
            }

            return hash;
        }
    };

    struct EntityCacheValue
    {
        QList<EntityType> entities;
        QDateTime timestamp;
        bool isValid = false;
    };

    struct RelationshipCacheValue
    {
        QHash<int, QList<int>> relationshipData;
        QList<int> rangeData;
        int count = 0;
        QDateTime timestamp;
        bool isValid = false;
    };

    static TableCache &instance()
    {
        static TableCache cache;
        return cache;
    }

    // Entity caching methods
    bool getCachedEntities(const QList<int> &ids, QList<EntityType> &result)
    {
        QMutexLocker locker(&m_mutex);

        // Sort IDs for consistent cache key
        QList<int> sortedIds = ids;
        std::sort(sortedIds.begin(), sortedIds.end());

        EntityCacheKey key;
        key.ids = sortedIds;

        auto it = m_entityCache.find(key);

        if (it != m_entityCache.end() && it->isValid && !isExpired(it->timestamp, 30))
        {
            result = it->entities;
            return true;
        }

        return false;
    }

    void setCachedEntities(const QList<int> &ids, const QList<EntityType> &entities)
    {
        QMutexLocker locker(&m_mutex);

        // Sort IDs for consistent cache key
        QList<int> sortedIds = ids;
        std::sort(sortedIds.begin(), sortedIds.end());

        EntityCacheKey key;
        key.ids = sortedIds;

        EntityCacheValue value;
        value.entities = entities;
        value.timestamp = QDateTime::currentDateTimeUtc();
        value.isValid = true;

        m_entityCache[key] = value;
    }

    // Relationship caching methods
    bool getCachedRelationshipData(const QList<int> &entityIds, RelationshipFieldType relationshipType,
                                   QHash<int, QList<int>> &result)
    {
        QMutexLocker locker(&m_mutex);

        // Sort entity IDs for consistent cache key
        QList<int> sortedEntityIds = entityIds;
        std::sort(sortedEntityIds.begin(), sortedEntityIds.end());

        RelationshipCacheKey key;
        key.entityIds = sortedEntityIds;
        key.relationshipType = relationshipType;
        key.operation = "getMany"_L1;

        auto it = m_relationshipCache.find(key);

        if (it != m_relationshipCache.end() && it->isValid && !isExpired(it->timestamp, 30))
        {
            result = it->relationshipData;
            return true;
        }

        return false;
    }

    void setCachedRelationshipData(const QList<int> &entityIds, RelationshipFieldType relationshipType,
                                   const QHash<int, QList<int>> &relationshipData)
    {
        QMutexLocker locker(&m_mutex);

        // Sort entity IDs for consistent cache key
        QList<int> sortedEntityIds = entityIds;
        std::sort(sortedEntityIds.begin(), sortedEntityIds.end());

        RelationshipCacheKey key;
        key.entityIds = sortedEntityIds;
        key.relationshipType = relationshipType;
        key.operation = "getMany"_L1;

        RelationshipCacheValue value;
        value.relationshipData = relationshipData;
        value.timestamp = QDateTime::currentDateTimeUtc();
        value.isValid = true;

        m_relationshipCache[key] = value;
    }

    bool getCachedRelationshipCount(int entityId, RelationshipFieldType relationshipType, int &result)
    {
        QMutexLocker locker(&m_mutex);

        RelationshipCacheKey key;
        key.entityId = entityId;
        key.relationshipType = relationshipType;
        key.operation = "getCount"_L1;

        auto it = m_relationshipCache.find(key);

        if (it != m_relationshipCache.end() && it->isValid && !isExpired(it->timestamp, 30))
        {
            result = it->count;
            return true;
        }

        return false;
    }

    void setCachedRelationshipCount(int entityId, RelationshipFieldType relationshipType, int count)
    {
        QMutexLocker locker(&m_mutex);

        RelationshipCacheKey key;
        key.entityId = entityId;
        key.relationshipType = relationshipType;
        key.operation = "getCount"_L1;

        RelationshipCacheValue value;
        value.count = count;
        value.timestamp = QDateTime::currentDateTimeUtc();
        value.isValid = true;

        m_relationshipCache[key] = value;
    }

    bool getCachedRelationshipRange(int entityId, RelationshipFieldType relationshipType, int offset, int limit,
                                    QList<int> &result)
    {
        QMutexLocker locker(&m_mutex);

        RelationshipCacheKey key;
        key.entityId = entityId;
        key.relationshipType = relationshipType;
        key.operation = "getRange"_L1;
        key.offset = offset;
        key.limit = limit;

        auto it = m_relationshipCache.find(key);

        if (it != m_relationshipCache.end() && it->isValid && !isExpired(it->timestamp, 30))
        {
            result = it->rangeData;
            return true;
        }

        return false;
    }

    void setCachedRelationshipRange(int entityId, RelationshipFieldType relationshipType, int offset, int limit,
                                    const QList<int> &rangeData)
    {
        QMutexLocker locker(&m_mutex);

        RelationshipCacheKey key;
        key.entityId = entityId;
        key.relationshipType = relationshipType;
        key.operation = "getRange"_L1;
        key.offset = offset;
        key.limit = limit;

        RelationshipCacheValue value;
        value.rangeData = rangeData;
        value.timestamp = QDateTime::currentDateTimeUtc();
        value.isValid = true;

        m_relationshipCache[key] = value;
    }

    // Invalidation methods
    void invalidateEntity(int entityId)
    {
        QMutexLocker locker(&m_mutex);

        // Remove all entity cache entries that contain this entityId
        auto it = m_entityCache.begin();
        while (it != m_entityCache.end())
        {
            if (it.key().ids.contains(entityId))
            {
                it = m_entityCache.erase(it);
            }
            else
            {
                ++it;
            }
        }
    }

    void invalidateEntities(const QList<int> &entityIds)
    {
        QMutexLocker locker(&m_mutex);

        QSet<int> entityIdSet(entityIds.begin(), entityIds.end());

        // Remove all entity cache entries that contain any of these entityIds
        auto it = m_entityCache.begin();
        while (it != m_entityCache.end())
        {
            bool hasOverlap = false;
            for (int id : it.key().ids)
            {
                if (entityIdSet.contains(id))
                {
                    hasOverlap = true;
                    break;
                }
            }

            if (hasOverlap)
            {
                it = m_entityCache.erase(it);
            }
            else
            {
                ++it;
            }
        }
    }

    void invalidateRelationships(int entityId)
    {
        QMutexLocker locker(&m_mutex);

        // Remove all relationship cache entries for this entityId
        auto it = m_relationshipCache.begin();
        while (it != m_relationshipCache.end())
        {
            if (it.key().entityId == entityId || it.key().entityIds.contains(entityId))
            {
                it = m_relationshipCache.erase(it);
            }
            else
            {
                ++it;
            }
        }
    }

    void invalidateRelationships(const QList<int> &entityIds)
    {
        QMutexLocker locker(&m_mutex);

        QSet<int> entityIdSet(entityIds.begin(), entityIds.end());

        // Remove all relationship cache entries for these entityIds
        auto it = m_relationshipCache.begin();
        while (it != m_relationshipCache.end())
        {
            bool hasMatch = entityIdSet.contains(it.key().entityId);

            if (!hasMatch)
            {
                for (int id : it.key().entityIds)
                {
                    if (entityIdSet.contains(id))
                    {
                        hasMatch = true;
                        break;
                    }
                }
            }

            if (hasMatch)
            {
                it = m_relationshipCache.erase(it);
            }
            else
            {
                ++it;
            }
        }
    }

    void invalidateAll()
    {
        QMutexLocker locker(&m_mutex);
        m_entityCache.clear();
        m_relationshipCache.clear();
    }

    // Cleanup methods
    void cleanupExpired(int maxAgeMinutes = 30)
    {
        QMutexLocker locker(&m_mutex);

        // Clean up expired entity cache entries
        auto entityIt = m_entityCache.begin();
        while (entityIt != m_entityCache.end())
        {
            if (isExpired(entityIt->timestamp, maxAgeMinutes))
            {
                entityIt = m_entityCache.erase(entityIt);
            }
            else
            {
                ++entityIt;
            }
        }

        // Clean up expired relationship cache entries
        auto relationshipIt = m_relationshipCache.begin();
        while (relationshipIt != m_relationshipCache.end())
        {
            if (isExpired(relationshipIt->timestamp, maxAgeMinutes))
            {
                relationshipIt = m_relationshipCache.erase(relationshipIt);
            }
            else
            {
                ++relationshipIt;
            }
        }
    }

    void clear()
    {
        invalidateAll();
    }

  private:
    QHash<EntityCacheKey, EntityCacheValue> m_entityCache;
    QHash<RelationshipCacheKey, RelationshipCacheValue> m_relationshipCache;
    QMutex m_mutex;

    bool isExpired(const QDateTime &timestamp, int maxAgeMinutes) const
    {
        return timestamp.addSecs(maxAgeMinutes * 60) < QDateTime::currentDateTimeUtc();
    }
};

} // namespace Skribisto::Common::Database
