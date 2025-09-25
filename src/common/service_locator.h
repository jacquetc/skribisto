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
#pragma once
#include "database/db_context.h"
#include "direct_access/event_registry.h"
#include "undo_redo/undo_redo_system.h"

#include <QObject>
#include <QPointer>

namespace Skribisto::Common
{

class ServiceLocator : public QObject
{
    Q_OBJECT
  public:
    explicit ServiceLocator(QObject *parent = nullptr);
    ~ServiceLocator();

    void setDbContext(Database::DbContext *db);
    void setEventRegistry(DirectAccess::EventRegistry *ev);
    void setUndoRedoSystem(UndoRedo::UndoRedoSystem *urs);

    Database::DbContext *dbContext() const;
    QPointer<DirectAccess::EventRegistry> eventRegistry() const;
    QPointer<UndoRedo::UndoRedoSystem> undoRedoSystem() const;

    static void setInstance(ServiceLocator *locator);
    static ServiceLocator *instance();

  private:
    inline static ServiceLocator *s_instance = nullptr;
    Database::DbContext *m_dbContext;
    QPointer<DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};

} // namespace Skribisto::Common