/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrpaperlistmodel.cpp
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
#include "skrpaperlistmodel.h"

SKRPaperListModel::SKRPaperListModel(QObject *parent, SKR::PaperType paperType)
    : QAbstractListModel(parent), m_paperType(paperType), m_headerData(QVariant())
{
    if (paperType == SKR::Sheet) {
        m_paperHub    = plmdata->sheetHub();
        m_propertyHub = plmdata->sheetPropertyHub();
    }
    else if (paperType == SKR::Note) {
        m_paperHub    = plmdata->noteHub();
        m_propertyHub = plmdata->notePropertyHub();
    }
    else {
        qFatal(this->metaObject()->className(), "Invalid PaperType");
    }


    m_rootItem = new SKRPaperItem(m_paperType);
    m_rootItem->setIsRootItem();

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &SKRPaperListModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &SKRPaperListModel::populate);


    connect(m_paperHub,
            &PLMPaperHub::paperAdded,
            this,
            &SKRPaperListModel::refreshAfterDataAddition);

    connect(m_paperHub,
            &PLMPaperHub::paperMoved,
            this,
            &SKRPaperListModel::refreshAfterDataMove);

    connect(m_paperHub,
            &PLMPaperHub::paperRemoved,
            this,
            &SKRPaperListModel::refreshAfterDataRemove);

    connect(m_paperHub,
            &PLMPaperHub::indentChanged,
            this,
            &SKRPaperListModel::refreshAfterIndentChanged);

    connect(m_paperHub,
            &PLMPaperHub::trashedChanged, // careful, paper is trashed = true,
                                          // not a true removal
            this,
            &SKRPaperListModel::refreshAfterTrashedStateChanged);

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectIsBackupChanged,
            this,
            &SKRPaperListModel::refreshAfterProjectIsBackupChanged);

    connect(plmdata->projectHub(),
            &PLMProjectHub::activeProjectChanged,
            this,
            &SKRPaperListModel::refreshAfterProjectIsActiveChanged);

    this->connectToPLMDataSignals();
}

QVariant SKRPaperListModel::headerData(int section, Qt::Orientation orientation,
                                       int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool SKRPaperListModel::setHeaderData(int             section,
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

QModelIndex SKRPaperListModel::index(int row, int column, const QModelIndex& parent) const
{
    if (column != 0) return QModelIndex();

    SKRPaperItem *parentItem;

    if (!parent.isValid()) parentItem = m_rootItem;

    SKRPaperItem *childItem = m_allPaperItems.at(row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    }
    return QModelIndex();
}

int SKRPaperListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the
    // list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not
    // become a tree model.
    if (parent.isValid()) return 0;

    return m_allPaperItems.count();
}

QVariant SKRPaperListModel::data(const QModelIndex& index, int role) const
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

    SKRPaperItem *item = static_cast<SKRPaperItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem()) {
        return item->data(SKRPaperItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole) {
        return item->data(SKRPaperItem::Roles::NameRole);
    }


    if (role == SKRPaperItem::Roles::ProjectNameRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::NameRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::PaperIdRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::LabelRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::ContentDateRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::TrashedRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::ProjectIsBackupRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::IsRenamableRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::IsMovableRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::CanAddPaperRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::IsTrashableRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::IsOpenableRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::IsCopyableRole) {
        return item->data(role);
    }

    if (role == SKRPaperItem::Roles::AttributesRole) {
        return item->data(role);
    }

    return QVariant();
}

bool SKRPaperListModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (data(index, role) != value) {
        SKRPaperItem *item = static_cast<SKRPaperItem *>(index.internalPointer());
        int projectId      = item->projectId();
        int paperId        = item->paperId();
        SKRResult result;

        this->disconnectFromPLMDataSignals();

        switch (role) {
        case SKRPaperItem::Roles::ProjectNameRole:
            result = plmdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case SKRPaperItem::Roles::ProjectIdRole:

            // useless
            break;

        case SKRPaperItem::Roles::PaperIdRole:

            // useless
            break;

        case SKRPaperItem::Roles::NameRole:
            result = m_paperHub->setTitle(projectId, paperId, value.toString());

            break;

        case SKRPaperItem::Roles::LabelRole:
            result = m_propertyHub->setProperty(projectId, paperId,
                                               "label", value.toString());
            break;

        case SKRPaperItem::Roles::IndentRole:
            result = m_paperHub->setIndent(projectId, paperId, value.toInt());
            break;

        case SKRPaperItem::Roles::SortOrderRole:
            result = m_paperHub->setSortOrder(projectId, paperId, value.toInt());
            IFOKDO(result, m_paperHub->renumberSortOrders(projectId));

            for (SKRPaperItem *item : m_allPaperItems) {
                item->invalidateData(role);
            }
            this->populate();

            break;

        case SKRPaperItem::Roles::TrashedRole:
            result = m_paperHub->setTrashedWithChildren(projectId,
                                                       paperId,
                                                       value.toBool());
            break;

        case SKRPaperItem::Roles::CreationDateRole:
            result = m_paperHub->setCreationDate(projectId,
                                                paperId,
                                                value.toDateTime());
            break;

        case SKRPaperItem::Roles::UpdateDateRole:
            result = m_paperHub->setUpdateDate(projectId,
                                              paperId,
                                              value.toDateTime());
            break;

        case SKRPaperItem::Roles::ContentDateRole:
            result = m_paperHub->setContentDate(projectId,
                                               paperId,
                                               value.toDateTime());
            break;

        case SKRPaperItem::Roles::HasChildrenRole:
            // useless
            break;

        case SKRPaperItem::Roles::CharCountRole:
            result = m_propertyHub->setProperty(projectId,
                                               paperId,
                                               "char_count",
                                               QString::number(
                                                   value.toInt()));
            break;

        case SKRPaperItem::Roles::WordCountRole:

            result = m_propertyHub->setProperty(projectId,
                                               paperId,
                                               "word_count",
                                               QString::number(
                                                   value.toInt()));
            break;

        case SKRPaperItem::Roles::ProjectIsActiveRole:

            plmdata->projectHub()->setActiveProject(projectId);
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

Qt::ItemFlags SKRPaperListModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;
}

QHash<int, QByteArray>SKRPaperListModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[SKRPaperItem::Roles::ProjectNameRole]     = "projectName";
    roles[SKRPaperItem::Roles::PaperIdRole]         = "paperId";
    roles[SKRPaperItem::Roles::ProjectIdRole]       = "projectId";
    roles[SKRPaperItem::Roles::NameRole]            = "name";
    roles[SKRPaperItem::Roles::LabelRole]           = "label";
    roles[SKRPaperItem::Roles::IndentRole]          = "indent";
    roles[SKRPaperItem::Roles::SortOrderRole]       = "sortOrder";
    roles[SKRPaperItem::Roles::CreationDateRole]    = "creationDate";
    roles[SKRPaperItem::Roles::UpdateDateRole]      = "updateDate";
    roles[SKRPaperItem::Roles::ContentDateRole]     = "contentDate";
    roles[SKRPaperItem::Roles::HasChildrenRole]     = "hasChildren";
    roles[SKRPaperItem::Roles::TrashedRole]         = "trashed";
    roles[SKRPaperItem::Roles::WordCountRole]       = "wordCount";
    roles[SKRPaperItem::Roles::CharCountRole]       = "charCount";
    roles[SKRPaperItem::Roles::ProjectIsBackupRole] = "projectIsBackup";
    roles[SKRPaperItem::Roles::ProjectIsActiveRole] = "projectIsActive";
    roles[SKRPaperItem::Roles::IsRenamableRole]     = "isRenamable";
    roles[SKRPaperItem::Roles::IsMovableRole]       = "isMovable";
    roles[SKRPaperItem::Roles::CanAddPaperRole]     = "canAddPaper";
    roles[SKRPaperItem::Roles::IsTrashableRole]     = "isTrashable";
    roles[SKRPaperItem::Roles::IsOpenableRole]      = "isOpenable";
    roles[SKRPaperItem::Roles::IsCopyableRole]      = "isCopyable";
    roles[SKRPaperItem::Roles::AttributesRole]      = "attributes";
    return roles;
}

void SKRPaperListModel::populate()
{
    this->beginResetModel();

    m_allPaperItems.clear();

    for (int projectId : plmdata->projectHub()->getProjectIdList()) {
        SKRPaperItem *projectItem = new SKRPaperItem(m_paperType);
        projectItem->setIsProjectItem(projectId);
        m_allPaperItems.append(projectItem);


        auto idList         = m_paperHub->getAllIds(projectId);
        auto sortOrdersHash = m_paperHub->getAllSortOrders(projectId);
        auto indentsHash    = m_paperHub->getAllIndents(projectId);

        for (int paperId : idList) {
            m_allPaperItems.append(new SKRPaperItem(m_paperType, projectId, paperId,
                                                    indentsHash.value(paperId),
                                                    sortOrdersHash.value(paperId)));
        }
    }
    this->endResetModel();
}

void SKRPaperListModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allPaperItems);
    this->endResetModel();
}

// --------------------------------------------------------------------

void SKRPaperListModel::refreshAfterDataAddition(int projectId, int paperId)
{
    // find parentIndex and row
    //    QModelIndex parentIndex;
    int row = 0;

    auto idList         = m_paperHub->getAllIds(projectId);
    auto sortOrdersHash = m_paperHub->getAllSortOrders(projectId);
    auto indentsHash    = m_paperHub->getAllIndents(projectId);


    int paperIndex = idList.indexOf(paperId);

    if (paperIndex == 0) { // meaning the parent have to be a project item
        for (SKRPaperItem *item : m_allPaperItems) {
            item->invalidateData(SKRPaperItem::Roles::SortOrderRole);
            item->invalidateData(SKRPaperItem::Roles::HasChildrenRole);
        }


        // find project PLMPaperItem:
        SKRPaperItem *itemBefore = this->getItem(projectId, -1);

        int indexBefore = m_allPaperItems.indexOf(itemBefore);

        int itemIndex = indexBefore + 1;


        beginInsertRows(QModelIndex(), itemIndex, itemIndex);

        m_allPaperItems.insert(itemIndex,
                               new SKRPaperItem(m_paperType, projectId, paperId,
                                                indentsHash.value(paperId),
                                                sortOrdersHash.value(paperId)));
        this->index(itemIndex, 0, QModelIndex());
        endInsertRows();

        return;
    }

    //    if (!parentFound) {
    //        for (int i = paperIndex - 1; i >= 0; --i) {
    //            int possibleParentId     = idList.at(i);
    //            int possibleParentIndent =
    // indentsHash.value(possibleParentId);

    //            if (paperIndent == possibleParentIndent + 1) {
    //                //                auto modelIndexList =
    //                // this->getModelIndex(projectId, possibleParentId);
    //                //                if(modelIndexList.isEmpty()){
    //                //                    qWarning() << Q_FUNC_INFO << "if
    //                // paperIndent == possibleParentIndent =>
    //                // modelIndexList.isEmpty()";
    //                //                    return;
    //                //                }
    //                //                parentIndex = modelIndexList.first();
    //                // int parentPaperId =
    //                //
    // parentIndex.data(SKRPaperItem::Roles::PaperIdRole).toInt();
    //                row         = paperIndex - i;
    //                parentFound = true;
    //                break;
    //            }
    //        }
    //    }

    //    if (!parentFound) {
    //        qWarning() << Q_FUNC_INFO << "parent not found, failsafe used";
    //        this->populate();
    //        return;
    //    }


    // find item just before in m_allNoteItems to determine item index to insert
    // in:


    int idBefore             = idList.at(paperIndex - 1);
    SKRPaperItem *itemBefore = this->getItem(projectId, idBefore);

    // needed because m_allPaperItems can have multiple projects :
    int indexBefore = m_allPaperItems.indexOf(itemBefore);

    int itemIndex = indexBefore + 1;

    //        if(itemIndex >= m_allNoteItems.count() && paperIndent ==
    // itemBefore->indent() + 1){
    //            qWarning() << Q_FUNC_INFO << "last in the m_allNoteItems list
    // and child of previous item, so failsafe used";
    //            this->populate();
    //            return;
    //        }


    for (SKRPaperItem *item : m_allPaperItems) {
        item->invalidateData(SKRPaperItem::Roles::SortOrderRole);
        item->invalidateData(SKRPaperItem::Roles::HasChildrenRole);
    }


    beginInsertRows(QModelIndex(), itemIndex, itemIndex);

    m_allPaperItems.insert(itemIndex, new SKRPaperItem(m_paperType, projectId, paperId,
                                                       indentsHash.value(paperId),
                                                       sortOrdersHash.value(paperId)));
    this->index(itemIndex, 0, QModelIndex());
    endInsertRows();
}

// --------------------------------------------------------------------

void SKRPaperListModel::refreshAfterDataRemove(int projectId, int paperId)
{
    Q_UNUSED(projectId)
    Q_UNUSED(paperId)

    this->populate();
}

// --------------------------------------------------------------------

void SKRPaperListModel::refreshAfterDataMove(int sourceProjectId,
                                             int sourcePaperId,
                                             int targetProjectId,
                                             int targetPaperId)
{
    int sourceIndex = -2;
    int targetIndex = -2;
    int sourceRow   = -2;
    int targetRow   = -2;


    SKRPaperItem *sourceItem = this->getItem(sourceProjectId, sourcePaperId);

    if (!sourceItem) {
        qWarning() << "refreshAfterDataMove no sourceItem";
        return;
    }
    SKRPaperItem *targetItem = this->getItem(targetProjectId, targetPaperId);

    if (!targetItem) {
        qWarning() << "refreshAfterDataMove no targetItem";
        return;
    }

    int i = 0;

    for (SKRPaperItem *item : m_allPaperItems) {
        if (item->paperId() == sourcePaperId) {
            sourceIndex = i;
        }

        if (item->paperId() == targetPaperId) {
            targetIndex = i;
        }
        i++;
    }


    sourceRow = sourceIndex;
    targetRow = targetIndex;

    if (sourceRow < targetRow) {
        targetRow += 1;
    }

    for (SKRPaperItem *item : m_allPaperItems) {
        item->invalidateData(SKRPaperItem::Roles::SortOrderRole);
    }

    beginMoveRows(QModelIndex(), sourceRow, sourceRow, QModelIndex(), targetRow);

    SKRPaperItem *tempItem = m_allPaperItems.takeAt(sourceIndex);

    m_allPaperItems.insert(targetIndex, tempItem);

    endMoveRows();
}

// --------------------------------------------------------------------
///
/// \brief SKRPaperListModel::refreshAfterTrashedStateChanged
/// \param projectId
/// \param paperId
/// \param newTrashedState
/// careful, paper is trashed = true, not a true removal
void SKRPaperListModel::refreshAfterTrashedStateChanged(int  projectId,
                                                        int  paperId,
                                                        bool newTrashedState)
{
    Q_UNUSED(projectId)
    Q_UNUSED(paperId)
    Q_UNUSED(newTrashedState)

    for (SKRPaperItem *item : m_allPaperItems) {
        item->invalidateData(SKRPaperItem::Roles::TrashedRole);
        item->invalidateData(SKRPaperItem::Roles::HasChildrenRole); // needed to
                                                                    // refresh
                                                                    // the
                                                                    // parent
                                                                    // item when
                                                                    // no child
                                                                    // anymore
    }
}

// --------------------------------------------------------------------

void SKRPaperListModel::refreshAfterProjectIsBackupChanged(int  projectId,
                                                           bool isProjectABackup)
{
    Q_UNUSED(projectId)
    Q_UNUSED(isProjectABackup)

    for (SKRPaperItem *item : m_allPaperItems) {
        item->invalidateData(SKRPaperItem::Roles::ProjectIsBackupRole);
    }
}

// --------------------------------------------------------------------

void SKRPaperListModel::refreshAfterProjectIsActiveChanged(int projectId)
{
    Q_UNUSED(projectId)

    for (SKRPaperItem *item : m_allPaperItems) {
        item->invalidateData(SKRPaperItem::Roles::ProjectIsActiveRole);
    }
}

// --------------------------------------------------------------------

void SKRPaperListModel::refreshAfterIndentChanged(int projectId, int paperId,
                                                  int newIndent)
{
    for (SKRPaperItem *item : m_allPaperItems) {
        item->invalidateData(SKRPaperItem::Roles::ProjectIsActiveRole);
        item->invalidateData(SKRPaperItem::Roles::HasChildrenRole);
    }
}

// --------------------------------------------------------------------

void SKRPaperListModel::exploitSignalFromPLMData(int                 projectId,
                                                 int                 paperId,
                                                 SKRPaperItem::Roles role)
{
    SKRPaperItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        SKRPaperItem::Roles::PaperIdRole,
                                        paperId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : list) {
        SKRPaperItem *t = static_cast<SKRPaperItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

// ---------------------------------------------------------------------------

void SKRPaperListModel::connectToPLMDataSignals()
{
    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::titleChanged, this,
                                           [this](int projectId, int paperId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId, SKRPaperItem::Roles::NameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->projectHub(),
                                           &PLMProjectHub::projectNameChanged, this,
                                           [this](int projectId,
                                                  const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, -1,
                                       SKRPaperItem::Roles::ProjectNameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "label") this->exploitSignalFromPLMData(projectId, paperCode,
                                                            SKRPaperItem::Roles::LabelRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::paperIdChanged, this,
                                           [this](int projectId, int paperId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       SKRPaperItem::Roles::PaperIdRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::indentChanged, this,
                                           [this](int projectId, int paperId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       SKRPaperItem::Roles::IndentRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::sortOrderChanged, this,
                                           [this](int projectId, int paperId,
                                                  int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       SKRPaperItem::Roles::SortOrderRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::contentDateChanged, this,
                                           [this](int projectId, int paperId,
                                                  const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       SKRPaperItem::Roles::ContentDateRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::updateDateChanged, this,
                                           [this](int projectId, int paperId,
                                                  const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       SKRPaperItem::Roles::UpdateDateRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_paperHub,
                                           &PLMPaperHub::trashedChanged, this,
                                           [this](int projectId, int paperId,
                                                  bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       SKRPaperItem::Roles::TrashedRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "char_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 SKRPaperItem::Roles::
                                                                 CharCountRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "word_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 SKRPaperItem::Roles::
                                                                 WordCountRole);
    }, Qt::UniqueConnection);


    m_dataConnectionsList << this->connect(plmdata->projectHub(),
                                           &PLMProjectHub::activeProjectChanged, this,
                                           [this](int projectId) {
        Q_UNUSED(projectId)

        for (int projectId : plmdata->projectHub()->getProjectIdList()) {
            this->exploitSignalFromPLMData(projectId, -1,
                                           SKRPaperItem::Roles::ProjectIsActiveRole);
        }
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "is_renamable") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                   SKRPaperItem::Roles::
                                                                   IsRenamableRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "is_movable") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 SKRPaperItem::Roles::
                                                                 IsMovableRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "can_add_paper") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                    SKRPaperItem::Roles::
                                                                    CanAddPaperRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "is_trashable") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                   SKRPaperItem::Roles::
                                                                   IsTrashableRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "is_openable") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                  SKRPaperItem::Roles::
                                                                  IsOpenableRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "is_copyable") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                  SKRPaperItem::Roles::
                                                                  IsCopyableRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                                  int            paperCode,
                                                  const QString& name,
                                                  const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "attributes") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 SKRPaperItem::Roles::
                                                                 AttributesRole);
    }, Qt::UniqueConnection);
}

void SKRPaperListModel::disconnectFromPLMDataSignals()
{
    // disconnect from PLMPaperHub signals :
    for (const QMetaObject::Connection& connection : m_dataConnectionsList) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}

// -----------------------------------------------------------------------------------


QModelIndexList SKRPaperListModel::getModelIndex(int projectId, int paperId)
{
    QModelIndexList list;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             SKRPaperItem::Roles::ProjectIdRole,
                                             projectId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap |
                                             Qt::MatchFlag::MatchRecursive);

    for (const QModelIndex& modelIndex : modelList) {
        int indexPaperId = modelIndex.data(SKRPaperItem::Roles::PaperIdRole).toInt();

        if (indexPaperId == paperId) {
            list.append(modelIndex);
        }
    }

    return list;
}

// -----------------------------------------------------------------------------------

SKRPaperItem * SKRPaperListModel::getParentPaperItem(SKRPaperItem *childItem)
{
    return childItem->parent(m_allPaperItems);
}

// -----------------------------------------------------------------------------------
SKRPaperItem * SKRPaperListModel::getItem(int projectId, int paperId)
{
    SKRPaperItem *result_item = nullptr;

    for (SKRPaperItem *item : m_allPaperItems) {
        if ((item->projectId() == projectId) && (item->paperId() == paperId)) {
            result_item = item;
            break;
        }
    }

    if (!result_item) {
        qDebug() << "result_item is null";
    }

    return result_item;
}

// -----------------------------------------------------------------------------------
