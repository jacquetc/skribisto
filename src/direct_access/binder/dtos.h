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
#include <QObject>
#include <QString>

namespace Skribisto::DirectAccess::Binder
{
Q_NAMESPACE

enum class BinderRelationshipField
{
    BinderItems,
};
Q_ENUM_NS(BinderRelationshipField)

struct BinderDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString name MEMBER name)
    Q_PROPERTY(QList<int> binderItems MEMBER binderItems)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString name;
    QList<int> binderItems = {};
    BinderDto() = default;
    ~BinderDto() = default;
    BinderDto(const BinderDto &) = default;
    BinderDto &operator=(const BinderDto &) = default;
    BinderDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name,
              const QList<int> &binderItems)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), name(name), binderItems(binderItems)
    {
    }
};

struct CreateBinderDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString name MEMBER name)
    Q_PROPERTY(QList<int> binderItems MEMBER binderItems)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString name;
    QList<int> binderItems = {};
    CreateBinderDto() = default;
    ~CreateBinderDto() = default;
    CreateBinderDto(const CreateBinderDto &) = default;
    CreateBinderDto &operator=(const CreateBinderDto &) = default;
    CreateBinderDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QString &name,
                    const QList<int> &binderItems)
        : createdAt(createdAt), updatedAt(updatedAt), name(name), binderItems(binderItems)
    {
    }
};
} // namespace Skribisto::DirectAccess::Binder
Q_DECLARE_METATYPE(Skribisto::DirectAccess::Binder::BinderDto)
Q_DECLARE_METATYPE(Skribisto::DirectAccess::Binder::CreateBinderDto)