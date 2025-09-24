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
#include "direct_access/content/i_content_repository.h"
#include "entities/content.h"

#include <QString>

namespace Skribisto::DirectAccess::Content
{
namespace SCE = Common::Entities;
namespace SCDContent = Common::DirectAccess::Content;

class IContentUnitOfWork
{
  public:
    virtual ~IContentUnitOfWork() = default;
    virtual void beginTransaction() = 0;
    virtual void commit() = 0;
    virtual void endTransaction() = 0;
    virtual void rollback() = 0;

    virtual void createSavepoint() = 0;
    virtual void rollbackToSavepoint() = 0;
    virtual void releaseSavepoint() = 0;

    virtual QList<SCE::Content> createContent(QList<SCE::Content> contents) = 0;
    virtual QList<SCE::Content> getContent(QList<int> contentIds) = 0;
    virtual QList<SCE::Content> updateContent(QList<SCE::Content> contents) = 0;
    virtual QList<int> removeContent(QList<int> contentIds) = 0;
};
} // namespace Skribisto::DirectAccess::Content