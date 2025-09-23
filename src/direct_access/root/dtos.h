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
    Projects,
    RecentProjects,
};
struct RootDto
{
    Q_GADGET
    Q_PROPERTY(int id MEMBER id)
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QList<int> projects MEMBER projects)
    Q_PROPERTY(QList<int> recentProjects MEMBER recentProjects)

  public:
    int id = 0;
    QDateTime createdAt;
    QDateTime updatedAt;
    QList<int> projects = {};
    QList<int> recentProjects = {};
    RootDto() = default;
    RootDto(const int id, const QDateTime &createdAt, const QDateTime &updatedAt, const QList<int> &projects,
            const QList<int> &recent_projects)
        : id(id), createdAt(createdAt), updatedAt(updatedAt), projects(projects), recentProjects(recent_projects)
    {
    }
};

struct CreateRootDto
{
    Q_GADGET
    Q_PROPERTY(QDateTime createdAt MEMBER createdAt)
    Q_PROPERTY(QDateTime updatedAt MEMBER updatedAt)
    Q_PROPERTY(QList<int> projects MEMBER projects)
    Q_PROPERTY(QList<int> recentProjects MEMBER recentProjects)

  public:
    QDateTime createdAt;
    QDateTime updatedAt;
    QList<int> projects = {};
    QList<int> recentProjects = {};
    CreateRootDto() = default;
    CreateRootDto(const QDateTime &createdAt, const QDateTime &updatedAt, const QList<int> &projects,
                  const QList<int> &recent_projects)
        : createdAt(createdAt), updatedAt(updatedAt), projects(projects), recentProjects(recent_projects)
    {
    }
};
} // namespace Skribisto::DirectAccess::Root