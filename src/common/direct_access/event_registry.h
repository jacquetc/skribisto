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

#include "direct_access/binder/binder_events.h"
#include "direct_access/binder_item/binder_item_events.h"
#include "direct_access/project/project_events.h"
#include "direct_access/recent_project/recent_project_events.h"
#include "direct_access/root/root_events.h"

// #include "undo_redo/undo_redo_events.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess
{

// Composite events structure that holds all event instances
class EventRegistry : public QObject
{
    Q_OBJECT
  public:
    explicit EventRegistry(QObject *parent = nullptr)
        : QObject(parent), m_projectEvents(new Project::ProjectEvents(parent)),
          m_binderEvents(new Binder::BinderEvents(parent)), m_rootEvents(new Root::RootEvents(parent)),
          m_binderItemEvents(new BinderItem::BinderItemEvents(parent)),
          m_recentProjectEvents(new RecentProject::RecentProjectEvents(parent))

    //    , m_undoRedoEvents(new UndoRedo::UndoRedoEvents(parent))
    {
    }

    // Helper to get appropriate events by type
    template <typename T> QPointer<T> getEvents() const;

  private:
    QPointer<Project::ProjectEvents> m_projectEvents;
    QPointer<Binder::BinderEvents> m_binderEvents;
    QPointer<Root::RootEvents> m_rootEvents;
    QPointer<BinderItem::BinderItemEvents> m_binderItemEvents;
    QPointer<RecentProject::RecentProjectEvents> m_recentProjectEvents;
    // QPointer<UndoRedo::UndoRedoEvents> m_undoRedoEvents;
};

// Template specializations for each event type
template <> inline QPointer<Project::ProjectEvents> EventRegistry::getEvents<Project::ProjectEvents>() const
{
    return m_projectEvents;
}

template <> inline QPointer<Binder::BinderEvents> EventRegistry::getEvents<Binder::BinderEvents>() const
{
    return m_binderEvents;
}

template <> inline QPointer<Root::RootEvents> EventRegistry::getEvents<Root::RootEvents>() const
{
    return m_rootEvents;
}

template <> inline QPointer<BinderItem::BinderItemEvents> EventRegistry::getEvents<BinderItem::BinderItemEvents>() const
{
    return m_binderItemEvents;
}

template <>
inline QPointer<RecentProject::RecentProjectEvents> EventRegistry::getEvents<RecentProject::RecentProjectEvents>() const
{
    return m_recentProjectEvents;
}

// template <> inline QPointer<UndoRedo::UndoRedoEvents> EventRegistry::getEvents<UndoRedo::UndoRedoEvents>() const
// {
//     return m_undoRedoEvents;
// }

} // namespace Skribisto::Common::DirectAccess
