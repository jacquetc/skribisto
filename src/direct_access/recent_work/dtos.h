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
#include <QString>

namespace Skribisto::DirectAccess::RecentWork
{
Q_NAMESPACE

struct RecentWorkDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QDateTime lastOpenedAt MEMBER lastOpenedAt)
    Q_PROPERTY(QString absolutePath MEMBER absolutePath)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QDateTime lastOpenedAt;
    QString absolutePath;
    RecentWorkDto() = default;
    ~RecentWorkDto() = default;
    RecentWorkDto(const RecentWorkDto &) = default;
    RecentWorkDto &operator=(const RecentWorkDto &) = default;
    RecentWorkDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
                  const QDateTime &lastOpenedAt, const QString &absolutePath)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), title(title), lastOpenedAt(lastOpenedAt),
          absolutePath(absolutePath)
    {
    }
};

struct CreateRecentWorkDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QDateTime lastOpenedAt MEMBER lastOpenedAt)
    Q_PROPERTY(QString absolutePath MEMBER absolutePath)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QDateTime lastOpenedAt;
    QString absolutePath;
    CreateRecentWorkDto() = default;
    ~CreateRecentWorkDto() = default;
    CreateRecentWorkDto(const CreateRecentWorkDto &) = default;
    CreateRecentWorkDto &operator=(const CreateRecentWorkDto &) = default;
    CreateRecentWorkDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
                        const QDateTime &lastOpenedAt, const QString &absolutePath)
        : createdAt(createdAt), updatedAt(updatedAt), title(title), lastOpenedAt(lastOpenedAt),
          absolutePath(absolutePath)
    {
    }
};
} // namespace Skribisto::DirectAccess::RecentWork
Q_DECLARE_METATYPE(Skribisto::DirectAccess::RecentWork::RecentWorkDto)
Q_DECLARE_METATYPE(Skribisto::DirectAccess::RecentWork::CreateRecentWorkDto)