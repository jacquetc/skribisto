/***************************************************************************
*   Copyright (C) 2019 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmnotelistproxymodel.cpp
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
#include "plmnotelistproxymodel.h"
#include "plmmodels.h"

#include <QTimer>

PLMNoteListProxyModel::PLMNoteListProxyModel(QObject *parent) : QSortFilterProxyModel(
        parent),
    m_showTrashedFilter(false), m_projectIdFilter(-2), m_parentIdFilter(-2)
{
    this->setSourceModel(plmmodels->noteListModel());
    this->setShowTrashedFilter(false);


    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectLoaded,
                                                                  this,
                                                                        &PLMNoteListProxyModel::loadProjectSettings);
    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectToBeClosed,
                                                                  this,
                                                                        &PLMNoteListProxyModel::saveProjectSettings,
        Qt::DirectConnection);
    connect(
        plmdata->projectHub(),
                                   &PLMProjectHub::projectClosed,
                                                                  this,
                                                                        &PLMNoteListProxyModel::clearHistory);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this]() {
        this->clearFilters();
    });
}

Qt::ItemFlags PLMNoteListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------

QVariant PLMNoteListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         PLMNoteItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool PLMNoteListProxyModel::setData(const QModelIndex& index, const QVariant& value,
                                    int role)
{
    QModelIndex sourceIndex = this->mapToSource(index);

    PLMNoteItem *item =
        static_cast<PLMNoteItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMNoteItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                PLMNoteItem::Roles::NameRole);
        }
    }
    return QSortFilterProxyModel::setData(index, value, role);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::setParentFilter(int projectId, int parentId)
{
    m_projectIdFilter = projectId;
    m_parentIdFilter  = parentId;
    emit parentIdFilterChanged(m_parentIdFilter);
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------


void PLMNoteListProxyModel::setShowTrashedFilter(bool showTrashed)
{
    m_showTrashedFilter = showTrashed;
    emit showTrashedFilterChanged(m_showTrashedFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------


bool PLMNoteListProxyModel::filterAcceptsRow(int                sourceRow,
                                             const QModelIndex& sourceParent) const
{
    bool result = false;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    PLMNoteItem *item       = static_cast<PLMNoteItem *>(index.internalPointer());
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    // displays or not project list :
    if ((m_parentIdFilter == -2) && item->isProjectItem()) {
        return true;
    }
    else if ((m_parentIdFilter != -2) && item->isProjectItem()) {
        return false;
    }

    // path / parent filtering :
    int indexProject = item->data(PLMNoteItem::Roles::ProjectIdRole).toInt();

    if (indexProject != m_projectIdFilter) {
        return false;
    }
    PLMNoteItem *parentItem = model->getParentNoteItem(item);

    if (parentItem) {
        if (parentItem->paperId() == m_parentIdFilter) {
            result = true;
        }
    }

    // trashed filtering :

    if (result &&
        (item->data(PLMNoteItem::Roles::TrashedRole).toBool() == m_showTrashedFilter)) {
        QString string = item->data(PLMNoteItem::Roles::NameRole).toString();

        // qDebug() << "trashed : " << string;
        result = true;
    }
    else {
        result = false;
    }

    // QString string = item->data(PLMNoteItem::Roles::NameRole).toString();
    // qDebug() << "result : " << string << result;


    return result;
}

void PLMNoteListProxyModel::setParentIdFilter(int parentIdFilter)
{
    m_parentIdFilter = parentIdFilter;
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

void PLMNoteListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::clearFilters()
{
    m_parentIdFilter  = -2;
    m_projectIdFilter = -2;
    emit projectIdFilterChanged(m_projectIdFilter);
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------


///
/// \brief PLMNoteListProxyModel::moveItem
/// \param from source item index number
/// \param to target item index number
/// Carefull, this is only used for manually moving a visual item
void PLMNoteListProxyModel::moveItem(int from, int to) {
    if (from == to) return;

    int modelFrom = from;
    int modelTo   = to + (from < to ? 1 : 0);


    QModelIndex fromIndex = this->index(modelFrom, 0);
    int fromPaperId       =
        this->data(fromIndex, PLMNoteItem::Roles::PaperIdRole).toInt();
    int fromProjectId =
        this->data(fromIndex, PLMNoteItem::Roles::ProjectIdRole).toInt();

    QModelIndex toIndex = this->index(modelTo, 0);
    int toPaperId       = this->data(toIndex, PLMNoteItem::Roles::PaperIdRole).toInt();
    int toProjectId     = this->data(toIndex, PLMNoteItem::Roles::ProjectIdRole).toInt();
    int toSortOrder     = this->data(toIndex, PLMNoteItem::Roles::SortOrderRole).toInt();

    qDebug() << "fromPaperId : " << fromPaperId << this->data(fromIndex,
                                                              PLMNoteItem::Roles::NameRole)
    .toString();
    qDebug() << "toPaperId : " << toPaperId << this->data(toIndex,
                                                          PLMNoteItem::Roles::NameRole).
    toString();

    int finalSortOrder = toSortOrder - 1;


    // append to end of list if moved here, paperId = 0

    if (toPaperId == 0) {
        PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

        QModelIndex modelIndex =
            model->getModelIndex(fromProjectId, fromPaperId).first();
        PLMNoteItem *item =
            static_cast<PLMNoteItem *>(modelIndex.internalPointer());

        int indent                 = item->indent();
        QList<int> idList          = plmdata->noteHub()->getAllIds(fromProjectId);
        QHash<int, int> indentHash = plmdata->noteHub()->getAllIndents(fromProjectId);
        int lastIdWithSameIndent   = fromPaperId;

        for (int id : idList) {
            if (indentHash.value(id) == indent) {
                lastIdWithSameIndent = id;
            }
        }

        if (lastIdWithSameIndent == fromPaperId) {
            return;
        }

        finalSortOrder = plmdata->noteHub()->getValidSortOrderAfterPaper(fromProjectId,
                                                                         lastIdWithSameIndent);
    }
    qDebug() << "finalSortOrder" << finalSortOrder;


    // beginMoveRows(QModelIndex(), modelFrom, modelFrom, QModelIndex(),
    // modelTo);
    // PLMError error = plmdata->noteHub()->movePaper(fromProjectId,
    // fromPaperId, toPaperId);
    this->setData(fromIndex, finalSortOrder, PLMNoteItem::Roles::SortOrderRole);

    // plmdata->noteHub()->setSortOrder(fromPaperId, fromPaperId, toSortOrder -
    // 1);

    // endMoveRows();
}

// --------------------------------------------------------------

int PLMNoteListProxyModel::goUp()
{
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
    PLMNoteItem *parentItem = getItem(m_projectIdFilter, m_parentIdFilter);

    if (!parentItem) {
        return -2;
    }
    PLMNoteItem *grandParentItem = model->getParentNoteItem(parentItem);

    int grandParentId = -2;

    if (grandParentItem) {
        grandParentId = grandParentItem->paperId();
    }
    this->setParentFilter(m_projectIdFilter, grandParentId);

    return grandParentId;
}

// --------------------------------------------------------------

PLMNoteItem * PLMNoteListProxyModel::getItem(int projectId, int paperId)
{
    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());

    //    QModelIndexList modelIndexList = model->getModelIndex(projectId,
    // paperId);
    //    if(modelIndexList.isEmpty()){
    //        return nullptr;
    //    }
    //    QModelIndex modelIndex = modelIndexList.first();

    //    PLMNoteItem *item = static_cast<PLMNoteItem
    // *>(modelIndex.internalPointer());

    PLMNoteItem *item = model->getItem(projectId, paperId);

    return item;
}

// --------------------------------------------------------------


void PLMNoteListProxyModel::loadProjectSettings(int projectId)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    int writeCurrentParent = settings.value("noteCurrentParent",
    // 0).toInt();
    //    this->setParentFilter(projectId, noteCurrentParent);
    settings.endGroup();
}

// --------------------------------------------------------------


void PLMNoteListProxyModel::saveProjectSettings(int projectId)
{
    if (m_projectIdFilter != projectId) {
        return;
    }

    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    settings.setValue("noteCurrentParent", m_parentIdFilter);
    settings.endGroup();
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::setForcedCurrentIndex(int projectId, int paperId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, paperId);

    setForcedCurrentIndex(forcedCurrentIndex);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::setCurrentPaperId(int projectId, int paperId)
{
    if (projectId == -2) {
        return;
    }

    if ((paperId == -2) && (projectId != -2)) {
        this->setParentFilter(projectId, -1);
        return;
    }


    PLMNoteListModel *model = static_cast<PLMNoteListModel *>(this->sourceModel());
    PLMNoteItem *item       = this->getItem(projectId, paperId);

    if (!item) {
        paperId = plmdata->noteHub()->getTopPaperId(projectId);
        item    = this->getItem(projectId, paperId);
    }


    PLMNoteItem *parentItem = model->getParentNoteItem(item);

    if (parentItem) {
        this->setParentFilter(projectId, parentItem->paperId());
    }
    else {
        this->setParentFilter(projectId, -2); // root item
    }
    this->setForcedCurrentIndex(projectId, paperId);
}

// --------------------------------------------------------------

bool PLMNoteListProxyModel::hasChildren(int projectId, int paperId)
{
    return plmdata->noteHub()->hasChildren(projectId, paperId);
}

// --------------------------------------------------------------

int PLMNoteListProxyModel::findVisualIndex(int projectId, int paperId)
{
    int rowCount = this->rowCount(QModelIndex());

    int result = -2;

    QModelIndex modelIndex;

    for (int i = 0; i < rowCount; ++i) {
        modelIndex = this->index(i, 0);

        if ((this->data(modelIndex,
                        PLMNoteItem::Roles::ProjectIdRole).toInt() == projectId)
            && (this->data(modelIndex,
                           PLMNoteItem::Roles::PaperIdRole).toInt() == paperId)) {
            result = i;
            break;
        }
    }
    return result;
}

int PLMNoteListProxyModel::getLastOfHistory(int projectId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    if (list.isEmpty()) {
        return -2;
    }

    return list.last();
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::removeLastOfHistory(int projectId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    if (list.isEmpty()) {
        return;
    }

    list.removeLast();
    m_historyList.insert(projectId, list);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::clearHistory(int projectId)
{
    m_historyList.remove(projectId);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::addHistory(int projectId, int paperId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    list.append(paperId);
    m_historyList.insert(projectId, list);
}

// --------------------------------------------------------------
QString PLMNoteListProxyModel::getItemName(int projectId, int paperId)
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
        PLMNoteItem *item = this->getItem(projectId, paperId);

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
int PLMNoteListProxyModel::getItemIndent(int projectId, int paperId)
{
    if ((projectId == -2) || (paperId == -2)) { // root item or error
        return -2;
    }

    if (paperId == -1) { // project item
        return -1;
    }

    int indent        = -2;
    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (item) {
        indent = item->indent();
    }


    return indent;
}

// -----------------------------------------------------------------

void PLMNoteListProxyModel::addItemAtEnd(int projectId, int parentPaperId,
                                         int visualIndex)
{
    //    PLMNoteItem *parentItem = this->getItem(projectId, parentPaperId);
    //    if(!parentItem){
    //        if(plmdata->projectHub()->getProjectCount() <= 1){

    //        }
    //        else if (plmdata->projectHub()->getProjectCount() > 1){

    //        }
    //        }

    //    finalSortOrder =
    // plmdata->noteHub()->getValidSortOrderAfterPaper(projectId,
    // lastIdWithSameIndent);

    PLMError error = plmdata->noteHub()->addChildPaper(projectId, parentPaperId);

    this->invalidateFilter();
    this->setForcedCurrentIndex(visualIndex);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::moveUp(int projectId, int paperId, int visualIndex)
{
    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }
    PLMError error = plmdata->noteHub()->movePaperUp(projectId, paperId);

    this->setForcedCurrentIndex(visualIndex - 1);
}

// --------------------------------------------------------------

void PLMNoteListProxyModel::moveDown(int projectId, int paperId, int visualIndex)
{
    //    PLMError error = plmdata->noteHub()->addChildPaper(projectId,
    // paperId);

    PLMNoteItem *item = this->getItem(projectId, paperId);

    if (!item) {
        return;
    }
    PLMError error = plmdata->noteHub()->movePaperDown(projectId, paperId);

    this->setForcedCurrentIndex(visualIndex + 1);
}
