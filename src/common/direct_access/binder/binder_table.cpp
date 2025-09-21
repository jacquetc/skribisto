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
    const QString now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    for (SCE::Binder b : binders)
    {
        q.prepare("INSERT INTO binder (creation_date, update_date, name) VALUES (:c, :u, :n)"_L1);
        q.bindValue(":c"_L1, now);
        q.bindValue(":u"_L1, now);
        q.bindValue(":n"_L1, b.name);
        if (!q.exec())
        {
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

    return created;
}

QList<SCE::Binder> SCDBinder::BinderTable::updateMany(const QList<SCE::Binder> &binders)
{
    QList<SCE::Binder> updated;
    updated.reserve(binders.size());

    QSqlDatabase db = m_dbSubContext.getConnection();
    QSqlQuery q(db);
    const QString now = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);

    for (const SCE::Binder &b : binders)
    {
        q.prepare("UPDATE binder SET update_date = :u, name = :n WHERE id = :id"_L1);
        q.bindValue(":u"_L1, now);
        q.bindValue(":n"_L1, b.name);
        q.bindValue(":id"_L1, b.id);
        if (q.exec() && q.numRowsAffected() > 0)
        {
            // Handle junction table relationships
            JunctionTableOps::OrderedOneToMany::upsertRightIds(db, b.id, BINDER_BINDER_ITEMS_JUNCTION, b.binderItems);

            updated.append(b);
        }
    }
    return updated;
}

QList<SCE::Binder> SCDBinder::BinderTable::findMany(const QList<int> &ids) const
{
    QList<SCE::Binder> result;
    result.reserve(ids.size());

    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();

    if (ids.isEmpty())
        return result;

    // Build a dynamic IN clause
    QStringList placeholders;
    placeholders.fill("?"_L1, ids.size());
    const QString sql = QStringLiteral("SELECT id, creation_date, update_date, name FROM binder WHERE id IN (%1)")
                            .arg(placeholders.join(","_L1));

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
            int id = q.value(0).toInt();
            QDateTime creationDate = QDateTime::fromString(q.value(1).toString(), Qt::ISODate);
            QDateTime updateDate = QDateTime::fromString(q.value(2).toString(), Qt::ISODate);
            QString name = q.value(3).toString();

            foundIds.append(id);
            binderMap[id] = SCE::Binder(id, creationDate, updateDate, name);
        }

        // Get relationship data for all found IDs
        QHash<int, QList<int>> binderItemsMap =
            JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, foundIds, BINDER_BINDER_ITEMS_JUNCTION);

        // Build result with relationships populated
        for (int id : foundIds)
        {
            SCE::Binder binder = binderMap[id];
            binder.binderItems = binderItemsMap.value(id);
            result.append(binder);
        }
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
    JunctionTableOps::OrderedOneToMany::removeLeftIdsMany(db, ids, BINDER_BINDER_ITEMS_JUNCTION);
    // Clean up junction backward table relationships
    auto leftIds = JunctionTableOps::OrderedOneToMany::getLeftIdMany(db, PROJECT_BINDERS_JUNCTION, ids);
    JunctionTableOps::OrderedOneToMany::removeRightIdsMany(db, leftIds.values(), PROJECT_BINDERS_JUNCTION);

    for (int id : ids)
    {
        q.prepare("DELETE FROM binder WHERE id = :id"_L1);
        q.bindValue(":id"_L1, id);
        if (q.exec() && q.numRowsAffected() > 0)
            removed.append(id);
    }

    return removed;
}

void SCDBinder::BinderTable::setRelationship(int binderId, BinderRelationshipField relationship, QList<int> relatedId)
{
    QSqlDatabase db = m_dbSubContext.getConnection();

    switch (relationship)
    {
    case BinderRelationshipField::BinderItems:
        JunctionTableOps::OrderedOneToMany::upsertRightIds(db, binderId, BINDER_BINDER_ITEMS_JUNCTION, relatedId);
        break;
    }
}

QHash<int, QList<int>> SCDBinder::BinderTable::getRelationshipMany(const QList<int> &binderIds,
                                                                   BinderRelationshipField relationship) const
{
    QSqlDatabase db = const_cast<DbSubContext &>(m_dbSubContext).getConnection();
    QHash<int, QList<int>> result;

    switch (relationship)
    {
    case BinderRelationshipField::BinderItems:
        result = JunctionTableOps::OrderedOneToMany::getRightIdsMany(db, binderIds, BINDER_BINDER_ITEMS_JUNCTION);
        break;
    default:

        throw std::invalid_argument("Unhandled relationship type");
    }

    return result;
}
