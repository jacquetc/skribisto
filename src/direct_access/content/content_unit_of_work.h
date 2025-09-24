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

#pragma once
#include "database/db_context.h"
#include "direct_access/content/content_events.h"
#include "direct_access/event_registry.h"
#include "use_cases/i_content_unit_of_work.h"

#include <QPointer>

namespace Skribisto::DirectAccess::Content
{
namespace SCE = Common::Entities;
namespace SCDatabase = Skribisto::Common::Database;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;
namespace SCD = Skribisto::Common::DirectAccess;

class ContentUnitOfWork final : public IContentUnitOfWork
{

  public:
    ContentUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry);

    ~ContentUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::Content> createContent(QList<SCE::Content> contents) override;
    QList<SCE::Content> getContent(QList<int> contentIds) override;
    QList<SCE::Content> updateContent(QList<SCE::Content> contents) override;
    QList<int> removeContent(QList<int> contentIds) override;

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::Content