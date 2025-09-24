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

#include "content_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDContent = Skribisto::DirectAccess::Content;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;

SDContent::ContentUnitOfWork::ContentUnitOfWork(SCDatabase::DbContext &dbContext,
                                                QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDContent::ContentUnitOfWork::~ContentUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDContent::ContentUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDContent::ContentUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDContent::ContentUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDContent::ContentUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDContent::ContentUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDContent::ContentUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDContent::ContentUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::Content> SDContent::ContentUnitOfWork::createContent(QList<SCE::Content> contents)
{
    auto repository = SCD::RepositoryFactory::createContentRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(contents);
}
QList<Skribisto::Common::Entities::Content> SDContent::ContentUnitOfWork::getContent(QList<int> contentIds)
{
    auto repository = SCD::RepositoryFactory::createContentRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(contentIds);
}
QList<Skribisto::Common::Entities::Content> SDContent::ContentUnitOfWork::updateContent(QList<SCE::Content> contents)
{
    auto repository = SCD::RepositoryFactory::createContentRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(contents);
}
QList<int> SDContent::ContentUnitOfWork::removeContent(QList<int> contentIds)
{
    auto repository = SCD::RepositoryFactory::createContentRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(contentIds);
}