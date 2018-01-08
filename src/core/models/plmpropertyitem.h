/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmpropertyitem.h                                                   *
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

#ifndef PLMPROPERTYITEM_H
#define PLMPROPERTYITEM_H


#include <QString>
#include <QList>
#include <QHash>
#include <QVariant>
#include <QModelIndex>

class PLMPropertyItem
{
public:
    PLMPropertyItem(PLMPropertyItem *parent = 0);
    ~PLMPropertyItem();

    void appendChild(PLMPropertyItem *child);
    PLMPropertyItem *parent();
    PLMPropertyItem *child(int row);
    int childCount() const;
    int columnCount() const;
    QList<PLMPropertyItem *> childrenItems();
    int row() const;
    QVariant data(const QString &key) const;
    QHash<QString,QVariant>* dataHash();
    void setDataHash(QHash<QString,QVariant> dataHash);

    QModelIndex index() const;
    void setIndex(const QModelIndex &index);

    QString name() const;
    void setName(const QString &name);

    QString value() const;
    void setValue(const QString &value);

    int code() const;
    void setCode(int code);

private:
    QList<PLMPropertyItem*> childItems;
    PLMPropertyItem *m_parentItem;
    QHash<QString,QVariant> itemDataHash;

    QString m_name, m_value;
    int m_code;
    QModelIndex m_index;

};

#endif // PLMPROPERTYITEM_H
