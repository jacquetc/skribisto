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

#include "root_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDRoot = Skribisto::DirectAccess::Root;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;

SDRoot::RootUnitOfWork::RootUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDRoot::RootUnitOfWork::~RootUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDRoot::RootUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDRoot::RootUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDRoot::RootUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDRoot::RootUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDRoot::RootUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDRoot::RootUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDRoot::RootUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::Root> SDRoot::RootUnitOfWork::createRoot(QList<SCE::Root> roots)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    return repository.create(roots);
}
QList<Skribisto::Common::Entities::Root> SDRoot::RootUnitOfWork::getRoot(QList<int> rootIds)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    return repository.get(rootIds);
}
QList<Skribisto::Common::Entities::Root> SDRoot::RootUnitOfWork::updateRoot(QList<SCE::Root> roots)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    return repository.update(roots);
}
QList<int> SDRoot::RootUnitOfWork::removeRoot(QList<int> rootIds)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    return repository.remove(rootIds);
}
QList<int> SDRoot::RootUnitOfWork::getRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    return repository.getRelationship(rootId, relationship);
}
void SDRoot::RootUnitOfWork::setRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship,
                                                 QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    repository.setRelationship(rootId, relationship, relatedIds);
}