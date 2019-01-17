/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: writetreeview.h                                                   *
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
#ifndef WRITETREEVIEW_H
#define WRITETREEVIEW_H

#include <QObject>
#include <QTreeView>

class WriteTreeView : public QTreeView {
public:

    WriteTreeView(QWidget *parent = nullptr);

private slots:

    void              setExpandStateToItems(const QModelIndex & topLeft,
                                            const QModelIndex & bottomRight,
                                            const QVector<int>& roles = QVector<int>());
    QList<QModelIndex>allIndexesFromModel();
};

#endif // WRITETREEVIEW_H
