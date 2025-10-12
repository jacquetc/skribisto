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

#include "features/work_management_events.h"

#include <QPointer>

namespace Skribisto::Common::Features
{

// Composite events structure that holds all event instances
class FeatureEventRegistry : public QObject
{
    Q_OBJECT
  public:
    explicit FeatureEventRegistry(QObject *parent = nullptr)
        : QObject(parent), m_workManagementEvents(new Features::WorkManagementEvents(parent))

    //    , m_undoRedoEvents(new UndoRedo::UndoRedoEvents(parent))
    {
    }

    // Helper to get appropriate events by type
    template <typename T> QPointer<T> getEvents() const;

  private:
    QPointer<Features::WorkManagementEvents> m_workManagementEvents;
};

// Template specializations for each event type
template <>
inline QPointer<Features::WorkManagementEvents> FeatureEventRegistry::getEvents<Features::WorkManagementEvents>() const
{
    return m_workManagementEvents;
}

} // namespace Skribisto::Common::Features
