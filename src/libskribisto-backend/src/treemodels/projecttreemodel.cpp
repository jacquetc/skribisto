#include "projecttreemodel.h"

#include <QIcon>

ProjectTreeModel::ProjectTreeModel(QObject *parent)
    : QAbstractItemModel(parent)
{
    m_instance = this;



    QList<PageTypeIconInterface *> pluginList =
            skrpluginhub->pluginsByType<PageTypeIconInterface>();
    for(auto *plugin : pluginList){
        m_typeWithPlugin.insert(plugin->pageType(), plugin);
    }

    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();

    m_rootItem = new ProjectTreeItem();
    m_rootItem->setIsRootItem();

    connect(skrdata->projectHub(),
            &PLMProjectHub::projectLoaded,
            this,
            &ProjectTreeModel::populate);
    connect(skrdata->projectHub(),
            &PLMProjectHub::projectClosed,
            this,
            &ProjectTreeModel::populate);
    connect(skrdata->treeHub(),
            &SKRTreeHub::treeReset,
            this,
            &ProjectTreeModel::populate);
    connect(skrdata->treeHub(),
            &SKRTreeHub::treeItemMoved,
            this,
            &ProjectTreeModel::populate);

    this->connectToSKRDataSignals();
}

ProjectTreeModel *ProjectTreeModel::m_instance = nullptr;

QVariant ProjectTreeModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    Q_UNUSED(section)
    Q_UNUSED(orientation)
    Q_UNUSED(role)

    return QVariant();
}

//-----------------------------------------------------------------------

bool ProjectTreeModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (value != headerData(section, orientation, role)) {
        // FIXME: Implement me!
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}

//-----------------------------------------------------------------------


QModelIndex ProjectTreeModel::index(int row, int column, const QModelIndex &parent) const
{
    if (!hasIndex(row, column, parent)) return QModelIndex();

    //if (column != 0) return QModelIndex();

    ProjectTreeItem *parentItem;
    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());

    if(parentItem->isRootItem()){
        int projectId = skrdata->projectHub()->getProjectIdList().at(row);
        auto *projectItem = this->getTreeItem(TreeItemAddress(projectId, 0));
        QModelIndex modelIndex = createIndex(row, column, projectItem);
        projectItem->setModelIndex(column, QPersistentModelIndex(modelIndex));

        return modelIndex;
    }


    QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(parentItem->treeItemAddress(), false, true);
    TreeItemAddress treeItemAddress = directChildren.at(row);

    if(skrdata->treeHub()->getInternalTitle(treeItemAddress) == "trash_folder"){
        return QModelIndex();
    }


    ProjectTreeItem *childItem = getTreeItem(treeItemAddress);
    QModelIndex modelIndex = createIndex(row, column, childItem);
    parentItem->setModelIndex(column, QPersistentModelIndex(modelIndex));

    return modelIndex;

}

//-----------------------------------------------------------------------


QModelIndex ProjectTreeModel::parent(const QModelIndex &index) const
{
    if (!index.isValid()) return QModelIndex();

    ProjectTreeItem *childItem  = static_cast<ProjectTreeItem *>(index.internalPointer());
    TreeItemAddress parentAddress = skrdata->treeHub()->getParentId(childItem->treeItemAddress());

    if(!parentAddress.isValid()){
        return QModelIndex();
    }

    ProjectTreeItem *parentItem = getTreeItem(parentAddress);

    QModelIndex parentIndex = createIndex(parentItem->row(), 0, parentItem);
    parentItem->setModelIndex(0, QPersistentModelIndex(parentIndex));
    return parentIndex;

}

//-----------------------------------------------------------------------


int ProjectTreeModel::rowCount(const QModelIndex &parent) const
{
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) {
        return skrdata->projectHub()->getProjectCount();
    }

    ProjectTreeItem *parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());

    QList<TreeItemAddress> ids = skrdata->treeHub()->getAllDirectChildren(parentItem->treeItemAddress(), false, true);
    return ids.count();
}

//-----------------------------------------------------------------------


int ProjectTreeModel::columnCount(const QModelIndex &parent) const
{
    //    if (!parent.isValid())
    //        return 0;

    return 4;
}

//-----------------------------------------------------------------------


QVariant ProjectTreeModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();



    Q_ASSERT(checkIndex(index,
                        QAbstractItemModel::CheckIndexOption::IndexIsValid));



    ProjectTreeItem *item = static_cast<ProjectTreeItem *>(index.internalPointer());


    if ((role == Qt::DisplayRole) && item->isProjectItem() && index.column() == 0) {
        return item->data(ProjectTreeItem::Roles::ProjectNameRole);
    }
    else if (role == Qt::DisplayRole && index.column() == 0) {
        return item->data(ProjectTreeItem::Roles::TitleRole);
    }


    if (role == ProjectTreeItem::Roles::ProjectNameRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TitleRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::InternalTitleRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TreeItemAddressRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ProjectIdRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::LabelRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IndentRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::SortOrderRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TypeRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CreationDateRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::UpdateDateRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ContentDateRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CharCountRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::WordCountRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CharCountGoalRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::WordCountGoalRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CharCountWithChildrenRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::WordCountWithChildrenRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::TrashedRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ProjectIsBackupRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ProjectIsActiveRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsRenamableRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsMovableRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CanAddSiblingTreeItemRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CanAddChildTreeItemRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsTrashableRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsOpenableRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::IsCopyableRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::OtherPropertiesRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CutCopyRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == Qt::DecorationRole && index.column() == 0) {
        auto *plugin = m_typeWithPlugin.value(item->data(ProjectTreeItem::Roles::TypeRole).toString(), nullptr);
        if(plugin){
            return QIcon(plugin->pageTypeIconUrl(item->data(ProjectTreeItem::Roles::TreeItemAddressRole).value<TreeItemAddress>()));
        }
        return QVariant();
    }

    return QVariant();
}

//-----------------------------------------------------------------------


bool ProjectTreeModel::setData(const QModelIndex &index, const QVariant &value, int role)
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

        case ProjectTreeItem::Roles::SecondaryContentRole:
            result = m_treeHub->setSecondaryContent(treeItemAddress, value.toString());
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

//-----------------------------------------------------------------------


Qt::ItemFlags ProjectTreeModel::flags(const QModelIndex &index) const
{

    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;

}

//-----------------------------------------------------------------------


bool ProjectTreeModel::insertRows(int row, int count, const QModelIndex &parent)
{
    beginInsertRows(parent, row, row + count - 1);
    // FIXME: Implement me!
    endInsertRows();
    return true;
}

bool ProjectTreeModel::insertColumns(int column, int count, const QModelIndex &parent)
{
    beginInsertColumns(parent, column, column + count - 1);
    // FIXME: Implement me!
    endInsertColumns();
    return true;
}

bool ProjectTreeModel::removeRows(int row, int count, const QModelIndex &parent)
{
    beginRemoveRows(parent, row, row + count - 1);
    // FIXME: Implement me!
    endRemoveRows();
    return true;
}

bool ProjectTreeModel::removeColumns(int column, int count, const QModelIndex &parent)
{
    beginRemoveColumns(parent, column, column + count - 1);
    // FIXME: Implement me!
    endRemoveColumns();
    return true;
}



//-----------------------------------------------------------------------

QHash<int, QByteArray>ProjectTreeModel::roleNames() const {
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
    roles[ProjectTreeItem::Roles::SecondaryContentRole]      = "secondaryContent";
    roles[ProjectTreeItem::Roles::CreationDateRole]          = "creationDate";
    roles[ProjectTreeItem::Roles::UpdateDateRole]            = "updateDate";
    roles[ProjectTreeItem::Roles::ContentDateRole]           = "contentDate";
    roles[ProjectTreeItem::Roles::HasChildrenRole]           = "hasChildren";
    roles[ProjectTreeItem::Roles::TrashedRole]               = "trashed";
    roles[ProjectTreeItem::Roles::WordCountRole]             = "wordCount";
    roles[ProjectTreeItem::Roles::CharCountRole]             = "charCount";
    roles[ProjectTreeItem::Roles::WordCountGoalRole]         = "wordCountGoal";
    roles[ProjectTreeItem::Roles::CharCountGoalRole]         = "charCountGoal";
    roles[ProjectTreeItem::Roles::WordCountWithChildrenRole] = "wordCountWithChildren";
    roles[ProjectTreeItem::Roles::CharCountWithChildrenRole] = "charCountWithChildren";
    roles[ProjectTreeItem::Roles::ProjectIsBackupRole]       = "projectIsBackup";
    roles[ProjectTreeItem::Roles::ProjectIsActiveRole]       = "projectIsActive";
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


//-----------------------------------------------------------------------

void ProjectTreeModel::populate()
{
    this->beginResetModel();
    qDeleteAll(m_itemList);
    m_itemList.clear();




    for (int projectId : skrdata->projectHub()->getProjectIdList()) {

        QList<TreeItemAddress> trashFolderIds = skrdata->treeHub()->getIdsWithInternalTitle(projectId, "trash_folder");
        QList<TreeItemAddress> trashedList;
        if(!trashFolderIds.isEmpty()){
            trashedList  = skrdata->treeHub()->getAllChildren(trashFolderIds.first());
        }



        auto idList         = skrdata->treeHub()->getAllIds(projectId);
        auto sortOrdersHash = skrdata->treeHub()->getAllSortOrders(projectId);
        auto indentsHash    = skrdata->treeHub()->getAllIndents(projectId);

        for (const TreeItemAddress &treeItemAddress : qAsConst(idList)) {
            // avoid trashed items
            if(trashedList.contains(treeItemAddress) || trashFolderIds.contains(treeItemAddress)){
                continue;
            }

            m_itemList.append(new ProjectTreeItem(treeItemAddress,
                                                  indentsHash.value(treeItemAddress),
                                                  sortOrdersHash.value(treeItemAddress)));
        }
    }

    this->endResetModel();
}

//-----------------------------------------------------------------------


void ProjectTreeModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_itemList);
    m_itemList.clear();
    this->endResetModel();
}

// --------------------------------------------------------------------

void ProjectTreeModel::exploitSignalFromSKRData(const TreeItemAddress &treeItemAddress,
                                                ProjectTreeItem::Roles role)
{
    ProjectTreeItem *item = this->getTreeItem(treeItemAddress);

    if (!item) {
        return;
    }

//    if(role == ProjectTreeItem::WordCountRole || role == ProjectTreeItem::WordCountWithChildrenRole ){

//        item->invalidateData(role);

//        QModelIndex index = item->getModelIndex(2);

//        if(index.isValid()){
//            QTimer::singleShot(2, this, [=](){emit dataChanged(index, index, QVector<int>() << role);});
//            return;
//        }
//        else {
//            qDebug() << "!index.isValid()";
//        }

//    }



    item->invalidateData(role);
    for(int column = 0; column < this->columnCount(QModelIndex()) ; column++){

        // search for index
        QModelIndex index = item->getModelIndex(column);

        if(!index.isValid()){


            QModelIndexList list =  this->match(this->index(0, column,
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
                QTimer::singleShot(2 + column*3, this, [=](){emit dataChanged(index, index, QVector<int>() << role);});
            }

        }
    }
}

// ---------------------------------------------------------------------------

void ProjectTreeModel::connectToSKRDataSignals()
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
                                           &SKRTreeHub::secondaryContentChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           const QString &newContent) {
        Q_UNUSED(newContent)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::SecondaryContentRole);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::trashedChanged, this,
                                           [this](const TreeItemAddress &treeItemAddress,
                                           bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(treeItemAddress,
                                       ProjectTreeItem::Roles::TrashedRole);
    }, Qt::QueuedConnection);

    m_dataConnectionsList << this->connect(skrdata->projectHub(),
                                           &PLMProjectHub::activeProjectChanged, this,
                                           [this](int projectId) {
        Q_UNUSED(projectId)

        for (int _projectId : skrdata->projectHub()->getProjectIdList()) {
            this->exploitSignalFromSKRData(TreeItemAddress(_projectId, 0),
                                           ProjectTreeItem::Roles::ProjectIsActiveRole);
        }
    }, Qt::QueuedConnection);

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

        else if (name == "char_count") this->exploitSignalFromSKRData(treeItemAddress,
                                                                 ProjectTreeItem::Roles::
                                                                 CharCountRole);

        else if (name == "word_count") this->exploitSignalFromSKRData(treeItemAddress,
                                                                 ProjectTreeItem::Roles::
                                                                 WordCountRole);

        else if (name == "char_count_goal") this->exploitSignalFromSKRData(treeItemAddress,
                                                                      ProjectTreeItem::Roles::
                                                                      CharCountGoalRole);

        else if (name == "word_count_goal") this->exploitSignalFromSKRData(treeItemAddress,
                                                                      ProjectTreeItem::Roles::
                                                                      WordCountGoalRole);

        else if (name == "char_count_with_children") this->exploitSignalFromSKRData(treeItemAddress,
                                                                               ProjectTreeItem::Roles::
                                                                               CharCountWithChildrenRole);

        else if (name == "word_count_with_children") this->exploitSignalFromSKRData(treeItemAddress,
                                                                               ProjectTreeItem::Roles::
                                                                               WordCountWithChildrenRole);

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

void ProjectTreeModel::disconnectFromSKRDataSignals()
{
    // disconnect from SKRTreeHub signals :
    for (const QMetaObject::Connection& connection : qAsConst(m_dataConnectionsList)) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}


QModelIndex ProjectTreeModel::getModelIndex(const TreeItemAddress &treeItemAddress) const {
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

ProjectTreeItem *ProjectTreeModel::getTreeItem(const TreeItemAddress &treeItemAddress) const
{
    ProjectTreeItem *result_item = nullptr;

    for (ProjectTreeItem *item : qAsConst(m_itemList)) {
        if (treeItemAddress == item->treeItemAddress()) {
            result_item = item;
            break;
        }
    }

    if (!result_item) {
        //        qDebug() << "result_item is null";
    }

    return result_item;
}

void ProjectTreeModel::removeProjectItem(const TreeItemAddress &treeItemAddress)
{
    QMutableListIterator<ProjectTreeItem *> iter(m_itemList);
    while (iter.hasNext()) {
        ProjectTreeItem *item = iter.next();
        if (treeItemAddress == item->treeItemAddress())  {
            iter.remove();
        }
    }
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddItemAfterCommand::AddItemAfterCommand(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QVariantMap &properties, ProjectTreeModel *model) :
    m_targetTreeItemAddress(targetTreeItemAddress),  m_type(type), m_model(model), m_newId(TreeItemAddress()), m_properties(properties)
{

    m_pageInterfacePluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();

}

void AddItemAfterCommand::undo(){
    TreeItemAddress parentId = skrdata->treeHub()->getParentId(m_newId);
    int row = skrdata->treeHub()->row(m_newId);
    QModelIndex parentIndex = m_model->getModelIndex(parentId);

    m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    m_savedItemValues = skrdata->treeHub()->saveId(m_newId);

    SKRResult result =  skrdata->treeHub()->removeTreeItem(m_newId);
    m_model->removeProjectItem(m_newId);


    for(int propertyId : m_propertyIds){
        skrdata->treePropertyHub()->removeProperty(m_newId.projectId, propertyId);
    }


    m_model->connectToSKRDataSignals();

    m_model->endRemoveRows();

}

void AddItemAfterCommand::redo(){
    TreeItemAddress parentId = skrdata->treeHub()->getParentId(m_targetTreeItemAddress);
    int row = skrdata->treeHub()->row(m_targetTreeItemAddress) + 1;
    QModelIndex parentIndex = m_model->getModelIndex(parentId);

    m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();
    SKRResult result =  skrdata->treeHub()->addTreeItemBelow(m_targetTreeItemAddress, m_type);


    if(!m_newId.isValid()){
        // store id
        m_newId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
    }
    else{
        // reapply id
        TreeItemAddress temporaryId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(temporaryId, m_newId.itemId);
        }

    }

    m_model->m_itemList.append(new ProjectTreeItem(m_newId));
    m_model->connectToSKRDataSignals();

    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_newId, m_savedItemValues);
    }

    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids


        for(auto *plugin : m_pageInterfacePluginList){
            if(plugin->pageType() == m_type){
                QVariantMap properties = plugin->propertiesForCreationOfTreeItem(m_properties);

                QVariantMap::const_iterator i = properties.constBegin();
                while (i != properties.constEnd()) {
                    SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
                    m_propertyIds.append(result.getData("propertyId", -1).toInt());
                    ++i;
                }

                break;
            }
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_newId.projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }



    m_model->endInsertRows();

}

TreeItemAddress AddItemAfterCommand::result() const
{
    return m_newId;

}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddItemBeforeCommand::AddItemBeforeCommand(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QVariantMap &properties, ProjectTreeModel *model) :
    m_targetTreeItemAddress(targetTreeItemAddress),  m_type(type), m_model(model), m_newId(TreeItemAddress()), m_properties(properties)
{
    m_pageInterfacePluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();

}

void AddItemBeforeCommand::undo()
{
    TreeItemAddress parentId = skrdata->treeHub()->getParentId(m_newId);
    int row = skrdata->treeHub()->row(m_newId);
    QModelIndex parentIndex = m_model->getModelIndex(parentId);

    m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    m_savedItemValues = skrdata->treeHub()->saveId(m_newId);

    SKRResult result =  skrdata->treeHub()->removeTreeItem(m_newId);
    m_model->removeProjectItem(m_newId);

    for(int propertyId : m_propertyIds){
        skrdata->treePropertyHub()->removeProperty(m_targetTreeItemAddress.projectId, propertyId);
    }

    m_model->connectToSKRDataSignals();
    m_model->endRemoveRows();
}

void AddItemBeforeCommand::redo()
{
    TreeItemAddress parentId = skrdata->treeHub()->getParentId(m_targetTreeItemAddress);
    int row = skrdata->treeHub()->row(m_targetTreeItemAddress);
    QModelIndex parentIndex = m_model->getModelIndex(parentId);

    m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();
    SKRResult result =  skrdata->treeHub()->addTreeItemAbove(m_targetTreeItemAddress, m_type);


    if(!m_newId.isValid()){
        // store id
        m_newId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
    }
    else{
        // reapply id
        TreeItemAddress temporaryId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(temporaryId, m_newId.itemId);
        }

    }

    m_model->m_itemList.append(new ProjectTreeItem(m_newId));
    m_model->connectToSKRDataSignals();


    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_newId, m_savedItemValues);
    }


    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids


        for(auto *plugin : m_pageInterfacePluginList){
            if(plugin->pageType() == m_type){
                QVariantMap properties = plugin->propertiesForCreationOfTreeItem(m_properties);

                QVariantMap::const_iterator i = properties.constBegin();
                while (i != properties.constEnd()) {
                    SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
                    m_propertyIds.append(result.getData("propertyId", -1).toInt());
                    ++i;
                }

                break;
            }
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_newId.projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }


    m_model->endInsertRows();
}

TreeItemAddress AddItemBeforeCommand::result() const
{
    return m_newId;

}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddSubItemCommand::AddSubItemCommand(const TreeItemAddress &targetTreeItemAddress, const QString &type, const QVariantMap &properties, ProjectTreeModel *model) :
    m_targetTreeItemAddress(targetTreeItemAddress),  m_type(type), m_model(model), m_newId(TreeItemAddress()), m_properties(properties)
{
    m_pageInterfacePluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();

}

void AddSubItemCommand::undo()
{
    TreeItemAddress parentId = skrdata->treeHub()->getParentId( m_newId);
    int row = skrdata->treeHub()->row(m_newId);
    QModelIndex parentIndex = m_model->getModelIndex(parentId);

    m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    m_savedItemValues = skrdata->treeHub()->saveId(m_newId);

    SKRResult result =  skrdata->treeHub()->removeTreeItem( m_newId);
    m_model->removeProjectItem(m_newId);

    for(int propertyId : m_propertyIds){
        skrdata->treePropertyHub()->removeProperty(m_newId.projectId, propertyId);
    }


    m_model->connectToSKRDataSignals();
    m_model->endRemoveRows();
}

void AddSubItemCommand::redo()
{

    QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(m_targetTreeItemAddress, true, true);

    int row = directChildren.count();

    QModelIndex parentIndex = m_model->getModelIndex(m_targetTreeItemAddress);

    m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    SKRResult result =  skrdata->treeHub()->addChildTreeItem(m_targetTreeItemAddress, m_type);


    if(!m_newId.isValid()){
        // store id
        m_newId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
    }
    else{
        // reapply id
        TreeItemAddress temporaryId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(temporaryId, m_newId.itemId);
        }

    }
    m_model->m_itemList.append(new ProjectTreeItem(m_newId));

    m_model->connectToSKRDataSignals();

    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_newId, m_savedItemValues);
    }

    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids


        for(auto *plugin : m_pageInterfacePluginList){
            if(plugin->pageType() == m_type){
                QVariantMap properties = plugin->propertiesForCreationOfTreeItem(m_properties);

                QVariantMap::const_iterator i = properties.constBegin();
                while (i != properties.constEnd()) {
                    SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
                    m_propertyIds.append(result.getData("propertyId", -1).toInt());
                    ++i;
                }

                break;
            }
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_newId.projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }


    m_model->endInsertRows();
}

TreeItemAddress AddSubItemCommand::result() const
{
    return m_newId;
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddRawItemCommand::AddRawItemCommand(int projectId, int sortOrder, int indent, const QString &type, const QString &title, const QString &internalTitle,  const QVariantMap &properties, bool renumber, ProjectTreeModel *model):
    m_projectId(projectId),
    m_sortOrder(sortOrder),
    m_indent(indent),
    m_renumber(renumber),
    m_newId(TreeItemAddress()),
    m_title(title),
    m_type(type),
    m_internalTitle(internalTitle),
    m_properties(properties),
    m_model(model)
{
    m_pageInterfacePluginList = skrdata->pluginHub()->pluginsByType<PageInterface>();


}
void AddRawItemCommand::undo()
{
    //m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    skrdata->treePropertyHub()->restore(m_projectId, m_oldPropertyTable);
    skrdata->treeHub()->restoreTree(m_projectId, m_oldTree);
    m_model->connectToSKRDataSignals();

    //m_model->endRemoveRows();
}

void AddRawItemCommand::redo()
{

    if(m_newTree.isEmpty()){
        m_oldPropertyTable = skrdata->treePropertyHub()->save(m_projectId);
        m_oldTree = skrdata->treeHub()->saveTree(m_projectId);
    }
    else {
        skrdata->treePropertyHub()->restore(m_projectId, m_newPropertyTable);
        skrdata->treeHub()->restoreTree(m_projectId, m_newTree);
        return;
    }
//    QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(m_targetTreeItemAddress, true, true);

//    int row = directChildren.count();

//    QModelIndex parentIndex = m_model->getModelIndex(m_targetTreeItemAddress);

    //m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    SKRResult result = skrdata->treeHub()->addTreeItem(m_projectId, m_sortOrder, m_indent, m_type, m_title, m_internalTitle, m_renumber);


    if(!m_newId.isValid()){
        // store id
        m_newId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
    }
    else{
        // reapply id
        TreeItemAddress temporaryId = result.getData("treeItemAddress", QVariant::fromValue(TreeItemAddress())).value<TreeItemAddress>();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(temporaryId, m_newId.itemId);
        }

    }
    //m_model->m_itemList.append(new ProjectTreeItem(m_newId));

    emit skrdata->treeHub()->treeReset(m_projectId);
    m_model->connectToSKRDataSignals();

    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_newId, m_savedItemValues);
    }

    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids

        for(auto *plugin : m_pageInterfacePluginList){
            if(plugin->pageType() == m_type){
                QVariantMap properties = plugin->propertiesForCreationOfTreeItem(m_properties);

                QVariantMap::const_iterator i = properties.constBegin();
                while (i != properties.constEnd()) {
                    SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
                    m_propertyIds.append(result.getData("propertyId", -1).toInt());
                    ++i;
                }

                break;
            }
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }


    //m_model->endInsertRows();

    m_newPropertyTable = skrdata->treePropertyHub()->save(m_projectId);
    m_newTree = skrdata->treeHub()->saveTree(m_projectId);

}

TreeItemAddress AddRawItemCommand::result() const
{
    return m_newId;
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

SetItemPropertyCommand::SetItemPropertyCommand(const TreeItemAddress &targetTreeItemAddress, const QString &property, const QVariant &value, bool isSystem) :
    m_targetTreeItemAddress(targetTreeItemAddress), m_property(property), m_newValue(value), m_isSystem(isSystem), m_newId(-1)
{

}

void SetItemPropertyCommand::undo()
{
    if(m_oldValue.isValid()){

        skrdata->treePropertyHub()->setProperty(m_targetTreeItemAddress, m_property , m_oldValue.toString(), m_isSystem);

    }
    else{
        skrdata->treePropertyHub()->removeProperty(m_targetTreeItemAddress.projectId, m_newId);
    }


}

void SetItemPropertyCommand::redo()
{
    QString propertyValue = skrdata->treePropertyHub()->getProperty(m_targetTreeItemAddress, m_property, QString());

    if(!propertyValue.isEmpty()){
        m_oldValue = propertyValue;
    }


    if(m_newId == -1){
        SKRResult result = skrdata->treePropertyHub()->setProperty(m_targetTreeItemAddress, m_property , m_newValue.toString(), m_isSystem);
        m_newId = result.getData("propertyId", -1).toInt();
    }
    else {
        SKRResult result = skrdata->treePropertyHub()->setProperty(m_targetTreeItemAddress, m_property , m_newValue.toString(), m_isSystem);
        int temporaryId = result.getData("propertyId", -1).toInt();
        skrdata->treePropertyHub()->setId(m_targetTreeItemAddress.projectId, temporaryId, m_newId);

    }


}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

MoveItemsCommand::MoveItemsCommand(QList<TreeItemAddress> sourceTreeItemAddresses, const TreeItemAddress &targetTreeItemAddress, Move move) : Command("Move items"),
m_sourceTreeItemAddresses(sourceTreeItemAddresses),
  m_targetTreeItemAddress(targetTreeItemAddress),
    m_move(move)
{

}

void MoveItemsCommand::undo()
{
    skrdata->treeHub()->restoreTree(m_targetTreeItemAddress.projectId, m_oldTree);

}

void MoveItemsCommand::redo()
{
    if(m_newTree.isEmpty()){
        m_oldTree = skrdata->treeHub()->saveTree(m_targetTreeItemAddress.projectId);
    }
    else {
        skrdata->treeHub()->restoreTree(m_targetTreeItemAddress.projectId, m_newTree);
        return;
    }

    if(m_move == Move::AsChildOf){

        for(int i = 0; i < m_sourceTreeItemAddresses.count() ; i++){
            if(i == 0){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_sourceTreeItemAddresses.at(i), m_targetTreeItemAddress);
            }
            else {
                // use the already moved items as reference
                skrdata->treeHub()->moveTreeItem(m_sourceTreeItemAddresses.at(i), m_sourceTreeItemAddresses.at(i - 1), true);
            }
        }

    }

    else if(m_move == Move::Above){

        for(int i = 0; i < m_sourceTreeItemAddresses.count() ; i++){
            if(i == 0){
                skrdata->treeHub()->moveTreeItem(m_sourceTreeItemAddresses.at(i), m_targetTreeItemAddress, false);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_sourceTreeItemAddresses.at(i), m_sourceTreeItemAddresses.at(i - 1), true);
            }
        }

    }

    else if(m_move == Move::Below){

        for(int i = 0; i < m_sourceTreeItemAddresses.count() ; i++){
            if(i == 0){
                skrdata->treeHub()->moveTreeItem(m_sourceTreeItemAddresses.at(i), m_targetTreeItemAddress, true);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_sourceTreeItemAddresses.at(i), m_sourceTreeItemAddresses.at(i - 1), true);
            }
        }
    }


    m_newTree = skrdata->treeHub()->saveTree(m_targetTreeItemAddress.projectId);

}


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

RenameItemCommand::RenameItemCommand(const TreeItemAddress &treeItemAddress, const QString &newName) : Command("Rename item"),
    m_treeItemAddress(treeItemAddress),
    m_newName(newName)
{}

void RenameItemCommand::undo()
{
    skrdata->treeHub()->setTitle(m_treeItemAddress, m_oldName);

}

void RenameItemCommand::redo()
{
    m_oldName = skrdata->treeHub()->getTitle(m_treeItemAddress);
    skrdata->treeHub()->setTitle(m_treeItemAddress, m_newName);
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------


TrashItemCommand::TrashItemCommand(const TreeItemAddress &treeItemAddress, bool newTrashState, const TreeItemAddress &forcedOriginalParentAddress, int forcedOriginalRow) : Command(newTrashState ? "Trash item" : "Restore item"),
    m_treeItemAddress(treeItemAddress),
    m_newTrashState(newTrashState),
    m_originalParentId(TreeItemAddress()),
    m_originalRow(-1),
    m_forcedOriginalParentAddress(forcedOriginalParentAddress),
    m_forcedOriginalRow(forcedOriginalRow),
    m_result(true)
{

}

void TrashItemCommand::undo()
{
    skrdata->treePropertyHub()->restore(m_treeItemAddress.projectId, m_oldPropertyTable);
    skrdata->treeHub()->restoreTree(m_treeItemAddress.projectId, m_oldTree);
}

void TrashItemCommand::redo()
{
    if(m_newTree.isEmpty()){
        m_oldPropertyTable = skrdata->treePropertyHub()->save(m_treeItemAddress.projectId);
        m_oldTree = skrdata->treeHub()->saveTree(m_treeItemAddress.projectId);
    }
    else {
        skrdata->treePropertyHub()->restore(m_treeItemAddress.projectId, m_newPropertyTable);
        skrdata->treeHub()->restoreTree(m_treeItemAddress.projectId, m_newTree);
        return;
    }


    m_oldTrashState = skrdata->treeHub()->getTrashed(m_treeItemAddress);

    if(m_oldTrashState == m_newTrashState){
        this->setObsolete(true);
        return;
    }

    QList<TreeItemAddress> trashFolderList = skrdata->treeHub()->getIdsWithInternalTitle(m_treeItemAddress.projectId, "trash_folder");

    if(trashFolderList.isEmpty()){
        return;
    }

    TreeItemAddress trashFolderId = trashFolderList.first();


    if(m_newTrashState){ //trash

            TreeItemAddress parentId = skrdata->treeHub()->getParentId(m_treeItemAddress);
            int row = skrdata->treeHub()->row(m_treeItemAddress);
            m_originalParentId = parentId;
            m_originalRow = row;

            skrdata->treePropertyHub()->setProperty(m_treeItemAddress, "parent_id_before_trashed", QString::number(m_originalParentId.itemId));
            skrdata->treePropertyHub()->setProperty(m_treeItemAddress, "row_before_trashed", QString::number(m_originalRow));

            skrdata->treeHub()->moveTreeItemAsChildOf(m_treeItemAddress, trashFolderId, true);

    } // restore
    else {
        if(!m_forcedOriginalParentAddress.isValid() && m_forcedOriginalRow == -1){

            // get from properties:
            m_originalParentId = TreeItemAddress(m_treeItemAddress.projectId, skrdata->treePropertyHub()->getProperty(m_treeItemAddress, "parent_id_before_trashed", "-1").toInt());
            m_originalRow = skrdata->treePropertyHub()->getProperty(m_treeItemAddress, "row_before_trashed", "-1").toInt();

            // no original parent found
            if(!m_originalParentId.isValid() || m_originalRow == -1){
                this->setObsolete(true);
                m_result = SKRResult(SKRResult::Warning, "TrashItemCommand", "no_valid_original_parent");
                return;
            }
            // no valid original parent found
            if(!m_originalParentId.isValid()  && m_originalRow != -1){
                if(!skrdata->treeHub()->getTrashed( m_originalParentId)) {
                    if(!skrdata->treeHub()->getAllIds(m_treeItemAddress.projectId).contains(m_originalParentId)){
                        this->setObsolete(true);
                        m_result = SKRResult(SKRResult::Warning, "TrashItemCommand", "no_valid_original_parent");
                        return;

                    }
                }

            }
            QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(m_originalParentId, false, true);
            if(directChildren.size() <= m_originalRow){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_treeItemAddress, m_originalParentId, true);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_treeItemAddress, directChildren.at(m_originalRow), false);
            }


        }
        else {

            // restore with forced original parent and row

            QList<TreeItemAddress> directChildren = skrdata->treeHub()->getAllDirectChildren(m_forcedOriginalParentAddress, false, true);
            if(m_forcedOriginalRow == -1){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_treeItemAddress, m_forcedOriginalParentAddress, true);
            }
            else if(directChildren.size() <= m_forcedOriginalRow){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_treeItemAddress, m_forcedOriginalParentAddress, true);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_treeItemAddress, directChildren.at(m_forcedOriginalRow), false);
            }
        }

    }
    skrdata->treeHub()->setTrashedWithChildren(m_treeItemAddress, m_newTrashState);

    m_newPropertyTable = skrdata->treePropertyHub()->save(m_treeItemAddress.projectId);
    m_newTree = skrdata->treeHub()->saveTree(m_treeItemAddress.projectId);

}

bool TrashItemCommand::result() const
{
    return m_result;
}
