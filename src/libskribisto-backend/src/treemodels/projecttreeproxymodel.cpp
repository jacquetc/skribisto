#include "projecttreecommands.h"
#include "projecttreeitem.h"
#include "projecttreemodel.h"
#include "projecttreeproxymodel.h"
#include "skrdata.h"

#include <QMimeData>


ProjectTreeProxyModel::ProjectTreeProxyModel(QObject *parent)
    : QIdentityProxyModel(parent)
{
    this->setSourceModel(ProjectTreeModel::instance());

}


QVariant ProjectTreeProxyModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    QModelIndex sourceIndex = this->mapToSource(index);
    int col                 = index.column();
    int row                 = index.row();
    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(sourceIndex.internalPointer());



    return QIdentityProxyModel::data(index, role);
}

Qt::ItemFlags ProjectTreeProxyModel::flags(const QModelIndex &index) const
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

    return defaultFlags;

}


QModelIndex ProjectTreeProxyModel::getModelIndex(int projectId, int treeItemId) const {
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


QStringList ProjectTreeProxyModel::mimeTypes() const
{
    QStringList list;
    list << "application/x-navigationtreeitem-list";

    return list;
}

QMimeData *ProjectTreeProxyModel::mimeData(const QModelIndexList &indexes) const
{

    // only keep toppest parents

    QList< QPair<int, int> > pairList;
    for(const QModelIndex &index : indexes){
        QPair<int, int> pair;
        pair.first = index.data(ProjectTreeItem::ProjectIdRole).toInt();
        pair.second = index.data(ProjectTreeItem::TreeItemIdRole).toInt();

        pairList.append(pair);
    }

    int projectId = pairList.first().first;

    QList<int> keptList;
    // forbid multiple projects
    for(const QPair<int, int> &pair : pairList) {
        if(projectId == pair.first){
            keptList.append(pair.second);
        }
    }

    keptList = skrdata->treeHub()->filterOutChildren(projectId, keptList);

    pairList.clear();
    for(int id : keptList) {
        QPair<int, int> pair;
        pair.first = projectId;
        pair.second = id;
        pairList.append(pair);
    }


    QByteArray byteArray;
    QDataStream stream(&byteArray, QIODevice::WriteOnly);
    stream << pairList;

    QMimeData *mimeData = new QMimeData;
    mimeData->setData("application/x-navigationtreeitem-list", byteArray);

    return mimeData;
}

bool ProjectTreeProxyModel::canDropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent) const
{
//    if(!parent.isValid()){
//        return false;
//    }

    QList< QPair<int, int> > pairList;
    QByteArray byteArray = data->data("application/x-navigationtreeitem-list");
    QDataStream stream(&byteArray, QIODevice::ReadOnly);
    stream >> pairList;

    int sourceProjectId = pairList.first().first;

    // forbid multiple projects
    for(const QPair<int, int> &pair : pairList) {
        if(sourceProjectId != pair.first){
            return false;
        }
    }

    // forbid drag drop on other project for now
    if(parent.data(ProjectTreeItem::ProjectIdRole).toInt() != sourceProjectId){
        return false;
    }


    return true;
}

bool ProjectTreeProxyModel::dropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent)
{
    if(data->hasFormat("application/x-navigationtreeitem-list")){

        QList< QPair<int, int> > pairList;
        QByteArray byteArray = data->data("application/x-navigationtreeitem-list");
        QDataStream stream(&byteArray, QIODevice::ReadOnly);
        stream >> pairList;

        int sourceProjectId = pairList.first().first;
        int sourceTreeItemId = pairList.first().second;

        // forbid multiple projects
        for(const QPair<int, int> &pair : pairList) {
            if(sourceProjectId != pair.first){
                return false;
            }
        }

        QList<int> sourceTreeItemIds;

        for(const QPair<int, int> &pair : pairList) {
            sourceTreeItemIds.append(pair.second);
        }



        int targetProjectId = parent.data(ProjectTreeItem::ProjectIdRole).toInt();
        int targetParentTreeItemId = parent.data(ProjectTreeItem::TreeItemIdRole).toInt();
        // as sub item
        if(row == -1 && column == -1){
            projectTreeCommands->moveItemsAsChildOf(sourceProjectId, sourceTreeItemIds, targetProjectId, targetParentTreeItemId);
        }
        //add below
        else if(row == skrdata->treeHub()->getAllDirectChildren(targetProjectId, targetParentTreeItemId, true, true).count()) {

            QModelIndex childIndex = this->index(row - 1, column, parent);

            if(!childIndex.isValid()){
                return false;
            }

            int targetTreeItemId = childIndex.data(ProjectTreeItem::TreeItemIdRole).toInt();
            projectTreeCommands->moveItemsBelow(sourceProjectId, sourceTreeItemIds, targetProjectId, targetTreeItemId);

        }
        //add before
        else {
            QModelIndex childIndex = this->index(row, column, parent);

            if(!childIndex.isValid()){
                return false;
            }

            int targetTreeItemId = childIndex.data(ProjectTreeItem::TreeItemIdRole).toInt();
            projectTreeCommands->moveItemsAbove(sourceProjectId, sourceTreeItemIds, targetProjectId, targetTreeItemId);
        }


        return true;
    }


    return false;
}

Qt::DropActions ProjectTreeProxyModel::supportedDropActions() const
{
    return Qt::MoveAction;
}

Qt::DropActions ProjectTreeProxyModel::supportedDragActions() const
{
    return Qt::MoveAction;

}
