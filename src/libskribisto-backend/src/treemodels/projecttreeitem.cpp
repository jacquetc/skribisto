#include "projecttreeitem.h"
#include "skrdata.h"

ProjectTreeItem::ProjectTreeItem():
m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();


    m_data.insert(Roles::ProjectIdRole,       -2);
    m_data.insert(Roles::TreeItemAddressRole, QVariant::fromValue(TreeItemAddress()));
    m_data.insert(Roles::IndentRole,          -2);
    m_data.insert(Roles::SortOrderRole, 99999999);
}

ProjectTreeItem::ProjectTreeItem(const TreeItemAddress &treeItemAddress) :

    m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();

    m_data.insert(Roles::ProjectIdRole,   treeItemAddress.projectId);
    m_data.insert(Roles::TreeItemAddressRole, QVariant::fromValue(treeItemAddress));

    this->invalidateAllData();
}

ProjectTreeItem::ProjectTreeItem(const TreeItemAddress &treeItemAddress,
                         int indent,
                         int sortOrder) :

    m_invalidatedRoles(), m_isRootItem(false), otherPropertiesIncrement(0)
{
    m_treeHub     = skrdata->treeHub();
    m_propertyHub = skrdata->treePropertyHub();

    m_data.insert(Roles::ProjectIdRole,   treeItemAddress.projectId);
    m_data.insert(Roles::TreeItemAddressRole, QVariant::fromValue(treeItemAddress));
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
        m_invalidatedRoles.removeAll(ProjectTreeItem::Roles::TreeItemAddressRole);
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
        const TreeItemAddress &treeItemAddress = this->treeItemAddress();


        switch (role) {
        case Roles::AllRoles:
            this->invalidateAllData();
            break;

        case Roles::ProjectNameRole:
            m_data.insert(role, skrdata->projectHub()->getProjectName(projectId));
            break;

        case Roles::ProjectIdRole:
            // useless
            break;

        case Roles::TreeItemAddressRole:

            // useless
            break;

        case Roles::TitleRole:
            m_data.insert(role, m_treeHub->getTitle(treeItemAddress));
            break;

        case Roles::InternalTitleRole:
            m_data.insert(role, m_treeHub->getInternalTitle(treeItemAddress));
            break;

        case Roles::TypeRole:
            m_data.insert(role, m_treeHub->getType(treeItemAddress));
            break;

        case Roles::LabelRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "label"));
            break;

        case Roles::IndentRole:
            m_data.insert(role, m_treeHub->getIndent(treeItemAddress));
            break;

        case Roles::SortOrderRole:
            m_data.insert(role, m_treeHub->getSortOrder(treeItemAddress));
            break;

        case Roles::SecondaryContentRole:
            m_data.insert(role, m_treeHub->getSecondaryContent(treeItemAddress));
            break;

        case Roles::TrashedRole:
            m_data.insert(role, m_treeHub->getTrashed(treeItemAddress));
            break;

        case Roles::CreationDateRole:
            m_data.insert(role, m_treeHub->getCreationDate(treeItemAddress));
            break;

        case Roles::UpdateDateRole:
            m_data.insert(role, m_treeHub->getUpdateDate(treeItemAddress));
            break;

        case Roles::CutCopyRole:
            m_data.insert(role, m_treeHub->isCutCopy(treeItemAddress));
            break;

        case Roles::CharCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "char_count"));
            break;

        case Roles::WordCountRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "word_count"));
            break;

        case Roles::CharCountGoalRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "char_count_goal", "0").toInt());
            break;

        case Roles::WordCountGoalRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "word_count_goal", "0").toInt());
            break;

        case Roles::CharCountWithChildrenRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "char_count_with_children"));
            break;

        case Roles::WordCountWithChildrenRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
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
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "is_renamable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsMovableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "is_movable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::CanAddSiblingTreeItemRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "can_add_sibling_tree_item",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::CanAddChildTreeItemRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "can_add_child_tree_item",
                                                     "true") == "true" ? true : false);
            break;


        case Roles::IsTrashableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "is_trashable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsOpenableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
                                                     "is_openable",
                                                     "true") == "true" ? true : false);
            break;

        case Roles::IsCopyableRole:
            m_data.insert(role,
                          m_propertyHub->getProperty(treeItemAddress,
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
    const TreeItemAddress &parentAddress = skrdata->treeHub()->getParentId(treeItemAddress());

    if(!parentAddress.isValid()){
        return skrdata->projectHub()->getProjectIdList().indexOf(projectId());
    }

    return skrdata->treeHub()->getAllDirectChildren(parentAddress, true, true).indexOf(treeItemAddress());
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
    m_data.insert(Roles::ProjectIdRole,       -2);
    m_data.insert(Roles::TreeItemAddressRole, QVariant::fromValue(TreeItemAddress()));
    m_data.insert(Roles::IndentRole,           -2);
    m_data.insert(Roles::SortOrderRole, -90000000);
}

QPersistentModelIndex ProjectTreeItem::getModelIndex(int column) const
{
    return m_columnWithModelIndexHash.value(column, QPersistentModelIndex());
}

void ProjectTreeItem::setModelIndex(int column, const QPersistentModelIndex &newModelIndex)
{
    m_columnWithModelIndexHash.insert(column, newModelIndex);
}

bool ProjectTreeItem::isProjectItem()
{
    return this->indent() == 0  && this->treeItemAddress().itemId == 0 ? true : false;
}

int ProjectTreeItem::projectId()
{
    return data(Roles::ProjectIdRole).toInt();
}

TreeItemAddress ProjectTreeItem::treeItemAddress()
{
    return data(Roles::TreeItemAddressRole).value<TreeItemAddress>();
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
