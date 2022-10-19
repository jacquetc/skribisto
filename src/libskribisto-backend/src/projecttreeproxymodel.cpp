#include "projecttreeitem.h"
#include "projecttreemodel.h"
#include "projecttreeproxymodel.h"
#include "skrdata.h"

#include <QMimeData>


ProjectTreeProxyModel::ProjectTreeProxyModel(QObject *parent)
    : QIdentityProxyModel(parent)
{
    this->setSourceModel(ProjectTreeModel::instance());


    QList<PageInterface *> pluginList =
            skrpluginhub->pluginsByType<PageInterface>();




    for(auto *plugin : pluginList){
        m_typeWithPlugin.insert(plugin->pageType(), plugin);
    }
}


QVariant ProjectTreeProxyModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(index.internalPointer());


    if (role == Qt::DecorationRole) {
        auto *plugin = m_typeWithPlugin.value(item->data(ProjectTreeItem::Roles::TypeRole).toString(), nullptr);
        if(plugin){
            return QIcon(plugin->pageTypeIconUrl(item->data(ProjectTreeItem::Roles::ProjectIdRole).toInt(), item->data(ProjectTreeItem::Roles::TreeItemIdRole).toInt()));
        }
        return QVariant();
    }


    return QIdentityProxyModel::data(index, role);
}

Qt::ItemFlags ProjectTreeProxyModel::flags(const QModelIndex &index) const
{
    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if(index.data(ProjectTreeItem::TreeItemIdRole).toInt() > 0){
        defaultFlags |= Qt::ItemIsDragEnabled;
    }
    if(index.data(ProjectTreeItem::CanAddChildTreeItemRole).toBool()){
        defaultFlags.setFlag(Qt::ItemIsDropEnabled);
    }

    if (!index.isValid())
        return Qt::NoItemFlags;

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
    QList< QPair<int, int> > pairList;
    for(const QModelIndex &index : indexes){
        QPair<int, int> pair;
        pair.first = index.data(ProjectTreeItem::ProjectIdRole).toInt();
        pair.second = index.data(ProjectTreeItem::TreeItemIdRole).toInt();

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
    if(!parent.isValid()){
        return false;
    }
    return true;
}

bool ProjectTreeProxyModel::dropMimeData(const QMimeData *data, Qt::DropAction action, int row, int column, const QModelIndex &parent)
{
    return true;
}

Qt::DropActions ProjectTreeProxyModel::supportedDropActions() const
{
    return Qt::MoveAction;
}

Qt::DropActions ProjectTreeProxyModel::supportedDragActions() const
{
    return Qt::MoveAction;

}
