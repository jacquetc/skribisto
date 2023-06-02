/***************************************************************************
 *   Copyright (C) 2019 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: authorlistmodel.cpp
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
#include "author_list_model.h"
#include "author/author_controller.h"
#include "system/system_controller.h"

using namespace Presenter::System;
using namespace Presenter::Author;

AuthorListModel::AuthorListModel(QObject *parent) : QAbstractListModel(parent)
{

    connect(System::SystemController::instance(), &System::SystemController::systemLoaded, this,
            [this]() { populate(); });

    connect(System::SystemController::instance(), &System::SystemController::systemClosed, this,
            [this]() { populate(); });
}

QVariant AuthorListModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return QVariant();
}

int AuthorListModel::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid())
        return 0;

    return m_items.count();
}

QVariant AuthorListModel::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(checkIndex(index, QAbstractItemModel::CheckIndexOption::IndexIsValid |
                                   QAbstractItemModel::CheckIndexOption::DoNotUseParent));

    if (!index.isValid())
        return QVariant();

    if (role == Qt::DisplayRole)
    {
        return m_items.at(index.row())->name;
    }

    if (role == AuthorListItem::Roles::CreationDateRole)
    {
        return m_items.at(index.row())->creationDate;
    }
    if (role == AuthorListItem::Roles::ModificationDateRole)
    {
        return m_items.at(index.row())->modificationDate;
    }
    return QVariant();
}

// ----------------------------------------------------------

QHash<int, QByteArray> AuthorListModel::roleNames() const
{
    QHash<int, QByteArray> roles;

    roles[Qt::DisplayRole] = "name";
    roles[AuthorListItem::Roles::CreationDateRole] = "creationDate";
    roles[AuthorListItem::Roles::ModificationDateRole] = "modificationDate";

    return roles;
}

// ----------------------------------------------------------

void AuthorListModel::populate()
{

    connect(Author::AuthorController::instance(), &Author::AuthorController::getAllReplied, this,
            &AuthorListModel::doPopulate, Qt::SingleShotConnection);

    Author::AuthorController::instance()->getAll();
}

void AuthorListModel::doPopulate(const QList<Contracts::DTO::Author::AuthorDTO> &all_authors_dto)
{

    this->beginResetModel();

    m_items.clear();

    for (const Contracts::DTO::Author::AuthorDTO &author_dto : all_authors_dto)
    {

        AuthorListItem *projectItem = new AuthorListItem();
        projectItem->name = author_dto.name();

        // is project opened ?
        m_items.append(projectItem);
    }

    this->endResetModel();
}

// ----------------------------------------------------------
