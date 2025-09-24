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
#include "direct_access/recent_work/i_recent_work_repository.h"
#include "entities/recent_work.h"

#include <QString>

namespace Skribisto::DirectAccess::RecentWork
{
namespace SCE = Common::Entities;
namespace SCDRecentWork = Common::DirectAccess::RecentWork;

class IRecentWorkUnitOfWork
{
  public:
    virtual ~IRecentWorkUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::RecentWork> createRecentWork(QList<SCE::RecentWork> recentWorks) = 0;
    virtual QList<SCE::RecentWork> getRecentWork(QList<int> recentWorkIds) = 0;
    virtual QList<SCE::RecentWork> updateRecentWork(QList<SCE::RecentWork> recentWorks) = 0;
    virtual QList<int> removeRecentWork(QList<int> recentWorkIds) = 0;
};
} // namespace Skribisto::DirectAccess::RecentWork