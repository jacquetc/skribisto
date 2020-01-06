/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetitem.cpp
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
#include "plmsheetitem.h"
#include "plmdata.h"


PLMSheetItem::PLMSheetItem() :
    m_invalidatedRoles(), m_isProjectItem(false), m_isRootItem(false)
{
    m_data.insert(Roles::ProjectIdRole, -2);
    m_data.insert(Roles::PaperIdRole,   -2);
    m_data.insert(Roles::IndentRole,    -2);
    m_data.insert(Roles::SortOrderRole,  0);
}

PLMSheetItem::PLMSheetItem(int projectId, int paperId, int indent,
                           int sortOrder) :
    m_invalidatedRoles(), m_isProjectItem(false), m_isRootItem(false)
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
    m_invalidatedRoles.append(role);
}

void PLMSheetItem::invalidateAllData()
{

    QMetaEnum metaEnum = QMetaEnum::fromType<PLMSheetItem::Roles>();
    for(int i = 0; i < metaEnum.keyCount(); ++i){
        m_invalidatedRoles <<   metaEnum.value(i);
        m_invalidatedRoles.removeAll(PLMSheetItem::Roles::ProjectIdRole);
        m_invalidatedRoles.removeAll(PLMSheetItem::Roles::PaperIdRole);
    }
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

        case Roles::HasChildrenRole:
            m_data.insert(role, plmdata->sheetHub()->hasChildren(projectId, paperId));
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
        m_invalidatedRoles.removeAll(role);
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
    if ((this->indent() == -1) &&
            (plmdata->projectHub()->getProjectIdList().count() > 1)) {
        return nullptr;
    }

    int index                        = itemList.indexOf(this);
    int indent                       = this->indent();
    int possibleParentIndex          = index - 1;

    if(plmdata->projectHub()->getProjectIdList().count() <= 1 && possibleParentIndex == -1){ // first of list, no parent possible
        return nullptr;
    }

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

// create sublist
    QList<PLMSheetItem *> itemSubList;
    for (int i = 0; i < index; ++i) {
        itemSubList.append(itemList.at(i));
    }


    QListIterator<PLMSheetItem *> iterator(itemSubList);
    iterator.toBack();
    while(iterator.hasPrevious()){
        PLMSheetItem *previousItem = iterator.previous();
        if(previousItem->indent() >= indent){

            if(previousItem->indent() == indent){
                possibleRow += 1;
            }
        }
        else {
            break;
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

        // switch between multiple projects or one project
        int parentIndent;
        plmdata->projectHub()->getProjectIdList().count() > 1 ? parentIndent = -2 : parentIndent = -1;



            QListIterator<PLMSheetItem *> iterator(itemList);
            iterator.toFront();
            while(iterator.hasNext()){
                PLMSheetItem *nextItem = iterator.next();
                if(nextItem->indent() > parentIndent){

                    if(nextItem->indent() == parentIndent + 1){
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


    int index              = itemList.indexOf(this);
    int indent             = this->indent();
    int childrenCount      = 0;
    int nextItemIndex      = index + 1;

    if(nextItemIndex >= itemList.count()){
        return 0;
    }

    QList<PLMSheetItem *> itemSubList = itemList.mid(nextItemIndex);

    QListIterator<PLMSheetItem *> iterator(itemSubList);
    iterator.toFront();
    while(iterator.hasNext()){
        PLMSheetItem *nextItem = iterator.next();
        QString thisTitle = this->data(PLMSheetItem::Roles::NameRole).toString();
        QString nextTitle = nextItem->data(PLMSheetItem::Roles::NameRole).toString();
        if(nextItem->indent() > indent){

            if(nextItem->indent() == indent + 1){
                childrenCount += 1;
            }
        }
        else {
            break;
        }
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
        PLMSheetItem *childItem = nullptr;

        // switch between multiple projects or one project


        int parentIndent;
        plmdata->projectHub()->getProjectIdList().count() > 1 ? parentIndent = -2 : parentIndent = -1;



            QListIterator<PLMSheetItem *> iterator(itemList);
            iterator.toFront();
            while(iterator.hasNext()){
                PLMSheetItem *nextItem = iterator.next();
                if(nextItem->indent() > parentIndent){

                    if(nextItem->indent() == parentIndent + 1){
                        childrenCount += 1;
                        if (childrenCount == row + 1){

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

    int index               = itemList.indexOf(this);
    int indent              = this->indent();
    int childrenCount       = 0;
    int nextItemIndex       = index + 1;

    PLMSheetItem *childItem = nullptr;


    QList<PLMSheetItem *> itemSubList = itemList.mid(nextItemIndex);

    QListIterator<PLMSheetItem *> iterator(itemSubList);
    iterator.toFront();
    while(iterator.hasNext()){
        PLMSheetItem *nextItem = iterator.next();
        QString thisTitle = this->data(PLMSheetItem::Roles::NameRole).toString();
        QString nextTitle = nextItem->data(PLMSheetItem::Roles::NameRole).toString();
        if(nextItem->indent() > indent){

            if(nextItem->indent() == indent + 1){
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
    m_invalidatedRoles.clear();

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
