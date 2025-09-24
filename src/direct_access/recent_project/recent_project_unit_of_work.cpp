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

#include "recent_project_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDRecentProject = Skribisto::DirectAccess::RecentProject;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;

SDRecentProject::RecentProjectUnitOfWork::RecentProjectUnitOfWork(SCDatabase::DbContext &dbContext,
                                                                  QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDRecentProject::RecentProjectUnitOfWork::~RecentProjectUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDRecentProject::RecentProjectUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDRecentProject::RecentProjectUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDRecentProject::RecentProjectUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDRecentProject::RecentProjectUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDRecentProject::RecentProjectUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDRecentProject::RecentProjectUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDRecentProject::RecentProjectUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::RecentProject> SDRecentProject::RecentProjectUnitOfWork::createRecentProject(
    QList<SCE::RecentProject> recentProjects)
{
    auto repository = SCD::RepositoryFactory::createRecentProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(recentProjects);
}
QList<Skribisto::Common::Entities::RecentProject> SDRecentProject::RecentProjectUnitOfWork::getRecentProject(
    QList<int> recentProjectIds)
{
    auto repository = SCD::RepositoryFactory::createRecentProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(recentProjectIds);
}
QList<Skribisto::Common::Entities::RecentProject> SDRecentProject::RecentProjectUnitOfWork::updateRecentProject(
    QList<SCE::RecentProject> recentProjects)
{
    auto repository = SCD::RepositoryFactory::createRecentProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(recentProjects);
}
QList<int> SDRecentProject::RecentProjectUnitOfWork::removeRecentProject(QList<int> recentProjectIds)
{
    auto repository = SCD::RepositoryFactory::createRecentProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(recentProjectIds);
}