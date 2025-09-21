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

namespace Skribisto::Common::Entities
{
struct Project
{
    int id = 0;
    QDateTime creationDate;
    QDateTime updateDate;
    QString title;
    QString dictLanguage;
    QList<int> binders;
    // Constructeurs optionnels
    Project() = default;
    Project(int id, const QString &title) : id(id), title(title)
    {
    }
    Project(int id, const QString &title, const QList<int> &binders) : id(id), title(title), binders(binders)
    {
    }
    Project(int id, const QString &title, const QList<int> &binders, const QDateTime &creationDate,
            const QDateTime &updateDate)
        : id(id), title(title), binders(binders), creationDate(creationDate), updateDate(updateDate)
    {
    }
};
} // namespace Skribisto::Common::Entities
