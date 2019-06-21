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
#include "tools.h"
#include "plmbasedock.h"

#include <QAction>
#include <QDebug>
#include <QMenu>
#include <QContextMenuEvent>

WriteTreeView::WriteTreeView(QWidget *parent) : QTreeView(parent), m_parentDockName(""), m_clickTime(QDateTime::currentSecsSinceEpoch())
{
    this->setupActions();

    this->setEditTriggers(QTreeView::EditKeyPressed);

    this->setHeaderHidden(true);
    this->setTabKeyNavigation(true);

    this->setModel(plmmodels->sheetProxyModel());
    m_model = plmmodels->sheetProxyModel();

    PLMBaseDock *parentDock = QObjectUtils::findParentOfACertainType<PLMBaseDock>(this);

    if (parentDock) m_parentDockName =  parentDock->objectName();


    connect(this, &QTreeView::clicked, this, &WriteTreeView::itemClicked);

    connect(this, &QTreeView::activated, this, &WriteTreeView::itemClicked);

    connect(m_model,
            &PLMSheetModel::modelReset,
            this,
            &WriteTreeView::setExpandStateToItems);
}

//----------------------------------------------------------------------

void WriteTreeView::contextMenuEvent(QContextMenuEvent *event)
{
    QModelIndex index = this->indexAt(event->pos());

    if(!index.isValid())
        return;

    PLMSheetItem *item =
            static_cast<PLMSheetItem *>(m_model->mapToSource(index).
                                        internalPointer());
        m_currentItem = item;
    if (item->isProjectItem()) {
        // do nothing for now
    }
    else {

        m_contextMenu->popup(event->globalPos());

    }


}

//----------------------------------------------------------------------

void WriteTreeView::setExpandStateToItems()
{
    disconnect(this, &WriteTreeView::expanded,  this, &WriteTreeView::itemExpandedSlot);
    disconnect(this, &WriteTreeView::collapsed, this, &WriteTreeView::itemCollapsedSlot);


    for (const QModelIndex& index : this->allIndexesFromModel()) {
        if (!index.isValid()) {
            continue;
        }
        int indexId =
                m_model->data(index, PLMSheetItem::Roles::PaperIdRole).toInt();
        int projectId =
                this->model()->data(index, PLMSheetItem::Roles::ProjectIdRole).toInt();
        QString result = plmdata->sheetPropertyHub()->getProperty(projectId,
                                                                  indexId,
                                                                  "expanded_in_" + m_parentDockName,
                                                                  "1");

        bool expanded;
        result == "1" ? expanded = true : expanded = false;
        this->setExpanded(index, expanded);
    }

    connect(this,
            &WriteTreeView::expanded,
            this,
            &WriteTreeView::itemExpandedSlot,
            Qt::UniqueConnection);
    connect(this,
            &WriteTreeView::collapsed,
            this,
            &WriteTreeView::itemCollapsedSlot,
            Qt::UniqueConnection);
}

QModelIndexList WriteTreeView::allIndexesFromModel()
{
    QModelIndexList indexList;

    QItemSelection selection = this->selectChildren(QModelIndex(), true);

    indexList = selection.indexes();

    return indexList;
}

QItemSelection WriteTreeView::selectChildren(const QModelIndex& parent,
                                             bool               recursively) const
{
    if (!m_model->hasChildren(parent)) {
        return QItemSelection();
    }

    QModelIndex topLeft     = m_model->index(0, 0, parent);
    QModelIndex bottomRight = m_model->index(m_model->rowCount(parent) - 1,
                                             m_model->columnCount(parent) - 1, parent);

    QItemSelection selection(topLeft, bottomRight);

    if (recursively) {
        QItemSelection subselection;

        for (const QModelIndex& index : selection.indexes()) {
            if (m_model->hasChildren(index)) {
                subselection = this->selectChildren(index, true);
            }

            selection.merge(subselection, QItemSelectionModel::Select);
        }
    }
    return selection;
}

void WriteTreeView::itemCollapsedSlot(QModelIndex index)
{
    int indexId =
            this->model()->data(index, PLMSheetItem::Roles::PaperIdRole).toInt();
    int projectId =
            this->model()->data(index, PLMSheetItem::Roles::ProjectIdRole).toInt();
    PLMError error = plmdata->sheetPropertyHub()->setProperty(projectId,
                                                              indexId,
                                                              "expanded_in_" + m_parentDockName,
                                                              "0");

    Q_UNUSED(error)
}

void WriteTreeView::itemExpandedSlot(QModelIndex index)
{
    int indexId =
            this->model()->data(index, PLMSheetItem::Roles::PaperIdRole).toInt();
    int projectId =
            this->model()->data(index, PLMSheetItem::Roles::ProjectIdRole).toInt();
    PLMError error = plmdata->sheetPropertyHub()->setProperty(projectId,
                                                              indexId,
                                                              "expanded_in_" + m_parentDockName,
                                                              "1");

    Q_UNUSED(error)
}

void WriteTreeView::itemClicked(QModelIndex index)
{
    if (index != m_oldIndex || QDateTime::currentSecsSinceEpoch() - m_clickTime > 2) { // reset if change
        m_clicksCount = 0;
    }
    m_oldIndex = index;

    m_clicksCount += 1;
    m_clickTime = QDateTime::currentSecsSinceEpoch();

    if (m_clicksCount == 3) { // third click
        this->edit(index);
        qDebug() << "edit";
    }

    if (m_clicksCount == 2) {      // second click
        // reserved for item expand/collapse
    }
    else if (m_clicksCount == 1) { // first click
        PLMSheetItem *item =
                static_cast<PLMSheetItem *>(m_model->mapToSource(index).
                                            internalPointer());

        if (item->isProjectItem()) {
            // do nothing for now
        }
        else {
            PLMCommand command;

            command.origin    = this->objectName();
            command.cmd       = "open_sheet";
            command.projectId = item->projectId();
            command.paperId   = item->paperId();


            emit plmpluginhub->commandSent(command);
            // emit openSheet(projectId, paperId);
        }
    }
}

//-----------------------------------------------------------------------

void WriteTreeView::setupActions()
{
    m_actionOpenSheet = new QAction(tr("Open sheet"), this);


    connect(m_actionOpenSheet, &QAction::triggered, [=](){
        if(m_currentItem.isNull()){
            return;
        }

        PLMCommand command;

        command.origin    = this->objectName();
        command.cmd       = "open_sheet";
        command.projectId = m_currentItem->projectId();
        command.paperId   = m_currentItem->paperId();


        emit plmpluginhub->commandSent(command);
    });
    //m_actionOpenSheet->setIcon()
    m_actionOpenSheetOnNewSubWindow = new QAction(tr("Open sheet in new view"), this);
    connect(m_actionOpenSheetOnNewSubWindow, &QAction::triggered, [=](){
        if(m_currentItem.isNull()){
            return;
        }

        PLMCommand command;

        command.origin    = this->objectName();
        command.cmd       = "open_sheet_on_new_view";
        command.projectId = m_currentItem->projectId();
        command.paperId   = m_currentItem->paperId();


        emit plmpluginhub->commandSent(command);
    });
    m_actionRename = new QAction(tr("Rename sheet"), this);
    connect(m_actionRename, &QAction::triggered, [=](){
        if(m_currentItem.isNull()){
            return;
        }
        this->edit(this->currentIndex());


    });

    m_actionAddSheet = new QAction(tr("Add Sheet"), this);
    connect(m_actionAddSheet, &QAction::triggered, [=](){
        if(m_currentItem.isNull()){
            return;
        }
        // add :
        PLMError error = plmdata->sheetHub()->addPaperBelow(m_currentItem->projectId(), m_currentItem->paperId());

        IFKO(error){
            qWarning() << "Error : Add Sheet" << m_currentItem->projectId() << m_currentItem->paperId();
            return;

        }

        // edit :
        int lastAddedId = plmdata->sheetHub()->getLastAddedId();

        QModelIndexList indexList = plmmodels->sheetModel()->getModelIndex(m_currentItem->projectId(), lastAddedId);
        if(indexList.isEmpty()){
            qWarning() << "Error : Add Sheet : indexList.isEmpty" << m_currentItem->projectId() << m_currentItem->paperId();
            return;
        }
       QModelIndex index = indexList.first();

        if(!index.isValid()){
            return;
        }
        QModelIndex proxyIndex = m_model->mapFromSource(index);

        this->edit(proxyIndex);

    });

    m_actionAddSubSheet = new QAction(tr("Add Sub-sheet"), this);
    connect(m_actionAddSubSheet, &QAction::triggered, [=](){
        if(m_currentItem.isNull()){
            return;
        }

        // add :

        PLMError error = plmdata->sheetHub()->addChildPaper(m_currentItem->projectId(), m_currentItem->paperId());

        IFKO(error){
            qWarning() << "Error : Add Sub-sheet" << m_currentItem->projectId() << m_currentItem->paperId();
            return;

        }

        // edit :
        int lastAddedId = plmdata->sheetHub()->getLastAddedId();
        QModelIndexList indexList = plmmodels->sheetModel()->getModelIndex(m_currentItem->projectId(), lastAddedId);

        if(indexList.isEmpty()){
            qWarning() << "Error : Add Sheet : indexList.isEmpty" << m_currentItem->projectId() << m_currentItem->paperId();
            return;
        }
        QModelIndex index = indexList.first();

        if(!index.isValid()){
            return;
        }
        QModelIndex proxyIndex = m_model->mapFromSource(index);

        this->edit(proxyIndex);


    });

    m_actionSortAlphabeticaly = new QAction(tr("Alphabeticaly"), this);
    connect(m_actionRename, &QAction::triggered, [=](){
        if(m_currentItem.isNull()){
            return;
        }

        //TODO: implement this action

    });

    m_contextMenu = new QMenu(this);
    m_contextMenu->addAction(m_actionOpenSheet);
    m_contextMenu->addAction(m_actionOpenSheetOnNewSubWindow);
    m_contextMenu->addSeparator();
    m_contextMenu->addAction(m_actionAddSheet);
    m_contextMenu->addAction(m_actionAddSubSheet);
    m_contextMenu->addAction(m_actionRename);
    m_contextMenu->addSeparator();
    QMenu *advancedMenu = new QMenu(tr("Advanced"), this);
    m_contextMenu->addMenu(advancedMenu);
    advancedMenu->addAction(m_actionSortAlphabeticaly);



}


