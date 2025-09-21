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

#include "entities/binder.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::Binder
{

// Relationships for Binder entity derived from its relational fields
// Currently, Binder has a single relationship field: `binderItems`
enum class BinderRelationshipField
{
    BinderItems,
};

class IBinderRepository
{
  public:
    virtual ~IBinderRepository() = default;

    // CRUD
    virtual QList<Entities::Binder> create(const QList<Entities::Binder> &binders) = 0;
    virtual QList<Entities::Binder> get(const QList<int> &binderIds) = 0;
    virtual QList<Entities::Binder> update(const QList<Entities::Binder> &binders) = 0;
    virtual QList<int> remove(const QList<int> &binderIds) = 0;

    // Relationship setters/getters
    // Set the relationship value for a given Binder id (e.g., set binder item id)
    virtual void setRelationship(int binderId, BinderRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Binder id (e.g., get binder item id)
    virtual QList<int> getRelationship(int binderId, BinderRelationshipField relationship) = 0;

    virtual QHash<int, QList<int>> getRelationshipMany(const QList<int> &binderIds,
                                                       BinderRelationshipField relationship) = 0;
};

} // namespace Skribisto::Common::DirectAccess::Binder
