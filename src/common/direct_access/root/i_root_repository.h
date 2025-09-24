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

#include "entities/root.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::Root
{

// Relationships for Root entity derived from its relational fields
enum class RootRelationshipField
{
    Works,
    RecentWorks,
};

class IRootRepository
{
  public:
    virtual ~IRootRepository() = default;

    // CRUD
    virtual QList<Entities::Root> create(const QList<Entities::Root> &roots) = 0;
    virtual QList<Entities::Root> get(const QList<int> &rootIds) = 0;
    virtual QList<Entities::Root> update(const QList<Entities::Root> &roots) = 0;
    virtual QList<int> remove(const QList<int> &rootIds) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Root id (e.g., set work id)
    virtual void setRelationshipIds(int rootId, RootRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Root id (e.g., get work id)
    virtual QList<int> getRelationshipIds(int rootId, RootRelationshipField relationship) = 0;
    virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &rootIds,
                                                          RootRelationshipField relationship) = 0;
    virtual int getRelationshipIdsCount(int rootId, RootRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int rootId, RootRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

} // namespace Skribisto::Common::DirectAccess::Root
