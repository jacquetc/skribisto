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
#include <QList>
#include <QString>
#include <QStringLiteral>

namespace Skribisto::Common::DirectAccess::Binder
{

inline QString getSqlTableDefinition()
{
    return QStringLiteral(
        "CREATE TABLE IF NOT EXISTS binder ("
        "    id INTEGER PRIMARY KEY ON CONFLICT ROLLBACK AUTOINCREMENT UNIQUE ON CONFLICT ROLLBACK NOT NULL,"
        "    created_at TEXT NOT NULL,"
        "    updated_at TEXT NOT NULL,"
        "    name TEXT NOT NULL"
        ");");
}

inline QList<QString> getSqlJunctionTableDefinitions()
{
    QList<QString> definitions;

    // ordered one-to-many junction table for binder_items
    definitions << QStringLiteral("CREATE TABLE IF NOT EXISTS work_binder_items_to_binder_item_junction ("
                                  "    left_id INTEGER NOT NULL,"
                                  "    right_id INTEGER NOT NULL,"
                                  "    order_ INTEGER NOT NULL,"
                                  "    PRIMARY KEY (left_id, right_id)"
                                  ");");

    return definitions;
}

} // namespace Skribisto::Common::DirectAccess::Binder
