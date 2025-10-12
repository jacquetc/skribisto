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
#include "../use_cases/load_work_uc/i_load_work_uow.h"
#include "database/db_context.h"
#include "direct_access/event_registry.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "entities/binder_tag.h"
#include "entities/content.h"
#include "entities/recent_work.h"
#include "entities/root.h"
#include "entities/work.h"
#include "features/feature_event_registry.h"

#include <QPointer>

namespace Skribisto::WorkManagement
{
namespace SCDatabase = Skribisto::Common::Database;
namespace SCD = Skribisto::Common::DirectAccess;
namespace SCF = Skribisto::Common::Features;
namespace SCE = Common::Entities;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;

class LoadWorkUnitOfWork final : public ILoadWorkUnitOfWork
{

  public:
    LoadWorkUnitOfWork(SCDatabase::DbContext &dbContext, QPointer<SCD::EventRegistry> eventRegistry,
                       QPointer<SCF::FeatureEventRegistry> featureEventRegistry);

    ~LoadWorkUnitOfWork() override;
    void beginTransaction() override;
    void commit() override;
    void endTransaction() override;
    void rollback() override;
    void createSavepoint() override;
    void rollbackToSavepoint() override;
    void releaseSavepoint() override;
    QList<SCE::Root> createRoot(QList<SCE::Root> roots) override;
    void setRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship, QList<int> relatedIds) override;
    QList<SCE::Work> createWork(QList<SCE::Work> works) override;
    void setWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship, QList<int> relatedIds) override;
    QList<SCE::Binder> createBinder(QList<SCE::Binder> binders) override;
    void setBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship,
                               QList<int> relatedIds) override;
    QList<SCE::BinderItem> createBinderItem(QList<SCE::BinderItem> binderItems) override;
    void setBinderItemRelationship(int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship,
                                   QList<int> relatedIds) override;
    QList<SCE::BinderTag> createBinderTag(QList<SCE::BinderTag> binderTags) override;
    QList<SCE::Content> createContent(QList<SCE::Content> contents) override;
    QList<SCE::RecentWork> createRecentWork(QList<SCE::RecentWork> recentWorks) override;
    // signals
    void publishWorkLoaded(int workId);

  private:
    SCDatabase::DbSubContext m_dbSubContext;
    QPointer<SCD::EventRegistry> m_eventRegistry;
    QPointer<SCF::FeatureEventRegistry> m_featureEventRegistry;
};
} // namespace Skribisto::WorkManagement