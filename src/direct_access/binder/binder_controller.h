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

namespace Skribisto::DirectAccess::Binder
{
namespace SCDatabase = Skribisto::Common::Database;

class BinderController : public QObject
{
    Q_OBJECT
  public:
    BinderController(const BinderController &) = delete;
    BinderController &operator=(const BinderController &) = delete;
    BinderController(BinderController &&) = delete;
    BinderController &operator=(BinderController &&) = delete;
    explicit BinderController(QObject *parent = nullptr);
    QCoro::Task<QList<BinderDto>> create(const QList<CreateBinderDto> &binders);
    static CreateBinderDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<BinderDto>> get(const QList<int> &binderIds);
    QCoro::Task<QList<BinderDto>> update(const QList<BinderDto> &binders);
    QCoro::Task<QList<int>> remove(const QList<int> &binderIds);
    QCoro::Task<QList<int>> getRelationshipIds(int binderId, BinderRelationshipField relationship);
    QCoro::Task<void> setRelationshipIds(int binderId, BinderRelationshipField relationship, QList<int> relatedIds);
    QCoro::Task<QHash<int, QList<int>>> getRelationshipIdsMany(const QList<int> &binderIds,
                                                               BinderRelationshipField relationship);
    QCoro::Task<int> getRelationshipIdsCount(int binderId, BinderRelationshipField relationship);
    QCoro::Task<QList<int>> getRelationshipIdsInRange(int binderId, BinderRelationshipField relationship, int offset,
                                                      int limit);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::Binder
