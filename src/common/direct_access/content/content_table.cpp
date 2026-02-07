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

#include "content_table.h"
#include "database/db_context.h"
#include "database/junction_table_ops/one_to_one.h"
#include "database/junction_table_ops/ordered_one_to_many.h"
#include "database/junction_table_ops/unordered_one_to_many.h"
#include "database/table_cache.h"
#include "entities/content.h"

#include <QDateTime>
#include <QList>
#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlError>
#include <QtSql/QSqlQuery>

namespace SCDContent = Skribisto::Common::DirectAccess::Content;
using namespace Skribisto::Common::Database;
namespace SCE = Skribisto::Common::Entities;

// backward relationship junction tables
const QString BINDER_ITEM_CONTENTS_JUNCTION = "binder_item_contents_to_content_junction"_L1;

SCDContent::ContentTable::ContentTable(DbSubContext &dbSubContext) : m_dbSubContext(dbSubContext)
{
}

QList<SCE::Content> SCDContent::ContentTable::createMany(const QList<SCE::Content> &contents)
{
    QList<SCE::Content> created;
    created.reserve(contents.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    for (SCE::Content r : contents)
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
                    << "role"_L1
                    << "data"_L1;

        valuePlaceholders << ":created_at"_L1 << ":updated_at"_L1 << ":role"_L1 << ":data"_L1;
        QString sqlString =
            "INSERT INTO content (%1) VALUES (%2)"_L1.arg(columnNames.join(","_L1), valuePlaceholders.join(","_L1));

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
        q.bindValue(":role"_L1, r.role);
        q.bindValue(":data"_L1, r.data);

        if (!q.exec())
        {
            qCritical() << "Failed to insert content:" << q.lastError().text() << " SQL:" << sqlString;
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
        for (const auto &content : created)
            createdIds.append(content.id);

        using ContentCache = Database::TableCache<SCE::Content, ContentRelationshipField>;
        ContentCache::instance().invalidateEntities(createdIds);
    }

    return created;
}

QList<SCE::Content> SCDContent::ContentTable::updateMany(const QList<SCE::Content> &contents)
{
    QList<SCE::Content> updated;
    updated.reserve(contents.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    QStringList columnNames;
    columnNames << "created_at = :created_at"_L1
                << "updated_at = :updated_at"_L1
                << "role = :role"_L1
                << "data = :data"_L1;

    QString sqlString = "UPDATE content SET %1 WHERE id = :id"_L1.arg(columnNames.join(","_L1));

    for (const SCE::Content &r : contents)
    {
        q.prepare(sqlString);
        q.bindValue(":id"_L1, r.id);
        q.bindValue(":created_at"_L1, r.createdAt.toString(Qt::ISODate));
        q.bindValue(":updated_at"_L1, r.updatedAt.toString(Qt::ISODate));
        q.bindValue(":role"_L1, r.role);
        q.bindValue(":data"_L1, r.data);

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
        for (const auto &content : updated)
            updatedIds.append(content.id);

        using ContentCache = Database::TableCache<SCE::Content, ContentRelationshipField>;
        ContentCache::instance().invalidateEntities(updatedIds);
    }

    return updated;
}

QList<SCE::Content> SCDContent::ContentTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Content> result;
    result.reserve(ids.size());

    if (ids.isEmpty())
        return result;

    // Try cache first
    using ContentCache = Database::TableCache<SCE::Content, ContentRelationshipField>;
    if (ContentCache::instance().getCachedEntities(ids, result))
    {
        return result;
    }

    QSqlDatabase db = m_dbSubContext.getConnection();

    // Build placeholder for SELECT fields
    QStringList selectPlaceholders;
    selectPlaceholders << "id"_L1
                       << "created_at"_L1
                       << "updated_at"_L1
                       << "role"_L1
                       << "data"_L1;
    // Build a dynamic IN clause
    QStringList inPlaceholders;
    inPlaceholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT %1 FROM content WHERE id IN (%2)")
                            .arg(selectPlaceholders.join(","_L1), inPlaceholders.join(","_L1));

    QSqlQuery q(db);
    q.prepare(sql);
    for (int id : ids)
        q.addBindValue(id);

    if (q.exec())
    {
        while (q.next())
        {
            SCE::Content content;
            content.id = q.value(0).toInt();
            content.createdAt = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            content.updatedAt = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            content.role = q.value(3).toString();
            content.data = q.value(4).toString();
            result.append(content);
        }

        // Cache the result
        ContentCache::instance().setCachedEntities(ids, result);
    }
    return result;
}

QList<int> SCDContent::ContentTable::removeMany(const QList<int> &ids)
{
    QList<int> removed;
    removed.reserve(ids.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);

    // Clean up backward relationship junction table
    JunctionTableOps::UnorderedOneToMany::removeWithRightIdsMany(db, ids, BINDER_ITEM_CONTENTS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM content WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    // Invalidate cache for removed entities
    if (!removed.isEmpty())
    {
        using ContentCache = Database::TableCache<SCE::Content, ContentRelationshipField>;
        ContentCache::instance().invalidateEntities(removed);
    }

    return removed;
}

void SCDContent::ContentTable::setRelationshipIds(int contentId, ContentRelationshipField relationship,
                                                  QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    // Invalidate cache for relationship changes
    using ContentCache = Database::TableCache<SCE::Content, ContentRelationshipField>;
    ContentCache::instance().invalidateEntity(contentId);
    ContentCache::instance().invalidateRelationships(contentId);
}

QHash<int, QList<int>> SCDContent::ContentTable::getRelationshipIdsMany(const QList<int> &contentIds,
                                                                        ContentRelationshipField relationship) const
{
    // Try cache first
    using ContentCache = Database::TableCache<SCE::Content, ContentRelationshipField>;
    QHash<int, QList<int>> result;
    if (ContentCache::instance().getCachedRelationshipData(contentIds, relationship, result))
    {
        return result;
    }

    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {

    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    // Cache the result
    ContentCache::instance().setCachedRelationshipData(contentIds, relationship, result);

    return result;
}

int SCDContent::ContentTable::getRelationshipIdsCount(int contentId, ContentRelationshipField relationship)
{
    QSqlDatabase db = m_dbSubContext.getConnection();
    int result;

    switch (relationship)
    {
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }
    return result;
}
QList<int> SCDContent::ContentTable::getRelationshipIdsInRange(int contentId, ContentRelationshipField relationship,
                                                               int offset, int limit)
{
    QSqlDatabase db = m_dbSubContext.getConnection();
    QList<int> result;

    switch (relationship)
    {

    default:
        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}