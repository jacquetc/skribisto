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
#include "skrpaperitem.h"
#include "plmdata.h"

SKRPaperItem::SKRPaperItem(SKR::PaperType paperType) :
    m_paperType(paperType), m_invalidatedRoles(), m_isProjectItem(false), m_isRootItem(
        false)
{
    if (paperType == SKR::Sheet) {
        m_paperHub    = plmdata->sheetHub();
        m_propertyHub = plmdata->sheetPropertyHub();
    }
    else if (paperType == SKR::Note) {
        m_paperHub    = plmdata->noteHub();
        m_propertyHub = plmdata->notePropertyHub();
    }
    else {
        qFatal(this->metaObject()->className(), "Invalid PaperType");
    }


    m_data.insert(Roles::ProjectIdRole, -2);
    m_data.insert(Roles::PaperIdRole,   -2);
    m_data.insert(Roles::IndentRole,    -2);
    m_data.insert(Roles::SortOrderRole, 0);
}

SKRPaperItem::SKRPaperItem(SKR::PaperType paperType,
                           int            projectId,
                           int            paperId,
                           int            indent,
                           int            sortOrder) :

    m_paperType(paperType), m_invalidatedRoles(), m_isProjectItem(false), m_isRootItem(
        false)
{
    if (paperType == SKR::Sheet) {
        m_paperHub    = plmdata->sheetHub();
        m_propertyHub = plmdata->sheetPropertyHub();
    }
    else if (paperType == SKR::Note) {
        m_paperHub    = plmdata->noteHub();
        m_propertyHub = plmdata->notePropertyHub();
    }
    else {
        qFatal(this->metaObject()->className(), "Invalid PaperType");
    }


    m_data.insert(Roles::ProjectIdRole, projectId);
    m_data.insert(Roles::PaperIdRole,   paperId);
    m_data.insert(Roles::IndentRole,    indent);
    m_data.insert(Roles::SortOrderRole, sortOrder);

    this->invalidateAllData();
}

SKRPaperItem::~SKRPaperItem()
{}


void SKRPaperItem::invalidateData(int role)
{
    m_invalidatedRoles.append(role);
}

void SKRPaperItem::invalidateAllData()
{
    QMetaEnum metaEnum = QMetaEnum::fromType<SKRPaperItem::Roles>();

    for (int i = 0; i < metaEnum.keyCount(); ++i) {
        m_invalidatedRoles <<   metaEnum.value(i);
        m_invalidatedRoles.removeAll(SKRPaperItem::Roles::ProjectIdRole);
        m_invalidatedRoles.removeAll(SKRPaperItem::Roles::PaperIdRole);
    }
}

QVariant SKRPaperItem::data(int role)
{
    QMetaEnum metaEnum = QMetaEnum::fromType<SKRPaperItem::Roles>();

    //    if (role != IndentRole)
    //        qDebug() << "item data : " << "pr :" <<
    //        m_data.value(Roles::ProjectIdRole).toInt() <<
    //            "id :" <<
    //            m_data.value(Roles::PaperIdRole).toInt() << "role : " <<
    //            metaEnum.valueToKey(role);

    if (m_invalidatedRoles.contains(role)) {
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
            m_data.insert(role, m_paperHub->getTitle(projectId, paperId));
            break;

        case Roles::LabelRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "label"));
            break;

        case Roles::IndentRole:
            m_data.insert(role, m_paperHub->getIndent(projectId, paperId));
            break;

        case Roles::SortOrderRole:
            m_data.insert(role, m_paperHub->getSortOrder(projectId, paperId));
            break;

        case Roles::TrashedRole:
            m_data.insert(role, m_paperHub->getTrashed(projectId, paperId));
            break;

        case Roles::CreationDateRole:
            m_data.insert(role, m_paperHub->getCreationDate(projectId, paperId));
            break;

        case Roles::UpdateDateRole:
            m_data.insert(role, m_paperHub->getUpdateDate(projectId, paperId));
            break;

        case Roles::ContentDateRole:
            m_data.insert(role, m_paperHub->getContentDate(projectId, paperId));
            break;

        case Roles::CharCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "char_count"));
            break;

        case Roles::WordCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "word_count"));
            break;


        case Roles::ProjectIsBackupRole:
            m_data.insert(role, plmdata->projectHub()->isThisProjectABackup(projectId));
            break;


        case Roles::ProjectIsActiveRole:
            m_data.insert(role, plmdata->projectHub()->isThisProjectActive(projectId));
            break;

        case Roles::IsRenamableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "is_renamable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsMovableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "is_movable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::CanAddPaperRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "can_add_paper",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::IsTrashableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "is_trashable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsOpenableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "is_openable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsCopyableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, paperId,
                                                     "is_copyable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::AttributesRole:
            m_data.insert(role, m_propertyHub->getProperty(projectId, paperId,
                                                           "attributes",
                                                           ""));
            break;
        }

        m_invalidatedRoles.removeAll(role);
    }
    return m_data.value(role);
}

QList<int>SKRPaperItem::dataRoles() const
{
    return m_data.keys();
}

SKRPaperItem * SKRPaperItem::parent(const QList<SKRPaperItem *>& itemList)
{
    if (this->isRootItem()) {
        return nullptr;
    }

    // for project items
    if (this->indent() == -1) {
        return nullptr;
    }


    int index               = itemList.indexOf(this);
    int indent              = this->indent();
    int possibleParentIndex = index - 1;

    if (possibleParentIndex == -1) { // first of list, so no real parent, parent
        // is root item
        return nullptr;
    }

    SKRPaperItem *possibleParentItem;


    for (int i = index - 1; i >= 0; i--) {
        possibleParentItem = itemList.at(i);

        if (possibleParentItem->indent() == indent - 1) {
            break;
        }
    }

    return possibleParentItem;
}

int SKRPaperItem::row(const QList<SKRPaperItem *>& itemList)
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
    QList<SKRPaperItem *> itemSubList;

    for (int i = 0; i < index; ++i) {
        itemSubList.append(itemList.at(i));
    }


    QListIterator<SKRPaperItem *> iterator(itemSubList);

    iterator.toBack();

    while (iterator.hasPrevious()) {
        SKRPaperItem *previousItem = iterator.previous();

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

int SKRPaperItem::childrenCount(const QList<SKRPaperItem *>& itemList) {
    if (this->isRootItem()) {
        if (itemList.isEmpty()) {
            return 0;
        }

        int childrenCount = 0;

        // switch between multiple projects or one project
        int parentIndent = -2;


        QListIterator<SKRPaperItem *> iterator(itemList);
        iterator.toFront();

        while (iterator.hasNext()) {
            SKRPaperItem *nextItem = iterator.next();

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

    QList<SKRPaperItem *> itemSubList = itemList.mid(nextItemIndex);

    QListIterator<SKRPaperItem *> iterator(itemSubList);

    iterator.toFront();

    while (iterator.hasNext()) {
        SKRPaperItem *nextItem = iterator.next();

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

SKRPaperItem * SKRPaperItem::child(const QList<SKRPaperItem *>& itemList, int row)
{
    // if this is root item :
    if (this->isRootItem()) {
        if (itemList.isEmpty()) {
            return nullptr;
        }

        int childrenCount       = 0;
        SKRPaperItem *childItem = nullptr;

        // switch between multiple projects or one project

        int parentIndent = this->indent(); // = -2


        QListIterator<SKRPaperItem *> iterator(itemList);
        iterator.toFront();

        while (iterator.hasNext()) {
            SKRPaperItem *nextItem = iterator.next();

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

    SKRPaperItem *childItem = nullptr;


    QList<SKRPaperItem *> itemSubList = itemList.mid(nextItemIndex);

    QListIterator<SKRPaperItem *> iterator(itemSubList);

    iterator.toFront();

    while (iterator.hasNext()) {
        SKRPaperItem *nextItem = iterator.next();
        QString thisTitle      = this->data(SKRPaperItem::Roles::NameRole).toString();
        QString nextTitle      = nextItem->data(SKRPaperItem::Roles::NameRole).toString();

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

bool SKRPaperItem::isRootItem() const
{
    return m_isRootItem;
}

void SKRPaperItem::setIsRootItem()
{
    m_isRootItem = true;

    m_data.clear();
    m_invalidatedRoles.clear();
    m_data.insert(Roles::PaperIdRole,   -2);
    m_data.insert(Roles::IndentRole,    -2);
    m_data.insert(Roles::SortOrderRole, -90000000);
}

bool SKRPaperItem::isProjectItem() const
{
    return m_isProjectItem;
}

void SKRPaperItem::setIsProjectItem(int projectId)
{
    m_isProjectItem = true;

    m_data.clear();
    m_invalidatedRoles.clear();

    m_data.insert(Roles::ProjectIdRole, projectId);
    m_data.insert(Roles::PaperIdRole,   -1);
    m_data.insert(Roles::IndentRole,    -1);
    m_data.insert(Roles::SortOrderRole, -1000);

    // TODO: add infoHub->getProjectName(projectId)
    // m_data.insert(Roles::NameRole, /*plmdata->infoHub()->*/ );
    this->invalidateData(Roles::ProjectNameRole);
    this->invalidateData(Roles::HasChildrenRole);
    this->invalidateData(Roles::ProjectIsBackupRole);
    this->invalidateData(Roles::ProjectIsActiveRole);
}

int SKRPaperItem::projectId()
{
    return data(Roles::ProjectIdRole).toInt();
}

int SKRPaperItem::paperId()
{
    return data(Roles::PaperIdRole).toInt();
}

int SKRPaperItem::sortOrder()
{
    return data(Roles::SortOrderRole).toInt();
}

int SKRPaperItem::indent()
{
    return data(Roles::IndentRole).toInt();
}

QString SKRPaperItem::name()
{
    return data(Roles::NameRole).toString();
}
