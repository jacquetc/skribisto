/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmspropertymodel.cpp                                                   *
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

#include "plmpropertymodel.h"
#include "../../data/plmdata.h"
#include <QHash>
#include <QDateTime>

PLMPropertyModel::PLMPropertyModel(const QString &tableName, const QString &codeName, QObject *parent, int projectId) :
    QAbstractTableModel(parent)
{
    m_projectId = projectId;
    m_rootItem = NULL;
    m_tableName = tableName;
    m_codeName = codeName;
}



PLMPropertyModel::~PLMPropertyModel()
{
    delete m_rootItem;
}

int PLMPropertyModel::rowCount(const QModelIndex &parent) const
{

    PLMPropertyItem *parentItem;
    if (parent.column() > 0)
        return 0;

    if (!parent.isValid())
        parentItem = m_rootItem;
    else
        parentItem = static_cast<PLMPropertyItem*>(parent.internalPointer());


    return parentItem->childCount();

    //    return 10;
}


int PLMPropertyModel::columnCount(const QModelIndex &parent) const
{
        if (parent.isValid())
            return static_cast<PLMPropertyItem*>(parent.internalPointer())->columnCount();
        else{
            int count = m_rootItem->columnCount();
            return count;
        }
    return 0;
    //return 2;

}

//QVariant PLMPropertyModel::headerData(int section, Qt::Orientation orientation,
//                                           int role) const
//{
//    if (role != Qt::DisplayRole)
//        return QVariant();


//    if (orientation == Qt::Horizontal && role == Qt::DisplayRole && section != -1){

//        return m_rootItem->data(section);
//    }
//    else
//        return QString("error");
//}
//--------------------------------------------------------------------------------------------------

QVariant PLMPropertyModel::data(const QModelIndex &index, int role) const
{
    int row = index.row();
    int col = index.column();





    if (!index.isValid())
        return QVariant();
    //name :
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 0){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data("t_name").toString();
    }
    //value :
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 1){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data("t_value").toString();
    }
    //id_code :
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 2){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data(m_codeName).toInt();
    }
    if (role == Qt::UserRole){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data(m_codeName).toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 4){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data("dt_created").toDateTime();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 5){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data("dt_updated").toDateTime();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 6){
        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
        return item->data("b_system").toBool();
    }

//    if (role == Qt::UserRole){
//        PLMPropertyItem *item = static_cast<PLMPropertyItem*>(index.internalPointer());
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

QModelIndex PLMPropertyModel::parent(const QModelIndex &index) const
{
    if (!index.isValid())
        return QModelIndex();


    PLMPropertyItem *childItem = static_cast<PLMPropertyItem*>(index.internalPointer());
    PLMPropertyItem *parentItem = childItem->parent();

    if (parentItem == m_rootItem)
        return QModelIndex();


    QModelIndex parentIndex = createIndex(parentItem->row(), 0, parentItem);
    parentItem->setIndex(parentIndex);
    return parentIndex;

    //    return QModelIndex();

}


//--------------------------------------------------------------------------------------------------

QModelIndex PLMPropertyModel::index ( int row, int column, const QModelIndex & parent ) const
{
    if (!hasIndex(row, column, parent))
        return QModelIndex();

    //    if (column == 0){

    PLMPropertyItem *parentItem;

    if (!parent.isValid())
        parentItem = m_rootItem;
    else
        parentItem = static_cast<PLMPropertyItem*>(parent.internalPointer());

    PLMPropertyItem *childItem = parentItem->child(row);
    if (childItem){
        QModelIndex index = createIndex(row, column, childItem);
        childItem->setIndex(index);
        return index;
    }else
        return QModelIndex();
    //    }
    //    return QModelIndex();
}



//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
//------------------Editing----------------------------------------------------------
//--------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------


void PLMPropertyModel::resetModel()
{

    beginResetModel();

    QList<QHash<QString, QVariant> > allPropertyData = plmdata->database(m_projectId)->getPLMProperty(m_tableName)->getAll();
    QStringList allPropertyHeaders = plmdata->database(m_projectId)->getPLMProperty(m_tableName)->getAllHeaders();
    QHash<QString, QVariant> headerHash;
    foreach (const QString &header, allPropertyHeaders) {
        headerHash.insert(header, QVariant());
    }
    if(m_rootItem != NULL)
        delete m_rootItem;
    m_rootItem = new PLMPropertyItem();
    m_rootItem->setDataHash(headerHash);
    for(int i = 0; i < allPropertyData.count(); ++i ) {
        QHash<QString,QVariant> dataHash = allPropertyData.at(i);
        PLMPropertyItem *item = new PLMPropertyItem(m_rootItem);
        item->setDataHash(dataHash);
        m_rootItem->appendChild(item);
    }

    endResetModel();

}
