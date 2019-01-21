/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmsheetitem.h                                                   *
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
#ifndef PLMSHEETITEM_H
#define PLMSHEETITEM_H

// #include "plmsheetmodel.h"

#include <QtCore>

class PLMSheetItem {
    Q_GADGET

public:

    enum Roles {
        // papers :
        ProjectNameRole  = Qt::UserRole,
        ProjectIdRole    = Qt::UserRole + 1,
        PaperIdRole      = Qt::UserRole + 2,
        NameRole         = Qt::UserRole + 3,
        TagRole          = Qt::UserRole + 4,
        IndentRole       = Qt::UserRole + 5,
        SortOrderRole    = Qt::UserRole + 6,
        DeletedRole      = Qt::UserRole + 7,
        CreationDateRole = Qt::UserRole + 8,
        UpdateDateRole   = Qt::UserRole + 9,
        ContentDateRole  = Qt::UserRole + 10,

        // specific to sheets:
        CharCountRole = Qt::UserRole + 11,
        WordCountRole = Qt::UserRole + 12
    };
    Q_ENUM(Roles)

    explicit PLMSheetItem();
    explicit PLMSheetItem(int projectId,
                          int paperId,
                          int indent,
                          int sortOrder);
    ~PLMSheetItem();

    void          invalidateData(int role);
    void          invalidateAllData();

    int           projectId();

    int           paperId();
    int           sortOrder();
    int           indent();
    QString       name();

    QVariant      data(int role);
    QList<int>    dataRoles() const;

    PLMSheetItem* parent(const QList<PLMSheetItem *>& itemList);
    int           row(const QList<PLMSheetItem *>& itemList);

    bool          isProjectItem() const;
    void          setIsProjectItem(int projectId);

    int           childrenCount(const QList<PLMSheetItem *>& itemList);
    PLMSheetItem* child(const QList<PLMSheetItem *>& itemList,
                        int                          row);
    bool          isRootItem() const;
    void          setIsRootItem();

signals:

public slots:

private:

    QHash<int, QVariant>m_data;
    QList<int>invalidatedRoles;
    bool m_isProjectItem, m_isRootItem;
};

#endif // PLMSHEETITEM_H
