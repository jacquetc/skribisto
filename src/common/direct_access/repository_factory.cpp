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

#include "binder/binder_events.h"
#include "binder/binder_table.h"
#include "project/project_events.h"
#include "project/project_table.h"
#include "root/root_events.h"
#include "root/root_table.h"

namespace SCD = Skribisto::Common::DirectAccess;

namespace Skribisto::Common::DirectAccess::RepositoryFactory
{

// Original factory methods with individual event pointers
SCD::Root::RootRepository createRootRepository(Database::DbSubContext &dbSubContext,
                                               QPointer<EventRegistry> eventRegistry)
{
    const auto table = new SCD::Root::RootTable(dbSubContext);
    return SCD::Root::RootRepository{*table, dbSubContext, std::move(eventRegistry)};
}

SCD::Project::ProjectRepository createProjectRepository(Database::DbSubContext &dbSubContext,
                                                        QPointer<EventRegistry> eventRegistry)
{
    const auto table = new SCD::Project::ProjectTable(dbSubContext);
    return SCD::Project::ProjectRepository{*table, dbSubContext, std::move(eventRegistry)};
}

SCD::Binder::BinderRepository createBinderRepository(Database::DbSubContext &dbSubContext,
                                                     QPointer<EventRegistry> eventRegistry)
{
    const auto table = new SCD::Binder::BinderTable(dbSubContext);
    return SCD::Binder::BinderRepository{*table, dbSubContext, std::move(eventRegistry)};
}

} // namespace Skribisto::Common::DirectAccess::RepositoryFactory
