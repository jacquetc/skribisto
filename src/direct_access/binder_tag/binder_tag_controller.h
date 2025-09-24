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
#include "direct_access/event_registry.h"
#include "dtos.h"
#include <QCoro/QCoroTask>

#include <QPointer>

namespace Skribisto::Common::UndoRedo
{
class UndoRedoSystem;
}

namespace Skribisto::DirectAccess::BinderTag
{
namespace SCDatabase = Skribisto::Common::Database;

class BinderTagController : public QObject
{
    Q_OBJECT
  public:
    BinderTagController(const BinderTagController &) = delete;
    BinderTagController &operator=(const BinderTagController &) = delete;
    BinderTagController(BinderTagController &&) = delete;
    BinderTagController &operator=(BinderTagController &&) = delete;
    explicit BinderTagController(QObject *parent = nullptr);
    QCoro::Task<QList<BinderTagDto>> create(const QList<CreateBinderTagDto> &binderTags);
    static CreateBinderTagDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<BinderTagDto>> get(const QList<int> &binderTagIds);
    QCoro::Task<QList<BinderTagDto>> update(const QList<BinderTagDto> &binderTags);
    QCoro::Task<QList<int>> remove(const QList<int> &binderTagIds);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::BinderTag
