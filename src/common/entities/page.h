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
struct Page
{
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString name;
    QString subName;
    QList<int> childPages;
    std::optional<int> parentPage;
    QString pageType;
    std::optional<QList<int>> contents;
    QString dictLang;

    Page() = default;
    Page(int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name, const QString &subName,
         const QList<int> &childPages, const QString &pageType, const QString &dictLang)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), name(name), subName(subName), childPages(childPages),
          pageType(pageType), dictLang(dictLang)
    {
    }
    Page(int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name, const QString &subName,
         const QList<int> &childPages, const std::optional<int> &parentPage, const QString &pageType,
         const std::optional<QList<int>> &contents, const QString &dictLang)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), name(name), subName(subName), childPages(childPages),
          parentPage(parentPage), pageType(pageType), contents(contents), dictLang(dictLang)
    {
    }
};
} // namespace Skribisto::Common::Entities
