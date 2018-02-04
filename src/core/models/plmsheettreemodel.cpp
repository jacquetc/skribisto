/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmsheettreemodel.cpp                                                   *
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

#include "plmsheettreemodel.h"

PLMSheetTreeModel::PLMSheetTreeModel(QObject *parent, int projectId):
    PLMTreeModel("tbl_sheet", "l_sheet_id",parent, projectId)
{
}

QVariant PLMSheetTreeModel::data(const QModelIndex &index, int role) const
{
    int row = index.row();
    int col = index.column();





    if (!index.isValid())
        return QVariant();
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 10){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_dna").toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 11){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_word_count").toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 12){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_char_count").toInt();
    }




    return PLMTreeModel::data(index, role);

}
