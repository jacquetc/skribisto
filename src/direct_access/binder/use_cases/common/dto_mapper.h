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

#include "binder/dtos.h"
#include "direct_access/binder/i_binder_repository.h"
#include "entities/binder.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::Binder
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDBinder = Skribisto::Common::DirectAccess::Binder;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::Binder toEntity(const CreateBinderDto &dto)
    {
        SCE::Binder binder;
        binder.id = 0;
        binder.createdAt = dto.createdAt;
        binder.updatedAt = dto.updatedAt;
        binder.name = dto.name;
        binder.binderItems = dto.binderItems;
        return binder;
    }

    static SCE::Binder toEntity(const BinderDto &dto)
    {
        SCE::Binder binder;
        binder.id = dto.id;
        binder.createdAt = dto.createdAt;
        binder.updatedAt = dto.updatedAt;
        binder.name = dto.name;
        binder.binderItems = dto.binderItems;
        return binder;
    }

    static BinderDto toDto(const SCE::Binder &entity)
    {
        return BinderDto{entity.id, entity.createdAt, entity.updatedAt,
                         entity.name, entity.binderItems};
    }

    static QList<SCE::Binder> toEntityList(const QList<CreateBinderDto> &dtos)
    {
        QList<SCE::Binder> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::Binder> toEntityList(const QList<BinderDto> &dtos)
    {
        QList<SCE::Binder> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<BinderDto> toDtoList(const QList<SCE::Binder> &entities)
    {
        QList<BinderDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }

    static SCDBinder::BinderRelationshipField toCommonRelationshipField(BinderRelationshipField field)
    {
        switch (field)
        {
        case BinderRelationshipField::BinderItems:
            return SCDBinder::BinderRelationshipField::BinderItems;
        }
        return SCDBinder::BinderRelationshipField::BinderItems; // fallback
    }
};
} // namespace Skribisto::DirectAccess::Binder
