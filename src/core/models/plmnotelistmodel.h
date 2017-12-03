/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmnotelistmodel.h                                                   *
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
#ifndef PLMNOTELISTMODEL_H
#define PLMNOTELISTMODEL_H

#include <QObject>
#include <QAbstractListModel>
#include "plmtreeitem.h"
#include <QVariant>

#include "../../data/plmdata.h"

class PLMNoteListModel : public QAbstractListModel
{
    Q_OBJECT

public:
    enum Roles {
        IdRole = Qt::UserRole,
        TitleRole = Qt::UserRole + 1,
        ContentRole = Qt::UserRole + 2,
        SortOrderRole = Qt::UserRole + 3,
        IndentRole = Qt::UserRole + 4,
        DateCreatedRole = Qt::UserRole + 5,
        DateUpdatedRole = Qt::UserRole + 6,
        DateContentRole = Qt::UserRole + 7,
        DeletedRole = Qt::UserRole + 8,
        VersionRole = Qt::UserRole + 9,
        SynopsisRole = Qt::UserRole + 10,
        SheetCodeRole = Qt::UserRole + 11
    };

    PLMNoteListModel(QObject *parent, int projectId);
    ~PLMNoteListModel();
    int rowCount(const QModelIndex &parent) const;
    QVariant data(const QModelIndex &index, int role) const;
    QModelIndex parent(const QModelIndex &index) const;
    QModelIndex index(int row, int column, const QModelIndex &parent) const;

    Qt::ItemFlags flags(const QModelIndex &index) const;
    bool setData(const QModelIndex &index, const QVariant &value, int role);
protected:
    PLMTreeItem *findItemById(int paperId);
    QList<PLMTreeItem *> itemList() const;
    int projectId();
signals:

public slots:

    void resetModel();
private:
    PLMTreeItem *m_rootItem;
    QString m_tableName, m_idName;
    QList<QHash<QString, QVariant> > m_allData;
    QList<PLMTreeItem *> m_itemList;
    int m_projectId;

    void populateItem(PLMTreeItem *parentItem);

};

#endif // PLMNOTELISTMODEL_H
