#include "overviewproxymodel.h"
#include "treemodels/projecttreeitem.h"

OverviewProxyModel::OverviewProxyModel(QObject *parent)
    : QSortFilterProxyModel(parent), m_projectId(-1)
{
}

int OverviewProxyModel::columnCount(const QModelIndex &parent) const
{
    Q_UNUSED(parent);
    return 3;
}

QVariant OverviewProxyModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

        int col                 = index.column();
        int row                 = index.row();

    QModelIndex sourceIndex = this->mapToSource(index);

    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(sourceIndex.internalPointer());



    if (role == Qt::DisplayRole && col == 2) {
        return item->data(ProjectTreeItem::Roles::WordCountRole);
    }



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



Qt::ItemFlags OverviewProxyModel::flags(const QModelIndex &index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);


    if (!index.isValid())
        return Qt::NoItemFlags;

    if(index.data(ProjectTreeItem::TreeItemIdRole).toInt() > 0){
        defaultFlags |= Qt::ItemIsDragEnabled;
    }
    if(index.data(ProjectTreeItem::CanAddChildTreeItemRole).toBool()){
        defaultFlags.setFlag(Qt::ItemIsDropEnabled);
    }

    if(index.column() <= 2 && index.data(ProjectTreeItem::TreeItemIdRole).toInt() > 0){
        defaultFlags.setFlag(Qt::ItemIsEditable);
    }

    return defaultFlags;
}
