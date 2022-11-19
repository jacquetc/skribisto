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

    if(index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>().itemId > 0){
        defaultFlags |= Qt::ItemIsDragEnabled;
    }
    if(index.data(ProjectTreeItem::CanAddChildTreeItemRole).toBool()){
        defaultFlags.setFlag(Qt::ItemIsDropEnabled);
    }

    return defaultFlags;

}


QModelIndex ProjectTreeProxyModel::getModelIndex(const TreeItemAddress &treeItemAddress) const {
    // search for index
    QModelIndex index;
    QModelIndexList list =  this->match(this->index(0, 0,
                                                    QModelIndex()),
                                        ProjectTreeItem::Roles::TreeItemAddressRole,
                                        QVariant::fromValue(treeItemAddress),
                                        -1,
                                        Qt::MatchFlag::MatchRecursive |
                                        Qt::MatchFlag::MatchExactly |
                                        Qt::MatchFlag::MatchWrap);

    for (const QModelIndex& modelIndex : qAsConst(list)) {
        ProjectTreeItem *t = static_cast<ProjectTreeItem *>(modelIndex.internalPointer());

        if (t)
            if (t->projectId() == treeItemAddress.projectId) index = modelIndex;
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

    QList< TreeItemAddress > sourceTreeItemAddresses;
    for(const QModelIndex &index : indexes){
         sourceTreeItemAddresses << index.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
    }

    int projectId = sourceTreeItemAddresses.first().projectId;
    // forbid multiple projects

    QList< TreeItemAddress > keptAddresses;
    for(const TreeItemAddress &sourceTreeItemAddress : sourceTreeItemAddresses){
        if(projectId == sourceTreeItemAddress.projectId){
            keptAddresses.append(sourceTreeItemAddress);
        }
    }

    sourceTreeItemAddresses = skrdata->treeHub()->filterOutChildren(keptAddresses);
    //qDebug() << "drag" << sourceTreeItemAddresses;

    QByteArray byteArray;
    QDataStream stream(&byteArray, QIODevice::WriteOnly);
    stream << sourceTreeItemAddresses;

    QMimeData *mimeData = new QMimeData;
    mimeData->setData("application/x-navigationtreeitem-list", byteArray);

    return mimeData;
}

bool ProjectTreeProxyModel::canDropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent) const
{
    //    if(!parent.isValid()){
    //        return false;
    //    }

    if(data->hasFormat("application/x-navigationtreeitem-list")){

        QList< TreeItemAddress >  sourceTreeItemAddresses;
      QByteArray byteArray = data->data("application/x-navigationtreeitem-list");
        QDataStream stream(&byteArray, QIODevice::ReadOnly);
        stream >> sourceTreeItemAddresses;
        //QList< QVariant > variantList = QVariant(byteArray).toList();

        //qDebug() << "drop" << sourceTreeItemAddresses;

        int sourceProjectId = sourceTreeItemAddresses.first().projectId;

        // forbid multiple projects
        for(const TreeItemAddress &treeItemAddress : sourceTreeItemAddresses) {
            if(sourceProjectId != treeItemAddress.projectId){
                return false;
            }
        }

        // forbid drag drop on other project for now
        if(parent.data(ProjectTreeItem::ProjectIdRole).toInt() != sourceProjectId){
            return false;
        }

        return true;

    }
    return false;

}

bool ProjectTreeProxyModel::dropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent)
{
    if(data->hasFormat("application/x-navigationtreeitem-list")){

        QList< TreeItemAddress >  sourceTreeItemAddresses;
        QByteArray byteArray = data->data("application/x-navigationtreeitem-list");
        QDataStream stream(&byteArray, QIODevice::ReadOnly);
        stream >> sourceTreeItemAddresses;

        int sourceProjectId = sourceTreeItemAddresses.first().projectId;

        // forbid multiple projects
        for(const TreeItemAddress &treeItemAddress : sourceTreeItemAddresses) {
            if(sourceProjectId != treeItemAddress.projectId){
                return false;
            }
        }

        TreeItemAddress targetParentTreeItemAddress = parent.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
        // as sub item
        if(row == -1 && column == -1){
            projectTreeCommands->moveItemsAsChildOf(sourceTreeItemAddresses, targetParentTreeItemAddress);
        }
        //add below
        else if(row == skrdata->treeHub()->getAllDirectChildren(targetParentTreeItemAddress, true, true).count()) {

            QModelIndex childIndex = this->index(row - 1, column, parent);

            if(!childIndex.isValid()){
                return false;
            }

            TreeItemAddress targetTreeItemAddress = childIndex.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
            projectTreeCommands->moveItemsBelow(sourceTreeItemAddresses, targetTreeItemAddress);

        }
        //add before
        else {
            QModelIndex childIndex = this->index(row, column, parent);

            if(!childIndex.isValid()){
                return false;
            }

            TreeItemAddress targetTreeItemAddress = childIndex.data(ProjectTreeItem::TreeItemAddressRole).value<TreeItemAddress>();
            projectTreeCommands->moveItemsAbove(sourceTreeItemAddresses, targetTreeItemAddress);
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
