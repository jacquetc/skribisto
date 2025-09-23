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
#include "direct_access/root/i_root_repository.h"
#include "entities/root.h"

#include <QString>

namespace Skribisto::DirectAccess::Root
{
namespace SCE = Common::Entities;
namespace SCDRoot = Common::DirectAccess::Root;

class IRootUnitOfWork
{
  public:
    virtual ~IRootUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::Root> createRoot(QList<SCE::Root> roots) = 0;
    virtual QList<SCE::Root> getRoot(QList<int> rootIds) = 0;
    virtual QList<SCE::Root> updateRoot(QList<SCE::Root> roots) = 0;
    virtual QList<int> removeRoot(QList<int> rootIds) = 0;
    virtual QList<int> getRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship) = 0;
    virtual void setRootRelationship(int rootId, SCDRoot::RootRelationshipField relationship,
                                     QList<int> relatedIds) = 0;
    virtual QHash<int, QList<int>> getRootRelationshipMany(const QList<int> &rootIds,
                                                          SCDRoot::RootRelationshipField relationship) = 0;
    virtual int getRootRelationshipCount(int rootId, SCDRoot::RootRelationshipField relationship) = 0;
    virtual QList<int> getRootRelationshipInRange(int rootId, SCDRoot::RootRelationshipField relationship,
                                                  int offset, int limit) = 0;
};
} // namespace Skribisto::DirectAccess::Root