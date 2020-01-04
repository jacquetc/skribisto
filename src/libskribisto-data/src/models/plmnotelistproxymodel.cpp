/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmnotelistproxymodel.cpp
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
#include "plmnotelistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

PLMNoteListProxyModel::PLMNoteListProxyModel(QObject *parent) : QSortFilterProxyModel(parent),
    m_showDeletedFilter(false), m_projectIdFilter(0), m_parentIdFilter(-1)
{
    this->setSourceModel(plmmodels->noteListModel());
    this->setDeletedFilter(false);
    m_parentIdFilter = 0;
    m_projectIdFilter = 0;

    connect(plmdata->projectHub(), &PLMProjectHub::projectLoaded, this,
            [this](int projectId){
        //TODO: replace that with loading project settings
        this->setParentFilter(projectId, 0);
        qDebug() << "setParentFilter";
    });
}

Qt::ItemFlags PLMNoteListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------

QVariant PLMNoteListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMNoteItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool PLMNoteListProxyModel::setData(const QModelIndex& index, const QVariant& value,
                                     int role)
{
    QModelIndex sourceIndex = this->mapToSource(index);

    PLMNoteItem *item =
            static_cast<PLMNoteItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMNoteItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMNoteItem::Roles::NameRole);
        }
    }
    return QSortFilterProxyModel::setData(index, value, role);
}

//--------------------------------------------------------------

void PLMNoteListProxyModel::setParentFilter(int projectId, int parentId)
{
    m_projectIdFilter = projectId;
    m_parentIdFilter = parentId;
    emit parentIdFilterChanged(m_parentIdFilter);
    emit projectIdFilterChanged(m_projectIdFilter);
    this->invalidate();
}
//--------------------------------------------------------------


void PLMNoteListProxyModel::setDeletedFilter(bool showDeleted)
{
    m_showDeletedFilter = showDeleted;
    this->invalidate();
}

//--------------------------------------------------------------


bool PLMNoteListProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const
{
    bool result = false;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);
    if (!index.isValid()){
        return false;
    }
    PLMNoteItem *item = static_cast<PLMNoteItem *>(index.internalPointer());
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    // displays project list :
    if(m_parentIdFilter == -2 && item->isProjectItem() && plmdata->projectHub()->getProjectIdList().count() > 1){
        return true;
    }

    // path / parent filtering :
    int indexProject = item->data(PLMNoteItem::Roles::ProjectIdRole).toInt();
    if (indexProject != m_projectIdFilter){
        return false;
    }
    PLMNoteItem *parentItem = model->getParentNoteItem(item);
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
    // deleted filtering :

    if(result && item->data(PLMNoteItem::Roles::DeletedRole).toBool() == m_showDeletedFilter){
        QString string = item->data(PLMNoteItem::Roles::NameRole).toString();
        //qDebug() << "deleted : " << string;
        return true;
    }
    QString string = item->data(PLMNoteItem::Roles::NameRole).toString();
    //qDebug() << "result : " << string << result;

    return result;
}

void PLMNoteListProxyModel::setParentIdFilter(int parentIdFilter)
{
    m_parentIdFilter = parentIdFilter;
    emit parentIdFilterChanged(m_parentIdFilter);
    this->invalidate();
}

void PLMNoteListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);
    this->invalidate();
}





//--------------------------------------------------------------


void PLMNoteListProxyModel::moveItem(int from, int to) {




    if (from == to)
        return;
    int modelFrom = from;
    int modelTo = to + (from < to ? 1 : 0);


    QModelIndex fromIndex = this->index(modelFrom, 0);
    int fromPaperId = this->data(fromIndex, PLMNoteItem::Roles::PaperIdRole).toInt();
    int fromProjectId = this->data(fromIndex, PLMNoteItem::Roles::ProjectIdRole).toInt();

    QModelIndex toIndex = this->index(modelTo, 0);
    int toPaperId = this->data(toIndex, PLMNoteItem::Roles::PaperIdRole).toInt();
    int toProjectId = this->data(toIndex, PLMNoteItem::Roles::ProjectIdRole).toInt();
    int toSortOrder = this->data(toIndex, PLMNoteItem::Roles::SortOrderRole).toInt();

    qDebug() << "fromPaperId : " << fromPaperId << this->data(fromIndex, PLMNoteItem::Roles::NameRole).toString();
    qDebug() << "toPaperId : " << toPaperId << this->data(toIndex, PLMNoteItem::Roles::NameRole).toString();

    int finalSortOrder = toSortOrder - 1;


    // append to end of list if moved here, paperId = 0

    if(toPaperId == 0){

        PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

        QModelIndex modelIndex = model->getModelIndex(fromProjectId, fromPaperId).first();
        PLMNoteItem *item = static_cast<PLMNoteItem *>(modelIndex.internalPointer());

        int indent = item->indent();
        QList<int> idList = plmdata->noteHub()->getAllIds(fromProjectId);
        QHash<int, int> indentHash = plmdata->noteHub()->getAllIndents(fromProjectId);
        int lastIdWithSameIndent = fromPaperId;

        for(int id : idList){
            if(indentHash.value(id) == indent){
            lastIdWithSameIndent = id;
            }
        }

        if(lastIdWithSameIndent == fromPaperId){
            return;
        }

        finalSortOrder = plmdata->noteHub()->getValidSortOrderAfterPaper(fromProjectId, lastIdWithSameIndent);
    }
        qDebug() << "finalSortOrder" << finalSortOrder;



        //beginMoveRows(QModelIndex(), modelFrom, modelFrom, QModelIndex(), modelTo);
        //PLMError error = plmdata->noteHub()->movePaper(fromProjectId, fromPaperId, toPaperId);
        this->setData(fromIndex, finalSortOrder, PLMNoteItem::Roles::SortOrderRole);
        //plmdata->noteHub()->setSortOrder(fromPaperId, fromPaperId, toSortOrder - 1);

        //endMoveRows();
}

//--------------------------------------------------------------

int PLMNoteListProxyModel::goUp()
{
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
    PLMNoteItem *parentItem = getItem(m_projectIdFilter, m_parentIdFilter);
    if(!parentItem){
        return -2;
    }
    PLMNoteItem *grandParentItem = model->getParentNoteItem(parentItem);

    int grandParentId = -2;
    if(grandParentItem){
        grandParentId = grandParentItem->paperId();
    }
    else{
        if(plmdata->projectHub()->getProjectIdList().count() == 1){
            grandParentId = 0;
        }
        else if(plmdata->projectHub()->getProjectIdList().count() > 1){
            grandParentId = -1;
        }
    }
    this->setParentFilter(m_projectIdFilter, grandParentId);

    return grandParentId;
}

//--------------------------------------------------------------

PLMNoteItem *PLMNoteListProxyModel::getItem(int projectId, int paperId)
{
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
    QModelIndexList modelIndexList = model->getModelIndex(projectId, paperId);
    if(modelIndexList.isEmpty()){
        return nullptr;
    }
    QModelIndex modelIndex = modelIndexList.first();

    PLMNoteItem *item = static_cast<PLMNoteItem *>(modelIndex.internalPointer());
    return item;
}

//--------------------------------------------------------------

QString PLMNoteListProxyModel::getItemName(int projectId, int paperId)
{
    qDebug() << "getItemName" << projectId << paperId;
    if(projectId == -2 || paperId == -2){
        return "";
    }
    QString name = "";
    if(paperId == 0 && plmdata->projectHub()->getProjectIdList().count() <= 1){
        name = plmdata->projectHub()->getProjectName(projectId);
    }
    else{
        PLMNoteItem *item = this->getItem(projectId, paperId);
        if(item){
            name = item->name();
        }
        else {
            name = "";
        }
    }

    return name;
}
