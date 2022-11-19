/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrtreelistmodel.cpp
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
#include "skrtreelistmodel.h"

SKRTreeListModel::SKRTreeListModel(QObject *parent)
    : QAbstractTableModel(parent), m_headerData(QVariant())
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();


    //    m_rootItem = new SKRTreeItem();
    //    m_rootItem->setIsRootItem();


    connect(skrdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &SKRTreeListModel::populate);
    connect(skrdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &SKRTreeListModel::populate);


    connect(m_treeHub,
            &SKRTreeHub::treeItemAdded,
            this, [this](){
        this->populate();
    });

    connect(m_treeHub,
            &SKRTreeHub::treeItemsAdded,
            this, [this](){
        this->populate();
    });

    connect(m_treeHub,
            &SKRTreeHub::treeItemMoved,
            this, [this](){
        this->populate();
    });

    connect(m_treeHub,
            &SKRTreeHub::treeItemRemoved,
            this, [this](){
        this->populate();
    });

    connect(m_treeHub,
            &SKRTreeHub::indentChanged,
            this,
            &SKRTreeListModel::refreshAfterIndentChanged);

    connect(m_treeHub,
            &SKRTreeHub::trashedChanged, // careful, treeItem is trashed = true,
            // not a true removal
            this,
            &SKRTreeListModel::refreshAfterTrashedStateChanged);

    connect(skrdata->projectHub(),
            &PLMProjectHub::projectIsBackupChanged,
            this,
            &SKRTreeListModel::refreshAfterProjectIsBackupChanged);

    connect(skrdata->projectHub(),
            &PLMProjectHub::activeProjectChanged,
            this,
            &SKRTreeListModel::refreshAfterProjectIsActiveChanged);

    this->connectToSKRDataSignals();
}

QVariant SKRTreeListModel::headerData(int section, Qt::Orientation orientation,
                                      int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool SKRTreeListModel::setHeaderData(int             section,
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

QModelIndex SKRTreeListModel::index(int row, int column, const QModelIndex& parent) const
{
    //    SKRTreeItem *parentItem;

    //    if (!parent.isValid()) parentItem = m_rootItem;

    SKRTreeItem *childItem = m_allTreeItems.at(row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    }
    return QModelIndex();
}

int SKRTreeListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allTreeItems.count();
}

int SKRTreeListModel::columnCount(const QModelIndex& parent) const
{
    if (parent.isValid()) return 0;

    return 1;
}

QVariant SKRTreeListModel::data(const QModelIndex& index, int role) const
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

    if (role == SKRTreeItem::Roles::InternalTitleRole) {
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

    if (role == SKRTreeItem::Roles::TypeRole) {
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

    if (role == SKRTreeItem::Roles::CharCountRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::WordCountRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::CharCountGoalRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::WordCountGoalRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::CharCountWithChildrenRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::WordCountWithChildrenRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::TrashedRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::ProjectIsBackupRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::ProjectIsActiveRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::IsRenamableRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::IsMovableRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::CanAddSiblingTreeItemRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::CanAddChildTreeItemRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::IsTrashableRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::IsOpenableRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::IsCopyableRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::OtherPropertiesRole) {
        return item->data(role);
    }

    if (role == SKRTreeItem::Roles::CutCopyRole) {
        return item->data(role);
    }

    return QVariant();
}

bool SKRTreeListModel::setData(const QModelIndex& index, const QVariant& value, int role)
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
            result = m_treeHub->setTitle(projectId, treeItemId, value.toString());

            break;

        case SKRTreeItem::Roles::TypeRole:
            result = m_treeHub->setType(projectId, treeItemId, value.toString());

            break;

        case SKRTreeItem::Roles::LabelRole:
            result = m_propertyHub->setProperty(projectId, treeItemId,
                                                "label", value.toString());
            break;

        case SKRTreeItem::Roles::IndentRole:
            result = m_treeHub->setIndent(projectId, treeItemId, value.toInt());
            break;

        case SKRTreeItem::Roles::SortOrderRole:
            result = m_treeHub->setSortOrder(projectId, treeItemId, value.toInt());
            IFOKDO(result, m_treeHub->renumberSortOrders(projectId));

            for (SKRTreeItem *item : qAsConst(m_allTreeItems)) {
                item->invalidateData(role);
            }
            this->populate();

            break;

        case SKRTreeItem::Roles::TrashedRole:
            result = m_treeHub->setTrashedWithChildren(projectId,
                                                       treeItemId,
                                                       value.toBool());
            break;

        case SKRTreeItem::Roles::CreationDateRole:
            result = m_treeHub->setCreationDate(projectId,
                                                treeItemId,
                                                value.toDateTime());
            break;

        case SKRTreeItem::Roles::UpdateDateRole:
            result = m_treeHub->setUpdateDate(projectId,
                                              treeItemId,
                                              value.toDateTime());
            break;

        case SKRTreeItem::Roles::HasChildrenRole:

            // useless
            break;

        case SKRTreeItem::Roles::CharCountRole:
            result = m_propertyHub->setProperty(projectId,
                                                treeItemId,
                                                "char_count",
                                                QString::number(
                                                    value.toInt()));
            break;

        case SKRTreeItem::Roles::WordCountRole:

            result = m_propertyHub->setProperty(projectId,
                                                treeItemId,
                                                "word_count",
                                                QString::number(
                                                    value.toInt()));
            break;

        case SKRTreeItem::Roles::ProjectIsActiveRole:

            skrdata->projectHub()->setActiveProject(projectId);
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

Qt::ItemFlags SKRTreeListModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;
}

QHash<int, QByteArray>SKRTreeListModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[SKRTreeItem::Roles::ProjectNameRole]           = "projectName";
    roles[SKRTreeItem::Roles::TreeItemIdRole]            = "treeItemId";
    roles[SKRTreeItem::Roles::ProjectIdRole]             = "projectId";
    roles[SKRTreeItem::Roles::TitleRole]                 = "title";
    roles[SKRTreeItem::Roles::InternalTitleRole]         = "internalTitle";
    roles[SKRTreeItem::Roles::LabelRole]                 = "label";
    roles[SKRTreeItem::Roles::IndentRole]                = "indent";
    roles[SKRTreeItem::Roles::SortOrderRole]             = "sortOrder";
    roles[SKRTreeItem::Roles::TypeRole]                  = "type";
    roles[SKRTreeItem::Roles::CreationDateRole]          = "creationDate";
    roles[SKRTreeItem::Roles::UpdateDateRole]            = "updateDate";
    roles[SKRTreeItem::Roles::ContentDateRole]           = "contentDate";
    roles[SKRTreeItem::Roles::HasChildrenRole]           = "hasChildren";
    roles[SKRTreeItem::Roles::TrashedRole]               = "trashed";
    roles[SKRTreeItem::Roles::WordCountRole]             = "wordCount";
    roles[SKRTreeItem::Roles::CharCountRole]             = "charCount";
    roles[SKRTreeItem::Roles::WordCountGoalRole]         = "wordCountGoal";
    roles[SKRTreeItem::Roles::CharCountGoalRole]         = "charCountGoal";
    roles[SKRTreeItem::Roles::WordCountWithChildrenRole] = "wordCountWithChildren";
    roles[SKRTreeItem::Roles::CharCountWithChildrenRole] = "charCountWithChildren";
    roles[SKRTreeItem::Roles::ProjectIsBackupRole]       = "projectIsBackup";
    roles[SKRTreeItem::Roles::ProjectIsActiveRole]       = "projectIsActive";
    roles[SKRTreeItem::Roles::IsRenamableRole]           = "isRenamable";
    roles[SKRTreeItem::Roles::IsMovableRole]             = "isMovable";
    roles[SKRTreeItem::Roles::CanAddSiblingTreeItemRole] = "canAddSiblingTreeItem";
    roles[SKRTreeItem::Roles::CanAddChildTreeItemRole]   = "canAddChildTreeItem";
    roles[SKRTreeItem::Roles::IsTrashableRole]           = "isTrashable";
    roles[SKRTreeItem::Roles::IsOpenableRole]            = "isOpenable";
    roles[SKRTreeItem::Roles::IsCopyableRole]            = "isCopyable";
    roles[SKRTreeItem::Roles::OtherPropertiesRole]       = "otherProperties";
    roles[SKRTreeItem::Roles::CutCopyRole]               = "cutCopy";
    return roles;
}

void SKRTreeListModel::populate()
{
    this->beginResetModel();
    this->resetAllTreeItemsList();
    this->endResetModel();
}

void SKRTreeListModel::resetAllTreeItemsList()
{
    m_allTreeItems.clear();

    for (int projectId : skrdata->projectHub()->getProjectIdList()) {
        auto idList         = m_treeHub->getAllIds(projectId);
        auto sortOrdersHash = m_treeHub->getAllSortOrders(projectId);
        auto indentsHash    = m_treeHub->getAllIndents(projectId);

        for (int treeItemId : qAsConst(idList)) {
            m_allTreeItems.append(new SKRTreeItem(projectId, treeItemId,
                                                  indentsHash.value(treeItemId),
                                                  sortOrdersHash.value(treeItemId)));
        }
    }
}

void SKRTreeListModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allTreeItems);
    this->endResetModel();
}

// --------------------------------------------------------------------
///
/// \brief SKRTreeListModel::refreshAfterTrashedStateChanged
/// \param projectId
/// \param treeItemId
/// \param newTrashedState
/// careful, treeItem is trashed = true, not a true removal
void SKRTreeListModel::refreshAfterTrashedStateChanged(int  projectId,
                                                       int  treeItemId,
                                                       bool newTrashedState)
{
    Q_UNUSED(projectId)
    Q_UNUSED(treeItemId)
    Q_UNUSED(newTrashedState)

    for (SKRTreeItem *item : qAsConst(m_allTreeItems)) {
        item->invalidateData(SKRTreeItem::Roles::TrashedRole);

        // needed to refresh the parent item when no child anymore
        item->invalidateData(SKRTreeItem::Roles::HasChildrenRole);
    }
}

// --------------------------------------------------------------------

void SKRTreeListModel::refreshAfterProjectIsBackupChanged(int  projectId,
                                                          bool isProjectABackup)
{
    Q_UNUSED(projectId)
    Q_UNUSED(isProjectABackup)

    for (SKRTreeItem *item : qAsConst(m_allTreeItems)) {
        item->invalidateData(SKRTreeItem::Roles::ProjectIsBackupRole);
    }
}

// --------------------------------------------------------------------

void SKRTreeListModel::refreshAfterProjectIsActiveChanged(int projectId)
{
    Q_UNUSED(projectId)

    for (SKRTreeItem *item : qAsConst(m_allTreeItems)) {
        item->invalidateData(SKRTreeItem::Roles::ProjectIsActiveRole);
    }
}

// --------------------------------------------------------------------

void SKRTreeListModel::refreshAfterIndentChanged(const TreeItemAddress &treeItemAddress,
                                                 int newIndent)
{
    for (SKRTreeItem *item : qAsConst(m_allTreeItems)) {
        item->invalidateData(SKRTreeItem::Roles::ProjectIsActiveRole);
        item->invalidateData(SKRTreeItem::Roles::HasChildrenRole);
    }
}

// --------------------------------------------------------------------
///
/// \brief SKRTreeListModel::sortAllTreeItemItems
/// sort by SortOrder
void SKRTreeListModel::sortAllTreeItemItems() {
    std::sort(m_allTreeItems.begin(), m_allTreeItems.end(), [](SKRTreeItem *item1, SKRTreeItem *item2) -> bool {
        return item1->sortOrder() < item2->sortOrder();
    }
    );
}

// --------------------------------------------------------------------

void SKRTreeListModel::exploitSignalFromSKRData(int                projectId,
                                                int                treeItemId,
                                                SKRTreeItem::Roles role)
{
    SKRTreeItem *item = this->getItem(projectId, treeItemId);

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

    for (const QModelIndex& modelIndex : qAsConst(list)) {
        SKRTreeItem *t = static_cast<SKRTreeItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

// ---------------------------------------------------------------------------

void SKRTreeListModel::connectToSKRDataSignals()
{
    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::titleChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId, SKRTreeItem::Roles::TitleRole);
    });

    m_dataConnectionsList << this->connect(skrdata->projectHub(),
                                           &PLMProjectHub::projectNameChanged, this,
                                           [this](int projectId,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, 0,
                                       SKRTreeItem::Roles::ProjectNameRole);
    });


    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::treeItemIdChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::TreeItemIdRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::indentChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::IndentRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::sortOrderChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::SortOrderRole);
    });


    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::updateDateChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::UpdateDateRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::trashedChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       SKRTreeItem::Roles::TrashedRole);
    });

    m_dataConnectionsList << this->connect(skrdata->projectHub(),
                                           &PLMProjectHub::activeProjectChanged, this,
                                           [this](int projectId) {
        Q_UNUSED(projectId)

        for (int _projectId : skrdata->projectHub()->getProjectIdList()) {
            this->exploitSignalFromSKRData(_projectId, -1,
                                           SKRTreeItem::Roles::ProjectIsActiveRole);
        }
    });

    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::cutCopyChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress, bool value) {
        Q_UNUSED(value)
       this->exploitSignalFromSKRData(projectId, treeItemId,
                                           SKRTreeItem::Roles::CutCopyRole);

    });

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &SKRPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            treeItemCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)


        if (name == "label") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                            SKRTreeItem::Roles::LabelRole);

        if (name == "char_count") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 SKRTreeItem::Roles::
                                                                 CharCountRole);

        if (name == "word_count") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 SKRTreeItem::Roles::
                                                                 WordCountRole);

        if (name == "char_count_goal") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 SKRTreeItem::Roles::
                                                                 CharCountGoalRole);

        if (name == "word_count_goal") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 SKRTreeItem::Roles::
                                                                 WordCountGoalRole);

        if (name == "char_count_with_children") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                               SKRTreeItem::Roles::
                                                                               CharCountWithChildrenRole);

        if (name == "word_count_with_children") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                               SKRTreeItem::Roles::
                                                                               WordCountWithChildrenRole);

        if (name == "is_renamable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                   SKRTreeItem::Roles::
                                                                   IsRenamableRole);

        if (name == "is_movable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 SKRTreeItem::Roles::
                                                                 IsMovableRole);

        if (name == "can_add_sibling_tree_item") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                                SKRTreeItem::Roles::
                                                                                CanAddSiblingTreeItemRole);

        if (name == "can_add_child_tree_item") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                              SKRTreeItem::Roles::
                                                                              CanAddChildTreeItemRole);

        if (name == "is_trashable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                   SKRTreeItem::Roles::
                                                                   IsTrashableRole);

        if (name == "is_openable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                  SKRTreeItem::Roles::
                                                                  IsOpenableRole);

        if (name == "is_copyable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                  SKRTreeItem::Roles::
                                                                  IsCopyableRole);
        else {
            this->exploitSignalFromSKRData(projectId, treeItemCode,
                                           SKRTreeItem::Roles::
                                           OtherPropertiesRole);
        }
    });
}

void SKRTreeListModel::disconnectFromSKRDataSignals()
{
    // disconnect from SKRTreeHub signals :
    for (const QMetaObject::Connection& connection : qAsConst(m_dataConnectionsList)) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}

// -----------------------------------------------------------------------------------


QModelIndexList SKRTreeListModel::getModelIndex(const TreeItemAddress &treeItemAddress)
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

// -----------------------------------------------------------------------------------

SKRTreeItem * SKRTreeListModel::getParentTreeItem(SKRTreeItem *childItem)
{
    return childItem->parent(m_allTreeItems);
}

// -----------------------------------------------------------------------------------
SKRTreeItem * SKRTreeListModel::getItem(const TreeItemAddress &treeItemAddress)
{
    SKRTreeItem *result_item = nullptr;

    for (SKRTreeItem *item : qAsConst(m_allTreeItems)) {
        if ((item->projectId() == projectId) && (item->treeItemId() == treeItemId)) {
            result_item = item;
            break;
        }
    }

    if (!result_item) {
        //    qDebug() << "result_item is null";
    }

    return result_item;
}

// -----------------------------------------------------------------------------------
