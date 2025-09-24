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

#include <QString>

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCE = Common::Entities;
namespace SCDBinderItem = Common::DirectAccess::BinderItem;

class IBinderItemUnitOfWork
{
  public:
    virtual ~IBinderItemUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::BinderItem> createBinderItem(QList<SCE::BinderItem> binderItems) = 0;
    virtual QList<SCE::BinderItem> getBinderItem(QList<int> binderItemIds) = 0;
    virtual QList<SCE::BinderItem> updateBinderItem(QList<SCE::BinderItem> binderItems) = 0;
    virtual QList<int> removeBinderItem(QList<int> binderItemIds) = 0;
    virtual QList<int> getBinderItemRelationship(int binderItemId,
                                                 SCDBinderItem::BinderItemRelationshipField relationship) = 0;
    virtual void setBinderItemRelationship(int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship,
                                           QList<int> relatedIds) = 0;
    virtual QHash<int, QList<int>> getBinderItemRelationshipMany(
        const QList<int> &binderItemIds, SCDBinderItem::BinderItemRelationshipField relationship) = 0;
    virtual int getBinderItemRelationshipCount(int binderItemId,
                                               SCDBinderItem::BinderItemRelationshipField relationship) = 0;
    virtual QList<int> getBinderItemRelationshipInRange(int binderItemId,
                                                        SCDBinderItem::BinderItemRelationshipField relationship,
                                                        int offset, int limit) = 0;
};
} // namespace Skribisto::DirectAccess::BinderItem