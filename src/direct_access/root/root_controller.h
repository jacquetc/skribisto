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
#include "direct_access/root/root_repository.h"
#include "dtos.h"
#include <QCoroTask>

#include <QPointer>

namespace Skribisto::DirectAccess::Root
{
namespace SCDatabase = Skribisto::Common::Database;

class RootController : public QObject
{
    Q_OBJECT
  public:
    RootController(const RootController &) = delete;
    RootController &operator=(const RootController &) = delete;
    RootController(RootController &&) = delete;
    RootController &operator=(RootController &&) = delete;
    explicit RootController(QObject *parent = nullptr);
    QCoro::Task<QList<RootDto>> create(const QList<CreateRootDto> &roots);
    static CreateRootDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<RootDto>> get(const QList<int> &rootIds);
    // QList<RootDto> update(const QList<RootDto> &roots);
    // QList<int> remove(const QList<int> &rootIds);
    // QList<int> getRelationship(int rootId, Common::DirectAccess::Root::RootRelationshipField relationship);
    // void setRelationship(int rootId, Common::DirectAccess::Root::RootRelationshipField relationship,
    //                      QList<int> relatedIds);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
};
} // namespace Skribisto::DirectAccess::Root
