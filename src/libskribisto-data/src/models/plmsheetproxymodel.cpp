/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetproxymodel.cpp
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
#include "plmsheetproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

PLMSheetProxyModel::PLMSheetProxyModel(QObject *parent) : QSortFilterProxyModel(parent)
{
    this->setSourceModel(plmmodels->sheetModel());
    this->setDeletedFilter(false);
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
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMSheetItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool PLMSheetProxyModel::setData(const QModelIndex& index, const QVariant& value,
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

// --------------------------------------------------------------


void PLMSheetProxyModel::setDeletedFilter(bool showDeleted)
{
    m_showDeleted = showDeleted;
    this->invalidate();
}

bool PLMSheetProxyModel::filterAcceptsRow(int                sourceRow,
                                          const QModelIndex& sourceParent) const
{
    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    bool indexDeleted =
        sourceModel()->data(index, PLMSheetItem::Roles::DeletedRole).toBool();

    if (indexDeleted == m_showDeleted) return true;

    return false;
}
