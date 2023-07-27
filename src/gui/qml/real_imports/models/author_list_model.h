/***************************************************************************
 *   Copyright (C) 2019 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmprojectlistmodel.h
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
#pragma once

#include <QAbstractListModel>
#include <QDateTime>
#include <QQmlEngine>
#include <QUrl>

#include "author/author_dto.h"

struct AuthorListItem
{
    Q_GADGET

  public:
    enum Roles
    {
        // papers :
        NameRole = Qt::DisplayRole,
        CreationDateRole = Qt::UserRole + 1,
        ModificationDateRole = Qt::UserRole + 2
    };
    Q_ENUM(Roles)

    explicit AuthorListItem()
    {
        name = "";
        creationDate = QDateTime();
        modificationDate = QDateTime();
    }

    QString name;
    QDateTime creationDate;
    QDateTime modificationDate;
};

class AuthorListModel : public QAbstractListModel
{
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON

  public:
    explicit AuthorListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;

  private slots:

    void populate();
    void doPopulate(const QList<Contracts::DTO::Author::AuthorDTO> &all_authors_dto);

  private:
    QList<AuthorListItem *> m_items;
};
