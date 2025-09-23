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

#include "project_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDProject = Skribisto::DirectAccess::Project;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDProject = Skribisto::Common::DirectAccess::Project;

SDProject::ProjectUnitOfWork::ProjectUnitOfWork(SCDatabase::DbContext &dbContext,
                                                QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDProject::ProjectUnitOfWork::~ProjectUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDProject::ProjectUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDProject::ProjectUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDProject::ProjectUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDProject::ProjectUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDProject::ProjectUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDProject::ProjectUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDProject::ProjectUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::Project> SDProject::ProjectUnitOfWork::createProject(QList<SCE::Project> projects)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(projects);
}
QList<Skribisto::Common::Entities::Project> SDProject::ProjectUnitOfWork::getProject(QList<int> projectIds)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(projectIds);
}
QList<Skribisto::Common::Entities::Project> SDProject::ProjectUnitOfWork::updateProject(QList<SCE::Project> projects)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(projects);
}
QList<int> SDProject::ProjectUnitOfWork::removeProject(QList<int> projectIds)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(projectIds);
}
QList<int> SDProject::ProjectUnitOfWork::getProjectRelationship(int projectId,
                                                                SCDProject::ProjectRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIds(projectId, relationship);
}
void SDProject::ProjectUnitOfWork::setProjectRelationship(int projectId,
                                                          SCDProject::ProjectRelationshipField relationship,
                                                          QList<int> relatedIds)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    repository->setRelationshipIds(projectId, relationship, relatedIds);
}
QHash<int, QList<int>> SDProject::ProjectUnitOfWork::getProjectRelationshipMany(
    const QList<int> &projectIds, SCDProject::ProjectRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsMany(projectIds, relationship);
}
int SDProject::ProjectUnitOfWork::getProjectRelationshipCount(int projectId,
                                                              SCDProject::ProjectRelationshipField relationship)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsCount(projectId, relationship);
}
QList<int> SDProject::ProjectUnitOfWork::getProjectRelationshipInRange(
    int projectId, SCDProject::ProjectRelationshipField relationship, int offset, int limit)
{
    auto repository = SCD::RepositoryFactory::createProjectRepository(m_dbSubContext, m_eventRegistry);
    return repository->getRelationshipIdsInRange(projectId, relationship, offset, limit);
}