/***************************************************************************
 *   Copyright (C) 2019 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmprojectlistmodel.h                                                   *
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
#ifndef SKRRECENTPROJECTLISTMODEL_H
#define SKRRECENTPROJECTLISTMODEL_H

#include <QAbstractListModel>
#include <QDateTime>
#include "./skribisto_data_global.h"



struct EXPORT PLMProjectItem {
    Q_GADGET

public:
    explicit PLMProjectItem(){
        title = "";
        fileName = "";
        lastModification = QDateTime();
        writable = false;
        exists = false;
        isOpened = false;
        projectId = -2;
    }
    QString title;
    QString fileName;
    QDateTime lastModification;
    bool writable;
    bool exists;
    bool isOpened;
    int projectId;

};

class EXPORT SKRRecentProjectListModel : public QAbstractListModel
{
    Q_OBJECT

public:
    explicit SKRRecentProjectListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;
    Q_INVOKABLE void insertInRecentProjects(const QString &title, const QString &fileName);

private slots:
    void insertInRecentProjectsFromAnId(int projectId);
    void populate();

private:
    QList<PLMProjectItem *> m_allRecentProjects;

};

#endif // SKRRECENTPROJECTLISTMODEL_H
