/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtreeitem.cpp                                                   *
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

#include "plmtreeitem.h"

PLMTreeItem::PLMTreeItem(PLMTreeItem *parent)
{
    m_parentItem = parent;

}

PLMTreeItem::~PLMTreeItem()
{
    qDeleteAll(childItems);
}

void PLMTreeItem::appendChild(PLMTreeItem *item)
{
    childItems.append(item);
}

PLMTreeItem *PLMTreeItem::child(int row)
{
    return childItems.value(row);
}

QList<PLMTreeItem*> PLMTreeItem::childrenItems()
{
    return childItems;
}
int PLMTreeItem::childCount() const
{
    return childItems.count();
}

int PLMTreeItem::columnCount() const
{
    return itemDataHash.count();
}
QVariant PLMTreeItem::data(const QString &key) const
{
    return itemDataHash.value(key);
}

QHash<QString, QVariant> *PLMTreeItem::dataHash()
{
    return &itemDataHash;

}

void PLMTreeItem::setDataHash(QHash<QString, QVariant> dataHash)
{
    itemDataHash = dataHash;
}

void PLMTreeItem::setData(const QString &key, const QVariant &value)
{
    itemDataHash.insert(key, value);
}

PLMTreeItem *PLMTreeItem::parent()
{
    return m_parentItem;
}

QModelIndex PLMTreeItem::index() const
{
    return m_index;
}

void PLMTreeItem::setIndex(const QModelIndex &index)
{
    m_index = index;
}

int PLMTreeItem::row() const
{
    if (m_parentItem)
        return m_parentItem->childItems.indexOf(const_cast<PLMTreeItem*>(this));

    return 0;
}
QString PLMTreeItem::title() const
{
    return itemDataHash.value("t_title").toString();
}

void PLMTreeItem::setTitle(const QString &title)
{
    itemDataHash.insert("t_title",title);
}

int PLMTreeItem::indent() const
{
    return itemDataHash.value("l_indent").toInt();
}

void PLMTreeItem::setIndent(int indent)
{
    itemDataHash.insert("l_indent",indent);
}

QVariant PLMTreeItem::content() const
{
    return itemDataHash.value("m_content");
}

void PLMTreeItem::setContent(const QVariant &value)
{
    itemDataHash.insert("m_content", value);
}
