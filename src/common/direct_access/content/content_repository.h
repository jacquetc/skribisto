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
#include "direct_access/content/content_events.h"
#include "direct_access/content/i_content_repository.h"
#include "direct_access/event_registry.h"
#include "entities/content.h"
#include <QPointer>

namespace Skribisto::Common::DirectAccess::Content
{
namespace SCE = Skribisto::Common::Entities;

class IContentTable
{
  public:
    virtual ~IContentTable() = default;

    // Creation assigns new ids
    virtual QList<SCE::Content> createMany(const QList<SCE::Content> &contents) = 0;

    // Update
    virtual QList<SCE::Content> updateMany(const QList<SCE::Content> &contents) = 0;

    // Query/Delete
    [[nodiscard]] virtual QList<SCE::Content> findMany(const QList<int> &ids) const = 0;
    virtual QList<int> removeMany(const QList<int> &ids) = 0;
    // Relationship setters/getters
    // Set the relationship value for a given Content id (e.g., set work id)
    virtual void setRelationshipIds(int contentId, ContentRelationshipField relationship, QList<int> relatedId) = 0;

    // Get the relationship value for a given Content id (e.g., get work id)
    [[nodiscard]] virtual QHash<int, QList<int>> getRelationshipIdsMany(
        const QList<int> &contentIds, ContentRelationshipField relationship) const = 0;
    virtual int getRelationshipIdsCount(int contentId, ContentRelationshipField relationship) = 0;
    virtual QList<int> getRelationshipIdsInRange(int contentId, ContentRelationshipField relationship, int offset,
                                                 int limit) = 0;
};

class ContentRepository : public IContentRepository
{
  public:
    ContentRepository(std::unique_ptr<IContentTable> table, Database::DbSubContext &dbSubContext,
                      QPointer<EventRegistry> eventRegistry);

    ~ContentRepository() override = default;

    // CRUD
    QList<SCE::Content> create(const QList<SCE::Content> &contents) override;
    QList<SCE::Content> get(const QList<int> &contentIds) override;
    QList<SCE::Content> update(const QList<SCE::Content> &contents) override;
    QList<int> remove(const QList<int> &contentIds) override;

    // Relationships
    void setRelationshipIds(int contentId, ContentRelationshipField relationship, QList<int> relatedId) override;
    QList<int> getRelationshipIds(int contentId, ContentRelationshipField relationship) override;
    QHash<int, QList<int>> getRelationshipIdsMany(const QList<int> &contentIds,
                                                  ContentRelationshipField relationship) override;
    int getRelationshipIdsCount(int contentId, ContentRelationshipField relationship) override;
    QList<int> getRelationshipIdsInRange(int contentId, ContentRelationshipField relationship, int offset,
                                         int limit) override;

  private:
    std::unique_ptr<IContentTable> m_table;
    QPointer<ContentEvents> m_events;        // not owned
    QPointer<EventRegistry> m_eventRegistry; // not owned

    // For cascade operations
    Database::DbSubContext &m_dbSubContext;

    void emitCreated(const QList<int> &ids) const;
    void emitUpdated(const QList<int> &ids) const;
    void emitRemoved(const QList<int> &ids) const;
    void emitRelationshipChanged(int contentId, ContentRelationshipField rel, const QList<int> &relatedIds) const;
};

} // namespace Skribisto::Common::DirectAccess::Content
