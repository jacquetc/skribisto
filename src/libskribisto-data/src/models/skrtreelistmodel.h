/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtreeItemlistmodel.h
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
#ifndef SKRPAPERLISTMODEL_H
#define SKRPAPERLISTMODEL_H

#include <QAbstractTableModel>
#include "plmdata.h"
#include "skrtreeitem.h"
#include "skr.h"
#include "./skribisto_data_global.h"


class EXPORT SKRTreeListModel : public QAbstractTableModel {
    Q_OBJECT

public:

    explicit SKRTreeListModel(QObject *parent);

    // Header:
    QVariant headerData(int             section,
                        Qt::Orientation orientation,
                        int             role = Qt::DisplayRole) const override;

    bool setHeaderData(int             section,
                       Qt::Orientation orientation,
                       const QVariant& value,
                       int             role = Qt::EditRole) override;

    // Basic functionality:
    QModelIndex index(int                row,
                      int                column,
                      const QModelIndex& parent = QModelIndex()) const override;

    int      rowCount(const QModelIndex& parent    = QModelIndex()) const override;
    int      columnCount(const QModelIndex& parent = QModelIndex()) const override;

    QVariant data(const QModelIndex& index,
                  int                role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex& index,
                 const QVariant   & value,
                 int                role = Qt::EditRole) override;

    Qt::ItemFlags         flags(const QModelIndex& index) const override;

    QHash<int, QByteArray>roleNames() const override;
    QModelIndexList       getModelIndex(int projectId,
                                        int treeItemId);
    SKRTreeItem         * getParentTreeItem(SKRTreeItem *childItem);
    SKRTreeItem         * getItem(int projectId,
                                  int treeItemId);

    void                  sortAllTreeItemItems();

private slots:

    void populate();
    void clear();
    void exploitSignalFromPLMData(int                projectId,
                                  int                treeItemId,
                                  SKRTreeItem::Roles role);

    void refreshAfterDataAddition(int projectId,
                                  int treeItemId);
    void refreshAfterDataRemove(int projectId,
                                int treeItemId);
    void refreshAfterDataMove(int       sourceProjectId,
                              QList<int>sourceTreeItemIds,
                              int       targetProjectId,
                              int       targetTreeItemId);
    void refreshAfterTrashedStateChanged(int  projectId,
                                         int  treeItemId,
                                         bool newTrashedState);
    void refreshAfterProjectIsBackupChanged(int  projectId,
                                            bool isProjectABackup);
    void refreshAfterProjectIsActiveChanged(int projectId);
    void refreshAfterIndentChanged(int projectId,
                                   int treeItemId,
                                   int newIndent);

signals:

    void sortOtherProxyModelsCalled();

private:

    void connectToPLMDataSignals();
    void disconnectFromPLMDataSignals();
    void resetAllTreeItemsList();

private:

    SKRTreeHub *m_treeHub;
    SKRPropertyHub *m_propertyHub;
    SKRTreeItem *m_rootItem;
    QVariant m_headerData;
    QList<SKRTreeItem *>m_allTreeItems;
    QList<QMetaObject::Connection>m_dataConnectionsList;
};


#endif // SKRPAPERLISTMODEL_H
