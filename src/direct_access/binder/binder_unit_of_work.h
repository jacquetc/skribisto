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
#include "direct_access/binder/binder_events.h"
#include "direct_access/event_registry.h"
#include "use_cases/i_binder_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::Binder
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCD = Skribisto::Common::DirectAccess;

class BinderUnitOfWork final : public IBinderUnitOfWork
{

  public:
    BinderUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~BinderUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::Binder> createBinder(QList<SCE::Binder> binders) override;
    QList<SCE::Binder> getBinder(QList<int> binderIds) override;
    QList<SCE::Binder> updateBinder(QList<SCE::Binder> binders) override;
    QList<int> removeBinder(QList<int> binderIds) override;
    QList<int> getBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship) override;
    void setBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship,
                               QList<int> relatedIds) override;
    QHash<int, QList<int>> getBinderRelationshipMany(const QList<int> &binderIds,
                                                     SCDBinder::BinderRelationshipField relationship) override;
    int getBinderRelationshipCount(int binderId, SCDBinder::BinderRelationshipField relationship) override;
    QList<int> getBinderRelationshipInRange(int binderId, SCDBinder::BinderRelationshipField relationship, int offset,
                                            int limit) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::Binder