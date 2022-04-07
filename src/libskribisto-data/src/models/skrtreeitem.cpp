/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrpaperitem.h
*                                                  *
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
#include "skrtreeitem.h"
#include "skrdata.h"

SKRTreeItem::SKRTreeItem() :
    m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();


    m_data.insert(Roles::ProjectIdRole,       -2);
    m_data.insert(Roles::TreeItemIdRole,      -2);
    m_data.insert(Roles::IndentRole,          -2);
    m_data.insert(Roles::SortOrderRole, 99999999);
}

SKRTreeItem::SKRTreeItem(int projectId,
                         int treeItemId,
                         int indent,
                         int sortOrder) :

    m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();

    m_data.insert(Roles::ProjectIdRole,   projectId);
    m_data.insert(Roles::TreeItemIdRole, treeItemId);
    m_data.insert(Roles::IndentRole,         indent);
    m_data.insert(Roles::SortOrderRole,   sortOrder);

    this->invalidateAllData();
}

SKRTreeItem::~SKRTreeItem()
{}


void SKRTreeItem::invalidateData(int role)
{
    m_invalidatedRoles.append(role);
}

void SKRTreeItem::invalidateAllData()
{
    QMetaEnum metaEnum = QMetaEnum::fromType<SKRTreeItem::Roles>();

    for (int i = 0; i < metaEnum.keyCount(); ++i) {
        m_invalidatedRoles <<   metaEnum.value(i);
        m_invalidatedRoles.removeAll(SKRTreeItem::Roles::ProjectIdRole);
        m_invalidatedRoles.removeAll(SKRTreeItem::Roles::TreeItemIdRole);
    }
}

QVariant SKRTreeItem::data(int role)
{
    QMetaEnum metaEnum = QMetaEnum::fromType<SKRTreeItem::Roles>();

    //    if (role != IndentRole)
    //        qDebug() << "item data : " << "pr :" <<
    //        m_data.value(Roles::ProjectIdRole).toInt() <<
    //            "id :" <<
    //            m_data.value(Roles::TreeItemIdRole).toInt() << "role : " <<
    //            metaEnum.valueToKey(role);

    if (m_invalidatedRoles.contains(role)) {
        int projectId  = this->projectId();
        int treeItemId = this->treeItemId();


        switch (role) {
        case Roles::ProjectNameRole:
            m_data.insert(role, skrdata->projectHub()->getProjectName(projectId));
            break;

        case Roles::ProjectIdRole:

            // useless
            break;

        case Roles::TreeItemIdRole:

            // useless
            break;

        case Roles::TitleRole:
            m_data.insert(role, m_treeHub->getTitle(projectId, treeItemId));
            break;

        case Roles::InternalTitleRole:
            m_data.insert(role, m_treeHub->getInternalTitle(projectId, treeItemId));
            break;

        case Roles::TypeRole:
            m_data.insert(role, m_treeHub->getType(projectId, treeItemId));
            break;

        case Roles::LabelRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "label"));
            break;

        case Roles::IndentRole:
            m_data.insert(role, m_treeHub->getIndent(projectId, treeItemId));
            break;

        case Roles::SortOrderRole:
            m_data.insert(role, m_treeHub->getSortOrder(projectId, treeItemId));
            break;

        case Roles::TrashedRole:
            m_data.insert(role, m_treeHub->getTrashed(projectId, treeItemId));
            break;

        case Roles::CreationDateRole:
            m_data.insert(role, m_treeHub->getCreationDate(projectId, treeItemId));
            break;

        case Roles::UpdateDateRole:
            m_data.insert(role, m_treeHub->getUpdateDate(projectId, treeItemId));
            break;

        case Roles::CutCopyRole:
            m_data.insert(role, m_treeHub->isCutCopy(projectId, treeItemId));
            break;

        case Roles::CharCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "char_count"));
            break;

        case Roles::WordCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "word_count"));
            break;

        case Roles::CharCountGoalRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "char_count_goal", "0").toInt());
            break;

        case Roles::WordCountGoalRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "word_count_goal", "0").toInt());
            break;

        case Roles::CharCountWithChildrenRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "char_count_with_children"));
            break;

        case Roles::WordCountWithChildrenRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "word_count_with_children"));
            break;


        case Roles::ProjectIsBackupRole:
            m_data.insert(role, skrdata->projectHub()->isThisProjectABackup(projectId));
            break;


        case Roles::ProjectIsActiveRole:
            m_data.insert(role, skrdata->projectHub()->isThisProjectActive(projectId));
            break;

        case Roles::IsRenamableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_renamable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsMovableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_movable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::CanAddSiblingTreeItemRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "can_add_sibling_tree_item",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::CanAddChildTreeItemRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "can_add_child_tree_item",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::IsTrashableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_trashable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsOpenableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_openable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsCopyableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_copyable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::OtherPropertiesRole:
            m_data.insert(role, otherPropertiesIncrement += 1);
            break;
        }

        m_invalidatedRoles.removeAll(role);
    }
    return m_data.value(role);
}

QList<int>SKRTreeItem::dataRoles() const
{
    return m_data.keys();
}

SKRTreeItem * SKRTreeItem::parent(const QList<SKRTreeItem *>& itemList)
{
    if (this->isRootItem()) {
        return nullptr;
    }

    // for project items
    if (this->indent() == 0) {
        return nullptr;
    }


    int index               = itemList.indexOf(this);
    int indent              = this->indent();
    int possibleParentIndex = index - 1;

    if (possibleParentIndex == -1) { // first of list, so no real parent, parent
        // is root item
        return nullptr;
    }

    SKRTreeItem *possibleParentItem;


    for (int i = index - 1; i >= 0; i--) {
        possibleParentItem = itemList.at(i);

        if (possibleParentItem->indent() == indent - 1) {
            break;
        }
    }

    return possibleParentItem;
}

int SKRTreeItem::row(const QList<SKRTreeItem *>& itemList)
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

    int index       = itemList.indexOf(this);
    int indent      = this->indent();
    int possibleRow = 0;

    // create sublist
    QList<SKRTreeItem *> itemSubList;

    for (int i = 0; i < index; ++i) {
        itemSubList.append(itemList.at(i));
    }


    QListIterator<SKRTreeItem *> iterator(itemSubList);

    iterator.toBack();

    while (iterator.hasPrevious()) {
        SKRTreeItem *previousItem = iterator.previous();

        if (previousItem->indent() >= indent) {
            if (previousItem->indent() == indent) {
                possibleRow += 1;
            }
        }
        else {
            break;
        }
    }


    return possibleRow;
}

int SKRTreeItem::childrenCount(const QList<SKRTreeItem *>& itemList) {
    if (this->isRootItem()) {
        if (itemList.isEmpty()) {
            return 0;
        }

        int childrenCount = 0;

        // switch between multiple projects or one project
        int parentIndent = -2;


        QListIterator<SKRTreeItem *> iterator(itemList);
        iterator.toFront();

        while (iterator.hasNext()) {
            SKRTreeItem *nextItem = iterator.next();

            if (nextItem->indent() > parentIndent) {
                if (nextItem->indent() == parentIndent + 1) {
                    childrenCount += 1;
                }
            }
            else {
                break;
            }
        }


        return childrenCount;
    }

    if (itemList.last() == this) {
        return 0;
    }


    int index         = itemList.indexOf(this);
    int indent        = this->indent();
    int childrenCount = 0;
    int nextItemIndex = index + 1;

    if (nextItemIndex >= itemList.count()) {
        return 0;
    }

    QList<SKRTreeItem *> itemSubList = itemList.mid(nextItemIndex);

    QListIterator<SKRTreeItem *> iterator(itemSubList);

    iterator.toFront();

    while (iterator.hasNext()) {
        SKRTreeItem *nextItem = iterator.next();

        if (nextItem->indent() > indent) {
            if (nextItem->indent() == indent + 1) {
                childrenCount += 1;
            }
        }
        else {
            break;
        }
    }


    return childrenCount;
}

SKRTreeItem * SKRTreeItem::child(const QList<SKRTreeItem *>& itemList, int row)
{
    // if this is root item :
    if (this->isRootItem()) {
        if (itemList.isEmpty()) {
            return nullptr;
        }

        int childrenCount      = 0;
        SKRTreeItem *childItem = nullptr;

        // switch between multiple projects or one project

        int parentIndent = this->indent(); // = -2


        QListIterator<SKRTreeItem *> iterator(itemList);
        iterator.toFront();

        while (iterator.hasNext()) {
            SKRTreeItem *nextItem = iterator.next();

            if (nextItem->indent() > parentIndent) {
                if (nextItem->indent() == parentIndent + 1) {
                    childrenCount += 1;

                    if (childrenCount == row + 1) {
                        // found searched-for child
                        childItem = nextItem;
                        break;
                    }
                }
            }
            else {
                break;
            }
        }


        return childItem;
    }

    if (itemList.isEmpty()) {
        return nullptr;
    }

    int index         = itemList.indexOf(this);
    int indent        = this->indent();
    int childrenCount = 0;
    int nextItemIndex = index + 1;

    SKRTreeItem *childItem = nullptr;


    QList<SKRTreeItem *> itemSubList = itemList.mid(nextItemIndex);

    QListIterator<SKRTreeItem *> iterator(itemSubList);

    iterator.toFront();

    while (iterator.hasNext()) {
        SKRTreeItem *nextItem = iterator.next();
        QString thisTitle     = this->data(SKRTreeItem::Roles::TitleRole).toString();
        QString nextTitle     = nextItem->data(SKRTreeItem::Roles::TitleRole).toString();

        if (nextItem->indent() > indent) {
            if (nextItem->indent() == indent + 1) {
                childrenCount += 1;

                if (childrenCount == row + 1) {
                    childItem = nextItem;
                    break;
                }
            }
        }
        else {
            break;
        }
    }


    return childItem;
}

bool SKRTreeItem::isRootItem() const
{
    return m_isRootItem;
}

void SKRTreeItem::setIsRootItem()
{
    m_isRootItem = true;

    m_data.clear();
    m_invalidatedRoles.clear();
    m_data.insert(Roles::TreeItemIdRole,       -2);
    m_data.insert(Roles::IndentRole,           -2);
    m_data.insert(Roles::SortOrderRole, -90000000);
}

bool SKRTreeItem::isProjectItem()
{
    return this->indent() == 0  && this->treeItemId() == 0 ? true : false;
}

int SKRTreeItem::projectId()
{
    return data(Roles::ProjectIdRole).toInt();
}

int SKRTreeItem::treeItemId()
{
    return data(Roles::TreeItemIdRole).toInt();
}

int SKRTreeItem::sortOrder()
{
    return data(Roles::SortOrderRole).toInt();
}

int SKRTreeItem::indent()
{
    return data(Roles::IndentRole).toInt();
}

QString SKRTreeItem::name()
{
    return data(Roles::TitleRole).toString();
}
