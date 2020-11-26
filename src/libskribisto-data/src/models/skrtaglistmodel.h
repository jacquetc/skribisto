/***************************************************************************
 *   Copyright (C) 2020 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: skrtaglistmodel.h                                                   *
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
#ifndef SKRTAGLISTMODEL_H
#define SKRTAGLISTMODEL_H

#include <QObject>
#include "plmdata.h"
#include "skrtagitem.h"
#include "./skribisto_data_global.h"

class EXPORT SKRTagListModel : public QAbstractListModel
{
    Q_OBJECT
public:
    explicit SKRTagListModel(QObject *parent = nullptr);
    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;

    // Basic functionality:
    QModelIndex index(int                row,
                      int                column,
                      const QModelIndex& parent = QModelIndex()) const override;


    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex &index, const QVariant &value,
                 int role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    // Add data:
    bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;

    // Remove data:
    bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;

    QHash<int, QByteArray> roleNames() const override;

    QModelIndexList getModelIndex(int projectId, int tagId);
    SKRTagItem *getItem(int projectId, int tagId);
    Q_INVOKABLE QString getTagName(int projectId, int tagId);

private slots:
    void populate();
    void clear();
    void exploitSignalFromPLMData(int                 projectId,
                                  int                 paperId,
                                  SKRTagItem::Roles role);
    void refreshAfterDataAddition(int                 projectId,
                                  int                 paperId);



private:

    void          connectToPLMDataSignals();
    void          disconnectFromPLMDataSignals();

private:

    SKRTagItem *m_rootItem;
    QVariant m_headerData;
    QList<SKRTagItem *>m_allTagItems;
    QList<QMetaObject::Connection>m_dataConnectionsList;

};

#endif // SKRTAGLISTMODEL_H
