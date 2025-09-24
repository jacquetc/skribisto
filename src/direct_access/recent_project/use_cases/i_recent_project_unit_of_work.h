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
#include "direct_access/recent_project/i_recent_project_repository.h"
#include "entities/recent_project.h"

#include <QString>

namespace Skribisto::DirectAccess::RecentProject
{
namespace SCE = Common::Entities;
namespace SCDRecentProject = Common::DirectAccess::RecentProject;

class IRecentProjectUnitOfWork
{
  public:
    virtual ~IRecentProjectUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::RecentProject> createRecentProject(QList<SCE::RecentProject> recentProjects) = 0;
    virtual QList<SCE::RecentProject> getRecentProject(QList<int> recentProjectIds) = 0;
    virtual QList<SCE::RecentProject> updateRecentProject(QList<SCE::RecentProject> recentProjects) = 0;
    virtual QList<int> removeRecentProject(QList<int> recentProjectIds) = 0;
};
} // namespace Skribisto::DirectAccess::RecentProject