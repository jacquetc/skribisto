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

#include "direct_access/recent_work/i_recent_work_repository.h"
#include "entities/recent_work.h"
#include "recent_work/dtos.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::RecentWork
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDRecentWork = Skribisto::Common::DirectAccess::RecentWork;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::RecentWork toEntity(const CreateRecentWorkDto &dto)
    {
        SCE::RecentWork recentWork;
        recentWork.id = 0;
        recentWork.createdAt = dto.createdAt;
        recentWork.updatedAt = dto.updatedAt;
        recentWork.title = dto.title;
        recentWork.lastOpenedAt = dto.lastOpenedAt;
        recentWork.absolutePath = dto.absolutePath;
        return recentWork;
    }

    static SCE::RecentWork toEntity(const RecentWorkDto &dto)
    {
        SCE::RecentWork recentWork;
        recentWork.id = dto.id;
        recentWork.createdAt = dto.createdAt;
        recentWork.updatedAt = dto.updatedAt;
        recentWork.title = dto.title;
        recentWork.lastOpenedAt = dto.lastOpenedAt;
        recentWork.absolutePath = dto.absolutePath;
        return recentWork;
    }

    static RecentWorkDto toDto(const SCE::RecentWork &entity)
    {
        return RecentWorkDto{entity.id,    entity.createdAt,    entity.updatedAt,
                             entity.title, entity.lastOpenedAt, entity.absolutePath};
    }

    static QList<SCE::RecentWork> toEntityList(const QList<CreateRecentWorkDto> &dtos)
    {
        QList<SCE::RecentWork> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::RecentWork> toEntityList(const QList<RecentWorkDto> &dtos)
    {
        QList<SCE::RecentWork> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<RecentWorkDto> toDtoList(const QList<SCE::RecentWork> &entities)
    {
        QList<RecentWorkDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }
};
} // namespace Skribisto::DirectAccess::RecentWork
