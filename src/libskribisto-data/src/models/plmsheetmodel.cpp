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
#include "plmsheetmodel.h"
#include "plmdata.h"
#include "plmpropertyhub.h"


#include <QDebug>

PLMSheetModel::PLMSheetModel(QObject *parent)
    : QAbstractItemModel(parent), m_headerData(QVariant())
{
    m_rootItem = new PLMSheetItem();
    m_rootItem->setIsRootItem();

    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &PLMSheetModel::populate);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &PLMSheetModel::populate);


//    connect(plmdata->sheetHub(),
//            &PLMSheetHub::paperAdded,
//            this,
//            &PLMSheetModel::addPaper);

    this->connectToPLMDataSignals();
}

QVariant PLMSheetModel::headerData(int section, Qt::Orientation orientation,
                                   int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return m_headerData;
}

bool PLMSheetModel::setHeaderData(int             section,
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

QModelIndex PLMSheetModel::index(int row, int column, const QModelIndex& parent) const
{
    if (!hasIndex(row, column, parent)) return QModelIndex();

    // if (column != 0) return QModelIndex();

    PLMSheetItem *parentItem;


    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<PLMSheetItem *>(parent.internalPointer());

    PLMSheetItem *childItem = parentItem->child(m_allSheetItems, row);

    if (childItem) {
        QModelIndex index = createIndex(row, column, childItem);
        return index;
    } else {
        return QModelIndex();
    }
}

QModelIndex PLMSheetModel::parent(const QModelIndex& index) const
{
    if (!index.isValid()) return QModelIndex();


    PLMSheetItem *childItem  = static_cast<PLMSheetItem *>(index.internalPointer());
    PLMSheetItem *parentItem = childItem->parent(m_allSheetItems);

    if (parentItem == nullptr) {
        return QModelIndex();
    }

    // if (parentItem->isRootItem()) r

    QModelIndex parentIndex =
            createIndex(parentItem->row(m_allSheetItems), 0, parentItem);
    return parentIndex;
}

int PLMSheetModel::rowCount(const QModelIndex& parent) const
{
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) {
        return m_rootItem->childrenCount(m_allSheetItems);
    }


    PLMSheetItem *parentItem = static_cast<PLMSheetItem *>(parent.internalPointer());


    return parentItem->childrenCount(m_allSheetItems);
}

int PLMSheetModel::columnCount(const QModelIndex& parent) const
{
    if (!parent.isValid()) return 1;

    return 1;
}

QVariant PLMSheetModel::data(const QModelIndex& index, int role) const
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

    PLMSheetItem *item = static_cast<PLMSheetItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem()) {
        return item->data(PLMSheetItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole) {
        return item->data(PLMSheetItem::Roles::NameRole);
    }


    if (role == PLMSheetItem::Roles::ProjectNameRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::NameRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::PaperIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::LabelRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::ContentDateRole) {
        return item->data(role);
    }

    if (role == PLMSheetItem::Roles::DeletedRole) {
        return item->data(role);
    }
    return QVariant();
}

bool PLMSheetModel::setData(const QModelIndex& index, const QVariant& value, int role)
{
    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid |
                        QAbstractItemModel::CheckIndexOption::DoNotUseParent));


    if (data(index, role) != value) {
        PLMSheetItem *item = static_cast<PLMSheetItem *>(index.internalPointer());
        int projectId      = item->projectId();
        int paperId        = item->paperId();
        PLMError error;

        this->disconnectFromPLMDataSignals();

        switch (role) {
        case PLMSheetItem::Roles::ProjectNameRole:
            error = plmdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case PLMSheetItem::Roles::ProjectIdRole:

            // useless
            break;

        case PLMSheetItem::Roles::PaperIdRole:

            // useless
            break;

        case PLMSheetItem::Roles::NameRole:
            error = plmdata->sheetHub()->setTitle(projectId, paperId, value.toString());
            break;

        case PLMSheetItem::Roles::LabelRole:
            error = plmdata->sheetPropertyHub()->setProperty(projectId, paperId,
                                                             "label", value.toString());
            break;

        case PLMSheetItem::Roles::IndentRole:
            error = plmdata->sheetHub()->setIndent(projectId, paperId, value.toInt());
            break;

        case PLMSheetItem::Roles::SortOrderRole:
            error = plmdata->sheetHub()->setSortOrder(projectId, paperId, value.toInt());
            break;

        case PLMSheetItem::Roles::DeletedRole:
            error = plmdata->sheetHub()->setDeleted(projectId, paperId, value.toBool());
            break;

        case PLMSheetItem::Roles::CreationDateRole:
            error = plmdata->sheetHub()->setCreationDate(projectId,
                                                         paperId,
                                                         value.toDateTime());
            break;

        case PLMSheetItem::Roles::UpdateDateRole:
            error = plmdata->sheetHub()->setUpdateDate(projectId,
                                                       paperId,
                                                       value.toDateTime());
            break;

        case PLMSheetItem::Roles::ContentDateRole:
            error = plmdata->sheetHub()->setContentDate(projectId,
                                                        paperId,
                                                        value.toDateTime());
            break;

        case PLMSheetItem::Roles::CharCountRole:
            error = plmdata->sheetPropertyHub()->setProperty(projectId,
                                                             paperId,
                                                             "char_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;

        case PLMSheetItem::Roles::WordCountRole:

            error = plmdata->sheetPropertyHub()->setProperty(projectId,
                                                             paperId,
                                                             "word_count",
                                                             QString::number(
                                                                 value.toInt()));
            break;
        }


        this->connectToPLMDataSignals();

        if (!error.isSuccess()) {
            return false;
        }
        item->invalidateData(role);

        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags PLMSheetModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags; // FIXME: Implement me!
}

bool PLMSheetModel::insertRows(int row, int count, const QModelIndex& parent)
{
    beginInsertRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endInsertRows();
    return false;
}

bool PLMSheetModel::insertColumns(int column, int count, const QModelIndex& parent)
{
    beginInsertColumns(parent, column, column + count - 1);

    // FIXME: Implement me!
    endInsertColumns();

    return false;
}

bool PLMSheetModel::removeRows(int row, int count, const QModelIndex& parent)
{
    beginRemoveRows(parent, row, row + count - 1);

    // FIXME: Implement me!
    endRemoveRows();
    return false;
}

bool PLMSheetModel::removeColumns(int column, int count, const QModelIndex& parent)
{
    beginRemoveColumns(parent, column, column + count - 1);

    // FIXME: Implement me!
    endRemoveColumns();
    return false;
}

QHash<int, QByteArray>PLMSheetModel::roleNames() const {
    QHash<int, QByteArray> roles;
    roles[PLMSheetItem::Roles::ProjectNameRole]  = "projectName";
    roles[PLMSheetItem::Roles::PaperIdRole]      = "paperId";
    roles[PLMSheetItem::Roles::ProjectIdRole]    = "projectId";
    roles[PLMSheetItem::Roles::NameRole]         = "name";
    roles[PLMSheetItem::Roles::LabelRole]          = "label";
    roles[PLMSheetItem::Roles::IndentRole]       = "indent";
    roles[PLMSheetItem::Roles::SortOrderRole]    = "sortOrder";
    roles[PLMSheetItem::Roles::CreationDateRole] = "creationDate";
    roles[PLMSheetItem::Roles::UpdateDateRole]   = "updateDate";
    roles[PLMSheetItem::Roles::ContentDateRole]  = "contentDate";
    roles[PLMSheetItem::Roles::DeletedRole]  = "deleted";
    return roles;
}

void PLMSheetModel::populate()
{
    this->beginResetModel();

    m_allSheetItems.clear();
    foreach(int projectId, plmdata->projectHub()->getProjectIdList()) {
        if (plmdata->projectHub()->getProjectIdList().count() > 1) {
            PLMSheetItem *projectItem = new PLMSheetItem();
            projectItem->setIsProjectItem(projectId);
            m_allSheetItems.append(projectItem);
        }

        auto idList         = plmdata->sheetHub()->getAllIds(projectId);
        auto sortOrdersHash = plmdata->sheetHub()->getAllSortOrders(projectId);
        auto indentsHash    = plmdata->sheetHub()->getAllIndents(projectId);

        foreach(int sheetId, idList) {
            m_allSheetItems.append(new PLMSheetItem(projectId, sheetId,
                                                    indentsHash.value(sheetId),
                                                    sortOrdersHash.value(sheetId)));
        }
    }
    this->endResetModel();
}

void PLMSheetModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_allSheetItems);
    this->endResetModel();
}

void PLMSheetModel::exploitSignalFromPLMData(int                 projectId,
                                             int                 paperId,
                                             PLMSheetItem::Roles role)
{
    PLMSheetItem *item = this->findPaperItem(projectId, paperId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        PLMSheetItem::Roles::PaperIdRole,
                                        paperId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : list) {
        PLMSheetItem *t = static_cast<PLMSheetItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid()) {
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

//--------------------------------------------------------------------

void PLMSheetModel::addPaper(int projectId, int paperId)
{
    //find parentIndex and row
    QModelIndex parentIndex;
    int row = 0;

    auto idList         = plmdata->sheetHub()->getAllIds(projectId);
    auto sortOrdersHash = plmdata->sheetHub()->getAllSortOrders(projectId);
    auto indentsHash    = plmdata->sheetHub()->getAllIndents(projectId);


    int paperIndex = idList.indexOf(paperId);
    int paperIndent = indentsHash.value(paperId);
    int paperSortOrders = sortOrdersHash.value(paperId);

    bool parentFound = false;
    if (plmdata->projectHub()->getProjectIdList().count() > 1) {

        if(paperIndex == 0){
            parentIndex = this->getModelIndex(projectId, -1).first();
            row = 0;
            parentFound = true;
        }

    }
    else if(paperIndex == 0){
        parentIndex = QModelIndex();
        row = 0;
        parentFound = true;

    }
    if(!parentFound){
        for (int i = paperIndex; i >= 0 ; --i ) {
            int possibleParentId = idList.at(i);
            int possibleParentIndent = indentsHash.value(possibleParentId);
            if(paperIndent == possibleParentIndent + 1){
                auto modelIndexList = this->getModelIndex(projectId, possibleParentId);
                if(modelIndexList.isEmpty()){
                    qWarning() << Q_FUNC_INFO << "if paperIndent == possibleParentIndent => modelIndexList.isEmpty()";
                    return;
                }
                parentIndex = modelIndexList.first();
                //int parentPaperId = parentIndex.data(PLMSheetItem::Roles::PaperIdRole).toInt();
                row = paperIndex - i - 1;
                parentFound = true;
                break;
            }
        }
    }
    if(!parentFound){
        qWarning() << Q_FUNC_INFO << "parent not found, failsafe used";
        this->populate();
        return;
    }

    // find item just before in m_allSheetItems to determine item index to insert in:

    int itemIndex = 0;
    if (plmdata->projectHub()->getProjectIdList().count() == 1 && paperIndex == 0){ // so no project items and first item
        itemIndex = 0;
    }
    else {
        if(paperIndex == 0){
            paperIndex = 1;
        }

        int idBefore = idList.at(paperIndex - 1);
        PLMSheetItem *itemBefore = this->findPaperItem(projectId, idBefore);

        int indexBefore = m_allSheetItems.indexOf(itemBefore);

        itemIndex = indexBefore + 1;

//        if(itemIndex >= m_allSheetItems.count() && paperIndent == itemBefore->indent() + 1){
//            qWarning() << Q_FUNC_INFO << "last in the m_allSheetItems list and child of previous item, so failsafe used";
//            this->populate();
//            return;
//        }
    }




    beginInsertRows(parentIndex, row, row);

    m_allSheetItems.insert(itemIndex, new PLMSheetItem(projectId, paperId,
                                                       indentsHash.value(paperId),
                                                       sortOrdersHash.value(paperId)));
    endInsertRows();
}

//--------------------------------------------------------------------

PLMSheetItem * PLMSheetModel::findPaperItem(int projectId, int paperId)
{
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        PLMSheetItem::Roles::PaperIdRole,
                                        paperId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);
    PLMSheetItem *item = nullptr;

    for (const QModelIndex& modelIndex : list) {
        PLMSheetItem *t = static_cast<PLMSheetItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) item = t;
    }

    return item;
}

void PLMSheetModel::connectToPLMDataSignals()
{
    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::titleChanged, this,
                                           [this](int projectId, int paperId,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId, PLMSheetItem::Roles::NameRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetPropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            paperCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "label") this->exploitSignalFromPLMData(projectId, paperCode,
                                                          PLMSheetItem::Roles::LabelRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::paperIdChanged, this,
                                           [this](int projectId, int paperId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::PaperIdRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::indentChanged, this,
                                           [this](int projectId, int paperId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::IndentRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList.append(this->connect(plmdata->sheetHub(),
                                               &PLMSheetHub::sortOrderChanged, this,
                                               [this](int projectId, int paperId,
                                               int value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::SortOrderRole);
    }));
    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::contentDateChanged, this,
                                           [this](int projectId, int paperId,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::ContentDateRole);
    }, Qt::UniqueConnection);

    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::updateDateChanged, this,
                                           [this](int projectId, int paperId,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::UpdateDateRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->sheetHub(),
                                           &PLMSheetHub::deletedChanged, this,
                                           [this](int projectId, int paperId,
                                           bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromPLMData(projectId, paperId,
                                       PLMSheetItem::Roles::DeletedRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->sheetPropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            paperCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "char_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 PLMSheetItem::Roles::
                                                                 CharCountRole);
    }, Qt::UniqueConnection);
    m_dataConnectionsList << this->connect(plmdata->sheetPropertyHub(),
                                           &PLMPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            paperCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)

        if (name == "word_count") this->exploitSignalFromPLMData(projectId, paperCode,
                                                                 PLMSheetItem::Roles::
                                                                 WordCountRole);
    }, Qt::UniqueConnection);
}

void PLMSheetModel::disconnectFromPLMDataSignals()
{
    // disconnect from PLMPaperHub signals :
    for (const QMetaObject::Connection& connection : m_dataConnectionsList) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}
//-----------------------------------------------------------------------------------


QModelIndexList PLMSheetModel::getModelIndex(int projectId, int paperId)
{
    QModelIndexList list;
    QModelIndexList modelList =  this->match(this->index(0, 0,
                                                         QModelIndex()),
                                             PLMSheetItem::Roles::ProjectIdRole,
                                             projectId,
                                             -1,
                                             Qt::MatchFlag::MatchExactly |
                                             Qt::MatchFlag::MatchWrap | Qt::MatchFlag::MatchRecursive);

    for (const QModelIndex& modelIndex : modelList) {

        int indexPaperId = modelIndex.data(PLMSheetItem::Roles::PaperIdRole).toInt();
        if (indexPaperId == paperId) {
            list.append(modelIndex);
        }
    }

    return list;
}



// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
// ----------------------------------------------------------------------------------
