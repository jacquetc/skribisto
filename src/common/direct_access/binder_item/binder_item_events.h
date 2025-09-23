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

#include "direct_access/binder_item/i_binder_item_repository.h"
#include "entities/binder_item.h"
#include <QList>
#include <QMetaObject>
#include <QObject>

// Ensure metatypes are declared for queued connections
Q_DECLARE_METATYPE(Skribisto::Common::Entities::BinderItem)

namespace Skribisto::Common::DirectAccess::BinderItem
{

class BinderItemEvents : public QObject
{
    Q_OBJECT
  public:
    explicit BinderItemEvents(QObject *parent = nullptr) : QObject(parent)
    {
        // Register metatypes for cross-thread signal delivery
        qRegisterMetaType<Entities::BinderItem>("BinderItem");
        qRegisterMetaType<QList<Entities::BinderItem>>("QList<BinderItem>");
        qRegisterMetaType<QList<int>>("QList<int>");
        qRegisterMetaType<BinderItemRelationshipField>("BinderItemRelationshipField");
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
    void publishRelationshipChanged(int binderItemId, BinderItemRelationshipField relationship,
                                    const QList<int> &relatedIds)
    {
        Q_EMIT relationshipChanged(binderItemId, relationship, relatedIds);
    }

  Q_SIGNALS:
    void created(const QList<int> &ids);
    void updated(const QList<int> &ids);
    void removed(const QList<int> &ids);
    void relationshipChanged(int binderItemId, BinderItemRelationshipField relationship, const QList<int> &relatedIds);
};

} // namespace Skribisto::Common::DirectAccess::BinderItem
