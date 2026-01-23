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

namespace Skribisto::DirectAccess::Root
{
namespace SCDatabase = Skribisto::Common::Database;

class RootController : public QObject
{
    Q_OBJECT
  public:
    RootController(const RootController &) = delete;
    RootController &operator=(const RootController &) = delete;
    RootController(RootController &&) = delete;
    RootController &operator=(RootController &&) = delete;
    explicit RootController(QObject *parent = nullptr);
    QCoro::Task<QList<RootDto>> create(const QList<CreateRootDto> &roots, int stackId = 0);
    static CreateRootDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<RootDto>> get(const QList<int> &rootIds, int stackId = 0);
    QCoro::Task<QList<RootDto>> update(const QList<RootDto> &roots, int stackId = 0);
    QCoro::Task<QList<int>> remove(const QList<int> &rootIds, int stackId = 0);
    QCoro::Task<QList<int>> getRelationshipIds(int rootId, RootRelationshipField relationship);
    QCoro::Task<void> setRelationshipIds(int rootId, RootRelationshipField relationship, QList<int> relatedIds, int stackId = 0);
    QCoro::Task<QHash<int, QList<int>>> getRelationshipIdsMany(const QList<int> &rootIds,
                                                               RootRelationshipField relationship, int stackId = 0);
    QCoro::Task<int> getRelationshipIdsCount(int rootId, RootRelationshipField relationship, int stackId = 0);
    QCoro::Task<QList<int>> getRelationshipIdsInRange(int rootId, RootRelationshipField relationship, int offset,
                                                      int limit, int stackId = 0);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::Root
