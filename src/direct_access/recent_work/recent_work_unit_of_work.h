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
#include "direct_access/recent_work/recent_work_events.h"
#include "use_cases/i_recent_work_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::RecentWork
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;
namespace SCD = Skribisto::Common::DirectAccess;

class RecentWorkUnitOfWork final : public IRecentWorkUnitOfWork
{

  public:
    RecentWorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~RecentWorkUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::RecentWork> createRecentWork(QList<SCE::RecentWork> recentWorks) override;
    QList<SCE::RecentWork> getRecentWork(QList<int> recentWorkIds) override;
    QList<SCE::RecentWork> updateRecentWork(QList<SCE::RecentWork> recentWorks) override;
    QList<int> removeRecentWork(QList<int> recentWorkIds) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::RecentWork