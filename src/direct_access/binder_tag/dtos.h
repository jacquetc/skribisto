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

namespace Skribisto::DirectAccess::BinderTag
{

struct BinderTagDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString name MEMBER name)
    Q_PROPERTY(QString color MEMBER color)
    Q_PROPERTY(QString textColor MEMBER textColor)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString name;
    QString color;
    QString textColor;
    BinderTagDto() = default;
    BinderTagDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name,
                 const QString &color, const QString &textColor)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), name(name), color(color), textColor(textColor)
    {
    }
};

struct CreateBinderTagDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString name MEMBER name)
    Q_PROPERTY(QString color MEMBER color)
    Q_PROPERTY(QString textColor MEMBER textColor)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString name;
    QString color;
    QString textColor;
    CreateBinderTagDto() = default;
    CreateBinderTagDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name,
                       const QString &color, const QString &textColor)
        : createdAt(createdAt), updatedAt(updatedAt), name(name), color(color), textColor(textColor)
    {
    }
};
} // namespace Skribisto::DirectAccess::BinderTag