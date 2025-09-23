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
struct DictWord
{
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString word;

    DictWord() = default;
    DictWord(int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &word)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), word(word)
    {
    }
};
} // namespace Skribisto::Common::Entities
