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

namespace Skribisto::DirectAccess::Content
{
namespace SCDatabase = Skribisto::Common::Database;

class ContentController : public QObject
{
    Q_OBJECT
  public:
    ContentController(const ContentController &) = delete;
    ContentController &operator=(const ContentController &) = delete;
    ContentController(ContentController &&) = delete;
    ContentController &operator=(ContentController &&) = delete;
    explicit ContentController(QObject *parent = nullptr);
    QCoro::Task<QList<ContentDto>> create(const QList<CreateContentDto> &contents);
    static CreateContentDto getCreateDto()
    {
        return {};
    }
    QCoro::Task<QList<ContentDto>> get(const QList<int> &contentIds);
    QCoro::Task<QList<ContentDto>> update(const QList<ContentDto> &contents);
    QCoro::Task<QList<int>> remove(const QList<int> &contentIds);

  private:
    void resolveDependencies();
    SCDatabase::DbContext *m_dbContext = nullptr;
    QPointer<Common::DirectAccess::EventRegistry> m_eventRegistry;
    QPointer<Common::UndoRedo::UndoRedoSystem> m_undoRedoSystem;
};
} // namespace Skribisto::DirectAccess::Content
