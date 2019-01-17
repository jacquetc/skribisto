/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: writetreeview.cpp
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
#include "writetreeview.h"
#include "plmdata.h"
#include "plmmodels.h"
#include "plmsheetmodel.h"
#include <QDebug>

WriteTreeView::WriteTreeView(QWidget *parent) : QTreeView(parent)
{
    setModel(plmmodels->sheetModel());
    connect(plmmodels->sheetModel(),
            &PLMSheetModel::dataChanged,
            this,
            &WriteTreeView::setExpandStateToItems);
}

void WriteTreeView::setExpandStateToItems(const QModelIndex & topLeft,
                                          const QModelIndex & bottomRight,
                                          const QVector<int>& roles)
{
    qDebug() << "dataChanged";
}

QList<QModelIndex>WriteTreeView::allIndexesFromModel()
{
    QList<QModelIndex> indexList;
    int columnCount     = plmmodels->sheetModel()->columnCount();
    QModelIndex topLeft = plmmodels->sheetModel()->index(0, 0, QModelIndex());


    QModelIndex currentIndex = QModelIndex();
    int rowCount             = plmmodels->sheetModel()->rowCount(currentIndex);

    while (rowCount > 0) {
        currentIndex = plmmodels->sheetModel()->index(rowCount - 1, 0, currentIndex);
        rowCount     = plmmodels->sheetModel()->rowCount(currentIndex);
    }
    QModelIndex bottomLeft =  currentIndex;


    QModelIndex bottomRight = plmmodels->sheetModel()->sibling(bottomLeft.row(),
                                                               columnCount - 1,
                                                               bottomLeft);

    return indexList;
}
