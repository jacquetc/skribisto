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

#include "content/dtos.h"
#include "direct_access/content/i_content_repository.h"
#include "entities/content.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::Content
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDContent = Skribisto::Common::DirectAccess::Content;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::Content toEntity(const CreateContentDto &dto)
    {
        SCE::Content content;
        content.id = 0;
        content.createdAt = dto.createdAt;
        content.updatedAt = dto.updatedAt;
        content.role = dto.role;
        content.data = dto.data;
        return content;
    }

    static SCE::Content toEntity(const ContentDto &dto)
    {
        SCE::Content content;
        content.id = dto.id;
        content.createdAt = dto.createdAt;
        content.updatedAt = dto.updatedAt;
        content.role = dto.role;
        content.data = dto.data;
        return content;
    }

    static ContentDto toDto(const SCE::Content &entity)
    {
        return ContentDto{entity.id, entity.createdAt, entity.updatedAt, entity.role, entity.data};
    }

    static QList<SCE::Content> toEntityList(const QList<CreateContentDto> &dtos)
    {
        QList<SCE::Content> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::Content> toEntityList(const QList<ContentDto> &dtos)
    {
        QList<SCE::Content> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<ContentDto> toDtoList(const QList<SCE::Content> &entities)
    {
        QList<ContentDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }
};
} // namespace Skribisto::DirectAccess::Content
