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

#include "direct_access/work/i_work_repository.h"
#include "entities/work.h"
#include <QList>
#include <QMetaObject>
#include <QObject>

// Ensure metatypes are declared for queued connections
Q_DECLARE_METATYPE(Skribisto::Common::Entities::Work)

namespace Skribisto::Common::DirectAccess::Work
{

class WorkEvents : public QObject
{
    Q_OBJECT
  public:
    explicit WorkEvents(QObject *parent = nullptr) : QObject(parent)
    {
        // Register metatypes for cross-thread signal delivery
        qRegisterMetaType<Entities::Work>("Work");
        qRegisterMetaType<QList<Entities::Work>>("QList<Work>");
        qRegisterMetaType<QList<int>>("QList<int>");
        qRegisterMetaType<WorkRelationshipField>("WorkRelationshipField");
    }

  public Q_SLOTS:
    // These methods can be invoked from any thread; they will emit signals in this object's thread
    void publishCreated(const QList<int> &ids)
    {
        Q_EMIT created(ids);
    }
    void publishUpdated(const QList<int> &ids)
    {
        Q_EMIT updated(ids);
    }
    void publishRemoved(const QList<int> &ids)
    {
        Q_EMIT removed(ids);
    }
    void publishRelationshipChanged(int workId, WorkRelationshipField relationship, const QList<int> &relatedIds)
    {
        Q_EMIT relationshipChanged(workId, relationship, relatedIds);
    }

  Q_SIGNALS:
    void created(const QList<int> &ids);
    void updated(const QList<int> &ids);
    void removed(const QList<int> &ids);
    void relationshipChanged(int workId, WorkRelationshipField relationship, const QList<int> &relatedIds);
};

} // namespace Skribisto::Common::DirectAccess::Work
