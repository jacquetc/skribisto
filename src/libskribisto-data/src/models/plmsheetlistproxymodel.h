/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetlistproxymodel.h
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
#ifndef PLMSHEETLISTPROXYMODEL_H
#define PLMSHEETLISTPROXYMODEL_H

#include <QSortFilterProxyModel>
#include "./skribisto_data_global.h"

class EXPORT PLMSheetListProxyModel : public QSortFilterProxyModel {
    Q_OBJECT

public:

    explicit PLMSheetListProxyModel(QObject *parent = nullptr);

    Qt::ItemFlags flags(const QModelIndex& index) const;

    QVariant      data(const QModelIndex& index,
                       int                role) const;
    bool          setData(const QModelIndex& index,
                          const QVariant   & value,
                          int                role);

signals:

public slots:

    void setDeletedFilter(bool showDeleted);
    void setParentFilter(int projectId, int parentId);
protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const;
private:
    bool m_showDeletedFilter;
    int m_projectIdFilter;
    int m_parentIdFilter;
};

#endif // PLMSHEETLISTPROXYMODEL_H
