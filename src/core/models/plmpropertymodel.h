/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmspropertymodel.h                                                   *
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

#ifndef PLMPROPERTYMODEL_H
#define PLMPROPERTYMODEL_H

#include <QAbstractTableModel>
#include <QVariant>

#include "plmpropertyitem.h"


class PLMPropertyModel : public QAbstractTableModel
{
    Q_OBJECT
public:
    explicit PLMPropertyModel(const QString &tableName, const QString &codeName, QObject *parent, int projectId);

    ~PLMPropertyModel();

    int rowCount(const QModelIndex &parent) const;
    int columnCount(const QModelIndex &parent) const;
//    QVariant headerData(int section, Qt::Orientation orientation,
//                        int role = Qt::DisplayRole) const;
    QVariant data(const QModelIndex &index, int role) const;
    QModelIndex parent(const QModelIndex &index) const;
    QModelIndex index(int row, int column, const QModelIndex &parent) const;
protected:

signals:
public slots:

    void resetModel();
private:
    PLMPropertyItem *m_rootItem;
    QString m_tableName, m_codeName;
    int m_projectId;



};

#endif // PLMPROPERTYMODEL_H
