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

namespace Skribisto::DirectAccess::Work
{
namespace SCDatabase = Skribisto::Common::Database;

class WorkController : public QObject
{
    Q_OBJECT
  public:
    WorkController(const WorkController &) = delete;
    WorkController &operator=(const WorkController &) = delete;
    WorkController(WorkController &&) = delete;
    WorkController &operator=(WorkController &&) = delete;
    explicit WorkController(QObject *parent = nullptr);
    QCoro::Task<QList<WorkDto>> create(const QList<CreateWorkDto> &works);
    static CreateWorkDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<WorkDto>> get(const QList<int> &workIds);
    QCoro::Task<QList<WorkDto>> update(const QList<WorkDto> &works);
    QCoro::Task<QList<int>> remove(const QList<int> &workIds);
    QCoro::Task<QList<int>> getRelationshipIds(int workId, WorkRelationshipField relationship);
    QCoro::Task<void> setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedIds);
    QCoro::Task<QHash<int, QList<int>>> getRelationshipIdsMany(const QList<int> &workIds,
                                                               WorkRelationshipField relationship);
    QCoro::Task<int> getRelationshipIdsCount(int workId, WorkRelationshipField relationship);
    QCoro::Task<QList<int>> getRelationshipIdsInRange(int workId, WorkRelationshipField relationship, int offset,
                                                      int limit);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::Work
