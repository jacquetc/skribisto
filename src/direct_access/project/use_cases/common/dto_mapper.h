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

#include "direct_access/project/i_project_repository.h"
#include "entities/project.h"
#include "project/dtos.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::Project
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDProject = Skribisto::Common::DirectAccess::Project;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::Project toEntity(const CreateProjectDto &dto)
    {
        SCE::Project project;
        project.id = 0;
        project.createdAt = dto.createdAt;
        project.updatedAt = dto.updatedAt;
        project.title = dto.title;
        project.dictLanguage = dto.dictLanguage;
        project.binders = dto.binders;
        return project;
    }

    static SCE::Project toEntity(const ProjectDto &dto)
    {
        SCE::Project project;
        project.id = dto.id;
        project.createdAt = dto.createdAt;
        project.updatedAt = dto.updatedAt;
        project.title = dto.title;
        project.dictLanguage = dto.dictLanguage;
        project.binders = dto.binders;
        return project;
    }

    static ProjectDto toDto(const SCE::Project &entity)
    {
        return ProjectDto{entity.id, entity.createdAt, entity.updatedAt, entity.title, entity.dictLanguage, entity.binders};
    }

    static QList<SCE::Project> toEntityList(const QList<CreateProjectDto> &dtos)
    {
        QList<SCE::Project> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::Project> toEntityList(const QList<ProjectDto> &dtos)
    {
        QList<SCE::Project> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<ProjectDto> toDtoList(const QList<SCE::Project> &entities)
    {
        QList<ProjectDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }

    static SCDProject::ProjectRelationshipField toCommonRelationshipField(ProjectRelationshipField field)
    {
        switch (field)
        {
        case ProjectRelationshipField::Binders:
            return SCDProject::ProjectRelationshipField::Binders;
        }
        return SCDProject::ProjectRelationshipField::Binders; // fallback
    }
};
} // namespace Skribisto::DirectAccess::Project
