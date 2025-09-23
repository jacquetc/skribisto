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
#include "project/project_table.h"
#include "recent_project/recent_project_table.h"
#include "root/root_table.h"

namespace Skribisto::Common::DirectAccess::RepositoryFactory
{

namespace SCD = Skribisto::Common::DirectAccess;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;
namespace SCDProject = Skribisto::Common::DirectAccess::Project;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;
namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;

// Original factory methods with individual event pointers
std::unique_ptr<SCDRoot::RootRepository> createRootRepository(Database::DbSubContext &dbSubContext,
                                                              QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::Root::RootTable>(dbSubContext);
    return std::make_unique<SCD::Root::RootRepository>(std::move(table), dbSubContext, std::move(eventRegistry));
}

std::unique_ptr<SCDProject::ProjectRepository> createProjectRepository(Database::DbSubContext &dbSubContext,
                                                                       QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::Project::ProjectTable>(dbSubContext);
    return std::make_unique<SCD::Project::ProjectRepository>(std::move(table), dbSubContext, std::move(eventRegistry));
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
std::unique_ptr<SCDRecentProject::RecentProjectRepository> createRecentProjectRepository(
    Database::DbSubContext &dbSubContext, QPointer<EventRegistry> eventRegistry)
{
    auto table = std::make_unique<SCD::RecentProject::RecentProjectTable>(dbSubContext);
    return std::make_unique<SCDRecentProject::RecentProjectRepository>(std::move(table), dbSubContext,
                                                                       std::move(eventRegistry));
}

} // namespace Skribisto::Common::DirectAccess::RepositoryFactory
