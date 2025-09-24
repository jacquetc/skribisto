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

#include "repository_factory.h"
#include "binder/binder_table.h"
#include "binder_item/binder_item_table.h"
#include "binder_tag/binder_tag_table.h"
#include "content/content_table.h"
#include "recent_work/recent_work_table.h"
#include "root/root_table.h"
#include "work/work_table.h"

namespace Skribisto::Common::DirectAccess::RepositoryFactory
{

namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;

// Original factory methods with individual event pointers
std::unique_ptr<SCDRoot::RootRepository> createRootRepository(Database::DbSubContext &dbSubContext,
                                                              QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::Root::RootTable>(dbSubContext);
    return std::make_unique<SCD::Root::RootRepository>(std::move(table), dbSubContext, std::move(eventRegistry));
}

std::unique_ptr<SCDWork::WorkRepository> createWorkRepository(Database::DbSubContext &dbSubContext,
                                                              QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::Work::WorkTable>(dbSubContext);
    return std::make_unique<SCD::Work::WorkRepository>(std::move(table), dbSubContext, std::move(eventRegistry));
}

std::unique_ptr<SCDBinder::BinderRepository> createBinderRepository(Database::DbSubContext &dbSubContext,
                                                                    QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::Binder::BinderTable>(dbSubContext);
    return std::make_unique<SCDBinder::BinderRepository>(std::move(table), dbSubContext, std::move(eventRegistry));
}

std::unique_ptr<SCDBinderItem::BinderItemRepository> createBinderItemRepository(Database::DbSubContext &dbSubContext,
                                                                                QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::BinderItem::BinderItemTable>(dbSubContext);
    return std::make_unique<SCDBinderItem::BinderItemRepository>(std::move(table), dbSubContext,
                                                                 std::move(eventRegistry));
}
std::unique_ptr<SCDRecentWork::RecentWorkRepository> createRecentWorkRepository(Database::DbSubContext &dbSubContext,
                                                                                QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::RecentWork::RecentWorkTable>(dbSubContext);
    return std::make_unique<SCDRecentWork::RecentWorkRepository>(std::move(table), dbSubContext,
                                                                 std::move(eventRegistry));
}

std::unique_ptr<SCDContent::ContentRepository> createContentRepository(Database::DbSubContext &dbSubContext,
                                                                       QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::Content::ContentTable>(dbSubContext);
    return std::make_unique<SCD::Content::ContentRepository>(std::move(table), dbSubContext, std::move(eventRegistry));
}

std::unique_ptr<SCDBinderTag::BinderTagRepository> createBinderTagRepository(Database::DbSubContext &dbSubContext,
                                                                             QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::BinderTag::BinderTagTable>(dbSubContext);
    return std::make_unique<SCDBinderTag::BinderTagRepository>(std::move(table), dbSubContext,
                                                               std::move(eventRegistry));
}

} // namespace Skribisto::Common::DirectAccess::RepositoryFactory
