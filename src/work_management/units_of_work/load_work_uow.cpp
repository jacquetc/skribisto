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

#include "load_work_uow.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"
namespace Skribisto::WorkManagement
{
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCE = Common::Entities;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;

LoadWorkUnitOfWork::LoadWorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry,
                                       QPointer<SCF::FeatureEventRegistry> featureEventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry)),
      m_featureEventRegistry(std::move(featureEventRegistry))
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
QList<SCE::Work> LoadWorkUnitOfWork::createWork(QList<SCE::Work> works)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(works);
}
void LoadWorkUnitOfWork::setWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship,
                                             QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createWorkRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(workId, relationship, relatedIds);
}
QList<SCE::Binder> LoadWorkUnitOfWork::createBinder(QList<SCE::Binder> binders)
{
    auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(binders);
}
void LoadWorkUnitOfWork::setBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship,
                                               QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createBinderRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(binderId, relationship, relatedIds);
}
QList<SCE::BinderItem> LoadWorkUnitOfWork::createBinderItem(QList<SCE::BinderItem> binderItems)
{
    auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(binderItems);
}
void LoadWorkUnitOfWork::setBinderItemRelationship(int binderItemId,
                                                   SCDBinderItem::BinderItemRelationshipField relationship,
                                                   QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createBinderItemRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(binderItemId, relationship, relatedIds);
}
QList<SCE::BinderTag> LoadWorkUnitOfWork::createBinderTag(QList<SCE::BinderTag> binderTags)
{
    auto repository = SCD::RepositoryFactory::createBinderTagRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(binderTags);
}
QList<SCE::Content> LoadWorkUnitOfWork::createContent(QList<SCE::Content> contents)
{
    auto repository = SCD::RepositoryFactory::createContentRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(contents);
}
QList<SCE::RecentWork> LoadWorkUnitOfWork::createRecentWork(QList<SCE::RecentWork> recentWorks)
{
    auto repository = SCD::RepositoryFactory::createRecentWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(recentWorks);
}
void LoadWorkUnitOfWork::publishWorkLoaded(int workId)
{
    m_featureEventRegistry->getEvents<SCF::WorkManagementEvents>()->publishWorkLoaded(workId);
}
} // namespace Skribisto::WorkManagement