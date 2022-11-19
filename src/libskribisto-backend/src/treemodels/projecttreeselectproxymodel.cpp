#include "projecttreeselectproxymodel.h"
#include "projecttreemodel.h"

ProjectTreeSelectProxyModel::ProjectTreeSelectProxyModel(QObject *parent, int projectId)
    : QSortFilterProxyModel(parent), m_projectId(projectId)
{
    this->setSourceModel(ProjectTreeModel::instance());
}

Qt::ItemFlags ProjectTreeSelectProxyModel::flags(const QModelIndex &index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    defaultFlags.setFlag(Qt::ItemIsUserCheckable);
    defaultFlags.setFlag(Qt::ItemIsAutoTristate);


    return defaultFlags;
}

//----------------------------------------------------------

int ProjectTreeSelectProxyModel::projectId() const
{
    return m_projectId;
}

void ProjectTreeSelectProxyModel::setProjectId(int newProjectId)
{
    m_projectId = newProjectId;

    this->invalidateFilter();
}

//----------------------------------------------------------

QVariant ProjectTreeSelectProxyModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();

    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(sourceIndex.internalPointer());

    if ((role == Qt::CheckStateRole) && (col == 0)) {
        return m_checkedIdsHash.value(item->treeItemAddress(), Qt::Unchecked);
    }

    return QSortFilterProxyModel::data(index, role);
}


//----------------------------------------------------------

bool ProjectTreeSelectProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const
{

    QModelIndex index = this->sourceModel()->index(sourceRow, 0, sourceParent);

    if (!index.isValid()) {
        return false;
    }
    ProjectTreeItem *item       = static_cast<ProjectTreeItem *>(index.internalPointer());


    if(item->projectId() == m_projectId){
        return true;
    }

return false;

}

//----------------------------------------------------------

bool ProjectTreeSelectProxyModel::setData(const QModelIndex &index, const QVariant &value, int role)
{

    QModelIndex sourceIndex = this->mapToSource(index);


    ProjectTreeItem *item =
        static_cast<ProjectTreeItem *>(sourceIndex.internalPointer());

    if ((role == Qt::CheckStateRole) && (sourceIndex.column() == 0)) {
        TreeItemAddress treeItemId            = item->treeItemAddress();
        Qt::CheckState checkState = static_cast<Qt::CheckState>(value.toInt());
        m_checkedIdsHash.insert(treeItemId, checkState);

            if ((checkState == Qt::Checked) || (checkState == Qt::Unchecked)) {
                this->checkStateOfAllChildren(item->treeItemAddress(), checkState);
            }
            this->determineCheckStateOfAllAncestors(item->treeItemAddress(),
                                                    checkState);

    }

    return QSortFilterProxyModel::setData(index, value, role);
}


// --------------------------------------------------------------

void ProjectTreeSelectProxyModel::checkStateOfAllChildren(const TreeItemAddress &treeItemAddress,
                                                          Qt::CheckState checkState) {
    ProjectTreeModel *model = static_cast<ProjectTreeModel *>(this->sourceModel());

    QList<TreeItemAddress> childrenIdsList = skrdata->treeHub()->getAllChildren(treeItemAddress);

    for (const TreeItemAddress &childTreeItemAddress : qAsConst(childrenIdsList)) {
        m_checkedIdsHash.insert(childTreeItemAddress, checkState);
        QModelIndex modelIndex = model->getModelIndex(childTreeItemAddress);

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}

// --------------------------------------------------------------

void ProjectTreeSelectProxyModel::determineCheckStateOfAllAncestors(const TreeItemAddress &treeItemAddress,
    Qt::CheckState checkState)
{
    ProjectTreeModel *model     = static_cast<ProjectTreeModel *>(this->sourceModel());
    QList<TreeItemAddress> ancestorsIdsList = skrdata->treeHub()->getAllAncestors(treeItemAddress);

    if (ancestorsIdsList.isEmpty()) {
        return;
    }

    // default state :
    Qt::CheckState ancestorCheckState = Qt::Unchecked;

    if (checkState == Qt::Unchecked) {
        // see if the direct ancestor has all its children unchecked
        QList<TreeItemAddress> siblingsIdsList = skrdata->treeHub()->getAllSiblings(treeItemAddress);

        if (siblingsIdsList.isEmpty()) {
            ancestorCheckState = Qt::PartiallyChecked;
        }
        else {
            bool areNoneOfTheSiblingsChecked          = false;
            bool areAtLeastOneSiblingChecked          = false;
            bool areAtLeastOneSiblingPartiallyChecked = false;
            bool areAtLeastOneSiblingUnchecked        = false;

            for (const TreeItemAddress &siblingTreeItemAddress : qAsConst(siblingsIdsList)) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingTreeItemAddress, Qt::Unchecked);

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


        QList<TreeItemAddress> siblingsIdsList = skrdata->treeHub()->getAllSiblings(treeItemAddress);

        if (siblingsIdsList.isEmpty()) {
            ancestorCheckState = Qt::Checked;
        }
        else {
            bool areAllSiblingsChecked                = false;
            bool areAtLeastOneSiblingChecked          = false;
            bool areAtLeastOneSiblingPartiallyChecked = false;
            bool areAtLeastOneSiblingUnchecked        = false;

            for (const TreeItemAddress &siblingTreeItemAddress : qAsConst(siblingsIdsList)) {
                Qt::CheckState state = m_checkedIdsHash.value(siblingTreeItemAddress, Qt::Unchecked);

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

//    if (!m_treeItemIdListFilter.isEmpty()) {
//        QList<int> newList;

//        for (int id : qAsConst(ancestorsIdsList)) {
//            if (m_treeItemIdListFilter.contains(id)) {
//                newList.append(id);
//            }
//        }

//        ancestorsIdsList = newList;
//    }

    if (ancestorsIdsList.empty()) {
        return;
    }

    // for (int ancestorId : ancestorsIdsList) {
    m_checkedIdsHash.insert(ancestorsIdsList.first(), ancestorCheckState);
    QModelIndex modelIndex =
        model->getModelIndex(ancestorsIdsList.first());

    emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                     QVector<int>() << Qt::CheckStateRole);

    // }
    determineCheckStateOfAllAncestors(ancestorsIdsList.first(),
                                      ancestorCheckState);
}

// --------------------------------------------------------------

//----------------------------------------------------------

QList<TreeItemAddress>ProjectTreeSelectProxyModel::getCheckedIdsList() {
    QList<TreeItemAddress> checkedList;

    QHash<TreeItemAddress, Qt::CheckState>::const_iterator i = m_checkedIdsHash.constBegin();

    while (i != m_checkedIdsHash.constEnd()) {
        if ((i.value() == Qt::Checked) || (i.value() == Qt::PartiallyChecked)) {
            checkedList << i.key();
        }
        ++i;
    }


    // sort list

    QList<TreeItemAddress> allSortedIds = skrdata->treeHub()->getAllIds(m_projectId);
    QList<TreeItemAddress> sortedCheckedIds;

    for (const TreeItemAddress &sortedTreeItemAddress : qAsConst(allSortedIds)) {
        if (checkedList.contains(sortedTreeItemAddress)) {
            sortedCheckedIds.append(sortedTreeItemAddress);
        }
    }


    return sortedCheckedIds;
}

// --------------------------------------------------------------


void ProjectTreeSelectProxyModel::setCheckedIdsList(const QList<TreeItemAddress> checkedAddresses) {
    ProjectTreeModel *model = static_cast<ProjectTreeModel *>(this->sourceModel());


    m_checkedIdsHash.clear();


    if (checkedAddresses.isEmpty()) {
        return;
    }

    for (int i = checkedAddresses.count() - 1; i >= 0; i--) {
        const TreeItemAddress &treeItemAddress = checkedAddresses.at(i);


        if (!skrdata->treeHub()->hasChildren(treeItemAddress)) {
            m_checkedIdsHash.insert(treeItemAddress, Qt::Checked);
        }
        else { // has children so verify if there is one checked or partially
            // checked
            Qt::CheckState finalState  = Qt::Unchecked;
            QList<TreeItemAddress> childrenIdsList = skrdata->treeHub()->getAllChildren(treeItemAddress);


            bool areAllChildrenChecked              = false;
            bool areAtLeastOneChildChecked          = false;
            bool areAtLeastOneChildPartiallyChecked = false;
            bool areAtLeastOneChildUnchecked        = false;

            for (const TreeItemAddress &childTreeItemAddress : qAsConst(childrenIdsList)) {
                Qt::CheckState state = m_checkedIdsHash.value(childTreeItemAddress, Qt::Unchecked);

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


            m_checkedIdsHash.insert(treeItemAddress, finalState);
        }

        // m_checkedIdsHash.insert(treeItemId, Qt::Checked);
        QModelIndex modelIndex = model->getModelIndex(treeItemAddress);

        if (!modelIndex.isValid()) {
            continue;
        }

        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}

// --------------------------------------------------------------

void ProjectTreeSelectProxyModel::checkNone()
{
    ProjectTreeModel *model = static_cast<ProjectTreeModel *>(this->sourceModel());
    QHash<TreeItemAddress, Qt::CheckState> checkedIdsHash(m_checkedIdsHash);

    m_checkedIdsHash.clear();

    for (const TreeItemAddress &treeItemAddress : checkedIdsHash.keys()) {
        QModelIndex modelIndex = model->getModelIndex(treeItemAddress);
        emit dataChanged(this->mapFromSource(modelIndex), this->mapFromSource(modelIndex),
                         QVector<int>() << Qt::CheckStateRole);
    }
}
