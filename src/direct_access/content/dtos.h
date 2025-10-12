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

namespace Skribisto::DirectAccess::Content
{
Q_NAMESPACE

struct ContentDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString role MEMBER role)
    Q_PROPERTY(QString data MEMBER data)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString role;
    QString data;
    ContentDto() = default;
    ~ContentDto() = default;
    ContentDto(const ContentDto &) = default;
    ContentDto &operator=(const ContentDto &) = default;
    ContentDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &role,
               const QString &data)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), role(role), data(data)
    {
    }
};

struct CreateContentDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString role MEMBER role)
    Q_PROPERTY(QString data MEMBER data)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString role;
    QString data;
    CreateContentDto() = default;
    ~CreateContentDto() = default;
    CreateContentDto(const CreateContentDto &) = default;
    CreateContentDto &operator=(const CreateContentDto &) = default;
    CreateContentDto(QDateTime createdAt, QDateTime updatedAt, QString role, QString data)
        : createdAt(std::move(createdAt)), updatedAt(std::move(updatedAt)), role(std::move(role)), data(std::move(data))
    {
    }
};
} // namespace Skribisto::DirectAccess::Content
Q_DECLARE_METATYPE(Skribisto::DirectAccess::Content::ContentDto)
Q_DECLARE_METATYPE(Skribisto::DirectAccess::Content::CreateContentDto)