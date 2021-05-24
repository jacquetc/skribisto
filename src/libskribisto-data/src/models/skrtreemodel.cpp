/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetmodel.cpp
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
#include "skrtreemodel.h"
#include "skrdata.h"
#include "skrpropertyhub.h"
#include "skr.h"


#include <QDebug>

SKRTreeModel::SKRTreeModel(QObject *parent)
    : QAbstractItemModel(parent), m_headerData(QVariant())
{
    m_rootItem = new SKRTreeItem();
    m_rootItem->setIsRootItem();

    connect(skrdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &SKRTreeModel::populate);
    connect(skrdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &SKRTreeModel::populate);


    //    connect(skrdata->treeHub(),
    //            &SKRTreeHub::paperAdded,
    //            this,
    //            &SKRTreeModel::addPaper);

    this->connectToSKRDataSignals();
}

QVariant SKRTreeModel::headerData(int section, Qt::Orientation orientation,
                                  int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool SKRTreeModel::setHeaderData(int             section,
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

QModelIndex SKRTreeModel::index(int row, int column, const QModelIndex& parent) const
{
    if (!hasIndex(row, column, parent)) return QModelIndex();

    // if (column != 0) return QModelIndex();

    SKRTreeItem *parentItem;


    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<SKRTreeItem *>(parent.internalPointer());

    SKRTreeItem *childItem = parentItem->child(m_allTreeItems, row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    } else {
        return QModelIndex();
    }
}

QModelIndex SKRTreeModel::parent(const QModelIndex& index) const
{
    if (!index.isValid()) return QModelIndex();


    SKRTreeItem *childItem  = static_cast<SKRTreeItem *>(index.internalPointer());
    SKRTreeItem *parentItem = childItem->parent(m_allTreeItems);

    if (parentItem == nullptr) {
        return QModelIndex();
    }

    // if (parentItem->isRootItem()) r

    QModelIndex parentIndex =
        createIndex(parentItem->row(m_allTreeItems), 0, parentItem);

    return parentIndex;
}

int SKRTreeModel::rowCount(const QModelIndex& parent) const
{
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) {
        return m_rootItem->childrenCount(m_allTreeItems);
    }


    SKRTreeItem *parentItem = static_cast<SKRTreeItem *>(parent.internalPointer());


    return parentItem->childrenCount(m_allTreeItems);
}

int SKRTreeModel::columnCount(const QModelIndex& parent) const
{
    if (!parent.isValid()) return 1;

    return 1;
}

QVariant SKRTreeModel::data(const QModelIndex& index, int role) const
{
    //    qDebug() << "checkIndex :"
    //             << (checkIndex(index,
    //                   QAbstractItemModel::CheckIndexOption::IndexIsValid
    //                   |
    //  QAbstractItemModel::CheckIndexOption::DoNotUseParent));
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid
                        |  QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (!index.isValid()) return QVariant();

    SKRTreeItem *item = static_cast<SKRTreeItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem()) {
        return item->data(SKRTreeItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole) {
        return item->data(SKRTreeItem::Roles::TitleRole);
    }


    if (role == SKRTreeItem::Roles::ProjectNameRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::TitleRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::TreeItemIdRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::LabelRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::ContentDateRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::TrashedRole) {
        return item->data(role);
    }
    return QVariant();
}

bool SKRTreeModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (data(index, role) != value) {
        SKRTreeItem *item = static_cast<SKRTreeItem *>(index.internalPointer());
        int projectId     = item->projectId();
        int treeItemId    = item->treeItemId();
        SKRResult result(this);

        this->disconnectFromSKRDataSignals();

        switch (role) {
        case SKRTreeItem::Roles::ProjectNameRole:
            result = skrdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case SKRTreeItem::Roles::ProjectIdRole:

            // useless
            break;

        case SKRTreeItem::Roles::TreeItemIdRole:

            // useless
            break;

        case SKRTreeItem::Roles::TitleRole:
            result = skrdata->treeHub()->setTitle(projectId, treeItemId, value.toString());
            break;

        case SKRTreeItem::Roles::LabelRole:
            result = skrdata->treePropertyHub()->setProperty(projectId, treeItemId,
                                                             "label", value.toString());
            break;

        case SKRTreeItem::Roles::IndentRole:
            result = skrdata->treeHub()->setIndent(projectId, treeItemId, value.toInt());
            break;

        case SKRTreeItem::Roles::SortOrderRole:
            result = skrdata->treeHub()->setSortOrder(projectId, treeItemId, value.toInt());
            break;

        case SKRTreeItem::Roles::TrashedRole:
            result = skrdata->treeHub()->setTrashedWithChildren(projectId,
                                                                treeItemId,
                                                                value.toBool());
            break;

        case SKRTreeItem::Roles::CreationDateRole:
            result = skrdata->treeHub()->setCreationDate(projectId,
                                                         treeItemId,
                                                         value.toDateTime());
            break;

        case SKRTreeItem::Roles::UpdateDateRole:
            result = skrdata->treeHub()->setUpdateDate(projectId,
                                                       treeItemId,
                                                       value.toDateTime());
            break;


        case SKRTreeItem::Roles::CharCountRole:
            result = skrdata->treePropertyHub()->setProperty(projectId,
                                                             treeItemId,
                                                             "char_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;

        case SKRTreeItem::Roles::WordCountRole:

            result = skrdata->treePropertyHub()->setProperty(projectId,
                                                             treeItemId,
                                                             "word_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;
        }


        this->connectToSKRDataSignals();

        if (!result.isSuccess()) {
            return false;
        }
        item->invalidateData(role);

        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags SKRTreeModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags; // FIXME: Implement me!
}

bool SKRTreeModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
    return false;
}

bool SKRTreeModel::insertColumns(int column, int count, const QModelIndex& parent)
{
    beginInsertColumns(parent, column, column + count - 1);

    // FIXME: Implement me!
    endInsertColumns();

    return false;
}

bool SKRTreeModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
    return false;
}

bool SKRTreeModel::removeColumns(int column, int count, const QModelIndex& parent)
{
    beginRemoveColumns(parent, column, column + count - 1);

    // FIXME: Implement me!
    endRemoveColumns();
    return false;
}

QHash<int, QByteArray>SKRTreeModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[SKRTreeItem::Roles::ProjectNameRole]  = "projectName";
    roles[SKRTreeItem::Roles::TreeItemIdRole]   = "treeItemId";
    roles[SKRTreeItem::Roles::ProjectIdRole]    = "projectId";
    roles[SKRTreeItem::Roles::TitleRole]        = "name";
    roles[SKRTreeItem::Roles::LabelRole]        = "label";
    roles[SKRTreeItem::Roles::IndentRole]       = "indent";
    roles[SKRTreeItem::Roles::SortOrderRole]    = "sortOrder";
    roles[SKRTreeItem::Roles::CreationDateRole] = "creationDate";
    roles[SKRTreeItem::Roles::UpdateDateRole]   = "updateDate";
    roles[SKRTreeItem::Roles::ContentDateRole]  = "contentDate";
    roles[SKRTreeItem::Roles::TrashedRole]      = "trashed";
    return roles;
}

void SKRTreeModel::populate()
{
    this->beginResetModel();

    m_allTreeItems.clear();

    for (int projectId : skrdata->projectHub()->getProjectIdList()) {
        auto idList         = skrdata->treeHub()->getAllIds(projectId);
        auto sortOrdersHash = skrdata->treeHub()->getAllSortOrders(projectId);
        auto indentsHash    = skrdata->treeHub()->getAllIndents(projectId);

        for (int sheetId : idList) {
            m_allTreeItems.append(new SKRTreeItem(projectId, sheetId,
                                                  indentsHash.value(sheetId),
                                                  sortOrdersHash.value(sheetId)));
        }
    }
    this->endResetModel();
}

void SKRTreeModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allTreeItems);
    this->endResetModel();
}

void SKRTreeModel::exploitSignalFromSKRData(int                projectId,
                                            int                treeItemId,
                                            SKRTreeItem::Roles role)
{
    SKRTreeItem *item = this->findPaperItem(projectId, treeItemId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        SKRTreeItem::Roles::TreeItemIdRole,
                                        treeItemId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : list) {
        SKRTreeItem *t = static_cast<SKRTreeItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

// --------------------------------------------------------------------

void SKRTreeModel::addPaper(int projectId, int treeItemId)
{
    // find parentIndex and row
    QModelIndex parentIndex;
    int row = 0;

    auto idList         = skrdata->treeHub()->getAllIds(projectId);
    auto sortOrdersHash = skrdata->treeHub()->getAllSortOrders(projectId);
    auto indentsHash    = skrdata->treeHub()->getAllIndents(projectId);


    int paperIndex      = idList.indexOf(treeItemId);
    int paperIndent     = indentsHash.value(treeItemId);
    int paperSortOrders = sortOrdersHash.value(treeItemId);

    bool parentFound = false;

    if (skrdata->projectHub()->getProjectIdList().count() > 1) {
        if (paperIndex == 0) {
            parentIndex = this->getModelIndex(projectId, -1).first();
            row         = 0;
            parentFound = true;
        }
    }
    else if (paperIndex == 0) {
        parentIndex = QModelIndex();
        row         = 0;
        parentFound = true;
    }

    if (!parentFound) {
        for (int i = paperIndex; i >= 0; --i) {
            int possibleParentId     = idList.at(i);
            int possibleParentIndent = indentsHash.value(possibleParentId);

            if (paperIndent == possibleParentIndent + 1) {
                auto modelIndexList = this->getModelIndex(projectId, possibleParentId);

                if (modelIndexList.isEmpty()) {
                    qWarning() << Q_FUNC_INFO <<
                        "if paperIndent == possibleParentIndent => modelIndexList.isEmpty()";
                    return;
                }
                parentIndex = modelIndexList.first();

                // int parentTreeItemId =
                // parentIndex.data(SKRTreeItem::Roles::TreeItemIdRole).toInt();
                row         = paperIndex - i - 1;
                parentFound = true;
                break;
            }
        }
    }

    if (!parentFound) {
        qWarning() << Q_FUNC_INFO << "parent not found, failsafe used";
        this->populate();
        return;
    }

    // find item just before in m_allTreeItems to determine item index to
    // insert in:

    int itemIndex = 0;

    if ((skrdata->projectHub()->getProjectIdList().count() == 1) && (paperIndex == 0)) { //
                                                                                         //
                                                                                         //
                                                                                         // so
                                                                                         //
                                                                                         //
                                                                                         // no
                                                                                         //
                                                                                         //
                                                                                         // project
                                                                                         //
                                                                                         //
                                                                                         // items
                                                                                         //
                                                                                         //
                                                                                         // and
                                                                                         //
                                                                                         //
                                                                                         // first
                                                                                         //
                                                                                         //
                                                                                         // item
        itemIndex = 0;
    }
    else {
        if (paperIndex == 0) {
            paperIndex = 1;
        }

        int idBefore            = idList.at(paperIndex - 1);
        SKRTreeItem *itemBefore = this->findPaperItem(projectId, idBefore);

        int indexBefore = m_allTreeItems.indexOf(itemBefore);

        itemIndex = indexBefore + 1;

        //        if(itemIndex >= m_allTreeItems.count() && paperIndent ==
        // itemBefore->indent() + 1){
        //            qWarning() << Q_FUNC_INFO << "last in the m_allTreeItems
        // list and child of previous item, so failsafe used";
        //            this->populate();
        //            return;
        //        }
    }


    beginInsertRows(parentIndex, row, row);

    m_allTreeItems.insert(itemIndex, new SKRTreeItem(projectId, treeItemId,
                                                     indentsHash.value(treeItemId),
                                                     sortOrdersHash.value(treeItemId)));
    endInsertRows();
}

// --------------------------------------------------------------------

SKRTreeItem * SKRTreeModel::findPaperItem(int projectId, int treeItemId)
{
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        SKRTreeItem::Roles::TreeItemIdRole,
                                        treeItemId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);
    SKRTreeItem *item = nullptr;

    for (const QModelIndex& modelIndex : list) {
        SKRTreeItem *t = static_cast<SKRTreeItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) item = t;
    }

    return item;
}

void SKRTreeModel::connectToSKRDataSignals()
{
    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::titleChanged, this,
                                           [this](int projectId, int treeItemId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId, SKRTreeItem::Roles::TitleRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(skrdata->treePropertyHub(),
                                           &SKRPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "label") this->exploitSignalFromSKRData(projectId, paperCode,
                                                            SKRTreeItem::Roles::LabelRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::treeItemIdChanged, this,
                                           [this](int projectId, int treeItemId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::TreeItemIdRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::indentChanged, this,
                                           [this](int projectId, int treeItemId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::IndentRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList.append(this->connect(skrdata->treeHub(),
                                               &SKRTreeHub::sortOrderChanged, this,
                                               [this](int projectId, int treeItemId,
                                                      int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::SortOrderRole);
    }));

    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::updateDateChanged, this,
                                           [this](int projectId, int treeItemId,
                                                  const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::UpdateDateRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::trashedChanged, this,
                                           [this](int projectId, int treeItemId,
                                                  bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::TrashedRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(skrdata->treePropertyHub(),
                                           &SKRPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "char_count") this->exploitSignalFromSKRData(projectId, paperCode,
                                                                 SKRTreeItem::Roles::
                                                                 CharCountRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(skrdata->treePropertyHub(),
                                           &SKRPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "word_count") this->exploitSignalFromSKRData(projectId, paperCode,
                                                                 SKRTreeItem::Roles::
                                                                 WordCountRole);
    }, Qt::UniqueConnection);
}

void SKRTreeModel::disconnectFromSKRDataSignals()
{
    // disconnect from PLMPaperHub signals :
    for (const QMetaObject::Connection& connection : m_dataConnectionsList) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}

// -----------------------------------------------------------------------------------


QModelIndexList SKRTreeModel::getModelIndex(int projectId, int treeItemId)
{
    QModelIndexList list;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             SKRTreeItem::Roles::ProjectIdRole,
                                             projectId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap |
                                             Qt::MatchFlag::MatchRecursive);

    for (const QModelIndex& modelIndex : qAsConst(modelList)) {
        int indexTreeItemId = modelIndex.data(SKRTreeItem::Roles::TreeItemIdRole).toInt();

        if (indexTreeItemId == treeItemId) {
            list.append(modelIndex);
        }
    }

    return list;
}

// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
