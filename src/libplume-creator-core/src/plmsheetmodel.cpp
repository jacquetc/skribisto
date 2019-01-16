/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsheetmodel.cpp
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
#include "plmsheetmodel.h"
#include "plmdata.h"

#include <QDebug>

PLMSheetModel::PLMSheetModel(QObject *parent)
    : QAbstractItemModel(parent), m_headerData(QVariant())
{
    m_rootItem = new PLMSheetItem();
    m_rootItem->setIsRootItem(true);

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMSheetModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMSheetModel::populate);
}

QVariant PLMSheetModel::headerData(int section, Qt::Orientation orientation,
                                   int role) const
{
    return m_headerData;
}

bool PLMSheetModel::setHeaderData(int             section,
                                  Qt::Orientation orientation,
                                  const QVariant& value,
                                  int             role)
{
    if (value != headerData(section, orientation, role)) {
        m_headerData = value;
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}

QModelIndex PLMSheetModel::index(int row, int column, const QModelIndex& parent) const
{
    if (!hasIndex(row, column, parent)) return QModelIndex();

    // if (column != 0) return QModelIndex();

    PLMSheetItem *parentItem;


    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<PLMSheetItem *>(parent.internalPointer());

    PLMSheetItem *childItem = parentItem->child(m_allSheetItems, row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    } else {
        return QModelIndex();
    }
}

QModelIndex PLMSheetModel::parent(const QModelIndex& index) const
{
    if (!index.isValid()) return QModelIndex();


    PLMSheetItem *childItem  = static_cast<PLMSheetItem *>(index.internalPointer());
    PLMSheetItem *parentItem = childItem->parent(m_allSheetItems);

    if (parentItem == nullptr) {
        return QModelIndex();
    }

    // if (parentItem->isRootItem()) r

    QModelIndex parentIndex =
        createIndex(parentItem->row(m_allSheetItems), 0, parentItem);
    return parentIndex;
}

int PLMSheetModel::rowCount(const QModelIndex& parent) const
{
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) {
        return m_rootItem->childrenCount(m_allSheetItems);
    }


    PLMSheetItem *parentItem = static_cast<PLMSheetItem *>(parent.internalPointer());


    return parentItem->childrenCount(m_allSheetItems);
}

int PLMSheetModel::columnCount(const QModelIndex& parent) const
{
    if (!parent.isValid()) return 1;

    return 1;
}

QVariant PLMSheetModel::data(const QModelIndex& index, int role) const
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (!index.isValid()) return QVariant();

    PLMSheetItem *item = static_cast<PLMSheetItem *>(index.internalPointer());


    // temp: to be removed ?
    if (role == Qt::DisplayRole) {
        return item->data(PLMSheetItem::Roles::NameRole);
    }

    if (role == PLMSheetItem::Roles::NameRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::PaperIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::TagRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ContentDateRole) {
        return item->data(role);
    }
    return QVariant();
}

bool PLMSheetModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    if (data(index, role) != value) {
        // FIXME: Implement me!
        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags PLMSheetModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags; // FIXME: Implement me!
}

bool PLMSheetModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
}

bool PLMSheetModel::insertColumns(int column, int count, const QModelIndex& parent)
{
    beginInsertColumns(parent, column, column + count - 1);

    // FIXME: Implement me!
    endInsertColumns();
}

bool PLMSheetModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
}

bool PLMSheetModel::removeColumns(int column, int count, const QModelIndex& parent)
{
    beginRemoveColumns(parent, column, column + count - 1);

    // FIXME: Implement me!
    endRemoveColumns();
}

QHash<int, QByteArray>PLMSheetModel::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[PLMSheetItem::Roles::PaperIdRole]      = "paperId";
    roles[PLMSheetItem::Roles::ProjectIdRole]    = "projectId";
    roles[PLMSheetItem::Roles::NameRole]         = "name";
    roles[PLMSheetItem::Roles::TagRole]          = "tag";
    roles[PLMSheetItem::Roles::IndentRole]       = "indent";
    roles[PLMSheetItem::Roles::SortOrderRole]    = "sortOrder";
    roles[PLMSheetItem::Roles::CreationDateRole] = "creationDate";
    roles[PLMSheetItem::Roles::UpdateDateRole]   = "updateDate";
    roles[PLMSheetItem::Roles::ContentDateRole]  = "contentDate";
    return roles;
}

void PLMSheetModel::populate()
{
    this->beginResetModel();

    m_allSheetItems.clear();
    foreach(int projectId, plmdata->projectHub()->getProjectIdList()) {
        if (plmdata->projectHub()->getProjectIdList().count() > 1) {
            PLMSheetItem *projectItem = new PLMSheetItem();
            projectItem->setIsProjectItem(projectId, true);
            m_allSheetItems.append(projectItem);
        }

        auto idList         = plmdata->sheetHub()->getAllIds(projectId);
        auto sortOrdersHash = plmdata->sheetHub()->getAllSortOrders(projectId);
        auto indentsHash    = plmdata->sheetHub()->getAllIndents(projectId);

        foreach(int sheetId, idList) {
            m_allSheetItems.append(new PLMSheetItem(projectId, sheetId,
                                                    indentsHash.value(sheetId),
                                                    sortOrdersHash.value(sheetId)));
        }
    }
    this->endResetModel();
}

void PLMSheetModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allSheetItems);
    this->endResetModel();
}

// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
