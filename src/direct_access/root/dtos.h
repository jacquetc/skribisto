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

//
// Created by cyril on 15/09/2025.
//

#pragma once
#include <QDateTime>
#include <QList>
#include <QObject>

namespace Skribisto::DirectAccess::Root
{
enum class RootRelationshipField
{
    Works,
    RecentWorks,
};
struct RootDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QList<int> works MEMBER works)
    Q_PROPERTY(QList<int> recentWorks MEMBER recentWorks)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString authorName;
    QList<int> works = {};
    QList<int> recentWorks = {};
    RootDto() = default;
    RootDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &authorName,
            const QList<int> &works, const QList<int> &recentWorks)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), authorName(authorName), works(works),
          recentWorks(recentWorks)
    {
    }
};

struct CreateRootDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString authorName MEMBER authorName)
    Q_PROPERTY(QList<int> works MEMBER works)
    Q_PROPERTY(QList<int> recentWorks MEMBER recentWorks)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString authorName;
    QList<int> works = {};
    QList<int> recentWorks = {};
    CreateRootDto() = default;
    CreateRootDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QString &authorName,
                  const QList<int> &works, const QList<int> &recentWorks)
        : createdAt(createdAt), updatedAt(updatedAt), authorName(authorName), works(works), recentWorks(recentWorks)
    {
    }
};
} // namespace Skribisto::DirectAccess::Root