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
#include "direct_access/binder/i_binder_repository.h"
#include "entities/binder.h"

#include <QString>

namespace Skribisto::DirectAccess::Binder
{
namespace SCE = Common::Entities;
namespace SCDBinder = Common::DirectAccess::Binder;

class IBinderUnitOfWork
{
  public:
    virtual ~IBinderUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::Binder> createBinder(QList<SCE::Binder> binders) = 0;
    virtual QList<SCE::Binder> getBinder(QList<int> binderIds) = 0;
    virtual QList<SCE::Binder> updateBinder(QList<SCE::Binder> binders) = 0;
    virtual QList<int> removeBinder(QList<int> binderIds) = 0;
    virtual QList<int> getBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship) = 0;
    virtual void setBinderRelationship(int binderId, SCDBinder::BinderRelationshipField relationship,
                                       QList<int> relatedIds) = 0;
    virtual QHash<int, QList<int>> getBinderRelationshipMany(const QList<int> &binderIds,
                                                             SCDBinder::BinderRelationshipField relationship) = 0;
    virtual int getBinderRelationshipCount(int binderId, SCDBinder::BinderRelationshipField relationship) = 0;
    virtual QList<int> getBinderRelationshipInRange(int binderId, SCDBinder::BinderRelationshipField relationship,
                                                    int offset, int limit) = 0;
};
} // namespace Skribisto::DirectAccess::Binder