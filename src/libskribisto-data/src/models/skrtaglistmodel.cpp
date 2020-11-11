/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtaglistmodel.cpp
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
#include "skrtaglistmodel.h"

SKRTagListModel::SKRTagListModel(QObject *parent)
    : QAbstractListModel(parent), m_headerData(QVariant())
{
    m_rootItem = new SKRTagItem();
    m_rootItem->setIsRootItem();

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &SKRTagListModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &SKRTagListModel::populate);


    connect(plmdata->tagHub(),
            &SKRTagHub::tagAdded,
            this,
            &SKRTagListModel::refreshAfterDataAddition);


    connect(plmdata->tagHub(),
            &SKRTagHub::tagRemoved,
            this,
            &SKRTagListModel::populate);


    this->connectToPLMDataSignals();
}

QVariant SKRTagListModel::headerData(int section, Qt::Orientation orientation,
                                     int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool SKRTagListModel::setHeaderData(int             section,
                                    Qt::Orientation orientation,
                                    const QVariant& value,
                                    int             role)
{
    if (value != headerData(section, orientation, role)) {
        m_headerData = value;
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}

QModelIndex SKRTagListModel::index(int row, int column, const QModelIndex& parent) const
{
    if (column != 0) return QModelIndex();

    SKRTagItem *parentItem;

    if (!parent.isValid()) parentItem = m_rootItem;

    SKRTagItem *childItem = m_allTagItems.at(row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    }
    return QModelIndex();
}

int SKRTagListModel::rowCount(const QModelIndex& parent) const
{
    if (parent.isValid()) return 0;

    return m_allTagItems.count();
}

QVariant SKRTagListModel::data(const QModelIndex& index, int role) const
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid
                        |  QAbstractItemModel::CheckIndexOption::DoNotUseParent));

    if (!index.isValid()) return QVariant();

    SKRTagItem *item = static_cast<SKRTagItem *>(index.internalPointer());


    if (role == Qt::DisplayRole) {
        return item->data(SKRTagItem::Roles::NameRole);
    }

    if (role == SKRTagItem::Roles::NameRole) {
        return item->data(role);
    }

    if (role == SKRTagItem::Roles::TagIdRole) {
        return item->data(role);
    }

    if (role == SKRTagItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == SKRTagItem::Roles::ColorRole) {
        return item->data(role);
    }

    if (role == SKRTagItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == SKRTagItem::Roles::UpdateDateRole) {
        return item->data(role);
    }


    return QVariant();
}

bool SKRTagListModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (data(index, role) != value) {
        SKRTagItem *item = static_cast<SKRTagItem *>(index.internalPointer());
        int projectId    = item->projectId();
        int tagId        = item->tagId();
        SKRResult result;

        this->disconnectFromPLMDataSignals();

        switch (role) {
        case SKRTagItem::Roles::ProjectIdRole:

            // useless
            break;

        case SKRTagItem::Roles::TagIdRole:

            // useless
            break;

        case SKRTagItem::Roles::NameRole:

            result = plmdata->tagHub()->setTagName(projectId, tagId, value.toString());
            break;

        case SKRTagItem::Roles::ColorRole:

            result = plmdata->tagHub()->setTagColor(projectId, tagId, value.toString());
            break;

        case SKRTagItem::Roles::CreationDateRole:
            result = plmdata->tagHub()->setCreationDate(projectId,
                                                       tagId,
                                                       value.toDateTime());
            break;

        case SKRTagItem::Roles::UpdateDateRole:
            result = plmdata->tagHub()->setUpdateDate(projectId,
                                                     tagId,
                                                     value.toDateTime());
            break;
        }

        this->connectToPLMDataSignals();

        if (!result.isSuccess()) {
            return false;
        }
        item->invalidateData(role);

        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags SKRTagListModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;
}

bool SKRTagListModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
    return false;
}

bool SKRTagListModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
    return false;
}

QHash<int, QByteArray>SKRTagListModel::roleNames() const
{
    QHash<int, QByteArray> roles;

    roles[SKRTagItem::Roles::TagIdRole]        = "tagId";
    roles[SKRTagItem::Roles::ProjectIdRole]    = "projectId";
    roles[SKRTagItem::Roles::NameRole]         = "name";
    roles[SKRTagItem::Roles::ColorRole]        = "color";
    roles[SKRTagItem::Roles::CreationDateRole] = "creationDate";
    roles[SKRTagItem::Roles::UpdateDateRole]   = "updateDate";
    return roles;
}

QModelIndexList SKRTagListModel::getModelIndex(int projectId, int tagId)
{
    QModelIndexList list;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             SKRTagItem::Roles::ProjectIdRole,
                                             projectId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap |
                                             Qt::MatchFlag::MatchRecursive);

    for (const QModelIndex& modelIndex : modelList) {
        int indexTagId = modelIndex.data(SKRTagItem::Roles::TagIdRole).toInt();

        if (indexTagId == tagId) {
            list.append(modelIndex);
        }
    }

    return list;
}

SKRTagItem * SKRTagListModel::getItem(int projectId, int tagId)
{
    SKRTagItem *result_item = nullptr;

    for (SKRTagItem *item : m_allTagItems) {
        if ((item->projectId() == projectId) && (item->tagId() == tagId)) {
            result_item = item;
            break;
        }
    }

    if (!result_item) {
        qDebug() << "result_item is null";
    }

    return result_item;
}

void SKRTagListModel::populate()
{
    this->beginResetModel();

    m_allTagItems.clear();

    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        auto idList = plmdata->tagHub()->getAllTagIds(projectId);

        for (int tagId : idList) {
            m_allTagItems.append(new SKRTagItem(projectId, tagId));
        }
    }
    this->endResetModel();
}

void SKRTagListModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allTagItems);
    this->endResetModel();
}

void SKRTagListModel::exploitSignalFromPLMData(int               projectId,
                                               int               tagId,
                                               SKRTagItem::Roles role)
{
    SKRTagItem *item = this->getItem(projectId, tagId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        SKRTagItem::Roles::TagIdRole,
                                        tagId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : list) {
        SKRTagItem *t = static_cast<SKRTagItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

void SKRTagListModel::refreshAfterDataAddition(int projectId, int tagId)
{
    int row = m_allTagItems.count();

    beginInsertRows(QModelIndex(), row, row);

    m_allTagItems.insert(row, new SKRTagItem(projectId, tagId));
    this->index(row, 0, QModelIndex());
    endInsertRows();
}

void SKRTagListModel::connectToPLMDataSignals()
{
    m_dataConnectionsList << this->connect(plmdata->tagHub(),
                                           &SKRTagHub::nameChanged, this,
                                           [this](int projectId, int tagId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, tagId, SKRTagItem::Roles::NameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->tagHub(),
                                           &SKRTagHub::colorChanged, this,
                                           [this](int projectId, int tagId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, tagId, SKRTagItem::Roles::ColorRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->tagHub(),
                                           &SKRTagHub::updateDateChanged, this,
                                           [this](int projectId, int tagId,
                                                  const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, tagId,
                                       SKRTagItem::Roles::UpdateDateRole);
    }, Qt::UniqueConnection);
}

void SKRTagListModel::disconnectFromPLMDataSignals()
{
    // disconnect from PLMPaperHub signals :
    for (const QMetaObject::Connection& connection : m_dataConnectionsList) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}
