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

#include "service_locator.h"

namespace SC = Skribisto::Common;

SC::ServiceLocator::ServiceLocator(QObject *parent) : QObject(parent)
{
}

Skribisto::Common::ServiceLocator::~ServiceLocator()
{
    // destroy in the reverse order of creation
    m_undoRedoSystem = nullptr;
    m_eventRegistry = nullptr;
    m_dbContext = nullptr;
}

void SC::ServiceLocator::setDbContext(SC::Database::DbContext *db)
{
    m_dbContext = db;
}
void SC::ServiceLocator::setEventRegistry(SC::DirectAccess::EventRegistry *ev)
{
    m_eventRegistry = ev;
}
void SC::ServiceLocator::setUndoRedoSystem(SC::UndoRedo::UndoRedoSystem *urs)
{
    m_undoRedoSystem = urs;
}

SC::Database::DbContext *SC::ServiceLocator::dbContext() const
{
    return m_dbContext;
}
QPointer<SC::DirectAccess::EventRegistry> SC::ServiceLocator::eventRegistry() const
{
    return m_eventRegistry;
}
QPointer<SC::UndoRedo::UndoRedoSystem> SC::ServiceLocator::undoRedoSystem() const
{
    return m_undoRedoSystem;
}
void SC::ServiceLocator::setInstance(ServiceLocator *locator)
{
    s_instance = locator;
}
SC::ServiceLocator *SC::ServiceLocator::instance()
{
    return s_instance;
}