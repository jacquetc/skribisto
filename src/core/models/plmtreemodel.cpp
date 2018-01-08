/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtreemodel.cpp                                                   *
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

#include "plmtreemodel.h"

#include <QHash>
#include <QDateTime>

PLMTreeModel::PLMTreeModel(const QString &tableName, const QString &idName, QObject *parent, int projectId) :
    QAbstractItemModel(parent)
{
    m_projectId = projectId;
    m_rootItem = NULL;
    m_tableName = tableName;
    m_idName = idName;

    m_dataTree = plmdata->database(m_projectId)->getPLMTree(m_tableName);

}

PLMTree *PLMTreeModel::dataTree()
{
    return m_dataTree;
}


PLMTreeModel::~PLMTreeModel()
{
    delete m_rootItem;
}

int PLMTreeModel::rowCount(const QModelIndex &parent) const
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


int PLMTreeModel::columnCount(const QModelIndex &parent) const
{
    if (parent.isValid())
        return static_cast<PLMTreeItem*>(parent.internalPointer())->columnCount();
    else
        return m_rootItem->columnCount();

    //return 2;

}

//--------------------------------------------------------------------------------------------------

QVariant PLMTreeModel::data(const QModelIndex &index, int role) const
{
    int row = index.row();
    int col = index.column();





    if (!index.isValid())
        return QVariant();

    //id :
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 0){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data(m_idName).toInt();
    }
    if (role == Qt::UserRole){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data(m_idName).toInt();
    }
    //title :
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 1){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("t_title").toString();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 2){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_sort_order").toString();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 3){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_indent").toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 4){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("dt_created").toDateTime();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 5){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("m_content").toString();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 6){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("dt_updated").toDateTime();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 7){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("dt_content").toDateTime();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 8){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("b_deleted").toBool();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 9){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_version").toInt();
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

QModelIndex PLMTreeModel::parent(const QModelIndex &index) const
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

QModelIndex PLMTreeModel::index ( int row, int column, const QModelIndex & parent ) const
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



QVariant PLMTreeModel::getContent(int paperId)
{

return findItemById(paperId)->dataHash()->value("m_content");
//    PLMTreeItem *item = findItemById(paperId);
//    QModelIndex index = this->index(item->row() ,5, item->parent()->index());
//    return data(index, Qt::DisplayRole);

}

void PLMTreeModel::setContent(int paperId, const QString &content)
{

    PLMTreeItem *item = findItemById(paperId);
    QModelIndex index = this->index(item->row() ,5, item->parent()->index());
    setData(index, content, Qt::EditRole);
}

int PLMTreeModel::addChildItem(int paperId)
{

    QList<int> idList = m_dataTree->addNewChildPapers(paperId, 1);
    resetModel();
    return idList.first();

}

PLMTreeItem *PLMTreeModel::findItemById(int paperId)
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




Qt::ItemFlags PLMTreeModel::flags(const QModelIndex &index) const
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

bool PLMTreeModel::setData(const QModelIndex &index,
                                const QVariant &value, int role)
{
    QVector<int> vector(1, role);

    PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
    int paperId = data(index, Qt::UserRole).toInt();


//    if(index.data(37).toBool()) //if separator
//        return false;
bool valid = index.isValid();
int col = index.column();
    // content :
    if (index.isValid() && role == Qt::EditRole && index.column() == 5) {

        m_dataTree->setContent(paperId, value);
        item->setContent(value);
        emit dataChanged(index, index, vector);

        return true;
    }


    return false;
}

//useless
bool PLMTreeModel::insertRows(int row, int count, const QModelIndex &parent)
{
    beginInsertRows(parent, row, row + count - 1);
    // FIXME: Implement me!

    endInsertRows();
}

















void PLMTreeModel::resetModel()
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

QList<PLMTreeItem *> PLMTreeModel::itemList() const
{
    return m_itemList;
}

int PLMTreeModel::projectId()
{
    return m_projectId;
}

void PLMTreeModel::populateItem(PLMTreeItem *parentItem)
{
    while(m_allData.count() != 0) {
        int indent =  m_allData.first().value("l_indent").toInt();

        if(parentItem->indent() < indent){

            QHash<QString,QVariant> dataHash = m_allData.takeFirst();
            PLMTreeItem *item = new PLMTreeItem(parentItem);
            item->setDataHash(dataHash);
            populateItem(item);
            m_itemList.append(item);
            parentItem->appendChild(item);
        }
        else
            return;
    }
}
