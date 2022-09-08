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
    else if (role == Qt::DisplayRole) {
        return item->data(ProjectTreeItem::Roles::TitleRole);
    }


    return QIdentityProxyModel::data(index, role);
}

Qt::ItemFlags ProjectTreeProxyModel::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return QAbstractItemModel::flags(index);// | Qt::ItemIsEditable; // FIXME: Implement me!
}
