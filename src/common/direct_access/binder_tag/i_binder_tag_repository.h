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

#include "entities/binder_tag.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::BinderTag
{

// Relationships for BinderTag entity derived from its relational fields
enum class BinderTagRelationshipField
{
};

class IBinderTagRepository
{
  public:
    virtual ~IBinderTagRepository() = default;

    // CRUD
    virtual QList<Entities::BinderTag> create(const QList<Entities::BinderTag> &binderTags) = 0;
    virtual QList<Entities::BinderTag> get(const QList<int> &binderTagIds) = 0;
    virtual QList<Entities::BinderTag> update(const QList<Entities::BinderTag> &binderTags) = 0;
    virtual QList<int> remove(const QList<int> &binderTagIds) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given BinderTag id (e.g., set project id)
    virtual void setRelationshipIds(int binderTagId, BinderTagRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given BinderTag id (e.g., get project id)
    virtual QList<int> getRelationshipIds(int binderTagId, BinderTagRelationshipField relationship) = 0;
    virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderTagIds,
                                                          BinderTagRelationshipField relationship) = 0;
    virtual int getRelationshipIdsCount(int binderTagId, BinderTagRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int binderTagId, BinderTagRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

} // namespace Skribisto::Common::DirectAccess::BinderTag
