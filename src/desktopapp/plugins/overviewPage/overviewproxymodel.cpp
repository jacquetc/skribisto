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


QVariant OverviewProxyModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    if(role == Qt::ItemDataRole::DisplayRole && orientation == Qt::Orientation::Horizontal){
    switch(section){
    case 0:
        return tr("Title");
        break;
    case 1:
        return tr("Outline");
        break;
    case 2:
        return tr("Word count");
        break;
//    case 3:
//        return tr("Title");
//        break;
    default:
        return QVariant();
        break;
    }
}

    return QVariant();

}

QVariant OverviewProxyModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

        int col                 = index.column();
        int row                 = index.row();

    QModelIndex sourceIndex = this->mapToSource(index);

    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(sourceIndex.internalPointer());



    if (role == ProjectTreeItem::Roles::SecondaryContentRole && col == 1) {
        return item->data(role);
    }

    if(col == 2){
        if (role == Qt::DisplayRole) {
            if(item->data(ProjectTreeItem::Roles::CanAddChildTreeItemRole).toBool()){
                return item->data(ProjectTreeItem::Roles::WordCountWithChildrenRole);
            }
            else {
                return item->data(ProjectTreeItem::Roles::WordCountRole);
            }
        }

        if (role == Qt::ItemDataRole::TextAlignmentRole) {
            return Qt::AlignmentFlag::AlignCenter;
        }

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


