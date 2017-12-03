/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtreemodel.h                                                   *
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

#ifndef PLMTREEMODEL_H
#define PLMTREEMODEL_H

#include <QAbstractItemModel>
#include <QVariant>

#include "plmtreeitem.h"
#include "../../../src/plmdata.h"

class PLMTreeModel : public QAbstractItemModel
{
    Q_OBJECT
public:
    explicit PLMTreeModel(const QString &tableName, const QString &idName, QObject *parent, int projectId);

    ~PLMTreeModel();
    int rowCount(const QModelIndex &parent) const;
    int columnCount(const QModelIndex &parent) const;
    QVariant data(const QModelIndex &index, int role) const;
    QModelIndex parent(const QModelIndex &index) const;
    QModelIndex index(int row, int column, const QModelIndex &parent) const;
//    QModelIndexList match(const QModelIndex & start, int role,
//                          const QVariant & value, int hits = 1,
//                          Qt::MatchFlags flags = Qt::MatchFlags
//            ( Qt::MatchExactly | Qt::MatchWrap | Qt::MatchRecursive )) const;
    QVariant getContent(int paperId);
    void setContent(int paperId, const QString &content);

    Qt::ItemFlags flags(const QModelIndex &index) const;
    bool setData(const QModelIndex &index, const QVariant &value, int role);
    bool insertRow(int row, const QModelIndex & parent = QModelIndex());
    bool insertRows(int row, int count, const QModelIndex &parent);

    int addChildItem(int paperId);
protected:
    QList<PLMTreeItem *> itemList() const;
    int projectId();
    PLMTreeItem *findItemById(int paperId);
    PLMTree *dataTree();

signals:
public slots:

    void resetModel();

private:
    PLMTreeItem *m_rootItem;
    PLMTree *m_dataTree;
    QString m_tableName, m_idName;
    QList<QHash<QString, QVariant> > m_allData;
    QList<PLMTreeItem *> m_itemList;
    int m_projectId;

    void populateItem(PLMTreeItem *parentItem);
};

#endif // PLMTREEMODEL_H
