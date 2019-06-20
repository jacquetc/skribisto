/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: writetreeview.h                                                   *
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
#ifndef WRITETREEVIEW_H
#define WRITETREEVIEW_H

#include <QDateTime>
#include <QObject>
#include <QPointer>
#include <QTreeView>
#include <plmsheetitem.h>
#include "plmsheetproxymodel.h"

class WriteTreeView : public QTreeView {
public:

    WriteTreeView(QWidget *parent = nullptr);

protected:
    void contextMenuEvent(QContextMenuEvent *event);
private slots:

    void              setExpandStateToItems();
    QList<QModelIndex>allIndexesFromModel();
    QItemSelection    selectChildren(const QModelIndex& parent,
                                     bool               recursively) const;
    void              itemCollapsedSlot(QModelIndex index);
    void              itemExpandedSlot(QModelIndex index);
    void              itemClicked(QModelIndex index);


private:
    void setupActions();

    QString m_parentDockName;

    //context menu
    QMenu *m_contextMenu;
    QPointer<PLMSheetItem> m_currentItem;
    QAction *m_actionOpenSheet, *m_actionOpenSheetOnNewSubWindow,  *m_actionAddSheet, *m_actionAddSubSheet
    , *m_actionRename, *m_actionSort, *m_actionSortAlphabeticaly;

    // clicks :
    int m_clicksCount;
    QModelIndex m_oldIndex;
    PLMSheetProxyModel *m_model;
    qint64 m_clickTime;
};

#endif // WRITETREEVIEW_H
