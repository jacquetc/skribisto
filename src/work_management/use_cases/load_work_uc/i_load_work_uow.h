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

#pragma once
#include "database/db_context.h"
#include "direct_access/binder/i_binder_repository.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "direct_access/root/i_root_repository.h"
#include "direct_access/work/i_work_repository.h"
#include "entities/binder.h"
#include "entities/binder_item.h"
#include "entities/binder_tag.h"
#include "entities/content.h"
#include "entities/recent_work.h"
#include "entities/root.h"
#include "entities/work.h"
#include <QList>
#include <memory>

namespace Skribisto::WorkManagement
{
namespace SCE = Common::Entities;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;

class ILoadWorkUnitOfWork
{
  public:
    virtual ~ILoadWorkUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::Root> createRoot(QList<SCE::Root> roots) = 0;
    virtual void setRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship,
                                     QList<int> relatedIds) = 0;
    virtual QList<SCE::Work> createWork(QList<SCE::Work> works) = 0;
    virtual void setWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship,
                                     QList<int> relatedIds) = 0;
    virtual QList<SCE::Binder> createBinder(QList<SCE::Binder> binders) = 0;
    virtual void setBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship,
                                       QList<int> relatedIds) = 0;
    virtual QList<SCE::BinderItem> createBinderItem(QList<SCE::BinderItem> binderItems) = 0;
    virtual void setBinderItemRelationship(int binderItemId, SCDBinderItem::BinderItemRelationshipField relationship,
                                           QList<int> relatedIds) = 0;
    virtual QList<SCE::BinderTag> createBinderTag(QList<SCE::BinderTag> binderTags) = 0;
    virtual QList<SCE::RecentWork> createRecentWork(QList<SCE::RecentWork> recentWorks) = 0;
    virtual QList<SCE::Content> createContent(QList<SCE::Content> contents) = 0;
};
} // namespace Skribisto::WorkManagement