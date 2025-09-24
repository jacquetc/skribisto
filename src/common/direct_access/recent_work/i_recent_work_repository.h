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

#include "entities/recent_work.h"

#include <QList>
#include <optional>

namespace Skribisto::Common::DirectAccess::RecentWork
{

// Relationships for RecentWork entity derived from its relational fields
enum class RecentWorkRelationshipField
{
};

class IRecentWorkRepository
{
  public:
    virtual ~IRecentWorkRepository() = default;

    // CRUD
    virtual QList<Entities::RecentWork> create(const QList<Entities::RecentWork> &recentWorks) = 0;
    virtual QList<Entities::RecentWork> get(const QList<int> &recentWorkIds) = 0;
    virtual QList<Entities::RecentWork> update(const QList<Entities::RecentWork> &recentWorks) = 0;
    virtual QList<int> remove(const QList<int> &recentWorkIds) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given RecentWork id (e.g., set work id)
    virtual void setRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship,
                                    QList<int> relatedId) = 0;

    // Get the relationship value for a given RecentWork id (e.g., get work id)
    virtual QList<int> getRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship) = 0;
    virtual QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &recentWorkIds,
                                                          RecentWorkRelationshipField relationship) = 0;
    virtual int getRelationshipIdsCount(int recentWorkId, RecentWorkRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int recentWorkId, RecentWorkRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

} // namespace Skribisto::Common::DirectAccess::RecentWork
