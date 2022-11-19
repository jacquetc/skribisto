#include "treeitemaddress.h"


TreeItemAddress::TreeItemAddress() : itemId(-1), projectId(-1)
{

}

TreeItemAddress::~TreeItemAddress()
{

}

TreeItemAddress::TreeItemAddress(const TreeItemAddress& treeItemAddress)
{

    projectId = treeItemAddress.projectId;
    itemId = treeItemAddress.itemId;
}

void TreeItemAddress::set(int projectId, int itemId)
{
    this->projectId = projectId;
    this->itemId = itemId;

}

TreeItemAddress::TreeItemAddress(int projectId, int itemId) : itemId(itemId), projectId(projectId)
{

}

//bool TreeItemAddress::operator!() const
//{
//    return !this->isValid();
//}

//TreeItemAddress::operator bool() const
//{
//    return this->isValid();
//}

TreeItemAddress &TreeItemAddress::operator=(const TreeItemAddress &treeItemAddress)
{

    projectId = treeItemAddress.projectId;
    itemId = treeItemAddress.itemId;

    return *this;
}

bool TreeItemAddress::operator==(const TreeItemAddress &treeItemAddress) const
{
    return projectId  == treeItemAddress.projectId &&  itemId == treeItemAddress.itemId;
}

bool TreeItemAddress::operator!=(const TreeItemAddress &treeItemAddress) const
{
    return projectId  != treeItemAddress.projectId ||  itemId != treeItemAddress.itemId;
}

bool TreeItemAddress::isValid() const
{
    return projectId > 0 && itemId > -1;
}

bool TreeItemAddress::hasOnlyProjectId() const
{
    return projectId > 0 && itemId == -1;
}
