/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmdocumentslistmodel.h
*                                                  *
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
#ifndef PLMDOCUMENTSLISTMODEL_H
#define PLMDOCUMENTSLISTMODEL_H

#include <QAbstractListModel>
#include "global_core.h"

struct EXPORT_CORE PLMDocumentListItem
{
    Q_GADGET

public:

    explicit PLMDocumentListItem();
    explicit PLMDocumentListItem(int            projectId,
                                 int            documentId,
                                 const QString& tableName);
    ~PLMDocumentListItem();
    PLMDocumentListItem(const PLMDocumentListItem& item);
    int     projectId;
    int     documentId;
    QString tableName;

signals:

public slots:

private:
};


class PLMDocumentListModel : public QAbstractListModel {
    Q_OBJECT

public:

    enum Roles {
        ProjectIdRole      = Qt::UserRole + 1,
        DocumentIdRole     = Qt::UserRole + 2,
        PaperCodeRole      = Qt::UserRole + 3,
        NameRole           = Qt::UserRole + 4,
        TypeRole           = Qt::UserRole + 5,
        SubWindowRole      = Qt::UserRole + 6,
        CursorPosRole      = Qt::UserRole + 7,
        PropertyRole       = Qt::UserRole + 8,
        UpdateDateRole     = Qt::UserRole + 9,
        LasFocusedDateRole = Qt::UserRole + 10
    };

    explicit PLMDocumentListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int             section,
                        Qt::Orientation orientation,
                        int             role = Qt::DisplayRole) const override;
    bool     setHeaderData(int             section,
                           Qt::Orientation orientation,
                           const QVariant& value,
                           int             role = Qt::EditRole) override;

    // Basic functionality:
    int      rowCount(const QModelIndex& parent = QModelIndex()) const override;

    QVariant data(const QModelIndex& index,
                  int                role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex& index,
                 const QVariant   & value,
                 int                role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;


    // Remove data:
    bool removeRows(int                row,
                    int                count,
                    const QModelIndex& parent = QModelIndex()) override;

    void                  addTableName(const QString& tableName);
    QHash<int, QByteArray>roleNames() const override;
    QList<int>            getSubWindowIdList(int projectId,
                                             int paperId);
    QList<int>            getDocumentId(int projectId,
                                        int paperId,
                                        int subWindowId);

private slots:

    void populate();
    void clear();

    void addDocument(int            projectId,
                     const QString& tableName,
                     int            documentId);

private:

    QVariant m_headerData;
    QStringList m_tableNames;
    QList<PLMDocumentListItem>m_allDocuments;
};

#endif // PLMDOCUMENTSLISTMODEL_H
