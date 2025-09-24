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

namespace Skribisto::Common::DirectAccess::Content
{

inline QString getSqlTableDefinition()
{
    return QStringLiteral("CREATE TABLE IF NOT EXISTS content ("
                          "    id INTEGER PRIMARY KEY,"
                          "    created_at TEXT NOT NULL,"
                          "    updated_at TEXT NOT NULL,"
                          "    role TEXT NOT NULL,"
                          "    data TEXT"
                          ");");
}

inline QList<QString> getSqlJunctionTableDefinitions()
{
    QList<QString> definitions;

    // Backward relationship: unordered one-to-many junction table from binder_item to content
    definitions << QStringLiteral("CREATE TABLE IF NOT EXISTS binder_item_contents_to_content_junction ("
                                  "    left_id INTEGER NOT NULL,"
                                  "    right_id INTEGER NOT NULL,"
                                  "    PRIMARY KEY (left_id, right_id)"
                                  ");");

    return definitions;
}

} // namespace Skribisto::Common::DirectAccess::Content
