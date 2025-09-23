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

#include "binder_item_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/one_to_one.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/binder_item.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// forward relationship junction tables
const QString BINDER_ITEM_CONTENTS_JUNCTION = "binder_item_contents_to_content_junction"_L1;
const QString BINDER_ITEM_BINDER_ITEMS_JUNCTION = "binder_item_binder_items_to_binder_item_junction"_L1;
const QString BINDER_ITEM_PARENT_ITEM_JUNCTION = "binder_item_parent_item_to_content_junction"_L1;
// backward relationship junction tables
const QString BINDER_BINDER_ITEMS_JUNCTION = "binder_binder_items_to_binder_item_junction"_L1;

SCDBinderItem::BinderItemTable::BinderItemTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::BinderItem> SCDBinderItem::BinderItemTable::createMany(const QList<SCE::BinderItem> &binderItems)
{
    QList<SCE::BinderItem> created;
    created.reserve(binderItems.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    for (SCE::BinderItem r : binderItems)
    {
        QStringList columnNames;
        QStringList valuePlaceholders;
        // Conditionally include id only if > 0
        if (r.id > 0)
        {
            columnNames << "id"_L1;
            valuePlaceholders << ":id"_L1;
        }

        columnNames << "created_at"_L1
                    << "updated_at"_L1
                    << "title"_L1
                    << "sub_title"_L1
                    << "role"_L1
                    << "dict"_L1;

        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":title"_L1 << ":sub_title"_L1
                          << ":role"_L1 << ":dict"_L1;
        QString sqlString =
            "INSERT INTO binder_item (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

        q.prepare(sqlString);

        // Set timestamps if not provided
        if (r.createdAt.isNull())
            r.createdAt = QDateTime::currentDateTimeUtc();
        if (r.updatedAt.isNull())
            r.updatedAt = r.createdAt;

        if (r.id > 0)
            q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":title"_L1, r.title);
        q.bindValue(":sub_title"_L1, r.subTitle);
        q.bindValue(":role"_L1, r.role);
        q.bindValue(":dict"_L1, r.dict);

        if (!q.exec())
        {
            qCritical() << "Failed to insert binderItem:" << q.lastError().text();
            // If insert fails, skip this row
            continue;
        }
        // Retrieve the auto-generated id
        QSqlQuery idq(db);
        if (idq.exec("SELECT last_insert_rowid()"_L1) && idq.next())
        {
            r.id = idq.value(0).toInt();

            // Handle junction table relationships
            if (!r.contents.isEmpty())
            {
                JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, BINDER_ITEM_CONTENTS_JUNCTION,
                                                                     r.contents);
            }
            if (!r.binderItems.isEmpty())
            {
                JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, BINDER_ITEM_BINDER_ITEMS_JUNCTION,
                                                                   r.binderItems);
            }

            if (r.parent.has_value())
            {
                JunctionTableOps::OneToOne::upsertRightId(db, r.id, BINDER_ITEM_PARENT_ITEM_JUNCTION, r.parent.value());
            }

            created.append(r);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &binderItem : created)
            createdIds.append(binderItem.id);

        using BinderItemCache = Database::TableCache<SCE::BinderItem, BinderItemRelationshipField>;
        BinderItemCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::BinderItem> SCDBinderItem::BinderItemTable::updateMany(const QList<SCE::BinderItem> &binderItems)
{
    QList<SCE::BinderItem> updated;
    updated.reserve(binderItems.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "title = :title"_L1
                << "sub_title = :sub_title"_L1
                << "role = :role"_L1
                << "dict = :dict"_L1;

    QString sqlString = "UPDATE binder_item SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::BinderItem &r : binderItems)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":title"_L1, r.title);
        q.bindValue(":sub_title"_L1, r.subTitle);
        q.bindValue(":role"_L1, r.role);
        q.bindValue(":dict"_L1, r.dict);

        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, r.id, BINDER_ITEM_CONTENTS_JUNCTION, r.contents);
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, r.id, BINDER_ITEM_BINDER_ITEMS_JUNCTION,
                                                               r.binderItems);
            JunctionTableOps::OneToOne::upsertRightId(db, r.id, BINDER_ITEM_PARENT_ITEM_JUNCTION, r.parent);

            updated.append(r);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &binderItem : updated)
            updatedIds.append(binderItem.id);

        using BinderItemCache = Database::TableCache<SCE::BinderItem, BinderItemRelationshipField>;
        BinderItemCache::instance().invalidateEntities(updatedIds);
        BinderItemCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::BinderItem> SCDBinderItem::BinderItemTable::findMany(const QList<int> &ids) const
{
    QList<SCE::BinderItem> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using BinderItemCache = Database::TableCache<SCE::BinderItem, BinderItemRelationshipField>;
    if (BinderItemCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "title"_L1
                       << "sub_title"_L1
                       << "role"_L1
                       << "dict"_L1;
    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM binder_item WHERE id IN (%2)")
                            .arg(selectPlaceholders.join(","_L1), inPlaceholders.join(","_L1));

    QSqlQuery q(db);
    q.prepare(sql);
    for (int id : ids)
        q.addBindValue(id);

    if (q.exec())
    {
        QList<int> foundIds;
        while (q.next())
        {
            foundIds.append(q.value(0).toInt());
            SCE::BinderItem binderItem;
            binderItem.id = q.value(0).toInt();
            binderItem.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            binderItem.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            binderItem.title = q.value(3).toString();
            binderItem.subTitle = q.value(4).toString();
            binderItem.role = q.value(5).toString();
            binderItem.dict = q.value(6).toString();
            result.append(binderItem);
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> projectsMap =
            JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, foundIds, BINDER_ITEM_CONTENTS_JUNCTION);
        QHash<int, QList<int>> binderItemsMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, BINDER_ITEM_BINDER_ITEMS_JUNCTION);
        QHash<int, std::optional<int>> parentMap =
            JunctionTableOps::OneToOne::getRightIdMany(db, foundIds, BINDER_ITEM_PARENT_ITEM_JUNCTION);

        // Build result with relationships populated
        for (auto &binderItem : result)
        {
            binderItem.contents = projectsMap.value(binderItem.id);
            binderItem.binderItems = binderItemsMap.value(binderItem.id);
            binderItem.parent = parentMap.value(binderItem.id);
        }

        // Cache the result
        BinderItemCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDBinderItem::BinderItemTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction table relationships first
    JunctionTableOps::UnorderedOneToMany::removeWithLeftIdsMany(db, ids, BINDER_ITEM_CONTENTS_JUNCTION);
    JunctionTableOps::OrderedOneToMany::removeWithLeftIdsMany(db, ids, BINDER_ITEM_BINDER_ITEMS_JUNCTION);
    JunctionTableOps::OneToOne::removeWithLeftIdMany(db, ids, BINDER_ITEM_PARENT_ITEM_JUNCTION);

    // Clean up junction backward table relationships
    auto rightAndLeftIds = JunctionTableOps::OrderedOneToMany::getLeftIdMany(db, BINDER_BINDER_ITEMS_JUNCTION, ids);
    JunctionTableOps::OrderedOneToMany::removeWithRightIdsMany(db, rightAndLeftIds.values(),
                                                               BINDER_BINDER_ITEMS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM binder_item WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using BinderItemCache = Database::TableCache<SCE::BinderItem, BinderItemRelationshipField>;
        BinderItemCache::instance().invalidateEntities(removed);
        BinderItemCache::instance().invalidateRelationships(removed);
    }

    return removed;
}
void SCDBinderItem::BinderItemTable::setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship,
                                                        QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case BinderItemRelationshipField::Contents:
        JunctionTableOps::UnorderedOneToMany::upsertRightIds(db, binderItemId, BINDER_ITEM_CONTENTS_JUNCTION,
                                                             relatedId);
        break;
    case BinderItemRelationshipField::BinderItems:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, binderItemId, BINDER_ITEM_BINDER_ITEMS_JUNCTION,
                                                           relatedId);

    case BinderItemRelationshipField::ParentItem:
        if (relatedId.size() > 1)
        {
            throw std::invalid_argument("ParentItem relationship can only have one related ID");
        }
        if (relatedId.isEmpty())
        {
            JunctionTableOps::OneToOne::upsertRightId(db, binderItemId, BINDER_ITEM_PARENT_ITEM_JUNCTION, std::nullopt);
        }
        else
        {
            JunctionTableOps::OneToOne::upsertRightId(db, binderItemId, BINDER_ITEM_PARENT_ITEM_JUNCTION,
                                                      std::make_optional(relatedId.first()));
        }
        break;
    }

    // Invalidate cache for relationship changes
    using BinderItemCache = Database::TableCache<SCE::BinderItem, BinderItemRelationshipField>;
    BinderItemCache::instance().invalidateEntity(binderItemId);
    BinderItemCache::instance().invalidateRelationships(binderItemId);
}

QHash<int, QList<int>> SCDBinderItem::BinderItemTable::getRelationshipIdsMany(
    const QList<int> &binderItemIds, BinderItemRelationshipField relationship) const
{
    // Try cache first
    using BinderItemCache = Database::TableCache<SCE::BinderItem, BinderItemRelationshipField>;
    QHash<int, QList<int>> result;
    if (BinderItemCache::instance().getCachedRelationshipData(binderItemIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();

    switch (relationship)
    {
    case BinderItemRelationshipField::Contents:
        result =
            JunctionTableOps::UnorderedOneToMany::getRightIdsMany(db, binderItemIds, BINDER_ITEM_CONTENTS_JUNCTION);
        break;
    case BinderItemRelationshipField::BinderItems:
        result =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, binderItemIds, BINDER_ITEM_BINDER_ITEMS_JUNCTION);
        break;
    case BinderItemRelationshipField::ParentItem: {
        QHash<int, std::optional<int>> tempResult =
            JunctionTableOps::OneToOne::getRightIdMany(db, binderItemIds, BINDER_ITEM_PARENT_ITEM_JUNCTION);
        // Convert std::optional<int> to QList<int> for uniformity
        for (auto it = tempResult.begin(); it != tempResult.end(); ++it)
        {
            if (it.value().has_value())
            {
                result[it.key()] = {it.value().value()};
            }
            else
            {
                result[it.key()] = {};
            }
        }
    }
    break;

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    BinderItemCache::instance().setCachedRelationshipData(binderItemIds, relationship, result);

    return result;
}

int SCDBinderItem::BinderItemTable::getRelationshipIdsCount(int binderItemId, BinderItemRelationshipField relationship)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    int result;

    switch (relationship)
    {
    case BinderItemRelationshipField::Contents:
        result =
            JunctionTableOps::UnorderedOneToMany::getRightIdsCount(db, binderItemId, BINDER_ITEM_CONTENTS_JUNCTION);
        break;
    case BinderItemRelationshipField::BinderItems:
        result =
            JunctionTableOps::OrderedOneToMany::getRightIdsCount(db, binderItemId, BINDER_ITEM_BINDER_ITEMS_JUNCTION);
        break;
    case BinderItemRelationshipField::ParentItem:
        result = JunctionTableOps::OneToOne::getRightIdCount(db, binderItemId, BINDER_ITEM_PARENT_ITEM_JUNCTION);
        break;

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDBinderItem::BinderItemTable::getRelationshipIdsInRange(int binderItemId,
                                                                     BinderItemRelationshipField relationship,
                                                                     int offset, int limit)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QList<int> result;

    switch (relationship)
    {
    case BinderItemRelationshipField::Contents:
        result = JunctionTableOps::UnorderedOneToMany::getRightIdsInRange(db, binderItemId,
                                                                          BINDER_ITEM_CONTENTS_JUNCTION, offset, limit);
        break;
    case BinderItemRelationshipField::BinderItems:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsInRange(
            db, binderItemId, BINDER_ITEM_BINDER_ITEMS_JUNCTION, offset, limit);
        break;
    case BinderItemRelationshipField::ParentItem:
        result = JunctionTableOps::OneToOne::getRightIdInRange(db, binderItemId, BINDER_ITEM_PARENT_ITEM_JUNCTION);
        break;

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}