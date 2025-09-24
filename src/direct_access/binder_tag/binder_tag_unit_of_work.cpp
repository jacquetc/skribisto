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

#include "binder_tag_unit_of_work.h"

#include "database/db_context.h"
#include "direct_access/repository_factory.h"

namespace SDBinderTag = Skribisto::DirectAccess::BinderTag;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;

SDBinderTag::BinderTagUnitOfWork::BinderTagUnitOfWork(SCDatabase::DbContext &dbContext,
                                                      QPointer<SCD::EventRegistry> eventRegistry)
    : m_dbSubContext(SCDatabase::DbSubContext(dbContext)), m_eventRegistry(std::move(eventRegistry))
{
}
SDBinderTag::BinderTagUnitOfWork::~BinderTagUnitOfWork()
{
    // connection is closed automatically when DbSubContext is destroyed
}
void SDBinderTag::BinderTagUnitOfWork::beginTransaction()
{
    m_dbSubContext.beginTransaction();
}
void SDBinderTag::BinderTagUnitOfWork::commit()
{
    m_dbSubContext.commit();
}
void SDBinderTag::BinderTagUnitOfWork::endTransaction()
{
    m_dbSubContext.endTransaction();
}
void SDBinderTag::BinderTagUnitOfWork::rollback()
{
    m_dbSubContext.rollback();
}
void SDBinderTag::BinderTagUnitOfWork::createSavepoint()
{
    m_dbSubContext.createSavepoint();
}
void SDBinderTag::BinderTagUnitOfWork::rollbackToSavepoint()
{
    m_dbSubContext.rollbackToSavepoint();
}
void SDBinderTag::BinderTagUnitOfWork::releaseSavepoint()
{
    m_dbSubContext.releaseSavepoint();
}
QList<Skribisto::Common::Entities::BinderTag> SDBinderTag::BinderTagUnitOfWork::createBinderTag(
    QList<SCE::BinderTag> binderTags)
{
    auto repository = SCD::RepositoryFactory::createBinderTagRepository(m_dbSubContext, m_eventRegistry);
    return repository->create(binderTags);
}
QList<Skribisto::Common::Entities::BinderTag> SDBinderTag::BinderTagUnitOfWork::getBinderTag(QList<int> binderTagIds)
{
    auto repository = SCD::RepositoryFactory::createBinderTagRepository(m_dbSubContext, m_eventRegistry);
    return repository->get(binderTagIds);
}
QList<Skribisto::Common::Entities::BinderTag> SDBinderTag::BinderTagUnitOfWork::updateBinderTag(
    QList<SCE::BinderTag> binderTags)
{
    auto repository = SCD::RepositoryFactory::createBinderTagRepository(m_dbSubContext, m_eventRegistry);
    return repository->update(binderTags);
}
QList<int> SDBinderTag::BinderTagUnitOfWork::removeBinderTag(QList<int> binderTagIds)
{
    auto repository = SCD::RepositoryFactory::createBinderTagRepository(m_dbSubContext, m_eventRegistry);
    return repository->remove(binderTagIds);
}