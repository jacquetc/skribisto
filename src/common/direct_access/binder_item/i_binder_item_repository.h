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

#include "entities/binder_item.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::BinderItem
{

// Relationships for BinderItem entity derived from its relational fields
// Currently, BinderItem has a single relationship field: `projects`
enum class BinderItemRelationshipField
{
    Contents,
    BinderItems,
    ParentItem,
};

class IBinderItemRepository
{
  public:
    virtual ~IBinderItemRepository() = default;

    // CRUD
    virtual QList<Entities::BinderItem> create(const QList<Entities::BinderItem> &binderItems) = 0;
    virtual QList<Entities::BinderItem> get(const QList<int> &binderItemIds) = 0;
    virtual QList<Entities::BinderItem> update(const QList<Entities::BinderItem> &binderItems) = 0;
    virtual QList<int> remove(const QList<int> &binderItemIds) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given BinderItem id (e.g., set project id)
    virtual void setRelationshipIds(int binderItemId, BinderItemRelationshipField relationship,
                                    QList<int> relatedId) = 0;

    // Get the relationship value for a given BinderItem id (e.g., get project id)
    virtual QList<int> getRelationshipIds(int binderItemId, BinderItemRelationshipField relationship) = 0;
    virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderItemIds,
                                                          BinderItemRelationshipField relationship) = 0;
    virtual int getRelationshipIdsCount(int binderItemId, BinderItemRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int binderItemId, BinderItemRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

} // namespace Skribisto::Common::DirectAccess::BinderItem
