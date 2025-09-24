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

#include "recent_work_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDRecentWork = Skribisto::DirectAccess::RecentWork;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;

SDRecentWork::RecentWorkUnitOfWork::RecentWorkUnitOfWork(SCDatabase::DbContext &dbContext,
                                                         QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDRecentWork::RecentWorkUnitOfWork::~RecentWorkUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDRecentWork::RecentWorkUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDRecentWork::RecentWorkUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDRecentWork::RecentWorkUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDRecentWork::RecentWorkUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDRecentWork::RecentWorkUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDRecentWork::RecentWorkUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDRecentWork::RecentWorkUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::RecentWork> SDRecentWork::RecentWorkUnitOfWork::createRecentWork(
    QList<SCE::RecentWork> recentWorks)
{
    auto repository = SCD::RepositoryFactory::createRecentWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(recentWorks);
}
QList<Skribisto::Common::Entities::RecentWork> SDRecentWork::RecentWorkUnitOfWork::getRecentWork(
    QList<int> recentWorkIds)
{
    auto repository = SCD::RepositoryFactory::createRecentWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(recentWorkIds);
}
QList<Skribisto::Common::Entities::RecentWork> SDRecentWork::RecentWorkUnitOfWork::updateRecentWork(
    QList<SCE::RecentWork> recentWorks)
{
    auto repository = SCD::RepositoryFactory::createRecentWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(recentWorks);
}
QList<int> SDRecentWork::RecentWorkUnitOfWork::removeRecentWork(QList<int> recentWorkIds)
{
    auto repository = SCD::RepositoryFactory::createRecentWorkRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(recentWorkIds);
}