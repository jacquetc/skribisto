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

namespace Skribisto::DirectAccess::Work
{
Q_NAMESPACE

enum class WorkRelationshipField
{
    Binders,
    Tags
};
Q_ENUM_NS(WorkRelationshipField)

struct WorkDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QString dictLanguage MEMBER dictLanguage)
    Q_PROPERTY(QList<int> binders MEMBER binders)
    Q_PROPERTY(QList<int> tags MEMBER tags)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QString dictLanguage;
    QList<int> binders = {};
    QList<int> tags = {};
    WorkDto() = default;
    ~WorkDto() = default;
    WorkDto(const WorkDto &) = default;
    WorkDto &operator=(const WorkDto &) = default;
    WorkDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
            const QString &dictLanguage, const QList<int> &binders, const QList<int> &tags)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), title(title), dictLanguage(dictLanguage),
          binders(binders), tags(tags)
    {
    }
};

struct CreateWorkDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QString title MEMBER title)
    Q_PROPERTY(QString dictLanguage MEMBER dictLanguage)
    Q_PROPERTY(QList<int> binders MEMBER binders)
    Q_PROPERTY(QList<int> tags MEMBER tags)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QString title;
    QString dictLanguage;
    QList<int> binders = {};
    QList<int> tags = {};
    CreateWorkDto() = default;
    ~CreateWorkDto() = default;
    CreateWorkDto(const CreateWorkDto &) = default;
    CreateWorkDto &operator=(const CreateWorkDto &) = default;
    CreateWorkDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QString &title,
                  const QString &dictLanguage, const QList<int> &binders, const QList<int> &tags)
        : createdAt(createdAt), updatedAt(updatedAt), title(title), dictLanguage(dictLanguage), binders(binders),
          tags(tags)
    {
    }
};
} // namespace Skribisto::DirectAccess::Work
Q_DECLARE_METATYPE(Skribisto::DirectAccess::Work::WorkDto)
Q_DECLARE_METATYPE(Skribisto::DirectAccess::Work::CreateWorkDto)