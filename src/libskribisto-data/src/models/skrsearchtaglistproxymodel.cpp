/***************************************************************************
*   Copyright (C) 2020 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: skrsearchtaglistproxymodel.cpp
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
#include "skrsearchtaglistproxymodel.h"
#include "plmmodels.h"
#include "skrtaghub.h"

#include <QTimer>

SKRSearchTagListProxyModel::SKRSearchTagListProxyModel(QObject *parent) :
    QSortFilterProxyModel(parent),
    m_textFilter(""), m_projectIdFilter(-2), m_sheetIdFilter(-2), m_noteIdFilter(-2)
{
    this->setSourceModel(plmmodels->tagListModel());


    connect(
        plmdata->projectHub(),
        &PLMProjectHub::projectLoaded,
        this,
        &SKRSearchTagListProxyModel::loadProjectSettings);
    connect(
        plmdata->projectHub(),
        &PLMProjectHub::projectToBeClosed,
        this,
        &SKRSearchTagListProxyModel::saveProjectSettings,
        Qt::DirectConnection);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this]() {
        this->invalidateFilter();
    });


    connect(plmdata->tagHub(), &SKRTagHub::tagRelationshipAdded, this, [this]() {
        this->populateRelationshipList();
        this->invalidateFilter();
    });


    connect(plmdata->tagHub(), &SKRTagHub::tagRelationshipRemoved, this, [this]() {
        this->populateRelationshipList();
        this->invalidateFilter();
    });

    connect(plmdata->tagHub(), &SKRTagHub::tagRelationshipChanged, this, [this]() {
        this->populateRelationshipList();
        this->invalidateFilter();
    });
}

// --------------------------------------------------------------------------------


Qt::ItemFlags SKRSearchTagListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// --------------------------------------------------------------------------------


QVariant SKRSearchTagListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         SKRTagItem::Roles::NameRole).toString();
    }

    return QSortFilterProxyModel::data(index, role);
}

// --------------------------------------------------------------------------------


bool SKRSearchTagListProxyModel::setData(const QModelIndex& index,
                                         const QVariant   & value,
                                         int                role)
{
    QModelIndex sourceIndex = this->mapToSource(index);

    SKRTagItem *item =
        static_cast<SKRTagItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        return this->sourceModel()->setData(sourceIndex,
                                            value,
                                            SKRTagItem::Roles::NameRole);
    }

    return QSortFilterProxyModel::setData(index, value, role);
}

// --------------------------------------------------------------------------------


QString SKRSearchTagListProxyModel::getItemName(int projectId, int paperId)
{
    // TODO: to fill
}

// --------------------------------------------------------------------------------

void SKRSearchTagListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::clearFilters()
{
    m_projectIdFilter = -2;
    emit projectIdFilterChanged(m_projectIdFilter);

    m_sheetIdFilter = -2;
    emit sheetIdFilterChanged(m_sheetIdFilter);

    m_noteIdFilter = -2;
    emit noteIdFilterChanged(m_noteIdFilter);

    m_textFilter = "";
    emit textFilterChanged(m_textFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::setForcedCurrentIndex(int projectId, int paperId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, paperId);

    setForcedCurrentIndex(forcedCurrentIndex);
}

// --------------------------------------------------------------------------------


int SKRSearchTagListProxyModel::findVisualIndex(int projectId, int paperId)
{
    int rowCount = this->rowCount(QModelIndex());

    int visualIndex = -2;

    QModelIndex modelIndex;

    for (int i = 0; i < rowCount; ++i) {
        modelIndex = this->index(i, 0);

        if ((this->data(modelIndex,
                        SKRTagItem::Roles::ProjectIdRole).toInt() == projectId)
            && (this->data(modelIndex,
                           SKRTagItem::Roles::TagIdRole).toInt() == paperId)) {
            visualIndex = i;
            break;
        }
    }
    return visualIndex;
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::setCurrentPaperId(int projectId, int tagId)
{
    if (projectId == -2) {
        return;
    }

    if ((tagId == -2) && (projectId != -2)) {
        return;
    }


    SKRTagListModel *model = static_cast<SKRTagListModel *>(this->sourceModel());
    SKRTagItem *item       = this->getItem(projectId, tagId);

    if (!item) {
        tagId = plmdata->tagHub()->getTopPaperId(projectId);
    }

    this->setForcedCurrentIndex(projectId, tagId);
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::setTextFilter(const QString& value)
{
    m_textFilter = value;
    emit textFilterChanged(value);

    this->invalidateFilter();
}

// --------------------------------------------------------------------------------


bool SKRSearchTagListProxyModel::filterAcceptsRow(int                sourceRow,
                                                  const QModelIndex& sourceParent) const
{
    bool value = true;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    SKRTagItem *item       = static_cast<SKRTagItem *>(index.internalPointer());
    SKRTagListModel *model = static_cast<SKRTagListModel *>(this->sourceModel());

    // project filtering :
    if (value &&
        (item->data(SKRTagItem::Roles::ProjectIdRole).toInt() == m_projectIdFilter)) {
        value = true;
    }
    else if (value) {
        value = false;
    }

    // sheet or note filtering:
    if (m_sheetIdFilter != -2 ^ m_noteIdFilter != -2) {
        if (value &&
            m_relationshipList.contains(item->data(SKRTagItem::Roles::TagIdRole).toInt()))
        {
            value = true;
        }
        else if (value) {
            value = false;
        }
    }


    // text filtering :

    if (value &&
        item->data(SKRTagItem::Roles::NameRole).toString().contains(m_textFilter,
                                                                    Qt::CaseInsensitive))
    {
        value = true;
    }
    else if (value) {
        value = false;
    }

    // hide tag ids

    if(value && !m_hideTagIdListFilter.isEmpty()){

        if(m_hideTagIdListFilter.contains(item->tagId())){
            value = false;
        }
    }

    return value;
}

// --------------------------------------------------------------------------------


SKRTagItem * SKRSearchTagListProxyModel::getItem(int projectId, int tagId)
{
    SKRTagListModel *model = static_cast<SKRTagListModel *>(this->sourceModel());

    SKRTagItem *item = model->getItem(projectId, tagId);

    return item;
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::loadProjectSettings(int projectId)
{
    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    int writeCurrentParent = settings.value("sheetCurrentParent",
    // 0).toInt();
    //    this->setParentFilter(projectId, sheetCurrentParent);
    settings.endGroup();
}

// --------------------------------------------------------------------------------

void SKRSearchTagListProxyModel::saveProjectSettings(int projectId)
{
    if (m_projectIdFilter != projectId) {
        return;
    }

    QString   unique_identifier = plmdata->projectHub()->getProjectUniqueId(projectId);
    QSettings settings;

    settings.beginGroup("project_" + unique_identifier);

    //    settings.setValue("sheetCurrentParent", m_parentIdFilter);
    settings.endGroup();
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::populateRelationshipList()
{
    m_relationshipList.clear();

    if (m_sheetIdFilter != -2) {
        m_relationshipList.append(plmdata->tagHub()->getTagsFromItemId(m_projectIdFilter,
                                                                       SKR::ItemType
                                                                       ::Sheet,
                                                                       m_sheetIdFilter));
    }

    if (m_noteIdFilter != -2) {
        m_relationshipList.append(plmdata->tagHub()->getTagsFromItemId(m_projectIdFilter,
                                                                       SKR::ItemType
                                                                       ::Note,
                                                                       m_noteIdFilter));
    }
}

void SKRSearchTagListProxyModel::setNoteIdFilter(int noteIdFilter)
{
    m_noteIdFilter = noteIdFilter;
    this->populateRelationshipList();

    this->invalidateFilter();
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::setHideTagIdListFilter(const QList<int> &hideTagIdListFilter)
{
    m_hideTagIdListFilter = hideTagIdListFilter;
    emit hideTagIdListFilterChanged(hideTagIdListFilter);
    this->invalidateFilter();
}

// --------------------------------------------------------------------------------


void SKRSearchTagListProxyModel::setSheetIdFilter(int sheetIdFilter)
{
    m_sheetIdFilter = sheetIdFilter;
    this->populateRelationshipList();
    emit sheetIdFilterChanged(sheetIdFilter);

    this->invalidateFilter();
}
