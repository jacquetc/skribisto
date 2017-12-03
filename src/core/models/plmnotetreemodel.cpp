/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmnotetreemodel.cpp                                                   *
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

#include "plmnotetreemodel.h"
#include "tree/plmnotetree.h"

PLMNoteTreeModel::PLMNoteTreeModel(QObject *parent, int projectId):
    PLMTreeModel("tbl_note", "l_note_id", parent, projectId)
{
}
QVariant PLMNoteTreeModel::data(const QModelIndex &index, int role) const
{
    int row = index.row();
    int col = index.column();





    if (!index.isValid())
        return QVariant();

    //geometry  :
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 10){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_xpos").toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 11){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_ypos").toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 12){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_size").toInt();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 13){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("b_synopsis").toBool();
    }
    if ((role == Qt::DisplayRole || role == Qt::EditRole) && col == 14){
        PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
        return item->data("l_sheet_code").toInt();
    }

    return PLMTreeModel::data(index, role);

}
int PLMNoteTreeModel::findSynopsisNoteId(int sheetId)
{
    int noteId = -1;
    foreach (PLMTreeItem *item, itemList()) {
        if(item->dataHash()->value("l_sheet_code").toInt() == sheetId
                && item->dataHash()->value("b_synopsis").toBool())
            noteId = item->dataHash()->value("l_note_id").toInt();
    }
    if(noteId != -1)
        return noteId;
    //if no synopsis :

    return addNewSynopsisToSheet(sheetId);
}
int PLMNoteTreeModel::addNewSynopsisToSheet(int sheetId){
    int noteId = addNewNote();

//    PLMNoteTree *dataNoteTree = plmdata->database(projectId())->getNoteTree();
//    PLMTreeItem *item = findItemById(noteId);
//    QModelIndex index = this->index(item->index().row() ,13, item->parent()->index());
//    int user = index.data(Qt::UserRole).toInt();
//    int row = index.row();
//    int col = index.column();

//    setData(index, true, Qt::EditRole);
//    index = this->index(item->index().row() ,14, item->parent()->index());
//    setData(index, sheetId, Qt::EditRole);


     plmdata->database(0)->getNoteTree()->setSheetCode(noteId, sheetId);
     plmdata->database(0)->getNoteTree()->setIsSynopsis(noteId, true);

    return noteId;
}

int PLMNoteTreeModel::addNewNote()
{
    int noteId = plmdata->database(0)->getNoteTree()->addNewChildNote();
    resetModel();
    return noteId;

}

//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
//------------------Editing----------------------------------------------------------
//--------------------------------------------------------------------------------------------------
//--------------------------------------------------------------------------------------------------




Qt::ItemFlags PLMNoteTreeModel::flags(const QModelIndex &index) const
{
    Qt::ItemFlags defaultFlags = PLMTreeModel::flags(index);

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

bool PLMNoteTreeModel::setData(const QModelIndex &index,
                                const QVariant &value, int role)
{
    QVector<int> vector(1, role);

    PLMTreeItem *item = static_cast<PLMTreeItem*>(index.internalPointer());
    int paperId = index.data(Qt::UserRole).toInt();
    PLMNoteTree *dataNoteTree = plmdata->database(projectId())->getNoteTree();


    // is synopsis :
    if (index.isValid() && role == Qt::EditRole && index.column() == 13) {

        dataNoteTree->setIsSynopsis(paperId, value.toBool());
        item->setData("b_synopsis" ,value.toBool());
        emit dataChanged(index, index, vector);

        return true;
    }

    // sheet_code :
    if (index.isValid() && role == Qt::EditRole && index.column() == 14) {

        dataNoteTree->setSheetCode(paperId, value.toInt());
        item->setData("l_sheet_code", value);
        emit dataChanged(index, index, vector);

        return true;
    }

    return PLMTreeModel::setData(index, value, role);
}




