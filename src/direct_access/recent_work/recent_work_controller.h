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

namespace Skribisto::DirectAccess::RecentWork
{
namespace SCDatabase = Skribisto::Common::Database;

class RecentWorkController : public QObject
{
    Q_OBJECT
  public:
    RecentWorkController(const RecentWorkController &) = delete;
    RecentWorkController &operator=(const RecentWorkController &) = delete;
    RecentWorkController(RecentWorkController &&) = delete;
    RecentWorkController &operator=(RecentWorkController &&) = delete;
    explicit RecentWorkController(QObject *parent = nullptr);
    QCoro::Task<QList<RecentWorkDto>> create(const QList<CreateRecentWorkDto> &recentWorks);
    static CreateRecentWorkDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<RecentWorkDto>> get(const QList<int> &recentWorkIds);
    QCoro::Task<QList<RecentWorkDto>> update(const QList<RecentWorkDto> &recentWorks);
    QCoro::Task<QList<int>> remove(const QList<int> &recentWorkIds);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::RecentWork
