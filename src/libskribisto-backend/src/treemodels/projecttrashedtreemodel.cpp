#include "projecttrashedtreemodel.h"

#include <QIcon>
#include <skrdata.h>

ProjectTrashedTreeModel::ProjectTrashedTreeModel(QObject *parent)
    : QAbstractItemModel(parent),
      m_projectId(-1)
{

    m_treeHub     = skrdata->treeHub();
     m_propertyHub = skrdata->treePropertyHub();

    m_rootItem = new ProjectTreeItem();
    m_rootItem->setIsRootItem();


    QList<PageTypeIconInterface *> pluginList =
            skrpluginhub->pluginsByType<PageTypeIconInterface>();
    for(auto *plugin : pluginList){
        m_typeWithPlugin.insert(plugin->pageType(), plugin);
    }

     connect(skrdata->projectHub(),
             &PLMProjectHub::projectLoaded,
             this,
             &ProjectTrashedTreeModel::populate);
     connect(skrdata->projectHub(),
             &PLMProjectHub::projectClosed,
             this,
             &ProjectTrashedTreeModel::populate);
     connect(skrdata->treeHub(),
             &SKRTreeHub::treeReset,
             this,
             &ProjectTrashedTreeModel::populate);
     connect(skrdata->treeHub(),
             &SKRTreeHub::trashedChanged,
             this,
             &ProjectTrashedTreeModel::populate);
     connect(skrdata->treeHub(),
             &SKRTreeHub::treeItemRemoved,
             this,
             &ProjectTrashedTreeModel::populate);
     this->connectToSKRDataSignals();
}

QVariant ProjectTrashedTreeModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    Q_UNUSED(section)
            Q_UNUSED(orientation)
            Q_UNUSED(role)

    return QVariant();
}

bool ProjectTrashedTreeModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (value != headerData(section, orientation, role)) {
        // FIXME: Implement me!
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}

QModelIndex ProjectTrashedTreeModel::index(int row, int column, const QModelIndex &parent) const
{
    if(m_projectId == -1){
        return QModelIndex();
    }

    if (!hasIndex(row, column, parent)) return QModelIndex();

    if (column != 0) return QModelIndex();

    ProjectTreeItem *parentItem;
    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());

    if(parentItem->isRootItem()){

        QList<TreeItemAddress> trashFolderIds = skrdata->treeHub()->getIdsWithInternalTitle(m_projectId, "trash_folder");

        if(trashFolderIds.isEmpty()){
            return QModelIndex();
        }

        QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(trashFolderIds.first(), true, false);


        auto *projectItem = this->getTreeItem(directChildren.at(row));
        QModelIndex modelIndex = createIndex(row, column, projectItem);
        projectItem->setModelIndex(column, QPersistentModelIndex(modelIndex));

        return modelIndex;
    }



    QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(parentItem->treeItemAddress(), true, false);
    TreeItemAddress treeItemId = directChildren.at(row);



    ProjectTreeItem *childItem = getTreeItem(treeItemId);
    QModelIndex modelIndex = createIndex(row, column, childItem);
    parentItem->setModelIndex(column, QPersistentModelIndex(modelIndex));
    return modelIndex;

}

QModelIndex ProjectTrashedTreeModel::parent(const QModelIndex &index) const
{
    if (!index.isValid()) return QModelIndex();

    if(m_projectId == -1){
        return QModelIndex();
    }

    ProjectTreeItem *childItem  = static_cast<ProjectTreeItem *>(index.internalPointer());
    TreeItemAddress parentAddress = skrdata->treeHub()->getParentId(childItem->treeItemAddress());

    if(!parentAddress.isValid()){
        return QModelIndex();
    }

    if(skrdata->treeHub()->getInternalTitle(parentAddress) == "trash_folder"){
        return QModelIndex();
    }
    ProjectTreeItem *parentItem = getTreeItem(parentAddress);

    QModelIndex parentIndex = createIndex(parentItem->row(), 0, parentItem);
    parentItem->setModelIndex(0, QPersistentModelIndex(parentIndex));
    return parentIndex;

}

int ProjectTrashedTreeModel::rowCount(const QModelIndex &parent) const
{
    if (parent.column() > 0) return 0;

    if(m_projectId < 0){
        return 0;
    }

    if (!parent.isValid()) {

        QList<TreeItemAddress> trashFolderIds = skrdata->treeHub()->getIdsWithInternalTitle(m_projectId, "trash_folder");

        if(trashFolderIds.isEmpty()){
            qCritical() << "no trash_folder found";
            return 0;
        }

        return skrdata->treeHub()->getAllDirectChildren(trashFolderIds.first(), true, false).count();
    }

    ProjectTreeItem *parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());
    return skrdata->treeHub()->getAllDirectChildren(parentItem->treeItemAddress(), true, false).count();
}

int ProjectTrashedTreeModel::columnCount(const QModelIndex &parent) const
{
//    if (!parent.isValid())
//        return 0;

    return 1;
}

QVariant ProjectTrashedTreeModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();



    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid));



    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem()) {
        return item->data(ProjectTreeItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole) {
        return item->data(ProjectTreeItem::Roles::TitleRole);
    }


    if (role == ProjectTreeItem::Roles::ProjectNameRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TitleRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::InternalTitleRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TreeItemAddressRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::LabelRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IndentRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::SortOrderRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TypeRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CreationDateRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::UpdateDateRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ContentDateRole) {
        return item->data(role);
    }

//    if (role == ProjectTreeItem::Roles::CharCountRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::WordCountRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::CharCountGoalRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::WordCountGoalRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::CharCountWithChildrenRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::WordCountWithChildrenRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::TrashedRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::ProjectIsBackupRole) {
//        return item->data(role);
//    }

//    if (role == ProjectTreeItem::Roles::ProjectIsActiveRole) {
//        return item->data(role);
//    }

    if (role == ProjectTreeItem::Roles::IsRenamableRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsMovableRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CanAddSiblingTreeItemRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CanAddChildTreeItemRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsTrashableRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsOpenableRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsCopyableRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::OtherPropertiesRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CutCopyRole) {
        return item->data(role);
    }

    if (role == Qt::DecorationRole) {
        auto *plugin = m_typeWithPlugin.value(item->data(ProjectTreeItem::Roles::TypeRole).toString(), nullptr);
        if(plugin){
            return QIcon(plugin->pageTypeIconUrl(item->data(ProjectTreeItem::Roles::TreeItemAddressRole).value<TreeItemAddress>()));
        }
        return QVariant();
    }

    return QVariant();
}

bool ProjectTrashedTreeModel::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (data(index, role) != value) {
        ProjectTreeItem *item = static_cast<ProjectTreeItem *>(index.internalPointer());
        int projectId     = item->projectId();
        TreeItemAddress treeItemAddress    = item->treeItemAddress();
        SKRResult result(this);

        this->disconnectFromSKRDataSignals();

        switch (role) {
        case ProjectTreeItem::Roles::ProjectNameRole:
            result = skrdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case ProjectTreeItem::Roles::ProjectIdRole:

            // useless
            break;

        case ProjectTreeItem::Roles::TreeItemAddressRole:

            // useless
            break;

        case ProjectTreeItem::Roles::TitleRole:
            result = m_treeHub->setTitle(treeItemAddress, value.toString());

            break;

        case ProjectTreeItem::Roles::TypeRole:
            result = m_treeHub->setType(treeItemAddress, value.toString());

            break;

        case ProjectTreeItem::Roles::LabelRole:
            result = m_propertyHub->setProperty(treeItemAddress,
                                                "label", value.toString());
            break;

        case ProjectTreeItem::Roles::IndentRole:
            result = m_treeHub->setIndent(treeItemAddress, value.toInt());
            break;

        case ProjectTreeItem::Roles::SortOrderRole:
            result = m_treeHub->setSortOrder(treeItemAddress, value.toInt());
            IFOKDO(result, m_treeHub->renumberSortOrders(projectId));

            for (ProjectTreeItem *item : qAsConst(m_itemList)) {
                item->invalidateData(role);
            }
            this->populate();

            break;

        case ProjectTreeItem::Roles::TrashedRole:
            result = m_treeHub->setTrashedWithChildren(treeItemAddress,
                                                       value.toBool());
            break;

        case ProjectTreeItem::Roles::CreationDateRole:
            result = m_treeHub->setCreationDate(treeItemAddress,
                                                value.toDateTime());
            break;

        case ProjectTreeItem::Roles::UpdateDateRole:
            result = m_treeHub->setUpdateDate(treeItemAddress,
                                              value.toDateTime());
            break;

        case ProjectTreeItem::Roles::HasChildrenRole:

            // useless
            break;

        case ProjectTreeItem::Roles::CharCountRole:
            result = m_propertyHub->setProperty(treeItemAddress,
                                                "char_count",
                                                QString::number(
                                                    value.toInt()));
            break;

        case ProjectTreeItem::Roles::WordCountRole:

            result = m_propertyHub->setProperty(treeItemAddress,
                                                "word_count",
                                                QString::number(
                                                    value.toInt()));
            break;

        case ProjectTreeItem::Roles::ProjectIsActiveRole:

            skrdata->projectHub()->setActiveProject(projectId);
            break;
        }


        this->connectToSKRDataSignals();

        if (!result.isSuccess()) {
            return false;
        }
        item->invalidateData(role);

        emit dataChanged(index, index, {role});
        return true;
    }
    return false;
}

Qt::ItemFlags ProjectTrashedTreeModel::flags(const QModelIndex &index) const
{

    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;

}

QHash<int, QByteArray>ProjectTrashedTreeModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[ProjectTreeItem::Roles::ProjectNameRole]           = "projectName";
    roles[ProjectTreeItem::Roles::TreeItemAddressRole]       = "treeItemAddress";
    roles[ProjectTreeItem::Roles::ProjectIdRole]             = "projectId";
    roles[ProjectTreeItem::Roles::TitleRole]                 = "title";
    roles[ProjectTreeItem::Roles::InternalTitleRole]         = "internalTitle";
    roles[ProjectTreeItem::Roles::LabelRole]                 = "label";
    roles[ProjectTreeItem::Roles::IndentRole]                = "indent";
    roles[ProjectTreeItem::Roles::SortOrderRole]             = "sortOrder";
    roles[ProjectTreeItem::Roles::TypeRole]                  = "type";
    roles[ProjectTreeItem::Roles::CreationDateRole]          = "creationDate";
    roles[ProjectTreeItem::Roles::UpdateDateRole]            = "updateDate";
    roles[ProjectTreeItem::Roles::ContentDateRole]           = "contentDate";
    roles[ProjectTreeItem::Roles::HasChildrenRole]           = "hasChildren";
    roles[ProjectTreeItem::Roles::TrashedRole]               = "trashed";
//    roles[ProjectTreeItem::Roles::WordCountRole]             = "wordCount";
//    roles[ProjectTreeItem::Roles::CharCountRole]             = "charCount";
//    roles[ProjectTreeItem::Roles::WordCountGoalRole]         = "wordCountGoal";
//    roles[ProjectTreeItem::Roles::CharCountGoalRole]         = "charCountGoal";
//    roles[ProjectTreeItem::Roles::WordCountWithChildrenRole] = "wordCountWithChildren";
//    roles[ProjectTreeItem::Roles::CharCountWithChildrenRole] = "charCountWithChildren";
//    roles[ProjectTreeItem::Roles::ProjectIsBackupRole]       = "projectIsBackup";
//    roles[ProjectTreeItem::Roles::ProjectIsActiveRole]       = "projectIsActive";
    roles[ProjectTreeItem::Roles::IsRenamableRole]           = "isRenamable";
    roles[ProjectTreeItem::Roles::IsMovableRole]             = "isMovable";
    roles[ProjectTreeItem::Roles::CanAddSiblingTreeItemRole] = "canAddSiblingTreeItem";
    roles[ProjectTreeItem::Roles::CanAddChildTreeItemRole]   = "canAddChildTreeItem";
    roles[ProjectTreeItem::Roles::IsTrashableRole]           = "isTrashable";
    roles[ProjectTreeItem::Roles::IsOpenableRole]            = "isOpenable";
    roles[ProjectTreeItem::Roles::IsCopyableRole]            = "isCopyable";
    roles[ProjectTreeItem::Roles::OtherPropertiesRole]       = "otherProperties";
    roles[ProjectTreeItem::Roles::CutCopyRole]               = "cutCopy";
    return roles;
}

void ProjectTrashedTreeModel::populate()
{
    this->beginResetModel();
    qDeleteAll(m_itemList);
    m_itemList.clear();

    if(m_projectId < 0 || skrdata->projectHub()->getProjectCount() == 0){
        this->endResetModel();
        return;
    }

    QList<TreeItemAddress> trashFolderIds = skrdata->treeHub()->getIdsWithInternalTitle(m_projectId, "trash_folder");

    if(trashFolderIds.isEmpty()){
        this->endResetModel();
        return;
    }


    auto idList         = skrdata->treeHub()->getAllChildren(trashFolderIds.first());
    auto sortOrdersHash = skrdata->treeHub()->getAllSortOrders(m_projectId);
    auto indentsHash    = skrdata->treeHub()->getAllIndents(m_projectId);

    for (const TreeItemAddress &treeItemAddress: qAsConst(idList)) {
        m_itemList.append(new ProjectTreeItem(treeItemAddress,
                                              indentsHash.value(treeItemAddress),
                                              sortOrdersHash.value(treeItemAddress)));
    }


    this->endResetModel();
}

void ProjectTrashedTreeModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_itemList);
    m_itemList.clear();
    this->endResetModel();
}

// --------------------------------------------------------------------

void ProjectTrashedTreeModel::exploitSignalFromSKRData(const TreeItemAddress &treeItemAddress,
                                                ProjectTreeItem::Roles role)
{
    ProjectTreeItem *item = this->getTreeItem(treeItemAddress);

    if (!item) {
        return;
    }


    item->invalidateData(role);

    // search for index
    QModelIndex index = item->getModelIndex(0);

    if(!index.isValid()){
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

    }
    if (index.isValid()) {
        if(role == ProjectTreeItem::Roles::AllRoles){
             emit dataChanged(index, index);
        }
        else {
             QTimer::singleShot(2, this, [=](){emit dataChanged(index, index, QVector<int>() << role);});
        }

    }
}

// ---------------------------------------------------------------------------

void ProjectTrashedTreeModel::connectToSKRDataSignals()
{
    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::allValuesChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress) {

        this->exploitSignalFromSKRData(treeItemAddress, ProjectTreeItem::Roles::AllRoles);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::titleChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress, ProjectTreeItem::Roles::TitleRole);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(skrdata->projectHub(),
                                           &PLMProjectHub::projectNameChanged, this,
                                           [this](int projectId,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(TreeItemAddress(projectId, 0),
                                       ProjectTreeItem::Roles::ProjectNameRole);
    }, Qt::QueuedConnection);


    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::treeItemIdChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::TreeItemAddressRole);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::indentChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::IndentRole);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::sortOrderChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::SortOrderRole);
    }, Qt::QueuedConnection);


    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::updateDateChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::UpdateDateRole);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::trashedChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::TrashedRole);
    }, Qt::QueuedConnection);

//    m_dataConnectionsList << this->connect(skrdata->projectHub(),
//                                           &PLMProjectHub::activeProjectChanged, this,
//                                           [this](int projectId) {
//        Q_UNUSED(projectId)

//        for (int _projectId : skrdata->projectHub()->getProjectIdList()) {
//            this->exploitSignalFromSKRData(TreeItemAddress(_projectId, 0),
//                                           ProjectTreeItem::Roles::ProjectIsActiveRole);
//        }
//    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::cutCopyChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress, bool value) {
        Q_UNUSED(value)
       this->exploitSignalFromSKRData(treeItemAddress,
                                           ProjectTreeItem::Roles::CutCopyRole);

    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &SKRPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            treeItemCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)
        const TreeItemAddress treeItemAddress(projectId, treeItemCode);


        if (name == "label") this->exploitSignalFromSKRData(treeItemAddress,
                                                            ProjectTreeItem::Roles::LabelRole);

//        else if (name == "char_count") this->exploitSignalFromSKRData(treeItemAddress,
//                                                                 ProjectTreeItem::Roles::
//                                                                 CharCountRole);

//        else if (name == "word_count") this->exploitSignalFromSKRData(treeItemAddress,
//                                                                 ProjectTreeItem::Roles::
//                                                                 WordCountRole);

//        else if (name == "char_count_goal") this->exploitSignalFromSKRData(treeItemAddress,
//                                                                 ProjectTreeItem::Roles::
//                                                                 CharCountGoalRole);

//        else if (name == "word_count_goal") this->exploitSignalFromSKRData(treeItemAddress,
//                                                                 ProjectTreeItem::Roles::
//                                                                 WordCountGoalRole);

//        else if (name == "char_count_with_children") this->exploitSignalFromSKRData(treeItemAddress,
//                                                                               ProjectTreeItem::Roles::
//                                                                               CharCountWithChildrenRole);

//        else if (name == "word_count_with_children") this->exploitSignalFromSKRData(treeItemAddress,
//                                                                               ProjectTreeItem::Roles::
//                                                                               WordCountWithChildrenRole);

        else if (name == "is_renamable") this->exploitSignalFromSKRData(treeItemAddress,
                                                                   ProjectTreeItem::Roles::
                                                                   IsRenamableRole);

        else if (name == "is_movable") this->exploitSignalFromSKRData(treeItemAddress,
                                                                 ProjectTreeItem::Roles::
                                                                 IsMovableRole);

        else if (name == "can_add_sibling_tree_item") this->exploitSignalFromSKRData(treeItemAddress,
                                                                                ProjectTreeItem::Roles::
                                                                                CanAddSiblingTreeItemRole);

        else if (name == "can_add_child_tree_item") this->exploitSignalFromSKRData(treeItemAddress,
                                                                              ProjectTreeItem::Roles::
                                                                              CanAddChildTreeItemRole);

        else if (name == "is_trashable") this->exploitSignalFromSKRData(treeItemAddress,
                                                                   ProjectTreeItem::Roles::
                                                                   IsTrashableRole);

        else if (name == "is_openable") this->exploitSignalFromSKRData(treeItemAddress,
                                                                  ProjectTreeItem::Roles::
                                                                  IsOpenableRole);

        else if (name == "is_copyable") this->exploitSignalFromSKRData(treeItemAddress,
                                                                  ProjectTreeItem::Roles::
                                                                  IsCopyableRole);
        else {
            this->exploitSignalFromSKRData(treeItemAddress,
                                           ProjectTreeItem::Roles::
                                           OtherPropertiesRole);
        }
    }, Qt::QueuedConnection);
}

void ProjectTrashedTreeModel::disconnectFromSKRDataSignals()
{
    // disconnect from SKRTreeHub signals :
    for (const QMetaObject::Connection& connection : qAsConst(m_dataConnectionsList)) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}

int ProjectTrashedTreeModel::projectId() const
{
    return m_projectId;
}

void ProjectTrashedTreeModel::setProjectId(int newProjectId)
{
    if (m_projectId == newProjectId)
        return;
    m_projectId = newProjectId;
    populate();
    emit projectIdChanged();
}


QModelIndex ProjectTrashedTreeModel::getModelIndex(const TreeItemAddress &treeItemAddress) const {
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

ProjectTreeItem *ProjectTrashedTreeModel::getTreeItem(const TreeItemAddress &treeItemAddress) const
{
    ProjectTreeItem *result_item = nullptr;

    for (ProjectTreeItem *item : qAsConst(m_itemList)) {
        if (treeItemAddress == item->treeItemAddress()) {
            result_item = item;
            break;
        }
    }

    if (!result_item ) {
    //      qDebug() << "result_item is null";
    }

    return result_item;
}

void ProjectTrashedTreeModel::removeProjectItem(const TreeItemAddress &treeItemAddress)
{
    QMutableListIterator<ProjectTreeItem *> iter(m_itemList);
    while (iter.hasNext()) {
        ProjectTreeItem *item = iter.next();
        if (treeItemAddress == item->treeItemAddress()) {
             iter.remove();
         }
    }
}
