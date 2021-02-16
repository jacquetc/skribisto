/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmdocumentslistproxymodel.h
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
#ifndef PLMDOCUMENTSLISTPROXYMODEL_H
#define PLMDOCUMENTSLISTPROXYMODEL_H

#include <QObject>
#include <QSortFilterProxyModel>
#include "./skribisto_data_global.h"

class EXPORT PLMDocumentListProxyModel : public QSortFilterProxyModel {
    Q_OBJECT

public:

    explicit PLMDocumentListProxyModel(QObject *parent = nullptr);
    void setSubWindowId(int subWindowId);


    Qt::ItemFlags flags(const QModelIndex& index) const override;

    virtual QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

protected:
    bool filterAcceptsRow(int source_row, const QModelIndex &source_parent) const override;

signals:

public slots:

private:
    int m_subWindowId;
};

#endif // PLMDOCUMENTSLISTPROXYMODEL_H
