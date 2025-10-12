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

#include "binder_item/dtos.h"
#include "direct_access/binder_item/i_binder_item_repository.h"
#include "entities/binder_item.h"

#include <QList>
#include <utility>

namespace Skribisto::DirectAccess::BinderItem
{
namespace SCE = Skribisto::Common::Entities;
namespace SCDBinderItem = Skribisto::Common::DirectAccess::BinderItem;

class DtoMapper
{

  public:
    DtoMapper() = delete;
    ~DtoMapper() = delete;
    DtoMapper(const DtoMapper &) = delete;
    DtoMapper &operator=(const DtoMapper &) = delete;
    DtoMapper(DtoMapper &&) = delete;
    DtoMapper &operator=(DtoMapper &&) = delete;

    static SCE::BinderItem toEntity(const CreateBinderItemDto &dto)
    {
        SCE::BinderItem binderItem;
        binderItem.id = 0;
        binderItem.createdAt = dto.createdAt;
        binderItem.updatedAt = dto.updatedAt;
        binderItem.title = dto.title;
        binderItem.subTitle = dto.subTitle;
        binderItem.role = dto.role;
        binderItem.dictLanguage = dto.dictLanguage;
        binderItem.contents = dto.contents;
        binderItem.binderItems = dto.binderItems;
        binderItem.parentItem = (dto.parentItem == 0) ? std::nullopt : std::make_optional(dto.parentItem);
        return binderItem;
    }

    static SCE::BinderItem toEntity(const BinderItemDto &dto)
    {
        SCE::BinderItem binderItem;
        binderItem.id = dto.id;
        binderItem.createdAt = dto.createdAt;
        binderItem.updatedAt = dto.updatedAt;
        binderItem.title = dto.title;
        binderItem.subTitle = dto.subTitle;
        binderItem.role = dto.role;
        binderItem.dictLanguage = dto.dictLanguage;
        binderItem.contents = dto.contents;
        binderItem.binderItems = dto.binderItems;
        binderItem.parentItem = (dto.parentItem == 0) ? std::nullopt : std::make_optional(dto.parentItem);
        return binderItem;
    }

    static BinderItemDto toDto(const SCE::BinderItem &entity)
    {
        return BinderItemDto{entity.id,           entity.createdAt,
                             entity.updatedAt,    entity.title,
                             entity.subTitle,     entity.role,
                             entity.dictLanguage, entity.contents,
                             entity.binderItems,  entity.parentItem.has_value() ? entity.parentItem.value() : 0};
    }

    static QList<SCE::BinderItem> toEntityList(const QList<CreateBinderItemDto> &dtos)
    {
        QList<SCE::BinderItem> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<SCE::BinderItem> toEntityList(const QList<BinderItemDto> &dtos)
    {
        QList<SCE::BinderItem> entities;
        entities.reserve(dtos.size());
        for (const auto &dto : dtos)
        {
            entities.append(toEntity(dto));
        }
        return entities;
    }

    static QList<BinderItemDto> toDtoList(const QList<SCE::BinderItem> &entities)
    {
        QList<BinderItemDto> dtos;
        dtos.reserve(entities.size());
        for (const auto &entity : entities)
        {
            dtos.append(toDto(entity));
        }
        return dtos;
    }

    static SCDBinderItem::BinderItemRelationshipField toCommonRelationshipField(BinderItemRelationshipField field)
    {
        switch (field)
        {
        case BinderItemRelationshipField::Contents:
            return SCDBinderItem::BinderItemRelationshipField::Contents;
        case BinderItemRelationshipField::BinderItems:
            return SCDBinderItem::BinderItemRelationshipField::BinderItems;
        case BinderItemRelationshipField::ParentItem:
            return SCDBinderItem::BinderItemRelationshipField::ParentItem;
        }
        return SCDBinderItem::BinderItemRelationshipField::Contents; // fallback
    }
};
} // namespace Skribisto::DirectAccess::BinderItem
