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
#include "direct_access/binder_item/binder_item_events.h"
#include "direct_access/event_registry.h"
#include "use_cases/i_binder_item_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCD = Skribisto::Common::DirectAccess;

class BinderItemUnitOfWork final : public IBinderItemUnitOfWork
{

  public:
    BinderItemUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~BinderItemUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::BinderItem> createBinderItem(QList<SCE::BinderItem> binderItems) override;
    QList<SCE::BinderItem> getBinderItem(QList<int> binderItemIds) override;
    QList<SCE::BinderItem> updateBinderItem(QList<SCE::BinderItem> binderItems) override;
    QList<int> removeBinderItem(QList<int> binderItemIds) override;
    QList<int> getBinderItemRelationship(int binderItemId,
                                         SCDBinderItem::BinderItemRelationshipField relationship) override;
    void setBinderItemRelationship(int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship,
                                   QList<int> relatedIds) override;
    QHash<int, QList<int>> getBinderItemRelationshipMany(
        const QList<int> &binderItemIds, SCDBinderItem::BinderItemRelationshipField relationship) override;
    int getBinderItemRelationshipCount(int binderItemId,
                                       SCDBinderItem::BinderItemRelationshipField relationship) override;
    QList<int> getBinderItemRelationshipInRange(int binderItemId,
                                                SCDBinderItem::BinderItemRelationshipField relationship, int offset,
                                                int limit) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::BinderItem