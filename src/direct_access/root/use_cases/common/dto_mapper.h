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
#include "root/dtos.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::Root
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDRoot = Skribisto::Common::DirectAccess::Root;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::Root toEntity(const CreateRootDto &dto)
    {
        return {0, dto.createdAt, dto.updatedAt, dto.authorName, dto.projects, dto.recentProjects};
    }

    static SCE::Root toEntity(const RootDto &dto)
    {
        return {0, dto.createdAt, dto.updatedAt, dto.authorName, dto.projects, dto.recentProjects};
    }

    static RootDto toDto(const SCE::Root &entity)
    {
        return RootDto{entity.id,         entity.createdAt, entity.updatedAt,
                       entity.authorName, entity.projects,  entity.recentProjects};
    }

    static QList<SCE::Root> toEntityList(const QList<CreateRootDto> &dtos)
    {
        QList<SCE::Root> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::Root> toEntityList(const QList<RootDto> &dtos)
    {
        QList<SCE::Root> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<RootDto> toDtoList(const QList<SCE::Root> &entities)
    {
        QList<RootDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }

    static SCDRoot::RootRelationshipField toCommonRelationshipField(RootRelationshipField field)
    {
        switch (field)
        {
        case RootRelationshipField::Projects:
            return SCDRoot::RootRelationshipField::Projects;
        case RootRelationshipField::RecentProjects:
            return SCDRoot::RootRelationshipField::RecentProjects;
        }
        return SCDRoot::RootRelationshipField::Projects; // fallback
    }
};
} // namespace Skribisto::DirectAccess::Root
