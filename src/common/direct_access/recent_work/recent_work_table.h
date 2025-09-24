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

#include "database/db_context.h"
#include "direct_access/recent_work/recent_work_repository.h"
#include "entities/recent_work.h"

#include <QList>

namespace Skribisto::Common::DirectAccess::RecentWork
{
namespace SCE = Skribisto::Common::Entities;

class RecentWorkTable final : public IRecentWorkTable
{
  public:
    explicit RecentWorkTable(Database::DbSubContext &dbSubContext);

    QList<SCE::RecentWork> createMany(const QList<SCE::RecentWork> &recentWorks) override;
    QList<SCE::RecentWork> updateMany(const QList<SCE::RecentWork> &recentWorks) override;
    [[nodiscard]] QList<SCE::RecentWork> findMany(const QList<int> &ids) const override;
    QList<int> removeMany(const QList<int> &ids) override;
    void setRelationshipIds(int recentWorkId, RecentWorkRelationshipField relationship, QList<int> relatedId) override;
    [[nodiscard]] QHash<int, QList<int>> getRelationshipIdsMany(
        const QList<int> &recentWorkIds, RecentWorkRelationshipField relationship) const override;
    int getRelationshipIdsCount(int recentWorkId, RecentWorkRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int recentWorkId, RecentWorkRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    Database::DbSubContext &m_dbSubContext;
};

} // namespace Skribisto::Common::DirectAccess::RecentWork
