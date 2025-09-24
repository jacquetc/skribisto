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
#include <optional>
namespace Skribisto::Common::Entities
{
struct Root
{
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString authorName;
    QList<int> projects;
    QList<int> recentProjects;

    // Default constructor
    Root() = default;

    Root(const int id, const QList<int> &projectIds, const QList<int> &recentProjects)
        : id(id), projects(projectIds), recentProjects(recentProjects)
    {
    }

    // with only id
    explicit Root(const int id) : id(id)
    {
    }

    // Constructor with creation and update dates
    Root(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &authorName,
         const QList<int> &projectIds, const QList<int> &recentProjects)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), authorName(authorName), projects(projectIds),
          recentProjects(recentProjects)
    {
    }
};
} // namespace Skribisto::Common::Entities