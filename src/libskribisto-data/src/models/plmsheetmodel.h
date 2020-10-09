/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetmodel.h                                                   *
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
#ifndef PLMSHEETMODEL_H
#define PLMSHEETMODEL_H

#include <QAbstractItemModel>
#include "plmsheetitem.h"
#include "./skribisto_data_global.h"


class EXPORT PLMSheetModel : public QAbstractItemModel {
    Q_OBJECT

public:

    explicit PLMSheetModel(QObject *parent = nullptr);

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
    QModelIndex parent(const QModelIndex& index) const override;

    int         rowCount(const QModelIndex& parent    = QModelIndex()) const override;
    int         columnCount(const QModelIndex& parent = QModelIndex()) const override;

    QVariant    data(const QModelIndex& index,
                     int                role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex& index,
                 const QVariant   & value,
                 int                role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    // Add data:
    bool          insertRows(int                row,
                             int                count,
                             const QModelIndex& parent = QModelIndex()) override;
    bool          insertColumns(int                column,
                                int                count,
                                const QModelIndex& parent = QModelIndex()) override;

    // Remove data:
    bool removeRows(int                row,
                    int                count,
                    const QModelIndex& parent = QModelIndex()) override;
    bool removeColumns(int                column,
                       int                count,
                       const QModelIndex& parent = QModelIndex()) override;

    QHash<int, QByteArray>roleNames() const override;

    QModelIndexList getModelIndex(int projectId, int paperId);
private slots:

    void populate();
    void clear();
    void exploitSignalFromPLMData(int                 projectId,
                                  int                 paperId,
                                  PLMSheetItem::Roles role);
    void addPaper(int                 projectId,
                  int                 paperId);

private:

    PLMSheetItem* findPaperItem(int projectId,
                                int paperId);
    void          connectToPLMDataSignals();
    void          disconnectFromPLMDataSignals();

private:

    PLMSheetItem *m_rootItem;
    QVariant m_headerData;
    QList<PLMSheetItem *>m_allSheetItems;
    QList<QMetaObject::Connection>m_dataConnectionsList;
};


#endif // PLMSHEETMODEL_H
