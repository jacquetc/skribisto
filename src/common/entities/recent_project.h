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
#include <QList>
#include <QString>
#include <optional>

namespace Skribisto::Common::Entities
{
struct RecentProject
{
    int id = 0;
    QDateTime creationDate;
    QDateTime updateDate;
    QString title;
    QString absolutePath;

    // Constructeurs optionnels
    RecentProject() = default;
    RecentProject(int id, const QDateTime &creationDate, const QDateTime &updateDate, const QString &title,
                  const QString &absolutePath)
        : id(id), creationDate(creationDate), updateDate(updateDate), title(title), absolutePath(absolutePath)
    {
    }
};
} // namespace Skribisto::Common::Entities
