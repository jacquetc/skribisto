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
#include <QString>

namespace Skribisto::Common::Entities
{
struct Tag
{
    int id = 0;
    QDateTime creationDate;
    QDateTime updateDate;
    QString name;
    QString color;

    Tag() = default;
    Tag(int id, const QDateTime &creationDate, const QDateTime &updateDate, const QString &name, const QString &color)
        : id(id), creationDate(creationDate), updateDate(updateDate), name(name), color(color)
    {
    }
};
} // namespace Skribisto::Common::Entities
