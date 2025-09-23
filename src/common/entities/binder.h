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
struct Binder
{
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString name;
    QList<int> binderItems;

    Binder() = default;
    Binder(int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), name(name)
    {
    }
    Binder(int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name,
           const QList<int> &binderItems)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), name(name), binderItems(binderItems)
    {
    }
};
} // namespace Skribisto::Common::Entities