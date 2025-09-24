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

#include "binder_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/binder.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// forward relationship junction tables
const QString BINDER_BINDER_ITEMS_JUNCTION = "binder_binder_items_to_binder_item_junction"_L1;

// backward relationship junction tables
const QString PROJECT_BINDERS_JUNCTION = "project_binders_to_binder_junction"_L1;

SCDBinder::BinderTable::BinderTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::Binder> SCDBinder::BinderTable::createMany(const QList<SCE::Binder> &binders)
{
    QList<SCE::Binder> created;
    created.reserve(binders.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    for (SCE::Binder b : binders)
    {
        QStringList columnNames;
        QStringList valuePlaceholders;
        // Conditionally include id only if > 0
        if (b.id > 0)
        {
            columnNames << "id"_L1;
            valuePlaceholders << ":id"_L1;
        }

        columnNames << "created_at"_L1
                    << "updated_at"_L1
                    << "name"_L1;

        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":name"_L1;
        QString sqlString =
            "INSERT INTO binder (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

        q.prepare(sqlString);

        // Set timestamps if not provided
        if (b.createdAt.isNull())
            b.createdAt = QDateTime::currentDateTimeUtc();
        if (b.updatedAt.isNull())
            b.updatedAt = b.createdAt;

        if (b.id > 0)
            q.bindValue(":id"_L1, b.id);
        q.bindValue(":created_at"_L1, b.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, b.updatedAt.toString(Qt::ISODate));
        q.bindValue(":name"_L1, b.name);

        if (!q.exec())
        {
            qCritical() << "Failed to insert binder:" << q.lastError().text() << " SQL:" << sqlString;

            // If insert fails, skip this row
            continue;
        }
        // Retrieve the auto-generated id
        QSqlQuery idq(db);
        if (idq.exec("SELECT last_insert_rowid()"_L1) && idq.next())
        {
            b.id = idq.value(0).toInt();

            // Handle junction table relationships
            if (!b.binderItems.isEmpty())
            {
                JunctionTableOps::OrderedOneToMany::upsertRightIds(db, b.id, BINDER_BINDER_ITEMS_JUNCTION,
                                                                   b.binderItems);
            }

            created.append(b);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &binder : created)
            createdIds.append(binder.id);

        using BinderCache = Database::TableCache<SCE::Binder, BinderRelationshipField>;
        BinderCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::Binder> SCDBinder::BinderTable::updateMany(const QList<SCE::Binder> &binders)
{
    QList<SCE::Binder> updated;
    updated.reserve(binders.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "name = :name"_L1;

    QString sqlString = "UPDATE binder SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::Binder &b : binders)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, b.id);
        q.bindValue(":created_at"_L1, b.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, b.updatedAt.toString(Qt::ISODate));
        q.bindValue(":name"_L1, b.name);

        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, b.id, BINDER_BINDER_ITEMS_JUNCTION, b.binderItems);

            updated.append(b);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &binder : updated)
            updatedIds.append(binder.id);

        using BinderCache = Database::TableCache<SCE::Binder, BinderRelationshipField>;
        BinderCache::instance().invalidateEntities(updatedIds);
        BinderCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::Binder> SCDBinder::BinderTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Binder> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using BinderCache = Database::TableCache<SCE::Binder, BinderRelationshipField>;
    if (BinderCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "name"_L1;

    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM binder WHERE id IN (%2)")
                            .arg(selectPlaceholders.join(","_L1), inPlaceholders.join(","_L1));

    QSqlQuery q(db);
    q.prepare(sql);
    for (int id : ids)
        q.addBindValue(id);

    if (q.exec())
    {
        QList<int> foundIds;
        QHash<int, SCE::Binder> binderMap;
        while (q.next())
        {
            foundIds.append(q.value(0).toInt());
            SCE::Binder binder;
            binder.id = q.value(0).toInt();
            binder.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            binder.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            binder.name = q.value(3).toString();
            result.append(binder);
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> binderItemsMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, BINDER_BINDER_ITEMS_JUNCTION);

        // Build result with relationships populated
        for (auto &binder : result)
        {
            binder.binderItems = binderItemsMap.value(binder.id);
        }

        // Cache the result
        BinderCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDBinder::BinderTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction table relationships first
    JunctionTableOps::OrderedOneToMany::removeWithLeftIdsMany(db, ids, BINDER_BINDER_ITEMS_JUNCTION);
    // Clean up junction backward table relationships
    JunctionTableOps::OrderedOneToMany::removeWithRightIdsMany(db, ids, PROJECT_BINDERS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM binder WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using BinderCache = Database::TableCache<SCE::Binder, BinderRelationshipField>;
        BinderCache::instance().invalidateEntities(removed);
        BinderCache::instance().invalidateRelationships(removed);
    }

    return removed;
}

void SCDBinder::BinderTable::setRelationshipIds(int binderId, BinderRelationshipField relationship,
                                                QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case BinderRelationshipField::BinderItems:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, binderId, BINDER_BINDER_ITEMS_JUNCTION, relatedId);
        break;
    }

    // Invalidate cache for relationship changes
    using BinderCache = Database::TableCache<SCE::Binder, BinderRelationshipField>;
    BinderCache::instance().invalidateEntity(binderId);
    BinderCache::instance().invalidateRelationships(binderId);
}

QHash<int, QList<int>> SCDBinder::BinderTable::getRelationshipIdsMany(const QList<int> &binderIds,
                                                                      BinderRelationshipField relationship) const
{
    // Try cache first
    using BinderCache = Database::TableCache<SCE::Binder, BinderRelationshipField>;
    QHash<int, QList<int>> result;
    if (BinderCache::instance().getCachedRelationshipData(binderIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    switch (relationship)
    {
    case BinderRelationshipField::BinderItems:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, binderIds, BINDER_BINDER_ITEMS_JUNCTION);
        break;
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    BinderCache::instance().setCachedRelationshipData(binderIds, relationship, result);

    return result;
}

int SCDBinder::BinderTable::getRelationshipIdsCount(int binderId, BinderRelationshipField relationship)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    int result;

    switch (relationship)
    {
    case BinderRelationshipField::BinderItems:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsCount(db, binderId, BINDER_BINDER_ITEMS_JUNCTION);
        break;
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDBinder::BinderTable::getRelationshipIdsInRange(int binderId, BinderRelationshipField relationship,
                                                             int offset, int limit)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QList<int> result;

    switch (relationship)
    {
    case BinderRelationshipField::BinderItems:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsInRange(db, binderId, BINDER_BINDER_ITEMS_JUNCTION,
                                                                        offset, limit);
        break;

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}