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
#include "direct_access/event_registry.h"
#include "direct_access/project/project_repository.h"
#include "direct_access/root/root_repository.h"

#include <QPointer>

namespace Skribisto::Common::DirectAccess::RepositoryFactory
{
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;

// Original methods with individual event pointers
SCDRoot::RootRepository createRootRepository(Database::DbSubContext &dbSubContext,
                                             QPointer<EventRegistry> eventRegistry);
Project::ProjectRepository createProjectRepository(Database::DbSubContext &dbSubContext,
                                                   QPointer<EventRegistry> eventRegistry);
Binder::BinderRepository createBinderRepository(Database::DbSubContext &dbSubContext,
                                                QPointer<EventRegistry> eventRegistry);

} // namespace Skribisto::Common::DirectAccess::RepositoryFactory
