#include "projecttreeitem.h"
#include "skrdata.h"

ProjectTreeItem::ProjectTreeItem():
m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();


    m_data.insert(Roles::ProjectIdRole,       -2);
    m_data.insert(Roles::TreeItemIdRole,      -2);
    m_data.insert(Roles::IndentRole,          -2);
    m_data.insert(Roles::SortOrderRole, 99999999);
}

ProjectTreeItem::ProjectTreeItem(int projectId, int treeItemId) :

    m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();

    m_data.insert(Roles::ProjectIdRole,   projectId);
    m_data.insert(Roles::TreeItemIdRole, treeItemId);

    this->invalidateAllData();
}

ProjectTreeItem::ProjectTreeItem(int projectId,
                         int treeItemId,
                         int indent,
                         int sortOrder) :

    m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();

    m_data.insert(Roles::ProjectIdRole,   projectId);
    m_data.insert(Roles::TreeItemIdRole, treeItemId);
    m_data.insert(Roles::IndentRole,         indent);
    m_data.insert(Roles::SortOrderRole,   sortOrder);

    this->invalidateAllData();
}

ProjectTreeItem::~ProjectTreeItem()
{}


void ProjectTreeItem::invalidateData(int role)
{
    m_invalidatedRoles.append(role);
}

void ProjectTreeItem::invalidateAllData()
{
    QMetaEnum metaEnum = QMetaEnum::fromType<ProjectTreeItem::Roles>();

    for (int i = 0; i < metaEnum.keyCount(); ++i) {
        m_invalidatedRoles <<   metaEnum.value(i);
        m_invalidatedRoles.removeAll(ProjectTreeItem::Roles::ProjectIdRole);
        m_invalidatedRoles.removeAll(ProjectTreeItem::Roles::TreeItemIdRole);
    }
}

QVariant ProjectTreeItem::data(int role)
{
    //QMetaEnum metaEnum = QMetaEnum::fromType<ProjectTreeItem::Roles>();

    //    if (role != IndentRole)
    //        qDebug() << "item data : " << "pr :" <<
    //        m_data.value(Roles::ProjectIdRole).toInt() <<
    //            "id :" <<
    //            m_data.value(Roles::TreeItemIdRole).toInt() << "role : " <<
    //            metaEnum.valueToKey(role);

    if (m_invalidatedRoles.contains(role)) {
        int projectId  = this->projectId();
        int treeItemId = this->treeItemId();


        switch (role) {
        case Roles::ProjectNameRole:
            m_data.insert(role, skrdata->projectHub()->getProjectName(projectId));
            break;

        case Roles::ProjectIdRole:

            // useless
            break;

        case Roles::TreeItemIdRole:

            // useless
            break;

        case Roles::TitleRole:
            m_data.insert(role, m_treeHub->getTitle(projectId, treeItemId));
            break;

        case Roles::InternalTitleRole:
            m_data.insert(role, m_treeHub->getInternalTitle(projectId, treeItemId));
            break;

        case Roles::TypeRole:
            m_data.insert(role, m_treeHub->getType(projectId, treeItemId));
            break;

        case Roles::LabelRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "label"));
            break;

        case Roles::IndentRole:
            m_data.insert(role, m_treeHub->getIndent(projectId, treeItemId));
            break;

        case Roles::SortOrderRole:
            m_data.insert(role, m_treeHub->getSortOrder(projectId, treeItemId));
            break;

        case Roles::TrashedRole:
            m_data.insert(role, m_treeHub->getTrashed(projectId, treeItemId));
            break;

        case Roles::CreationDateRole:
            m_data.insert(role, m_treeHub->getCreationDate(projectId, treeItemId));
            break;

        case Roles::UpdateDateRole:
            m_data.insert(role, m_treeHub->getUpdateDate(projectId, treeItemId));
            break;

        case Roles::CutCopyRole:
            m_data.insert(role, m_treeHub->isCutCopy(projectId, treeItemId));
            break;

        case Roles::CharCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "char_count"));
            break;

        case Roles::WordCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "word_count"));
            break;

        case Roles::CharCountGoalRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "char_count_goal", "0").toInt());
            break;

        case Roles::WordCountGoalRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "word_count_goal", "0").toInt());
            break;

        case Roles::CharCountWithChildrenRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "char_count_with_children"));
            break;

        case Roles::WordCountWithChildrenRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "word_count_with_children"));
            break;


        case Roles::ProjectIsBackupRole:
            m_data.insert(role, skrdata->projectHub()->isThisProjectABackup(projectId));
            break;


        case Roles::ProjectIsActiveRole:
            m_data.insert(role, skrdata->projectHub()->isThisProjectActive(projectId));
            break;

        case Roles::IsRenamableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_renamable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsMovableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_movable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::CanAddSiblingTreeItemRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "can_add_sibling_tree_item",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::CanAddChildTreeItemRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "can_add_child_tree_item",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::IsTrashableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_trashable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsOpenableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_openable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsCopyableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(projectId, treeItemId,
                                                     "is_copyable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::OtherPropertiesRole:
            m_data.insert(role, otherPropertiesIncrement += 1);
            break;
        }

        m_invalidatedRoles.removeAll(role);
    }
    return m_data.value(role);
}

QList<int>ProjectTreeItem::dataRoles() const
{
    return m_data.keys();
}

int ProjectTreeItem::row()
{
    int parentId = skrdata->treeHub()->getParentId(projectId(), treeItemId());

    if(parentId == -1){
        return skrdata->projectHub()->getProjectIdList().indexOf(projectId());
    }

    return skrdata->treeHub()->getAllDirectChildren(projectId(), parentId, true, true).indexOf(treeItemId());
}

bool ProjectTreeItem::isRootItem() const
{
    return m_isRootItem;
}

void ProjectTreeItem::setIsRootItem()
{
    m_isRootItem = true;

    m_data.clear();
    m_invalidatedRoles.clear();
    m_data.insert(Roles::TreeItemIdRole,       -2);
    m_data.insert(Roles::IndentRole,           -2);
    m_data.insert(Roles::SortOrderRole, -90000000);
}

bool ProjectTreeItem::isProjectItem()
{
    return this->indent() == 0  && this->treeItemId() == 0 ? true : false;
}

int ProjectTreeItem::projectId()
{
    return data(Roles::ProjectIdRole).toInt();
}

int ProjectTreeItem::treeItemId()
{
    return data(Roles::TreeItemIdRole).toInt();
}

int ProjectTreeItem::sortOrder()
{
    return data(Roles::SortOrderRole).toInt();
}

int ProjectTreeItem::indent()
{
    return data(Roles::IndentRole).toInt();
}

QString ProjectTreeItem::name()
{
    return data(Roles::TitleRole).toString();
}
