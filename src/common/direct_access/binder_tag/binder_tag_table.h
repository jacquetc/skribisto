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
#include "direct_access/binder_tag/binder_tag_repository.h"
#include "entities/binder_tag.h"

#include <QList>

namespace Skribisto::Common::DirectAccess::BinderTag
{
namespace SCE = Skribisto::Common::Entities;

class BinderTagTable final : public IBinderTagTable
{
  public:
    explicit BinderTagTable(Database::DbSubContext &dbSubContext);

    QList<SCE::BinderTag> createMany(const QList<SCE::BinderTag> &binderTags) override;
    QList<SCE::BinderTag> updateMany(const QList<SCE::BinderTag> &binderTags) override;
    [[nodiscard]] QList<SCE::BinderTag> findMany(const QList<int> &ids) const override;
    QList<int> removeMany(const QList<int> &ids) override;
    void setRelationshipIds(int binderTagId, BinderTagRelationshipField relationship, QList<int> relatedId) override;
    [[nodiscard]] QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderTagIds,
                                                                BinderTagRelationshipField relationship) const override;
    int getRelationshipIdsCount(int binderTagId, BinderTagRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int binderTagId, BinderTagRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    Database::DbSubContext &m_dbSubContext;
};

} // namespace Skribisto::Common::DirectAccess::BinderTag
