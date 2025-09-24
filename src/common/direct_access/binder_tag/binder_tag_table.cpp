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

#include "binder_tag_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/one_to_one.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/binder_tag.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// forward relationship junction tables

// backward relationship junction tables
const QString BINDER_ITEM_BINDER_TAGS_JUNCTION = "binder_item_binder_tags_to_binder_tag_junction"_L1;

SCDBinderTag::BinderTagTable::BinderTagTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::BinderTag> SCDBinderTag::BinderTagTable::createMany(const QList<SCE::BinderTag> &binderTags)
{
    QList<SCE::BinderTag> created;
    created.reserve(binderTags.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    for (SCE::BinderTag r : binderTags)
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
                    << "name"_L1
                    << "color"_L1
                    << "text_color"_L1;

        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":name"_L1 << ":color"_L1 << ":text_color"_L1;
        QString sqlString =
            "INSERT INTO binder_tag (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

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
        q.bindValue(":name"_L1, r.name);
        q.bindValue(":color"_L1, r.color);
        q.bindValue(":text_color"_L1, r.textColor);

        if (!q.exec())
        {
            qCritical() << "Failed to insert binderTag:" << q.lastError().text() << " SQL:" << sqlString;
            // If insert fails, skip this row
            continue;
        }
        // Retrieve the auto-generated id
        QSqlQuery idq(db);
        if (idq.exec("SELECT last_insert_rowid()"_L1) && idq.next())
        {
            r.id = idq.value(0).toInt();
            created.append(r);
        }
    }

    // Invalidate cache for created entities
    if (!created.isEmpty())
    {
        QList<int> createdIds;
        createdIds.reserve(created.size());
        for (const auto &binderTag : created)
            createdIds.append(binderTag.id);

        using BinderTagCache = Database::TableCache<SCE::BinderTag, BinderTagRelationshipField>;
        BinderTagCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::BinderTag> SCDBinderTag::BinderTagTable::updateMany(const QList<SCE::BinderTag> &binderTags)
{
    QList<SCE::BinderTag> updated;
    updated.reserve(binderTags.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "id = :id"_L1
                << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "name = :name"_L1
                << "color = :color"_L1
                << "text_color = :text_color"_L1;

    QString sqlString = "UPDATE binder_tag SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::BinderTag &r : binderTags)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":name"_L1, r.name);
        q.bindValue(":color"_L1, r.color);
        q.bindValue(":text_color"_L1, r.textColor);

        if (q.exec() && q.numRowsAffected() > 0)
        {
            updated.append(r);
        }
    }

    // Invalidate cache for updated entities
    if (!updated.isEmpty())
    {
        QList<int> updatedIds;
        updatedIds.reserve(updated.size());
        for (const auto &binderTag : updated)
            updatedIds.append(binderTag.id);

        using BinderTagCache = Database::TableCache<SCE::BinderTag, BinderTagRelationshipField>;
        BinderTagCache::instance().invalidateEntities(updatedIds);
        BinderTagCache::instance().invalidateRelationships(updatedIds);
    }

    return updated;
}

QList<SCE::BinderTag> SCDBinderTag::BinderTagTable::findMany(const QList<int> &ids) const
{
    QList<SCE::BinderTag> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using BinderTagCache = Database::TableCache<SCE::BinderTag, BinderTagRelationshipField>;
    if (BinderTagCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "name"_L1
                       << "color"_L1
                       << "text_color"_L1;

    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM binder_tag WHERE id IN (%2)")
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
            SCE::BinderTag binderTag;
            binderTag.id = q.value(0).toInt();
            binderTag.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            binderTag.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            binderTag.name = q.value(3).toString();
            binderTag.color = q.value(4).toString();
            binderTag.textColor = q.value(5).toString();
            result.append(binderTag);
        }

        // Cache the result
        BinderTagCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDBinderTag::BinderTagTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up junction backward table relationships
    JunctionTableOps::OrderedOneToMany::removeWithRightIdsMany(db, ids, BINDER_ITEM_BINDER_TAGS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM binder_tag WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using BinderTagCache = Database::TableCache<SCE::BinderTag, BinderTagRelationshipField>;
        BinderTagCache::instance().invalidateEntities(removed);
        BinderTagCache::instance().invalidateRelationships(removed);
    }

    return removed;
}
void SCDBinderTag::BinderTagTable::setRelationshipIds(int binderTagId, BinderTagRelationshipField relationship,
                                                      QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    // Invalidate cache for relationship changes
    using BinderTagCache = Database::TableCache<SCE::BinderTag, BinderTagRelationshipField>;
    BinderTagCache::instance().invalidateEntity(binderTagId);
    BinderTagCache::instance().invalidateRelationships(binderTagId);
}

QHash<int, QList<int>> SCDBinderTag::BinderTagTable::getRelationshipIdsMany(
    const QList<int> &binderTagIds, BinderTagRelationshipField relationship) const
{
    // Try cache first
    using BinderTagCache = Database::TableCache<SCE::BinderTag, BinderTagRelationshipField>;
    QHash<int, QList<int>> result;
    if (BinderTagCache::instance().getCachedRelationshipData(binderTagIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();

    switch (relationship)
    {
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    BinderTagCache::instance().setCachedRelationshipData(binderTagIds, relationship, result);

    return result;
}

int SCDBinderTag::BinderTagTable::getRelationshipIdsCount(int binderTagId, BinderTagRelationshipField relationship)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    int result;

    switch (relationship)
    {
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDBinderTag::BinderTagTable::getRelationshipIdsInRange(int binderTagId,
                                                                   BinderTagRelationshipField relationship, int offset,
                                                                   int limit)
{
    QSqlDatabase db = const_cast<Database::DbSubContext &>(m_dbSubContext).getConnection();
    QList<int> result;

    switch (relationship)
    {

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}