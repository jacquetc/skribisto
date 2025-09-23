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

#pragma once

#include <QDateTime>
#include <QList>
#include <QString>
#include <optional>

namespace Skribisto::Common::Entities
{
struct BinderItem
{
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QString subTitle;
    QString role;
    QString text;
    QString dict;
    QList<int> contents;
    QList<int> binderItems;
    std::optional<int> parent;

    BinderItem() = default;
    BinderItem(int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
               const QString &subTitle, const QString &role, const QString &text)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), title(title), subTitle(subTitle), role(role), text(text)

    {
    }
};
} // namespace Skribisto::Common::Entities