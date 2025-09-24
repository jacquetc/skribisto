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
#include "undo_redo/undo_redo_system.h"
#include <QCoro/QCoroTask>

#include <QPointer>

namespace Skribisto::WorkManagement
{
namespace SCDatabase = Skribisto::Common::Database;

class WorkManagementController : public QObject
{
    Q_OBJECT
  public:
    WorkManagementController(const WorkManagementController &) = delete;
    WorkManagementController &operator=(const WorkManagementController &) = delete;
    WorkManagementController(WorkManagementController &&) = delete;
    WorkManagementController &operator=(WorkManagementController &&) = delete;
    explicit WorkManagementController(QObject *parent = nullptr);
    static LoadWorkDto getLoadWorkDto()
    {
        return {};
    }
    QCoro::Task<bool> loadWork(const LoadWorkDto &loadWorkDto);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::WorkManagement
