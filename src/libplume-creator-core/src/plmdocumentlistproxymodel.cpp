/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmdocumentslistproxymodel.cpp
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
#include "plmdocumentlistproxymodel.h"
#include "plmdocumentlistmodel.h"
#include <QDebug>


PLMDocumentListProxyModel::PLMDocumentListProxyModel(QObject *parent) : QSortFilterProxyModel(parent)
{
    m_subWindowId = -1;

}

void PLMDocumentListProxyModel::setSubWindowId(int subWindowId)
{
    m_subWindowId = subWindowId;
    this->invalidate();
}


//--------------------------------------------------------------


bool PLMDocumentListProxyModel::filterAcceptsRow(int source_row, const QModelIndex &source_parent) const
{
    QModelIndex index = this->sourceModel()->index(source_row, 0, source_parent);
    qDebug() << "source_row :" << source_row;
    if (!index.isValid()){
        return false;
    }
    int indexSubWindowId = sourceModel()->data(index, PLMDocumentListModel::Roles::SubWindowRole).toInt();

    qDebug() << indexSubWindowId << m_subWindowId;

    if(indexSubWindowId == m_subWindowId)
        return true;

    return false;
}

//--------------------------------------------------------------

QVariant PLMDocumentListProxyModel::data(const QModelIndex &index, int role) const
{
    //   if (!this->mapToSource(index).isValid()) return QVariant();

    //    QModelIndex sourceIndex = this->mapToSource(index);
    //    int col                 = index.column();

    //    if ((role == Qt::DisplayRole) && (col == 0)) {
    //        return this->sourceModel()->data(sourceIndex, PLMDocumentListModel::Roles::NameRole);
    //    }

    return QSortFilterProxyModel::data(index, role);

}

//--------------------------------------------------------------

Qt::ItemFlags PLMDocumentListProxyModel::flags(const QModelIndex& index) const
{
    if (!index.isValid()) return Qt::NoItemFlags;


    return Qt::ItemFlag::ItemIsEnabled | Qt::ItemFlag::ItemIsSelectable;
}

