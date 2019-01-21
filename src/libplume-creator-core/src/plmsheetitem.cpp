/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsheetitem.cpp
*                                                  *
*  This file is part of Plume Creator.                                    *
*                                                                         *
*  Plume Creator is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Plume Creator is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#include "plmsheetitem.h"
#include "plmdata.h"

PLMSheetItem::PLMSheetItem() : m_isProjectItem(false), m_isRootItem(false)
{
    m_data.insert(Roles::ProjectIdRole, -2);
    m_data.insert(Roles::PaperIdRole,   -2);
    m_data.insert(Roles::IndentRole,    -2);
    m_data.insert(Roles::SortOrderRole,  0);
}

PLMSheetItem::PLMSheetItem(int projectId, int paperId, int indent,
                           int sortOrder) : m_isProjectItem(false), m_isRootItem(false)
{
    m_data.insert(Roles::ProjectIdRole, projectId);
    m_data.insert(Roles::PaperIdRole,     paperId);
    m_data.insert(Roles::IndentRole,       indent);
    m_data.insert(Roles::SortOrderRole, sortOrder);

    this->invalidateAllData();
}

PLMSheetItem::~PLMSheetItem()
{}


void PLMSheetItem::invalidateData(int role)
{
    invalidatedRoles.append(role);
}

void PLMSheetItem::invalidateAllData()
{
    QHash<QString, QVariant> newData = plmdata->sheetHub()->getSheetData(
        this->projectId(),
        this->paperId());
    QString newProjectName = plmdata->projectHub()->getProjectName(this->projectId());
    m_data.insert(Roles::ProjectNameRole, newProjectName);


    QHash<QString, QVariant>::const_iterator i = newData.constBegin();

    while (i != newData.constEnd()) {
        if (i.key() == "l_sheet_id") {
            // unused
        }

        if (i.key() == "l_dna") {
            // unused
        }


        if (i.key() == "l_sort_order") {
            m_data.insert(Roles::SortOrderRole, i.value().toInt());
        }

        if (i.key() == "l_indent") {
            m_data.insert(Roles::IndentRole, i.value().toInt());
        }

        if (i.key() == "l_version") {
            // unused
        }

        if (i.key() == "t_title") {
            m_data.insert(Roles::NameRole, i.value().toString());
        }

        if (i.key() == "dt_created") {
            m_data.insert(Roles::CreationDateRole, i.value().toDateTime());
        }

        if (i.key() == "dt_updated") {
            m_data.insert(Roles::UpdateDateRole, i.value().toDateTime());
        }

        if (i.key() == "dt_content") {
            m_data.insert(Roles::ContentDateRole, i.value().toDateTime());
        }

        if (i.key() == "b_deleted") {
            m_data.insert(Roles::DeletedRole, i.value().toBool());
        }

        ++i;
    }


    invalidatedRoles.clear();
}

QVariant PLMSheetItem::data(int role)
{
    QMetaEnum metaEnum = QMetaEnum::fromType<PLMSheetItem::Roles>();

    //    if (role != IndentRole)
    //        qDebug() << "item data : " << "pr :" <<
    //        m_data.value(Roles::ProjectIdRole).toInt() <<
    //            "id :" <<
    //            m_data.value(Roles::PaperIdRole).toInt() << "role : " <<
    //            metaEnum.valueToKey(role);

    if (invalidatedRoles.contains(role)) {
        int projectId = this->projectId();
        int paperId   = this->paperId();


        switch (role) {
        case Roles::ProjectNameRole:
            m_data.insert(role, plmdata->projectHub()->getProjectName(projectId));
            break;

        case Roles::ProjectIdRole:

            // useless
            break;

        case Roles::PaperIdRole:

            // useless
            break;

        case Roles::NameRole:
            m_data.insert(role, plmdata->sheetHub()->getTitle(projectId, paperId));
            break;

        case Roles::TagRole:
            m_data.insert(role,
                          plmdata->sheetPropertyHub()->getProperty(projectId, paperId,
                                                                   "tag"));
            break;

        case Roles::IndentRole:
            m_data.insert(role, plmdata->sheetHub()->getIndent(projectId, paperId));
            break;

        case Roles::SortOrderRole:
            m_data.insert(role, plmdata->sheetHub()->getSortOrder(projectId, paperId));
            break;

        case Roles::DeletedRole:
            m_data.insert(role, plmdata->sheetHub()->getDeleted(projectId, paperId));
            break;

        case Roles::CreationDateRole:
            m_data.insert(role, plmdata->sheetHub()->getCreationDate(projectId, paperId));
            break;

        case Roles::UpdateDateRole:
            m_data.insert(role, plmdata->sheetHub()->getUpdateDate(projectId, paperId));
            break;

        case Roles::ContentDateRole:
            m_data.insert(role, plmdata->sheetHub()->getContentDate(projectId, paperId));
            break;

        case Roles::CharCountRole:
            m_data.insert(role,
                          plmdata->sheetPropertyHub()->getProperty(projectId, paperId,
                                                                   "char_count"));
            break;

        case Roles::WordCountRole:
            m_data.insert(role,
                          plmdata->sheetPropertyHub()->getProperty(projectId, paperId,
                                                                   "word_count"));
            break;
        }
        invalidatedRoles.removeAll(role);
    }
    return m_data.value(role);
}

QList<int>PLMSheetItem::dataRoles() const
{
    return m_data.keys();
}

PLMSheetItem * PLMSheetItem::parent(const QList<PLMSheetItem *>& itemList)
{
    if (this->isRootItem()) {
        return nullptr;
    }

    // for project items
    if (this->indent() == -1) {
        return nullptr;
    }


    if ((this->indent() == 0) &&
        (plmdata->projectHub()->getProjectIdList().count() <= 1)) {
        return nullptr;
    }

    int index                        = itemList.indexOf(this);
    int indent                       = this->indent();
    int possibleParentIndex          = index - 1;
    PLMSheetItem *possibleParentItem = itemList.at(possibleParentIndex);

    while (possibleParentItem->indent() >= indent) {
        possibleParentIndex -= 1;
        possibleParentItem   = itemList.at(possibleParentIndex);
    }

    return possibleParentItem;
}

int PLMSheetItem::row(const QList<PLMSheetItem *>& itemList)
{
    if (this->isRootItem()) {
        return 0;
    }

    if (itemList.first() == this) {
        return 0;
    }

    //    if ((this->indent() == 0)) {
    //        return 0;
    //    }

    int index                  = itemList.indexOf(this);
    int indent                 = this->indent();
    int possibleRow            = 0;
    int previousItemIndex      = index - 1;
    PLMSheetItem *previousItem = itemList.at(previousItemIndex);


    while (previousItem->indent() >= indent && itemList.first() != previousItem) {
        previousItemIndex -= 1;
        previousItem       = itemList.at(previousItemIndex);

        if (previousItem->indent() == indent) {
            possibleRow += 1;
        }
    }

    return possibleRow;
}

int PLMSheetItem::childrenCount(const QList<PLMSheetItem *>& itemList) {
    if (this->isRootItem()) {
        if (itemList.isEmpty()) {
            return 0;
        }

        int childrenCount      = 0;
        int nextItemIndex      = 0;
        PLMSheetItem *nextItem = itemList.at(nextItemIndex);

        // switch between multiple projects or one project
        int indent;
        plmdata->projectHub()->getProjectIdList().count() > 1 ? indent = -1 : indent = 0;

        while (nextItem->indent() >= indent && itemList.last() != nextItem) {
            nextItem = itemList.at(nextItemIndex);

            if (nextItem->indent() == indent) {
                childrenCount += 1;
            }
            nextItemIndex += 1;
        }
        return childrenCount;
    }

    if (itemList.last() == this) {
        return 0;
    }


    int index              = itemList.indexOf(this);
    int indent             = this->indent();
    int childrenCount      = 0;
    int nextItemIndex      = index + 1;
    PLMSheetItem *nextItem = itemList.at(nextItemIndex);


    while (nextItem->indent() > indent && itemList.last() != nextItem) {
        nextItem = itemList.at(nextItemIndex);

        if (nextItem->indent() == indent + 1) {
            childrenCount += 1;
        }
        nextItemIndex += 1;
    }

    return childrenCount;
}

PLMSheetItem * PLMSheetItem::child(const QList<PLMSheetItem *>& itemList, int row)
{
    if (this->isRootItem()) {
        if (itemList.isEmpty()) {
            return nullptr;
        }

        int childrenCount       = 0;
        int nextItemIndex       = 0;
        PLMSheetItem *nextItem  = itemList.at(nextItemIndex);
        PLMSheetItem *childItem = nullptr;

        // switch between multiple projects or one project
        int indent;
        plmdata->projectHub()->getProjectIdList().count() > 1 ? indent = -1 : indent = 0;

        while (nextItem->indent() >= indent && itemList.last() != nextItem) {
            nextItem = itemList.at(nextItemIndex);

            if (nextItem->indent() == indent) {
                childrenCount += 1;

                if (childrenCount == row + 1) childItem = nextItem;
            }
            nextItemIndex += 1;
        }
        return childItem;
    }

    if (itemList.isEmpty()) {
        return nullptr;
    }

    int index               = itemList.indexOf(this);
    int indent              = this->indent();
    int childrenCount       = 0;
    int nextItemIndex       = index + 1;
    PLMSheetItem *nextItem  = itemList.at(nextItemIndex);
    PLMSheetItem *childItem = nullptr;

    while (nextItem->indent() > indent && itemList.last() != nextItem) {
        nextItem = itemList.at(nextItemIndex);

        if (nextItem->indent() == indent + 1) {
            childrenCount += 1;

            if (childrenCount == row + 1) childItem = nextItem;
        }
        nextItemIndex += 1;
    }

    return childItem;
}

bool PLMSheetItem::isRootItem() const
{
    return m_isRootItem;
}

void PLMSheetItem::setIsRootItem()
{
    m_isRootItem = true;
}

bool PLMSheetItem::isProjectItem() const
{
    return m_isProjectItem;
}

void PLMSheetItem::setIsProjectItem(int projectId)
{
    m_isProjectItem = true;

    m_data.clear();
    invalidatedRoles.clear();

    m_data.insert(Roles::ProjectIdRole, projectId);
    m_data.insert(Roles::PaperIdRole,          -1);
    m_data.insert(Roles::IndentRole,           -1);
    m_data.insert(Roles::SortOrderRole,     -1000);

    // TODO: add infoHub->getProjectName(projectId)
    // m_data.insert(Roles::NameRole, /*plmdata->infoHub()->*/ );
    this->invalidateData(Roles::ProjectNameRole);
}

int PLMSheetItem::projectId()
{
    return data(Roles::ProjectIdRole).toInt();
}

int PLMSheetItem::paperId()
{
    return data(Roles::PaperIdRole).toInt();
}

int PLMSheetItem::sortOrder()
{
    return data(Roles::SortOrderRole).toInt();
}

int PLMSheetItem::indent()
{
    return data(Roles::IndentRole).toInt();
}

QString PLMSheetItem::name()
{
    return data(Roles::NameRole).toString();
}
