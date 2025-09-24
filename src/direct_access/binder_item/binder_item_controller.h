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
#include "dtos.h"
#include <QCoro/QCoroTask>

#include <QPointer>

namespace Skribisto::Common::UndoRedo
{
class UndoRedoSystem;
}

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCDatabase = Skribisto::Common::Database;

class BinderItemController : public QObject
{
    Q_OBJECT
  public:
    BinderItemController(const BinderItemController &) = delete;
    BinderItemController &operator=(const BinderItemController &) = delete;
    BinderItemController(BinderItemController &&) = delete;
    BinderItemController &operator=(BinderItemController &&) = delete;
    explicit BinderItemController(QObject *parent = nullptr);
    QCoro::Task<QList<BinderItemDto>> create(const QList<CreateBinderItemDto> &binderItems);
    static CreateBinderItemDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<BinderItemDto>> get(const QList<int> &binderItemIds);
    QCoro::Task<QList<BinderItemDto>> update(const QList<BinderItemDto> &binderItems);
    QCoro::Task<QList<int>> remove(const QList<int> &binderItemIds);
    QCoro::Task<QList<int>> getRelationshipIds(int binderItemId, BinderItemRelationshipField relationship);
    QCoro::Task<void> setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship,
                                         QList<int> relatedIds);
    QCoro::Task<QHash<int, QList<int>>> getRelationshipIdsMany(const QList<int> &binderItemIds,
                                                               BinderItemRelationshipField relationship);
    QCoro::Task<int> getRelationshipIdsCount(int binderItemId, BinderItemRelationshipField relationship);
    QCoro::Task<QList<int>> getRelationshipIdsInRange(int binderItemId, BinderItemRelationshipField relationship,
                                                      int offset, int limit);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::BinderItem
