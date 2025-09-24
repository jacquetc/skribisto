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

#include "load_work_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"
namespace Skribisto::WorkManagement
{
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;

LoadWorkUnitOfWork::LoadWorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
LoadWorkUnitOfWork::~LoadWorkUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void LoadWorkUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void LoadWorkUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void LoadWorkUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void LoadWorkUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void LoadWorkUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void LoadWorkUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void LoadWorkUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::Root> LoadWorkUnitOfWork::createRoot(QList<SCE::Root> roots)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(roots);
}
void LoadWorkUnitOfWork::setRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship,
                                             QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createRootRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(rootId, relationship, relatedIds);
}
} // namespace Skribisto::WorkManagement