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
#include "database/db_context.h"
#include "direct_access/event_registry.h"
#include "direct_access/recent_project/recent_project_events.h"
#include "use_cases/i_recent_project_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::RecentProject
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;
namespace SCD = Skribisto::Common::DirectAccess;

class RecentProjectUnitOfWork final : public IRecentProjectUnitOfWork
{

  public:
    RecentProjectUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~RecentProjectUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::RecentProject> createRecentProject(QList<SCE::RecentProject> recentProjects) override;
    QList<SCE::RecentProject> getRecentProject(QList<int> recentProjectIds) override;
    QList<SCE::RecentProject> updateRecentProject(QList<SCE::RecentProject> recentProjects) override;
    QList<int> removeRecentProject(QList<int> recentProjectIds) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::RecentProject