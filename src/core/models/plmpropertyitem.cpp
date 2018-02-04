/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmpropertyitem.cpp                                                   *
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

#include "plmpropertyitem.h"

PLMPropertyItem::PLMPropertyItem(PLMPropertyItem *parent)
{
    m_parentItem = parent;

}

PLMPropertyItem::~PLMPropertyItem()
{
    qDeleteAll(childItems);
}

void PLMPropertyItem::appendChild(PLMPropertyItem *item)
{
    childItems.append(item);
}

PLMPropertyItem *PLMPropertyItem::child(int row)
{
    return childItems.value(row);
}

QList<PLMPropertyItem*> PLMPropertyItem::childrenItems()
{
    return childItems;
}
int PLMPropertyItem::childCount() const
{
    return childItems.count();
}



PLMPropertyItem *PLMPropertyItem::parent()
{
    return m_parentItem;
}

QModelIndex PLMPropertyItem::index() const
{
    return m_index;
}

void PLMPropertyItem::setIndex(const QModelIndex &index)
{
    m_index = index;
}

int PLMPropertyItem::row() const
{
    if (m_parentItem)
        return m_parentItem->childItems.indexOf(const_cast<PLMPropertyItem*>(this));

    return 0;
}

int PLMPropertyItem::columnCount() const
{
    return itemDataHash.count();
}
QVariant PLMPropertyItem::data(const QString &key) const
{
    return itemDataHash.value(key);
}

QHash<QString, QVariant> *PLMPropertyItem::dataHash()
{
    return &itemDataHash;

}

void PLMPropertyItem::setDataHash(QHash<QString, QVariant> dataHash)
{
    itemDataHash = dataHash;
}

QString PLMPropertyItem::name() const
{
    return itemDataHash.value("name").toString();
}

void PLMPropertyItem::setName(const QString &name)
{
    itemDataHash.insert("name",name);
}
QString PLMPropertyItem::value() const
{
    return itemDataHash.value("value").toString();
}

void PLMPropertyItem::setValue(const QString &value)
{
    itemDataHash.insert("value",value);

}
int PLMPropertyItem::code() const
{
    return itemDataHash.value("code").toInt();
}

void PLMPropertyItem::setCode(int code)
{
    itemDataHash.insert("code",code);

}



