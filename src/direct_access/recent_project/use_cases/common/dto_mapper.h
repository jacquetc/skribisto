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

#include "direct_access/recent_project/i_recent_project_repository.h"
#include "entities/recent_project.h"
#include "recent_project/dtos.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::RecentProject
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDRecentProject = Skribisto::Common::DirectAccess::RecentProject;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::RecentProject toEntity(const CreateRecentProjectDto &dto)
    {
        SCE::RecentProject recentProject;
        recentProject.id = 0;
        recentProject.createdAt = dto.createdAt;
        recentProject.updatedAt = dto.updatedAt;
        recentProject.title = dto.title;
        recentProject.lastOpenedAt = dto.lastOpenedAt;
        recentProject.absolutePath = dto.absolutePath;
        return recentProject;
    }

    static SCE::RecentProject toEntity(const RecentProjectDto &dto)
    {
        SCE::RecentProject recentProject;
        recentProject.id = dto.id;
        recentProject.createdAt = dto.createdAt;
        recentProject.updatedAt = dto.updatedAt;
        recentProject.title = dto.title;
        recentProject.lastOpenedAt = dto.lastOpenedAt;
        recentProject.absolutePath = dto.absolutePath;
        return recentProject;
    }

    static RecentProjectDto toDto(const SCE::RecentProject &entity)
    {
        return RecentProjectDto{entity.id, entity.createdAt, entity.updatedAt,
                                entity.title, entity.lastOpenedAt, entity.absolutePath};
    }

    static QList<SCE::RecentProject> toEntityList(const QList<CreateRecentProjectDto> &dtos)
    {
        QList<SCE::RecentProject> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::RecentProject> toEntityList(const QList<RecentProjectDto> &dtos)
    {
        QList<SCE::RecentProject> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<RecentProjectDto> toDtoList(const QList<SCE::RecentProject> &entities)
    {
        QList<RecentProjectDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }
};
} // namespace Skribisto::DirectAccess::RecentProject
