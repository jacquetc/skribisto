#include "projecttreeitem.h"
#include "projecttreemodel.h"
#include "projecttreeproxymodel.h"
#include "skrdata.h"


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
    if (!index.isValid())
        return Qt::NoItemFlags;

    return QAbstractItemModel::flags(index);// | Qt::ItemIsEditable; // FIXME: Implement me!
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
