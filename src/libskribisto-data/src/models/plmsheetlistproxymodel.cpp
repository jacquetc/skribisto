/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmsheetlistproxymodel.cpp
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
#include "plmsheetlistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

PLMSheetListProxyModel::PLMSheetListProxyModel(QObject *parent) : QSortFilterProxyModel(
        parent),
    m_showTrashedFilter(false), m_projectIdFilter(-2), m_parentIdFilter(-2)
{
    this->setSourceModel(plmmodels->sheetListModel());
    this->setShowTrashedFilter(false);


    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectLoaded,
                                                                  this,
                                                                        &PLMSheetListProxyModel::loadProjectSettings);
    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectToBeClosed,
                                                                  this,
                                                                        &PLMSheetListProxyModel::saveProjectSettings,
        Qt::DirectConnection);
    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectClosed,
                                                                  this,
                                                                        &PLMSheetListProxyModel::clearHistory);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this]() {
        this->clearFilters();
    });

//    connect(this->sourceModel(), &QAbstractItemModel::rowsInserted, this, [this](const QModelIndex &parent, int first, int last) {
//        this->invalidateFilter();
//    });

}

Qt::ItemFlags PLMSheetListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------

QVariant PLMSheetListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMSheetItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool PLMSheetListProxyModel::setData(const QModelIndex& index, const QVariant& value,
                                     int role)
{
    QModelIndex sourceIndex = this->mapToSource(index);

    PLMSheetItem *item =
        static_cast<PLMSheetItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMSheetItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMSheetItem::Roles::NameRole);
        }
    }
    return QSortFilterProxyModel::setData(index, value, role);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::setParentFilter(int projectId, int parentId)
{
    m_projectIdFilter = projectId;
    m_parentIdFilter  = parentId;
    emit parentIdFilterChanged(m_parentIdFilter);
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------


void PLMSheetListProxyModel::setShowTrashedFilter(bool showTrashed)
{
    m_showTrashedFilter = showTrashed;
    emit showTrashedFilterChanged(m_showTrashedFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------


bool PLMSheetListProxyModel::filterAcceptsRow(int                sourceRow,
                                              const QModelIndex& sourceParent) const
{
    bool result = false;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    PLMSheetItem *item       = static_cast<PLMSheetItem *>(index.internalPointer());
    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());

    // displays or not project list :
    if ((m_parentIdFilter == -2) && item->isProjectItem()) {
        return true;
    }
    else if ((m_parentIdFilter != -2) && item->isProjectItem()) {
        return false;
    }

    // path / parent filtering :
    int indexProject = item->data(PLMSheetItem::Roles::ProjectIdRole).toInt();

    if (indexProject != m_projectIdFilter) {
        return false;
    }
    PLMSheetItem *parentItem = model->getParentSheetItem(item);

    if (parentItem) {
        if (parentItem->paperId() == m_parentIdFilter) {
            result = true;
        }
    }

    // trashed filtering :

    if (result &&
        (item->data(PLMSheetItem::Roles::TrashedRole).toBool() == m_showTrashedFilter)) {
        QString string = item->data(PLMSheetItem::Roles::NameRole).toString();

        // qDebug() << "trashed : " << string;
        result = true;
    }
    else {
        result = false;
    }

    // QString string = item->data(PLMSheetItem::Roles::NameRole).toString();
    // qDebug() << "result : " << string << result;


    return result;
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::setParentIdFilter(int parentIdFilter)
{
    m_parentIdFilter = parentIdFilter;
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::clearFilters()
{
    m_parentIdFilter  = -2;
    m_projectIdFilter = -2;
    emit projectIdFilterChanged(m_projectIdFilter);
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

///
/// \brief PLMSheetListProxyModel::moveItem
/// \param from source item index number
/// \param to target item index number
/// Carefull, this is only used for manually moving a visual item
void PLMSheetListProxyModel::moveItem(int from, int to) {
    if (from == to) return;

    int modelFrom = from;
    int modelTo   = to + (from < to ? 1 : 0);


    QModelIndex fromIndex = this->index(modelFrom, 0);
    int fromPaperId       =
        this->data(fromIndex, PLMSheetItem::Roles::PaperIdRole).toInt();
    int fromProjectId =
        this->data(fromIndex, PLMSheetItem::Roles::ProjectIdRole).toInt();

    QModelIndex toIndex = this->index(modelTo, 0);
    int toPaperId       = this->data(toIndex, PLMSheetItem::Roles::PaperIdRole).toInt();
    int toProjectId     = this->data(toIndex, PLMSheetItem::Roles::ProjectIdRole).toInt();
    int toSortOrder     = this->data(toIndex, PLMSheetItem::Roles::SortOrderRole).toInt();

    qDebug() << "fromPaperId : " << fromPaperId << this->data(fromIndex,
                                                              PLMSheetItem::Roles::NameRole)
    .toString();
    qDebug() << "toPaperId : " << toPaperId << this->data(toIndex,
                                                          PLMSheetItem::Roles::NameRole).
    toString();

    int finalSortOrder = toSortOrder - 1;


    // append to end of list if moved here, paperId = 0

    if (toPaperId == 0) {
        PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());

        QModelIndex modelIndex =
            model->getModelIndex(fromProjectId, fromPaperId).first();
        PLMSheetItem *item =
            static_cast<PLMSheetItem *>(modelIndex.internalPointer());

        int indent                 = item->indent();
        QList<int> idList          = plmdata->sheetHub()->getAllIds(fromProjectId);
        QHash<int, int> indentHash = plmdata->sheetHub()->getAllIndents(fromProjectId);
        int lastIdWithSameIndent   = fromPaperId;

        for (int id : idList) {
            if (indentHash.value(id) == indent) {
                lastIdWithSameIndent = id;
            }
        }

        if (lastIdWithSameIndent == fromPaperId) {
            return;
        }

        finalSortOrder = plmdata->sheetHub()->getValidSortOrderAfterPaper(fromProjectId,
                                                                          lastIdWithSameIndent);
    }
    qDebug() << "finalSortOrder" << finalSortOrder;


    // beginMoveRows(QModelIndex(), modelFrom, modelFrom, QModelIndex(),
    // modelTo);
    // PLMError error = plmdata->sheetHub()->movePaper(fromProjectId,
    // fromPaperId, toPaperId);
    this->setData(fromIndex, finalSortOrder, PLMSheetItem::Roles::SortOrderRole);

    // plmdata->sheetHub()->setSortOrder(fromPaperId, fromPaperId, toSortOrder -
    // 1);

    // endMoveRows();
}

// --------------------------------------------------------------

int PLMSheetListProxyModel::goUp()
{
    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());
    PLMSheetItem *parentItem = getItem(m_projectIdFilter, m_parentIdFilter);

    if (!parentItem) {
        return -2;
    }
    PLMSheetItem *grandParentItem = model->getParentSheetItem(parentItem);

    int grandParentId = -2;

    if (grandParentItem) {
        grandParentId = grandParentItem->paperId();
    }
    this->setParentFilter(m_projectIdFilter, grandParentId);

    return grandParentId;
}

// --------------------------------------------------------------

PLMSheetItem * PLMSheetListProxyModel::getItem(int projectId, int paperId)
{
    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());

    //    QModelIndexList modelIndexList = model->getModelIndex(projectId,
    // paperId);
    //    if(modelIndexList.isEmpty()){
    //        return nullptr;
    //    }
    //    QModelIndex modelIndex = modelIndexList.first();

    //    PLMSheetItem *item = static_cast<PLMSheetItem
    // *>(modelIndex.internalPointer());

    PLMSheetItem *item = model->getItem(projectId, paperId);

    return item;
}

// --------------------------------------------------------------


void PLMSheetListProxyModel::loadProjectSettings(int projectId)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    int writeCurrentParent = settings.value("writeCurrentParent",
    // 0).toInt();
    //    this->setParentFilter(projectId, writeCurrentParent);
    settings.endGroup();
}

// --------------------------------------------------------------


void PLMSheetListProxyModel::saveProjectSettings(int projectId)
{
    if (m_projectIdFilter != projectId) {
        return;
    }

    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    settings.setValue("writeCurrentParent", m_parentIdFilter);
    settings.endGroup();
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::setForcedCurrentIndex(int projectId, int paperId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, paperId);

    setForcedCurrentIndex(forcedCurrentIndex);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::setCurrentPaperId(int projectId, int paperId)
{
    if (projectId == -2) {
        return;
    }

    if ((paperId == -2) && (projectId != -2)) {
        this->setParentFilter(projectId, -1);
        return;
    }


    PLMSheetListModel *model = static_cast<PLMSheetListModel *>(this->sourceModel());
    PLMSheetItem *item       = this->getItem(projectId, paperId);

    if (!item) {
        paperId = plmdata->sheetHub()->getTopPaperId(projectId);
        item    = this->getItem(projectId, paperId);
    }


    PLMSheetItem *parentItem = model->getParentSheetItem(item);

    if (parentItem) {
        this->setParentFilter(projectId, parentItem->paperId());
    }
    else {
        this->setParentFilter(projectId, -2); // root item
    }
    this->setForcedCurrentIndex(projectId, paperId);
}

// --------------------------------------------------------------

bool PLMSheetListProxyModel::hasChildren(int projectId, int paperId)
{
    return plmdata->sheetHub()->hasChildren(projectId, paperId);
}

// --------------------------------------------------------------

int PLMSheetListProxyModel::findVisualIndex(int projectId, int paperId)
{
    int rowCount = this->rowCount(QModelIndex());

    int result = -2;

    QModelIndex modelIndex;

    for (int i = 0; i < rowCount; ++i) {
        modelIndex = this->index(i, 0);

        if ((this->data(modelIndex,
                        PLMSheetItem::Roles::ProjectIdRole).toInt() == projectId)
            && (this->data(modelIndex,
                           PLMSheetItem::Roles::PaperIdRole).toInt() == paperId)) {
            result = i;
            break;
        }
    }
    return result;
}

int PLMSheetListProxyModel::getLastOfHistory(int projectId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    if (list.isEmpty()) {
        return -2;
    }

    return list.last();
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::removeLastOfHistory(int projectId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    if (list.isEmpty()) {
        return;
    }

    list.removeLast();
    m_historyList.insert(projectId, list);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::clearHistory(int projectId)
{
    m_historyList.remove(projectId);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::addHistory(int projectId, int paperId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    list.append(paperId);
    m_historyList.insert(projectId, list);
}

// --------------------------------------------------------------

QString PLMSheetListProxyModel::getItemName(int projectId, int paperId)
{
    // qDebug() << "getItemName" << projectId << paperId;
    if ((projectId == -2) || (paperId == -2)) {
        return "";
    }
    QString name = "";

    if (paperId == -1) {
        name = plmdata->projectHub()->getProjectName(projectId);
    }
    else {
        PLMSheetItem *item = this->getItem(projectId, paperId);

        if (item) {
            name = item->name();
        }
        else {
            qDebug() << this->metaObject()->className() << "no valid item found";
            name = "";
        }
    }

    return name;
}

// --------------------------------------------------------------
int PLMSheetListProxyModel::getItemIndent(int projectId, int paperId)
{
    if ((projectId == -2) || (paperId == -2)) { // root item or error
        return -2;
    }

    if (paperId == -1) { // project item
        return -1;
    }

    int indent         = -2;
    PLMSheetItem *item = this->getItem(projectId, paperId);

    if (item) {
        indent = item->indent();
    }


    return indent;
}

// -----------------------------------------------------------------

void PLMSheetListProxyModel::addChildItem(int projectId,
                                          int parentPaperId,
                                          int visualIndex)
{
    PLMError error = plmdata->sheetHub()->addChildPaper(projectId, parentPaperId);

    //this->invalidateFilter();
    this->setForcedCurrentIndex(visualIndex);
}

// -----------------------------------------------------------------

void PLMSheetListProxyModel::addItemAbove(int projectId,
                                          int parentPaperId,
                                          int visualIndex)
{
    PLMError error = plmdata->sheetHub()->addPaperAbove(projectId, parentPaperId);

    //this->invalidateFilter();
    this->setForcedCurrentIndex(visualIndex);
}


// -----------------------------------------------------------------

void PLMSheetListProxyModel::addItemBelow(int projectId,
                                          int parentPaperId,
                                          int visualIndex)
{
    PLMError error = plmdata->sheetHub()->addPaperBelow(projectId, parentPaperId);

    //this->invalidateFilter();
    this->setForcedCurrentIndex(visualIndex);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::moveUp(int projectId, int paperId, int visualIndex)
{
    PLMSheetItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }
    PLMError error = plmdata->sheetHub()->movePaperUp(projectId, paperId);

    this->setForcedCurrentIndex(visualIndex - 1);
}

// --------------------------------------------------------------

void PLMSheetListProxyModel::moveDown(int projectId, int paperId, int visualIndex)
{
    //    PLMError error = plmdata->sheetHub()->addChildPaper(projectId,
    // paperId);

    PLMSheetItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }
    PLMError error = plmdata->sheetHub()->movePaperDown(projectId, paperId);

    this->setForcedCurrentIndex(visualIndex + 1);
}
