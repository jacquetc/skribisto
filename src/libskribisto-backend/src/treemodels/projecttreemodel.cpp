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

bool ProjectTreeModel::setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role)
{
    if (value != headerData(section, orientation, role)) {
        // FIXME: Implement me!
        emit headerDataChanged(orientation, section, section);
        return true;
    }
    return false;
}

QModelIndex ProjectTreeModel::index(int row, int column, const QModelIndex &parent) const
{
    if (!hasIndex(row, column, parent)) return QModelIndex();

    //if (column != 0) return QModelIndex();

    ProjectTreeItem *parentItem;
    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());

    if(parentItem->isRootItem()){
        int projectId = skrdata->projectHub()->getProjectIdList().at(row);
        auto *projectItem = this->getTreeItem(projectId, 0);

        return createIndex(row, column, projectItem);
    }



    QList<int> directChildren = skrdata->treeHub()->getAllDirectChildren(parentItem->projectId(), parentItem->treeItemId(), false, true);
    int projectId = parentItem->projectId();
    int treeItemId = directChildren.at(row);

    if(skrdata->treeHub()->getInternalTitle(projectId, treeItemId) == "trash_folder"){
        return QModelIndex();
    }


    ProjectTreeItem *childItem = getTreeItem(projectId, treeItemId);


    return createIndex(row, column, childItem);

}

QModelIndex ProjectTreeModel::parent(const QModelIndex &index) const
{
    if (!index.isValid()) return QModelIndex();

    ProjectTreeItem *childItem  = static_cast<ProjectTreeItem *>(index.internalPointer());
    int parentId = skrdata->treeHub()->getParentId(childItem->projectId(), childItem->treeItemId());

    if(parentId == -1){
        return QModelIndex();
    }

    ProjectTreeItem *parentItem = getTreeItem(childItem->projectId(), parentId);

    QModelIndex parentIndex = createIndex(parentItem->row(), 0, parentItem);
    return parentIndex;

}

int ProjectTreeModel::rowCount(const QModelIndex &parent) const
{
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) {
        return skrdata->projectHub()->getProjectCount();
    }

    ProjectTreeItem *parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());

    QList<int> ids = skrdata->treeHub()->getAllDirectChildren(parentItem->projectId(), parentItem->treeItemId(), false, true);
    return ids.count();
}

int ProjectTreeModel::columnCount(const QModelIndex &parent) const
{
    //    if (!parent.isValid())
    //        return 0;

    return 4;
}

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

    if (role == ProjectTreeItem::Roles::TreeItemIdRole) {
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

    if (role == ProjectTreeItem::Roles::CharCountRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::WordCountRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CharCountGoalRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::WordCountGoalRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::CharCountWithChildrenRole && index.column() == 0) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::WordCountWithChildrenRole && index.column() == 0) {
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
            return QIcon(plugin->pageTypeIconUrl(item->data(ProjectTreeItem::Roles::ProjectIdRole).toInt(), item->data(ProjectTreeItem::Roles::TreeItemIdRole).toInt()));
        }
        return QVariant();
    }

    return QVariant();
}

bool ProjectTreeModel::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (data(index, role) != value) {
        ProjectTreeItem *item = static_cast<ProjectTreeItem *>(index.internalPointer());
        int projectId     = item->projectId();
        int treeItemId    = item->treeItemId();
        SKRResult result(this);

        this->disconnectFromSKRDataSignals();

        switch (role) {
        case ProjectTreeItem::Roles::ProjectNameRole:
            result = skrdata->projectHub()->setProjectName(projectId, value.toString());
            break;

        case ProjectTreeItem::Roles::ProjectIdRole:

            // useless
            break;

        case ProjectTreeItem::Roles::TreeItemIdRole:

            // useless
            break;

        case ProjectTreeItem::Roles::TitleRole:
            result = m_treeHub->setTitle(projectId, treeItemId, value.toString());

            break;

        case ProjectTreeItem::Roles::TypeRole:
            result = m_treeHub->setType(projectId, treeItemId, value.toString());

            break;

        case ProjectTreeItem::Roles::LabelRole:
            result = m_propertyHub->setProperty(projectId, treeItemId,
                                                "label", value.toString());
            break;

        case ProjectTreeItem::Roles::IndentRole:
            result = m_treeHub->setIndent(projectId, treeItemId, value.toInt());
            break;

        case ProjectTreeItem::Roles::SortOrderRole:
            result = m_treeHub->setSortOrder(projectId, treeItemId, value.toInt());
            IFOKDO(result, m_treeHub->renumberSortOrders(projectId));

            for (ProjectTreeItem *item : qAsConst(m_itemList)) {
                item->invalidateData(role);
            }
            this->populate();

            break;

        case ProjectTreeItem::Roles::SecondaryContentRole:
            result = m_treeHub->setSecondaryContent(projectId, treeItemId, value.toString());
            break;

        case ProjectTreeItem::Roles::TrashedRole:
            result = m_treeHub->setTrashedWithChildren(projectId,
                                                       treeItemId,
                                                       value.toBool());
            break;

        case ProjectTreeItem::Roles::CreationDateRole:
            result = m_treeHub->setCreationDate(projectId,
                                                treeItemId,
                                                value.toDateTime());
            break;

        case ProjectTreeItem::Roles::UpdateDateRole:
            result = m_treeHub->setUpdateDate(projectId,
                                              treeItemId,
                                              value.toDateTime());
            break;

        case ProjectTreeItem::Roles::HasChildrenRole:

            // useless
            break;

        case ProjectTreeItem::Roles::CharCountRole:
            result = m_propertyHub->setProperty(projectId,
                                                treeItemId,
                                                "char_count",
                                                QString::number(
                                                    value.toInt()));
            break;

        case ProjectTreeItem::Roles::WordCountRole:

            result = m_propertyHub->setProperty(projectId,
                                                treeItemId,
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

Qt::ItemFlags ProjectTreeModel::flags(const QModelIndex &index) const
{

    Qt::ItemFlags defaultFlags = QAbstractItemModel::flags(index);

    if (!index.isValid()) return defaultFlags;

    return defaultFlags;

}

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


QHash<int, QByteArray>ProjectTreeModel::roleNames() const {
    QHash<int, QByteArray> roles;

    roles[ProjectTreeItem::Roles::ProjectNameRole]           = "projectName";
    roles[ProjectTreeItem::Roles::TreeItemIdRole]            = "treeItemId";
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

void ProjectTreeModel::populate()
{
    this->beginResetModel();
    qDeleteAll(m_itemList);
    m_itemList.clear();




    for (int projectId : skrdata->projectHub()->getProjectIdList()) {

        QList<int> trashFolderIds = skrdata->treeHub()->getIdsWithInternalTitle(projectId, "trash_folder");
        QList<int> trashedList;
        if(!trashFolderIds.isEmpty()){
            trashedList  = skrdata->treeHub()->getAllChildren(projectId, trashFolderIds.first());
        }



        auto idList         = skrdata->treeHub()->getAllIds(projectId);
        auto sortOrdersHash = skrdata->treeHub()->getAllSortOrders(projectId);
        auto indentsHash    = skrdata->treeHub()->getAllIndents(projectId);

        for (int treeItemId : qAsConst(idList)) {
            // avoid trashed items
            if(trashedList.contains(treeItemId) || trashFolderIds.contains(treeItemId)){
                continue;
            }

            m_itemList.append(new ProjectTreeItem(projectId, treeItemId,
                                                  indentsHash.value(treeItemId),
                                                  sortOrdersHash.value(treeItemId)));
        }
    }

    this->endResetModel();
}

void ProjectTreeModel::clear()
{
    this->beginResetModel();
    qDeleteAll(m_itemList);
    m_itemList.clear();
    this->endResetModel();
}

// --------------------------------------------------------------------

void ProjectTreeModel::exploitSignalFromSKRData(int                projectId,
                                                int                treeItemId,
                                                ProjectTreeItem::Roles role)
{
    ProjectTreeItem *item = this->getTreeItem(projectId, treeItemId);

    if (!item) {
        return;
    }

    item->invalidateData(role);

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


    if (index.isValid()) {
        if(role == ProjectTreeItem::Roles::AllRoles){
            emit dataChanged(index, index);
        }
        else {
            emit dataChanged(index, index, QVector<int>() << role);
        }

    }
}

// ---------------------------------------------------------------------------

void ProjectTreeModel::connectToSKRDataSignals()
{
    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::allValuesChanged, this,
                                           [this](int projectId, int treeItemId) {

        this->exploitSignalFromSKRData(projectId, treeItemId, ProjectTreeItem::Roles::AllRoles);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::titleChanged, this,
                                           [this](int projectId, int treeItemId,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId, ProjectTreeItem::Roles::TitleRole);
    });

    m_dataConnectionsList << this->connect(skrdata->projectHub(),
                                           &PLMProjectHub::projectNameChanged, this,
                                           [this](int projectId,
                                           const QString& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, 0,
                                       ProjectTreeItem::Roles::ProjectNameRole);
    });


    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::treeItemIdChanged, this,
                                           [this](int projectId, int treeItemId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::TreeItemIdRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::indentChanged, this,
                                           [this](int projectId, int treeItemId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::IndentRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::sortOrderChanged, this,
                                           [this](int projectId, int treeItemId,
                                           int value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::SortOrderRole);
    });


    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::updateDateChanged, this,
                                           [this](int projectId, int treeItemId,
                                           const QDateTime& value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::UpdateDateRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::secondaryContentChanged, this,
                                           [this](int projectId, int treeItemId,
                                           const QString &newContent) {
        Q_UNUSED(newContent)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::SecondaryContentRole);
    });

    m_dataConnectionsList << this->connect(m_treeHub,
                                           &SKRTreeHub::trashedChanged, this,
                                           [this](int projectId, int treeItemId,
                                           bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::TrashedRole);
    });

    m_dataConnectionsList << this->connect(skrdata->projectHub(),
                                           &PLMProjectHub::activeProjectChanged, this,
                                           [this](int projectId) {
        Q_UNUSED(projectId)

        for (int _projectId : skrdata->projectHub()->getProjectIdList()) {
            this->exploitSignalFromSKRData(_projectId, -1,
                                           ProjectTreeItem::Roles::ProjectIsActiveRole);
        }
    });

    m_dataConnectionsList << this->connect(skrdata->treeHub(),
                                           &SKRTreeHub::cutCopyChanged, this,
                                           [this](int projectId, int treeItemId, bool value) {
        Q_UNUSED(value)
        this->exploitSignalFromSKRData(projectId, treeItemId,
                                       ProjectTreeItem::Roles::CutCopyRole);

    });

    m_dataConnectionsList << this->connect(m_propertyHub,
                                           &SKRPropertyHub::propertyChanged, this,
                                           [this](int projectId, int propertyId,
                                           int            treeItemCode,
                                           const QString& name,
                                           const QString& value) {
        Q_UNUSED(value)
        Q_UNUSED(propertyId)


        if (name == "label") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                            ProjectTreeItem::Roles::LabelRole);

        if (name == "char_count") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 ProjectTreeItem::Roles::
                                                                 CharCountRole);

        if (name == "word_count") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 ProjectTreeItem::Roles::
                                                                 WordCountRole);

        if (name == "char_count_goal") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                      ProjectTreeItem::Roles::
                                                                      CharCountGoalRole);

        if (name == "word_count_goal") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                      ProjectTreeItem::Roles::
                                                                      WordCountGoalRole);

        if (name == "char_count_with_children") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                               ProjectTreeItem::Roles::
                                                                               CharCountWithChildrenRole);

        if (name == "word_count_with_children") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                               ProjectTreeItem::Roles::
                                                                               WordCountWithChildrenRole);

        if (name == "is_renamable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                   ProjectTreeItem::Roles::
                                                                   IsRenamableRole);

        if (name == "is_movable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                 ProjectTreeItem::Roles::
                                                                 IsMovableRole);

        if (name == "can_add_sibling_tree_item") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                                ProjectTreeItem::Roles::
                                                                                CanAddSiblingTreeItemRole);

        if (name == "can_add_child_tree_item") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                              ProjectTreeItem::Roles::
                                                                              CanAddChildTreeItemRole);

        if (name == "is_trashable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                   ProjectTreeItem::Roles::
                                                                   IsTrashableRole);

        if (name == "is_openable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                  ProjectTreeItem::Roles::
                                                                  IsOpenableRole);

        if (name == "is_copyable") this->exploitSignalFromSKRData(projectId, treeItemCode,
                                                                  ProjectTreeItem::Roles::
                                                                  IsCopyableRole);
        else {
            this->exploitSignalFromSKRData(projectId, treeItemCode,
                                           ProjectTreeItem::Roles::
                                           OtherPropertiesRole);
        }
    });
}

void ProjectTreeModel::disconnectFromSKRDataSignals()
{
    // disconnect from SKRTreeHub signals :
    for (const QMetaObject::Connection& connection : qAsConst(m_dataConnectionsList)) {
        QObject::disconnect(connection);
    }

    m_dataConnectionsList.clear();
}


QModelIndex ProjectTreeModel::getModelIndex(int projectId, int treeItemId) const {
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

ProjectTreeItem *ProjectTreeModel::getTreeItem(int projectId, int treeItemId) const
{
    ProjectTreeItem *result_item = nullptr;

    for (ProjectTreeItem *item : qAsConst(m_itemList)) {
        if ((item->projectId() == projectId) && (item->treeItemId() == treeItemId)) {
            result_item = item;
            break;
        }
    }

    if (!result_item && treeItemId != -1) {
        //        qDebug() << "result_item is null";
    }

    return result_item;
}

void ProjectTreeModel::removeProjectItem(int projectId, int treeItemId)
{
    QMutableListIterator<ProjectTreeItem *> iter(m_itemList);
    while (iter.hasNext()) {
        ProjectTreeItem *item = iter.next();
        if ((item->projectId() == projectId) && (item->treeItemId() == treeItemId))  {
            iter.remove();
        }
    }
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddItemAfterCommand::AddItemAfterCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model) :
    m_projectId(projectId), m_targetId(targetId),  m_type(type), m_model(model), m_newId(-1), m_properties(properties) {

}

void AddItemAfterCommand::undo(){
    int parentId = skrdata->treeHub()->getParentId(m_projectId, m_newId);
    int row = skrdata->treeHub()->row(m_projectId, m_newId);
    QModelIndex parentIndex = m_model->getModelIndex(m_projectId, parentId);

    m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    m_savedItemValues = skrdata->treeHub()->saveId(m_projectId, m_newId);

    SKRResult result =  skrdata->treeHub()->removeTreeItem(m_projectId, m_newId);
    m_model->removeProjectItem(m_projectId, m_newId);


    for(int propertyId : m_propertyIds){
        skrdata->treePropertyHub()->removeProperty(m_projectId, propertyId);
    }


    m_model->connectToSKRDataSignals();

    m_model->endRemoveRows();

}

void AddItemAfterCommand::redo(){
    int parentId = skrdata->treeHub()->getParentId(m_projectId, m_targetId);
    int row = skrdata->treeHub()->row(m_projectId, m_targetId) + 1;
    QModelIndex parentIndex = m_model->getModelIndex(m_projectId, parentId);

    m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();
    SKRResult result =  skrdata->treeHub()->addTreeItemBelow(m_projectId, m_targetId, m_type);


    if(m_newId == -1){
        // store id
        m_newId = result.getData("treeItemId", -1).toInt();
    }
    else{
        // reapply id
        int temporaryId = result.getData("treeItemId", -1).toInt();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(m_projectId, temporaryId, m_newId);
        }

    }

    m_model->m_itemList.append(new ProjectTreeItem(m_projectId, m_newId));
    m_model->connectToSKRDataSignals();

    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_projectId, m_newId, m_savedItemValues);
    }

    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids

        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_newId, i.key() , i.value().toString(), true );
            m_propertyIds.append(result.getData("propertyId", -1).toInt());
            ++i;
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }



    m_model->endInsertRows();

}

int AddItemAfterCommand::result()
{
    return m_newId;

}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddItemBeforeCommand::AddItemBeforeCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model) :
    m_projectId(projectId), m_targetId(targetId),  m_type(type), m_model(model), m_newId(-1), m_properties(properties)
{

}

void AddItemBeforeCommand::undo()
{
    int parentId = skrdata->treeHub()->getParentId(m_projectId, m_newId);
    int row = skrdata->treeHub()->row(m_projectId, m_newId);
    QModelIndex parentIndex = m_model->getModelIndex(m_projectId, parentId);

    m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    m_savedItemValues = skrdata->treeHub()->saveId(m_projectId, m_newId);

    SKRResult result =  skrdata->treeHub()->removeTreeItem(m_projectId, m_newId);
    m_model->removeProjectItem(m_projectId, m_newId);

    for(int propertyId : m_propertyIds){
        skrdata->treePropertyHub()->removeProperty(m_projectId, propertyId);
    }

    m_model->connectToSKRDataSignals();
    m_model->endRemoveRows();
}

void AddItemBeforeCommand::redo()
{
    int parentId = skrdata->treeHub()->getParentId(m_projectId, m_targetId);
    int row = skrdata->treeHub()->row(m_projectId, m_targetId);
    QModelIndex parentIndex = m_model->getModelIndex(m_projectId, parentId);

    m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();
    SKRResult result =  skrdata->treeHub()->addTreeItemAbove(m_projectId, m_targetId, m_type);


    if(m_newId == -1){
        // store id
        m_newId = result.getData("treeItemId", -1).toInt();
    }
    else{
        // reapply id
        int temporaryId = result.getData("treeItemId", -1).toInt();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(m_projectId, temporaryId, m_newId);
        }

    }

    m_model->m_itemList.append(new ProjectTreeItem(m_projectId, m_newId));
    m_model->connectToSKRDataSignals();


    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_projectId, m_newId, m_savedItemValues);
    }


    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids

        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_newId, i.key() , i.value().toString(), true );
            m_propertyIds.append(result.getData("propertyId", -1).toInt());
            ++i;
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }


    m_model->endInsertRows();
}

int AddItemBeforeCommand::result()
{
    return m_newId;

}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

AddSubItemCommand::AddSubItemCommand(int projectId, int targetId, const QString &type, const QVariantMap &properties, ProjectTreeModel *model) :
    m_projectId(projectId), m_targetId(targetId),  m_type(type), m_model(model), m_newId(-1), m_properties(properties)
{

}

void AddSubItemCommand::undo()
{
    int parentId = skrdata->treeHub()->getParentId(m_projectId, m_newId);
    int row = skrdata->treeHub()->row(m_projectId, m_newId);
    QModelIndex parentIndex = m_model->getModelIndex(m_projectId, parentId);

    m_model->beginRemoveRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    m_savedItemValues = skrdata->treeHub()->saveId(m_projectId, m_newId);

    SKRResult result =  skrdata->treeHub()->removeTreeItem(m_projectId, m_newId);
    m_model->removeProjectItem(m_projectId, m_newId);

    for(int propertyId : m_propertyIds){
        skrdata->treePropertyHub()->removeProperty(m_projectId, propertyId);
    }


    m_model->connectToSKRDataSignals();
    m_model->endRemoveRows();
}

void AddSubItemCommand::redo()
{

    QList<int> directChildren = skrdata->treeHub()->getAllDirectChildren(m_projectId, m_targetId, true, true);

    int row = directChildren.count();

    QModelIndex parentIndex = m_model->getModelIndex(m_projectId, m_targetId);

    m_model->beginInsertRows(parentIndex, row, row);
    m_model->disconnectFromSKRDataSignals();

    SKRResult result =  skrdata->treeHub()->addChildTreeItem(m_projectId, m_targetId, m_type);


    if(m_newId == -1){
        // store id
        m_newId = result.getData("treeItemId", -1).toInt();
    }
    else{
        // reapply id
        int temporaryId = result.getData("treeItemId", -1).toInt();
        if(temporaryId != m_newId){
            skrdata->treeHub()->setTreeItemId(m_projectId, temporaryId, m_newId);
        }

    }
    m_model->m_itemList.append(new ProjectTreeItem(m_projectId, m_newId));

    m_model->connectToSKRDataSignals();

    // if redoing an undo wich have content :
    if(!m_savedItemValues.isEmpty()){
        skrdata->treeHub()->restoreId(m_projectId, m_newId, m_savedItemValues);
    }

    // properties are added here to benefit from updating by connectToSKRDataSignals

    if(m_propertyIds.isEmpty()){
        // store ids

        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_newId, i.key() , i.value().toString(), true );
            m_propertyIds.append(result.getData("propertyId", -1).toInt());
            ++i;
        }
    }
    else {
        // reapply id
        int index = 0;
        QVariantMap::const_iterator i = m_properties.constBegin();
        while (i != m_properties.constEnd()) {
            SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_newId, i.key() , i.value().toString(), true );
            int temporaryId = result.getData("propertyId", -1).toInt();

            skrdata->treePropertyHub()->setId(m_projectId, temporaryId, m_propertyIds.at(index));
            index++;
            ++i;
        }
    }


    m_model->endInsertRows();
}

int AddSubItemCommand::result()
{
    return m_newId;
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

SetItemPropertyCommand::SetItemPropertyCommand(int projectId, int targetId, const QString &property, const QVariant &value, bool isSystem) :
    m_projectId(projectId), m_targetId(targetId), m_property(property), m_newValue(value), m_isSystem(isSystem), m_newId(-1)
{

}

void SetItemPropertyCommand::undo()
{
    if(m_oldValue.isValid()){

        skrdata->treePropertyHub()->setProperty(m_projectId, m_targetId, m_property , m_oldValue.toString(), m_isSystem);

    }
    else{
        skrdata->treePropertyHub()->removeProperty(m_projectId, m_newId);
    }


}

void SetItemPropertyCommand::redo()
{
    QString propertyValue = skrdata->treePropertyHub()->getProperty(m_projectId, m_targetId, m_property, QString());

    if(!propertyValue.isEmpty()){
        m_oldValue = propertyValue;
    }


    if(m_newId == -1){
        SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_targetId, m_property , m_newValue.toString(), m_isSystem);
        m_newId = result.getData("propertyId", -1).toInt();
    }
    else {
        SKRResult result = skrdata->treePropertyHub()->setProperty(m_projectId, m_targetId, m_property , m_newValue.toString(), m_isSystem);
        int temporaryId = result.getData("propertyId", -1).toInt();
        skrdata->treePropertyHub()->setId(m_projectId, temporaryId, m_newId);

    }


}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

MoveItemsCommand::MoveItemsCommand(int sourceProjectId, QList<int> sourceIds, int targetProjectId, int targetId, Move move) : Command("Move items"),
    m_sourceProjectId(sourceProjectId),
    m_targetProjectId(targetProjectId),
    m_targetId(targetId),
    m_sourceIds(sourceIds),
    m_move(move)
{

}

void MoveItemsCommand::undo()
{
    skrdata->treeHub()->restoreTree(m_targetProjectId, m_oldTree);

}

void MoveItemsCommand::redo()
{
    if(m_newTree.isEmpty()){
        m_oldTree = skrdata->treeHub()->saveTree(m_targetProjectId);
    }
    else {
        skrdata->treeHub()->restoreTree(m_targetProjectId, m_newTree);
        return;
    }

    if(m_move == Move::AsChildOf){

        for(int i = 0; i < m_sourceIds.count() ; i++){
            if(i == 0){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_sourceProjectId, m_sourceIds.at(i), m_targetProjectId, m_targetId);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_sourceProjectId, m_sourceIds.at(i), m_targetProjectId, m_sourceIds.at(i - 1), true);
            }
        }

    }

    else if(m_move == Move::Above){

        for(int i = 0; i < m_sourceIds.count() ; i++){
            if(i == 0){
                skrdata->treeHub()->moveTreeItem(m_sourceProjectId, m_sourceIds.at(i), m_targetProjectId, m_targetId, false);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_sourceProjectId, m_sourceIds.at(i), m_targetProjectId, m_sourceIds.at(i - 1), true);
            }
        }

    }

    else if(m_move == Move::Below){

        for(int i = 0; i < m_sourceIds.count() ; i++){
            if(i == 0){
                skrdata->treeHub()->moveTreeItem(m_sourceProjectId, m_sourceIds.at(i), m_targetProjectId, m_targetId, true);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_sourceProjectId, m_sourceIds.at(i), m_targetProjectId, m_sourceIds.at(i - 1), true);
            }
        }
    }


    m_newTree = skrdata->treeHub()->saveTree(m_targetProjectId);

}


//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------

RenameItemCommand::RenameItemCommand(int projectId, int treeItemId, const QString &newName) : Command("Rename item"),
    m_projectId(projectId),
    m_treeItemId(treeItemId),
    m_newName(newName)
{}

void RenameItemCommand::undo()
{
    skrdata->treeHub()->setTitle(m_projectId, m_treeItemId, m_oldName);

}

void RenameItemCommand::redo()
{
    m_oldName = skrdata->treeHub()->getTitle(m_projectId, m_treeItemId);
    skrdata->treeHub()->setTitle(m_projectId, m_treeItemId, m_newName);
}

//------------------------------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------------------------------


TrashItemCommand::TrashItemCommand(int projectId, int treeItemId, bool newTrashState, int forcedOriginalParentId, int forcedOriginalRow) : Command(newTrashState ? "Trash item" : "Restore item"),
    m_projectId(projectId),
    m_treeItemId(treeItemId),
    m_newTrashState(newTrashState),
    m_originalParentId(-1),
    m_originalRow(-1),
    m_forcedOriginalParentId(forcedOriginalParentId),
    m_forcedOriginalRow(forcedOriginalRow),
    m_result(true)
{

}

void TrashItemCommand::undo()
{
    skrdata->treePropertyHub()->restore(m_projectId, m_oldPropertyTable);
    skrdata->treeHub()->restoreTree(m_projectId, m_oldTree);
}

void TrashItemCommand::redo()
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


    m_oldTrashState = skrdata->treeHub()->getTrashed(m_projectId, m_treeItemId);

    if(m_oldTrashState == m_newTrashState){
        this->setObsolete(true);
        return;
    }

    QList<int> trashFolderList = skrdata->treeHub()->getIdsWithInternalTitle(m_projectId, "trash_folder");

    if(trashFolderList.isEmpty()){
        return;
    }

    int trashFolderId = trashFolderList.first();


    if(m_newTrashState){ //trash

            int parentId = skrdata->treeHub()->getParentId(m_projectId, m_treeItemId);
            int row = skrdata->treeHub()->row(m_projectId, m_treeItemId);
            m_originalParentId = parentId;
            m_originalRow = row;

            skrdata->treePropertyHub()->setProperty(m_projectId, m_treeItemId, "parent_id_before_trashed", QString::number(m_originalParentId));
            skrdata->treePropertyHub()->setProperty(m_projectId, m_treeItemId, "row_before_trashed", QString::number(m_originalRow));

            skrdata->treeHub()->moveTreeItemAsChildOf(m_projectId, m_treeItemId, m_projectId, trashFolderId, true);

    } // restore
    else {
        if(m_forcedOriginalParentId == -1 && m_forcedOriginalRow == -1){

            // get from properties:
            m_originalParentId = skrdata->treePropertyHub()->getProperty(m_projectId, m_treeItemId, "parent_id_before_trashed", "-1").toInt();
            m_originalRow = skrdata->treePropertyHub()->getProperty(m_projectId, m_treeItemId, "row_before_trashed", "-1").toInt();

            // no original parent found
            if(m_originalParentId == -1 || m_originalRow == -1){
                this->setObsolete(true);
                m_result = SKRResult(SKRResult::Warning, "TrashItemCommand", "no_valid_original_parent");
                return;
            }
            // no valid original parent found
            if(m_originalParentId != -1 && m_originalRow != -1){
                if(!skrdata->treeHub()->getTrashed(m_projectId, m_originalParentId)) {
                    if(!skrdata->treeHub()->getAllIds(m_projectId).contains(m_originalParentId)){
                        this->setObsolete(true);
                        m_result = SKRResult(SKRResult::Warning, "TrashItemCommand", "no_valid_original_parent");
                        return;

                    }
                }

            }
            QList<int> directChildren = skrdata->treeHub()->getAllDirectChildren(m_projectId, m_originalParentId, false, true);
            if(directChildren.size() <= m_originalRow){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_projectId, m_treeItemId, m_projectId, m_originalParentId, true);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_projectId, m_treeItemId, m_projectId, directChildren.at(m_originalRow), false);
            }


        }
        else {

            // restore with forced original parent and row

            QList<int> directChildren = skrdata->treeHub()->getAllDirectChildren(m_projectId, m_forcedOriginalParentId, false, true);
            if(m_forcedOriginalRow == -1){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_projectId, m_treeItemId, m_projectId, m_forcedOriginalParentId, true);
            }
            else if(directChildren.size() <= m_forcedOriginalRow){
                skrdata->treeHub()->moveTreeItemAsChildOf(m_projectId, m_treeItemId, m_projectId, m_forcedOriginalParentId, true);
            }
            else {
                skrdata->treeHub()->moveTreeItem(m_projectId, m_treeItemId, m_projectId, directChildren.at(m_forcedOriginalRow), false);
            }
        }

    }
    skrdata->treeHub()->setTrashedWithChildren(m_projectId, m_treeItemId, m_newTrashState);

    m_newPropertyTable = skrdata->treePropertyHub()->save(m_projectId);
    m_newTree = skrdata->treeHub()->saveTree(m_projectId);

}

bool TrashItemCommand::result() const
{
    return m_result;
}
