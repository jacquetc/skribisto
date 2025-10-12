/*
 * Copyright (C) 2025 by Cyril Jacquet
 * cyril.jacquet@skribisto.eu
 *
 * This file is part of Skribisto.
 *
 * Skribisto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Skribisto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Skribisto.  If not, see <http://www.gnu.org/licenses/>.
 */

#pragma once

#include <QList>
#include <QMetaObject>
#include <QObject>
namespace Skribisto::Common::Features
{
class WorkManagementEvents : public QObject
{
    Q_OBJECT
  public:
    explicit WorkManagementEvents(QObject *parent = nullptr) : QObject(parent)
    {
        // Register metatypes for cross-thread signal delivery
        qRegisterMetaType<QList<int>>("QList<int>");
    }

  public Q_SLOTS:
    // These methods can be invoked from any thread; they will emit signals in this object's thread
    void publishWorkLoaded(int workId)
    {
        Q_EMIT workLoaded(workId);
    }
    void publishWorkSaved()
    {
        Q_EMIT workSaved();
    }

  Q_SIGNALS:
    void workLoaded(int workId);
    void workSaved();
};

} // namespace Skribisto::Common::Features
