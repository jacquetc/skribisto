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
struct Content
{
    int id = 0;
    QDateTime creationDate;
    QDateTime updateDate;
    QString name;
    QString data;

    Content() = default;
    Content(int id, const QDateTime &creationDate, const QDateTime &updateDate, const QString &name,
            const QString &data)
        : id(id), creationDate(creationDate), updateDate(updateDate), name(name), data(data)
    {
    }
};
#pragma once

#include <QDateTime>
#include <QList>
#include <QString>
#include <optional>

struct Binder
{
    int id = 0;
    QDateTime creationDate;
    QDateTime updateDate;
    QString name;
    std::optional<QList<int>> pages;

    Binder() = default;
    Binder(int id, const QDateTime &creationDate, const QDateTime &updateDate, const QString &name)
        : id(id), creationDate(creationDate), updateDate(updateDate), name(name)
    {
    }
    Binder(int id, const QDateTime &creationDate, const QDateTime &updateDate, const QString &name,
           const std::optional<QList<int>> &pages)
        : id(id), creationDate(creationDate), updateDate(updateDate), name(name), pages(pages)
    {
    }
};
} // namespace Skribisto::Common::Entities
