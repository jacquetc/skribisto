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

namespace Skribisto::DirectAccess::BinderItem
{
Q_NAMESPACE

enum class BinderItemRelationshipField
{
    Contents,
    BinderItems,
    ParentItem,
};
Q_ENUM_NS(BinderItemRelationshipField)

struct BinderItemDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QString subTitle MEMBER subTitle)
    Q_PROPERTY(QString role MEMBER role)
    Q_PROPERTY(QString dictLanguage MEMBER dictLanguage)
    Q_PROPERTY(QList<int> contents MEMBER contents)
    Q_PROPERTY(QList<int> binderItems MEMBER binderItems)
    Q_PROPERTY(int parentItem MEMBER parentItem)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QString subTitle;
    QString role;
    QString dictLanguage;
    QList<int> contents = {};
    QList<int> binderItems = {};
    int parentItem = 0;
    BinderItemDto() = default;
    ~BinderItemDto() = default;
    BinderItemDto(const BinderItemDto &) = default;
    BinderItemDto &operator=(const BinderItemDto &) = default;
    BinderItemDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
                  const QString &subTitle, const QString &role, const QString &dictLanguage, const QList<int> &contents,
                  const QList<int> &binderItems, const int parentItem)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), title(title), subTitle(subTitle), role(role),
          dictLanguage(dictLanguage), contents(contents), binderItems(binderItems), parentItem(parentItem)
    {
    }
};

struct CreateBinderItemDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QString subTitle MEMBER subTitle)
    Q_PROPERTY(QString role MEMBER role)
    Q_PROPERTY(QString dictLanguage MEMBER dictLanguage)
    Q_PROPERTY(QList<int> contents MEMBER contents)
    Q_PROPERTY(QList<int> binderItems MEMBER binderItems)
    Q_PROPERTY(int parentItem MEMBER parentItem)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QString subTitle;
    QString role;
    QString dictLanguage;
    QList<int> contents = {};
    QList<int> binderItems = {};
    int parentItem = 0;
    CreateBinderItemDto() = default;
    ~CreateBinderItemDto() = default;
    CreateBinderItemDto(const CreateBinderItemDto &) = default;
    CreateBinderItemDto &operator=(const CreateBinderItemDto &) = default;
    CreateBinderItemDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
                        const QString &subTitle, const QString &role, const QString &dictLanguage,
                        const QList<int> &contents, const QList<int> &binderItems, const int parentItem)
        : createdAt(createdAt), updatedAt(updatedAt), title(title), subTitle(subTitle), role(role),
          dictLanguage(dictLanguage), contents(contents), binderItems(binderItems), parentItem(parentItem)
    {
    }
};
} // namespace Skribisto::DirectAccess::BinderItem
Q_DECLARE_METATYPE(Skribisto::DirectAccess::BinderItem::BinderItemDto)
Q_DECLARE_METATYPE(Skribisto::DirectAccess::BinderItem::CreateBinderItemDto)