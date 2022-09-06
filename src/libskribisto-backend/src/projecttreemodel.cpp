#include "projecttreemodel.h"

#include <skrdata.h>

ProjectTreeModel::ProjectTreeModel(QObject *parent)
    : QAbstractItemModel(parent)
{
    m_instance = this;

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

    if (column != 0) return QModelIndex();

    ProjectTreeItem *parentItem;
    if (!parent.isValid()) parentItem = m_rootItem;
    else parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());

    if(parentItem->isRootItem()){
        int projectId = skrdata->projectHub()->getProjectIdList().at(row);
        ProjectTreeItem *childItem = new ProjectTreeItem(projectId, 0, 0, 0);
        return createIndex(row, column, childItem);
    }



    QList<int> directChildren = skrdata->treeHub()->getAllDirectChildren(parentItem->projectId(), parentItem->treeItemId(), true, true);
    int projectId = parentItem->projectId();
    int treeItemId = directChildren.at(row);


    ProjectTreeItem *childItem = searchForExistingItem(projectId, treeItemId);


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

    ProjectTreeItem *parentItem = searchForExistingItem(childItem->projectId(), parentId);

    QModelIndex parentIndex = createIndex(parentItem->row(), 0, parentItem);
    return parentIndex;

}

int ProjectTreeModel::rowCount(const QModelIndex &parent) const
{
    if (parent.column() > 0) return 0;

    if (!parent.isValid()) {
        qDebug() << skrdata->projectHub()->getProjectCount();
        return skrdata->projectHub()->getProjectCount();
    }

    ProjectTreeItem *parentItem = static_cast<ProjectTreeItem *>(parent.internalPointer());
    return skrdata->treeHub()->getAllDirectChildren(parentItem->projectId(), parentItem->treeItemId(), true, true).count();
}

int ProjectTreeModel::columnCount(const QModelIndex &parent) const
{
//    if (!parent.isValid())
//        return 0;

    return 1;
}

QVariant ProjectTreeModel::data(const QModelIndex &index, int role) const
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

    if (role == ProjectTreeItem::Roles::TreeItemIdRole) {
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

    if (role == ProjectTreeItem::Roles::TrashedRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ProjectIsBackupRole) {
        return item->data(role);
    }

    if (role == ProjectTreeItem::Roles::ProjectIsActiveRole) {
        return item->data(role);
    }

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
        auto idList         = skrdata->treeHub()->getAllIds(projectId);
        auto sortOrdersHash = skrdata->treeHub()->getAllSortOrders(projectId);
        auto indentsHash    = skrdata->treeHub()->getAllIndents(projectId);

        for (int treeItemId : qAsConst(idList)) {
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

//---------------------------------------------------------

ProjectTreeItem *ProjectTreeModel::searchForExistingItem(int projectId, int treeItemId) const
{
        ProjectTreeItem * foundItem = nullptr;

    for(ProjectTreeItem * item : m_itemList){
        if(item->projectId() == projectId && item->treeItemId() == treeItemId){
            return item;
        }
    }

   return foundItem;

}

// --------------------------------------------------------------------

void ProjectTreeModel::exploitSignalFromSKRData(int                projectId,
                                                int                treeItemId,
                                                ProjectTreeItem::Roles role)
{
    ProjectTreeItem *item = this->searchForExistingItem(projectId, treeItemId);

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
        emit dataChanged(index, index, QVector<int>() << role);
    }
}

// ---------------------------------------------------------------------------

void ProjectTreeModel::connectToSKRDataSignals()
{
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
