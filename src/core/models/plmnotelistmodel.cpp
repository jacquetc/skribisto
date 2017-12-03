/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmnotelistmodel.cpp                                                   *
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
#include "plmnotelistmodel.h"

#include <QHash>
#include <QDateTime>

PLMNoteListModel::PLMNoteListModel(QObject *parent, int projectId) :
    QAbstractListModel(parent), m_idName("l_note_id"), m_tableName("tbl_note")
  , m_projectId(projectId), m_rootItem(NULL)
{

}



PLMNoteListModel::~PLMNoteListModel()
{
    delete m_rootItem;
}

int PLMNoteListModel::rowCount(const QModelIndex &parent) const
{

    PLMTreeItem *parentItem;
    if (parent.column() > 0)
        return 0;

    if (!parent.isValid())
        parentItem = m_rootItem;
    else
        parentItem = static_cast<PLMTreeItem*>(parent.internalPointer());


    return parentItem->childCount();

    //    return 10;
}


//--------------------------------------------------------------------------------------------------

QVariant PLMNoteListModel::data(const QModelIndex &index, int role) const
{
    int row = index.row();
    int col = index.column();





    if (!index.isValid())
        return QVariant();

    //id :
    if ((role == Qt::DisplayRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data(m_idName).toInt();
    }
    if (role == IdRole){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data(m_idName).toInt();
    }
    //title :
    if ((role == TitleRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("t_title").toString();
    }
    if ((role == SortOrderRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_sort_order").toString();
    }
    if ((role == IndentRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_indent").toInt();
    }
    if ((role == DateCreatedRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("dt_created").toDateTime();
    }
    if ((role == ContentRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("m_content").toString();
    }
    if ((role == DateUpdatedRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("dt_updated").toDateTime();
    }
    if ((role == DateContentRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("dt_content").toDateTime();
    }
    if ((role == DeletedRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("b_deleted").toBool();
    }
    if ((role == VersionRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_version").toInt();
    }
    if ((role == SynopsisRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("b_synopsis").toBool();
    }
    if ((role == SheetCodeRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_sheet_code").toInt();
    }



    //    //value :
    //    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 1){
    //        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
    //        return item->data(col).toString();
    //    }
    //    //id_code :
    //    if (role == Qt::UserRole){
    //        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
    //        return item->code();
    //    }
    //    // PoV :
    //    if (role == 35 && col == 3){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->data(col);
    //    }
    //    // status :
    //    if (role == 38 && col == 4){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->status();
    //    }
    //    //     badge :
    //    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 5){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->badge();
    //    }

    //    if (role == Qt::UserRole){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->idNumber();
    //    }
    //    if (role == 34){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->isExpanded(MainTreeItem::DockedTree);
    //    }
    //    if (role == 39){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->isExpanded(MainTreeItem::Outliner);
    //    }
    //    if (role == 40){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->isExpanded(MainTreeItem::Exporter);
    //    }
    //    if (role == 36){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->type();
    //    }
    //    if (role == 37){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        return item->isTrashed();
    //    }
    //    if (role == Qt::BackgroundRole && col == 0){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        if(item->idNumber() == m_currentSheet)
    //            return QBrush(Qt::lightGray);
    //        else
    //            return QVariant();
    //    }
    //    if (role == Qt::FontRole && col == 0){
    //        MainTreeItem *item = static_cast<MainTreeItem*>(index.internalPointer());
    //        if(item->idNumber() == m_currentSheet){
    //            QFont font;
    //            font.setBold(true);
    //            return font;
    //        }
    //    }
    return QVariant();

}


//--------------------------------------------------------------------------------------------------

QModelIndex PLMNoteListModel::parent(const QModelIndex &index) const
{
    if (!index.isValid())
        return QModelIndex();


    PLMTreeItem *childItem = static_cast<PLMTreeItem*>(index.internalPointer());
    PLMTreeItem *parentItem = childItem->parent();

    if (parentItem == m_rootItem)
        return QModelIndex();


    QModelIndex parentIndex = createIndex(parentItem->row(), 0, parentItem);
    parentItem->setIndex(parentIndex);
    return parentIndex;

    //    return QModelIndex();

}


//--------------------------------------------------------------------------------------------------

QModelIndex PLMNoteListModel::index ( int row, int column, const QModelIndex & parent ) const
{
    if (!hasIndex(row, column, parent))
        return QModelIndex();

    //    if (column == 0){

    PLMTreeItem *parentItem;

    if (!parent.isValid())
        parentItem = m_rootItem;
    else
        parentItem = static_cast<PLMTreeItem*>(parent.internalPointer());

    PLMTreeItem *childItem = parentItem->child(row);
    if (childItem){
        QModelIndex index = createIndex(row, column, childItem);
        childItem->setIndex(index);
        return index;
    }
    else
        return QModelIndex();
    //    }
    //    return QModelIndex();
}

//QModelIndexList PLMTreeModel::match(const QModelIndex &start, int role, const QVariant &value, int hits, Qt::MatchFlags flags) const
//{

//    QModelIndexList list;

//    if(start.column() != 0 && flags.testFlag(Qt::MatchRecursive) && role == Qt::UserRole){

//    }


//    return QAbstractItemModel::match(start, role, value, hits, flags);
//}


PLMTreeItem *PLMNoteListModel::findItemById(int paperId)
{
    foreach (PLMTreeItem *item, m_itemList) {
        if(item->dataHash()->value(m_idName).toInt() == paperId)
            return item;
    }
    return 0;

}


//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
//------------------Editing----------------------------------------------------------
//--------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------




Qt::ItemFlags PLMNoteListModel::flags(const QModelIndex &index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid())
        return defaultFlags;
    if (index.column() == 0)
        return defaultFlags & ~(Qt::ItemIsEditable);


    //    if (index.column() == 0 || index.column() == 1 || index.column() == 2 || index.column() == 3
    //            || index.column() == 4 || index.column() == 5)
    //        return defaultFlags;

//    if(index.data(40).toHash().value("isSeparator", false).toBool() == true)
//        return defaultFlags & ~(Qt::ItemIsEditable|Qt::ItemIsDropEnabled);

    return defaultFlags;
}

//--------------------------------------------------------------------------------------------------

bool PLMNoteListModel::setData(const QModelIndex &index,
                                const QVariant &value, int role)
{
    QVector<int> vector(1, role);

    PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
    int paperId = data(index, Qt::UserRole).toInt();
    PLMTree *dataTree = plmdata->database(m_projectId)->getPLMTree(m_tableName);

//    if(index.data(37).toBool()) //if separator
//        return false;
bool valid = index.isValid();
int col = index.column();
    // content :
    if (index.isValid() && role == Qt::EditRole && index.column() == 0) {

        dataTree->setContent(paperId, value);
        item->setContent(value);
        emit dataChanged(index, index, vector);
        emit layoutChanged();

        return true;
    }


    return false;
}






void PLMNoteListModel::resetModel()
{
    beginResetModel();
    m_allData = plmdata->database(m_projectId)->getPLMTree(m_tableName)->getAll();
    QStringList allHeaders = plmdata->database(m_projectId)->getPLMTree(m_tableName)->getAllHeaders();
    QHash<QString, QVariant> headerHash;
    foreach (const QString &header, allHeaders) {
        headerHash.insert(header, QVariant());
    }
    if(m_rootItem != NULL)
        delete m_rootItem;
    m_rootItem = new PLMTreeItem();
    m_rootItem->setDataHash(headerHash);
    m_rootItem->setIndent(-1);
    m_itemList.clear();

    populateItem(m_rootItem);


    endResetModel();
}

QList<PLMTreeItem *> PLMNoteListModel::itemList() const
{
    return m_itemList;
}

int PLMNoteListModel::projectId()
{
    return m_projectId;
}

void PLMNoteListModel::populateItem(PLMTreeItem *parentItem)
{
    while(m_allData.count() != 0) {
        int indent =  m_allData.first().value("l_indent").toInt();

        if(parentItem->indent() < indent){

            QHash<QString,QVariant> dataHash = m_allData.takeFirst();
            PLMTreeItem *item = new PLMTreeItem(parentItem);
            item->setDataHash(dataHash);
            populateItem(m_rootItem);
            m_itemList.append(item);
            parentItem->appendChild(item);
        }
        else
            return;
    }
}

