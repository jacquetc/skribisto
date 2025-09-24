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
#include "direct_access/work/i_work_repository.h"
#include "entities/work.h"

#include <QString>

namespace Skribisto::DirectAccess::Work
{
namespace SCE = Common::Entities;
namespace SCDWork = Common::DirectAccess::Work;

class IWorkUnitOfWork
{
  public:
    virtual ~IWorkUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::Work> createWork(QList<SCE::Work> works) = 0;
    virtual QList<SCE::Work> getWork(QList<int> workIds) = 0;
    virtual QList<SCE::Work> updateWork(QList<SCE::Work> works) = 0;
    virtual QList<int> removeWork(QList<int> workIds) = 0;
    virtual QList<int> getWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship) = 0;
    virtual void setWorkRelationship(int workId, SCDWork::WorkRelationshipField relationship,
                                     QList<int> relatedIds) = 0;
    virtual QHash<int, QList<int>> getWorkRelationshipMany(const QList<int> &workIds,
                                                           SCDWork::WorkRelationshipField relationship) = 0;
    virtual int getWorkRelationshipCount(int workId, SCDWork::WorkRelationshipField relationship) = 0;
    virtual QList<int> getWorkRelationshipInRange(int workId, SCDWork::WorkRelationshipField relationship, int offset,
                                                  int limit) = 0;
};
} // namespace Skribisto::DirectAccess::Work