/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsheetproxymodel.cpp
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
#include "plmsheetproxymodel.h"
#include "plmmodels.h"

PLMSheetProxyModel::PLMSheetProxyModel(QObject *parent) : QSortFilterProxyModel(parent)
{
    this->setSourceModel(plmmodels->sheetModel());
}

Qt::ItemFlags PLMSheetProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------

QVariant PLMSheetProxyModel::data(const QModelIndex& index, int role) const
{
    int col = index.column();

    if ((role == Qt::EditRole) && (col == 0)) {
        return QSortFilterProxyModel::data(index,
                                           PLMSheetItem::Roles::NameRole);
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool PLMSheetProxyModel::setData(const QModelIndex& index, const QVariant& value,
                                 int role)
{
    PLMSheetItem *item =
        static_cast<PLMSheetItem *>(this->mapToSource(index).internalPointer());

    if ((role == Qt::EditRole) && (index.column() == 0)) {
        if (item->isProjectItem()) {
            return QSortFilterProxyModel::setData(index,
                                                  value,
                                                  PLMSheetItem::Roles::ProjectNameRole);
        } else {
            return QSortFilterProxyModel::setData(index,
                                                  value,
                                                  PLMSheetItem::Roles::NameRole);
        }
    }
    return QSortFilterProxyModel::setData(index, value, role);
}
