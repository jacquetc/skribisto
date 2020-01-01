/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetlistproxymodel.cpp
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
#include "plmsheetlistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

PLMSheetListProxyModel::PLMSheetListProxyModel(QObject *parent) : QSortFilterProxyModel(parent),
    m_showDeletedFilter(false), m_projectIdFilter(0), m_parentIdFilter(-1)
{
    this->setSourceModel(plmmodels->sheetListModel());
    this->setDeletedFilter(false);
    m_parentIdFilter = 0;
    m_projectIdFilter = 0;
}

Qt::ItemFlags PLMSheetListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------

QVariant PLMSheetListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMSheetItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool PLMSheetListProxyModel::setData(const QModelIndex& index, const QVariant& value,
                                     int role)
{
    QModelIndex sourceIndex = this->mapToSource(index);

    PLMSheetItem *item =
            static_cast<PLMSheetItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMSheetItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMSheetItem::Roles::NameRole);
        }
    }
    return QSortFilterProxyModel::setData(index, value, role);
}

//--------------------------------------------------------------

void PLMSheetListProxyModel::setParentFilter(int projectId, int parentId)
{
    m_projectIdFilter = projectId;
    m_parentIdFilter = parentId;
    this->invalidate();
}
//--------------------------------------------------------------


void PLMSheetListProxyModel::setDeletedFilter(bool showDeleted)
{
    m_showDeletedFilter = showDeleted;
    this->invalidate();
}

//--------------------------------------------------------------


bool PLMSheetListProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const
{
    bool result = false;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);
    if (!index.isValid()){
        return false;
    }
    PLMSheetItem *item = static_cast<PLMSheetItem *>(index.internalPointer());
    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());

    // displays project list :
    if(m_parentIdFilter == -2 && item->isProjectItem() && plmdata->projectHub()->getProjectIdList().count() > 1){
        return true;
    }

    // path / parent filtering :
    int indexProject = item->data(PLMSheetItem::Roles::ProjectIdRole).toInt();
    if (indexProject == m_projectIdFilter){
        return false;
    }
    PLMSheetItem *parentItem = model->getParentSheetItem(item);
    if(parentItem){
        if(parentItem->paperId() == m_parentIdFilter){
            result = true;
        }
    }
    else if(!parentItem && item->indent() == 0 ){
        result = true;
    }
    else {
        return false;
    }
QString string = item->data(PLMSheetItem::Roles::TagRole).toString();
    // deleted filtering :

    if(result && item->data(PLMSheetItem::Roles::DeletedRole).toBool() == m_showDeletedFilter){
        return true;
    }
    return false;
}
