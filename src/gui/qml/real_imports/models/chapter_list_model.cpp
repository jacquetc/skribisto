/***************************************************************************
 *   Copyright (C) 2019 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: chapterlistmodel.cpp
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
#include "chapter_list_model.h"
#include "chapter/chapter_controller.h"
#include "jsdto_mapper.h"
#include "system/system_controller.h"

using namespace Presenter::System;
using namespace Presenter::Chapter;

ChapterListModel::ChapterListModel(QObject *parent) : QAbstractListModel(parent)
{

    QObject::connect(Chapter::ChapterController::instance(), &Chapter::ChapterController::chapterCreated,
                     [this](ChapterDTO chapterDTO) {
                         int id = chapterDTO.id();

                         this->beginInsertRows(QModelIndex(), 0, 0);

                         ChapterListItem *projectItem = new ChapterListItem();
                         projectItem->title = chapterDTO.title();

                         m_items.insert(0, projectItem);

                         this->endInsertRows();
                     });

    connect(System::SystemController::instance(), &System::SystemController::systemLoaded, this,
            [this]() { populate(); });

    connect(System::SystemController::instance(), &System::SystemController::systemClosed, this,
            [this]() { populate(); });
}

QVariant ChapterListModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return QVariant();
}

int ChapterListModel::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid())
        return 0;

    return m_items.count();
}

QVariant ChapterListModel::data(const QModelIndex &index, int role) const
{
    Q_ASSERT(checkIndex(index, QAbstractItemModel::CheckIndexOption::IndexIsValid |
                                   QAbstractItemModel::CheckIndexOption::DoNotUseParent));

    if (!index.isValid())
        return QVariant();

    if (role == Qt::DisplayRole)
    {
        return m_items.at(index.row())->title;
    }

    if (role == ChapterListItem::Roles::CreationDateRole)
    {
        return m_items.at(index.row())->creationDate;
    }
    if (role == ChapterListItem::Roles::ModificationDateRole)
    {
        return m_items.at(index.row())->modificationDate;
    }
    return QVariant();
}

// ----------------------------------------------------------

QHash<int, QByteArray> ChapterListModel::roleNames() const
{
    QHash<int, QByteArray> roles;

    roles[Qt::DisplayRole] = "title";
    roles[ChapterListItem::Roles::CreationDateRole] = "creationDate";
    roles[ChapterListItem::Roles::ModificationDateRole] = "modificationDate";

    return roles;
}

// ----------------------------------------------------------

void ChapterListModel::populate()
{

    connect(Chapter::ChapterController::instance(), &Chapter::ChapterController::getAllReplied, this,
            &ChapterListModel::doPopulate, Qt::SingleShotConnection);

    Chapter::ChapterController::instance()->getAll();
}

void ChapterListModel::doPopulate(const QList<Contracts::DTO::Chapter::ChapterDTO> &all_chapters_dto)
{

    this->beginResetModel();

    m_items.clear();

    for (const Contracts::DTO::Chapter::ChapterDTO &chapter_dto : all_chapters_dto)
    {

        ChapterListItem *projectItem = new ChapterListItem();
        projectItem->title = chapter_dto.title();

        // is project opened ?
        m_items.append(projectItem);
    }

    this->endResetModel();
}
