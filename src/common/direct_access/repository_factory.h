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
#include "direct_access/binder/binder_repository.h"
#include "direct_access/binder_item/binder_item_repository.h"
#include "direct_access/binder_tag/binder_tag_repository.h"
#include "direct_access/content/content_repository.h"
#include "direct_access/event_registry.h"
#include "direct_access/recent_work/recent_work_repository.h"
#include "direct_access/root/root_repository.h"
#include "direct_access/work/work_repository.h"

#include <QPointer>

namespace Skribisto::Common::DirectAccess::RepositoryFactory
{
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;

// Original methods with individual event pointers
std::unique_ptr<SCDRoot::RootRepository> createRootRepository(Database::DbSubContext &dbSubContext,
                                                              QPointer<EventRegistry> eventRegistry);
std::unique_ptr<SCDWork::WorkRepository> createWorkRepository(Database::DbSubContext &dbSubContext,
                                                              QPointer<EventRegistry> eventRegistry);
std::unique_ptr<SCDBinder::BinderRepository> createBinderRepository(Database::DbSubContext &dbSubContext,
                                                                    QPointer<EventRegistry> eventRegistry);
std::unique_ptr<SCDBinderItem::BinderItemRepository> createBinderItemRepository(Database::DbSubContext &dbSubContext,
                                                                                QPointer<EventRegistry> eventRegistry);
std::unique_ptr<SCDBinderTag::BinderTagRepository> createBinderTagRepository(Database::DbSubContext &dbSubContext,
                                                                             QPointer<EventRegistry> eventRegistry);
std::unique_ptr<SCDRecentWork::RecentWorkRepository> createRecentWorkRepository(Database::DbSubContext &dbSubContext,
                                                                                QPointer<EventRegistry> eventRegistry);
std::unique_ptr<SCDContent::ContentRepository> createContentRepository(Database::DbSubContext &dbSubContext,
                                                                       QPointer<EventRegistry> eventRegistry);

} // namespace Skribisto::Common::DirectAccess::RepositoryFactory
