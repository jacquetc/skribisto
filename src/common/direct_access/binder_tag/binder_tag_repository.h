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
#include "direct_access/binder_tag/binder_tag_events.h"
#include "direct_access/binder_tag/i_binder_tag_repository.h"
#include "direct_access/event_registry.h"
#include "entities/binder_tag.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::BinderTag
{
namespace SCE = Skribisto::Common::Entities;

class IBinderTagTable
{
  public:
    virtual ~IBinderTagTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::BinderTag> createMany(const QList<SCE::BinderTag> &binderTags) = 0;

    // Update
    virtual QList<SCE::BinderTag> updateMany(const QList<SCE::BinderTag> &binderTags) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::BinderTag> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given BinderTag id (e.g., set project id)
    virtual void setRelationshipIds(int binderTagId, BinderTagRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given BinderTag id (e.g., get project id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(
        const QList<int> &binderTagIds, BinderTagRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int binderTagId, BinderTagRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int binderTagId, BinderTagRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

class BinderTagRepository : public IBinderTagRepository
{
  public:
    BinderTagRepository(std::unique_ptr<IBinderTagTable> table, Database::DbSubContext &dbSubContext,
                        QPointer<EventRegistry> eventRegistry);

    ~BinderTagRepository() override = default;

    // CRUD
    QList<SCE::BinderTag> create(const QList<SCE::BinderTag> &binderTags) override;
    QList<SCE::BinderTag> get(const QList<int> &binderTagIds) override;
    QList<SCE::BinderTag> update(const QList<SCE::BinderTag> &binderTags) override;
    QList<int> remove(const QList<int> &binderTagIds) override;

    // Relationships
    void setRelationshipIds(int binderTagId, BinderTagRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationshipIds(int binderTagId, BinderTagRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &binderTagIds,
                                                  BinderTagRelationshipField relationship) override;
    int getRelationshipIdsCount(int binderTagId, BinderTagRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int binderTagId, BinderTagRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IBinderTagTable> m_table;
    QPointer<BinderTagEvents> m_events;      // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int binderTagId, BinderTagRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::BinderTag
