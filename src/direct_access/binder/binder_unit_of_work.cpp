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

#include "binder_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDBinder = Skribisto::DirectAccess::Binder;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;

SDBinder::BinderUnitOfWork::BinderUnitOfWork(SCDatabase::DbContext &dbContext,
                                             QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDBinder::BinderUnitOfWork::~BinderUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDBinder::BinderUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDBinder::BinderUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDBinder::BinderUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDBinder::BinderUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDBinder::BinderUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDBinder::BinderUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDBinder::BinderUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::Binder> SDBinder::BinderUnitOfWork::createBinder(QList<SCE::Binder> binders)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(binders);
}
QList<Skribisto::Common::Entities::Binder> SDBinder::BinderUnitOfWork::getBinder(QList<int> binderIds)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(binderIds);
}
QList<Skribisto::Common::Entities::Binder> SDBinder::BinderUnitOfWork::updateBinder(QList<SCE::Binder> binders)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(binders);
}
QList<int> SDBinder::BinderUnitOfWork::removeBinder(QList<int> binderIds)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(binderIds);
}
QList<int> SDBinder::BinderUnitOfWork::getBinderRelationship(int binderId,
                                                             SCDBinder::BinderRelationshipField relationship)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIds(binderId, relationship);
}
void SDBinder::BinderUnitOfWork::setBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship,
                                                       QList<int> relatedIds)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(binderId, relationship, relatedIds);
}
QHash<int, QList<int>> SDBinder::BinderUnitOfWork::getBinderRelationshipMany(
    const QList<int> &binderIds, SCDBinder::BinderRelationshipField relationship)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsMany(binderIds, relationship);
}
int SDBinder::BinderUnitOfWork::getBinderRelationshipCount(int binderId,
                                                           SCDBinder::BinderRelationshipField relationship)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsCount(binderId, relationship);
}
QList<int> SDBinder::BinderUnitOfWork::getBinderRelationshipInRange(int binderId,
                                                                    SCDBinder::BinderRelationshipField relationship,
                                                                    int offset, int limit)
{
    const auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsInRange(binderId, relationship, offset, limit);
}