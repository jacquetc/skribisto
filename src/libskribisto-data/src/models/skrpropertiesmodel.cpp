/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmpropertiesmodel.cpp
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
#include "skrpropertiesmodel.h"

SKRPropertiesModel::SKRPropertiesModel(QObject *parent)
    : QAbstractItemModel(parent)
{}

//QVariant SKRPropertiesModel::headerData(int section, Qt::Orientation orientation,
//                                        int role) const
//{
//    // FIXME: Implement me!
//}

//bool SKRPropertiesModel::setHeaderData(int             section,
//                                       Qt::Orientation orientation,
//                                       const QVariant& value,
//                                       int             role)
//{
//    if (value != headerData(section, orientation, role)) {
//        // FIXME: Implement me!
//        emit headerDataChanged(orientation, section, section);
//        return true;
//    }
//    return false;
//}

//QModelIndex SKRPropertiesModel::index(int row, int column,
//                                      const QModelIndex& parent) const
//{
//    // FIXME: Implement me!
//}

//QModelIndex SKRPropertiesModel::parent(const QModelIndex& index) const
//{
//    // FIXME: Implement me!
//}

//int SKRPropertiesModel::rowCount(const QModelIndex& parent) const
//{
//    if (!parent.isValid()) return 0;

//    // FIXME: Implement me!
//}

//int SKRPropertiesModel::columnCount(const QModelIndex& parent) const
//{
//    if (!parent.isValid()) return 0;

//    // FIXME: Implement me!
//}

//QVariant SKRPropertiesModel::data(const QModelIndex& index, int role) const
//{
//    if (!index.isValid()) return QVariant();

//    // FIXME: Implement me!
//    return QVariant();
//}

//bool SKRPropertiesModel::setData(const QModelIndex& index, const QVariant& value,
//                                 int role)
//{
//    if (data(index, role) != value) {
//        // FIXME: Implement me!
//        emit dataChanged(index, index, QVector<int>() << role);
//        return true;
//    }
//    return false;
//}

//Qt::ItemFlags SKRPropertiesModel::flags(const QModelIndex& index) const
//{
//    if (!index.isValid()) return Qt::NoItemFlags;

//    return Qt::ItemIsEditable; // FIXME: Implement me!
//}

//bool SKRPropertiesModel::insertRows(int row, int count, const QModelIndex& parent)
//{
//    beginInsertRows(parent, row, row + count - 1);

//    // FIXME: Implement me!
//    endInsertRows();
//}

//bool SKRPropertiesModel::insertColumns(int column, int count, const QModelIndex& parent)
//{
//    beginInsertColumns(parent, column, column + count - 1);

//    // FIXME: Implement me!
//    endInsertColumns();
//}

//bool SKRPropertiesModel::removeRows(int row, int count, const QModelIndex& parent)
//{
//    beginRemoveRows(parent, row, row + count - 1);

//    // FIXME: Implement me!
//    endRemoveRows();
//}

//bool SKRPropertiesModel::removeColumns(int column, int count, const QModelIndex& parent)
//{
//    beginRemoveColumns(parent, column, column + count - 1);

//    // FIXME: Implement me!
//    endRemoveColumns();
//}
