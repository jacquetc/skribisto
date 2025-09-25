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

#include "binder_item_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDBinderItem = Skribisto::DirectAccess::BinderItem;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;

SDBinderItem::BinderItemUnitOfWork::BinderItemUnitOfWork(SCDatabase::DbContext &dbContext,
                                                         QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDBinderItem::BinderItemUnitOfWork::~BinderItemUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDBinderItem::BinderItemUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDBinderItem::BinderItemUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDBinderItem::BinderItemUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDBinderItem::BinderItemUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDBinderItem::BinderItemUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDBinderItem::BinderItemUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDBinderItem::BinderItemUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::BinderItem> SDBinderItem::BinderItemUnitOfWork::createBinderItem(
    QList<SCE::BinderItem> binderItems)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(binderItems);
}
QList<Skribisto::Common::Entities::BinderItem> SDBinderItem::BinderItemUnitOfWork::getBinderItem(
    QList<int> binderItemIds)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(binderItemIds);
}
QList<Skribisto::Common::Entities::BinderItem> SDBinderItem::BinderItemUnitOfWork::updateBinderItem(
    QList<SCE::BinderItem> binderItems)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(binderItems);
}
QList<int> SDBinderItem::BinderItemUnitOfWork::removeBinderItem(QList<int> binderItemIds)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(binderItemIds);
}
QList<int> SDBinderItem::BinderItemUnitOfWork::getBinderItemRelationship(
    int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIds(binderItemId, relationship);
}
void SDBinderItem::BinderItemUnitOfWork::setBinderItemRelationship(
    int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship, QList<int> relatedIds)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(binderItemId, relationship, relatedIds);
}
QHash<int, QList<int>> SDBinderItem::BinderItemUnitOfWork::getBinderItemRelationshipMany(
    const QList<int> &binderItemIds, SCDBinderItem::BinderItemRelationshipField relationship)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsMany(binderItemIds, relationship);
}
int SDBinderItem::BinderItemUnitOfWork::getBinderItemRelationshipCount(
    int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsCount(binderItemId, relationship);
}
QList<int> SDBinderItem::BinderItemUnitOfWork::getBinderItemRelationshipInRange(
    int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship, int offset, int limit)
{
    const auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsInRange(binderItemId, relationship, offset, limit);
}