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
#include "direct_access/content/content_repository.h"
#include "entities/content.h"

#include <QList>

namespace Skribisto::Common::DirectAccess::Content
{
namespace SCE = Skribisto::Common::Entities;

class ContentTable final : public IContentTable
{
  public:
    explicit ContentTable(Database::DbSubContext &dbSubContext);

    QList<SCE::Content> createMany(const QList<SCE::Content> &contents) override;
    QList<SCE::Content> updateMany(const QList<SCE::Content> &contents) override;
    [[nodiscard]] QList<SCE::Content> findMany(const QList<int> &ids) const override;
    QList<int> removeMany(const QList<int> &ids) override;
    void setRelationshipIds(int contentId, ContentRelationshipField relationship, QList<int> relatedId) override;
    [[nodiscard]] QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &contentIds,
                                                                ContentRelationshipField relationship) const override;
    int getRelationshipIdsCount(int contentId, ContentRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int contentId, ContentRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    Database::DbSubContext &m_dbSubContext;
};

} // namespace Skribisto::Common::DirectAccess::Content
