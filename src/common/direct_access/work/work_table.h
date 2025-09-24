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
#include "direct_access/work/work_repository.h"
#include "entities/work.h"

#include <QList>

namespace Skribisto::Common::DirectAccess::Work
{
namespace SCE = Skribisto::Common::Entities;

class WorkTable final : public IWorkTable
{
  public:
    explicit WorkTable(Database::DbSubContext &dbSubContext);

    QList<SCE::Work> createMany(const QList<SCE::Work> &works) override;
    QList<SCE::Work> updateMany(const QList<SCE::Work> &works) override;
    [[nodiscard]] QList<SCE::Work> findMany(const QList<int> &ids) const override;
    QList<int> removeMany(const QList<int> &ids) override;
    void setRelationshipIds(int workId, WorkRelationshipField relationship, QList<int> relatedId) override;
    [[nodiscard]] QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &workIds,
                                                                WorkRelationshipField relationship) const override;
    int getRelationshipIdsCount(int rootId, WorkRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int rootId, WorkRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    Database::DbSubContext &m_dbSubContext;
};

} // namespace Skribisto::Common::DirectAccess::Work
