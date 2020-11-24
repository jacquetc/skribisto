/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtagitem.cpp                                                   *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#include "skrtagitem.h"
#include "plmdata.h"

SKRTagItem::SKRTagItem()
{
    m_data.insert(Roles::ProjectIdRole, -2);
    m_data.insert(Roles::TagIdRole,     -2);
}

SKRTagItem::SKRTagItem(int projectId, int tagId)
{
    m_data.insert(Roles::ProjectIdRole, projectId);
    m_data.insert(Roles::TagIdRole,     tagId);
    this->invalidateAllData();
}

SKRTagItem::~SKRTagItem()
{}

void SKRTagItem::invalidateData(int role)
{
    m_invalidatedRoles.append(role);
}

void SKRTagItem::invalidateAllData()
{
    QMetaEnum metaEnum = QMetaEnum::fromType<SKRTagItem::Roles>();

    for (int i = 0; i < metaEnum.keyCount(); ++i) {
        m_invalidatedRoles <<   metaEnum.value(i);
        m_invalidatedRoles.removeAll(SKRTagItem::Roles::ProjectIdRole);
        m_invalidatedRoles.removeAll(SKRTagItem::Roles::TagIdRole);
    }
}

int SKRTagItem::projectId()
{
    return data(Roles::ProjectIdRole).toInt();
}

int SKRTagItem::tagId()
{
    return data(Roles::TagIdRole).toInt();
}

QString SKRTagItem::name()
{
    return data(Roles::NameRole).toString();
}

QString SKRTagItem::color()
{
    return data(Roles::ColorRole).toString();
}

QString SKRTagItem::textColor()
{
    return data(Roles::TextColorRole).toString();
}

QVariant SKRTagItem::data(int role)
{
    QMetaEnum metaEnum = QMetaEnum::fromType<SKRTagItem::Roles>();

    if (m_invalidatedRoles.contains(role)) {
        int projectId = this->projectId();
        int tagId     = this->tagId();


        switch (role) {
        case Roles::ProjectIdRole:

            // useless
            break;

        case Roles::TagIdRole:

            // useless
            break;

        case Roles::NameRole:
            m_data.insert(role, plmdata->tagHub()->getTagName(projectId, tagId));
            break;


        case Roles::ColorRole:
            m_data.insert(role, plmdata->tagHub()->getTagColor(projectId, tagId));
            break;

        case Roles::TextColorRole:
            m_data.insert(role, plmdata->tagHub()->getTagTextColor(projectId, tagId));
            break;

        case Roles::CreationDateRole:
            m_data.insert(role, plmdata->tagHub()->getCreationDate(projectId, tagId));
            break;

        case Roles::UpdateDateRole:
            m_data.insert(role, plmdata->tagHub()->getUpdateDate(projectId, tagId));
            break;
        }
        m_invalidatedRoles.removeAll(role);
    }
    return m_data.value(role);
}

QList<int>SKRTagItem::dataRoles() const
{
    return m_data.keys();
}

bool SKRTagItem::isRootItem() const
{
    return m_isRootItem;
}

void SKRTagItem::setIsRootItem()
{
    m_isRootItem = true;

    m_data.clear();
    m_invalidatedRoles.clear();
    m_data.insert(Roles::TagIdRole, -2);
}
