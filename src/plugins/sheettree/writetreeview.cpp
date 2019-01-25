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

#include <QDebug>

WriteTreeView::WriteTreeView(QWidget *parent) : QTreeView(parent), m_parentDockName("")
{
    this->setEditTriggers(QTreeView::EditKeyPressed);

    this->setHeaderHidden(true);

    this->setModel(plmmodels->sheetProxyModel());
    m_model = plmmodels->sheetProxyModel();

    PLMBaseDock *parentDock = QObjectUtils::findParentOfACertainType<PLMBaseDock>(this);

    if (parentDock) m_parentDockName =  parentDock->objectName();


    connect(this, &QTreeView::clicked, this, &WriteTreeView::itemClicked);

    connect(m_model,
            &PLMSheetModel::modelReset,
            this,
            &WriteTreeView::setExpandStateToItems);
}

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
    if (index != m_oldIndex) { // reset if change
        m_clicksCount = 0;
    }
    m_oldIndex = index;

    m_clicksCount += 1;

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
        PLMCommand command;
        command.origin    = this->objectName();
        command.cmd       = "open_sheet";
        command.projectId = item->projectId();
        command.paperId   = item->paperId();


        emit plmpluginhub->commandSent(command);

        //        int paperId = item->paperId();
        //        int projectId = item->projectId();
        if (item->isProjectItem()) {
            // do nothing for now
        }
        else {
            // emit openSheet(projectId, paperId);
        }
    }
}
