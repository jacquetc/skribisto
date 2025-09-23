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
#include "direct_access/root/root_repository.h"
#include "entities/root.h"

#include <QList>

namespace Skribisto::Common::DirectAccess::Root
{
namespace SCE = Skribisto::Common::Entities;

class RootTable final : public IRootTable
{
  public:
    explicit RootTable(Database::DbSubContext &dbSubContext);

    QList<SCE::Root> createMany(const QList<SCE::Root> &roots) override;
    QList<SCE::Root> updateMany(const QList<SCE::Root> &roots) override;
    [[nodiscard]] QList<SCE::Root> findMany(const QList<int> &ids) const override;
    QList<int> removeMany(const QList<int> &ids) override;
    void setRelationshipIds(int rootId, RootRelationshipField relationship, QList<int> relatedId) override;
    [[nodiscard]] QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &rootIds,
                                                                RootRelationshipField relationship) const override;
    int getRelationshipIdsCount(int rootId, RootRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int rootId, RootRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    Database::DbSubContext &m_dbSubContext;
};

} // namespace Skribisto::Common::DirectAccess::Root
