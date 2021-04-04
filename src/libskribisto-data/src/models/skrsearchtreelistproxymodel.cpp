#include "skrsearchtreelistproxymodel.h"
#include "skrmodels.h"
#include <QTimer>

SKRSearchTreeListProxyModel::SKRSearchTreeListProxyModel()
    :
    QSortFilterProxyModel(),
    m_showTrashedFilter(true), m_showNotTrashedFilter(true), m_navigateByBranchesEnabled(
        false), m_textFilter(""),
    m_projectIdFilter(-2), m_parentIdFilter(-2), m_showParentWhenParentIdFilter(false)
{
    this->setSourceModel(skrmodels->treeListModel());

    m_treeHub     = plmdata->treeHub();
    m_propertyHub = plmdata->treePropertyHub();


    this->setSortRole(SKRTreeItem::SortOrderRole);
    this->setDynamicSortFilter(false);


    connect(plmdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &SKRSearchTreeListProxyModel::loadProjectSettings);
    connect(plmdata->projectHub(),
            &PLMProjectHub::projectToBeClosed,
            this,
            &SKRSearchTreeListProxyModel::saveProjectSettings,
            Qt::DirectConnection);
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this,
            [this](int
                   projectId) {
        this->clearHistory(projectId);
    });
    connect(plmdata->projectHub(), &PLMProjectHub::projectClosed, this, [this]() {
        this->invalidateFilter();
    });


    // connect this proxy model to all other proxy models using the main model

    SKRTreeListModel *listModel = static_cast<SKRTreeListModel *>(this->sourceModel());

    connect(this,
                       &SKRSearchTreeListProxyModel::sortOtherProxyModelsCalled,
                                                                      listModel,
                                                                            &SKRTreeListModel::sortOtherProxyModelsCalled);
    connect(listModel, &SKRTreeListModel::sortOtherProxyModelsCalled, this, [this]() {
        if (this->sender() != this) {
            this->sort(0);
        }
    });
}

SKRSearchTreeListProxyModel * SKRSearchTreeListProxyModel::clone()
{
    SKRSearchTreeListProxyModel *newInstance = new SKRSearchTreeListProxyModel();

    newInstance->setProjectIdFilter(m_projectIdFilter);
    newInstance->setShowTrashedFilter(m_showTrashedFilter);
    newInstance->setShowNotTrashedFilter(m_showNotTrashedFilter);
    newInstance->setNavigateByBranchesEnabled(m_navigateByBranchesEnabled);
    newInstance->setTextFilter(m_textFilter);
    newInstance->setTreeItemIdListFilter(m_treeItemIdListFilter);
    newInstance->setHideTreeItemIdListFilter(m_hideTreeItemIdListFilter);
    newInstance->setForcedCurrentIndex(m_forcedCurrentIndex);
    newInstance->setParentIdFilter(m_parentIdFilter);
    newInstance->setShowParentWhenParentIdFilter(m_showParentWhenParentIdFilter);
    newInstance->setTagIdListFilter(m_tagIdListFilter);
    newInstance->setShowOnlyWithAttributesFilter(m_showOnlyWithAttributesFilter);
    newInstance->setHideThoseWithAttributesFilter(m_hideThoseWithAttributesFilter);

    newInstance->invalidateFilter();

    return newInstance;
}

int SKRSearchTreeListProxyModel::columnCount(const QModelIndex& parent) const
{
    return 1;
}

Qt::ItemFlags SKRSearchTreeListProxyModel::flags(const QModelIndex& index) const
{
    Qt::ItemFlags defaultFlags = QSortFilterProxyModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags | Qt::ItemFlag::ItemIsEditable;
}

// -----------------------------------------------------------------------


QVariant SKRSearchTreeListProxyModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid()) return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();


    SKRTreeItem *item = static_cast<SKRTreeItem *>(sourceIndex.internalPointer());
    int projectId     = item->projectId();
    int treeItemId    = item->treeItemId();


    if ((role == Qt::EditRole) && (col == 0)) {
        return this->sourceModel()->data(sourceIndex,
                                         SKRTreeItem::Roles::TitleRole).toString();
    }

    if ((role == Qt::CheckStateRole) && (col == 0)) {
        return m_checkedIdsHash.value(treeItemId, Qt::Unchecked);
    }

    if (role == SKRTreeItem::Roles::HasChildrenRole) {
        return this->hasChildren(projectId, treeItemId);
    }


    return QSortFilterProxyModel::data(index, role);
}

// -----------------------------------------------------------------------

bool SKRSearchTreeListProxyModel::setData(const QModelIndex& index,
                                          const QVariant   & value,
                                          int                role)
{
    QModelIndex sourceIndex = this->mapToSource(index);


    SKRTreeItem *item =
        static_cast<SKRTreeItem *>(sourceIndex.internalPointer());

    if ((role == Qt::EditRole) && (sourceIndex.column() == 0)) {
        if (item->isProjectItem()) {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                SKRTreeItem::Roles::ProjectNameRole);
        } else {
            return this->sourceModel()->setData(sourceIndex,
                                                value,
                                                SKRTreeItem::Roles::TitleRole);
        }
    }


    if ((role == Qt::CheckStateRole) && (sourceIndex.column() == 0)) {
        int treeItemId            = item->treeItemId();
        Qt::CheckState checkState = static_cast<Qt::CheckState>(value.toInt());
        m_checkedIdsHash.insert(treeItemId, checkState);

        if ((checkState == Qt::Checked) || (checkState == Qt::Unchecked)) {
            this->checkStateOfAllChildren(item->projectId(), item->treeItemId(), checkState);
        }
        this->determineCheckStateOfAllAncestors(item->projectId(),
                                                item->treeItemId(),
                                                checkState);
    }

    return QSortFilterProxyModel::setData(index, value, role);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::checkStateOfAllChildren(int            projectId,
                                                          int            treeItemId,
                                                          Qt::CheckState checkState) {
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());

    QList<int> childrenIdsList = this->getChildrenList(projectId,
                                                       treeItemId,
                                                       m_showTrashedFilter,
                                                       m_showNotTrashedFilter);

    for (int childId : qAsConst(childrenIdsList)) {
        m_checkedIdsHash.insert(childId, checkState);
        QModelIndex modelIndex = model->getModelIndex(projectId, childId).first();

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::determineCheckStateOfAllAncestors(
    int            projectId,
    int            treeItemId,
    Qt::CheckState checkState)
{
    SKRTreeListModel *model     = static_cast<SKRTreeListModel *>(this->sourceModel());
    QList<int> ancestorsIdsList = this->getAncestorsList(projectId,
                                                         treeItemId,
                                                         m_showTrashedFilter,
                                                         m_showNotTrashedFilter);

    if (ancestorsIdsList.isEmpty()) {
        return;
    }

    // default state :
    Qt::CheckState ancestorCheckState = Qt::Unchecked;

    if (checkState == Qt::Unchecked) {
        // see if the direct ancestor has all its children unchecked
        QList<int> siblingsIdsList = this->getSiblingsList(projectId,
                                                           treeItemId,
                                                           m_showTrashedFilter,
                                                           m_showNotTrashedFilter);

        if (siblingsIdsList.isEmpty()) {
            ancestorCheckState = Qt::PartiallyChecked;
        }
        else {
            bool areNoneOfTheSiblingsChecked          = false;
            bool areAtLeastOneSiblingChecked          = false;
            bool areAtLeastOneSiblingPartiallyChecked = false;
            bool areAtLeastOneSiblingUnchecked        = false;

            for (int siblingId : siblingsIdsList) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingId, Qt::Unchecked);

                if (state == Qt::Checked) {
                    areAtLeastOneSiblingChecked = true;
                }

                if (state == Qt::PartiallyChecked) {
                    areAtLeastOneSiblingPartiallyChecked = true;
                }

                if (state == Qt::Unchecked) {
                    areAtLeastOneSiblingUnchecked = true;
                }
            }
            areNoneOfTheSiblingsChecked = !areAtLeastOneSiblingChecked &&
                                          !areAtLeastOneSiblingPartiallyChecked;

            if (areAtLeastOneSiblingChecked) { // but this one
                ancestorCheckState = Qt::PartiallyChecked;
            }

            if (areAtLeastOneSiblingPartiallyChecked) {
                ancestorCheckState = Qt::PartiallyChecked;
            }
            else if (areNoneOfTheSiblingsChecked) {
                ancestorCheckState = Qt::PartiallyChecked;
            }
        }
    }
    else if (checkState == Qt::Checked) {
        // see if the direct ancestor has all its children checked


        QList<int> siblingsIdsList = this->getSiblingsList(projectId,
                                                           treeItemId,
                                                           m_showTrashedFilter,
                                                           m_showNotTrashedFilter);

        if (siblingsIdsList.isEmpty()) {
            ancestorCheckState = Qt::Checked;
        }
        else {
            bool areAllSiblingsChecked                = false;
            bool areAtLeastOneSiblingChecked          = false;
            bool areAtLeastOneSiblingPartiallyChecked = false;
            bool areAtLeastOneSiblingUnchecked        = false;

            for (int siblingId : siblingsIdsList) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingId, Qt::Unchecked);

                if (state == Qt::Checked) {
                    areAtLeastOneSiblingChecked = true;
                }

                if (state == Qt::PartiallyChecked) {
                    areAtLeastOneSiblingPartiallyChecked = true;
                }

                if (state == Qt::Unchecked) {
                    areAtLeastOneSiblingUnchecked = true;
                }
            }
            areAllSiblingsChecked = !areAtLeastOneSiblingUnchecked &&
                                    !areAtLeastOneSiblingPartiallyChecked;

            if (areAllSiblingsChecked) {
                ancestorCheckState = Qt::Checked;
            }
            else if (areAtLeastOneSiblingUnchecked) { // but this one
                ancestorCheckState = Qt::PartiallyChecked;
            }
            else if (areAtLeastOneSiblingPartiallyChecked) {
                ancestorCheckState = Qt::PartiallyChecked;
            }
        }
    }
    else if (checkState == Qt::PartiallyChecked) {
        ancestorCheckState = Qt::PartiallyChecked;
    }

    if (!m_treeItemIdListFilter.isEmpty()) {
        QList<int> newList;

        for (int id : qAsConst(ancestorsIdsList)) {
            if (m_treeItemIdListFilter.contains(id)) {
                newList.append(id);
            }
        }

        ancestorsIdsList = newList;
    }

    if (ancestorsIdsList.empty()) {
        return;
    }

    // for (int ancestorId : ancestorsIdsList) {
    m_checkedIdsHash.insert(ancestorsIdsList.first(), ancestorCheckState);
    QModelIndex modelIndex =
        model->getModelIndex(projectId, ancestorsIdsList.first()).first();

    emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                     QVector<int>() << Qt::CheckStateRole);

    // }
    determineCheckStateOfAllAncestors(m_projectIdFilter,
                                      ancestorsIdsList.first(),
                                      ancestorCheckState);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setTagIdListFilter(const QList<int>& tagIdListFilter)
{
    m_tagIdListFilter = tagIdListFilter;


    emit tagIdListFilterChanged(tagIdListFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setHideThoseWithAttributesFilter(const QStringList& hideThoseWithAttributesFilter)
{
    m_hideThoseWithAttributesFilter = hideThoseWithAttributesFilter;

    emit hideThoseWithAttributesFilterChanged(hideThoseWithAttributesFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setShowOnlyWithAttributesFilter(const QStringList& showOnlyWithAttributesFilter)
{
    m_showOnlyWithAttributesFilter = showOnlyWithAttributesFilter;

    emit showOnlyWithAttributesFilterChanged(showOnlyWithAttributesFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

QList<int>SKRSearchTreeListProxyModel::getCheckedIdsList() {
    QList<int> list;

    QHash<int, Qt::CheckState>::const_iterator i = m_checkedIdsHash.constBegin();

    while (i != m_checkedIdsHash.constEnd()) {
        if ((i.value() == Qt::Checked) || (i.value() == Qt::PartiallyChecked)) {
            list << i.key();
        }
        ++i;
    }


    // sort list

    QList<int> allSortedIds = m_treeHub->getAllIds(m_projectIdFilter);
    QList<int> sortedCheckedIds;

    for (int sortedId : qAsConst(allSortedIds)) {
        if (list.contains(sortedId)) {
            sortedCheckedIds.append(sortedId);
        }
    }


    return sortedCheckedIds;
}

// --------------------------------------------------------------


void SKRSearchTreeListProxyModel::setCheckedIdsList(const QList<int>checkedIdsList) {
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());


    this->clearCheckedList();


    if (checkedIdsList.isEmpty()) {
        return;
    }

    for (int i = checkedIdsList.count() - 1; i >= 0; i--) {
        int treeItemId = checkedIdsList[i];


        if (!this->hasChildren(m_projectIdFilter, treeItemId)) {
            m_checkedIdsHash.insert(treeItemId, Qt::Checked);
        }
        else { // has children so verify if there is one checked or partially
            // checked
            Qt::CheckState finalState  = Qt::Unchecked;
            QList<int> childrenIdsList = this->getChildrenList(m_projectIdFilter,
                                                               treeItemId,
                                                               m_showTrashedFilter,
                                                               m_showNotTrashedFilter);


            bool areAllChildrenChecked              = false;
            bool areAtLeastOneChildChecked          = false;
            bool areAtLeastOneChildPartiallyChecked = false;
            bool areAtLeastOneChildUnchecked        = false;

            for (int childId : qAsConst(childrenIdsList)) {
                Qt::CheckState state = m_checkedIdsHash.value(childId, Qt::Unchecked);

                if (state == Qt::Checked) {
                    areAtLeastOneChildChecked = true;
                }
                else if (state == Qt::PartiallyChecked) {
                    areAtLeastOneChildPartiallyChecked = true;
                }
                else {
                    areAtLeastOneChildUnchecked = true;
                }
            }

            areAllChildrenChecked = !areAtLeastOneChildUnchecked &&
                                    !areAtLeastOneChildPartiallyChecked;

            if (areAtLeastOneChildUnchecked) { // but this one
                finalState = Qt::PartiallyChecked;
            }
            else if (areAllChildrenChecked) {
                finalState = Qt::Checked;
            }
            else {
                finalState = Qt::PartiallyChecked;
            }


            m_checkedIdsHash.insert(treeItemId, finalState);
        }

        // m_checkedIdsHash.insert(treeItemId, Qt::Checked);
        QModelIndexList modelIndexList = model->getModelIndex(m_projectIdFilter, treeItemId);

        if (modelIndexList.isEmpty()) {
            continue;
        }
        QModelIndex modelIndex = modelIndexList.first();

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}

// --------------------------------------------------------------

///
/// \brief SKRSearchTreeListProxyModel::findIdsTrashedAtTheSameTimeThan
/// \param projectId
/// \param treeItemId
/// \return
/// more or less 1 second between items' trashed dates
QList<int>SKRSearchTreeListProxyModel::findIdsTrashedAtTheSameTimeThan(int projectId,
                                                                       int treeItemId) {
    QList<int> list;

    QDateTime parentDate = m_treeHub->getTrashedDate(projectId, treeItemId);


    if (!m_treeItemIdListFilter.isEmpty()) {
        for (int id : m_treeItemIdListFilter) {
            QDateTime childDate = m_treeHub->getTrashedDate(projectId, id);

            if (qAbs(childDate.secsTo(parentDate)) < 1) {
                list.append(id);
            }
        }
    }
    else {
        QList<int> childrenIds = this->getChildrenList(projectId,
                                                       treeItemId,
                                                       m_showTrashedFilter,
                                                       m_showNotTrashedFilter);

        for (int id : childrenIds) {
            QDateTime childDate = m_treeHub->getTrashedDate(projectId, id);

            if (qAbs(childDate.secsTo(parentDate)) < 1) {
                list.append(id);
            }
        }
    }


    return list;
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::deleteDefinitively(int projectId, int treeItemId)
{
    m_treeHub->removeTreeItem(projectId, treeItemId);
    sort(0);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setParentIdFilter(int parentIdFilter)
{
    m_parentIdFilter = parentIdFilter;
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------


void SKRSearchTreeListProxyModel::setShowParentWhenParentIdFilter(bool showParent)
{
    m_showParentWhenParentIdFilter = showParent;
    emit showParentWhenParentIdFilterChanged(showParent);

    if (m_parentIdFilter != -2) {
        this->invalidateFilter();
    }
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setNavigateByBranchesEnabled(bool navigateByBranches)
{
    m_navigateByBranchesEnabled = navigateByBranches;

    emit navigateByBranchesEnabledChanged(navigateByBranches);
}

// --------------------------------------------------------------


bool SKRSearchTreeListProxyModel::filterAcceptsRow(int                sourceRow,
                                                   const QModelIndex& sourceParent) const
{
    bool value = true;

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    SKRTreeItem *item       = static_cast<SKRTreeItem *>(index.internalPointer());
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());

    //        // avoid project item
    //        if (item->data(SKRTreeItem::Roles::TreeItemIdRole).toInt() == -1)
    // {
    //            value = false;
    //        }

    // displays or not project list :
    if (m_navigateByBranchesEnabled) {
        if ((m_parentIdFilter == -2) && item->isProjectItem()) {
            return true;
        }
        else if ((m_parentIdFilter != -2) && item->isProjectItem()) {
            return false;
        }
        else if ((m_parentIdFilter == -2) && !item->isProjectItem()) {
            return false;
        }
    }
    else if ((m_parentIdFilter == -2) && item->isProjectItem()) {
        return false;
    }

    // project filtering :
    if (value &&
        (item->data(SKRTreeItem::Roles::ProjectIdRole).toInt() == m_projectIdFilter)) {
        value = true;
    }
    else if (value) {
        value = false;
    }

    // trashed filtering :
    if (value && (item->data(SKRTreeItem::Roles::TrashedRole).toBool() == true)) {
        value = m_showTrashedFilter;
    }

    // 'not trashed' filtering :
    if (value && (item->data(SKRTreeItem::Roles::TrashedRole).toBool() == false)) {
        value = m_showNotTrashedFilter;
    }


    if (value &&
        item->data(SKRTreeItem::Roles::TitleRole).toString().contains(m_textFilter,
                                                                      Qt::CaseInsensitive))
    {
        value = true;
    }
    else if (value) {
        value = false;
    }

    int treeItemId = item->data(SKRTreeItem::Roles::TreeItemIdRole).toInt();

    // treeItemIdListFiltering :
    if (value && (!m_treeItemIdListFilter.isEmpty() || !m_hideTreeItemIdListFilter.isEmpty())) {
        int showed = false;

        if (m_treeItemIdListFilter.isEmpty()) {
            showed = true;
        }
        else if (m_hideTreeItemIdListFilter.isEmpty()) {
            showed = false;
        }
        else { // both are with values
            showed = false;
        }


        if (m_treeItemIdListFilter.contains(treeItemId)) {
            showed = true;
        }

        if (m_hideTreeItemIdListFilter.contains(treeItemId)) {
            showed = false;
        }
        value = showed;
    }


    // parentId filtering :
    if (value && (m_parentIdFilter != -2)) {
        if (m_showParentWhenParentIdFilter &&
            (m_parentIdFilter == treeItemId)) {
            value = true;
        }
        else {
            SKRTreeListModel *model =
                static_cast<SKRTreeListModel *>(this->sourceModel());
            SKRTreeItem *parentItem = model->getParentTreeItem(item);

            if (parentItem) {
                if (parentItem->treeItemId() == m_parentIdFilter) {
                    value = true;
                }
                else {
                    value = false;
                }
            }
        }
    }

    // tagId filtering

    if (value && !m_tagIdListFilter.isEmpty() && (m_projectIdFilter != -2)) {
        QList<int> tagIds = plmdata->tagHub()->getTagsFromItemId(m_projectIdFilter, treeItemId);

        value = false;

        int tagCountGoal = m_tagIdListFilter.count();
        int tagCount     = 0;

        for (int tag : qAsConst(tagIds)) {
            if (m_tagIdListFilter.contains(tag)) {
                tagCount += 1;
            }
        }

        if (tagCount == tagCountGoal) {
            value = true;
        }
    }

    //  attribute filtering
    if (value && (!m_showOnlyWithAttributesFilter.isEmpty() || !m_hideThoseWithAttributesFilter.isEmpty()) &&
        (m_projectIdFilter != -2)) {
        QStringList attributes = m_propertyHub->getProperty(m_projectIdFilter, treeItemId, "attributes").split(";",
                                                                                                               Qt::SkipEmptyParts);

        int showed = false;

        if (m_showOnlyWithAttributesFilter.isEmpty()) {
            showed = true;
        }
        else if (m_hideThoseWithAttributesFilter.isEmpty()) {
            showed = false;
        }
        else { // both are with values
            showed = false;
        }

        for (const QString& attribute : qAsConst(attributes)) {
            if (m_showOnlyWithAttributesFilter.contains(attribute)) {
                showed = true;
                break;
            }

            if (m_hideThoseWithAttributesFilter.contains(attribute)) {
                showed = false;
                break;
            }
        }


        value = showed;
    }


    return value;
}

void SKRSearchTreeListProxyModel::setProjectIdFilter(int projectIdFilter)
{
    m_projectIdFilter = projectIdFilter;
    emit projectIdFilterChanged(m_projectIdFilter);

    m_parentIdFilter = -2;
    emit parentIdFilterChanged(m_parentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------
// TODO: complete this :
void SKRSearchTreeListProxyModel::clearFilters()
{
    m_projectIdFilter = -2;
    emit projectIdFilterChanged(m_projectIdFilter);

    m_showTrashedFilter = true;
    emit showTrashedFilterChanged(m_showTrashedFilter);

    m_showNotTrashedFilter = true;
    emit showNotTrashedFilterChanged(m_showNotTrashedFilter);

    m_textFilter = "";
    emit textFilterChanged(m_textFilter);

    m_parentIdFilter = -2;
    emit parentIdFilterChanged(m_parentIdFilter);

    m_showParentWhenParentIdFilter = false;
    emit showParentWhenParentIdFilterChanged(m_showParentWhenParentIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::clearCheckedList()
{
    m_checkedIdsHash.clear();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::checkAll()
{
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());

    if (m_treeItemIdListFilter.isEmpty()) {
        QList<int> filteredIdsList;
        QList<int> allIdsList = m_treeHub->getAllIds(m_projectIdFilter);

        for (int id : qAsConst(allIdsList)) {
            bool isTrashed = m_treeHub->getTrashed(m_projectIdFilter, id);

            if (m_showTrashedFilter && isTrashed) {
                filteredIdsList.append(id);
            }

            if (m_showNotTrashedFilter && !isTrashed) {
                filteredIdsList.append(id);
            }
        }

        for (int id : filteredIdsList) {
            m_checkedIdsHash.insert(id, Qt::Checked);

            QModelIndex modelIndex = model->getModelIndex(m_projectIdFilter, id).first();

            emit dataChanged(this->mapFromSource(modelIndex),
                             this->mapFromSource(modelIndex),
                             QVector<int>() << Qt::CheckStateRole);
        }
    }
    else {
        for (int treeItemId : qAsConst(m_treeItemIdListFilter)) {
            m_checkedIdsHash.insert(treeItemId, Qt::Checked);

            QModelIndex modelIndex =
                model->getModelIndex(m_projectIdFilter, treeItemId).first();

            emit dataChanged(this->mapFromSource(modelIndex),
                             this->mapFromSource(modelIndex),
                             QVector<int>() << Qt::CheckStateRole);
        }
    }
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::checkAllButNonPrintable()
{
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());

    if (m_treeItemIdListFilter.isEmpty()) {
        QList<int> filteredIdsList;
        QList<int> allIdsList = m_treeHub->getAllIds(m_projectIdFilter);

        for (int id : qAsConst(allIdsList)) {
            bool isTrashed   = m_treeHub->getTrashed(m_projectIdFilter, id);
            bool isPrintable =
                m_propertyHub->getProperty(m_projectIdFilter, id, "printable", "true") == "true" ? true : false;

            if (m_showTrashedFilter && isTrashed && isPrintable) {
                filteredIdsList.append(id);
            }

            if (m_showNotTrashedFilter && !isTrashed && isPrintable) {
                filteredIdsList.append(id);
            }
        }

        for (int id : filteredIdsList) {
            m_checkedIdsHash.insert(id, Qt::Checked);

            QModelIndex modelIndex = model->getModelIndex(m_projectIdFilter, id).first();

            emit dataChanged(this->mapFromSource(modelIndex),
                             this->mapFromSource(modelIndex),
                             QVector<int>() << Qt::CheckStateRole);
        }
    }
    else {
        for (int treeItemId : qAsConst(m_treeItemIdListFilter)) {
            bool isPrintable =
                m_propertyHub->getProperty(m_projectIdFilter, treeItemId, "printable", "true") == "true" ? true : false;

            if (!isPrintable) {
                continue;
            }

            m_checkedIdsHash.insert(treeItemId, Qt::Checked);

            QModelIndex modelIndex =
                model->getModelIndex(m_projectIdFilter, treeItemId).first();

            emit dataChanged(this->mapFromSource(modelIndex),
                             this->mapFromSource(modelIndex),
                             QVector<int>() << Qt::CheckStateRole);
        }
    }
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::checkNone()
{
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());
    QHash<int, Qt::CheckState> checkedIdsHash(m_checkedIdsHash);

    this->clearCheckedList();

    for (int treeItemId : checkedIdsHash.keys()) {
        QModelIndex modelIndex = model->getModelIndex(m_projectIdFilter, treeItemId).first();
        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}

// --------------------------------------------------------------


SKRTreeItem * SKRSearchTreeListProxyModel::getItem(int projectId, int treeItemId)
{
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());

    SKRTreeItem *item = model->getItem(projectId, treeItemId);

    return item;
}

// --------------------------------------------------------------


void SKRSearchTreeListProxyModel::loadProjectSettings(int projectId)
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


void SKRSearchTreeListProxyModel::saveProjectSettings(int projectId)
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

void SKRSearchTreeListProxyModel::setTreeItemIdListFilter(
    const QList<int>& treeItemIdListFilter)
{
    m_treeItemIdListFilter = treeItemIdListFilter;

    emit treeItemIdListFilterChanged(treeItemIdListFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setHideTreeItemIdListFilter(const QList<int>& hideTreeItemIdListFilter)
{
    m_hideTreeItemIdListFilter = hideTreeItemIdListFilter;

    emit hideTreeItemIdListFilterChanged(hideTreeItemIdListFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setForcedCurrentIndex(int forcedCurrentIndex)
{
    m_forcedCurrentIndex = forcedCurrentIndex;
    emit forcedCurrentIndexChanged(m_forcedCurrentIndex);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setForcedCurrentIndex(int projectId, int treeItemId)
{
    int forcedCurrentIndex = this->findVisualIndex(projectId, treeItemId);

    setForcedCurrentIndex(forcedCurrentIndex);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::setCurrentTreeItemId(int projectId, int treeItemId)
{
    if (projectId == -2) {
        return;
    }

    if ((treeItemId == -2) && (projectId != -2)) {
        this->setParentFilter(-2, -2);
        return;
    }


    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());
    SKRTreeItem *item       = this->getItem(projectId, treeItemId);

    if (!item) {
        treeItemId = m_treeHub->getTopTreeItemId(projectId);
        item       = this->getItem(projectId, treeItemId);
    }

    if (!item) {
        return;
    }


    if (m_navigateByBranchesEnabled) {
        SKRTreeItem *parentItem = model->getParentTreeItem(item);

        if (parentItem) {
            this->setParentFilter(projectId, parentItem->treeItemId());
        }
        else {
            this->setParentFilter(-2, -2); // root item
        }
    }

    this->setForcedCurrentIndex(projectId, treeItemId);
}

void SKRSearchTreeListProxyModel::setTextFilter(const QString& value)
{
    m_textFilter = value;
    emit textFilterChanged(value);

    this->invalidateFilter();
}

// ----------------------------------------------------------------------------------

QList<int>SKRSearchTreeListProxyModel::getChildrenList(int  projectId,
                                                       int  treeItemId,
                                                       bool getTrashed,
                                                       bool getNotTrashed) const
{
    QList<int> resultChildrenIdList;

    QList<int> allChildrenIdList = m_treeHub->getAllChildren(projectId, treeItemId);

    for (int childId : qAsConst(allChildrenIdList)) {
        bool isTrashed = m_treeHub->getTrashed(projectId, childId);

        if (getTrashed && isTrashed) {
            resultChildrenIdList.append(childId);
        }

        if (getNotTrashed && !isTrashed) {
            resultChildrenIdList.append(childId);
        }
    }

    return resultChildrenIdList;
}

// ----------------------------------------------------------------------------------

QList<int>SKRSearchTreeListProxyModel::getAncestorsList(int  projectId,
                                                        int  treeItemId,
                                                        bool getTrashed,
                                                        bool getNotTrashed) {
    QList<int> resultAllAncestorsIdList;

    QList<int> allAncestorsIdList =
        m_treeHub->getAllAncestors(projectId, treeItemId);

    for (int ancestorId : qAsConst(allAncestorsIdList)) {
        bool isTrashed = m_treeHub->getTrashed(projectId, ancestorId);

        if (getTrashed && isTrashed) {
            resultAllAncestorsIdList.append(ancestorId);
        }

        if (getNotTrashed && !isTrashed) {
            resultAllAncestorsIdList.append(ancestorId);
        }
    }

    return resultAllAncestorsIdList;
}

// ----------------------------------------------------------------------------------

QList<int>SKRSearchTreeListProxyModel::getSiblingsList(int  projectId,
                                                       int  treeItemId,
                                                       bool getTrashed,
                                                       bool getNotTrashed) {
    QList<int> resultAllSiblingsIdList;

    QList<int> allSiblingsIdList =
        m_treeHub->getAllSiblings(projectId, treeItemId);

    for (int ancestorId : allSiblingsIdList) {
        bool isTrashed = m_treeHub->getTrashed(projectId, ancestorId);

        if (getTrashed && isTrashed) {
            resultAllSiblingsIdList.append(ancestorId);
        }

        if (getNotTrashed && !isTrashed) {
            resultAllSiblingsIdList.append(ancestorId);
        }
    }

    return resultAllSiblingsIdList;
}

// ----------------------------------------------------------------------------------


void SKRSearchTreeListProxyModel::setShowNotTrashedFilter(bool showNotTrashedFilter)
{
    m_showNotTrashedFilter = showNotTrashedFilter;

    emit showNotTrashedFilterChanged(showNotTrashedFilter);

    this->invalidateFilter();
}

void SKRSearchTreeListProxyModel::setShowTrashedFilter(bool showTrashedFilter)
{
    m_showTrashedFilter = showTrashedFilter;

    emit showTrashedFilterChanged(showTrashedFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

int SKRSearchTreeListProxyModel::getChildrenCount(int projectId, int treeItemId) const
{
    return this->getChildrenList(projectId,
                                 treeItemId,
                                 m_showTrashedFilter,
                                 m_showNotTrashedFilter).count();
}

// --------------------------------------------------------------

bool SKRSearchTreeListProxyModel::hasChildren(int projectId, int treeItemId) const
{
    return m_treeHub->hasChildren(projectId,
                                  treeItemId,
                                  m_showTrashedFilter,
                                  m_showNotTrashedFilter);
}

// --------------------------------------------------------------

int SKRSearchTreeListProxyModel::findVisualIndex(int projectId, int treeItemId)
{
    int rowCount = this->rowCount(QModelIndex());

    int visualIndex = -2;

    QModelIndex modelIndex;

    for (int i = 0; i < rowCount; ++i) {
        modelIndex = this->index(i, 0);

        if ((this->data(modelIndex,
                        SKRTreeItem::Roles::ProjectIdRole).toInt() == projectId)
            && (this->data(modelIndex,
                           SKRTreeItem::Roles::TreeItemIdRole).toInt() == treeItemId)) {
            visualIndex = i;
            break;
        }
    }
    return visualIndex;
}

// --------------------------------------------------------------
QString SKRSearchTreeListProxyModel::getItemName(int projectId, int treeItemId)
{
    // qDebug() << "getItemName" << projectId << treeItemId;
    if ((projectId == -2) || (treeItemId == -2)) {
        return "";
    }
    QString name = "";

    if (treeItemId == -1) {
        name = plmdata->projectHub()->getProjectName(projectId);
    }
    else {
        SKRTreeItem *item = this->getItem(projectId, treeItemId);

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

int SKRSearchTreeListProxyModel::getItemIndent(int projectId, int treeItemId)
{
    if ((projectId == -2) || (treeItemId == -2)) {
        return -2;
    }
    int indent = -2;

    if (treeItemId == -1) {
        indent = -1;
    }
    else {
        SKRTreeItem *item = this->getItem(projectId, treeItemId);

        if (item) {
            indent = item->indent();
        }
        else {
            qDebug() << this->metaObject()->className() << "no valid item found";
            indent = -2;
        }
    }

    return indent;
}

// -----------------------------------------------------------------

SKRResult SKRSearchTreeListProxyModel::addChildItem(int            projectId,
                                                    int            parentTreeItemId,
                                                    const QString& type)
{
    SKRResult result = m_treeHub->addChildTreeItem(projectId, parentTreeItemId, type);

    IFOK(result) {
        int newTreeItemId =  m_treeHub->getLastAddedId();

        result.addData("treeItemId", newTreeItemId);


        sort(0);
        emit sortOtherProxyModelsCalled();
        int  newVisualIndex = this->findVisualIndex(projectId, newTreeItemId);

        this->setForcedCurrentIndex(newVisualIndex);
    }
    return result;
}

// -----------------------------------------------------------------

SKRResult SKRSearchTreeListProxyModel::addItemAbove(int            projectId,
                                                    int            parentTreeItemId,
                                                    const QString& type)
{
    SKRResult result = m_treeHub->addTreeItemAbove(projectId, parentTreeItemId, type);

    IFOK(result) {
        int newTreeItemId =  m_treeHub->getLastAddedId();

        result.addData("treeItemId", newTreeItemId);


        sort(0);
        emit sortOtherProxyModelsCalled();
        int  newVisualIndex = this->findVisualIndex(projectId, newTreeItemId);

        this->setForcedCurrentIndex(newVisualIndex);
    }
    return result;
}

// -----------------------------------------------------------------

SKRResult SKRSearchTreeListProxyModel::addItemBelow(int            projectId,
                                                    int            parentTreeItemId,
                                                    const QString& type)
{
    SKRResult result = m_treeHub->addTreeItemBelow(projectId, parentTreeItemId, type);

    IFOK(result) {
        int newTreeItemId =  m_treeHub->getLastAddedId();

        result.addData("treeItemId", newTreeItemId);


        sort(0);
        emit sortOtherProxyModelsCalled();
        int  newVisualIndex = this->findVisualIndex(projectId, newTreeItemId);

        this->setForcedCurrentIndex(newVisualIndex);
    }
    return result;
}

// --------------------------------------------------------------

SKRResult SKRSearchTreeListProxyModel::moveUp(int projectId, int treeItemId, int visualIndex)
{
    SKRTreeItem *item = this->getItem(projectId, treeItemId);

    if (!item) {
        return SKRResult();
    }
    SKRResult result = m_treeHub->moveTreeItemUp(projectId, treeItemId);

    IFOK(result) {
        sort(0);
        emit sortOtherProxyModelsCalled();
        int  newVisualIndex = this->findVisualIndex(projectId, treeItemId);

        this->setForcedCurrentIndex(newVisualIndex);
    }
    return result;
}

// --------------------------------------------------------------

SKRResult SKRSearchTreeListProxyModel::moveDown(int projectId, int treeItemId, int visualIndex)
{
    SKRTreeItem *item = this->getItem(projectId, treeItemId);

    if (!item) {
        return SKRResult();
    }
    SKRResult result = m_treeHub->moveTreeItemDown(projectId, treeItemId);

    IFOK(result) {
        sort(0);
        emit sortOtherProxyModelsCalled();
        int  newVisualIndex = this->findVisualIndex(projectId, treeItemId);

        this->setForcedCurrentIndex(newVisualIndex);
    }
    return result;
}

// --------------------------------------------------------------


///
/// \brief SKRSearchTreeListProxyModel::moveItem
/// \param from source item index number
/// \param to target item index number
/// Carefull, this is only used for manually moving a visual item
void SKRSearchTreeListProxyModel::moveItem(int from, int to) {
    if (from == to) return;

    int modelFrom = from;
    int modelTo   = to + (from < to ? 1 : 0);


    QModelIndex fromIndex = this->index(modelFrom, 0);
    int fromTreeItemId    =
        this->data(fromIndex, SKRTreeItem::Roles::TreeItemIdRole).toInt();
    int fromProjectId =
        this->data(fromIndex, SKRTreeItem::Roles::ProjectIdRole).toInt();

    QModelIndex toIndex = this->index(modelTo, 0);
    int toTreeItemId    = this->data(toIndex, SKRTreeItem::Roles::TreeItemIdRole).toInt();
    int toProjectId     = this->data(toIndex, SKRTreeItem::Roles::ProjectIdRole).toInt();
    int toSortOrder     = this->data(toIndex, SKRTreeItem::Roles::SortOrderRole).toInt();

    qDebug() << "fromTreeItemId : " << fromTreeItemId << this->data(fromIndex,
                                                                    SKRTreeItem::Roles::TitleRole)
        .toString();
    qDebug() << "toTreeItemId : " << toTreeItemId << this->data(toIndex,
                                                                SKRTreeItem::Roles::TitleRole).
        toString();


    m_treeHub->moveTreeItem(fromProjectId, fromTreeItemId, toTreeItemId, false);


    sort(0);
    emit sortOtherProxyModelsCalled();

    this->invalidate();
}

// --------------------------------------------------------------

int SKRSearchTreeListProxyModel::goUp()
{
    SKRTreeListModel *model = static_cast<SKRTreeListModel *>(this->sourceModel());
    SKRTreeItem *parentItem = getItem(m_projectIdFilter, m_parentIdFilter);

    if (!parentItem) {
        return -2;
    }
    SKRTreeItem *grandParentItem = model->getParentTreeItem(parentItem);

    int grandParentId = -2;

    if (grandParentItem) {
        grandParentId = grandParentItem->treeItemId();
    }
    this->setParentFilter(m_projectIdFilter, grandParentId);

    return grandParentId;
}

// --------------------------------------------------------------
void SKRSearchTreeListProxyModel::setParentFilter(int projectId, int parentId)
{
    m_projectIdFilter = projectId;
    m_parentIdFilter  = parentId;
    emit parentIdFilterChanged(m_parentIdFilter);
    emit projectIdFilterChanged(m_projectIdFilter);

    this->invalidateFilter();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::trashItemWithChildren(int projectId, int treeItemId)
{
    SKRTreeItem *item = this->getItem(projectId, treeItemId);

    if (!item) {
        return;
    }

    m_treeHub->setTrashedWithChildren(projectId, treeItemId, true);
}

// --------------------------------------------------------------

QHash<int, QByteArray>SKRSearchTreeListProxyModel::roleNames() const {
    QHash<int, QByteArray> roles = QSortFilterProxyModel::roleNames();

    roles[Qt::CheckStateRole] = "checkState";

    return roles;
}

// --------------------------------------------------------------

int SKRSearchTreeListProxyModel::getLastOfHistory(int projectId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    if (list.isEmpty()) {
        return -2;
    }

    return list.last();
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::removeLastOfHistory(int projectId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    if (list.isEmpty()) {
        return;
    }

    list.removeLast();
    m_historyList.insert(projectId, list);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::clearHistory(int projectId)
{
    m_historyList.remove(projectId);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::addHistory(int projectId, int treeItemId)
{
    QList<int> list = m_historyList.value(projectId, QList<int>());

    list.append(treeItemId);
    m_historyList.insert(projectId, list);
}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::cut(int projectId, int treeItemId) {}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::copy(int projectId, int treeItemId) {}

// --------------------------------------------------------------

void SKRSearchTreeListProxyModel::paste(int targetProjectId, int targetParentId) {}

// --------------------------------------------------------------
