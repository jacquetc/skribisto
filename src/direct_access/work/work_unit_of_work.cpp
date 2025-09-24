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

#include "work_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDWork = Skribisto::DirectAccess::Work;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;

SDWork::WorkUnitOfWork::WorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDWork::WorkUnitOfWork::~WorkUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDWork::WorkUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDWork::WorkUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDWork::WorkUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDWork::WorkUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDWork::WorkUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDWork::WorkUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDWork::WorkUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::Work> SDWork::WorkUnitOfWork::createWork(QList<SCE::Work> works)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(works);
}
QList<Skribisto::Common::Entities::Work> SDWork::WorkUnitOfWork::getWork(QList<int> workIds)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(workIds);
}
QList<Skribisto::Common::Entities::Work> SDWork::WorkUnitOfWork::updateWork(QList<SCE::Work> works)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(works);
}
QList<int> SDWork::WorkUnitOfWork::removeWork(QList<int> workIds)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(workIds);
}
QList<int> SDWork::WorkUnitOfWork::getWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIds(workId, relationship);
}
void SDWork::WorkUnitOfWork::setWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship,
                                                 QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(workId, relationship, relatedIds);
}
QHash<int, QList<int>> SDWork::WorkUnitOfWork::getWorkRelationshipMany(const QList<int> &workIds,
                                                                       SCDWork::WorkRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsMany(workIds, relationship);
}
int SDWork::WorkUnitOfWork::getWorkRelationshipCount(int workId, SCDWork::WorkRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsCount(workId, relationship);
}
QList<int> SDWork::WorkUnitOfWork::getWorkRelationshipInRange(int workId, SCDWork::WorkRelationshipField relationship,
                                                              int offset, int limit)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsInRange(workId, relationship, offset, limit);
}