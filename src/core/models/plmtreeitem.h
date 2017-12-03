/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtreeitem.h                                                   *
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

#ifndef PLMTREEITEM_H
#define PLMTREEITEM_H

#include <QString>
#include <QList>
#include <QHash>
#include <QVariant>
#include <QModelIndex>

class PLMTreeItem
{
public:
    PLMTreeItem(PLMTreeItem *parent = 0);
    ~PLMTreeItem();

    void appendChild(PLMTreeItem *child);
    PLMTreeItem *parent();
    PLMTreeItem *child(int row);
    int childCount() const;
    int columnCount() const;
    QList<PLMTreeItem *> childrenItems();
    int row() const;
    QVariant data(const QString &key) const;
    void setData(const QString &key, const QVariant &value);
    QHash<QString,QVariant>* dataHash();
    void setDataHash(QHash<QString,QVariant> dataHash);

    QModelIndex index() const;
    void setIndex(const QModelIndex &index);

    QString title() const;
    void setTitle(const QString &title);
    int indent() const;
    void setIndent(int indent);
    QVariant content() const;
    void setContent(const QVariant &value);
private:
    QList<PLMTreeItem*> childItems;
    PLMTreeItem *m_parentItem;

    QHash<QString,QVariant> itemDataHash;
    QModelIndex m_index;
};

#endif // PLMTREEITEM_H
