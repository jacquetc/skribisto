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

#include "binder_tag/dtos.h"
#include "direct_access/binder_tag/i_binder_tag_repository.h"
#include "entities/binder_tag.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::BinderTag
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDBinderTag = Skribisto::Common::DirectAccess::BinderTag;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::BinderTag toEntity(const CreateBinderTagDto &dto)
    {
        SCE::BinderTag binderTag;
        binderTag.id = 0;
        binderTag.createdAt = dto.createdAt;
        binderTag.updatedAt = dto.updatedAt;
        binderTag.name = dto.name;
        binderTag.color = dto.color;
        binderTag.textColor = dto.textColor;
        return binderTag;
    }

    static SCE::BinderTag toEntity(const BinderTagDto &dto)
    {
        SCE::BinderTag binderTag;
        binderTag.id = dto.id;
        binderTag.createdAt = dto.createdAt;
        binderTag.updatedAt = dto.updatedAt;
        binderTag.name = dto.name;
        binderTag.color = dto.color;
        binderTag.textColor = dto.textColor;
        return binderTag;
    }

    static BinderTagDto toDto(const SCE::BinderTag &entity)
    {
        return BinderTagDto{entity.id,
                            entity.createdAt,
                            entity.updatedAt,
                            entity.name,
                            entity.color,
                            entity.textColor};
    }

    static QList<SCE::BinderTag> toEntityList(const QList<CreateBinderTagDto> &dtos)
    {
        QList<SCE::BinderTag> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::BinderTag> toEntityList(const QList<BinderTagDto> &dtos)
    {
        QList<SCE::BinderTag> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<BinderTagDto> toDtoList(const QList<SCE::BinderTag> &entities)
    {
        QList<BinderTagDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }
};
} // namespace Skribisto::DirectAccess::BinderTag
