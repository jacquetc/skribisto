#include "overviewproxymodel.h"
#include "treemodels/projecttreeitem.h"

OverviewProxyModel::OverviewProxyModel(QObject *parent)
    : QSortFilterProxyModel(parent), m_projectId(-1)
{
}

QVariant OverviewProxyModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

   // ProjectTreeItem *item = static_cast<ProjectTreeItem *>(index.internalPointer());



    return QSortFilterProxyModel::data(index, role);
}




bool OverviewProxyModel::filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const
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

int OverviewProxyModel::projectId() const
{
    return m_projectId;
}

void OverviewProxyModel::setProjectId(int newProjectId)
{
    m_projectId = newProjectId;

    this->invalidateFilter();
}

QModelIndex OverviewProxyModel::getModelIndex(int projectId, int treeItemId) const {
    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        ProjectTreeItem::Roles::TreeItemIdRole,
                                        treeItemId,
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : qAsConst(list)) {
        ProjectTreeItem *t = static_cast<ProjectTreeItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == projectId) index = modelIndex;
    }

    if (index.isValid())
        return index;

    return QModelIndex();
}
