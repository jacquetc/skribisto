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

#include "entities/work.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::Work
{

// Relationships for Work entity derived from its relational fields
enum class WorkRelationshipField
{
    Binders,
};

class IWorkRepository
{
  public:
    virtual ~IWorkRepository() = default;

    // CRUD
    virtual QList<Entities::Work> create(const QList<Entities::Work> &works) = 0;
    virtual QList<Entities::Work> get(const QList<int> &workIds) = 0;
    virtual QList<Entities::Work> update(const QList<Entities::Work> &works) = 0;
    virtual QList<int> remove(const QList<int> &workIds) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Work id (e.g., set work id)
    virtual void setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Work id (e.g., get work id)
    virtual QList<int> getRelationshipIds(int workId, WorkRelationshipField relationship) = 0;
    virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &workIds,
                                                          WorkRelationshipField relationship) = 0;
    virtual int getRelationshipIdsCount(int rootId, WorkRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int rootId, WorkRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

} // namespace Skribisto::Common::DirectAccess::Work
