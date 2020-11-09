/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrpaperlistmodel.h
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
#ifndef SKRPAPERLISTMODEL_H
#define SKRPAPERLISTMODEL_H

#include <QAbstractListModel>
#include "plmdata.h"
#include "skrpaperitem.h"
#include "skr.h"
#include "./skribisto_data_global.h"


class EXPORT SKRPaperListModel : public QAbstractListModel {
    Q_OBJECT

public:

    explicit SKRPaperListModel(QObject       *parent,
                               SKR::PaperType paperType);

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

    int      rowCount(const QModelIndex& parent = QModelIndex()) const override;

    QVariant data(const QModelIndex& index,
                  int                role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex& index,
                 const QVariant   & value,
                 int                role = Qt::EditRole) override;

    Qt::ItemFlags         flags(const QModelIndex& index) const override;

    QHash<int, QByteArray>roleNames() const override;
    QModelIndexList       getModelIndex(int projectId,
                                        int paperId);
    SKRPaperItem        * getParentPaperItem(SKRPaperItem *childItem);
    SKRPaperItem        * getItem(int projectId,
                                  int paperId);

private slots:

    void populate();
    void clear();
    void exploitSignalFromPLMData(int                 projectId,
                                  int                 paperId,
                                  SKRPaperItem::Roles role);

    void refreshAfterDataAddition(int projectId,
                                  int paperId);
    void refreshAfterDataRemove(int projectId,
                                int paperId);
    void refreshAfterDataMove(int sourceProjectId,
                              int sourcePaperId,
                              int targetProjectId,
                              int targetPaperId);
    void refreshAfterTrashedStateChanged(int  projectId,
                                         int  paperId,
                                         bool newTrashedState);
    void refreshAfterProjectIsBackupChanged(int  projectId,
                                            bool isProjectABackup);
    void refreshAfterProjectIsActiveChanged(int projectId);
    void refreshAfterIndentChanged(int projectId,
                                   int paperId,
                                   int newIndent);

private:

    void connectToPLMDataSignals();
    void disconnectFromPLMDataSignals();

private:

    PLMPaperHub *m_paperHub;
    PLMPropertyHub *m_propertyHub;
    SKR::PaperType m_paperType;
    SKRPaperItem *m_rootItem;
    QVariant m_headerData;
    QList<SKRPaperItem *>m_allPaperItems;
    QList<QMetaObject::Connection>m_dataConnectionsList;
};


#endif // SKRPAPERLISTMODEL_H
