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
#include "direct_access/work/work_events.h"
#include "use_cases/i_work_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::Work
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCD = Skribisto::Common::DirectAccess;

class WorkUnitOfWork final : public IWorkUnitOfWork
{

  public:
    WorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~WorkUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::Work> createWork(QList<SCE::Work> works) override;
    QList<SCE::Work> getWork(QList<int> workIds) override;
    QList<SCE::Work> updateWork(QList<SCE::Work> works) override;
    QList<int> removeWork(QList<int> workIds) override;
    QList<int> getWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship) override;
    void setWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship, QList<int> relatedIds) override;
    QHash<int, QList<int>> getWorkRelationshipMany(const QList<int> &workIds,
                                                   SCDWork::WorkRelationshipField relationship) override;
    int getWorkRelationshipCount(int workId, SCDWork::WorkRelationshipField relationship) override;
    QList<int> getWorkRelationshipInRange(int workId, SCDWork::WorkRelationshipField relationship, int offset,
                                          int limit) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::Work