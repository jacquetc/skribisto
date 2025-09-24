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
#include "work/dtos.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::Work
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDWork = Skribisto::Common::DirectAccess::Work;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::Work toEntity(const CreateWorkDto &dto)
    {
        SCE::Work work;
        work.id = 0;
        work.createdAt = dto.createdAt;
        work.updatedAt = dto.updatedAt;
        work.title = dto.title;
        work.dictLanguage = dto.dictLanguage;
        work.binders = dto.binders;
        return work;
    }

    static SCE::Work toEntity(const WorkDto &dto)
    {
        SCE::Work work;
        work.id = dto.id;
        work.createdAt = dto.createdAt;
        work.updatedAt = dto.updatedAt;
        work.title = dto.title;
        work.dictLanguage = dto.dictLanguage;
        work.binders = dto.binders;
        return work;
    }

    static WorkDto toDto(const SCE::Work &entity)
    {
        return WorkDto{entity.id,    entity.createdAt,    entity.updatedAt,
                       entity.title, entity.dictLanguage, entity.binders};
    }

    static QList<SCE::Work> toEntityList(const QList<CreateWorkDto> &dtos)
    {
        QList<SCE::Work> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::Work> toEntityList(const QList<WorkDto> &dtos)
    {
        QList<SCE::Work> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<WorkDto> toDtoList(const QList<SCE::Work> &entities)
    {
        QList<WorkDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }

    static SCDWork::WorkRelationshipField toCommonRelationshipField(WorkRelationshipField field)
    {
        switch (field)
        {
        case WorkRelationshipField::Binders:
            return SCDWork::WorkRelationshipField::Binders;
        }
        return SCDWork::WorkRelationshipField::Binders; // fallback
    }
};
} // namespace Skribisto::DirectAccess::Work
