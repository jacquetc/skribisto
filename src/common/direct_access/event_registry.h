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
#include "direct_access/binder_tag/binder_tag_events.h"
#include "direct_access/content/content_events.h"
#include "direct_access/recent_work/recent_work_events.h"
#include "direct_access/root/root_events.h"
#include "direct_access/work/work_events.h"

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
        : QObject(parent), m_workEvents(new Work::WorkEvents(parent)), m_binderEvents(new Binder::BinderEvents(parent)),
          m_rootEvents(new Root::RootEvents(parent)), m_binderItemEvents(new BinderItem::BinderItemEvents(parent)),
          m_recentWorkEvents(new RecentWork::RecentWorkEvents(parent)),
          m_contentEvents(new Content::ContentEvents(parent)), m_binderTagEvents(new BinderTag::BinderTagEvents(parent))

    //    , m_undoRedoEvents(new UndoRedo::UndoRedoEvents(parent))
    {
    }

    // Helper to get appropriate events by type
    template <typename T> QPointer<T> getEvents() const;

  private:
    QPointer<Work::WorkEvents> m_workEvents;
    QPointer<Binder::BinderEvents> m_binderEvents;
    QPointer<Root::RootEvents> m_rootEvents;
    QPointer<BinderItem::BinderItemEvents> m_binderItemEvents;
    QPointer<RecentWork::RecentWorkEvents> m_recentWorkEvents;
    QPointer<Content::ContentEvents> m_contentEvents;
    QPointer<BinderTag::BinderTagEvents> m_binderTagEvents;
    // QPointer<UndoRedo::UndoRedoEvents> m_undoRedoEvents;
};

// Template specializations for each event type
template <> inline QPointer<Work::WorkEvents> EventRegistry::getEvents<Work::WorkEvents>() const
{
    return m_workEvents;
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

template <> inline QPointer<RecentWork::RecentWorkEvents> EventRegistry::getEvents<RecentWork::RecentWorkEvents>() const
{
    return m_recentWorkEvents;
}

template <> inline QPointer<Content::ContentEvents> EventRegistry::getEvents<Content::ContentEvents>() const
{
    return m_contentEvents;
}

template <> inline QPointer<BinderTag::BinderTagEvents> EventRegistry::getEvents<BinderTag::BinderTagEvents>() const
{
    return m_binderTagEvents;
}

// template <> inline QPointer<UndoRedo::UndoRedoEvents> EventRegistry::getEvents<UndoRedo::UndoRedoEvents>() const
// {
//     return m_undoRedoEvents;
// }

} // namespace Skribisto::Common::DirectAccess
